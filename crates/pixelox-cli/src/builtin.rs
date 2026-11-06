//! Contract-level commands: `schema`, `describe`, `docs`, `replay`, `verify`, `history *`,
//! `version`, `completions`, and `doctor` (`update` lives in update.rs).

use crate::docgen;
use pixelox_core::ctx::{is_pxo, OpCtx};
use pixelox_core::doc::Document;
use pixelox_core::envelope::{sha256_file, Sidecar, BUILD_GIT, BUILD_PROFILE, BUILD_TARGET, PIXELOX_VERSION};
use pixelox_core::exec::{replay_into, run_recipe, ReplayOptions};
use pixelox_core::op::{Inputs, OpSpec};
use pixelox_core::prelude::*;
use pixelox_core::{schema, Registry};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::custom("schema", Category::Agents, "JSON Schema of a command, or the command list", schema_op)
            .inputs(Inputs::NONE)
            .about("Prints the JSON Schema (draft 2020-12) of a command's parameters and envelope, plus the error codes it can raise. `sidecar` and `pipeline` print those formats; `--all` bundles every command.")
            .param(param("command", ParamKind::Str).help("Command (dotted id or words), `sidecar`, or `pipeline`"))
            .param(param("all", ParamKind::Flag).help("Every command's schema in one bundle"))
            .positional("command")
            .example("pixelox schema filter.gaussian", "Parameters and envelope of the Gaussian blur")
            .example("pixelox schema comp template apply --json", "A multi-word command")
            .example("pixelox schema --all --json", "Every schema in one document"),
        OpSpec::custom("describe", Category::Agents, "The whole command surface for agents", describe)
            .inputs(Inputs::NONE)
            .about("`--agent` prints the compact briefing: grammar, global flags, the envelope, addressing, five workflows, and one line per command (under 4,000 tokens). With --json the envelope's result also carries the machine index (ids, categories, modes, schema versions); --format markdown prints the catalog as Markdown.")
            .param(param("agent", ParamKind::Flag).help("The compact agent briefing (the default text)"))
            .example("pixelox describe --agent", "The briefing an agent reads once")
            .example("pixelox describe --json", "The briefing plus the machine index of every command")
            .example("pixelox describe --format markdown", "The catalog as Markdown"),
        OpSpec::custom("docs", Category::Agents, "Offline reference for a command, topic, or error", docs)
            .inputs(Inputs::NONE)
            .about("Prints a command's full page (synopsis, every flag, examples, errors), a topic (contract, addressing, pipelines, sidecars, mcp), the error catalog (`errors`), or one code (`errors/PX1401`). With no argument, lists everything.")
            .param(param("topic", ParamKind::List).help("Command (dotted id or words), topic, `errors`, or `errors/PX####`"))
            .positional("topic")
            .example("pixelox docs filter.gaussian", "The Gaussian blur page")
            .example("pixelox docs errors/PX1401", "What an error means and what to do")
            .example("pixelox docs contract", "The agent contract"),
        OpSpec::custom("docs build-site", Category::Agents, "Generate the docs site source (mdBook)", docs_build_site)
            .inputs(Inputs::NONE)
            .param(param("out", ParamKind::Path).default("target/docs-site").help("Directory for book.toml and src/"))
            .param(param("guides", ParamKind::Path).default("docs/guides").help("Hand-written guides copied into the site"))
            .param(param("check", ParamKind::Flag).help("Fail when a command is below the documentation bar"))
            .example("pixelox docs build-site --out target/docs-site", "Write the site source")
            .example("pixelox docs build-site --check", "Fail on missing examples or help")
            .example("pixelox docs build-site --guides docs/guides --out site --check", "Custom folders"),
        OpSpec::custom("replay", Category::Agents, "Rebuild an output from its sidecar recipe", replay)
            .about("Re-executes the sidecar's recipe from the recorded source files (hashes checked first; PX1601 on drift) and writes -o (default: the recorded output path, which needs --overwrite when it exists). Reports whether the result is bit-identical; --strict fails (exit 9) when it is not.")
            .param(param("input", ParamKind::Str).repeatable().help("Substitute an input: role=path (roles are in the sidecar's inputs)"))
            .param(param("allow-input-drift", ParamKind::Flag).help("Run even when an input's hash changed (warns)"))
            .param(param("strict", ParamKind::Flag).help("Require a bit-identical output"))
            .example("pixelox replay out.png.pixelox.json -o again.png", "Reproduce an output")
            .example("pixelox replay out.png.pixelox.json -o again.png --strict --json", "Fail unless bit-identical")
            .example("pixelox replay out.png.pixelox.json -o v2.png --input source=new-shot.jpg", "Same recipe, new photo"),
        OpSpec::custom("verify", Category::Agents, "Check an output against its sidecar", verify)
            .about("--fast (default): the output and every recorded input still match their SHA-256. --full: also replay into a temporary directory and compare (bit-exact on the same build, SSIM otherwise). Exit 9 with a report on mismatch.")
            .param(param("fast", ParamKind::Flag).help("Hash checks only (the default)"))
            .param(param("full", ParamKind::Flag).help("Replay into a temporary directory and compare"))
            .example("pixelox verify out.png.pixelox.json --json", "Confirm nothing changed")
            .example("pixelox verify out.png.pixelox.json --full --json", "Prove the recipe still reproduces it")
            .example("pixelox verify dist/poster.png.pixelox.json -q", "Exit code only"),
        OpSpec::doc("history list", Category::Agents, "A document's recipe, step by step", history_list)
            .read_only()
            .about("Every recorded step (op, params, inputs) and named checkpoints.")
            .example("pixelox history list doc.pxo --json", "The op history")
            .example("pixelox history list shot.pxo", "As text")
            .example("pixelox history list poster.pxo --json", "Which template filled it"),
        OpSpec::doc("history undo", Category::Agents, "Rebuild a document without its last steps", history_undo)
            .about("Replays the recipe without the last --steps steps (or back to a --to checkpoint) from the recorded sources, so any earlier state is recoverable.")
            .milestone("v0.2")
            .param(param("steps", ParamKind::Int).default("1").range("1..100000").help("How many steps to undo"))
            .param(param("to", ParamKind::Str).help("Go back to this checkpoint instead"))
            .example("pixelox history undo doc.pxo --in-place", "Undo the last step")
            .example("pixelox history undo doc.pxo --steps 3 -o earlier.pxo", "Three steps back, as a copy")
            .example("pixelox history undo doc.pxo --to before-grade --in-place", "Back to a checkpoint"),
        OpSpec::doc("history checkpoint", Category::Agents, "Name the current point in the history", history_checkpoint)
            .milestone("v0.2")
            .param(param("name", ParamKind::Str).required().help("Checkpoint name"))
            .example("pixelox history checkpoint doc.pxo --in-place --name before-grade", "Mark a point")
            .example("pixelox history checkpoint doc.pxo --in-place --name v1 --json", "As JSON")
            .example("pixelox history checkpoint doc.pxo -o marked.pxo --name draft", "Mark a copy"),
        OpSpec::custom("version", Category::System, "Version, build, and features", version)
            .inputs(Inputs::NONE)
            .example("pixelox version --json", "Version and build as JSON")
            .example("pixelox version", "As text")
            .example("pixelox --version", "Just the number"),
        OpSpec::custom("completions", Category::System, "Print a shell completion script", completions)
            .inputs(Inputs::NONE)
            .param(param("shell", ParamKind::Choice).choices(&["bash", "zsh", "fish", "powershell", "elvish"]).required().help("Shell"))
            .positional("shell")
            .example("pixelox completions zsh > ~/.zfunc/_pixelox", "zsh")
            .example("pixelox completions bash > ~/.local/share/bash-completion/completions/pixelox", "bash")
            .example("pixelox completions fish > ~/.config/fish/completions/pixelox.fish", "fish"),
        OpSpec::custom("doctor", Category::System, "Environment diagnostics: build, codecs, fonts, paths", doctor)
            .inputs(Inputs::NONE)
            .about("Build features and their gates, codecs, installed renderers, fonts found, models, effective configuration, and state directories. --migrate-pxo rewrites an older .pxo document in the current format (in place, keeping the original as <file>.bak).")
            .param(param("migrate-pxo", ParamKind::Path).help("Rewrite this .pxo in the current format version"))
            .example("pixelox doctor", "Check the installation")
            .example("pixelox doctor --json", "The same report as JSON")
            .example("pixelox doctor --config ./ci.toml --json", "With another config file")
            .example("pixelox doctor --migrate-pxo old.pxo", "Rewrite an old document in the current format"),
    ]
}

