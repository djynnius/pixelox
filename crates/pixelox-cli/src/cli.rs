//! Command-line parsing: the clap tree is generated from the registry (ADR 0001), so every
//! command's flags, help, defaults, and choices come from its `OpSpec`.

use crate::config::Config;
use crate::progress::StderrProgress;
use clap::{Arg, ArgAction, ArgMatches, Command};
use pixelox_core::ctx::Globals;
use pixelox_core::envelope::Envelope;
use pixelox_core::exec::{Invocation, Runtime};
use pixelox_core::op::{OpSpec, ParamKind, ParamSpec};
use pixelox_core::{units, Error, Registry, Result};
use std::collections::BTreeSet;
use std::io::{IsTerminal, Write};
use std::path::PathBuf;

/// Global flags (PRD §8.1). No op parameter may reuse these names.
pub const GLOBAL_FLAGS: &[&str] = &[
    "output",
    "format",
    "json",
    "dry-run",
    "overwrite",
    "in-place",
    "seed",
    "yes",
    "quiet",
    "verbose",
    "threads",
    "mem-budget",
    "progress",
    "no-sidecar",
    "background",
    "config",
    "redact-paths",
    "limit-pixels",
];

/// Global flags that take no value (for argv scanning in tests and batches).
pub const GLOBAL_SWITCHES: &[&str] = &[
    "json",
    "dry-run",
    "overwrite",
    "in-place",
    "yes",
    "quiet",
    "verbose",
    "no-sidecar",
    "background",
    "redact-paths",
];

const ABOUT: &str = "pixelox: a command-line raster and vector graphics workstation for scripts and AI agents.\n\nEvery command takes --json for one machine-readable envelope on stdout, --dry-run to plan without writing, and writes a provenance sidecar (<output>.pixelox.json) next to each output. Nothing is overwritten without --overwrite; source images are never edited in place.\n\nStart with `pixelox describe --agent` (the whole surface, compactly) or `pixelox docs <command>`.";

fn global_args() -> Vec<Arg> {
    let g = |a: Arg| a.global(true).help_heading("Global options");
    vec![
        g(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("PATH")
            .allow_hyphen_values(true)
            .help("Output path; the extension picks the format (.pxo keeps layers)")),
        g(Arg::new("format").long("format").value_name("FMT").help("Output format when the extension is ambiguous")),
        g(Arg::new("json").long("json").action(ArgAction::SetTrue).help("One JSON envelope on stdout, nothing else")),
        g(Arg::new("dry-run")
            .long("dry-run")
            .action(ArgAction::SetTrue)
            .help("Validate and return the plan; write nothing")),
        g(Arg::new("overwrite")
            .long("overwrite")
            .action(ArgAction::SetTrue)
            .help("Allow replacing an existing output")),
        g(Arg::new("in-place")
            .long("in-place")
            .action(ArgAction::SetTrue)
            .help("Rewrite the input .pxo document instead of writing -o")),
        g(Arg::new("seed").long("seed").value_name("N|random").help("Seed for every random draw [default: 0]")),
        g(Arg::new("yes")
            .short('y')
            .long("yes")
            .action(ArgAction::SetTrue)
            .help("Consent to network actions (model downloads) for this run")),
        g(Arg::new("quiet").short('q').long("quiet").action(ArgAction::SetTrue).help("Only errors on stderr")),
        g(Arg::new("verbose").short('v').long("verbose").action(ArgAction::Count).help("More logging on stderr")),
        g(Arg::new("threads").long("threads").value_name("N").help("Worker threads [default: all cores]")),
        g(Arg::new("mem-budget").long("mem-budget").value_name("MB").help("Memory budget [default: 4096]")),
        g(Arg::new("progress")
            .long("progress")
            .value_name("MODE")
            .value_parser(["auto", "bar", "json", "none"])
            .help("Progress on stderr [default: auto]")),
        g(Arg::new("no-sidecar")
            .long("no-sidecar")
            .action(ArgAction::SetTrue)
            .help("Do not write <output>.pixelox.json")),
        g(Arg::new("background")
            .long("background")
            .action(ArgAction::SetTrue)
            .help("Run as a background job; prints its id")),
        g(Arg::new("config").long("config").value_name("PATH").help("Alternate config file")),
        g(Arg::new("redact-paths")
            .long("redact-paths")
            .action(ArgAction::SetTrue)
            .help("Replace directories with hashes in sidecars")),
        g(Arg::new("limit-pixels")
            .long("limit-pixels")
            .value_name("N")
            .help("Refuse images larger than N pixels [default: 512000000]")),
    ]
}

