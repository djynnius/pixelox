//! In-memory tests of the JSON-RPC layer, the tools, the sandbox, and the resources. They
//! use a small registry of dummy ops and `NoIo` (plus a fake IO layer for previews).

use super::*;
use crate::util::base64;
use pixelox_core::ctx::{ImageIo, NoIo, OpenOptions};
use pixelox_core::envelope::Warning;
use pixelox_core::testing::{self, TempDir};
use pixelox_core::{ImageInfo, Registry, Runner};
use std::io::Cursor;
use std::path::Path;

fn touched(_: &Args, _: &mut Document, _: &mut OpCtx) -> Result<Value> {
    Ok(json!({"touched": true}))
}

fn echo(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    ctx.set_progress(0.5);
    Ok(json!({
        "text": a.str_or("text", ""),
        "id": a.str("id"),
        "yes": ctx.globals.yes,
        "read_only": ctx.globals.read_only,
        "allow": ctx.globals.allow.len(),
        "overwrite": ctx.globals.overwrite,
        "seed": ctx.globals.seed,
    }))
}

fn briefing(_: &Args, _: &mut OpCtx) -> Result<Value> {
    Ok(json!({"text": "pixelox agent briefing: filter.blur, dummy.echo"}))
}

fn registry(with_describe: bool) -> Registry {
    let mut r = Registry::new();
    r.extend(vec![
        OpSpec::doc("filter blur", Category::Filter, "Blur a layer", touched)
            .param(param("sigma", ParamKind::Float).default("2").range("0..100").help("Blur radius in px"))
            .params(common::region())
            .param(param("lut", ParamKind::Path).help("A lookup file")),
        OpSpec::doc("analyze stats", Category::Analyze, "Channel statistics", touched).read_only(),
        OpSpec::doc("detect subject", Category::Detect, "Find the main subject", touched).milestone("v0.2"),
        OpSpec::custom("dummy echo", Category::System, "Echo text", echo)
            .inputs(Inputs::NONE)
            .param(param("text", ParamKind::Str).help("Text to echo")),
        OpSpec::custom("models pull", Category::Models, "Download a model", echo)
            .inputs(Inputs::NONE)
            .param(param("text", ParamKind::Str).help("Model id")),
        OpSpec::custom("jobs status", Category::Automation, "Job status", echo)
            .inputs(Inputs::NONE)
            .positional("id")
            .param(param("id", ParamKind::Str).help("Job id")),
    ]);
    r.extend(crate::ops());
    if with_describe {
        r.extend(vec![OpSpec::custom("describe", Category::Agents, "Agent briefing", briefing)
            .inputs(Inputs::NONE)
            .param(param("agent", ParamKind::Flag).help("Compact agent briefing"))]);
    }
    r
}

fn config(dir: &TempDir) -> ServerConfig {
    let mut cfg = ServerConfig::new(std::slice::from_ref(&dir.0));
    cfg.audit_log = Some(dir.path("state/mcp-audit.ndjson"));
    cfg
}

fn req(id: i64, method: &str, params: Value) -> String {
    json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}).to_string()
}

fn call(id: i64, tool: &str, args: Value) -> String {
    req(id, "tools/call", json!({"name": tool, "arguments": args}))
}

/// Feed lines through the stdio loop; return every message the server wrote.
fn exchange(server: &Server, lines: &[String]) -> Vec<Value> {
    let mut out: Vec<u8> = Vec::new();
    server.serve_lines(Cursor::new(lines.join("\n")), &mut out).unwrap();
    String::from_utf8(out).unwrap().lines().map(|l| serde_json::from_str(l).unwrap()).collect()
}

fn reply(msgs: &[Value], id: i64) -> Value {
    msgs.iter().find(|m| m["id"] == json!(id)).cloned().unwrap_or_else(|| panic!("no reply to {id} in {msgs:#?}"))
}

/// Run one tool call; return its `result`.
fn tool(server: &Server, name: &str, args: Value) -> Value {
    let msgs = exchange(server, &[call(1, name, args)]);
    reply(&msgs, 1)["result"].clone()
}

