//! The executor: runs one command with resolved arguments. It loads documents, enforces
//! the consent gates (`-o` / `--in-place` / `--overwrite`), appends the recipe step, writes
//! outputs atomically, writes provenance sidecars, and builds the envelope. The CLI, the
//! MCP server, pipelines, batches, and `replay` all go through here.

use crate::compose;
use crate::ctx::CancelToken;
use crate::ctx::{format_of, is_pxo, EncodeOptions, Globals, ImageIo, OpCtx, OutputEntry, Progress};
use crate::doc::{Document, InputRef, RecipeStep};
use crate::envelope::{
    now_utc, redact_path, sha256_file, CommandInfo, Envelope, FileRecord, OutputInfo, Sidecar, ToolInfo,
    SIDECAR_VERSION,
};
use crate::error::{Error, Result};
use crate::op::{json_to_pairs, Args, DocMode, OpSpec, Runner};
use crate::registry::Registry;
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub struct Runtime<'a> {
    pub registry: &'a Registry,
    pub io: &'a dyn ImageIo,
    pub progress: &'a dyn Progress,
    pub cancel: CancelToken,
}

/// One request to run a command.
#[derive(Clone, Debug, Default)]
pub struct Invocation {
    /// Command path or dotted id.
    pub command: String,
    pub args: Vec<(String, String)>,
    pub inputs: Vec<String>,
    pub globals: Globals,
    /// The original command line, recorded in sidecars.
    pub argv: Vec<String>,
}

/// The envelope plus bytes destined for stdout (`-o -`).
pub struct Outcome {
    pub envelope: Envelope,
    pub stdout: Option<Vec<u8>>,
}

fn expand_inputs(inputs: &[String]) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for i in inputs {
        if i.contains(['*', '?', '[']) && !Path::new(i).exists() {
            let mut matched: Vec<PathBuf> = glob::glob(i)
                .map_err(|e| Error::usage(format!("bad glob '{i}': {e}")))?
                .filter_map(|r| r.ok())
                .filter(|p| !p.to_string_lossy().ends_with(".pixelox.json"))
                .collect();
            matched.sort();
            if matched.is_empty() {
                return Err(Error::new("PX1101", format!("no files match '{i}'")).with_context("glob", i));
            }
            out.extend(matched);
        } else {
            out.push(PathBuf::from(i));
        }
    }
    Ok(out)
}