fn reg<'a>(ctx: &OpCtx<'a>) -> Result<&'a Registry> {
    ctx.registry()
}

fn schema_op(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let r = reg(ctx)?;
    if a.flag("all") {
        let all: Map<String, Value> = r.ops().iter().map(|o| (o.id(), schema::command_schema(o))).collect();
        return Ok(json!({"x-pixelox-schema-version": schema::SCHEMA_VERSION, "commands": all}));
    }
    match a.str("command").as_deref() {
        None => Ok(
            json!({"commands": r.ops().iter().map(|o| json!({"command": o.id(), "summary": o.summary, "category": o.category})).collect::<Vec<_>>()}),
        ),
        Some("sidecar") => Ok(sidecar_schema()),
        Some("pipeline") => Ok(crate::pipeline::pipeline_schema()),
        Some(c) => Ok(schema::command_schema(r.require(c)?)),
    }
}

fn sidecar_schema() -> Value {
    json!({
        "$schema": schema::DIALECT,
        "$id": format!("{}/sidecar.json", schema::BASE),
        "title": "pixelox provenance sidecar",
        "type": "object",
        "required": ["pixelox_sidecar", "tool", "created_utc", "base_dir", "command", "recipe", "inputs", "output", "env", "determinism"],
        "properties": {
            "pixelox_sidecar": {"const": 1},
            "tool": {"type": "object", "required": ["name", "version", "build", "target", "profile"]},
            "created_utc": {"type": "string"},
            "base_dir": {"type": "string"},
            "command": {"type": "object", "required": ["id", "argv", "params", "seed"]},
            "recipe": {"type": "array", "items": {"type": "object", "required": ["op", "params"], "properties": {
                "op": {"type": "string"}, "params": {"type": "object"},
                "inputs": {"type": "array", "items": {"type": "object", "required": ["role", "path", "sha256", "bytes"]}},
                "model": {"type": "object"}, "fonts": {"type": "array"}}}},
            "inputs": {"type": "array"},
            "models": {"type": "array"},
            "fonts": {"type": "array"},
            "output": {"type": "object", "required": ["path", "sha256", "bytes"]},
            "env": {"type": "object"},
            "determinism": {"type": "object"},
            "warnings": {"type": "array"}
        }
    })
}

