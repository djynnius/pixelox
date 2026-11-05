//! `config.toml` and precedence (PRD §7.13): CLI flag > `PIXELOX_*` environment variable >
//! config file > built-in default. `config get|set|list|path` manage the file.

use pixelox_core::prelude::*;
use std::path::{Path, PathBuf};

/// Keys the config file accepts, with their environment variable and meaning.
pub const KEYS: &[(&str, &str, &str)] = &[
    ("threads", "PIXELOX_THREADS", "Worker threads (0 = all cores)"),
    ("mem_budget_mb", "PIXELOX_MEM_BUDGET", "Memory budget in MB"),
    ("progress", "PIXELOX_PROGRESS", "Progress on stderr: bar, json, or none"),
    ("limit_pixels", "PIXELOX_LIMIT_PIXELS", "Decompression-bomb guard in pixels"),
    ("sidecar", "PIXELOX_SIDECAR", "Write provenance sidecars: true or false"),
    ("redact_paths", "PIXELOX_REDACT_PATHS", "Redact directories in sidecars: true or false"),
    ("models.auto_consent", "PIXELOX_AUTO_CONSENT", "Standing consent for model downloads: true or false"),
    ("mcp.allow", "PIXELOX_MCP_ALLOW", "Default MCP allow roots (comma list)"),
];

#[derive(Debug, Default, Clone)]
pub struct Config {
    pub table: toml::Table,
    pub path: Option<PathBuf>,
}

fn flatten(prefix: &str, t: &toml::Table, out: &mut Vec<(String, toml::Value)>) {
    for (k, v) in t {
        let key = if prefix.is_empty() { k.clone() } else { format!("{prefix}.{k}") };
        match v {
            toml::Value::Table(inner) => flatten(&key, inner, out),
            other => out.push((key, other.clone())),
        }
    }
}

fn as_string(v: &toml::Value) -> String {
    match v {
        toml::Value::String(s) => s.clone(),
        toml::Value::Array(a) => a.iter().map(as_string).collect::<Vec<_>>().join(","),
        other => other.to_string(),
    }
}

impl Config {
    /// Load an explicit file (must exist), else `PIXELOX_CONFIG`/the default (may be absent).
    pub fn load(explicit: Option<&Path>) -> Result<Config> {
        let path = match explicit {
            Some(p) => {
                if !p.exists() {
                    return Err(Error::input_missing(p));
                }
                p.to_path_buf()
            }
            None => pixelox_core::paths::config_file(),
        };
        if !path.exists() {
            return Ok(Config { table: toml::Table::new(), path: Some(path) });
        }
        let text = std::fs::read_to_string(&path)?;
        let table: toml::Table =
            toml::from_str(&text).map_err(|e| Error::new("PX1020", format!("{}: {e}", path.display())))?;
        let cfg = Config { table, path: Some(path) };
        for (k, _) in cfg.entries() {
            if !KEYS.iter().any(|(n, _, _)| *n == k) {
                return Err(Error::new("PX1020", format!("unknown config key '{k}'"))
                    .with_hint(format!("known keys: {}", KEYS.iter().map(|k| k.0).collect::<Vec<_>>().join(", "))));
            }
        }
        Ok(cfg)
    }

    pub fn entries(&self) -> Vec<(String, toml::Value)> {
        let mut v = Vec::new();
        flatten("", &self.table, &mut v);
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v
    }

    /// Effective value: environment first, then the file.
    pub fn get(&self, key: &str) -> Option<String> {
        if let Some((_, env, _)) = KEYS.iter().find(|(k, _, _)| *k == key) {
            if let Ok(v) = std::env::var(env) {
                if !v.is_empty() {
                    return Some(v);
                }
            }
        }
        self.entries().into_iter().find(|(k, _)| k == key).map(|(_, v)| as_string(&v))
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key).and_then(|v| pixelox_core::op::parse_bool(&v).ok())
    }
}