fn error_code(result: &Value) -> String {
    assert_eq!(result["isError"], true, "expected an error: {result:#}");
    result["structuredContent"]["error"]["code"].as_str().unwrap().to_string()
}

#[test]
fn initialize_negotiates_versions() {
    let d = TempDir::new("mcp-init");
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    let msgs = exchange(
        &s,
        &[
            req(
                1,
                "initialize",
                json!({"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "t", "version": "1"}}),
            ),
            json!({"jsonrpc": "2.0", "method": "notifications/initialized"}).to_string(),
            req(2, "initialize", json!({"protocolVersion": "2025-03-26"})),
            req(3, "initialize", json!({"protocolVersion": "2024-11-05"})),
            req(4, "initialize", json!({"protocolVersion": "1999-01-01"})),
            req(5, "ping", Value::Null),
        ],
    );
    assert_eq!(msgs.len(), 5, "the notification gets no reply: {msgs:#?}");
    let init = reply(&msgs, 1)["result"].clone();
    assert_eq!(init["protocolVersion"], "2025-06-18");
    assert_eq!(init["serverInfo"]["name"], "pixelox");
    assert!(init["capabilities"]["tools"].is_object() && init["capabilities"]["resources"].is_object());
    assert!(init["instructions"].as_str().unwrap().contains(&d.0.canonicalize().unwrap().display().to_string()));
    assert_eq!(reply(&msgs, 2)["result"]["protocolVersion"], "2025-03-26");
    assert_eq!(reply(&msgs, 3)["result"]["protocolVersion"], "2024-11-05");
    assert_eq!(reply(&msgs, 4)["result"]["protocolVersion"], PROTOCOL_VERSIONS[0]);
    assert_eq!(reply(&msgs, 5)["result"], json!({}));
}

#[test]
fn tools_list_has_the_eight_tools() {
    let d = TempDir::new("mcp-list");
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    let msgs = exchange(&s, &[req(1, "tools/list", json!({}))]);
    let tools = reply(&msgs, 1)["result"]["tools"].as_array().unwrap().clone();
    let names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names, TOOL_NAMES);
    assert_eq!(names.len(), 8);
    for t in &tools {
        assert_eq!(t["inputSchema"]["type"], "object", "{}", t["name"]);
        assert!(t["inputSchema"]["properties"].is_object(), "{}", t["name"]);
        assert!(!t["description"].as_str().unwrap().is_empty());
        assert!(t["annotations"]["readOnlyHint"].is_boolean());
    }
    let detect = tools.iter().find(|t| t["name"] == "pixelox_detect").unwrap();
    assert_eq!(detect["inputSchema"]["properties"]["kind"]["enum"], json!(["subject"]));
    let run = tools.iter().find(|t| t["name"] == "pixelox_run").unwrap();
    assert_eq!(run["inputSchema"]["required"], json!(["command"]));
    assert_eq!(run["inputSchema"]["properties"]["consent"]["items"]["enum"], json!(["model-download"]));
}

#[test]
fn schema_tool_returns_the_command_schema() {
    let d = TempDir::new("mcp-schema");
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    let r = tool(&s, "pixelox_schema", json!({"command": "filter.blur"}));
    assert_eq!(r["isError"], false, "{r:#}");
    let schema = &r["structuredContent"];
    assert_eq!(schema["command"], "filter.blur");
    assert!(schema["params"]["properties"]["sigma"].is_object());
    assert!(schema["params"]["properties"]["output"].is_object());
    let text: Value = serde_json::from_str(r["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(&text, schema);

    let r = tool(&s, "pixelox_schema", json!({"command": "filter.blurr"}));
    assert_eq!(error_code(&r), "PX1001");
    assert!(r["structuredContent"]["error"]["hint"].as_str().unwrap().contains("filter blur"));
    let r = tool(&s, "pixelox_schema", json!({}));
    assert_eq!(error_code(&r), "PX1003");
}

#[test]
fn run_validates_then_executes() {
    let d = TempDir::new("mcp-run");
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    let r = tool(
        &s,
        "pixelox_run",
        json!({"command": "dummy.echo", "params": {"text": "hi", "seed": 7, "overwrite": true}}),
    );
    assert_eq!(r["isError"], false, "{r:#}");
    let env = &r["structuredContent"];
    assert_eq!(env["ok"], true);
    assert_eq!(env["command"], "dummy.echo");
    assert_eq!(env["result"]["text"], "hi");
    assert_eq!(env["result"]["allow"], 1, "the sandbox reaches the executor");
    assert_eq!(env["result"]["seed"], 7);
    assert_eq!(env["result"]["overwrite"], true);
    assert_eq!(env["result"]["yes"], false);
    let text: Value = serde_json::from_str(r["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(&text, env);

    let r = tool(&s, "pixelox_run", json!({"command": "dummy echo", "params": {"text": "plan"}, "dry_run": true}));
    assert_eq!(r["structuredContent"]["dry_run"], true);

    // validation happens before anything runs
    let r = tool(&s, "pixelox_run", json!({"command": "dummy.echo", "params": {"nope": 1}}));
    assert_eq!(error_code(&r), "PX1002");
    let r = tool(
        &s,
        "pixelox_run",
        json!({"command": "filter.blur", "params": {"input": "x.png", "output": "y.png", "sigma": 1000}}),
    );
    assert_eq!(error_code(&r), "PX1013");
    let r = tool(&s, "pixelox_run", json!({"command": "filter.blur", "input": "x.png"}));
    assert_eq!(error_code(&r), "PX1002");
    assert!(r["structuredContent"]["error"]["hint"].as_str().unwrap().contains("params"));
    let r = tool(&s, "pixelox_run", json!({"command": "nope.cmd"}));
    assert_eq!(error_code(&r), "PX1001");
    let r = tool(&s, "pixelox_run", json!({"command": "mcp.serve"}));
    assert_eq!(error_code(&r), "PX1520");
}

#[test]
fn sandbox_violations_are_refused() {
    let d = TempDir::new("mcp-sandbox");
    let outside = TempDir::new("mcp-outside");
    std::fs::write(d.path("in.png"), b"x").unwrap();
    std::fs::write(outside.path("in.png"), b"x").unwrap();
    std::fs::write(outside.path("m.png"), b"x").unwrap();
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    let inside = |n: &str| d.path(n).display().to_string();
    let out = |n: &str| outside.path(n).display().to_string();
    let run = |params: Value| tool(&s, "pixelox_run", json!({"command": "filter.blur", "params": params}));

    let r = run(json!({"input": out("in.png"), "output": inside("o.png")}));
    assert_eq!(error_code(&r), "PX1310");
    assert!(r["structuredContent"]["error"]["context"]["allowed"].is_array());
    let escape = format!("{}/../escape.png", d.0.display());
    let r = run(json!({"input": inside("in.png"), "output": escape}));
    assert_eq!(error_code(&r), "PX1310");
    assert!(!d.0.parent().unwrap().join("escape.png").exists());
    let r = run(json!({"input": "relative-to-cwd.png", "output": inside("o.png")}));
    assert_eq!(error_code(&r), "PX1310", "relative paths resolve against the cwd, which is outside the root");
    let r = run(json!({"input": inside("in.png"), "output": inside("o.png"), "mask": out("m.png")}));
    assert_eq!(error_code(&r), "PX1310", "mask files are paths too");
    let r = run(json!({"input": inside("in.png"), "output": inside("o.png"), "lut": out("m.png")}));
    assert_eq!(error_code(&r), "PX1310", "Path params are checked");
    let r = run(json!({"inputs": [inside("in.png")], "output": "-"}));
    assert_eq!(error_code(&r), "PX1010");
    let r = tool(&s, "pixelox_preview", json!({"input": out("in.png")}));
    assert_eq!(error_code(&r), "PX1310");
    let r = tool(&s, "pixelox_info", json!({"input": out("in.png")}));
    assert_eq!(error_code(&r), "PX1001", "info is not registered here, checked before the path");
    let r = tool(&s, "pixelox_detect", json!({"input": out("in.png"), "kind": "subject", "in_place": true}));
    assert_eq!(error_code(&r), "PX1310");

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&outside.0, d.path("link")).unwrap();
        let r = run(json!({"input": inside("link/in.png"), "output": inside("o.png")}));
        assert_eq!(error_code(&r), "PX1310", "symlinks are resolved");
        let r = run(json!({"input": inside("in.png"), "output": inside("link/o.png")}));
        assert_eq!(error_code(&r), "PX1310");
    }

    // inside paths pass the sandbox and reach the IO layer (NoIo cannot open anything)
    let r = run(json!({"input": inside("in.png"), "output": inside("o.png"), "mask": "subject"}));
    assert_ne!(error_code(&r), "PX1310", "{r:#}");
}

#[test]
fn read_only_refuses_commands_that_write() {
    let d = TempDir::new("mcp-ro");
    std::fs::write(d.path("in.png"), b"x").unwrap();
    let reg = registry(false);
    let mut cfg = config(&d);
    cfg.read_only = true;
    let s = Server::new(&reg, &NoIo, cfg);
    let input = d.path("in.png").display().to_string();
    let r = tool(
        &s,
        "pixelox_run",
        json!({"command": "filter.blur", "params": {"input": input, "output": d.path("o.png").display().to_string()}}),
    );
    assert_eq!(error_code(&r), "PX1310");
    assert!(r["structuredContent"]["error"]["message"].as_str().unwrap().contains("read-only"));
    let r = tool(&s, "pixelox_run", json!({"command": "dummy.echo", "params": {"text": "x"}, "dry_run": true}));
    assert_eq!(error_code(&r), "PX1310");
    // inspection still runs (and fails later, in NoIo, not at the gate)
    let r = tool(&s, "pixelox_run", json!({"command": "analyze.stats", "params": {"input": input}}));
    assert_ne!(error_code(&r), "PX1310", "{r:#}");
    let r = tool(&s, "pixelox_schema", json!({"command": "filter.blur"}));
    assert_eq!(r["isError"], false);
    let r = tool(&s, "pixelox_jobs", json!({"action": "status", "id": "j1"}));
    assert_eq!(r["structuredContent"]["result"]["read_only"], true);
    assert!(tools::never_writes(reg.get("analyze stats").unwrap()));
    assert!(!tools::never_writes(reg.get("filter blur").unwrap()));
}

#[test]
fn protocol_errors() {
    let d = TempDir::new("mcp-proto");
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    let msgs = exchange(
        &s,
        &[
            call(1, "pixelox_nope", json!({})),
            "{this is not json".to_string(),
            req(2, "nope/method", json!({})),
            json!({"jsonrpc": "1.0", "id": 3, "method": "ping"}).to_string(),
            json!([{"jsonrpc": "2.0", "id": 4, "method": "ping"}, {"jsonrpc": "2.0", "method": "notifications/initialized"}]).to_string(),
            json!({"jsonrpc": "2.0", "id": 9, "result": {}}).to_string(),
            "[]".to_string(),
            req(5, "ping", Value::Null),
        ],
    );
    let unknown = reply(&msgs, 1);
    assert_eq!(unknown["error"]["code"], -32602);
    assert!(unknown["error"]["message"].as_str().unwrap().contains("pixelox_nope"));
    let parse: Vec<&Value> = msgs.iter().filter(|m| m["error"]["code"] == -32700).collect();
    assert_eq!(parse.len(), 1);
    assert_eq!(parse[0]["id"], Value::Null);
    assert_eq!(reply(&msgs, 2)["error"]["code"], -32601);
    assert_eq!(reply(&msgs, 3)["error"]["code"], -32600);
    let batch = msgs.iter().find(|m| m.is_array()).expect("batch reply");
    assert_eq!(batch.as_array().unwrap().len(), 1);
    assert_eq!(batch[0]["id"], 4);
    assert!(msgs.iter().all(|m| m["id"] != 9), "client responses get no reply");
    assert_eq!(msgs.iter().filter(|m| m["error"]["code"] == -32600 && m["id"].is_null()).count(), 1, "empty batch");
    assert_eq!(reply(&msgs, 5)["result"], json!({}), "the loop survives bad input");
}

#[test]
fn describe_runs_the_registered_command() {
    let d = TempDir::new("mcp-describe");
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    let r = tool(&s, "pixelox_describe", json!({}));
    assert_eq!(error_code(&r), "PX1001");
    assert!(r["structuredContent"]["error"]["message"].as_str().unwrap().contains("describe"));

    let reg = registry(true);
    let s = Server::new(&reg, &NoIo, config(&d));
    let r = tool(&s, "pixelox_describe", json!({}));
    assert_eq!(r["isError"], false, "{r:#}");
    assert_eq!(r["content"][0]["text"], "pixelox agent briefing: filter.blur, dummy.echo");
}

#[test]
fn model_downloads_need_both_keys() {
    let d = TempDir::new("mcp-consent");
    let reg = registry(false);
    let consent = json!(["model-download"]);
    let s = Server::new(&reg, &NoIo, config(&d));
    let r = tool(&s, "pixelox_run", json!({"command": "dummy.echo", "consent": consent}));
    assert_eq!(r["structuredContent"]["result"]["yes"], false, "consent alone is not enough");
    let r = tool(&s, "pixelox_run", json!({"command": "models.pull", "consent": consent}));
    assert_eq!(error_code(&r), "PX1502");
    let r = tool(&s, "pixelox_run", json!({"command": "dummy.echo", "consent": ["network"]}));
    assert_eq!(error_code(&r), "PX1010");

    let mut cfg = config(&d);
    cfg.allow_model_downloads = true;
    let s = Server::new(&reg, &NoIo, cfg);
    let r = tool(&s, "pixelox_run", json!({"command": "dummy.echo"}));
    assert_eq!(r["structuredContent"]["result"]["yes"], false, "the server flag alone is not enough");
    let r = tool(&s, "pixelox_run", json!({"command": "models.pull"}));
    assert_eq!(error_code(&r), "PX1502");
    let r = tool(&s, "pixelox_run", json!({"command": "dummy.echo", "consent": consent}));
    assert_eq!(r["structuredContent"]["result"]["yes"], true);
    let r = tool(&s, "pixelox_run", json!({"command": "models.pull", "params": {"text": "lama"}, "consent": consent}));
    assert_eq!(r["structuredContent"]["ok"], true, "{r:#}");
}

#[test]
fn audit_log_gets_one_line_per_call() {
    let d = TempDir::new("mcp-audit");
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    exchange(
        &s,
        &[
            call(1, "pixelox_schema", json!({"command": "filter.blur"})),
            call(2, "pixelox_run", json!({"command": "nope"})),
            call(3, "pixelox_nope", json!({})),
            req(4, "ping", Value::Null),
        ],
    );
    let text = std::fs::read_to_string(d.path("state/mcp-audit.ndjson")).unwrap();
    let lines: Vec<Value> = text.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
    assert_eq!(lines.len(), 3);
    for l in &lines {
        assert_eq!(l["ts"].as_str().unwrap().len(), 20);
        assert_eq!(l["params_sha256"].as_str().unwrap().len(), 64);
        assert!(l["ok"].is_boolean() && l["tool"].is_string());
    }
    let by_tool = |t: &str| lines.iter().find(|l| l["tool"] == t).unwrap().clone();
    assert_eq!(by_tool("pixelox_schema")["ok"], true);
    assert_eq!(by_tool("pixelox_schema")["code"], Value::Null);
    assert_eq!(by_tool("pixelox_run")["code"], "PX1001");
    assert_eq!(by_tool("pixelox_nope")["ok"], false);
}

#[test]
fn jobs_detect_and_progress() {
    let d = TempDir::new("mcp-jobs");
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    let r = tool(&s, "pixelox_jobs", json!({"action": "status", "id": "job-7"}));
    assert_eq!(r["structuredContent"]["command"], "jobs.status");
    assert_eq!(r["structuredContent"]["result"]["id"], "job-7");
    let r = tool(&s, "pixelox_jobs", json!({"action": "status"}));
    assert_eq!(error_code(&r), "PX1003");
    let r = tool(&s, "pixelox_jobs", json!({"action": "wait", "id": "job-7"}));
    assert_eq!(error_code(&r), "PX1001");
    let r = tool(&s, "pixelox_detect", json!({"input": "x.png", "kind": "faces"}));
    assert_eq!(error_code(&r), "PX1001");
    assert!(r["structuredContent"]["error"]["hint"].as_str().unwrap().contains("subject"));

    let msgs = exchange(
        &s,
        &[req(
            1,
            "tools/call",
            json!({"name": "pixelox_run", "arguments": {"command": "dummy.echo"}, "_meta": {"progressToken": "tok"}}),
        )],
    );
    let progress: Vec<&Value> = msgs.iter().filter(|m| m["method"] == "notifications/progress").collect();
    assert!(!progress.is_empty(), "{msgs:#?}");
    assert!(progress.iter().all(|p| p["params"]["progressToken"] == "tok"));
    let pcts: Vec<i64> = progress.iter().map(|p| p["params"]["progress"].as_i64().unwrap()).collect();
    assert!(pcts.windows(2).all(|w| w[0] < w[1]), "progress is monotonic: {pcts:?}");
    assert_eq!(reply(&msgs, 1)["result"]["isError"], false);
}

/// Opens every file as a 200x100 gradient; "encodes" as `format:WxH` text.
struct FakeIo;

impl ImageIo for FakeIo {
    fn open(&self, _: &Path, _: &OpenOptions) -> Result<(Document, Vec<Warning>)> {
        Ok((testing::doc_from(testing::gradient(200, 100)), vec![]))
    }
    fn decode(&self, _: &[u8], _: Option<&str>, _: &OpenOptions) -> Result<(Image, ImageInfo)> {
        Err(Error::unsupported("fake"))
    }
    fn probe(&self, _: &Path) -> Result<ImageInfo> {
        Err(Error::unsupported("fake"))
    }
    fn encode(&self, image: &Image, _: Option<&Document>, path: &Path, opts: &EncodeOptions) -> Result<Vec<Warning>> {
        std::fs::write(path, format!("{}:{}x{}", opts.format, image.width, image.height))?;
        Ok(vec![])
    }
    fn save_pxo(&self, _: &Document, _: &Path) -> Result<()> {
        Err(Error::new("PX1106", "fake"))
    }
    fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec![], vec![])
    }
}

#[test]
fn preview_returns_png_image_content() {
    let d = TempDir::new("mcp-preview");
    std::fs::write(d.path("photo.png"), b"x").unwrap();
    let reg = registry(false);
    let s = Server::new(&reg, &FakeIo, config(&d));
    let input = d.path("photo.png").display().to_string();

    let r = tool(&s, "pixelox_preview", json!({"input": input, "max_size": 64}));
    assert_eq!(r["isError"], false, "{r:#}");
    assert_eq!(r["content"][0]["type"], "image");
    assert_eq!(r["content"][0]["mimeType"], "image/png");
    assert_eq!(r["content"][0]["data"], base64(b"png:64x32"));
    assert_eq!(r["structuredContent"]["preview"]["size"], json!([64, 32]));
    assert_eq!(r["structuredContent"]["document"]["size"], json!([200, 100]));

    let r = tool(&s, "pixelox_preview", json!({"input": input, "layer": "Background", "region": "10,10,50x40"}));
    assert_eq!(r["structuredContent"]["preview"]["size"], json!([50, 40]), "{r:#}");
    assert_eq!(r["structuredContent"]["region"], json!([10, 10, 50, 40]));
    let r = tool(&s, "pixelox_preview", json!({"input": input}));
    assert_eq!(r["structuredContent"]["preview"]["size"], json!([200, 100]), "small images are not enlarged");

    assert_eq!(error_code(&tool(&s, "pixelox_preview", json!({"input": input, "layer": "nope"}))), "PX2301");
    assert_eq!(error_code(&tool(&s, "pixelox_preview", json!({"input": input, "region": "500,500,10x10"}))), "PX1011");
    assert_eq!(error_code(&tool(&s, "pixelox_preview", json!({"input": input, "max_size": 4}))), "PX1013");
    assert_eq!(error_code(&tool(&s, "pixelox_preview", json!({"input": input, "zoom": 2}))), "PX1002");

    let s = Server::new(&reg, &NoIo, config(&d));
    assert_eq!(error_code(&tool(&s, "pixelox_preview", json!({"input": input}))), "PX1105");
}

#[test]
fn resources_list_and_read() {
    let d = TempDir::new("mcp-res");
    std::fs::create_dir_all(d.path("sub")).unwrap();
    std::fs::create_dir_all(d.path(".hidden")).unwrap();
    std::fs::write(d.path("out.png.pixelox.json"), br#"{"pixelox_sidecar": 1}"#).unwrap();
    std::fs::write(d.path("sub/my shot.png.pixelox.json"), br#"{"pixelox_sidecar": 1}"#).unwrap();
    std::fs::write(d.path(".hidden/c.png.pixelox.json"), b"{}").unwrap();
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    let msgs = exchange(
        &s,
        &[
            req(1, "resources/list", json!({})),
            req(2, "resources/read", json!({"uri": "pixelox://sidecar/out.png.pixelox.json"})),
            req(3, "resources/read", json!({"uri": "pixelox://sidecar/sub/my%20shot.png.pixelox.json"})),
            req(4, "resources/read", json!({"uri": "pixelox://sidecar/../etc/x.pixelox.json"})),
            req(5, "resources/read", json!({"uri": "pixelox://docs/filter.blur"})),
            req(6, "resources/read", json!({"uri": "pixelox://docs/PX1401"})),
            req(7, "resources/read", json!({"uri": "pixelox://nope"})),
            req(8, "resources/read", json!({"uri": "pixelox://sidecar/missing.png.pixelox.json"})),
            req(9, "resources/templates/list", json!({})),
        ],
    );
    let list = reply(&msgs, 1)["result"]["resources"].as_array().unwrap().clone();
    let uris: Vec<&str> = list.iter().map(|r| r["uri"].as_str().unwrap()).collect();
    assert!(uris.contains(&"pixelox://sidecar/out.png.pixelox.json"), "{uris:?}");
    assert!(uris.contains(&"pixelox://sidecar/sub/my%20shot.png.pixelox.json"), "{uris:?}");
    assert!(!uris.iter().any(|u| u.contains("hidden")));
    assert!(uris.contains(&"pixelox://docs/filter.blur"));
    assert!(uris.contains(&"pixelox://docs/mcp.serve"));
    let read = reply(&msgs, 2)["result"]["contents"][0].clone();
    assert_eq!(read["mimeType"], "application/json");
    assert!(read["text"].as_str().unwrap().contains("pixelox_sidecar"));
    assert!(reply(&msgs, 3)["result"]["contents"][0]["text"].as_str().unwrap().contains("pixelox_sidecar"));
    let escape = reply(&msgs, 4);
    assert_eq!(escape["error"]["code"], -32602);
    assert_eq!(escape["error"]["data"]["code"], "PX1310");
    let page = reply(&msgs, 5)["result"]["contents"][0]["text"].as_str().unwrap().to_string();
    assert!(page.contains("pixelox filter blur") && page.contains("--sigma"), "{page}");
    assert!(reply(&msgs, 6)["result"]["contents"][0]["text"].as_str().unwrap().contains("PX1401"));
    assert_eq!(reply(&msgs, 7)["error"]["code"], -32002);
    assert_eq!(reply(&msgs, 8)["error"]["code"], -32002);
    assert_eq!(reply(&msgs, 9)["result"]["resourceTemplates"].as_array().unwrap().len(), 2);

    #[cfg(unix)]
    {
        let outside = TempDir::new("mcp-res-out");
        std::fs::write(outside.path("secret.png.pixelox.json"), b"{}").unwrap();
        std::os::unix::fs::symlink(outside.path("secret.png.pixelox.json"), d.path("link.png.pixelox.json")).unwrap();
        let msgs = exchange(&s, &[req(1, "resources/read", json!({"uri": "pixelox://sidecar/link.png.pixelox.json"}))]);
        assert_eq!(reply(&msgs, 1)["error"]["data"]["code"], "PX1310");
    }
}

#[test]
fn docs_tool_falls_back_to_the_spec() {
    let d = TempDir::new("mcp-docs");
    let reg = registry(false);
    let s = Server::new(&reg, &NoIo, config(&d));
    let r = tool(&s, "pixelox_docs", json!({"command": "mcp.serve"}));
    assert_eq!(r["isError"], false, "{r:#}");
    let text = r["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("--allow") && text.contains("pixelox mcp serve --allow ./work"), "{text}");
    let r = tool(&s, "pixelox_docs", json!({"command": "errors/PX1310"}));
    assert!(r["content"][0]["text"].as_str().unwrap().starts_with("# PX1310"));
    assert_eq!(error_code(&tool(&s, "pixelox_docs", json!({"command": "nope.nope"}))), "PX1001");
}

#[test]
fn mcp_serve_spec_follows_the_authoring_contract() {
    let ops = ops();
    assert_eq!(ops.len(), 1);
    let spec = &ops[0];
    assert_eq!(spec.name, "mcp serve");
    assert_eq!(spec.inputs, Inputs::NONE);
    assert!(spec.summary.len() <= 60 && !spec.summary.ends_with('.'), "{}", spec.summary);
    assert!(!spec.about.is_empty());
    assert!(spec.examples.len() >= 3);
    for e in &spec.examples {
        assert!(e.cmd.starts_with("pixelox mcp serve"), "{}", e.cmd);
        assert!(!e.explain.is_empty());
    }
    for p in &spec.params {
        assert!(!p.help.is_empty(), "--{} needs help", p.name);
    }
    for name in ["allow", "read-only", "allow-model-downloads"] {
        assert!(spec.find_param(name).is_some(), "--{name}");
    }
    let a = Args::build(
        spec,
        vec![("allow".into(), "a".into()), ("allow".into(), "b".into()), ("read-only".into(), "true".into())],
    )
    .unwrap();
    assert_eq!(a.all("allow"), vec!["a", "b"]);
}

#[test]
fn serve_dry_run_reports_the_configuration() {
    let d = TempDir::new("mcp-serve-dry");
    let reg = registry(false);
    let spec = reg.get("mcp serve").unwrap();
    let g = pixelox_core::Globals { dry_run: true, ..Default::default() };
    let mut ctx = OpCtx::new(spec.name, vec![], g, &NoIo, &pixelox_core::ctx::NoProgress);
    ctx.registry = Some(&reg);
    let root = d.0.display().to_string();
    let a = Args::build(spec, vec![("allow".into(), root), ("read-only".into(), "true".into())]).unwrap();
    let Runner::Custom(f) = spec.runner else { panic!("custom op") };
    let v = f(&a, &mut ctx).unwrap();
    assert_eq!(v["read_only"], true);
    assert_eq!(v["allow"][0], d.0.canonicalize().unwrap().display().to_string());
    assert_eq!(v["tools"].as_array().unwrap().len(), 8);

    let a = Args::build(spec, vec![("allow".into(), d.path("missing").display().to_string())]).unwrap();
    assert_eq!(f(&a, &mut ctx).unwrap_err().code, "PX1101");
}
