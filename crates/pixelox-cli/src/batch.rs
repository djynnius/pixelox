//! `batch run` (PRD §8.20): one pipeline or command template per matched file. Each item
//! runs through the full executor, so it gets its own envelope, atomic writes, and
//! sidecars; item envelopes stream as NDJSON with `--progress json`.

use crate::pipeline::item_binding;
use pixelox_core::exec::{Invocation, Runtime};
use pixelox_core::prelude::*;
use std::path::{Path, PathBuf};

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::custom("batch run", Category::Automation, "Run a pipeline or command over many files", batch_run)
        .inputs(Inputs::NONE)
        .about("Matches --glob (sorted; sidecars skipped) and runs, per file, either a pipeline (bindings ${item.in} ${item.stem} ${item.ext} ${item.dir} ${item.n} ${item.out}) or a command template (--cmd with {in} {stem} {ext} {dir} {n} {out}; no shell is involved). ${item.out} / {out} is --out-dir/<stem>.<ext>. Items run in parallel up to --jobs; each writes its own outputs and sidecars. Any failed item makes the exit code non-zero (PX7301) unless --continue-on-error, which still reports it.")
        .param(param("glob", ParamKind::Str).required().help("Files to process, e.g. 'shoot/*.jpg' (quote it)"))
        .param(param("pipeline", ParamKind::Path).help("Pipeline file run once per item"))
        .param(param("cmd", ParamKind::Str).help("Command template, e.g. 'export {in} -o out/{stem}.webp'"))
        .param(param("out-dir", ParamKind::Path).help("Directory for {out} / ${item.out}"))
        .param(param("jobs", ParamKind::Int).default("1").range("1..256").help("Items processed in parallel"))
        .param(param("continue-on-error", ParamKind::Flag).help("Keep going after a failed item and exit 0 (failures stay in the report)"))
        .param(param("manifest", ParamKind::Path).help("Also write the per-item report to this JSON file"))
        .param(param("var", ParamKind::Str).repeatable().help("Pipeline variable override k=v"))
        .example("pixelox batch run --glob 'shoot/*.jpg' --pipeline watermark.yml --out-dir out/ --jobs 8 --manifest out/manifest.json --progress json", "Watermark a shoot")
        .example("pixelox batch run --glob 'in/*.jpg' --cmd 'export {in} -o out/{stem}.webp' --jobs 4", "Convert a folder")
        .example("pixelox batch run --glob 'raw/*.png' --cmd 'adjust auto {in} -o fixed/{stem}.png --tone' --continue-on-error --json", "Auto-tone everything, report failures")]
}

/// Split a command line like a shell would (quotes, backslash escapes; no expansion).
pub fn shell_split(s: &str) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    let mut any = false;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match quote {
            Some(q) if c == q => quote = None,
            Some('"') if c == '\\' => {
                if let Some(n) = chars.next() {
                    cur.push(n);
                }
            }
            Some(_) => cur.push(c),
            None if c == '"' || c == '\'' => {
                quote = Some(c);
                any = true;
            }
            None if c == '\\' => {
                if let Some(n) = chars.next() {
                    cur.push(n);
                }
            }
            None if c.is_whitespace() => {
                if !cur.is_empty() || any {
                    out.push(std::mem::take(&mut cur));
                    any = false;
                }
            }
            None => cur.push(c),
        }
    }
    if quote.is_some() {
        return Err(Error::new("PX1010", format!("unbalanced quote in --cmd '{s}'")));
    }
    if !cur.is_empty() || any {
        out.push(cur);
    }
    Ok(out)
}

fn fill_template(t: &str, item: &Map<String, Value>) -> String {
    let mut s = t.to_string();
    for (k, v) in item {
        s = s.replace(&format!("{{{k}}}"), v.as_str().unwrap_or(""));
    }
    s
}

/// Quote a value for re-splitting.
fn quote(v: &str) -> String {
    if v.chars().any(|c| c.is_whitespace() || c == '\'' || c == '"' || c == '\\') {
        format!("'{}'", v.replace('\'', "'\\''"))
    } else {
        v.to_string()
    }
}

