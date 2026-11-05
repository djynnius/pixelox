//! The MCP server: JSON-RPC 2.0 over newline-delimited stdio, protocol negotiation, the
//! path sandbox, and the audit log. Tools live in `tools.rs`, resources in `resources.rs`.

use crate::{resources, tools};
use pixelox_core::ctx::{absolutize, resolve_for_sandbox, CancelToken, Globals, ImageIo, Progress};
use pixelox_core::envelope::{now_utc, sha256_bytes, PIXELOX_VERSION};
use pixelox_core::{paths, Error, Registry, Result};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};

/// Protocol revisions this server speaks, newest first. A client asking for anything else
/// gets the newest.
pub const PROTOCOL_VERSIONS: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];

/// Where responses and notifications are written (one JSON object per line).
pub(crate) type Sink<'w> = Mutex<dyn Write + Send + 'w>;

/// How the server was started.
#[derive(Clone, Debug)]
pub struct ServerConfig {
    /// Allowed roots: absolute, symlinks resolved. Every path in every call must resolve
    /// inside one of them (the models directory is also readable).
    pub allow: Vec<PathBuf>,
    /// Refuse commands that can write files.
    pub read_only: bool,
    /// First key for model downloads; the second is `"consent": ["model-download"]` on a call.
    pub allow_model_downloads: bool,
    /// Globals each call starts from (threads, memory budget, sidecar policy, seed).
    pub base_globals: Globals,
    /// NDJSON audit log, one line per `tools/call` (`None` disables it).
    pub audit_log: Option<PathBuf>,
}

impl ServerConfig {
    /// A configuration confined to `allow` (resolved here), with the default audit log.
    pub fn new(allow: &[PathBuf]) -> ServerConfig {
        let mut roots: Vec<PathBuf> = Vec::new();
        for r in allow {
            let p = resolve_for_sandbox(r);
            if !roots.contains(&p) {
                roots.push(p);
            }
        }
        ServerConfig {
            allow: roots,
            read_only: false,
            allow_model_downloads: false,
            base_globals: Globals::default(),
            audit_log: Some(paths::audit_log()),
        }
    }
}

/// Counters reported when the server stops.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ServeStats {
    pub messages: u64,
    pub tool_calls: u64,
}

/// What a path is used for: reads may also come from the models directory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Access {
    Read,
    Write,
}

/// A JSON-RPC error.
#[derive(Debug)]
pub(crate) struct RpcError {
    pub code: i64,
    pub message: String,
    pub data: Option<Value>,
}

impl RpcError {
    pub fn new(code: i64, message: impl Into<String>) -> RpcError {
        RpcError { code, message: message.into(), data: None }
    }
    pub fn invalid_params(message: impl Into<String>) -> RpcError {
        RpcError::new(-32602, message)
    }
    /// An invalid-params error carrying a pixelox error (code, hint) as `data`.
    pub fn from_error(code: i64, e: Error) -> RpcError {
        RpcError { code, message: format!("{}: {}", e.code, e.message), data: serde_json::to_value(&e).ok() }
    }
}

fn rpc_ok(id: &Value, result: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": result})
}

fn rpc_error(id: &Value, e: RpcError) -> Value {
    let mut err = json!({"code": e.code, "message": e.message});
    if let Some(d) = e.data {
        err["data"] = d;
    }
    json!({"jsonrpc": "2.0", "id": id, "error": err})
}

fn lock<T: ?Sized>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}

fn write_line(sink: &Sink<'_>, v: &Value) {
    let mut w = lock(sink);
    let _ = writeln!(w, "{v}");
    let _ = w.flush();
}

/// Sends `notifications/progress` for a call that carried a `progressToken`.
pub(crate) struct McpProgress<'s, 'w> {
    sink: Option<&'s Sink<'w>>,
    token: Option<Value>,
    last: AtomicI64,
}

impl<'s, 'w> McpProgress<'s, 'w> {
    pub fn new(sink: Option<&'s Sink<'w>>, token: Option<Value>) -> Self {
        McpProgress { sink, token, last: AtomicI64::new(-1) }
    }
}