// ---------------------------------------------------------------------------------------
// describe

const BRIEFING: &str = "pixelox: image editor CLI for agents. Grammar: pixelox <group> <cmd> INPUT [flags] (-o OUT | --in-place).
INPUT: image (jpg png tif webp gif bmp psd svg) or .pxo document; images open as one layer `Background`. -o x.pxo keeps layers; -o x.png flattens. --in-place only rewrites .pxo.
Always: --json (one envelope: ok, result, outputs[].path+sha256, warnings[], error{code,hint}); --dry-run before writing; --overwrite only when asked; -y only with download consent.
Globals: --seed N|random --threads N --progress json --no-sidecar --background --redact-paths --config F.
Address: --layer name|#N|@top|@bottom|@active; --mask name|@sel|@alpha:LAYER|@mask:LAYER|mask.png [--invert-mask]; X,Y,WxH / X,Y / WxH in px % pt mm in (negative = from right/bottom), --anchor tl..br; colors #rrggbb[aa] rgb() hsl() names transparent; stops '#000@0,#fff@1'.
Exit: 0 ok 1 runtime 2 usage 3 input 4 exists 5 format 6 consent 7 script 8 job 9 verify 10 interrupt 11 memory 12 not-in-build. Each output gets out.pixelox.json (replay/verify).
More than 3 steps: write a pipeline (pixelox docs pipelines). Check work: analyze diff A B; verify out.pixelox.json.
Workflows:
1 cutout: select color s.pxo --in-place --at 4,4 --contiguous --tolerance 40; select invert s.pxo --in-place; export s.pxo -o cut.png --apply-mask @sel
2 grade: adjust curves|levels|hsl|white-balance DOC --in-place ... (--as-layer = non-destructive)
3 poster: comp template apply t.pxo --set headline=X --set-image product=p.png -o out.pxo; export out.pxo -o out.png
4 type+art: new -o d.pxo --preset ig-post; shape star ...; text add d.pxo --in-place --content X --font F --at 50%,10% --anchor t; fx drop-shadow --layer L
5 batch: batch run --glob 'in/*.jpg' --cmd 'export {in} -o out/{stem}.webp' --jobs 8
Commands (INPUT implied unless shown; <x> = positional):
";

