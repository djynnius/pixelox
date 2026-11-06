//! Documentation generated from op specs (help text is written once, on the OpSpec):
//! command reference pages, the error catalog, `llms.txt`, and the mdBook site source.

use pixelox_core::error::{ErrorKind, CATALOG, WARNINGS};
use pixelox_core::op::{OpSpec, ParamKind};
use pixelox_core::{schema, Registry};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;

pub const SITE_BASE: &str = "https://djynnius.github.io/pixelox";

fn md_escape(s: &str) -> String {
    s.replace('|', "\\|")
}

/// One reference page per command.
pub fn command_page(op: &OpSpec) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "# pixelox {}\n", op.name);
    let _ = writeln!(s, "{}\n", op.summary);
    if let Some(m) = op.milestone {
        let _ = writeln!(s, "> **Milestone {m}.** {}\n", gate_note(op));
    }
    let inputs = match (op.inputs.min, op.inputs.max) {
        (0, Some(0)) => String::new(),
        (0, Some(1)) => " [INPUT]".to_string(),
        (_, Some(1)) => " INPUT".to_string(),
        (_, Some(n)) => format!(" INPUT×{n}"),
        _ => " INPUTS...".to_string(),
    };
    let pos: String = op.positionals.iter().map(|p| format!(" <{p}>")).collect();
    let out = if op.is_read_only() {
        ""
    } else if op.is_doc() {
        " (-o OUT | --in-place)"
    } else {
        " [-o OUT]"
    };
    let _ = writeln!(s, "## Synopsis\n\n```text\npixelox {}{inputs}{pos} [options]{out}\n```\n", op.name);
    let _ = writeln!(s, "## What it does\n\n{}\n", if op.about.is_empty() { op.summary } else { op.about });
    if op.is_doc() && !op.is_read_only() && op.mode == pixelox_core::DocMode::Mutate {
        let _ = writeln!(
            s,
            "INPUT may be an image (JPEG, PNG, TIFF, WebP, GIF, BMP, PSD, SVG) or a `.pxo` document. Write the result with `-o` (`.pxo` keeps layers; an image extension flattens) or rewrite a `.pxo` input with `--in-place`. Source images are never edited in place.\n"
        );
    }
    if !op.examples.is_empty() {
        let _ = writeln!(s, "## Examples\n");
        for e in &op.examples {
            let _ = writeln!(s, "{}\n\n```bash\n{}\n```\n", e.explain, e.cmd);
        }
    }
    if !op.params.is_empty() {
        let _ = writeln!(s, "## Options\n\n| Flag | Type | Default | Meaning |\n| --- | --- | --- | --- |");
        for p in &op.params {
            let flag =
                if op.positionals.contains(&p.name) { format!("`<{}>`", p.name) } else { format!("`--{}`", p.name) };
            let ty = match p.kind {
                ParamKind::Choice if !p.choices.is_empty() => p.choices.join(" \\| "),
                k => k.type_name().to_string(),
            };
            let mut meaning = md_escape(p.help);
            if let Some(r) = p.range {
                meaning.push_str(&format!(" Range: {}.", md_escape(r)));
            }
            if p.auto {
                meaning.push_str(" Accepts `auto`.");
            }
            if p.repeatable {
                meaning.push_str(" Repeatable.");
            }
            if p.required {
                meaning.push_str(" **Required.**");
            }
            let _ = writeln!(
                s,
                "| {flag} | {} | {} | {meaning} |",
                md_escape(&ty),
                p.default.map(|d| format!("`{}`", md_escape(d))).unwrap_or_else(|| "—".into())
            );
        }
        let _ = writeln!(s);
    }
    let _ = writeln!(s, "Every command also takes the global options (`-o`, `--json`, `--dry-run`, `--overwrite`, `--in-place`, `--seed`, `-y`, `--progress`, ...); see `pixelox docs contract`.\n");
    let _ = writeln!(s, "## Errors\n\n| Code | Exit | Meaning |\n| --- | --- | --- |");
    for code in schema::error_codes(op) {
        if let Some(e) = pixelox_core::error::catalog_entry(code) {
            let _ = writeln!(s, "| `{}` | {} | {} |", e.code, e.kind.exit_code(), md_escape(e.title));
        }
    }
    let _ = writeln!(s);
    let _ = writeln!(s, "## For agents\n");
    let _ = writeln!(s, "- Command id: `{}` (pipelines `run:`, MCP `pixelox_run`, sidecar recipes)", op.id());
    let _ = writeln!(s, "- Schema: `pixelox schema {}`", op.id());
    let _ = writeln!(
        s,
        "- Mode: {}",
        match (op.is_doc(), op.mode) {
            (true, pixelox_core::DocMode::ReadOnly) => "read-only (writes nothing)",
            (true, pixelox_core::DocMode::Export) => "export (renders deliverables from a document)",
            (true, _) => "mutates a document (`-o` or `--in-place`; `--dry-run` plans)",
            _ => "command",
        }
    );
    let defaults: Vec<String> =
        op.params.iter().filter_map(|p| p.default.map(|d| format!("\"{}\": \"{}\"", p.key(), d))).collect();
    let _ = writeln!(s, "\n```json\n{{\"command\": \"{}\", \"defaults\": {{{}}}}}\n```", op.id(), defaults.join(", "));
    s
}