fn param_help(p: &ParamSpec) -> String {
    let mut h = p.help.to_string();
    if p.kind == ParamKind::Choice && !p.choices.is_empty() && p.choices.len() <= 12 {
        h.push_str(&format!(" [{}]", p.choices.join("|")));
    }
    if let Some(r) = p.range {
        h.push_str(&format!(" (range {r})"));
    }
    if let Some(d) = p.default {
        h.push_str(&format!(" [default: {d}]"));
    }
    if p.repeatable {
        h.push_str(" (repeatable)");
    }
    h
}

fn op_args(op: &OpSpec) -> Vec<Arg> {
    let mut args = Vec::new();
    for p in &op.params {
        if GLOBAL_FLAGS.contains(&p.name) || op.positionals.contains(&p.name) {
            continue;
        }
        let mut a = Arg::new(p.name).long(p.name).help(param_help(p));
        if let Some(s) = p.short {
            a = a.short(s);
        }
        a = match p.kind {
            ParamKind::Flag => a.action(ArgAction::SetTrue),
            ParamKind::Switch => {
                a.num_args(0..=1).default_missing_value("on").value_name("on|off").action(ArgAction::Set)
            }
            _ => {
                let a = a
                    .num_args(1)
                    .allow_hyphen_values(true)
                    .value_name(p.kind.type_name().to_uppercase().replace(' ', "_"));
                if p.repeatable {
                    a.action(ArgAction::Append)
                } else {
                    a.action(ArgAction::Set)
                }
            }
        };
        if p.advanced {
            a = a.hide_short_help(true);
        }
        args.push(a);
    }
    if op.inputs.max != Some(0) || !op.positionals.is_empty() {
        let mut names: Vec<String> = Vec::new();
        if op.inputs.max != Some(0) {
            names.push(if op.inputs.max == Some(1) { "INPUT".into() } else { "INPUTS".into() });
        }
        names.extend(op.positionals.iter().map(|p| p.to_uppercase()));
        args.push(
            Arg::new("__words")
                .num_args(0..)
                .action(ArgAction::Append)
                .value_name(names.join(" "))
                .help("Input file(s), then positional values"),
        );
    }
    args
}

fn op_long_about(op: &OpSpec) -> String {
    let mut s = op.summary.to_string();
    if let Some(m) = op.milestone {
        s.push_str(&format!(" [{m}]"));
    }
    if !op.about.is_empty() {
        s.push_str("\n\n");
        s.push_str(op.about);
    }
    if !op.examples.is_empty() {
        s.push_str("\n\nExamples:");
        for e in &op.examples {
            s.push_str(&format!("\n  {}\n      {}", e.cmd, e.explain));
        }
    }
    s.push_str(&format!("\n\nSchema: pixelox schema {}   Reference: pixelox docs {}", op.id(), op.id()));
    s
}

/// Build the full clap tree from the registry.
pub fn build_command(reg: &Registry) -> Command {
    let paths: BTreeSet<Vec<&'static str>> = reg.ops().iter().map(|o| o.name.split(' ').collect()).collect();
    let mut prefixes: BTreeSet<Vec<&'static str>> = BTreeSet::new();
    for p in &paths {
        for i in 1..=p.len() {
            prefixes.insert(p[..i].to_vec());
        }
    }
    fn node(prefix: Vec<&'static str>, reg: &Registry, prefixes: &BTreeSet<Vec<&'static str>>) -> Command {
        let name = *prefix.last().unwrap();
        let joined = prefix.join(" ");
        let mut cmd = Command::new(name);
        let children: Vec<Vec<&'static str>> = prefixes
            .iter()
            .filter(|p| p.len() == prefix.len() + 1 && p[..prefix.len()] == prefix[..])
            .cloned()
            .collect();
        if let Some(op) = reg.ops().iter().find(|o| o.name == joined) {
            cmd = cmd.about(op.summary).long_about(op_long_about(op)).args(op_args(op));
            for a in &op.aliases {
                let words: Vec<&str> = a.split(' ').collect();
                if words.len() == prefix.len() && words[..words.len() - 1] == prefix[..prefix.len() - 1] {
                    cmd = cmd.visible_alias(words[words.len() - 1]);
                }
            }
            if !children.is_empty() {
                cmd = cmd.args_conflicts_with_subcommands(true).subcommand_negates_reqs(true);
            }
        } else {
            let kids: Vec<String> = children.iter().map(|c| c.last().unwrap().to_string()).collect();
            cmd = cmd
                .about(format!("{name}: {}", kids.join(", ")))
                .subcommand_required(true)
                .arg_required_else_help(true);
        }
        for c in children {
            cmd = cmd.subcommand(node(c, reg, prefixes));
        }
        cmd
    }
    let mut root = Command::new("pixelox")
        .version(pixelox_core::PIXELOX_VERSION)
        .about(ABOUT)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .max_term_width(100)
        .args(global_args());
    for p in prefixes.iter().filter(|p| p.len() == 1) {
        root = root.subcommand(node(p.clone(), reg, &prefixes));
    }
    root
}