/// One line per command: name, unusual inputs, summary.
pub fn agent_catalog(reg: &Registry) -> Vec<(String, String)> {
    let mut ops: Vec<&OpSpec> = reg.ops().iter().collect();
    ops.sort_by_key(|o| (o.category, o.name));
    ops.iter()
        .map(|o| {
            let inputs = match (o.inputs.min, o.inputs.max) {
                (_, Some(0)) => "",
                (1, Some(1)) => "",
                (0, Some(1)) => " [IN]",
                (2, Some(2)) => " A B",
                _ => " FILES..",
            };
            let pos: String = o.positionals.iter().map(|p| format!(" <{p}>")).collect();
            // (ml): runs a local model (downloaded on first use with -y)
            let gate = if (o.category == Category::Detect && o.name != "detect list-models")
                || o.name == "fill content-aware"
            {
                " (ml)"
            } else {
                ""
            };
            (o.category.title().to_string(), format!("{}{inputs}{pos}: {}{gate}", o.name, o.summary))
        })
        .collect()
}

pub fn agent_briefing(reg: &Registry) -> String {
    let mut s = BRIEFING.to_string();
    for (_, l) in agent_catalog(reg) {
        s.push_str(&l);
        s.push('\n');
    }
    s
}

fn describe(_a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let r = reg(ctx)?;
    let fmt = ctx.globals.format.clone().unwrap_or_else(|| "text".into());
    let text = if fmt == "markdown" {
        let mut s = String::from("# pixelox agent catalog\n\n```text\n");
        s.push_str(BRIEFING);
        s.push_str("```\n");
        let mut cur = String::new();
        for (cat, line) in agent_catalog(r) {
            if cat != cur {
                s.push_str(&format!("\n## {cat}\n\n"));
                cur = cat;
            }
            s.push_str(&format!("- `{line}`\n"));
        }
        s
    } else {
        agent_briefing(r)
    };
    let approx_tokens = text.len() / 4;
    let index: Vec<Value> = r
        .ops()
        .iter()
        .map(|o| json!({"command": o.id(), "cli": o.name, "category": o.category, "summary": o.summary, "mode": o.mode, "milestone": o.milestone, "schema_version": schema::SCHEMA_VERSION}))
        .collect();
    Ok(json!({"text": text, "approx_tokens": approx_tokens, "commands": index}))
}

// ---------------------------------------------------------------------------------------
// docs

