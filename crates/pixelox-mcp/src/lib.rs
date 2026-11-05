//! # pixelox-mcp
//!
//! `pixelox mcp serve`: a Model Context Protocol server (revision 2025-06-18, also
//! negotiating 2025-03-26 and 2024-11-05) over newline-delimited JSON-RPC on stdio.
//!
//! The tool surface is deliberately coarse (PRD §10.1): `pixelox_describe`,
//! `pixelox_schema`, `pixelox_run` (any command, validated against the CLI's own schema),
//! `pixelox_info`, `pixelox_preview` (rendered PNG as image content), `pixelox_detect`,
//! `pixelox_jobs`, and `pixelox_docs`. Sidecars under the allowed directories and the
//! reference pages are exposed as resources.
//!
//! Security (PRD §10.1, §11.8): every path must resolve, symlinks resolved, inside an
//! `--allow` root (PX1310 otherwise, checked before anything runs and again by the
//! executor); commands run in-process with no shell; `--read-only` refuses commands that
//! can write; model downloads need the `--allow-model-downloads` server flag *and*
//! `"consent": ["model-download"]` on the call; every `tools/call` is appended to the
//! NDJSON audit log under the state directory.

// `pixelox_core::Error` is 176 bytes (code, message, hint, docs, context map); clippy's
// `result_large_err` fires on every fn returning `pixelox_core::Result`. Boxing the error
// is a core decision, so the lint is silenced here rather than worked around per fn.
#![allow(clippy::result_large_err)]

pub mod server;

mod resources;
mod tools;
mod util;

pub use server::{ServeStats, Server, ServerConfig, PROTOCOL_VERSIONS};
pub use tools::TOOL_NAMES;

use pixelox_core::ctx::resolve_for_sandbox;
use pixelox_core::paths;
use pixelox_core::prelude::*;
use std::path::PathBuf;

/// Every command this crate defines.
pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::custom("mcp serve", Category::Agents, "Serve pixelox to AI agents over MCP (stdio)", serve)
        .about(
            "Runs a Model Context Protocol server on stdin/stdout until the client disconnects; MCP hosts launch it as a \
             subprocess (Claude Code: `claude mcp add pixelox -- pixelox mcp serve --allow ./work`). It exposes eight coarse \
             tools: pixelox_describe, pixelox_schema, pixelox_run (any command, validated against its JSON Schema), \
             pixelox_info, pixelox_preview (a downscaled PNG of a document, layer, or region), pixelox_detect, pixelox_jobs, \
             and pixelox_docs, plus sidecars and reference pages as resources. Every path in every call must resolve, \
             symlinks resolved, inside an --allow directory (default: the current directory); commands run in-process with \
             no shell; --read-only refuses commands that write; model downloads need --allow-model-downloads and the call's \
             consent; every tool call is appended to the audit log (mcp-audit.ndjson in the state directory).",
        )
        .inputs(Inputs::NONE)
        .param(param("allow", ParamKind::Path).repeatable().help("Directory the server may read and write (repeatable); default: the current directory"))
        .param(param("read-only", ParamKind::Flag).help("Refuse commands that write files; inspection, schemas, docs, and previews still work"))
        .param(param("allow-model-downloads", ParamKind::Flag).help("Let calls that pass consent [\"model-download\"] download missing models"))
        .example("pixelox mcp serve --allow ./work", "Serve the ./work directory to an agent over stdio")
        .example("pixelox mcp serve --allow ./work --allow ./assets --read-only", "Let an agent inspect two folders without changing anything")
        .example("pixelox mcp serve --allow photos --allow-model-downloads", "Allow consented model downloads for detect and retouch commands")]
}

/// Install renderer hooks (if any) into the compositor.
pub fn install() {}

fn serve(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let registry = ctx.registry()?;
    let mut roots: Vec<PathBuf> = a.all("allow").iter().map(PathBuf::from).collect();
    if roots.is_empty() {
        roots = ctx.globals.allow.clone();
    }
    if roots.is_empty() {
        roots.push(std::env::current_dir()?);
    }
    for r in &roots {
        if !resolve_for_sandbox(r).is_dir() {
            return Err(Error::new("PX1101", format!("allowed directory {} does not exist", r.display()))
                .with_hint(format!(
                    "create it first, or pass another --allow: pixelox mcp serve --allow {}",
                    r.display()
                ))
                .with_context("path", r.display().to_string()));
        }
    }
    let mut cfg = ServerConfig::new(&roots);
    cfg.read_only = a.flag("read-only") || ctx.globals.read_only;
    cfg.allow_model_downloads = a.flag("allow-model-downloads");
    cfg.base_globals = ctx.globals.clone();
    cfg.audit_log = Some(paths::audit_log());
    let summary = json!({
        "allow": cfg.allow.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
        "read_only": cfg.read_only,
        "allow_model_downloads": cfg.allow_model_downloads,
        "audit_log": paths::audit_log().display().to_string(),
        "protocol_versions": PROTOCOL_VERSIONS,
        "tools": TOOL_NAMES,
    });
    if ctx.dry_run() {
        return Ok(summary);
    }
    let server = Server::new(registry, ctx.io, cfg);
    let stats = server.serve_stdio()?;
    let mut out = summary;
    out["messages"] = json!(stats.messages);
    out["tool_calls"] = json!(stats.tool_calls);
    Ok(out)
}

#[cfg(test)]
mod tests;