fn leaf(m: &ArgMatches) -> (Vec<String>, &ArgMatches) {
    let mut path = Vec::new();
    let mut cur = m;
    while let Some((name, sub)) = cur.subcommand() {
        path.push(name.to_string());
        cur = sub;
    }
    (path, cur)
}

fn get_str(m: &ArgMatches, id: &str) -> Option<String> {
    m.try_get_one::<String>(id).ok().flatten().cloned()
}

fn get_flag(m: &ArgMatches, id: &str) -> bool {
    m.try_get_one::<bool>(id).ok().flatten().copied().unwrap_or(false)
}

/// Output preferences (how to print; not part of the invocation).
#[derive(Clone, Debug)]
pub struct Prefs {
    pub json: bool,
    pub progress: String,
    pub verbosity: i32,
    pub background: bool,
}

fn random_seed() -> u64 {
    use std::hash::{BuildHasher, Hasher};
    let mut h = std::collections::hash_map::RandomState::new().build_hasher();
    h.write_u128(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0));
    h.finish()
}

/// Global flags with precedence flag > env > config > default.
pub fn globals_from(m: &ArgMatches, cfg: &Config) -> Result<(Globals, Prefs)> {
    let mut g = Globals {
        output: get_str(m, "output"),
        format: get_str(m, "format"),
        overwrite: get_flag(m, "overwrite"),
        in_place: get_flag(m, "in-place"),
        dry_run: get_flag(m, "dry-run"),
        ..Default::default()
    };
    match get_str(m, "seed").as_deref() {
        None => {}
        Some("random") => {
            g.seed = random_seed();
            g.seed_drawn = true;
        }
        Some(s) => {
            g.seed = s
                .trim()
                .parse::<u64>()
                .map_err(|_| Error::new("PX1010", format!("--seed must be a whole number or 'random', got '{s}'")))?
        }
    }
    g.yes = get_flag(m, "yes") || cfg.get_bool("models.auto_consent").unwrap_or(false);
    let num = |v: Option<String>, name: &str| -> Result<Option<u64>> {
        match v {
            None => Ok(None),
            Some(s) => s
                .trim()
                .parse::<u64>()
                .map(Some)
                .map_err(|_| Error::new("PX1010", format!("--{name} must be a whole number, got '{s}'"))),
        }
    };
    g.threads = num(get_str(m, "threads").or_else(|| cfg.get("threads")), "threads")?.unwrap_or(0) as usize;
    g.mem_budget_mb = match get_str(m, "mem-budget").or_else(|| cfg.get("mem_budget_mb")) {
        Some(s) => units::parse_megabytes(&s)?,
        None => 4096,
    };
    g.sidecar = !get_flag(m, "no-sidecar") && cfg.get_bool("sidecar").unwrap_or(true);
    g.redact_paths = get_flag(m, "redact-paths") || cfg.get_bool("redact_paths").unwrap_or(false);
    g.config_path = get_str(m, "config").map(PathBuf::from);
    if let Some(lp) = num(get_str(m, "limit-pixels").or_else(|| cfg.get("limit_pixels")), "limit-pixels")? {
        g.open.limit_pixels = lp;
    }
    let quiet = get_flag(m, "quiet");
    let verbose = m.try_get_one::<u8>("verbose").ok().flatten().copied().unwrap_or(0) as i32;
    g.verbose = if quiet { -1 } else { verbose };
    let prefs = Prefs {
        json: get_flag(m, "json"),
        progress: get_str(m, "progress").or_else(|| cfg.get("progress")).unwrap_or_else(|| "auto".into()),
        verbosity: g.verbose,
        background: get_flag(m, "background"),
    };
    if prefs.json && g.output.as_deref() == Some("-") {
        return Err(Error::new("PX1005", "--json and -o - both want stdout; write to a file or drop --json"));
    }
    Ok((g, prefs))
}