fn docs(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let r = reg(ctx)?;
    let what = a.list("topic").join(" ");
    let what = what.trim();
    let text = if what.is_empty() {
        let mut s = String::from("pixelox documentation (offline). `pixelox docs <command|topic>`.\n\nTopics: contract, addressing, pipelines, sidecars, mcp, errors\n\nCommands:\n");
        for (_, l) in agent_catalog(r) {
            s.push_str("  ");
            s.push_str(&l);
            s.push('\n');
        }
        s
    } else if what == "errors" {
        docgen::error_catalog()
    } else if let Some(code) =
        what.strip_prefix("errors/").or_else(|| if what.starts_with("PX") { Some(what) } else { None })
    {
        docgen::error_entry(code)
            .ok_or_else(|| Error::new("PX1001", format!("no error code '{code}'")).with_hint("pixelox docs errors"))?
    } else if let Some(t) = docgen::topic(what) {
        t.to_string()
    } else {
        docgen::command_page(r.require(what)?)
    };
    Ok(json!({ "text": text }))
}

fn docs_build_site(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let r = reg(ctx)?;
    let out = a.path("out").unwrap_or_else(|| PathBuf::from("target/docs-site"));
    let guides = a.path("guides").unwrap_or_else(|| PathBuf::from("docs/guides"));
    let problems = docgen::coverage_problems(r);
    if a.flag("check") && !problems.is_empty() {
        return Err(Error::new(
            "PX1010",
            format!("{} documentation problems:\n{}", problems.len(), problems.join("\n")),
        ));
    }
    if ctx.dry_run() {
        return Ok(json!({"out": out.display().to_string(), "problems": problems, "planned": true}));
    }
    let n = docgen::build_site(r, &guides, &out)?;
    Ok(json!({"out": out.display().to_string(), "command_pages": n, "problems": problems}))
}

// ---------------------------------------------------------------------------------------
// replay and verify

fn overrides(a: &Args) -> Result<BTreeMap<String, String>> {
    let mut m = BTreeMap::new();
    for s in a.all("input") {
        let (k, v) =
            s.split_once('=').ok_or_else(|| Error::new("PX1010", format!("--input expects role=path, got '{s}'")))?;
        m.insert(k.trim().to_string(), v.trim().to_string());
    }
    Ok(m)
}

fn resolve_recorded(p: &str, base: &str) -> PathBuf {
    let pb = PathBuf::from(p);
    if pb.exists() || pb.is_absolute() || base.is_empty() {
        return pb;
    }
    let b = Path::new(base).join(&pb);
    if b.exists() {
        b
    } else {
        pb
    }
}

fn perceptual(ctx: &mut OpCtx, a: &Path, b: &Path) -> Option<f64> {
    let (ia, _) = ctx.io.decode(&std::fs::read(a).ok()?, None, &Default::default()).ok()?;
    let (ib, _) = ctx.io.decode(&std::fs::read(b).ok()?, None, &Default::default()).ok()?;
    if (ia.width, ia.height) != (ib.width, ib.height) {
        return Some(0.0);
    }
    Some(kernels::ssim(&ia, &ib))
}

fn replay(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let sc_path = ctx.input(0)?.to_path_buf();
    ctx.check_allowed(&sc_path)?;
    let sc = Sidecar::read(&sc_path)?;
    let out = match &ctx.globals.output {
        Some(o) => PathBuf::from(o),
        None => resolve_recorded(&sc.output.path, &sc.base_dir),
    };
    let opts = ReplayOptions {
        base_dir: Some(PathBuf::from(&sc.base_dir)),
        overrides: overrides(a)?,
        allow_drift: a.flag("allow-input-drift"),
    };
    let r = replay_into(ctx, &sc.recipe, &opts, &out)?;
    if ctx.dry_run() {
        return Ok(json!({"replayed_steps": r.steps, "output": out.display().to_string(), "planned": true}));
    }
    let sha = sha256_file(&out).unwrap_or_default();
    let identical = sha == sc.output.sha256;
    let same_target = sc.tool.target == BUILD_TARGET && sc.tool.version == PIXELOX_VERSION;
    let recorded = resolve_recorded(&sc.output.path, &sc.base_dir);
    let ssim = if identical || recorded == out || !recorded.exists() { None } else { perceptual(ctx, &recorded, &out) };
    if a.flag("strict") && !identical {
        return Err(Error::new(
            "PX1602",
            format!(
                "replayed output differs from the recorded {} (sha256 {} vs {})",
                sc.output.path,
                &sha[..12.min(sha.len())],
                &sc.output.sha256[..12.min(sc.output.sha256.len())]
            ),
        )
        .with_context("ssim", ssim)
        .with_context("same_target", same_target));
    }
    if !identical && same_target {
        ctx.warn(
            "PXW0101",
            "the replay is not bit-identical although the build matches; an input or font may differ".to_string(),
        );
    }
    Ok(json!({
        "replayed_steps": r.steps,
        "output": out.display().to_string(),
        "sha256": sha,
        "recorded_sha256": sc.output.sha256,
        "identical": identical,
        "same_build": same_target,
        "ssim": ssim,
    }))
}

