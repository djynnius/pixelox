//! The eight MCP tools (PRD §10.1). The surface is deliberately coarse: `pixelox_run` runs
//! any command with the CLI's own schemas, and the rest are conveniences around it.

use crate::server::{Access, Server};
use crate::util::base64;
use pixelox_core::ctx::{CancelToken, EncodeOptions, NoProgress, OpCtx, Progress};
use pixelox_core::envelope::Envelope;
use pixelox_core::exec::{Invocation, Runtime};
use pixelox_core::image::Image;
use pixelox_core::kernels::{self, Filter};
use pixelox_core::op::{json_to_pairs, parse_bool, scalar_to_string, OpSpec, ParamKind};
use pixelox_core::units::{RectI, RectSpec};
use pixelox_core::{compose, error, schema, Error, Result};
use serde_json::{json, Map, Value};
use std::path::{Path, PathBuf};

pub const TOOL_NAMES: &[&str] = &[
    "pixelox_describe",
    "pixelox_schema",
    "pixelox_run",
    "pixelox_info",
    "pixelox_preview",
    "pixelox_detect",
    "pixelox_jobs",
    "pixelox_docs",
];

/// Commands that make no sense inside a tool call (they own the process or the terminal).
pub const CLI_ONLY: &[&str] = &["mcp serve", "self-update", "self-uninstall"];

/// Commands that reach the network: they need both keys (server flag and call consent).
pub const NETWORK_COMMANDS: &[&str] = &["models pull"];

/// The consent tokens `pixelox_run` understands.
pub const CONSENTS: &[&str] = &["model-download"];

/// Global flags an agent may put inside `params` (ops never declare these names).
const GLOBAL_KEYS: &[&str] = &["overwrite", "seed", "format", "dry_run"];

const JOB_ACTIONS: &[&str] = &["list", "status", "wait", "cancel", "result"];
const DETECT_KINDS: &[&str] = &["subject", "objects", "salient", "faces", "text", "segment"];
const PREVIEW_DEFAULT: u32 = 768;
const PREVIEW_RANGE: (u32, u32) = (16, 4096);

/// Commands a `--read-only` server still runs besides doc ops marked read-only.
const READ_ONLY_COMMANDS: &[&str] = &["schema", "describe", "docs", "info", "version", "licenses", "verify"];
const READ_ONLY_SUFFIXES: &[&str] = &[
    " list",
    " info",
    " status",
    " result",
    " wait",
    " show",
    " validate",
    " explain",
    " slots",
    " path",
    " list-models",
];

/// True when a command never writes files.
pub fn never_writes(spec: &OpSpec) -> bool {
    spec.is_read_only()
        || READ_ONLY_COMMANDS.contains(&spec.name)
        || READ_ONLY_SUFFIXES.iter().any(|s| spec.name.ends_with(s))
}

/// A tool result plus what the audit log records.
pub(crate) struct ToolOutcome {
    pub result: Value,
    pub ok: bool,
    pub code: Option<String>,
}

impl ToolOutcome {
    /// The envelope as `structuredContent` with a text copy; `isError` when not ok.
    pub fn envelope(env: &Envelope) -> ToolOutcome {
        let v = serde_json::to_value(env).unwrap_or(Value::Null);
        ToolOutcome {
            result: json!({"content": [{"type": "text", "text": v.to_string()}], "structuredContent": v, "isError": !env.ok}),
            ok: env.ok,
            code: env.error.as_ref().map(|e| e.code.clone()),
        }
    }

    /// A failure envelope for an error raised before (or instead of) running a command.
    pub fn failure(command: &str, e: Error) -> ToolOutcome {
        ToolOutcome::failure_dry(command, e, false)
    }

    fn failure_dry(command: &str, e: Error, dry_run: bool) -> ToolOutcome {
        ToolOutcome::envelope(&Envelope::failure(command, e, 0, dry_run, 0))
    }

    fn text(text: String) -> ToolOutcome {
        ToolOutcome {
            result: json!({"content": [{"type": "text", "text": text}], "isError": false}),
            ok: true,
            code: None,
        }
    }

    fn json(v: Value) -> ToolOutcome {
        ToolOutcome {
            result: json!({"content": [{"type": "text", "text": v.to_string()}], "structuredContent": v, "isError": false}),
            ok: true,
            code: None,
        }
    }
}

// ---------------------------------------------------------------------------------------
// definitions