fn check_key(k: &str) -> Result<()> {
    if KEYS.iter().any(|(n, _, _)| *n == k) {
        Ok(())
    } else {
        Err(Error::new("PX1020", format!("unknown config key '{k}'"))
            .with_hint(format!("known keys: {}", KEYS.iter().map(|k| k.0).collect::<Vec<_>>().join(", "))))
    }
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::custom("config get", Category::Models, "Print one effective configuration value", config_get)
            .inputs(Inputs::NONE)
            .param(param("key", ParamKind::Str).required().help("Key, e.g. threads or models.auto_consent"))
            .positional("key")
            .example("pixelox config get threads", "The effective thread setting")
            .example("pixelox config get models.auto_consent --json", "Whether model downloads are pre-approved")
            .example("pixelox config get progress", "The default progress mode"),
        OpSpec::custom("config set", Category::Models, "Set a value in config.toml", config_set)
            .inputs(Inputs::NONE)
            .param(param("key", ParamKind::Str).required().help("Key to set"))
            .param(param("value", ParamKind::Str).required().help("Value (true/false, a number, or text)"))
            .positional("key")
            .positional("value")
            .example("pixelox config set threads 8", "Use eight worker threads")
            .example("pixelox config set models.auto_consent true", "Pre-approve model downloads")
            .example("pixelox config set progress json --dry-run --json", "Show the change without writing"),
        OpSpec::custom("config list", Category::Models, "List every key with its effective value", config_list)
            .inputs(Inputs::NONE)
            .example("pixelox config list", "Every setting")
            .example("pixelox config list --json", "As JSON with sources")
            .example("pixelox config list --config ./ci.toml --json", "Inspect another config file"),
        OpSpec::custom("config path", Category::Models, "Print the config file path", config_path)
            .inputs(Inputs::NONE)
            .example("pixelox config path", "Where config.toml lives")
            .example("pixelox config path --json", "As JSON")
            .example("pixelox config path --config ./ci.toml", "Resolve an explicit file"),
    ]
}

fn load_for(ctx: &OpCtx) -> Result<Config> {
    Config::load(ctx.globals.config_path.as_deref())
}

fn config_get(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let k = a.str_or("key", "");
    check_key(&k)?;
    let cfg = load_for(ctx)?;
    Ok(json!({"key": k, "value": cfg.get(&k), "text": format!("{}\n", cfg.get(&k).unwrap_or_default())}))
}

fn parse_scalar(v: &str) -> toml::Value {
    if let Ok(b) = pixelox_core::op::parse_bool(v) {
        if matches!(v, "true" | "false") {
            return toml::Value::Boolean(b);
        }
    }
    if let Ok(i) = v.parse::<i64>() {
        return toml::Value::Integer(i);
    }
    toml::Value::String(v.to_string())
}

fn config_set(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let k = a.str_or("key", "");
    let v = a.str_or("value", "");
    check_key(&k)?;
    let mut cfg = load_for(ctx)?;
    let mut cur = &mut cfg.table;
    let parts: Vec<&str> = k.split('.').collect();
    for p in &parts[..parts.len() - 1] {
        cur = cur
            .entry(p.to_string())
            .or_insert_with(|| toml::Value::Table(toml::Table::new()))
            .as_table_mut()
            .ok_or_else(|| Error::new("PX1020", format!("'{p}' is not a table")))?;
    }
    cur.insert(parts[parts.len() - 1].to_string(), parse_scalar(&v));
    let path = cfg.path.clone().unwrap_or_else(pixelox_core::paths::config_file);
    let text = toml::to_string_pretty(&cfg.table).map_err(|e| Error::internal(e.to_string()))?;
    if !ctx.dry_run() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let tmp = pixelox_core::ctx::tmp_path(&path);
        std::fs::write(&tmp, text.as_bytes())?;
        std::fs::rename(&tmp, &path)?;
    }
    Ok(json!({"key": k, "value": v, "path": path.display().to_string(), "written": !ctx.dry_run()}))
}

fn config_list(_a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let cfg = load_for(ctx)?;
    let mut rows = Vec::new();
    let mut text = String::new();
    for (k, env, help) in KEYS {
        let from_env = std::env::var(env).ok().filter(|v| !v.is_empty());
        let from_file = cfg.entries().into_iter().find(|(key, _)| key == k).map(|(_, v)| as_string(&v));
        let (value, source) = match (&from_env, &from_file) {
            (Some(v), _) => (Some(v.clone()), "env"),
            (None, Some(v)) => (Some(v.clone()), "file"),
            _ => (None, "default"),
        };
        text.push_str(&format!("{k} = {}  ({source}; {help})\n", value.clone().unwrap_or_else(|| "(default)".into())));
        rows.push(json!({"key": k, "value": value, "source": source, "env": env, "help": help}));
    }
    Ok(json!({"path": cfg.path.map(|p| p.display().to_string()), "keys": rows, "text": text}))
}

fn config_path(_a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let p = ctx.globals.config_path.clone().unwrap_or_else(pixelox_core::paths::config_file);
    Ok(json!({"path": p.display().to_string(), "exists": p.exists(), "text": format!("{}\n", p.display())}))
}