impl<'a> Runtime<'a> {
    pub fn new(registry: &'a Registry, io: &'a dyn ImageIo, progress: &'a dyn Progress) -> Runtime<'a> {
        Runtime { registry, io, progress, cancel: CancelToken::default() }
    }

    /// Run a command; never panics (panics become PX9001 envelopes).
    pub fn invoke(&self, inv: Invocation) -> Outcome {
        let start = Instant::now();
        let id = self.registry.get(&inv.command).map(|s| s.id()).unwrap_or_else(|| inv.command.replace(' ', "."));
        let seed = inv.globals.seed;
        let dry = inv.globals.dry_run;
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| self.invoke_inner(&inv, start)));
        match r {
            Ok(o) => o,
            Err(p) => {
                let msg = p
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                    .unwrap_or_else(|| "panic".into());
                let e = Error::new("PX9001", format!("internal error (panic): {msg}"));
                Outcome {
                    envelope: Envelope::failure(&id, e, seed, dry, start.elapsed().as_millis() as u64),
                    stdout: None,
                }
            }
        }
    }

    fn invoke_inner(&self, inv: &Invocation, start: Instant) -> Outcome {
        let ms = || start.elapsed().as_millis() as u64;
        let fail = |id: &str, e: Error| Outcome {
            envelope: Envelope::failure(
                id,
                e,
                inv.globals.seed,
                inv.globals.dry_run,
                start.elapsed().as_millis() as u64,
            ),
            stdout: None,
        };
        let spec = match self.registry.require(&inv.command) {
            Ok(s) => s,
            Err(e) => return fail(&inv.command.replace(' ', "."), e),
        };
        let id = spec.id();
        let args = match Args::build(spec, inv.args.clone()) {
            Ok(a) => a,
            Err(e) => return fail(&id, e),
        };
        let inputs = match expand_inputs(&inv.inputs) {
            Ok(i) => i,
            Err(e) => return fail(&id, e),
        };
        if inputs.len() < spec.inputs.min {
            let e = Error::new(
                "PX1003",
                format!("'{}' needs {} input file(s), got {}", spec.name, spec.inputs.min, inputs.len()),
            )
            .with_hint(format!("pixelox docs {id}"));
            return fail(&id, e);
        }
        if let Some(max) = spec.inputs.max {
            if inputs.len() > max {
                let e = Error::new(
                    "PX1003",
                    format!("'{}' takes at most {max} input file(s), got {}", spec.name, inputs.len()),
                )
                .with_hint("process several files with `pixelox batch run --glob ...`");
                return fail(&id, e);
            }
        }
        let mut ctx = OpCtx::new(spec.name, inputs, inv.globals.clone(), self.io, self.progress);
        ctx.registry = Some(self.registry);
        ctx.cancel = self.cancel.clone();
        if ctx.globals.seed_drawn {
            ctx.warn("PXW0002", format!("--seed random drew seed {}", ctx.globals.seed));
        }
        if spec.is_read_only() && ctx.globals.dry_run {
            ctx.warn("PXW0001", "--dry-run was given to a read-only command and ignored");
            ctx.globals.dry_run = false;
        }
        let res = run_spec(spec, &args, &mut ctx);
        let mut env = Envelope::new(&id, ctx.globals.seed, ctx.globals.dry_run);
        match res {
            Ok(result) => {
                if !ctx.globals.dry_run && ctx.globals.sidecar {
                    if let Err(e) = write_sidecars(spec, &args, &mut ctx, &inv.argv) {
                        cleanup(&ctx);
                        return fail(&id, e);
                    }
                }
                env.result = result;
                env.inputs = ctx.input_records.clone();
                env.outputs = ctx.outputs.iter().map(|o| o.record.clone()).collect();
                env.warnings = ctx.warnings.clone();
                env.timing_ms = ms();
                Outcome { envelope: env, stdout: ctx.stdout.take() }
            }
            Err(e) => {
                // atomic semantics: a failed command leaves no outputs (verification failures
                // keep theirs: the report is the point)
                if e.code != "PX1602" && e.code != "PX1601" {
                    cleanup(&ctx);
                }
                let mut env = Envelope::failure(&id, e, ctx.globals.seed, ctx.globals.dry_run, ms());
                env.inputs = ctx.input_records.clone();
                env.warnings = ctx.warnings.clone();
                Outcome { envelope: env, stdout: None }
            }
        }
    }
}

fn cleanup(ctx: &OpCtx) {
    for t in &ctx.temp_files {
        let _ = std::fs::remove_file(t);
    }
    if ctx.globals.dry_run {
        return;
    }
    for o in &ctx.outputs {
        if o.record.action.as_deref() == Some("mutate-in-place") || o.record.path == "-" {
            continue;
        }
        let _ = std::fs::remove_file(&o.record.path);
    }
}

/// The recipe step for `spec` with `args`, the context's recorded inputs, and values
/// resolved at run time.
pub fn make_step(spec: &OpSpec, args: &Args, ctx: &OpCtx) -> RecipeStep {
    let mut params = args.to_json();
    for (k, v) in &ctx.resolved {
        params.insert(k.clone(), v.clone());
    }
    RecipeStep {
        op: spec.id(),
        params,
        inputs: ctx.step_inputs.clone(),
        model: ctx.models.first().cloned(),
        fonts: ctx.fonts.clone(),
    }
}

/// The recipe an exported deliverable records: the document's history plus `step`.
pub fn recipe_with(doc: &Document, step: RecipeStep) -> Vec<RecipeStep> {
    let mut r = doc.recipe.clone();
    r.push(step);
    r
}