fn gate_note(op: &OpSpec) -> &'static str {
    match op.category {
        pixelox_core::Category::Detect | pixelox_core::Category::Models => {
            "Not in this release: `--dry-run` returns the plan (model, size, license) and a real run exits 12 (PX1520)."
        }
        _ if op.name == "mask refine" || op.name == "fill content-aware" => {
            "Not in this release: `--dry-run` returns the plan (model, size, license) and a real run exits 12 (PX1520)."
        }
        _ => "Scheduled for this milestone in the PRD.",
    }
}

/// The error catalog page (`pixelox docs errors`).
pub fn error_catalog() -> String {
    let mut s = String::from("# Error catalog\n\nEvery error carries a stable `code`, a `message`, a `hint` (the next action), `docs`, `context` (the offending values), and `exit_code`. Codes are append-only; their meanings never change.\n\n## Exit codes\n\n| Exit | Meaning |\n| --- | --- |\n| 0 | Success (warnings allowed) |\n");
    for k in ErrorKind::all() {
        let _ = writeln!(s, "| {} | {} |", k.exit_code(), k.describe());
    }
    let _ = writeln!(s, "\n## Error codes\n\n| Code | Exit | Meaning | What to do |\n| --- | --- | --- | --- |");
    for e in CATALOG {
        let _ =
            writeln!(s, "| `{}` | {} | {} | {} |", e.code, e.kind.exit_code(), md_escape(e.title), md_escape(e.remedy));
    }
    let _ =
        writeln!(s, "\n## Warning codes\n\nWarnings never change the exit code.\n\n| Code | Meaning |\n| --- | --- |");
    for (c, m) in WARNINGS {
        let _ = writeln!(s, "| `{c}` | {} |", md_escape(m));
    }
    s
}

/// One catalog entry (`pixelox docs errors/PX1401`).
pub fn error_entry(code: &str) -> Option<String> {
    if let Some(e) = pixelox_core::error::catalog_entry(code) {
        return Some(format!(
            "# {}: {}\n\nExit code: {} ({})\n\nWhat to do: {}\n",
            e.code,
            e.title,
            e.kind.exit_code(),
            e.kind.describe(),
            e.remedy
        ));
    }
    WARNINGS
        .iter()
        .find(|(c, _)| *c == code)
        .map(|(c, m)| format!("# {c}\n\nWarning: {m}\n\nWarnings never change the exit code.\n"))
}

/// Built-in topic pages (also published on the docs site).
pub const TOPICS: &[(&str, &str)] = &[
    ("contract", include_str!("../topics/contract.md")),
    ("addressing", include_str!("../topics/addressing.md")),
    ("pipelines", include_str!("../topics/pipelines.md")),
    ("sidecars", include_str!("../topics/sidecars.md")),
    ("mcp", include_str!("../topics/mcp.md")),
];