fn batch_run(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let registry = ctx.registry()?;
    let pattern = a.str_or("glob", "");
    let pipeline = a.path("pipeline");
    let cmd = a.str("cmd");
    match (&pipeline, &cmd) {
        (Some(_), Some(_)) => return Err(Error::new("PX1005", "pass --pipeline or --cmd, not both")),
        (None, None) => return Err(Error::new("PX1003", "batch run needs --pipeline FILE or --cmd 'TEMPLATE'")),
        _ => {}
    }
    let mut files: Vec<PathBuf> = glob::glob(&pattern)
        .map_err(|e| Error::new("PX1010", format!("bad --glob '{pattern}': {e}")))?
        .flatten()
        .filter(|p| p.is_file() && !p.to_string_lossy().ends_with(".pixelox.json"))
        .collect();
    files.sort();
    if files.is_empty() {
        return Err(Error::new("PX1101", format!("no files match '{pattern}'")).with_context("glob", &pattern));
    }
    for f in &files {
        ctx.check_allowed(f)?;
    }
    let out_dir = a.path("out-dir");
    if let Some(d) = &out_dir {
        ctx.check_allowed(d)?;
        if !ctx.dry_run() {
            std::fs::create_dir_all(d)?;
        }
    }
    let jobs = a.usize("jobs")?.max(1);
    let vars = a.all("var");
    // build one invocation per item
    let mut invs: Vec<(PathBuf, Invocation)> = Vec::new();
    for (i, f) in files.iter().enumerate() {
        let item = item_binding(f, i + 1, out_dir.as_deref());
        let mut inv = match (&pipeline, &cmd) {
            (Some(p), _) => {
                let mut args: Vec<(String, String)> = item
                    .iter()
                    .map(|(k, v)| ("bind".to_string(), format!("item.{k}={}", v.as_str().unwrap_or(""))))
                    .collect();
                for v in &vars {
                    args.push(("var".into(), v.clone()));
                }
                Invocation {
                    command: "pipeline run".into(),
                    args,
                    inputs: vec![p.display().to_string()],
                    globals: ctx.globals.clone(),
                    argv: vec![],
                }
            }
            (None, Some(t)) => {
                let quoted: Map<String, Value> =
                    item.iter().map(|(k, v)| (k.clone(), json!(quote(v.as_str().unwrap_or(""))))).collect();
                let line = fill_template(t, &quoted);
                let mut argv = vec!["pixelox".to_string()];
                argv.extend(shell_split(&line)?);
                let (mut inv, _) = crate::cli::parse_argv(registry, &argv)?;
                // the batch's sandbox and plan mode always apply
                inv.globals.allow = ctx.globals.allow.clone();
                inv.globals.read_only = ctx.globals.read_only;
                inv.globals.dry_run |= ctx.globals.dry_run;
                inv.globals.sidecar &= ctx.globals.sidecar;
                inv.globals.redact_paths |= ctx.globals.redact_paths;
                inv
            }
            _ => unreachable!(),
        };
        if inv.argv.is_empty() {
            inv.argv = vec!["batch".into(), "item".into(), f.display().to_string()];
        }
        invs.push((f.clone(), inv));
    }
    let rt = Runtime { registry, io: ctx.io, progress: ctx.progress, cancel: ctx.cancel.clone() };
    let mut items = Vec::new();
    let mut failed = 0usize;
    let mut stop = false;
    for chunk in invs.chunks(jobs) {
        if stop {
            break;
        }
        ctx.check_cancel()?;
        let rt_ref = &rt;
        let outcomes: Vec<(PathBuf, pixelox_core::Envelope)> = std::thread::scope(|s| {
            let handles: Vec<_> = chunk
                .iter()
                .map(|(f, inv)| s.spawn(move || (f.clone(), rt_ref.invoke(inv.clone()).envelope)))
                .collect();
            handles.into_iter().map(|h| h.join().expect("batch item thread panicked")).collect()
        });
        for (f, env) in outcomes {
            let n = items.len() + 1;
            let entry = json!({
                "n": n,
                "input": f.display().to_string(),
                "ok": env.ok,
                "command": env.command,
                "outputs": env.outputs,
                "warnings": env.warnings,
                "error": env.error,
                "timing_ms": env.timing_ms,
            });
            ctx.progress.event(&json!({"event": "item", "n": n, "of": invs.len(), "input": f.display().to_string(), "ok": env.ok, "envelope": env}));
            if !env.ok {
                failed += 1;
                if !a.flag("continue-on-error") {
                    stop = true;
                }
            }
            items.push(entry);
        }
    }
    let skipped = invs.len() - items.len();
    let report = json!({"glob": pattern, "items": items, "succeeded": items.len() - failed, "failed": failed, "skipped": skipped});
    if let Some(m) = a.path("manifest") {
        ctx.write_file(&m, &serde_json::to_vec_pretty(&report)?, Some("manifest"), Some(vec![]))?;
    }
    if failed > 0 {
        if a.flag("continue-on-error") {
            ctx.warn("PXW3001", format!("{failed} of {} items failed (see result.items)", invs.len()));
        } else {
            return Err(Error::new("PX7301", format!("{failed} item(s) failed; {skipped} not started"))
                .with_context("report", &report));
        }
    }
    Ok(report)
}

/// For tests: where `{out}` points for a file.
pub fn out_path(out_dir: &Path, file: &Path) -> PathBuf {
    item_binding(file, 1, Some(out_dir))["out"].as_str().map(PathBuf::from).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitting_and_templates() {
        assert_eq!(
            shell_split("export 'my file.jpg' -o \"out dir/x.png\" --q 9").unwrap(),
            vec!["export", "my file.jpg", "-o", "out dir/x.png", "--q", "9"]
        );
        assert!(shell_split("a 'b").is_err());
        let mut m = Map::new();
        m.insert("in".into(), json!(quote("a b.jpg")));
        m.insert("stem".into(), json!("ab"));
        let line = fill_template("export {in} -o out/{stem}.png", &m);
        assert_eq!(shell_split(&line).unwrap(), vec!["export", "a b.jpg", "-o", "out/ab.png"]);
    }
}