impl Progress for McpProgress<'_, '_> {
    fn progress(&self, stage: &str, fraction: f64) {
        let (Some(sink), Some(token)) = (self.sink, &self.token) else { return };
        // whole percents only, strictly increasing (the spec requires monotonic progress)
        let pct = (fraction.clamp(0.0, 1.0) * 100.0).round() as i64;
        if self.last.fetch_max(pct, Ordering::SeqCst) >= pct {
            return;
        }
        let msg = json!({"jsonrpc": "2.0", "method": "notifications/progress",
            "params": {"progressToken": token, "progress": pct, "total": 100, "message": stage}});
        write_line(sink, &msg);
    }
}

/// An MCP server over one registry and IO layer.
pub struct Server<'a> {
    pub(crate) registry: &'a Registry,
    pub(crate) io: &'a dyn ImageIo,
    pub(crate) cfg: ServerConfig,
    cancels: Mutex<BTreeMap<String, CancelToken>>,
    audit_lock: Mutex<()>,
    tmp_counter: AtomicU64,
    messages: AtomicU64,
    tool_calls: AtomicU64,
}

impl<'a> Server<'a> {
    pub fn new(registry: &'a Registry, io: &'a dyn ImageIo, cfg: ServerConfig) -> Server<'a> {
        Server {
            registry,
            io,
            cfg,
            cancels: Mutex::new(BTreeMap::new()),
            audit_lock: Mutex::new(()),
            tmp_counter: AtomicU64::new(0),
            messages: AtomicU64::new(0),
            tool_calls: AtomicU64::new(0),
        }
    }

    pub fn config(&self) -> &ServerConfig {
        &self.cfg
    }

    pub(crate) fn next_tmp(&self) -> u64 {
        self.tmp_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Globals for one call: the base, confined to the sandbox, every gate closed.
    pub(crate) fn call_globals(&self) -> Globals {
        let mut g = self.cfg.base_globals.clone();
        g.allow = self.cfg.allow.clone();
        g.read_only = self.cfg.read_only;
        g.output = None;
        g.in_place = false;
        g.dry_run = false;
        g.overwrite = false;
        g.yes = false;
        g.format = None;
        g
    }

    // -----------------------------------------------------------------------------------
    // sandbox

    /// Check a path from a call against the allowed roots (symlinks resolved) and return
    /// the path to hand to the executor. `-` (stdin/stdout) is refused: stdio is the MCP
    /// channel. `~` is expanded here so the check and the use agree.
    pub(crate) fn sandbox_path(&self, raw: &str, access: Access) -> Result<String> {
        if raw.trim().is_empty() {
            return Err(Error::new("PX1010", "empty path"));
        }
        if raw == "-" {
            return Err(Error::new("PX1010", "'-' (stdin/stdout) is the MCP channel; pass a file path")
                .with_hint("write to a file inside an allowed directory, e.g. output: \"out.png\""));
        }
        let resolved = resolve_for_sandbox(Path::new(raw));
        let inside = self.cfg.allow.iter().any(|r| resolved.starts_with(r));
        let models = access == Access::Read && resolved.starts_with(resolve_for_sandbox(&paths::models_dir()));
        if !inside && !models {
            let allowed: Vec<String> = self.cfg.allow.iter().map(|a| a.display().to_string()).collect();
            return Err(Error::new("PX1310", format!("{raw} is outside the allowed directories"))
                .with_hint(format!(
                    "use a path inside {}, or ask the user to restart the server with --allow <dir>",
                    allowed.join(", ")
                ))
                .with_context("path", raw)
                .with_context("resolved", resolved.display().to_string())
                .with_context("allowed", allowed));
        }
        Ok(if raw.starts_with('~') { absolutize(Path::new(raw)).display().to_string() } else { raw.to_string() })
    }

    // -----------------------------------------------------------------------------------
    // audit log

    pub(crate) fn audit(&self, tool: &str, args: &Value, ok: bool, code: Option<&str>) {
        let Some(path) = &self.cfg.audit_log else { return };
        let line = json!({"ts": now_utc(), "tool": tool, "params_sha256": sha256_bytes(args.to_string().as_bytes()), "ok": ok, "code": code});
        let _guard = lock(&self.audit_lock);
        let res = (|| -> std::io::Result<()> {
            if let Some(dir) = path.parent() {
                std::fs::create_dir_all(dir)?;
            }
            let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
            writeln!(f, "{line}")
        })();
        if let Err(e) = res {
            eprintln!("pixelox mcp: cannot append to the audit log {}: {e}", path.display());
        }
    }

    // -----------------------------------------------------------------------------------
    // protocol

    fn instructions(&self) -> String {
        let roots: Vec<String> = self.cfg.allow.iter().map(|a| a.display().to_string()).collect();
        let cwd = std::env::current_dir().map(|p| p.display().to_string()).unwrap_or_else(|_| ".".into());
        let mut s = format!(
            "pixelox edits images and layered .pxo documents. Call pixelox_describe once for the command index, \
             pixelox_schema for a command's exact params, then pixelox_run. Tools return the pixelox JSON envelope \
             (ok, result, outputs with SHA-256, warnings, error with a runnable hint). Paths must resolve inside: {}; \
             relative paths resolve against {cwd}. Plan with dry_run: true before changing files; existing files are \
             only replaced with overwrite: true, and .pxo documents change only with in_place: true. Look at your \
             work with pixelox_preview.",
            roots.join(", ")
        );
        if self.cfg.read_only {
            s.push_str(" This server is read-only: commands that write files are refused.");
        }
        if self.cfg.allow_model_downloads {
            s.push_str(" Model downloads need consent: [\"model-download\"] on the call, after the user agrees.");
        } else {
            s.push_str(" Model downloads are disabled on this server.");
        }
        s
    }

    fn initialize(&self, params: &Value) -> Value {
        let requested = params.get("protocolVersion").and_then(Value::as_str);
        let version = requested.filter(|v| PROTOCOL_VERSIONS.contains(v)).unwrap_or(PROTOCOL_VERSIONS[0]);
        json!({
            "protocolVersion": version,
            "capabilities": {
                "tools": {"listChanged": false},
                "resources": {"subscribe": false, "listChanged": false},
            },
            "serverInfo": {"name": "pixelox", "title": "pixelox image editor", "version": PIXELOX_VERSION},
            "instructions": self.instructions(),
        })
    }

    fn cancel_token(&self, id: &Value) -> CancelToken {
        lock(&self.cancels).entry(id.to_string()).or_default().clone()
    }

    fn notification(&self, method: &str, params: &Value) {
        if method == "notifications/cancelled" {
            if let Some(rid) = params.get("requestId") {
                if let Some(c) = lock(&self.cancels).get(&rid.to_string()) {
                    c.cancel();
                }
            }
        }
    }

    fn tools_call(&self, id: &Value, params: &Value, sink: Option<&Sink<'_>>) -> std::result::Result<Value, RpcError> {
        self.tool_calls.fetch_add(1, Ordering::SeqCst);
        let name = params.get("name").and_then(Value::as_str).unwrap_or("");
        let args = params.get("arguments").cloned().unwrap_or_else(|| json!({}));
        if !tools::TOOL_NAMES.contains(&name) {
            self.audit(name, &args, false, Some("PX1001"));
            let mut e = RpcError::invalid_params(format!("unknown tool '{name}'"));
            e.data = Some(json!({"tools": tools::TOOL_NAMES}));
            return Err(e);
        }
        let token = params.get("_meta").and_then(|m| m.get("progressToken")).cloned();
        let progress = McpProgress::new(sink, token);
        let cancel = self.cancel_token(id);
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            tools::call(self, name, &args, &progress, cancel)
        }))
        .unwrap_or_else(|p| {
            let msg = p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "panic".into());
            tools::ToolOutcome::failure(name, Error::new("PX9001", format!("internal error (panic): {msg}")))
        });
        lock(&self.cancels).remove(&id.to_string());
        self.audit(name, &args, outcome.ok, outcome.code.as_deref());
        Ok(outcome.result)
    }

    /// Handle one JSON-RPC message; `None` for notifications and client responses.
    pub fn handle(&self, msg: &Value) -> Option<Value> {
        self.handle_with(msg, None)
    }

    pub(crate) fn handle_with(&self, msg: &Value, sink: Option<&Sink<'_>>) -> Option<Value> {
        self.messages.fetch_add(1, Ordering::SeqCst);
        let Some(obj) = msg.as_object() else {
            return Some(rpc_error(&Value::Null, RpcError::new(-32600, "invalid request: expected a JSON object")));
        };
        let id = obj.get("id").cloned();
        let Some(method) = obj.get("method").and_then(Value::as_str) else {
            if obj.contains_key("result") || obj.contains_key("error") {
                return None; // a response to a request we never sent
            }
            return Some(rpc_error(
                &id.unwrap_or(Value::Null),
                RpcError::new(-32600, "invalid request: missing method"),
            ));
        };
        let params = obj.get("params").cloned().unwrap_or(Value::Null);
        let Some(id) = id else {
            self.notification(method, &params);
            return None;
        };
        if !(id.is_string() || id.is_number()) {
            return Some(rpc_error(
                &Value::Null,
                RpcError::new(-32600, "invalid request: id must be a string or a number"),
            ));
        }
        if obj.get("jsonrpc").and_then(Value::as_str) != Some("2.0") {
            return Some(rpc_error(&id, RpcError::new(-32600, "invalid request: jsonrpc must be \"2.0\"")));
        }
        let result = match method {
            "initialize" => Ok(self.initialize(&params)),
            "ping" => Ok(json!({})),
            "tools/list" => Ok(json!({"tools": tools::definitions(self)})),
            "tools/call" => self.tools_call(&id, &params, sink),
            "resources/list" => Ok(resources::list(self)),
            "resources/templates/list" => Ok(resources::templates()),
            "resources/read" => resources::read(self, &params),
            other => Err(RpcError::new(-32601, format!("method not found: {other}"))),
        };
        Some(match result {
            Ok(v) => rpc_ok(&id, v),
            Err(e) => rpc_error(&id, e),
        })
    }

    /// Serve newline-delimited JSON-RPC until `input` ends. Tool calls run on their own
    /// threads so pings and cancellations are answered while a long command runs; when
    /// the client hangs up, running calls are cancelled and joined.
    pub fn serve_lines<R: BufRead, W: Write + Send>(&self, input: R, out: W) -> Result<ServeStats> {
        let sink_owned = Mutex::new(out);
        let sink: &Sink<'_> = &sink_owned;
        std::thread::scope(|scope| {
            for line in input.lines() {
                let Ok(line) = line else { break };
                let text = line.trim();
                if text.is_empty() {
                    continue;
                }
                let msg: Value = match serde_json::from_str(text) {
                    Ok(v) => v,
                    Err(e) => {
                        write_line(sink, &rpc_error(&Value::Null, RpcError::new(-32700, format!("parse error: {e}"))));
                        continue;
                    }
                };
                match msg {
                    Value::Array(batch) => {
                        if batch.is_empty() {
                            write_line(
                                sink,
                                &rpc_error(&Value::Null, RpcError::new(-32600, "invalid request: empty batch")),
                            );
                            continue;
                        }
                        let replies: Vec<Value> =
                            batch.iter().filter_map(|m| self.handle_with(m, Some(sink))).collect();
                        if !replies.is_empty() {
                            write_line(sink, &Value::Array(replies));
                        }
                    }
                    m if m.get("method").and_then(Value::as_str) == Some("tools/call")
                        && m.get("id").is_some_and(|i| !i.is_null()) =>
                    {
                        // register the cancel token now so a hang-up cannot race the spawn
                        if let Some(id) = m.get("id") {
                            self.cancel_token(id);
                        }
                        scope.spawn(move || {
                            if let Some(reply) = self.handle_with(&m, Some(sink)) {
                                write_line(sink, &reply);
                            }
                        });
                    }
                    m => {
                        if let Some(reply) = self.handle_with(&m, Some(sink)) {
                            write_line(sink, &reply);
                        }
                    }
                }
            }
            for c in lock(&self.cancels).values() {
                c.cancel();
            }
        });
        Ok(ServeStats {
            messages: self.messages.load(Ordering::SeqCst),
            tool_calls: self.tool_calls.load(Ordering::SeqCst),
        })
    }

    /// Serve on this process's stdin/stdout until stdin closes.
    pub fn serve_stdio(&self) -> Result<ServeStats> {
        self.serve_lines(std::io::stdin().lock(), std::io::stdout())
    }
}
