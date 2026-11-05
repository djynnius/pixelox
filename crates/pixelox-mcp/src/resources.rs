//! MCP resources: provenance sidecars under the allowed roots
//! (`pixelox://sidecar/<relpath>`) and reference pages (`pixelox://docs/<command-id>`).

use crate::server::{RpcError, Server};
use crate::tools;
use crate::util::{pct_decode, pct_encode};
use pixelox_core::ctx::resolve_for_sandbox;
use pixelox_core::Error;
use serde_json::{json, Value};
use std::path::{Component, Path, PathBuf};

pub const SIDECAR_PREFIX: &str = "pixelox://sidecar/";
pub const DOCS_PREFIX: &str = "pixelox://docs/";
const SIDECAR_SUFFIX: &str = ".pixelox.json";
/// Directory depth and count limits for the sidecar scan (keeps `resources/list` fast).
const MAX_DEPTH: usize = 8;
const MAX_SIDECARS: usize = 500;
const MAX_SIDECAR_BYTES: u64 = 16 << 20;
/// JSON-RPC error code MCP uses for an unknown resource.
const NOT_FOUND: i64 = -32002;

/// Sidecars under `root` (sorted; hidden directories and symlinks are skipped, so the scan
/// never leaves the root).
fn scan(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    if depth > MAX_DEPTH || out.len() >= MAX_SIDECARS {
        return;
    }
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.file_name());
    for e in entries {
        if out.len() >= MAX_SIDECARS {
            return;
        }
        let name = e.file_name().to_string_lossy().to_string();
        let Ok(ft) = e.file_type() else { continue };
        if name.starts_with('.') || ft.is_symlink() {
            continue;
        }
        if ft.is_dir() {
            scan(&e.path(), depth + 1, out);
        } else if ft.is_file() && name.ends_with(SIDECAR_SUFFIX) {
            out.push(e.path());
        }
    }
}

fn rel_uri(root: &Path, path: &Path) -> Option<String> {
    let rel = path.strip_prefix(root).ok()?;
    let parts: Vec<String> = rel.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
    Some(parts.join("/"))
}

/// `resources/list`.
pub(crate) fn list(s: &Server) -> Value {
    let mut resources = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for root in &s.cfg.allow {
        let mut found = Vec::new();
        scan(root, 0, &mut found);
        for p in found {
            let Some(rel) = rel_uri(root, &p) else { continue };
            let uri = format!("{SIDECAR_PREFIX}{}", pct_encode(&rel));
            if !seen.insert(uri.clone()) {
                continue;
            }
            let output = p.display().to_string();
            let output = output.strip_suffix(SIDECAR_SUFFIX).unwrap_or(&output).to_string();
            resources.push(json!({
                "uri": uri,
                "name": rel,
                "title": format!("Sidecar of {}", Path::new(&output).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()),
                "description": format!("Provenance sidecar (recipe, inputs, hashes) of {output}"),
                "mimeType": "application/json",
            }));
        }
    }
    for op in s.registry.ops() {
        resources.push(json!({
            "uri": format!("{DOCS_PREFIX}{}", op.id()),
            "name": format!("docs/{}", op.id()),
            "title": format!("pixelox {}", op.name),
            "description": op.summary,
            "mimeType": "text/markdown",
        }));
    }
    json!({ "resources": resources })
}

/// `resources/templates/list`.
pub(crate) fn templates() -> Value {
    json!({"resourceTemplates": [
        {"uriTemplate": format!("{DOCS_PREFIX}{{command}}"), "name": "docs", "title": "Reference page of a command or error code", "mimeType": "text/markdown"},
        {"uriTemplate": format!("{SIDECAR_PREFIX}{{path}}"), "name": "sidecar", "title": "Provenance sidecar under an allowed directory", "mimeType": "application/json"},
    ]})
}

fn sandbox_error(uri: &str, why: &str) -> RpcError {
    RpcError::from_error(-32602, Error::new("PX1310", format!("{uri}: {why}")).with_context("uri", uri))
}

fn read_sidecar(s: &Server, uri: &str, rel: &str) -> Result<String, RpcError> {
    let rel = pct_decode(rel).ok_or_else(|| RpcError::invalid_params(format!("{uri}: malformed percent-encoding")))?;
    let relp = Path::new(&rel);
    if rel.is_empty() || !relp.components().all(|c| matches!(c, Component::Normal(_))) {
        return Err(sandbox_error(uri, "the path must be relative to an allowed directory, without '..'"));
    }
    if !rel.ends_with(SIDECAR_SUFFIX) {
        return Err(RpcError::invalid_params(format!("{uri}: not a sidecar (expected *{SIDECAR_SUFFIX})")));
    }
    for root in &s.cfg.allow {
        let real = resolve_for_sandbox(&root.join(relp));
        if !s.cfg.allow.iter().any(|r| real.starts_with(r)) {
            return Err(sandbox_error(uri, "resolves outside the allowed directories"));
        }
        let Ok(meta) = std::fs::metadata(&real) else { continue };
        if !meta.is_file() {
            continue;
        }
        if meta.len() > MAX_SIDECAR_BYTES {
            return Err(RpcError::invalid_params(format!("{uri}: file is too large to be a sidecar")));
        }
        return std::fs::read_to_string(&real).map_err(|e| RpcError::from_error(NOT_FOUND, Error::from(e)));
    }
    Err(RpcError::new(NOT_FOUND, format!("resource not found: {uri}")))
}

/// `resources/read`.
pub(crate) fn read(s: &Server, params: &Value) -> Result<Value, RpcError> {
    let uri = params
        .get("uri")
        .and_then(Value::as_str)
        .ok_or_else(|| RpcError::invalid_params("resources/read needs a uri"))?;
    let (mime, text) = if let Some(rel) = uri.strip_prefix(SIDECAR_PREFIX) {
        ("application/json", read_sidecar(s, uri, rel)?)
    } else if let Some(topic) = uri.strip_prefix(DOCS_PREFIX) {
        let topic =
            pct_decode(topic).ok_or_else(|| RpcError::invalid_params(format!("{uri}: malformed percent-encoding")))?;
        let text = tools::docs_text(s, &topic).map_err(|e| {
            if e.code == "PX1001" {
                RpcError::from_error(NOT_FOUND, e)
            } else {
                RpcError::from_error(-32603, e)
            }
        })?;
        ("text/markdown", text)
    } else {
        return Err(RpcError::new(NOT_FOUND, format!("resource not found: {uri}")));
    };
    Ok(json!({"contents": [{"uri": uri, "mimeType": mime, "text": text}]}))
}