fn detect_kinds(s: &Server) -> Vec<String> {
    let kinds: Vec<String> =
        s.registry.ops().iter().filter_map(|o| o.name.strip_prefix("detect ")).map(String::from).collect();
    if kinds.is_empty() {
        DETECT_KINDS.iter().map(|k| k.to_string()).collect()
    } else {
        kinds
    }
}

fn annotations(read_only: bool, destructive: bool, open_world: bool) -> Value {
    json!({"readOnlyHint": read_only, "destructiveHint": destructive, "idempotentHint": read_only, "openWorldHint": open_world})
}

/// `tools/list`.
pub(crate) fn definitions(s: &Server) -> Vec<Value> {
    let net = s.cfg.allow_model_downloads;
    let consent = json!({"type": "array", "items": {"type": "string", "enum": CONSENTS},
        "description": "Consents for this call. \"model-download\" lets a missing model download (the server must also run with --allow-model-downloads); ask the user first."});
    let dry_run = json!({"type": "boolean", "default": false, "description": "Validate and return the plan (outputs, models, consents needed) without writing anything"});
    vec![
        json!({"name": "pixelox_describe", "title": "Agent briefing",
            "description": "The agent briefing: every command with a one-line summary, global flags, envelope and layer/mask addressing rules (same as `pixelox describe --agent`). Call once per session and cache it.",
            "inputSchema": {"type": "object", "properties": {}, "additionalProperties": false},
            "annotations": annotations(true, false, false)}),
        json!({"name": "pixelox_schema", "title": "Command schema",
            "description": "JSON Schema of one command: its params (the keys pixelox_run takes), result envelope, and error codes.",
            "inputSchema": {"type": "object", "properties": {
                "command": {"type": "string", "description": "Command id, e.g. filter.gaussian or comp.template.apply"}},
                "required": ["command"], "additionalProperties": false},
            "annotations": annotations(true, false, false)}),
        json!({"name": "pixelox_run", "title": "Run a command",
            "description": "Run any pixelox command in-process and return its JSON envelope (ok, result, inputs and outputs with SHA-256, warnings, error with a runnable hint). params holds the command's parameters by JSON key (see pixelox_schema) plus input/inputs, output, in_place, and the global flags overwrite, seed, format. Plan with dry_run before changing the user's files.",
            "inputSchema": {"type": "object", "properties": {
                "command": {"type": "string", "description": "Command id or path: filter.gaussian, \"layer add\", export"},
                "params": {"type": "object", "additionalProperties": true, "description": "Parameters, e.g. {\"input\": \"work/shot.pxo\", \"in_place\": true, \"sigma\": 4}. Paths must be inside the allowed directories."},
                "dry_run": dry_run,
                "consent": consent},
                "required": ["command"], "additionalProperties": false},
            "annotations": annotations(false, true, net)}),
        json!({"name": "pixelox_info", "title": "Inspect a file",
            "description": "Fast read-only summary of an image or .pxo document: size, color space, depth, layer tree, masks, fonts (the `info` command).",
            "inputSchema": {"type": "object", "properties": {
                "input": {"type": "string", "description": "Image, PSD, SVG, or .pxo document"}},
                "required": ["input"], "additionalProperties": false},
            "annotations": annotations(true, false, false)}),
        json!({"name": "pixelox_preview", "title": "Preview",
            "description": "Render a document, one layer, or a region, downscaled to fit max_size, and return it as a PNG image: your eyes on your own work.",
            "inputSchema": {"type": "object", "properties": {
                "input": {"type": "string", "description": "Image, PSD, SVG, or .pxo document"},
                "layer": {"type": "string", "description": "Render only this layer: name, #N (1 = top), @top, @bottom, @active"},
                "region": {"type": "string", "description": "Canvas region X,Y,WxH (px or %), e.g. 0,0,50%x50%"},
                "max_size": {"type": "integer", "minimum": PREVIEW_RANGE.0, "maximum": PREVIEW_RANGE.1, "default": PREVIEW_DEFAULT,
                    "description": "Longest side of the preview in px"}},
                "required": ["input"], "additionalProperties": false},
            "annotations": annotations(true, false, false)}),
        json!({"name": "pixelox_detect", "title": "Detect",
            "description": "AI detection (`detect <kind>`): subject/background, objects, faces, text, salient regions, promptable segments. Other keys are the detect command's params (pixelox_schema detect.<kind>); output/in_place save masks or cutouts.",
            "inputSchema": {"type": "object", "properties": {
                "input": {"type": "string", "description": "Image or .pxo document"},
                "kind": {"type": "string", "enum": detect_kinds(s), "description": "What to detect"},
                "output": {"type": "string", "description": "Write the result document here (-o)"},
                "in_place": {"type": "boolean", "description": "Update the input .pxo document"},
                "dry_run": dry_run,
                "consent": consent},
                "required": ["input", "kind"], "additionalProperties": true},
            "annotations": annotations(false, false, net)}),
        json!({"name": "pixelox_jobs", "title": "Background jobs",
            "description": "Background jobs: list them, or get the status, wait for, cancel, or fetch the result envelope of one by id.",
            "inputSchema": {"type": "object", "properties": {
                "action": {"type": "string", "enum": JOB_ACTIONS, "description": "What to do"},
                "id": {"type": "string", "description": "Job id (every action but list)"}},
                "required": ["action"], "additionalProperties": true},
            "annotations": annotations(false, false, false)}),
        json!({"name": "pixelox_docs", "title": "Reference page",
            "description": "Offline reference page for a command id (filter.gaussian) or an error code (PX1401).",
            "inputSchema": {"type": "object", "properties": {
                "command": {"type": "string", "description": "Command id or error code"}},
                "required": ["command"], "additionalProperties": false},
            "annotations": annotations(true, false, false)}),
    ]
}

