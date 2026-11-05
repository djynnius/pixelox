//! Pipeline files (PRD §9): declarative multi-step recipes run in one process. Steps share
//! documents in memory (a step without `input` works on the document of the step it
//! needs), fan out over globs or CSV rows, and every written output gets its sidecar.

use pixelox_core::ctx::{absolutize, is_pxo, OpCtx, OutputEntry};
use pixelox_core::doc::Document;
use pixelox_core::envelope::{now_utc, sha256_bytes};
use pixelox_core::exec::{make_step, run_spec, write_document};
use pixelox_core::op::{json_to_pairs, scalar_to_string, Args, DocMode, OpSpec, Runner};
use pixelox_core::prelude::*;
use pixelox_core::Registry;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Defaults {
    #[serde(default)]
    pub seed: Option<u64>,
    #[serde(default, alias = "on_error")]
    pub on_error: Option<String>,
    #[serde(default, alias = "mem_budget")]
    pub mem_budget: Option<Value>,
    #[serde(default, alias = "color_space")]
    pub color_space: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InputDecl {
    Path(String),
    Pinned {
        path: String,
        #[serde(default)]
        sha256: Option<String>,
    },
}

impl InputDecl {
    fn path(&self) -> &str {
        match self {
            InputDecl::Path(p) => p,
            InputDecl::Pinned { path, .. } => path,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Foreach {
    #[serde(default)]
    pub glob: Option<String>,
    #[serde(default)]
    pub csv: Option<String>,
    #[serde(default, rename = "as")]
    pub as_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Step {
    pub id: String,
    pub run: String,
    #[serde(default)]
    pub with: Map<String, Value>,
    #[serde(default)]
    pub needs: Option<Vec<String>>,
    #[serde(default)]
    pub foreach: Option<Foreach>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PipelineFile {
    pub version: u32,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub defaults: Defaults,
    #[serde(default)]
    pub vars: BTreeMap<String, Value>,
    #[serde(default)]
    pub inputs: BTreeMap<String, InputDecl>,
    pub steps: Vec<Step>,
    #[serde(default)]
    pub outputs: Vec<String>,
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::custom("pipeline run", Category::Automation, "Run a pipeline file in one process", pipeline_run)
            .about("Runs the steps of a pipeline file (YAML or JSON; `pixelox docs pipelines`) in dependency order. Steps share documents in memory; every written output gets a sidecar, and a run record (<name>.pipeline.pixelox.json) is written next to the first declared output. --dry-run prints the resolved plan.")
            .param(param("var", ParamKind::Str).repeatable().help("Override a variable: k=v (repeatable)"))
            .param(param("jobs", ParamKind::Int).default("1").range("1..256").help("Parallel items for foreach steps"))
            .param(param("bind", ParamKind::Str).repeatable().advanced().help("Bind an item value for the run: item.in=path (used by batch run)"))
            .example("pixelox pipeline run poster.yml --json", "Build everything the file declares")
            .example("pixelox pipeline run poster.yml --var headline=\"WINTER DROP 01\" --json", "Override a variable")
            .example("pixelox pipeline run social-cards.yml --var csv=speakers.csv --jobs 4 --dry-run --json", "Plan a fan-out"),
        OpSpec::custom("pipeline validate", Category::Automation, "Check a pipeline file without running it", pipeline_validate)
            .about("Parses the file, checks step commands and parameters against their schemas, the `needs` graph (PX7102, PX7103), variables (PX7105), and pinned input hashes (PX1601).")
            .param(param("var", ParamKind::Str).repeatable().help("Override a variable: k=v"))
            .example("pixelox pipeline validate poster.yml --json", "Validate before running")
            .example("pixelox pipeline validate watermark.yml", "Exit 2 means a step or parameter is wrong")
            .example("pixelox pipeline validate poster.yml --var headline=X --json", "Validate with an override"),
        OpSpec::custom("pipeline new", Category::Automation, "Scaffold a pipeline, optionally from a sidecar", pipeline_new)
            .inputs(Inputs::NONE)
            .about("Writes a starter pipeline to -o. With --from-sidecar, turns an output's recorded recipe into a reusable pipeline (provenance → automation).")
            .param(param("from-sidecar", ParamKind::Path).help("A <output>.pixelox.json whose recipe becomes the steps"))
            .example("pixelox pipeline new -o p.yml", "A commented starter file")
            .example("pixelox pipeline new -o again.yml --from-sidecar out.png.pixelox.json", "A pipeline from what was done")
            .example("pixelox pipeline new -o p.yml --dry-run --json", "Preview it"),
        OpSpec::custom("pipeline graph", Category::Automation, "Print a pipeline's step graph", pipeline_graph)
            .about("Edges `a -> b` as text, or Graphviz with --format dot.")
            .example("pixelox pipeline graph poster.yml", "Text edges")
            .example("pixelox pipeline graph poster.yml --format dot > poster.dot", "Graphviz source")
            .example("pixelox pipeline graph poster.yml --json", "As JSON"),
    ]
}

/// The pipeline file JSON Schema (`pixelox schema pipeline`).
pub fn pipeline_schema() -> Value {
    json!({
        "$schema": pixelox_core::schema::DIALECT,
        "$id": format!("{}/pipeline.json", pixelox_core::schema::BASE),
        "title": "pixelox pipeline file",
        "type": "object",
        "required": ["version", "steps"],
        "properties": {
            "version": {"const": 1},
            "name": {"type": "string"},
            "description": {"type": "string"},
            "defaults": {"type": "object", "properties": {"seed": {"type": "integer"}, "on-error": {"enum": ["stop", "continue"]}, "mem-budget": {}, "color-space": {"type": "string"}}},
            "vars": {"type": "object"},
            "inputs": {"type": "object", "additionalProperties": {"oneOf": [{"type": "string"}, {"type": "object", "required": ["path"], "properties": {"path": {"type": "string"}, "sha256": {"type": "string"}}}]}},
            "steps": {"type": "array", "items": {"type": "object", "required": ["id", "run"], "properties": {
                "id": {"type": "string"},
                "run": {"type": "string", "description": "Dotted command id, e.g. filter.gaussian"},
                "with": {"type": "object", "description": "The command's parameters by flag name, plus input, inputs, output, in-place"},
                "needs": {"type": "array", "items": {"type": "string"}},
                "foreach": {"type": "object", "properties": {"glob": {"type": "string"}, "csv": {"type": "string"}, "as": {"type": "string"}}}}}},
            "outputs": {"type": "array", "items": {"type": "string"}}
        }
    })
}

fn invalid(msg: impl Into<String>) -> Error {
    Error::new("PX7101", msg)
}

pub fn parse_text(text: &str) -> Result<PipelineFile> {
    let p: PipelineFile = serde_yaml::from_str(text).map_err(|e| invalid(format!("pipeline file: {e}")))?;
    if p.version != 1 {
        return Err(invalid(format!("pipeline version {} is not supported (expected 1)", p.version)));
    }
    let mut seen = std::collections::BTreeSet::new();
    for s in &p.steps {
        if !seen.insert(s.id.clone()) {
            return Err(invalid(format!("step id '{}' is used twice", s.id)));
        }
    }
    Ok(p)
}

pub fn load(ctx: &mut OpCtx, path: &Path) -> Result<(PipelineFile, Vec<u8>)> {
    let bytes = ctx.read_aux(path, "pipeline", None)?;
    let text = String::from_utf8(bytes.clone()).map_err(|_| invalid("the pipeline file is not UTF-8"))?;
    Ok((parse_text(&text)?, bytes))
}

/// Topological order; `needs` defaults to the previous step.
pub fn order(p: &PipelineFile) -> Result<Vec<usize>> {
    let ids: HashMap<&str, usize> = p.steps.iter().enumerate().map(|(i, s)| (s.id.as_str(), i)).collect();
    let mut deps: Vec<Vec<usize>> = vec![vec![]; p.steps.len()];
    for (i, s) in p.steps.iter().enumerate() {
        match &s.needs {
            Some(n) => {
                for d in n {
                    let j = *ids.get(d.as_str()).ok_or_else(|| {
                        Error::new("PX7102", format!("step '{}' needs '{d}', which is not a step", s.id))
                    })?;
                    deps[i].push(j);
                }
            }
            None if i > 0 => deps[i].push(i - 1),
            None => {}
        }
    }
    let mut state = vec![0u8; p.steps.len()];
    let mut out = Vec::new();
    fn visit(i: usize, deps: &[Vec<usize>], st: &mut [u8], out: &mut Vec<usize>, p: &PipelineFile) -> Result<()> {
        match st[i] {
            2 => return Ok(()),
            1 => {
                return Err(Error::new("PX7103", format!("the pipeline has a cycle through step '{}'", p.steps[i].id)))
            }
            _ => {}
        }
        st[i] = 1;
        for &d in &deps[i] {
            visit(d, deps, st, out, p)?;
        }
        st[i] = 2;
        out.push(i);
        Ok(())
    }
    for i in 0..p.steps.len() {
        visit(i, &deps, &mut state, &mut out, p)?;
    }
    Ok(out)
}

fn first_need(p: &PipelineFile, i: usize) -> Option<usize> {
    match &p.steps[i].needs {
        Some(n) => n.first().and_then(|d| p.steps.iter().position(|s| &s.id == d)),
        None if i > 0 => Some(i - 1),
        None => None,
    }
}

/// Variable scopes for `${scope.key}`.
#[derive(Clone, Debug, Default)]
pub struct Bindings {
    pub vars: Map<String, Value>,
    pub inputs: Map<String, Value>,
    pub scopes: BTreeMap<String, Map<String, Value>>,
}

impl Bindings {
    fn lookup(&self, key: &str) -> Option<String> {
        let (scope, name) = key.split_once('.')?;
        let v = match scope {
            "vars" => self.vars.get(name),
            "inputs" => self.inputs.get(name),
            other => self.scopes.get(other).and_then(|m| m.get(name)),
        }?;
        scalar_to_string(v).ok()
    }
}

/// Unresolved variables, collected instead of failing (for validation).
#[derive(Default)]
struct Missing(Vec<String>);

fn interp_str(s: &str, b: &Bindings, missing: &mut Missing) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find('}') else {
            out.push_str(&rest[start..]);
            return out;
        };
        let key = after[..end].trim();
        match b.lookup(key) {
            Some(v) => out.push_str(&v),
            None => {
                missing.0.push(key.to_string());
                out.push_str(&format!("${{{key}}}"));
            }
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

fn interp(v: &Value, b: &Bindings, missing: &mut Missing) -> Value {
    match v {
        Value::String(s) => Value::String(interp_str(s, b, missing)),
        Value::Array(a) => Value::Array(a.iter().map(|x| interp(x, b, missing)).collect()),
        Value::Object(o) => Value::Object(o.iter().map(|(k, x)| (k.clone(), interp(x, b, missing))).collect()),
        other => other.clone(),
    }
}

fn unresolved(missing: Missing, step: &str) -> Result<()> {
    if missing.0.is_empty() {
        return Ok(());
    }
    Err(Error::new("PX7105", format!("step '{step}' uses undefined variable(s): {}", missing.0.join(", ")))
        .with_hint("define them under vars: or pass --var k=v"))
}

/// Special keys of `with:` that are not command parameters.
struct StepIo {
    input: Option<PathBuf>,
    inputs: Vec<PathBuf>,
    output: Option<PathBuf>,
    in_place: bool,
}

fn split_with(with: &Map<String, Value>) -> Result<(StepIo, Vec<(String, String)>)> {
    let mut io = StepIo { input: None, inputs: vec![], output: None, in_place: false };
    let mut rest = Map::new();
    for (k, v) in with {
        match k.as_str() {
            "input" => io.input = Some(PathBuf::from(scalar_to_string(v)?)),
            "inputs" => match v {
                Value::Array(a) => {
                    io.inputs = a.iter().map(|x| scalar_to_string(x).map(PathBuf::from)).collect::<Result<_>>()?
                }
                other => io.inputs = vec![PathBuf::from(scalar_to_string(other)?)],
            },
            "output" => io.output = Some(PathBuf::from(scalar_to_string(v)?)),
            "in-place" | "in_place" => io.in_place = pixelox_core::op::parse_bool(&scalar_to_string(v)?)?,
            _ => {
                rest.insert(k.clone(), v.clone());
            }
        }
    }
    // a map expands a repeatable flag: set: {headline: X} → --set headline=X
    let mut pairs = Vec::new();
    for (k, v) in &rest {
        if let Value::Object(m) = v {
            for (mk, mv) in m {
                pairs.push((k.clone(), format!("{mk}={}", scalar_to_string(mv)?)));
            }
        } else {
            pairs.extend(json_to_pairs(&json!({ k: v }))?);
        }
    }
    if let Some(i) = &io.input {
        if io.inputs.is_empty() {
            io.inputs = vec![i.clone()];
        }
    }
    Ok((io, pairs))
}

fn no_record(spec: &OpSpec) -> bool {
    matches!(spec.name, "save" | "history undo" | "history checkpoint")
}

/// Give every output without an explicit recipe this step's own recipe.
pub fn pin_recipes(child: &mut OpCtx, spec: &OpSpec, args: &Args) {
    let step = make_step(spec, args, child);
    for o in child.outputs.iter_mut() {
        if o.recipe.is_none() {
            o.recipe = Some(vec![step.clone()]);
        }
    }
}

/// Run one step in `child`. Returns the step's document (if any) and its result.
fn run_step(
    child: &mut OpCtx,
    spec: &OpSpec,
    io: &StepIo,
    pairs: Vec<(String, String)>,
    prev: Option<Document>,
    cache: &HashMap<PathBuf, Document>,
) -> Result<(Option<Document>, Value)> {
    let args = Args::build(spec, pairs)?;
    child.inputs = io.inputs.clone();
    child.globals.output = io.output.as_ref().map(|p| p.display().to_string());
    child.globals.in_place = io.in_place;
    match spec.runner {
        Runner::Doc(f) => {
            let mut doc = match &io.input {
                Some(p) => match cache.get(&absolutize(p)) {
                    Some(d) => d.clone(),
                    None => child.load_document(p)?,
                },
                None => prev.ok_or_else(|| {
                    Error::new(
                        "PX7102",
                        format!("step '{}' has no input and no earlier document to work on", spec.id()),
                    )
                    .with_hint("give it `input:` or `needs:` a step that produces a document")
                })?,
            };
            child.op_index = doc.recipe.len();
            let v = f(&args, &mut doc, child)?;
            if spec.mode == DocMode::Mutate {
                doc.quantize_to_storage();
                if !no_record(spec) {
                    let step = make_step(spec, &args, child);
                    doc.recipe.push(step);
                }
                let target = if io.in_place {
                    let p = io.input.clone().ok_or_else(|| Error::new("PX1402", "in-place needs an input .pxo"))?;
                    if !is_pxo(&p) {
                        return Err(Error::new(
                            "PX1108",
                            format!("{} is a source image; write a new file with output:", p.display()),
                        ));
                    }
                    Some(p)
                } else {
                    io.output.clone()
                };
                if let Some(t) = target {
                    write_document(child, &doc, &t)?;
                }
            }
            pin_recipes(child, spec, &args);
            Ok((Some(doc), v))
        }
        Runner::Source(f) => {
            let mut doc = f(&args, child)?;
            doc.quantize_to_storage();
            if doc.recipe.is_empty() {
                let step = make_step(spec, &args, child);
                doc.recipe.push(step);
            }
            if let Some(t) = &io.output {
                write_document(child, &doc, t)?;
            }
            Ok((Some(doc), Value::Null))
        }
        Runner::Custom(_) => {
            let v = run_spec(spec, &args, child)?;
            pin_recipes(child, spec, &args);
            Ok((None, v))
        }
    }
}

fn csv_rows(text: &str) -> Vec<Map<String, Value>> {
    fn split(line: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut cur = String::new();
        let mut q = false;
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '"' if q && chars.peek() == Some(&'"') => {
                    cur.push('"');
                    chars.next();
                }
                '"' => q = !q,
                ',' if !q => out.push(std::mem::take(&mut cur)),
                _ => cur.push(c),
            }
        }
        out.push(cur);
        out.into_iter().map(|s| s.trim().to_string()).collect()
    }
    let mut lines = text.lines().filter(|l| !l.trim().is_empty());
    let Some(header) = lines.next() else { return vec![] };
    let cols = split(header);
    lines
        .map(|l| {
            let vals = split(l);
            cols.iter()
                .enumerate()
                .map(|(i, c)| (c.clone(), Value::String(vals.get(i).cloned().unwrap_or_default())))
                .collect()
        })
        .collect()
}

/// `item` bindings for a file path.
pub fn item_binding(path: &Path, n: usize, out_dir: Option<&Path>) -> Map<String, Value> {
    let stem = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let ext = path.extension().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let dir = path.parent().map(|p| p.display().to_string()).unwrap_or_default();
    let mut m = Map::new();
    m.insert("in".into(), json!(path.display().to_string()));
    m.insert("stem".into(), json!(stem));
    m.insert("ext".into(), json!(ext));
    m.insert("dir".into(), json!(dir));
    m.insert("n".into(), json!(n.to_string()));
    if let Some(d) = out_dir {
        m.insert("out".into(), json!(d.join(format!("{stem}.{ext}")).display().to_string()));
    }
    m
}

fn foreach_items(f: &Foreach, b: &Bindings) -> Result<(String, Vec<Map<String, Value>>)> {
    let mut missing = Missing::default();
    if let Some(g) = &f.glob {
        let g = interp_str(g, b, &mut missing);
        unresolved(missing, "foreach")?;
        let mut paths: Vec<PathBuf> = glob::glob(&g)
            .map_err(|e| invalid(format!("bad glob '{g}': {e}")))?
            .flatten()
            .filter(|p| !p.to_string_lossy().ends_with(".pixelox.json"))
            .collect();
        paths.sort();
        let out_dir = b.scopes.get("batch").and_then(|m| m.get("out_dir")).and_then(|v| v.as_str()).map(PathBuf::from);
        let items = paths.iter().enumerate().map(|(i, p)| item_binding(p, i + 1, out_dir.as_deref())).collect();
        return Ok((f.as_name.clone().unwrap_or_else(|| "item".into()), items));
    }
    if let Some(c) = &f.csv {
        let c = interp_str(c, b, &mut missing);
        unresolved(missing, "foreach")?;
        let text = std::fs::read_to_string(&c).map_err(|_| Error::input_missing(Path::new(&c)))?;
        return Ok((f.as_name.clone().unwrap_or_else(|| "row".into()), csv_rows(&text)));
    }
    Err(invalid("foreach needs glob: or csv:"))
}

fn parse_kv(list: &[String], what: &str) -> Result<Vec<(String, String)>> {
    list.iter()
        .map(|s| {
            s.split_once('=')
                .map(|(k, v)| (k.trim().to_string(), v.to_string()))
                .ok_or_else(|| Error::new("PX1010", format!("--{what} expects k=v, got '{s}'")))
        })
        .collect()
}

fn bindings_for(p: &PipelineFile, vars: &[(String, String)], binds: &[(String, String)]) -> Bindings {
    let mut b = Bindings::default();
    for (k, v) in &p.vars {
        b.vars.insert(k.clone(), v.clone());
    }
    for (k, v) in vars {
        b.vars.insert(k.clone(), json!(v));
    }
    for (k, v) in &p.inputs {
        b.inputs.insert(k.clone(), json!(v.path()));
    }
    for (k, v) in binds {
        if let Some((scope, name)) = k.split_once('.') {
            b.scopes.entry(scope.to_string()).or_default().insert(name.to_string(), json!(v));
        }
    }
    b
}

fn check_pins(p: &PipelineFile, b: &Bindings) -> Result<()> {
    for (name, decl) in &p.inputs {
        if let InputDecl::Pinned { path, sha256: Some(sha) } = decl {
            let mut m = Missing::default();
            let path = interp_str(path, b, &mut m);
            let actual = pixelox_core::envelope::sha256_file(Path::new(&path))
                .map_err(|_| Error::input_missing(Path::new(&path)))?;
            if !actual.starts_with(sha.trim_end_matches('…')) {
                return Err(Error::new(
                    "PX1601",
                    format!("pipeline input '{name}' ({path}) does not match its pinned sha256"),
                )
                .with_context("expected", sha)
                .with_context("actual", actual));
            }
        }
    }
    Ok(())
}

/// Execute a pipeline in `ctx` (outputs are absorbed into it for sidecars).
pub fn execute(ctx: &mut OpCtx, p: &PipelineFile, b: &Bindings, jobs: usize, file: &Path) -> Result<Value> {
    let registry: &Registry = ctx.registry()?;
    let ord = order(p)?;
    check_pins(p, b)?;
    let continue_on_error = p.defaults.on_error.as_deref() == Some("continue");
    if let Some(s) = p.defaults.seed {
        ctx.globals.seed = s;
    }
    let mut docs: HashMap<usize, Document> = HashMap::new();
    let mut cache: HashMap<PathBuf, Document> = HashMap::new();
    let mut failed: Vec<usize> = Vec::new();
    let mut report = Vec::new();
    for (n, &i) in ord.iter().enumerate() {
        ctx.check_cancel()?;
        let step = &p.steps[i];
        ctx.progress.stage(&step.id, n + 1, ord.len());
        let spec = registry.require(&step.run).map_err(|e| invalid(format!("step '{}': {}", step.id, e.message)))?;
        if let Some(need) = first_need(p, i) {
            if failed.contains(&need) {
                failed.push(i);
                report.push(json!({"id": step.id, "run": spec.id(), "ok": false, "skipped": true}));
                continue;
            }
        }
        let prev = first_need(p, i).and_then(|j| docs.get(&j).cloned());
        let items: Vec<Bindings> = match &step.foreach {
            None => vec![b.clone()],
            Some(f) => {
                let (scope, rows) = foreach_items(f, b)?;
                rows.into_iter()
                    .map(|r| {
                        let mut bb = b.clone();
                        bb.scopes.insert(scope.clone(), r);
                        bb
                    })
                    .collect()
            }
        };
        // resolve each item's parameters up front
        let mut prepared = Vec::new();
        for bb in &items {
            let mut missing = Missing::default();
            let with = match interp(&Value::Object(step.with.clone()), bb, &mut missing) {
                Value::Object(m) => m,
                _ => Map::new(),
            };
            unresolved(missing, &step.id)?;
            prepared.push(split_with(&with)?);
        }
        let mut results: Vec<(OpCtx, Result<(Option<Document>, Value)>)> = Vec::new();
        let chunk = jobs.max(1);
        let mut queue: Vec<(StepIo, Vec<(String, String)>)> = prepared;
        while !queue.is_empty() {
            let take = queue.len().min(chunk);
            let batch: Vec<(StepIo, Vec<(String, String)>)> = queue.drain(..take).collect();
            let children: Vec<OpCtx> = batch.iter().map(|_| ctx.child(spec.name, vec![])).collect();
            let cache_ref = &cache;
            let done: Vec<(OpCtx, Result<(Option<Document>, Value)>)> = if batch.len() == 1 {
                let (io, pairs) = batch.into_iter().next().unwrap();
                let mut child = children.into_iter().next().unwrap();
                child.op_index = 0;
                let r = run_step(&mut child, spec, &io, pairs, prev.clone(), cache_ref);
                vec![(child, r)]
            } else {
                std::thread::scope(|s| {
                    let handles: Vec<_> = batch
                        .into_iter()
                        .zip(children)
                        .map(|((io, pairs), mut child)| {
                            let prev = prev.clone();
                            s.spawn(move || {
                                let r = run_step(&mut child, spec, &io, pairs, prev, cache_ref);
                                (child, r)
                            })
                        })
                        .collect();
                    handles.into_iter().map(|h| h.join().expect("pipeline step thread panicked")).collect()
                })
            };
            results.extend(done);
        }
        let mut step_ok = true;
        let mut outputs = Vec::new();
        let mut item_results = Vec::new();
        let mut last_doc = None;
        for (child, r) in results {
            match r {
                Ok((doc, v)) => {
                    for o in &child.outputs {
                        outputs.push(o.record.path.clone());
                        if is_pxo(Path::new(&o.record.path)) {
                            if let Some(d) = &doc {
                                cache.insert(absolutize(Path::new(&o.record.path)), d.clone());
                            }
                        }
                    }
                    item_results.push(v);
                    last_doc = doc;
                    ctx.absorb(child);
                }
                Err(e) => {
                    step_ok = false;
                    for t in &child.temp_files {
                        let _ = std::fs::remove_file(t);
                    }
                    if !continue_on_error {
                        // keep what earlier steps wrote: their outputs are complete deliverables
                        return Err(Error::new(
                            &e.code,
                            format!("step '{}' ({}) failed: {}", step.id, spec.id(), e.message),
                        )
                        .with_context("step", &step.id));
                    }
                    ctx.warn(
                        "PXW3001",
                        format!("step '{}' failed and on-error is continue: {}: {}", step.id, e.code, e.message),
                    );
                    item_results.push(json!({"error": e}));
                }
            }
        }
        if step.foreach.is_none() {
            if let Some(d) = last_doc {
                docs.insert(i, d);
            }
        }
        if !step_ok {
            failed.push(i);
        }
        report.push(json!({
            "id": step.id,
            "run": spec.id(),
            "ok": step_ok,
            "items": item_results.len(),
            "outputs": outputs,
            "result": if step.foreach.is_some() { Value::Array(item_results) } else { item_results.into_iter().next().unwrap_or(Value::Null) },
        }));
    }
    let mut declared = Vec::new();
    for o in &p.outputs {
        let mut m = Missing::default();
        let path = interp_str(o, b, &mut m);
        if !ctx.dry_run() && !Path::new(&path).exists() {
            return Err(Error::new("PX7104", format!("declared output {path} was not written")));
        }
        declared.push(path);
    }
    let name = p.name.clone().unwrap_or_else(|| {
        file.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "pipeline".into())
    });
    let run = json!({
        "pixelox_pipeline_run": 1,
        "pipeline": file.display().to_string(),
        "name": name,
        "created_utc": now_utc(),
        "vars": b.vars,
        "steps": report,
        "outputs": declared,
    });
    if let Some(first) = declared.first() {
        if !ctx.dry_run() {
            let dir = Path::new(first).parent().map(|d| d.to_path_buf()).unwrap_or_default();
            let path = dir.join(format!("{name}.pipeline.pixelox.json"));
            let bytes = serde_json::to_vec_pretty(&run)?;
            ctx.write_file(&path, &bytes, Some("pipeline-run"), Some(vec![]))?;
        }
    }
    Ok(json!({"pipeline": name, "steps": run["steps"], "outputs": declared, "failed_steps": failed.len()}))
}

fn plan(ctx: &mut OpCtx, p: &PipelineFile, b: &Bindings, validate_only: bool) -> Result<Value> {
    let registry = ctx.registry()?;
    let ord = order(p)?;
    if !validate_only {
        check_pins(p, b)?;
    }
    let mut steps = Vec::new();
    for &i in &ord {
        let step = &p.steps[i];
        let spec = registry.require(&step.run).map_err(|e| invalid(format!("step '{}': {}", step.id, e.message)))?;
        let mut items = 1usize;
        let mut bb = b.clone();
        let mut deferred = false;
        if let Some(f) = &step.foreach {
            match foreach_items(f, b) {
                Ok((scope, rows)) => {
                    items = rows.len();
                    if let Some(first) = rows.into_iter().next() {
                        bb.scopes.insert(scope, first);
                    } else {
                        deferred = true;
                    }
                }
                Err(_) => deferred = true,
            }
        }
        let mut missing = Missing::default();
        let with = match interp(&Value::Object(step.with.clone()), &bb, &mut missing) {
            Value::Object(m) => m,
            _ => Map::new(),
        };
        let runtime_scope = missing.0.iter().all(|k| !k.starts_with("vars.") && !k.starts_with("inputs."));
        if !missing.0.is_empty() && !runtime_scope {
            unresolved(missing, &step.id)?;
        } else if !missing.0.is_empty() {
            deferred = true;
        }
        let (io, pairs) = split_with(&with)?;
        if !deferred {
            Args::build(spec, pairs.clone())
                .map_err(|e| Error::new(&e.code, format!("step '{}': {}", step.id, e.message)))?;
        }
        steps.push(json!({
            "id": step.id,
            "run": spec.id(),
            "needs": step.needs.clone().unwrap_or_else(|| first_need(p, i).map(|j| vec![p.steps[j].id.clone()]).unwrap_or_default()),
            "items": items,
            "input": io.input.map(|p| p.display().to_string()),
            "output": io.output.map(|p| p.display().to_string()),
            "with": with,
            "validated": !deferred,
        }));
    }
    Ok(
        json!({"valid": true, "order": ord.iter().map(|&i| p.steps[i].id.clone()).collect::<Vec<_>>(), "steps": steps, "outputs": p.outputs}),
    )
}

fn pipeline_run(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let path = ctx.input(0)?.to_path_buf();
    let (p, _) = load(ctx, &path)?;
    let vars = parse_kv(&a.all("var"), "var")?;
    let binds = parse_kv(&a.all("bind"), "bind")?;
    let b = bindings_for(&p, &vars, &binds);
    if ctx.dry_run() {
        let plan = plan(ctx, &p, &b, false)?;
        return Ok(json!({"dry_run": true, "plan": plan}));
    }
    execute(ctx, &p, &b, a.usize("jobs")?, &path)
}

fn pipeline_validate(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let path = ctx.input(0)?.to_path_buf();
    let (p, _) = load(ctx, &path)?;
    let vars = parse_kv(&a.all("var"), "var")?;
    let b = bindings_for(&p, &vars, &[]);
    check_pins(&p, &b)?;
    plan(ctx, &p, &b, true)
}

fn pipeline_graph(_a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let path = ctx.input(0)?.to_path_buf();
    let (p, _) = load(ctx, &path)?;
    order(&p)?;
    let mut edges = Vec::new();
    for (i, s) in p.steps.iter().enumerate() {
        let deps: Vec<String> = match &s.needs {
            Some(n) => n.clone(),
            None if i > 0 => vec![p.steps[i - 1].id.clone()],
            None => vec![],
        };
        for d in deps {
            edges.push((d, s.id.clone()));
        }
    }
    let dot = ctx.globals.format.as_deref() == Some("dot");
    let text = if dot {
        let mut s = String::from("digraph pipeline {\n  rankdir=LR;\n");
        for st in &p.steps {
            s.push_str(&format!("  \"{}\" [label=\"{}\\n{}\"];\n", st.id, st.id, st.run));
        }
        for (a, b) in &edges {
            s.push_str(&format!("  \"{a}\" -> \"{b}\";\n"));
        }
        s.push_str("}\n");
        s
    } else {
        let mut s = String::new();
        for st in &p.steps {
            s.push_str(&format!("{} ({})\n", st.id, st.run));
        }
        for (a, b) in &edges {
            s.push_str(&format!("{a} -> {b}\n"));
        }
        s
    };
    Ok(
        json!({"nodes": p.steps.iter().map(|s| json!({"id": s.id, "run": s.run})).collect::<Vec<_>>(), "edges": edges.iter().map(|(a, b)| json!([a, b])).collect::<Vec<_>>(), "text": text}),
    )
}

const STARTER: &str = "# pixelox pipeline (see `pixelox docs pipelines`)
version: 1
name: example
vars:
  sigma: \"3\"
inputs:
  photo: photo.jpg
steps:
  - id: soften
    run: filter.gaussian
    with: { input: \"${inputs.photo}\", sigma: \"${vars.sigma}\" }
  - id: deliver
    run: export
    with: { output: out/photo-soft.png }
outputs: [out/photo-soft.png]
";

fn pipeline_new(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let out =
        PathBuf::from(ctx.globals.output.clone().ok_or_else(|| Error::new("PX1004", "pipeline new needs -o p.yml"))?);
    let text = match a.path("from-sidecar") {
        None => STARTER.to_string(),
        Some(sc_path) => {
            ctx.check_allowed(&sc_path)?;
            let sc = pixelox_core::Sidecar::read(&sc_path)?;
            let mut inputs = Map::new();
            let mut steps = Vec::new();
            for (i, s) in sc.recipe.iter().enumerate() {
                let mut with = s.params.clone();
                for inp in &s.inputs {
                    if inp.role == "source" {
                        inputs.insert(
                            format!("source{}", if i == 0 { String::new() } else { i.to_string() }),
                            json!({"path": inp.path, "sha256": inp.sha256}),
                        );
                    }
                }
                match s.op.as_str() {
                    "open" => {
                        let mut w = with.clone();
                        w.insert("input".into(), json!("${inputs.source}"));
                        steps.push(json!({"id": "open", "run": "open", "with": w}));
                    }
                    "export" | "save" => {
                        with.insert("output".into(), json!(sc.output.path));
                        steps.push(json!({"id": "deliver", "run": if s.op == "save" { "save" } else { "export" }, "with": with}));
                    }
                    op => {
                        if steps.is_empty() {
                            if let Some(src) = s.inputs.iter().find(|x| x.role == "source") {
                                inputs.insert("source".into(), json!({"path": src.path, "sha256": src.sha256}));
                                with.insert("input".into(), json!("${inputs.source}"));
                            }
                        }
                        steps.push(json!({"id": format!("step{}", i + 1), "run": op, "with": with}));
                    }
                }
            }
            let doc = json!({
                "version": 1,
                "name": Path::new(&sc.output.path).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "replayed".into()),
                "description": format!("Generated from {} (pixelox {})", sc_path.display(), sc.tool.version),
                "inputs": inputs,
                "steps": steps,
                "outputs": [sc.output.path],
            });
            serde_yaml::to_string(&doc).map_err(|e| Error::internal(e.to_string()))?
        }
    };
    parse_text(&text)?;
    ctx.write_file(&out, text.as_bytes(), None, None)?;
    Ok(
        json!({"pipeline": out.display().to_string(), "steps": parse_text(&text)?.steps.len(), "sha256": sha256_bytes(text.as_bytes())}),
    )
}

/// Unused-entry helper so OutputEntry stays in the public API surface of this module.
pub fn output_paths(outputs: &[OutputEntry]) -> Vec<String> {
    outputs.iter().map(|o| o.record.path.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const POSTER: &str = r#"
version: 1
name: poster
vars: { headline: "FALL DROP 09" }
inputs:
  template: assets/poster.pxo
steps:
  - id: fill
    run: comp.template.apply
    with: { input: "${inputs.template}", set: { headline: "${vars.headline}" }, output: work/poster.pxo }
  - id: print
    run: export
    needs: [fill]
    with: { input: work/poster.pxo, output: dist/poster.png }
  - id: og
    run: export
    needs: [fill]
    with: { input: work/poster.pxo, output: dist/og.png, size: 1200x630 }
outputs: [dist/poster.png, dist/og.png]
"#;

    #[test]
    fn parses_orders_and_interpolates() {
        let p = parse_text(POSTER).unwrap();
        assert_eq!(order(&p).unwrap(), vec![0, 1, 2]);
        let b = bindings_for(&p, &[("headline".into(), "WINTER".into())], &[]);
        let mut m = Missing::default();
        let w = interp(&Value::Object(p.steps[0].with.clone()), &b, &mut m);
        assert!(m.0.is_empty());
        assert_eq!(w["set"]["headline"], "WINTER");
        assert_eq!(w["input"], "assets/poster.pxo");
        let (io, pairs) = split_with(w.as_object().unwrap()).unwrap();
        assert_eq!(io.output.unwrap(), PathBuf::from("work/poster.pxo"));
        assert_eq!(pairs, vec![("set".to_string(), "headline=WINTER".to_string())]);
    }

    #[test]
    fn graph_errors() {
        let cyc = "version: 1\nsteps:\n  - {id: a, run: info, needs: [b]}\n  - {id: b, run: info, needs: [a]}\n";
        assert_eq!(order(&parse_text(cyc).unwrap()).unwrap_err().code, "PX7103");
        let bad = "version: 1\nsteps:\n  - {id: a, run: info, needs: [zzz]}\n";
        assert_eq!(order(&parse_text(bad).unwrap()).unwrap_err().code, "PX7102");
        assert_eq!(parse_text("version: 2\nsteps: []\n").unwrap_err().code, "PX7101");
        let mut m = Missing::default();
        let _ = interp_str("${vars.nope}", &Bindings::default(), &mut m);
        assert_eq!(unresolved(m, "x").unwrap_err().code, "PX7105");
    }

    #[test]
    fn csv_and_items() {
        let rows = csv_rows("name,title\n\"Ada, L.\",Engineer\nGrace,\"Rear \"\"Admiral\"\"\"\n");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0]["name"], "Ada, L.");
        assert_eq!(rows[1]["title"], "Rear \"Admiral\"");
        let it = item_binding(Path::new("in/a.jpg"), 3, Some(Path::new("out")));
        assert_eq!(it["stem"], "a");
        assert_eq!(it["out"], "out/a.jpg");
    }
}
