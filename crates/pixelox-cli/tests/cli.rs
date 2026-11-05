//! End-to-end tests of the built binary: the agent contract (envelopes, exit codes, consent
//! gates, dry runs, sidecars, replay, verify), determinism across runs and thread counts,
//! pipelines, batches, and the MCP server.

use pixelox_core::testing::TempDir;
use serde_json::Value;
use std::path::Path;
use std::process::{Command, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_pixelox");

fn run_in(dir: &Path, args: &[&str]) -> (i32, Value) {
    let out = Command::new(BIN)
        .args(args)
        .arg("--json")
        .current_dir(dir)
        .env("PIXELOX_HOME", dir.join(".home"))
        .env("PIXELOX_SYSTEM_FONTS", "0")
        .output()
        .expect("run pixelox");
    let v: Value = serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|e| panic!("{args:?}: not JSON ({e}): {}", String::from_utf8_lossy(&out.stdout)));
    (out.status.code().unwrap_or(-1), v)
}

fn ok(dir: &Path, args: &[&str]) -> Value {
    let (code, v) = run_in(dir, args);
    assert_eq!(code, 0, "{args:?} failed: {}", v["error"]);
    assert_eq!(v["ok"], true);
    v
}

fn sample(dir: &Path) {
    ok(dir, &["new", "-o", "base.pxo", "--size", "160x100", "--fill", "#336699"]);
    ok(
        dir,
        &["shape", "ellipse", "base.pxo", "--in-place", "--rect", "20,10,80x60", "--fill", "#ffcc00", "--name", "Sun"],
    );
    ok(
        dir,
        &[
            "text",
            "add",
            "base.pxo",
            "--in-place",
            "--content",
            "Hi",
            "--font",
            "sans-serif",
            "--size",
            "24px",
            "--at",
            "120,50",
            "--anchor",
            "c",
            "--name",
            "Label",
        ],
    );
    ok(dir, &["export", "base.pxo", "-o", "photo.png"]);
}

#[test]
fn envelope_shape_and_sidecar() {
    let t = TempDir::new("cli-env");
    sample(&t.0);
    let v = ok(&t.0, &["filter", "gaussian", "photo.png", "-o", "soft.png", "--sigma", "2"]);
    assert_eq!(v["tool"], "pixelox");
    assert_eq!(v["envelope"], 1);
    assert_eq!(v["command"], "filter.gaussian");
    assert_eq!(v["result"]["op"], "filter.gaussian");
    assert_eq!(v["inputs"][0]["path"], "photo.png");
    assert_eq!(v["outputs"][0]["sha256"].as_str().unwrap().len(), 64);
    let sc: Value = serde_json::from_slice(&std::fs::read(t.path("soft.png.pixelox.json")).unwrap()).unwrap();
    assert_eq!(sc["pixelox_sidecar"], 1);
    let ops: Vec<&str> = sc["recipe"].as_array().unwrap().iter().map(|s| s["op"].as_str().unwrap()).collect();
    assert_eq!(ops, ["open", "filter.gaussian", "export"]);
    assert_eq!(sc["inputs"][0]["role"], "source");
    assert_eq!(sc["output"]["sha256"], v["outputs"][0]["sha256"]);
}