// ---------------------------------------------------------------------------------------
// dispatch

/// Run one tool. Arguments were already checked to be a known tool name.
pub(crate) fn call(s: &Server, name: &str, args: &Value, progress: &dyn Progress, cancel: CancelToken) -> ToolOutcome {
    let Some(obj) = args.as_object() else {
        return ToolOutcome::failure(name, Error::new("PX1010", "tool arguments must be a JSON object"));
    };
    match name {
        "pixelox_describe" => describe(s, progress, cancel),
        "pixelox_schema" => match str_arg(obj, "command").and_then(|c| s.registry.require(c)) {
            Ok(spec) => ToolOutcome::json(schema::command_schema(spec)),
            Err(e) => ToolOutcome::failure("schema", e),
        },
        "pixelox_run" => run_tool(s, obj, progress, cancel),
        "pixelox_info" => info(s, obj, progress, cancel),
        "pixelox_preview" => match preview(s, obj) {
            Ok(o) => o,
            Err(e) => ToolOutcome::failure("mcp.preview", e),
        },
        "pixelox_detect" => detect(s, obj, progress, cancel),
        "pixelox_jobs" => jobs(s, obj, progress, cancel),
        "pixelox_docs" => match str_arg(obj, "command").and_then(|c| docs_text(s, c)) {
            Ok(t) => ToolOutcome::text(t),
            Err(e) => ToolOutcome::failure("docs", e),
        },
        other => ToolOutcome::failure(other, Error::new("PX1001", format!("unknown tool '{other}'"))),
    }
}

fn str_arg<'v>(obj: &'v Map<String, Value>, key: &str) -> Result<&'v str> {
    match obj.get(key) {
        Some(Value::String(s)) if !s.trim().is_empty() => Ok(s.as_str()),
        Some(Value::Null) | None => Err(Error::new("PX1003", format!("missing '{key}'")).with_context("param", key)),
        Some(_) => Err(Error::new("PX1010", format!("'{key}' must be a non-empty string")).with_context("param", key)),
    }
}

fn opt_str<'v>(obj: &'v Map<String, Value>, key: &str) -> Result<Option<&'v str>> {
    match obj.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(_) => str_arg(obj, key).map(Some),
    }
}

fn bool_value(key: &str, v: &Value) -> Result<bool> {
    let raw = scalar_to_string(v)?;
    parse_bool(&raw).map_err(|mut e| {
        e.message = format!("{key}: {}", e.message);
        e
    })
}

fn invoke(s: &Server, inv: Invocation, progress: &dyn Progress, cancel: CancelToken) -> Envelope {
    let rt = Runtime { registry: s.registry, io: s.io, progress, cancel };
    rt.invoke(inv).envelope
}

/// Run a helper command (describe, docs) quietly with plain args.
fn invoke_plain(
    s: &Server,
    spec: &OpSpec,
    args: Vec<(String, String)>,
    progress: &dyn Progress,
    cancel: CancelToken,
) -> Envelope {
    let mut g = s.call_globals();
    g.sidecar = false;
    let argv = vec!["pixelox".to_string(), "(mcp)".into(), spec.name.to_string()];
    invoke(s, Invocation { command: spec.name.to_string(), args, inputs: vec![], globals: g, argv }, progress, cancel)
}