pub fn topic(name: &str) -> Option<&'static str> {
    TOPICS.iter().find(|(n, _)| *n == name).map(|(_, t)| *t)
}

pub fn llms_txt(reg: &Registry) -> String {
    let mut s = String::from("# pixelox\n\n> A command-line raster and vector graphics workstation for scripts and AI agents: layers, masks, selections, blend modes, adjustments, filters, retouching, type, shapes, layer styles, templates, SVG authoring, tracing, and generative rendering. Every command has --json output, a JSON Schema, a provenance sidecar, and an MCP tool.\n\n");
    let _ = writeln!(s, "## Start here\n\n- [Agent guide]({SITE_BASE}/agents.html): the whole surface, compactly\n- [The agent contract]({SITE_BASE}/topics/contract.html)\n- [Error catalog]({SITE_BASE}/errors.html)\n- [Full reference as one file]({SITE_BASE}/llms-full.txt)\n");
    let mut by_cat: BTreeMap<_, Vec<&OpSpec>> = BTreeMap::new();
    for op in reg.ops() {
        by_cat.entry(op.category).or_default().push(op);
    }
    for (cat, ops) in by_cat {
        let _ = writeln!(s, "## {}\n", cat.title());
        for op in ops {
            let _ = writeln!(s, "- [pixelox {}]({SITE_BASE}/commands/{}.html): {}", op.name, op.id(), op.summary);
        }
        let _ = writeln!(s);
    }
    s
}

pub fn llms_full(reg: &Registry) -> String {
    let mut s = String::from("# pixelox full reference\n\n");
    for (_, t) in TOPICS {
        s.push_str(t);
        s.push_str("\n---\n\n");
    }
    for op in reg.ops() {
        s.push_str(&command_page(op));
        s.push_str("\n---\n\n");
    }
    s.push_str(&error_catalog());
    s
}

fn site_escape(md: &str) -> String {
    let mut out = String::with_capacity(md.len() + 64);
    let mut fenced = false;
    for line in md.split_inclusive('\n') {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            out.push_str(line);
            continue;
        }
        if fenced {
            out.push_str(line);
            continue;
        }
        let mut in_code = false;
        for c in line.chars() {
            match c {
                '`' => {
                    in_code = !in_code;
                    out.push(c);
                }
                '<' if !in_code => out.push_str("\\<"),
                _ => out.push(c),
            }
        }
    }
    out
}

fn first_heading(p: &Path) -> Option<String> {
    let text = std::fs::read_to_string(p).ok()?;
    text.lines().find(|l| l.starts_with("# ")).map(|l| l[2..].trim().to_string())
}