#[test]
fn consent_gates_and_exit_codes() {
    let t = TempDir::new("cli-gates");
    sample(&t.0);
    // exists without --overwrite: exit 4, nothing changed
    let before = std::fs::read(t.path("photo.png")).unwrap();
    let (code, v) = run_in(&t.0, &["export", "base.pxo", "-o", "photo.png"]);
    assert_eq!((code, v["error"]["code"].as_str()), (4, Some("PX1401")));
    assert_eq!(std::fs::read(t.path("photo.png")).unwrap(), before);
    // source images are never edited in place
    let (code, v) = run_in(&t.0, &["filter", "gaussian", "photo.png", "--in-place"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX1108")));
    // mutating without a destination
    let (code, v) = run_in(&t.0, &["filter", "gaussian", "base.pxo"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX1004")));
    // missing input, unknown command, bad value, out of range
    assert_eq!(run_in(&t.0, &["info", "nope.png"]).0, 3);
    let (code, v) = run_in(&t.0, &["frobnicate", "x.png"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX1001")));
    let (code, v) = run_in(&t.0, &["layer", "opacity", "base.pxo", "-o", "x.pxo", "--layer", "Sun", "--value", "150"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX1013")));
    // gated feature: exit 12
    let (code, v) = run_in(&t.0, &["detect", "subject", "photo.png", "--cutout", "c.png"]);
    assert_eq!((code, v["error"]["code"].as_str()), (12, Some("PX1510")));
    // wrong layer kind names the remedy
    let (code, v) = run_in(&t.0, &["filter", "gaussian", "base.pxo", "-o", "x.pxo", "--layer", "Sun"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX2303")));
    assert!(v["error"]["hint"].as_str().unwrap().contains("layer rasterize"));
}

#[test]
fn dry_run_writes_nothing() {
    let t = TempDir::new("cli-dry");
    sample(&t.0);
    let v = ok(&t.0, &["filter", "gaussian", "photo.png", "-o", "planned.png", "--sigma", "3", "--dry-run"]);
    assert_eq!(v["dry_run"], true);
    assert_eq!(v["result"]["plan"]["outputs_predicted"][0]["action"], "write");
    assert!(!t.path("planned.png").exists() && !t.path("planned.png.pixelox.json").exists());
    let before = std::fs::read(t.path("base.pxo")).unwrap();
    ok(&t.0, &["layer", "hide", "base.pxo", "--in-place", "--layer", "Sun", "--dry-run"]);
    assert_eq!(std::fs::read(t.path("base.pxo")).unwrap(), before);
}

#[test]
fn replay_is_bit_identical_and_verify_catches_tampering() {
    let t = TempDir::new("cli-replay");
    sample(&t.0);
    ok(&t.0, &["open", "photo.png", "-o", "doc.pxo"]);
    ok(&t.0, &["adjust", "hsl", "doc.pxo", "--in-place", "--hue", "40", "--as-layer"]);
    ok(&t.0, &["filter", "add-noise", "doc.pxo", "--in-place", "--layer", "Background", "--amount", "8"]);
    ok(&t.0, &["export", "doc.pxo", "-o", "final.png"]);
    let v = ok(&t.0, &["replay", "final.png.pixelox.json", "-o", "again.png", "--strict"]);
    assert_eq!(v["result"]["identical"], true);
    assert_eq!(std::fs::read(t.path("final.png")).unwrap(), std::fs::read(t.path("again.png")).unwrap());
    ok(&t.0, &["verify", "final.png.pixelox.json", "--full"]);
    // a changed source is input drift (PX1601); a changed output is PX1602; both exit 9
    let mut f = std::fs::OpenOptions::new().append(true).open(t.path("final.png")).unwrap();
    std::io::Write::write_all(&mut f, b"x").unwrap();
    let (code, v) = run_in(&t.0, &["verify", "final.png.pixelox.json"]);
    assert_eq!((code, v["error"]["code"].as_str()), (9, Some("PX1602")));
    std::fs::write(t.path("photo.png"), b"changed").unwrap();
    let (code, v) = run_in(&t.0, &["replay", "again.png.pixelox.json", "-o", "third.png"]);
    assert_eq!((code, v["error"]["code"].as_str()), (9, Some("PX1601")));
}

#[test]
fn deterministic_across_runs_and_thread_counts() {
    let t = TempDir::new("cli-det");
    sample(&t.0);
    let mut outs = Vec::new();
    for (i, threads) in ["1", "3", "8"].iter().enumerate() {
        let doc = format!("d{i}.pxo");
        let png = format!("d{i}.png");
        ok(&t.0, &["open", "photo.png", "-o", &doc, "--threads", threads]);
        ok(&t.0, &["filter", "add-noise", &doc, "--in-place", "--amount", "10", "--threads", threads]);
        ok(&t.0, &["filter", "gaussian", &doc, "--in-place", "--sigma", "13", "--threads", threads]);
        ok(&t.0, &["transform", "resize", &doc, "--in-place", "--size", "97x", "--threads", threads]);
        ok(&t.0, &["export", &doc, "-o", &png, "--threads", threads]);
        outs.push((std::fs::read(t.path(&doc)).unwrap(), std::fs::read(t.path(&png)).unwrap()));
    }
    for o in &outs[1..] {
        assert!(o.0 == outs[0].0, ".pxo bytes differ across thread counts");
        assert!(o.1 == outs[0].1, "PNG bytes differ across thread counts");
    }
    // a different seed changes seeded ops
    ok(&t.0, &["filter", "add-noise", "photo.png", "-o", "s1.png", "--amount", "10", "--seed", "1"]);
    ok(&t.0, &["filter", "add-noise", "photo.png", "-o", "s2.png", "--amount", "10", "--seed", "2"]);
    assert_ne!(std::fs::read(t.path("s1.png")).unwrap(), std::fs::read(t.path("s2.png")).unwrap());
}

#[test]
fn pipelines_and_batches() {
    let t = TempDir::new("cli-pipe");
    sample(&t.0);
    std::fs::write(
        t.path("p.yml"),
        "version: 1\nvars: { sigma: \"2\" }\nsteps:\n  - { id: open, run: open, with: { input: photo.png } }\n  - { id: blur, run: filter.gaussian, with: { sigma: \"${vars.sigma}\" } }\n  - { id: out, run: export, with: { output: out/p.png } }\noutputs: [out/p.png]\n",
    )
    .unwrap();
    ok(&t.0, &["pipeline", "validate", "p.yml"]);
    let v = ok(&t.0, &["pipeline", "run", "p.yml", "--dry-run"]);
    assert_eq!(v["result"]["plan"]["order"], serde_json::json!(["open", "blur", "out"]));
    assert!(!t.path("out/p.png").exists());
    ok(&t.0, &["pipeline", "run", "p.yml", "--var", "sigma=4"]);
    let sc: Value = serde_json::from_slice(&std::fs::read(t.path("out/p.png.pixelox.json")).unwrap()).unwrap();
    assert_eq!(sc["recipe"][1]["params"]["sigma"], "4");
    ok(&t.0, &["replay", "out/p.png.pixelox.json", "-o", "out/p2.png", "--strict"]);
    std::fs::create_dir_all(t.path("in")).unwrap();
    for n in ["a", "b", "c"] {
        std::fs::copy(t.path("photo.png"), t.path(&format!("in/{n}.png"))).unwrap();
    }
    let v = ok(
        &t.0,
        &[
            "batch",
            "run",
            "--glob",
            "in/*.png",
            "--cmd",
            "transform resize {in} -o small/{stem}.jpg --size 40x",
            "--jobs",
            "2",
        ],
    );
    assert_eq!(v["result"]["succeeded"], 3);
    assert!(t.path("small/c.jpg").exists() && t.path("small/c.jpg.pixelox.json").exists());
    // a failing item fails the batch (exit 1) unless --continue-on-error
    let (code, _) = run_in(
        &t.0,
        &["batch", "run", "--glob", "in/*.png", "--cmd", "transform resize {in} -o small/{stem}.jpg --size 40x"],
    );
    assert_eq!(code, 1);
    let v = ok(
        &t.0,
        &[
            "batch",
            "run",
            "--glob",
            "in/*.png",
            "--cmd",
            "transform resize {in} -o small/{stem}.jpg --size 40x",
            "--continue-on-error",
        ],
    );
    assert_eq!(v["result"]["failed"], 3);
}

#[test]
fn help_json_prints_the_schema() {
    let out = Command::new(BIN).args(["filter", "gaussian", "--help", "--json"]).output().unwrap();
    let v: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(v["command"], "filter.gaussian");
    assert!(v["params"]["properties"]["sigma"].is_object());
}

#[test]
fn mcp_serve_handshake() {
    use std::io::{BufRead, BufReader, Write};
    let t = TempDir::new("cli-mcp");
    let mut child = Command::new(BIN)
        .args(["mcp", "serve", "--allow", t.0.to_str().unwrap()])
        .env("PIXELOX_HOME", t.0.join(".home"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let mut lines = BufReader::new(child.stdout.take().unwrap()).lines();
    writeln!(stdin, r#"{{"jsonrpc":"2.0","id":1,"method":"initialize","params":{{"protocolVersion":"2025-06-18","capabilities":{{}},"clientInfo":{{"name":"t","version":"0"}}}}}}"#).unwrap();
    let r: Value = serde_json::from_str(&lines.next().unwrap().unwrap()).unwrap();
    assert_eq!(r["result"]["serverInfo"]["name"], "pixelox");
    writeln!(stdin, r#"{{"jsonrpc":"2.0","id":2,"method":"tools/list"}}"#).unwrap();
    let r: Value = serde_json::from_str(&lines.next().unwrap().unwrap()).unwrap();
    assert_eq!(r["result"]["tools"].as_array().unwrap().len(), 8);
    drop(stdin);
    let _ = child.wait();
}