/// The printable text of a command result (`describe --agent`, `docs`).
fn result_text(v: &Value) -> String {
    if let Some(t) = v.as_str() {
        return t.to_string();
    }
    for k in ["text", "markdown", "page", "content", "briefing"] {
        if let Some(t) = v.get(k).and_then(Value::as_str) {
            return t.to_string();
        }
    }
    serde_json::to_string_pretty(v).unwrap_or_default()
}

/// The param a positional word fills (`docs <command>`, `jobs status <id>`).
fn positional_param(spec: &OpSpec, fallbacks: &[&str]) -> String {
    spec.positionals
        .first()
        .map(|p| p.to_string())
        .or_else(|| fallbacks.iter().find_map(|n| spec.find_param(n).map(|p| p.name.to_string())))
        .unwrap_or_else(|| fallbacks[0].to_string())
}

// ---------------------------------------------------------------------------------------
// pixelox_run and the tools built on it

/// One `pixelox_run` request.
struct RunRequest {
    command: String,
    params: Map<String, Value>,
    dry_run: bool,
    consent: Vec<String>,
}

fn consent_list(v: Option<&Value>) -> Result<Vec<String>> {
    let list: Vec<String> = match v {
        None | Some(Value::Null) => vec![],
        Some(Value::String(s)) => vec![s.clone()],
        Some(Value::Array(a)) => a
            .iter()
            .map(|x| {
                x.as_str().map(String::from).ok_or_else(|| Error::new("PX1010", "consent must be a list of strings"))
            })
            .collect::<Result<_>>()?,
        Some(_) => return Err(Error::new("PX1010", "consent must be a list of strings")),
    };
    for c in &list {
        if !CONSENTS.contains(&c.as_str()) {
            return Err(Error::new("PX1010", format!("unknown consent '{c}'"))
                .with_hint(format!("accepted: {}", CONSENTS.join(", "))));
        }
    }
    Ok(list)
}

fn run_tool(s: &Server, obj: &Map<String, Value>, progress: &dyn Progress, cancel: CancelToken) -> ToolOutcome {
    let parsed = (|| -> Result<RunRequest> {
        if let Some(k) = obj.keys().find(|k| !matches!(k.as_str(), "command" | "params" | "dry_run" | "consent")) {
            return Err(Error::new("PX1002", format!("pixelox_run does not take '{k}'"))
                .with_hint("put the command's parameters (and input, output, in_place) inside \"params\"")
                .with_context("param", k));
        }
        let command = str_arg(obj, "command")?.to_string();
        let params = match obj.get("params") {
            None | Some(Value::Null) => Map::new(),
            Some(Value::Object(m)) => m.clone(),
            Some(_) => return Err(Error::new("PX1010", "params must be a JSON object")),
        };
        let dry_run = match obj.get("dry_run") {
            None | Some(Value::Null) => false,
            Some(v) => bool_value("dry_run", v)?,
        };
        Ok(RunRequest { command, params, dry_run, consent: consent_list(obj.get("consent"))? })
    })();
    match parsed {
        Ok(req) => run_request(s, req, progress, cancel),
        Err(e) => {
            let id = obj
                .get("command")
                .and_then(Value::as_str)
                .map(|c| c.trim().replace(' ', "."))
                .unwrap_or_else(|| "mcp.run".into());
            ToolOutcome::failure(&id, e)
        }
    }
}

fn run_request(s: &Server, req: RunRequest, progress: &dyn Progress, cancel: CancelToken) -> ToolOutcome {
    let spec = match s.registry.require(&req.command) {
        Ok(spec) => spec,
        Err(e) => return ToolOutcome::failure(&req.command.trim().replace(' ', "."), e),
    };
    let dry = req.dry_run;
    match prepare(s, spec, req) {
        Ok(inv) => {
            let mut env = invoke(s, inv, progress, cancel);
            explain_consent(s, &mut env);
            ToolOutcome::envelope(&env)
        }
        Err(e) => ToolOutcome::failure_dry(&spec.id(), e, dry),
    }
}

/// Make a PX1502 (consent required) hint say which key is missing on this server.
fn explain_consent(s: &Server, env: &mut Envelope) {
    let Some(e) = env.error.as_mut() else { return };
    if e.code != "PX1502" {
        return;
    }
    if let Some(h) = e.hint.take() {
        e.context.insert("cli_remedy".into(), Value::String(h));
    }
    e.hint = Some(if s.cfg.allow_model_downloads {
        "ask the user to approve the download, then repeat the call with consent: [\"model-download\"]".into()
    } else {
        "this MCP server does not download models: ask the user to run the `pixelox models pull` command in context.cli_remedy, or to restart the server with --allow-model-downloads".into()
    });
}