/// Where a mutating doc op writes: `--in-place` (the .pxo input), `-o`, or nothing in a
/// dry run without `-o`.
fn resolve_target(ctx: &OpCtx, input: &Path, optional: bool) -> Result<Option<PathBuf>> {
    if ctx.globals.in_place {
        if ctx.globals.output.is_some() {
            return Err(Error::new("PX1005", "pass either -o or --in-place, not both"));
        }
        if !is_pxo(input) {
            return Err(Error::new(
                "PX1108",
                format!("{} is a source image; pixelox never edits source images in place", input.display()),
            )
            .with_hint(format!(
                "write a new file with -o, or make a document first: pixelox open {} -o doc.pxo",
                input.display()
            )));
        }
        ctx.prepare_output(input)?;
        return Ok(Some(input.to_path_buf()));
    }
    match &ctx.globals.output {
        Some(o) => {
            let p = PathBuf::from(o);
            ctx.prepare_output(&p)?;
            Ok(Some(p))
        }
        None if ctx.globals.dry_run || optional => Ok(None),
        None => Err(Error::new("PX1004", format!("'{}' changes the document; say where to write it", ctx.command))
            .with_hint(if is_pxo(input) {
                "pass -o <path> or --in-place".to_string()
            } else {
                "pass -o <path> (.pxo keeps layers; .png/.jpg/... flattens)".to_string()
            })),
    }
}

/// Write a document to `path`: `.pxo` keeps everything; other formats flatten.
pub fn write_document(ctx: &mut OpCtx, doc: &Document, path: &Path) -> Result<()> {
    if is_pxo(path) {
        return ctx.write_doc(path, doc, None);
    }
    let mut opts = EncodeOptions::for_path(path);
    if let Some(f) = &ctx.globals.format {
        opts.format = f.clone();
    }
    let step = RecipeStep { op: "export".into(), params: Map::new(), inputs: vec![], model: None, fonts: vec![] };
    let recipe = recipe_with(doc, step);
    if ctx.globals.dry_run {
        let action = ctx.prepare_output(path)?;
        let info = OutputInfo {
            size: Some([doc.width, doc.height]),
            format: Some(opts.format.clone()),
            bit_depth: Some(doc.depth),
            ..Default::default()
        };
        ctx.outputs.push(OutputEntry {
            record: FileRecord {
                path: path.display().to_string(),
                size: info.size,
                format: info.format.clone(),
                action: Some(action.into()),
                ..Default::default()
            },
            info,
            recipe: Some(recipe),
        });
        return Ok(());
    }
    let img = compose::composite(doc)?;
    ctx.write_image(path, &img, Some(doc), &opts, None, Some(recipe))
}

/// Run a spec with prepared args in `ctx` (no sidecars; the caller writes them).
pub fn run_spec(spec: &OpSpec, args: &Args, ctx: &mut OpCtx) -> Result<Value> {
    match spec.runner {
        Runner::Custom(f) => {
            ctx.set_progress(0.0);
            let v = f(args, ctx)?;
            ctx.set_progress(1.0);
            Ok(v)
        }
        Runner::Source(f) => {
            let target = match &ctx.globals.output {
                Some(o) => {
                    let p = PathBuf::from(o);
                    ctx.prepare_output(&p)?;
                    Some(p)
                }
                None if spec.inputs.min > 0 || ctx.globals.dry_run => None,
                None => {
                    return Err(Error::new(
                        "PX1004",
                        format!("'{}' creates a document; say where to write it", spec.name),
                    )
                    .with_hint("pass -o doc.pxo"))
                }
            };
            let mut doc = f(args, ctx)?;
            doc.quantize_to_storage();
            if doc.recipe.is_empty() {
                let step = make_step(spec, args, ctx);
                doc.recipe.push(step);
            }
            if let Some(t) = &target {
                write_document(ctx, &doc, t)?;
            }
            Ok(doc_result(spec, args, ctx, &doc, Value::Null, target.as_deref()))
        }
        Runner::Doc(f) => {
            let input = ctx.input(0)?.to_path_buf();
            let target =
                if spec.mode == DocMode::Mutate { resolve_target(ctx, &input, spec.target_optional)? } else { None };
            let mut doc = ctx.load_document(&input)?;
            ctx.op_index = doc.recipe.len();
            ctx.set_progress(0.0);
            let v = f(args, &mut doc, ctx)?;
            if spec.mode == DocMode::Mutate {
                doc.quantize_to_storage();
                if !no_record(spec) {
                    let step = make_step(spec, args, ctx);
                    doc.recipe.push(step);
                }
                if let Some(t) = &target {
                    write_document(ctx, &doc, t)?;
                }
            }
            ctx.set_progress(1.0);
            Ok(doc_result(spec, args, ctx, &doc, v, target.as_deref().or(Some(input.as_path()))))
        }
    }
}