fn verify(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let sc_path = ctx.input(0)?.to_path_buf();
    ctx.check_allowed(&sc_path)?;
    let sc = Sidecar::read(&sc_path)?;
    let mut files = Vec::new();
    let mut ok = true;
    let out = resolve_recorded(&sc.output.path, &sc.base_dir);
    let actual = sha256_file(&out).ok();
    let good = actual.as_deref() == Some(sc.output.sha256.as_str());
    ok &= good;
    files.push(json!({"kind": "output", "path": sc.output.path, "expected": sc.output.sha256, "actual": actual, "match": good}));
    for i in &sc.inputs {
        let p = resolve_recorded(&i.path, &sc.base_dir);
        let actual = sha256_file(&p).ok();
        let good = actual.as_deref() == Some(i.sha256.as_str());
        ok &= good;
        files.push(json!({"kind": "input", "role": i.role, "path": i.path, "expected": i.sha256, "actual": actual, "match": good}));
    }
    let mut full = Value::Null;
    if a.flag("full") && ok {
        let dir = std::env::temp_dir().join(format!(
            "pixelox-verify-{}-{}",
            std::process::id(),
            sc.output.sha256.get(..8).unwrap_or("x")
        ));
        std::fs::create_dir_all(&dir)?;
        let name = Path::new(&sc.output.path).file_name().map(|n| n.to_os_string()).unwrap_or_else(|| "out.png".into());
        let target = dir.join(name);
        let mut child = ctx.child("replay", vec![]);
        child.globals.sidecar = false;
        child.globals.overwrite = true;
        child.globals.allow.clear();
        let opts = ReplayOptions {
            base_dir: Some(PathBuf::from(&sc.base_dir)),
            overrides: BTreeMap::new(),
            allow_drift: false,
        };
        let res = replay_into(&mut child, &sc.recipe, &opts, &target);
        let outcome = match res {
            Ok(_) => {
                let sha = sha256_file(&target).unwrap_or_default();
                let identical = sha == sc.output.sha256;
                let ssim = if identical { Some(1.0) } else { perceptual(ctx, &out, &target) };
                let same = sc.tool.target == BUILD_TARGET && sc.tool.version == PIXELOX_VERSION;
                let pass = identical || (!same && ssim.unwrap_or(0.0) >= 0.999);
                ok &= pass;
                json!({"replayed": true, "identical": identical, "ssim": ssim, "same_build": same, "pass": pass})
            }
            Err(e) => {
                ok = false;
                json!({"replayed": false, "error": e})
            }
        };
        let _ = std::fs::remove_dir_all(&dir);
        full = outcome;
    }
    if !ok {
        let drift = files.iter().any(|f| f["kind"] == "input" && f["match"] == false);
        let code = if drift { "PX1601" } else { "PX1602" };
        return Err(Error::new(code, format!("{} does not verify", sc_path.display()))
            .with_context("files", &files)
            .with_context("full", &full));
    }
    Ok(json!({"verified": true, "mode": if a.flag("full") { "full" } else { "fast" }, "files": files, "full": full}))
}