/// Validate, sandbox-check, and translate a request into an invocation. Nothing here
/// touches the filesystem beyond resolving paths.
fn prepare(s: &Server, spec: &OpSpec, req: RunRequest) -> Result<Invocation> {
    let id = spec.id();
    if CLI_ONLY.contains(&spec.name) {
        return Err(Error::new("PX1520", format!("'{id}' is not available over MCP"))
            .with_hint(format!("run `pixelox {}` from a shell", spec.name)));
    }
    if s.cfg.read_only && !never_writes(spec) {
        return Err(Error::new("PX1310", format!("this server is read-only; '{id}' can write files"))
            .with_hint("use inspection commands (info, analyze ..., pixelox_preview), or ask the user to restart the server without --read-only")
            .with_context("command", &id));
    }
    let model_consent = req.consent.iter().any(|c| c == "model-download");
    let downloads = model_consent && s.cfg.allow_model_downloads;
    if NETWORK_COMMANDS.contains(&spec.name) && !downloads {
        let missing = if s.cfg.allow_model_downloads {
            "consent: [\"model-download\"] on the call"
        } else {
            "a server started with --allow-model-downloads"
        };
        return Err(Error::new("PX1502", format!("'{id}' downloads models and needs {missing}")).with_hint(
            if s.cfg.allow_model_downloads {
                "ask the user to approve the download, then repeat the call with consent: [\"model-download\"]"
            } else {
                "ask the user to run the download themselves, or to restart the server with --allow-model-downloads"
            },
        ));
    }

    let mut g = s.call_globals();
    g.dry_run = req.dry_run;
    g.yes = downloads;
    let mut params = req.params;
    for key in GLOBAL_KEYS {
        if spec.find_param(key).is_some() {
            continue;
        }
        let Some(v) = params.remove(*key) else { continue };
        if v.is_null() {
            continue;
        }
        match *key {
            "overwrite" => g.overwrite = bool_value(key, &v)?,
            "dry_run" => g.dry_run |= bool_value(key, &v)?,
            "seed" => {
                g.seed = scalar_to_string(&v)?.trim().parse::<u64>().map_err(|_| {
                    Error::new("PX1010", "seed must be a non-negative integer").with_context("param", "seed")
                })?;
            }
            _ => g.format = Some(scalar_to_string(&v)?),
        }
    }
    schema::validate_params(spec, &Value::Object(params.clone()))?;

    // inputs and outputs
    let mut inputs = Vec::new();
    for key in ["input", "inputs"] {
        match params.remove(key) {
            None | Some(Value::Null) => {}
            Some(Value::String(p)) => inputs.push(s.sandbox_path(&p, Access::Read)?),
            Some(Value::Array(items)) => {
                for it in items {
                    let p = it
                        .as_str()
                        .ok_or_else(|| Error::new("PX1010", format!("{key} must be a path or a list of paths")))?;
                    inputs.push(s.sandbox_path(p, Access::Read)?);
                }
            }
            Some(_) => return Err(Error::new("PX1010", format!("{key} must be a path or a list of paths"))),
        }
    }
    if let Some(v) = params.remove("output") {
        if !v.is_null() {
            let p = v.as_str().ok_or_else(|| Error::new("PX1010", "output must be a path"))?;
            g.output = Some(s.sandbox_path(p, Access::Write)?);
        }
    }
    if let Some(v) = params.remove("in_place") {
        if !v.is_null() {
            g.in_place = bool_value("in_place", &v)?;
        }
    }
    // path-valued params stay inside the sandbox too
    for (k, v) in params.iter_mut() {
        let Some(p) = spec.find_param(k) else { continue };
        let check = |raw: &str| -> Result<Option<String>> {
            match p.kind {
                ParamKind::Path => s.sandbox_path(raw, Access::Read).map(Some),
                ParamKind::Mask if looks_like_file(raw) => s.sandbox_path(raw, Access::Read).map(Some),
                _ => Ok(None),
            }
        };
        match v {
            Value::String(raw) => {
                if let Some(np) = check(raw)? {
                    *raw = np;
                }
            }
            Value::Array(items) => {
                for it in items.iter_mut() {
                    if let Value::String(raw) = it {
                        if let Some(np) = check(raw)? {
                            *raw = np;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let pairs = json_to_pairs(&Value::Object(params))?;
    let argv = cli_argv(spec, &inputs, &pairs, &g);
    Ok(Invocation { command: spec.name.to_string(), args: pairs, inputs, globals: g, argv })
}

/// A mask reference that names a file rather than a mask (mirrors `OpCtx::resolve_mask`).
fn looks_like_file(r: &str) -> bool {
    let r = r.trim();
    if r.is_empty() || r.starts_with('@') {
        return false;
    }
    let p = Path::new(r);
    p.extension().is_some() || r.contains('/') || r.contains('\\') || r.starts_with('~') || p.exists()
}

/// The equivalent command line, recorded in sidecars.
fn cli_argv(spec: &OpSpec, inputs: &[String], pairs: &[(String, String)], g: &pixelox_core::Globals) -> Vec<String> {
    let mut v = vec!["pixelox".to_string()];
    v.extend(spec.name.split(' ').map(String::from));
    v.extend(inputs.iter().cloned());
    for (k, val) in pairs {
        let name = k.replace('_', "-");
        let flag = spec.find_param(&name).map(|p| p.kind == ParamKind::Flag).unwrap_or(false);
        if flag {
            if parse_bool(val).unwrap_or(false) {
                v.push(format!("--{name}"));
            }
        } else {
            v.push(format!("--{name}"));
            v.push(val.clone());
        }
    }
    if let Some(o) = &g.output {
        v.extend(["-o".to_string(), o.clone()]);
    }
    for (on, flag) in
        [(g.in_place, "--in-place"), (g.overwrite, "--overwrite"), (g.dry_run, "--dry-run"), (g.yes, "--yes")]
    {
        if on {
            v.push(flag.into());
        }
    }
    if g.seed != 0 {
        v.extend(["--seed".to_string(), g.seed.to_string()]);
    }
    if let Some(f) = &g.format {
        v.extend(["--format".to_string(), f.clone()]);
    }
    v
}

fn describe(s: &Server, progress: &dyn Progress, cancel: CancelToken) -> ToolOutcome {
    let Some(spec) = s.registry.get("describe") else {
        return ToolOutcome::failure(
            "describe",
            Error::new(
                "PX1001",
                "the 'describe' command is not registered in this build, so the agent briefing is unavailable",
            )
            .with_hint(
                "use pixelox_schema and pixelox_docs with a command id, or read the pixelox://docs/... resources",
            ),
        );
    };
    let args =
        if spec.find_param("agent").is_some() { vec![("agent".to_string(), "true".to_string())] } else { vec![] };
    let env = invoke_plain(s, spec, args, progress, cancel);
    if env.ok {
        ToolOutcome::text(result_text(&env.result))
    } else {
        ToolOutcome::envelope(&env)
    }
}

fn info(s: &Server, obj: &Map<String, Value>, progress: &dyn Progress, cancel: CancelToken) -> ToolOutcome {
    let input = match str_arg(obj, "input") {
        Ok(i) => i,
        Err(e) => return ToolOutcome::failure("info", e),
    };
    if s.registry.get("info").is_none() {
        return ToolOutcome::failure(
            "info",
            Error::new("PX1001", "the 'info' command is not registered in this build")
                .with_hint("use pixelox_preview to look at the file"),
        );
    }
    let mut params = Map::new();
    params.insert("input".into(), Value::String(input.to_string()));
    run_request(s, RunRequest { command: "info".into(), params, dry_run: false, consent: vec![] }, progress, cancel)
}

/// Split a convenience tool's arguments into `pixelox_run` parts: `dry_run` and `consent`
/// are lifted out, a nested `params` object is merged, the rest are params.
fn split_args(obj: &Map<String, Value>, skip: &[&str]) -> Result<(Map<String, Value>, bool, Vec<String>)> {
    let mut params = Map::new();
    for (k, v) in obj {
        if skip.contains(&k.as_str()) || matches!(k.as_str(), "dry_run" | "consent") {
            continue;
        }
        if k == "params" {
            match v {
                Value::Object(m) => params.extend(m.clone()),
                Value::Null => {}
                _ => return Err(Error::new("PX1010", "params must be a JSON object")),
            }
            continue;
        }
        params.insert(k.clone(), v.clone());
    }
    let dry_run = match obj.get("dry_run") {
        None | Some(Value::Null) => false,
        Some(v) => bool_value("dry_run", v)?,
    };
    Ok((params, dry_run, consent_list(obj.get("consent"))?))
}

fn detect(s: &Server, obj: &Map<String, Value>, progress: &dyn Progress, cancel: CancelToken) -> ToolOutcome {
    let kind = match str_arg(obj, "kind") {
        Ok(k) => k.trim().to_string(),
        Err(e) => return ToolOutcome::failure("detect", e),
    };
    let command = format!("detect {kind}");
    if s.registry.get(&command).is_none() || kind.contains([' ', '.', '_']) {
        let kinds = detect_kinds(s);
        return ToolOutcome::failure(
            &command.replace(' ', "."),
            Error::new("PX1001", format!("unknown detect kind '{kind}'"))
                .with_hint(format!("kind is one of: {}", kinds.join(", ")))
                .with_context("kinds", kinds),
        );
    }
    match split_args(obj, &["kind"]) {
        Ok((params, dry_run, consent)) => {
            run_request(s, RunRequest { command, params, dry_run, consent }, progress, cancel)
        }
        Err(e) => ToolOutcome::failure(&command.replace(' ', "."), e),
    }
}

fn jobs(s: &Server, obj: &Map<String, Value>, progress: &dyn Progress, cancel: CancelToken) -> ToolOutcome {
    let action = match str_arg(obj, "action") {
        Ok(a) if JOB_ACTIONS.contains(&a) => a,
        Ok(a) => {
            return ToolOutcome::failure(
                "jobs",
                Error::new("PX1010", format!("unknown jobs action '{a}'"))
                    .with_hint(format!("action is one of: {}", JOB_ACTIONS.join(", "))),
            )
        }
        Err(e) => return ToolOutcome::failure("jobs", e),
    };
    let command = format!("jobs {action}");
    let cid = command.replace(' ', ".");
    let Some(spec) = s.registry.get(&command) else {
        return ToolOutcome::failure(
            &cid,
            Error::new("PX1001", format!("the '{command}' command is not registered in this build")),
        );
    };
    let (mut params, dry_run, consent) = match split_args(obj, &["action", "id"]) {
        Ok(p) => p,
        Err(e) => return ToolOutcome::failure(&cid, e),
    };
    match opt_str(obj, "id") {
        Ok(Some(id)) => {
            params.insert(
                positional_param(spec, &["id", "job", "job-id"]).replace('-', "_"),
                Value::String(id.to_string()),
            );
        }
        Ok(None) if action != "list" => {
            return ToolOutcome::failure(
                &cid,
                Error::new("PX1003", format!("jobs {action} needs an id"))
                    .with_hint("list jobs with pixelox_jobs {\"action\": \"list\"}"),
            );
        }
        Ok(None) => {}
        Err(e) => return ToolOutcome::failure(&cid, e),
    }
    run_request(s, RunRequest { command, params, dry_run, consent }, progress, cancel)
}

/// The reference page for a command id or error code: the `docs` command when this build
/// has it, else a page rendered from the op spec or the error catalog.
pub(crate) fn docs_text(s: &Server, topic: &str) -> Result<String> {
    let topic = topic.trim();
    if let Some(spec) = s.registry.get("docs") {
        let key = positional_param(spec, &["command", "topic", "id"]);
        let env = invoke_plain(s, spec, vec![(key, topic.to_string())], &NoProgress, CancelToken::default());
        return match env.error {
            Some(e) => Err(e),
            None => Ok(result_text(&env.result)),
        };
    }
    let code = topic.trim_start_matches("errors/").to_ascii_uppercase();
    if let Some(e) = error::catalog_entry(&code) {
        return Ok(format!(
            "# {}: {}\n\nExit code {} ({}).\n\n{}\n",
            e.code,
            e.title,
            e.kind.exit_code(),
            e.kind.describe(),
            e.remedy
        ));
    }
    Ok(render_page(s.registry.require(topic)?))
}

/// A minimal reference page from an op spec (used when `docs` is not registered).
fn render_page(spec: &OpSpec) -> String {
    let mut md = format!("# pixelox {}\n\n{}\n", spec.name, spec.summary);
    if !spec.about.is_empty() {
        md.push_str(&format!("\n{}\n", spec.about));
    }
    if let Some(m) = spec.milestone {
        md.push_str(&format!("\nAvailable from {m}.\n"));
    }
    if !spec.params.is_empty() {
        md.push_str("\n## Parameters\n\n");
        for p in &spec.params {
            md.push_str(&format!("- `--{}` ({})", p.name, p.kind.type_name()));
            if let Some(d) = p.default {
                md.push_str(&format!(", default `{d}`"));
            }
            if !p.choices.is_empty() {
                md.push_str(&format!(", one of {}", p.choices.join(", ")));
            }
            md.push_str(&format!(": {}\n", p.help));
        }
    }
    if !spec.examples.is_empty() {
        md.push_str("\n## Examples\n\n");
        for e in &spec.examples {
            md.push_str(&format!("{}:\n\n```\n{}\n```\n\n", e.explain, e.cmd));
        }
    }
    md
}

// ---------------------------------------------------------------------------------------
// pixelox_preview

fn preview(s: &Server, obj: &Map<String, Value>) -> Result<ToolOutcome> {
    if let Some(k) = obj.keys().find(|k| !matches!(k.as_str(), "input" | "layer" | "region" | "max_size")) {
        return Err(Error::new("PX1002", format!("pixelox_preview does not take '{k}'"))
            .with_hint("accepted: input, layer, region, max_size"));
    }
    let input = s.sandbox_path(str_arg(obj, "input")?, Access::Read)?;
    let layer = opt_str(obj, "layer")?;
    let region = opt_str(obj, "region")?;
    let max_size = match obj.get("max_size") {
        None | Some(Value::Null) => PREVIEW_DEFAULT,
        Some(v) => {
            let n = scalar_to_string(v)?
                .trim()
                .parse::<u32>()
                .map_err(|_| Error::new("PX1010", "max_size must be a positive integer"))?;
            if n < PREVIEW_RANGE.0 || n > PREVIEW_RANGE.1 {
                return Err(Error::new(
                    "PX1013",
                    format!("max_size {n} is outside {}..{}", PREVIEW_RANGE.0, PREVIEW_RANGE.1),
                )
                .with_context("param", "max_size"));
            }
            n
        }
    };
    let mut ctx = OpCtx::new("mcp preview", vec![PathBuf::from(&input)], s.call_globals(), s.io, &NoProgress);
    ctx.registry = Some(s.registry);
    let doc = ctx.load_document(Path::new(&input))?;
    if doc.width == 0 || doc.height == 0 {
        return Err(Error::new("PX2320", "the document has no pixels to preview"));
    }
    let canvas = RectI::new(0, 0, doc.width, doc.height);
    let mut img = match layer {
        Some(l) => {
            let path = doc.resolve(l)?;
            match compose::render_layer(&doc, doc.layer(&path))? {
                Some(r) => r.to_canvas(doc.width, doc.height),
                None => Image::new(doc.width, doc.height),
            }
        }
        None => compose::composite(&doc)?,
    };
    let mut rect = canvas;
    if let Some(r) = region {
        let spec = RectSpec::parse(r).map_err(|mut e| {
            e.message = format!("region: {}", e.message);
            e
        })?;
        rect = spec.resolve(doc.width as f64, doc.height as f64, doc.dpi as f64).round().intersect(&canvas);
        if rect.is_empty() {
            return Err(Error::new(
                "PX1011",
                format!("region {r} lies outside the {}x{} canvas", doc.width, doc.height),
            )
            .with_context("region", r));
        }
        img = img.crop(rect);
    }
    let (w, h) = (img.width, img.height);
    let longest = w.max(h);
    let (nw, nh) = if longest > max_size {
        let k = max_size as f64 / longest as f64;
        (((w as f64 * k).round() as u32).max(1), ((h as f64 * k).round() as u32).max(1))
    } else {
        (w, h)
    };
    if (nw, nh) != (w, h) {
        img = kernels::resize(&img, nw, nh, Filter::auto(longest, max_size));
    }
    let png = encode_png(s, &img)?;
    let summary = json!({
        "input": input,
        "document": {"size": [doc.width, doc.height], "layers": doc.layers.len()},
        "layer": layer,
        "region": rect.to_json(),
        "preview": {"size": [nw, nh], "scale": nw as f64 / w as f64, "format": "png", "bytes": png.len()},
        "warnings": ctx.warnings,
    });
    Ok(ToolOutcome {
        result: json!({
            "content": [{"type": "image", "data": base64(&png), "mimeType": "image/png"}, {"type": "text", "text": summary.to_string()}],
            "structuredContent": summary,
            "isError": false,
        }),
        ok: true,
        code: None,
    })
}

/// Encode through the IO layer into a private temp file, read it back, delete it.
fn encode_png(s: &Server, img: &Image) -> Result<Vec<u8>> {
    let tmp = std::env::temp_dir().join(format!("pixelox-mcp-preview-{}-{}.png", std::process::id(), s.next_tmp()));
    let res = s.io.encode(img, None, &tmp, &EncodeOptions { format: "png".into(), ..Default::default() });
    let bytes = res.and_then(|_| std::fs::read(&tmp).map_err(Error::from));
    let _ = std::fs::remove_file(&tmp);
    bytes
}