/// Ops that manage the recipe themselves (`save`, `history undo`).
fn no_record(spec: &OpSpec) -> bool {
    matches!(spec.name, "save" | "history undo" | "history checkpoint")
}

fn doc_result(spec: &OpSpec, args: &Args, ctx: &OpCtx, doc: &Document, v: Value, path: Option<&Path>) -> Value {
    let mut params = args.to_json();
    for (k, val) in &ctx.resolved {
        params.insert(k.clone(), val.clone());
    }
    let mut summary = doc.summary();
    if let Some(p) = path {
        summary["path"] = json!(p.display().to_string());
    }
    let mut r = json!({"op": spec.id(), "params": params, "document": summary});
    match v {
        Value::Object(m) => {
            for (k, val) in m {
                r[k] = val;
            }
        }
        Value::Null => {}
        other => r["value"] = other,
    }
    if ctx.globals.dry_run {
        let mut plan = Map::new();
        plan.insert("ops".into(), json!([{"op": spec.id(), "params": r["params"].clone()}]));
        plan.insert("outputs_predicted".into(), json!(ctx.outputs.iter().map(|o| &o.record).collect::<Vec<_>>()));
        plan.insert("models_required".into(), json!([]));
        plan.insert("consent_required".into(), json!([]));
        for (k, val) in &ctx.plan {
            plan.insert(k.clone(), val.clone());
        }
        r["plan"] = Value::Object(plan);
    }
    r
}

/// Inputs referenced by a recipe (deduplicated by path + role).
pub fn recipe_inputs(recipe: &[RecipeStep]) -> Vec<InputRef> {
    let mut out: Vec<InputRef> = Vec::new();
    for s in recipe {
        for i in &s.inputs {
            if !out.iter().any(|o| o.path == i.path && o.role == i.role) {
                out.push(i.clone());
            }
        }
    }
    out
}