// ---------------------------------------------------------------------------------------
// history

fn history_list(_a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let steps: Vec<Value> = doc
        .recipe
        .iter()
        .enumerate()
        .map(|(i, s)| json!({"index": i + 1, "op": s.op, "params": s.params, "inputs": s.inputs.iter().map(|x| json!({"role": x.role, "path": x.path})).collect::<Vec<_>>()}))
        .collect();
    let mut text = String::new();
    for (i, s) in doc.recipe.iter().enumerate() {
        let params: Vec<String> = s
            .params
            .iter()
            .map(|(k, v)| format!("{k}={}", v.as_str().map(String::from).unwrap_or_else(|| v.to_string())))
            .collect();
        text.push_str(&format!("{:>3}  {} {}\n", i + 1, s.op, params.join(" ")));
    }
    Ok(json!({"steps": steps, "checkpoints": doc.checkpoints, "text": text}))
}

fn history_undo(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let keep = match a.str("to") {
        Some(name) => *doc.checkpoints.get(&name).ok_or_else(|| {
            Error::new("PX1010", format!("no checkpoint '{name}'")).with_context("checkpoints", &doc.checkpoints)
        })?,
        None => doc.recipe.len().saturating_sub(a.usize("steps")?),
    };
    if keep == 0 {
        return Err(Error::new("PX1013", "cannot undo past the step that created the document"));
    }
    let removed = doc.recipe.len() - keep;
    let recipe = doc.recipe[..keep].to_vec();
    let input = ctx.input(0)?.to_path_buf();
    let base = input.parent().map(|p| p.to_path_buf());
    let opts = ReplayOptions { base_dir: base, overrides: BTreeMap::new(), allow_drift: false };
    let r = run_recipe(ctx, &recipe, &opts)?;
    let mut d = r.doc.ok_or_else(|| Error::new("PX1604", "the remaining recipe does not build a document"))?;
    d.checkpoints = doc.checkpoints.iter().filter(|(_, v)| **v <= keep).map(|(k, v)| (k.clone(), *v)).collect();
    *doc = d;
    Ok(json!({"undone_steps": removed, "steps_now": keep}))
}

fn history_checkpoint(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let name = a.str_or("name", "");
    doc.checkpoints.insert(name.clone(), doc.recipe.len());
    Ok(json!({"checkpoint": name, "at_step": doc.recipe.len()}))
}

// ---------------------------------------------------------------------------------------
// system

pub fn features() -> Value {
    json!({
        "ml": pixelox_ml::ML_AVAILABLE,
        "psd_read_layered": true,
        "psd_write": "flattened + basic layered",
        "webp_lossy_write": false,
        "color_management": false,
        "js_render": true,
        "vendored_d3_three": false,
        "self_update": true,
    })
}

fn version(_a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    Ok(json!({
        "version": PIXELOX_VERSION,
        "build": format!("git:{BUILD_GIT}"),
        "target": BUILD_TARGET,
        "profile": BUILD_PROFILE,
        "features": features(),
        "text": format!("pixelox {PIXELOX_VERSION} (git:{BUILD_GIT}, {BUILD_TARGET}, {BUILD_PROFILE})\n"),
    }))
}

fn completions(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    use clap_complete::Shell;
    let shell = match a.choice("shell").as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" => Shell::PowerShell,
        _ => Shell::Elvish,
    };
    let mut cmd = crate::cli::build_command(reg(ctx)?);
    let mut buf = Vec::new();
    clap_complete::generate(shell, &mut cmd, "pixelox", &mut buf);
    Ok(json!({ "text": String::from_utf8_lossy(&buf) }))
}