/// Turn parsed matches into an invocation (command, op args, inputs, globals).
pub fn invocation_from(
    reg: &Registry,
    matches: &ArgMatches,
    cfg: &Config,
    argv: &[String],
) -> Result<(Invocation, Prefs)> {
    let (path, leaf_m) = leaf(matches);
    let command = path.join(" ");
    let op = reg.require(&command)?;
    let (globals, prefs) = globals_from(leaf_m, cfg)?;
    let mut raw: Vec<(String, String)> = Vec::new();
    for p in &op.params {
        if op.positionals.contains(&p.name) || GLOBAL_FLAGS.contains(&p.name) {
            continue;
        }
        match p.kind {
            ParamKind::Flag => {
                if get_flag(leaf_m, p.name) {
                    raw.push((p.name.to_string(), "true".into()));
                }
            }
            _ if p.repeatable => {
                if let Ok(Some(vals)) = leaf_m.try_get_many::<String>(p.name) {
                    for v in vals {
                        raw.push((p.name.to_string(), v.clone()));
                    }
                }
            }
            _ => {
                if let Some(v) = get_str(leaf_m, p.name) {
                    raw.push((p.name.to_string(), v));
                }
            }
        }
    }
    let words: Vec<String> =
        leaf_m.try_get_many::<String>("__words").ok().flatten().map(|v| v.cloned().collect()).unwrap_or_default();
    let (inputs, rest) = if op.positionals.is_empty() {
        (words, Vec::new())
    } else {
        let n = op.inputs.max.unwrap_or(0).min(words.len());
        (words[..n].to_vec(), words[n..].to_vec())
    };
    for (i, w) in rest.iter().enumerate() {
        let last = i + 1 >= op.positionals.len();
        let name = op.positionals[i.min(op.positionals.len() - 1)];
        let p = op.find_param(name);
        let multi = p.map(|p| p.repeatable || p.kind == ParamKind::List).unwrap_or(false);
        if last && !multi && i + 1 < rest.len() {
            // join the remaining words (e.g. `pixelox docs comp template apply`)
            raw.push((name.to_string(), rest[i..].join(" ")));
            break;
        }
        raw.push((name.to_string(), w.clone()));
    }
    let inv = Invocation {
        command: op.name.to_string(),
        args: raw,
        inputs,
        globals,
        argv: argv.iter().skip(1).cloned().collect(),
    };
    Ok((inv, prefs))
}

/// Parse a full argv (`["pixelox", "filter", "gaussian", ...]`) without running it.
pub fn parse_argv(reg: &Registry, argv: &[String]) -> Result<(Invocation, Prefs)> {
    let m = build_command(reg).try_get_matches_from(argv).map_err(|e| {
        Error::new(
            "PX1002",
            e.to_string().lines().next().unwrap_or("invalid arguments").trim_start_matches("error: ").to_string(),
        )
    })?;
    let cfg = Config::load(None).unwrap_or_default();
    invocation_from(reg, &m, &cfg, argv)
}

pub fn print_envelope(env: &Envelope, prefs: &Prefs) {
    let out = std::io::stdout();
    let mut out = out.lock();
    if prefs.json {
        let text = if out.is_terminal() { serde_json::to_string_pretty(env) } else { serde_json::to_string(env) };
        let _ = writeln!(out, "{}", text.unwrap_or_default());
        return;
    }
    if let Some(e) = &env.error {
        eprintln!("error: {}: {}", e.code, e.message);
        if let Some(h) = &e.hint {
            eprintln!("hint: {h}");
        }
        return;
    }
    if prefs.verbosity < 0 {
        return;
    }
    if let Some(t) = env.result.get("text").and_then(|t| t.as_str()) {
        let _ = write!(out, "{t}");
        if !t.ends_with('\n') {
            let _ = writeln!(out);
        }
        return;
    }
    for o in &env.outputs {
        match &o.action {
            Some(a) => {
                let _ = writeln!(out, "{} (would {a})", o.path);
            }
            None => {
                let _ = writeln!(out, "{}", o.path);
            }
        }
    }
    if env.outputs.is_empty() || env.dry_run {
        let mut r = env.result.clone();
        if let Some(o) = r.as_object_mut() {
            o.remove("params");
        }
        if !r.is_null() {
            let _ = writeln!(out, "{}", serde_json::to_string_pretty(&r).unwrap_or_default());
        }
    }
}