fn write_sidecars(spec: &OpSpec, args: &Args, ctx: &mut OpCtx, argv: &[String]) -> Result<()> {
    let base_dir = std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_default();
    let own_step = make_step(spec, args, ctx);
    let mut params = args.to_json();
    for (k, v) in &ctx.resolved {
        params.insert(k.clone(), v.clone());
    }
    let redact = ctx.globals.redact_paths;
    let entries: Vec<OutputEntry> = ctx.outputs.clone();
    for (i, o) in entries.iter().enumerate() {
        if o.record.path == "-" || o.record.path.ends_with(".pixelox.json") {
            continue;
        }
        let mut recipe = o.recipe.clone().unwrap_or_else(|| vec![own_step.clone()]);
        let mut inputs = recipe_inputs(&recipe);
        let mut output = o.info.clone();
        let mut base = base_dir.clone();
        let mut argv_rec = argv.to_vec();
        if redact {
            for s in recipe.iter_mut() {
                for inp in s.inputs.iter_mut() {
                    inp.path = redact_path(&inp.path);
                }
            }
            for inp in inputs.iter_mut() {
                inp.path = redact_path(&inp.path);
            }
            output.path = redact_path(&output.path);
            base = "<redacted>".into();
            argv_rec = argv
                .iter()
                .map(|a| if a.contains('/') || a.contains('\\') { redact_path(a) } else { a.clone() })
                .collect();
        }
        let sc = Sidecar {
            pixelox_sidecar: SIDECAR_VERSION,
            tool: ToolInfo::current(),
            created_utc: now_utc(),
            base_dir: base,
            command: CommandInfo { id: spec.id(), argv: argv_rec, params: params.clone(), seed: ctx.globals.seed },
            recipe,
            inputs,
            models: ctx.models.clone(),
            fonts: ctx.fonts.clone(),
            output,
            env: json!({"threads": ctx.threads(), "mem_budget_mb": ctx.globals.mem_budget_mb, "locale": "C"}),
            determinism: Sidecar::determinism_block(),
            warnings: ctx.warnings.clone(),
        };
        let path = Sidecar::path_for(Path::new(&o.record.path));
        let tmp = crate::ctx::tmp_path(&path);
        std::fs::write(&tmp, serde_json::to_vec_pretty(&sc)?)?;
        std::fs::rename(&tmp, &path)?;
        ctx.outputs[i].record.sidecar = Some(path.display().to_string());
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------
// recipes: replay and history

/// How to run a recorded recipe.
#[derive(Clone, Debug, Default)]
pub struct ReplayOptions {
    /// Directory relative recipe paths resolve against when they do not exist as given.
    pub base_dir: Option<PathBuf>,
    /// `--input role=path` overrides.
    pub overrides: BTreeMap<String, String>,
    pub allow_drift: bool,
}

/// What running a recipe produced: the document (if the recipe built one) and the final
/// step that names the deliverable (`export`, `save`, or a custom command).
pub struct Replayed {
    pub doc: Option<Document>,
    pub last: Option<RecipeStep>,
    /// Number of steps executed.
    pub steps: usize,
}

fn resolve_rel(p: &str, base: Option<&Path>) -> PathBuf {
    let pb = PathBuf::from(p);
    if pb.exists() || pb.is_absolute() {
        return pb;
    }
    match base {
        Some(b) if b.join(&pb).exists() => b.join(&pb),
        _ => pb,
    }
}

/// Check (and possibly substitute) a step's recorded inputs. Returns the step's params
/// with overrides applied and the source paths.
fn prepare_step_inputs(
    ctx: &mut OpCtx,
    step: &RecipeStep,
    opts: &ReplayOptions,
) -> Result<(Map<String, Value>, Vec<PathBuf>)> {
    let mut params = step.params.clone();
    let mut sources = Vec::new();
    for inp in &step.inputs {
        let (path, overridden) = match opts.overrides.get(&inp.role) {
            Some(o) => (PathBuf::from(o), true),
            None => (resolve_rel(&inp.path, opts.base_dir.as_deref()), false),
        };
        // a recorded path is untrusted input (e.g. a sidecar handed to the MCP server)
        ctx.check_allowed(&path)?;
        if !path.exists() {
            return Err(Error::new(
                "PX1604",
                format!("step '{}' needs {} ({}), which is missing", step.op, path.display(), inp.role),
            )
            .with_hint(format!("restore it or map a replacement with --input {}=<path>", inp.role))
            .with_context("role", &inp.role)
            .with_context("path", path.display().to_string()));
        }
        let actual = sha256_file(&path)?;
        if actual != inp.sha256 && !overridden {
            if opts.allow_drift {
                ctx.warn_ctx(
                    "PXW0101",
                    format!("input {} changed since it was recorded", path.display()),
                    json!({"expected": inp.sha256, "actual": actual}),
                );
            } else {
                return Err(Error::new(
                    "PX1601",
                    format!("input {} ({}) changed since it was recorded", path.display(), inp.role),
                )
                .with_context("role", &inp.role)
                .with_context("path", path.display().to_string())
                .with_context("expected", &inp.sha256)
                .with_context("actual", &actual));
            }
        }
        if overridden {
            if let Some(param) = &inp.param {
                params.insert(param.clone(), Value::String(path.display().to_string()));
            }
        }
        if inp.role == "source" {
            sources.push(path);
        }
    }
    Ok((params, sources))
}

/// Execute a recipe in memory up to (not including) its final `export`/`save` step.
pub fn run_recipe(ctx: &mut OpCtx, recipe: &[RecipeStep], opts: &ReplayOptions) -> Result<Replayed> {
    let registry = ctx.registry()?;
    let mut doc: Option<Document> = None;
    let mut steps = 0;
    for (i, step) in recipe.iter().enumerate() {
        ctx.check_cancel()?;
        if step.op == "export" || step.op == "save" {
            return Ok(Replayed { doc, last: Some(step.clone()), steps });
        }
        let spec = registry
            .require(&step.op)
            .map_err(|e| Error::new("PX1604", format!("step {} ('{}') cannot run: {}", i + 1, step.op, e.message)))?;
        if spec.is_doc() && spec.mode == DocMode::Export {
            // a deliverable written from the document (svg from-doc, mask export, ...)
            return Ok(Replayed { doc, last: Some(step.clone()), steps });
        }
        let (params, sources) = prepare_step_inputs(ctx, step, opts)?;
        let args = Args::build(spec, json_to_pairs(&Value::Object(params))?)
            .map_err(|e| Error::new("PX1604", format!("step {} ('{}'): {}", i + 1, step.op, e.message)))?;
        let mut child = ctx.child(spec.name, sources);
        child.op_index = i;
        child.replaying = true;
        child.globals.output = None;
        child.globals.in_place = false;
        child.globals.dry_run = false;
        ctx.progress.stage(&step.op, i + 1, recipe.len());
        match spec.runner {
            Runner::Source(f) => {
                let mut d = f(&args, &mut child)?;
                d.quantize_to_storage();
                d.recipe = recipe[..=i].to_vec();
                doc = Some(d);
            }
            Runner::Doc(f) => {
                let d = doc.as_mut().ok_or_else(|| {
                    Error::new("PX1604", format!("step {} ('{}') has no document to work on", i + 1, step.op))
                })?;
                f(&args, d, &mut child)?;
                d.quantize_to_storage();
                d.recipe = recipe[..=i].to_vec();
            }
            Runner::Custom(_) => {
                // a custom command is a deliverable step: the caller runs it
                return Ok(Replayed { doc, last: Some(step.clone()), steps });
            }
        }
        child.outputs.clear();
        ctx.absorb(child);
        steps += 1;
    }
    Ok(Replayed { doc, last: None, steps })
}

/// Replay a recipe into `output`: runs the steps, then the final deliverable step.
pub fn replay_into(ctx: &mut OpCtx, recipe: &[RecipeStep], opts: &ReplayOptions, output: &Path) -> Result<Replayed> {
    let registry = ctx.registry()?;
    let r = run_recipe(ctx, recipe, opts)?;
    let last = r.last.clone();
    match (&r.doc, &last) {
        (Some(doc), Some(step)) if step.op == "save" => {
            ctx.write_doc(output, doc, None)?;
        }
        (Some(doc), Some(step)) => {
            let spec = registry.require(&step.op)?;
            let (params, _) = prepare_step_inputs(ctx, step, opts)?;
            let args = Args::build(spec, json_to_pairs(&Value::Object(params))?)?;
            match spec.runner {
                Runner::Doc(f) => {
                    let mut d = doc.clone();
                    let mut child = ctx.child(spec.name, vec![]);
                    child.op_index = d.recipe.len();
                    child.globals.output = Some(output.display().to_string());
                    f(&args, &mut d, &mut child)?;
                    ctx.absorb(child);
                }
                _ => return Err(Error::new("PX1604", format!("final step '{}' is not an export", step.op))),
            }
        }
        (Some(doc), None) => {
            write_document(ctx, doc, output)?;
        }
        (None, Some(step)) => {
            let spec = registry.require(&step.op)?;
            let (params, sources) = prepare_step_inputs(ctx, step, opts)?;
            let args = Args::build(spec, json_to_pairs(&Value::Object(params))?)?;
            let mut child = ctx.child(spec.name, sources);
            child.globals.output = Some(output.display().to_string());
            run_spec(spec, &args, &mut child)?;
            ctx.absorb(child);
        }
        (None, None) => return Err(Error::new("PX1604", "the recipe is empty")),
    }
    Ok(r)
}

/// The output format a path implies (for messages).
pub fn describe_target(path: &Path) -> String {
    format_of(path).unwrap_or_else(|| "unknown".into())
}