fn migrate(path: &Path, ctx: &mut OpCtx) -> Result<Value> {
    ctx.check_allowed(path)?;
    let doc = ctx.load_document(path)?;
    if ctx.dry_run() {
        return Ok(json!({"path": path.display().to_string(), "format_version": doc.format_version, "planned": true}));
    }
    let bak = PathBuf::from(format!("{}.bak", path.display()));
    std::fs::copy(path, &bak)?;
    let mut g = ctx.globals.clone();
    g.in_place = true;
    let saved = std::mem::replace(&mut ctx.globals, g);
    ctx.inputs = vec![path.to_path_buf()];
    let r = ctx.write_doc(path, &doc, Some("migrated"));
    ctx.globals = saved;
    r?;
    Ok(
        json!({"path": path.display().to_string(), "backup": bak.display().to_string(), "format_version": pixelox_core::doc::PXO_FORMAT_VERSION}),
    )
}

fn doctor(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    use pixelox_core::paths;
    if let Some(p) = a.path("migrate-pxo") {
        return migrate(&p, ctx);
    }
    let r = reg(ctx)?;
    let (read, write) = ctx.io.formats();
    let cfg = crate::config::Config::load(ctx.globals.config_path.as_deref()).ok();
    let config: Map<String, Value> = crate::config::KEYS
        .iter()
        .map(|(k, _, _)| (k.to_string(), json!(cfg.as_ref().and_then(|c| c.get(k)))))
        .collect();
    let fonts = r.get("fonts list").and_then(|spec| {
        let args = Args::build(spec, vec![]).ok()?;
        let mut child = ctx.child(spec.name, vec![]);
        pixelox_core::exec::run_spec(spec, &args, &mut child).ok()
    });
    let font_count = fonts.as_ref().and_then(|f| f.get("families").and_then(|x| x.as_array()).map(|a| a.len()));
    let mut issues = Vec::new();
    if !pixelox_ml::ML_AVAILABLE {
        issues.push("this build has no ML runtime: detect *, fill content-aware, and models pull exit 12 (PX1510); their --dry-run plans still work (mask refine is classical and works)");
    } else {
        issues.push("local models: detect subject and detect salient run here (models download on first use with -y); detect objects/faces/text/segment and fill content-aware are not in this release (exit 12)");
    }
    issues.push("no color-management engine: ICC profiles are kept and re-embedded, not converted; export --profile other than srgb exits 12");
    issues.push("WebP is written lossless only; JPEG is written baseline (no progressive)");
    issues.push("render d3/three need a local bundle passed with --lib (not vendored in this build)");
    let text = format!(
        "pixelox {PIXELOX_VERSION} (git:{BUILD_GIT}, {BUILD_TARGET}, {BUILD_PROFILE})\ncommands: {}\nread: {}\nwrite: {}\nrenderers: {}\nfonts: {}\nconfig: {}\nstate: {}\nmodels: {}\nnotes:\n  - {}\n",
        r.ops().len(),
        read.join(" "),
        write.join(" "),
        compose::installed_renderers().join(" "),
        font_count.map(|n| format!("{n} families")).unwrap_or_else(|| "unknown".into()),
        paths::config_file().display(),
        paths::state_dir().display(),
        paths::models_dir().display(),
        issues.join("\n  - ")
    );
    Ok(json!({
        "version": PIXELOX_VERSION,
        "build": {"git": BUILD_GIT, "target": BUILD_TARGET, "profile": BUILD_PROFILE},
        "features": features(),
        "commands": r.ops().len(),
        "codecs": {"read": read, "write": write},
        "renderers": compose::installed_renderers(),
        "fonts": fonts,
        "models": {"dir": paths::models_dir().display().to_string(), "installed": pixelox_ml::installed_models(), "build_supports_ml": pixelox_ml::ML_AVAILABLE},
        "paths": {
            "config": paths::config_file().display().to_string(),
            "cache": paths::cache_dir().display().to_string(),
            "data": paths::data_dir().display().to_string(),
            "state": paths::state_dir().display().to_string(),
            "jobs": paths::jobs_dir().display().to_string(),
        },
        "config": config,
        "known_issues": issues,
        "ok": true,
        "text": text,
    }))
}

/// Used by tests: a document's path is a .pxo.
pub fn is_document(p: &Path) -> bool {
    is_pxo(p)
}