fn error_exit(command: &str, e: Error, json: bool) -> i32 {
    let code = e.exit_code;
    let env = Envelope::failure(command, e, 0, false, 0);
    print_envelope(&env, &Prefs { json, progress: "none".into(), verbosity: 0, background: false });
    code
}

/// `pixelox <command> --help --json` prints the command's schema instead of text help.
fn help_json(reg: &Registry, argv: &[String]) -> Option<i32> {
    let has_help = argv.iter().any(|a| a == "--help" || a == "-h");
    let has_json = argv.iter().any(|a| a == "--json");
    if !(has_help && has_json) {
        return None;
    }
    let words: Vec<&str> = argv.iter().skip(1).filter(|a| !a.starts_with('-')).map(|s| s.as_str()).collect();
    for n in (1..=words.len().min(3)).rev() {
        let name = words[..n].join(" ");
        if let Some(op) = reg.ops().iter().find(|o| o.name == name) {
            println!("{}", serde_json::to_string_pretty(&pixelox_core::schema::command_schema(op)).unwrap_or_default());
            return Some(0);
        }
    }
    let list: Vec<serde_json::Value> =
        reg.ops().iter().map(|o| serde_json::json!({"command": o.id(), "summary": o.summary})).collect();
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({ "commands": list })).unwrap_or_default());
    Some(0)
}

pub fn run(argv: Vec<String>) -> i32 {
    let reg = crate::registry();
    if let Some(code) = help_json(&reg, &argv) {
        return code;
    }
    let wants_json = argv.iter().any(|a| a == "--json");
    let matches = match build_command(&reg).try_get_matches_from(&argv) {
        Ok(m) => m,
        Err(e) => {
            use clap::error::ErrorKind as K;
            return match e.kind() {
                K::DisplayHelp | K::DisplayVersion | K::DisplayHelpOnMissingArgumentOrSubcommand => {
                    let _ = e.print();
                    0
                }
                _ if wants_json => {
                    let msg = e.to_string();
                    let first =
                        msg.lines().next().unwrap_or("invalid arguments").trim_start_matches("error: ").to_string();
                    let code = if msg.contains("unrecognized subcommand") { "PX1001" } else { "PX1002" };
                    error_exit("pixelox", Error::new(code, first), true)
                }
                _ => {
                    let _ = e.print();
                    2
                }
            };
        }
    };
    let cfg_path = {
        let (_, l) = leaf(&matches);
        get_str(l, "config").map(PathBuf::from)
    };
    let cfg = match Config::load(cfg_path.as_deref()) {
        Ok(c) => c,
        Err(e) => return error_exit("pixelox", e, wants_json),
    };
    let (inv, prefs) = match invocation_from(&reg, &matches, &cfg, &argv) {
        Ok(x) => x,
        Err(e) => {
            let (path, _) = leaf(&matches);
            return error_exit(&path.join("."), e, wants_json);
        }
    };
    if inv.globals.threads > 0 {
        let _ = rayon::ThreadPoolBuilder::new().num_threads(inv.globals.threads).build_global();
    }
    if prefs.background {
        return crate::jobs::spawn_background(&argv, prefs.json);
    }
    let progress = StderrProgress::new(&prefs.progress, prefs.verbosity);
    let io = pixelox_io::Io;
    let rt = Runtime::new(&reg, &io, &progress);
    crate::jobs::install_interrupt_handler(&rt.cancel);
    let outcome = rt.invoke(inv);
    progress.finish();
    if let Some(bytes) = &outcome.stdout {
        let mut so = std::io::stdout().lock();
        let _ = so.write_all(bytes);
        let _ = so.flush();
    }
    print_envelope(&outcome.envelope, &prefs);
    outcome.envelope.exit_code()
}