/// Write the mdBook source tree (book.toml + src/) into `out`; returns the page count.
pub fn build_site(reg: &Registry, guides_dir: &Path, out: &Path) -> std::io::Result<usize> {
    let src = out.join("src");
    std::fs::create_dir_all(src.join("commands"))?;
    std::fs::create_dir_all(src.join("topics"))?;
    let mut summary = String::from("# Summary\n\n[Introduction](README.md)\n\n");
    // hand-written guides
    let mut intro = false;
    if guides_dir.is_dir() {
        let mut entries: Vec<std::path::PathBuf> = std::fs::read_dir(guides_dir)?.flatten().map(|e| e.path()).collect();
        entries.sort_by_key(|p| {
            let n = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            let rank = if !p.is_dir() {
                if n == "getting-started.md" {
                    0
                } else {
                    1
                }
            } else {
                match n.as_str() {
                    "tutorials" => 2,
                    "cookbook" => 3,
                    "concepts" => 4,
                    _ => 5,
                }
            };
            (rank, n)
        });
        for p in entries {
            let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if p.is_dir() {
                if name == "images" {
                    std::fs::create_dir_all(src.join("images"))?;
                    for f in std::fs::read_dir(&p)?.flatten() {
                        std::fs::copy(f.path(), src.join("images").join(f.file_name()))?;
                    }
                    continue;
                }
                std::fs::create_dir_all(src.join(&name))?;
                let mut files: Vec<std::path::PathBuf> = std::fs::read_dir(&p)?
                    .flatten()
                    .map(|e| e.path())
                    .filter(|f| f.extension().map(|e| e == "md").unwrap_or(false))
                    .collect();
                files.sort();
                if files.is_empty() {
                    continue;
                }
                let title = name.replace('-', " ");
                let _ = writeln!(summary, "\n# {}\n", title[..1].to_uppercase() + &title[1..]);
                for f in files {
                    let fname = f.file_name().unwrap().to_string_lossy().to_string();
                    std::fs::copy(&f, src.join(&name).join(&fname))?;
                    let t = first_heading(&f).unwrap_or_else(|| fname.trim_end_matches(".md").into());
                    let _ = writeln!(summary, "- [{t}]({name}/{fname})");
                }
            } else if name.ends_with(".md") {
                std::fs::copy(&p, src.join(&name))?;
                if name == "README.md" {
                    intro = true;
                    continue;
                }
                let t = first_heading(&p).unwrap_or_else(|| name.trim_end_matches(".md").into());
                let _ = writeln!(summary, "- [{t}]({name})");
            }
        }
    }
    if !intro {
        std::fs::write(
            src.join("README.md"),
            "# pixelox\n\nA command-line raster and vector graphics workstation for scripts and AI agents.\n",
        )?;
    }
    // agent guide, topics, errors
    let mut agents = String::from("# Agent guide\n\nThe same text `pixelox describe --agent` prints.\n\n```text\n");
    agents.push_str(&crate::builtin::agent_briefing(reg));
    agents.push_str("```\n");
    std::fs::write(src.join("agents.md"), agents)?;
    let _ = writeln!(summary, "\n# Reference\n\n- [Agent guide](agents.md)");
    for (name, text) in TOPICS {
        std::fs::write(src.join("topics").join(format!("{name}.md")), site_escape(text))?;
        let t =
            text.lines().find(|l| l.starts_with("# ")).map(|l| l[2..].to_string()).unwrap_or_else(|| name.to_string());
        let _ = writeln!(summary, "- [{t}](topics/{name}.md)");
    }
    std::fs::write(src.join("errors.md"), site_escape(&error_catalog()))?;
    let _ = writeln!(summary, "- [Error catalog](errors.md)\n\n# Command reference\n");
    let mut by_cat: BTreeMap<_, Vec<&OpSpec>> = BTreeMap::new();
    for op in reg.ops() {
        by_cat.entry(op.category).or_default().push(op);
    }
    let mut n = 0;
    for (cat, ops) in by_cat {
        let _ = writeln!(summary, "- [{}]()", cat.title());
        for op in ops {
            std::fs::write(src.join("commands").join(format!("{}.md", op.id())), site_escape(&command_page(op)))?;
            let _ = writeln!(summary, "  - [{}](commands/{}.md)", op.name, op.id());
            n += 1;
        }
    }
    std::fs::write(src.join("SUMMARY.md"), summary)?;
    std::fs::write(src.join("llms.txt"), llms_txt(reg))?;
    std::fs::write(src.join("llms-full.txt"), llms_full(reg))?;
    std::fs::write(
        out.join("book.toml"),
        "[book]\ntitle = \"pixelox\"\nauthors = [\"Sunday Ikpe\"]\nlanguage = \"en\"\nsrc = \"src\"\n\n[output.html]\ngit-repository-url = \"https://github.com/djynnius/pixelox\"\nsite-url = \"/pixelox/\"\n",
    )?;
    Ok(n)
}

/// Every command needs this many examples (the documentation depth bar).
pub const MIN_EXAMPLES: usize = 3;

pub fn coverage_problems(reg: &Registry) -> Vec<String> {
    let mut v = Vec::new();
    for op in reg.ops() {
        if op.examples.len() < MIN_EXAMPLES {
            v.push(format!("{}: {} example(s); the bar is {MIN_EXAMPLES}", op.name, op.examples.len()));
        }
        if op.summary.len() > 60 {
            v.push(format!("{}: summary is {} characters; keep it under 60", op.name, op.summary.len()));
        }
        for p in &op.params {
            if p.help.is_empty() {
                v.push(format!("{} --{}: no help text", op.name, p.name));
            }
        }
    }
    v
}
