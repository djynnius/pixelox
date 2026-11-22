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
    // a model-backed command whose model is not downloaded: consent needed (exit 6), or
    // exit 12 in builds without the ML runtime
    let (code, v) = run_in(&t.0, &["detect", "subject", "photo.png", "--cutout", "c.png"]);
    assert!(
        matches!((code, v["error"]["code"].as_str()), (6, Some("PX1502")) | (12, Some("PX1510"))),
        "{code} {}",
        v["error"]
    );
    assert!(!t.path("c.png").exists());
    // models that are not in this release: exit 12
    assert_eq!(run_in(&t.0, &["detect", "faces", "photo.png"]).0, 12);
    // wrong layer kind names the remedy
    let (code, v) =
        run_in(&t.0, &["paint", "stroke", "base.pxo", "-o", "x.pxo", "--layer", "Sun", "--points", "1,1 9,9"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX2303")));
    assert!(v["error"]["hint"].as_str().unwrap().contains("layer rasterize"));
    // a fill on a live layer names the live alternative
    let (code, v) = run_in(&t.0, &["fill", "solid", "base.pxo", "-o", "x.pxo", "--layer", "Sun", "--color", "red"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX2303")));
    assert!(v["error"]["hint"].as_str().unwrap().contains("fx overlay"), "{}", v["error"]["hint"]);
    // a filter on a live layer is a smart filter (ADR 0014); a mask is refused with a hint
    let (code, v) = run_in(&t.0, &["filter", "gaussian", "base.pxo", "-o", "x.pxo", "--layer", "Sun"]);
    assert_eq!((code, v["result"]["smart_filter"].as_str()), (0, Some("filter.gaussian")), "{v}");
    let (code, v) = run_in(
        &t.0,
        &["filter", "gaussian", "base.pxo", "-o", "x.pxo", "--layer", "Sun", "--mask", "@alpha:Sun", "--overwrite"],
    );
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX2303")));
    assert!(v["error"]["hint"].as_str().unwrap().contains("mask add"));
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
fn smart_filters_on_placed_layers_replay_and_bake() {
    let t = TempDir::new("cli-smart");
    let d = t.0.as_path();
    sample(d);
    ok(d, &["new", "-o", "doc.pxo", "--size", "200x120", "--fill", "white"]);
    ok(d, &["comp", "place", "doc.pxo", "--in-place", "photo.png", "--as", "Photo", "--size", "100x", "--at", "50,30"]);
    let v = ok(d, &["filter", "gaussian", "doc.pxo", "--in-place", "--layer", "Photo", "--sigma", "6"]);
    assert_eq!(v["result"]["smart_filter"], "filter.gaussian");
    let v = ok(d, &["filter", "add-noise", "doc.pxo", "--in-place", "--layer", "Photo", "--amount", "8"]);
    assert_eq!(v["result"]["smart_filters"], 2);
    ok(d, &["adjust", "curves", "doc.pxo", "--in-place", "--layer", "Photo", "--points", "0,0 128,100 255,255"]);
    // the layer stays placed and lists its filters
    let v = ok(d, &["layer", "list", "doc.pxo"]);
    let photo = v["result"]["layers"].as_array().unwrap().iter().find(|l| l["name"] == "Photo").unwrap().clone();
    assert_eq!(photo["kind"], "placed");
    assert_eq!(photo["smart_filters"].as_array().unwrap().len(), 3);
    // the blur reaches past the placed rect (rendered on the canvas)
    ok(d, &["export", "doc.pxo", "-o", "out.png"]);
    let v = ok(d, &["analyze", "stats", "out.png", "--rect", "47,60,1x1"]);
    assert!(v["result"]["channels"]["l"]["mean"].as_f64().unwrap() < 250.0, "{}", v["result"]);
    // replay rebuilds it bit for bit
    let v = ok(d, &["replay", "out.png.pixelox.json", "-o", "again.png"]);
    assert_eq!(v["result"]["identical"], true);
    // a masked filter on a placed layer is refused with a rasterize hint
    let (code, v) =
        run_in(d, &["filter", "gaussian", "doc.pxo", "--in-place", "--layer", "Photo", "--mask", "@alpha:Photo"]);
    assert_eq!(code, 2);
    assert_eq!(v["error"]["code"], "PX2303");
    // rasterizing bakes the filters into pixels with the same look
    ok(d, &["layer", "rasterize", "doc.pxo", "-o", "flat.pxo", "--layer", "Photo"]);
    ok(d, &["export", "flat.pxo", "-o", "flat.png"]);
    let v = ok(d, &["analyze", "diff", "out.png", "flat.png"]);
    assert!(v["result"]["ssim"].as_f64().unwrap() > 0.999, "{}", v["result"]);
}

/// ADR 0014 end to end: blurs grow small layers, filters on live text and shapes,
/// placement flags, true negative coordinates, and gradient and live masks, all replayed
/// bit for bit.
#[test]
fn layers_bounds_and_masks_replay() {
    let t = TempDir::new("cli-ws1");
    let d = t.0.as_path();
    let layer = |v: &Value, name: &str| -> Value {
        v["result"]["layers"].as_array().unwrap().iter().find(|l| l["name"] == name).cloned().unwrap()
    };
    ok(d, &["new", "-o", "doc.pxo", "--size", "240x160", "--fill", "#203040"]);
    // a rasterized shape blurred: the layer grows, so the blur is not cut off
    ok(
        d,
        &["shape", "ellipse", "doc.pxo", "--in-place", "--name", "Spot", "--rect", "90,60,60x40", "--fill", "#ffcc00"],
    );
    ok(d, &["layer", "rasterize", "doc.pxo", "--in-place", "--layer", "Spot"]);
    let v = ok(d, &["filter", "gaussian", "doc.pxo", "--in-place", "--layer", "Spot", "--sigma", "8"]);
    let g = v["result"]["grown"].as_array().expect("grown").iter().map(|x| x.as_i64().unwrap()).collect::<Vec<_>>();
    assert!(g.iter().all(|&x| x >= 15), "{g:?}");
    // a live shape with a smart motion blur, hanging off the top-left corner (0-20)
    let v = ok(
        d,
        &["shape", "rect", "doc.pxo", "--in-place", "--name", "Bar", "--rect", "0-20,0-10,80x30", "--fill", "#ff3366"],
    );
    assert_eq!(v["result"]["bbox"], serde_json::json!([-20.0, -10.0, 80.0, 30.0]));
    let v = ok(
        d,
        &["filter", "motion-blur", "doc.pxo", "--in-place", "--layer", "Bar", "--angle", "0", "--distance", "40"],
    );
    assert_eq!(v["result"]["smart_filter"], "filter.motion-blur");
    // live text with a blur stays text; new layers go on top, --below places one lower
    ok(
        d,
        &[
            "text",
            "add",
            "doc.pxo",
            "--in-place",
            "--name",
            "Title",
            "--content",
            "Hi",
            "--size",
            "40px",
            "--at",
            "50%,50%",
            "--anchor",
            "c",
            "--color",
            "white",
        ],
    );
    ok(d, &["layer", "dup", "doc.pxo", "--in-place", "--layer", "Title", "--name", "Glow", "--below", "Title"]);
    ok(d, &["filter", "gaussian", "doc.pxo", "--in-place", "--layer", "Glow", "--sigma", "6"]);
    ok(d, &["layer", "reorder", "doc.pxo", "--in-place", "--layer", "Spot", "--index", "1"]);
    ok(d, &["layer", "add", "doc.pxo", "--in-place", "--name", "Top", "--fill", "#00000010"]);
    let v = ok(d, &["layer", "list", "doc.pxo"]);
    let names: Vec<&str> =
        v["result"]["layers"].as_array().unwrap().iter().map(|l| l["name"].as_str().unwrap()).collect();
    assert_eq!(
        names,
        ["Top", "Spot", "Title", "Glow", "Bar", "Background"],
        "after a reorder, a new layer still goes on top"
    );
    assert_eq!(layer(&v, "Glow")["kind"], "text");
    // a gradient mask and a live mask that follows the text
    ok(
        d,
        &[
            "mask",
            "add",
            "doc.pxo",
            "--in-place",
            "--layer",
            "Spot",
            "--gradient",
            "linear",
            "--from",
            "0,40%",
            "--to",
            "0,80%",
        ],
    );
    ok(d, &["mask", "add", "doc.pxo", "--in-place", "--layer", "Top", "--from-layer", "Title"]);
    let before = ok(d, &["mask", "list", "doc.pxo"]);
    ok(d, &["text", "edit", "doc.pxo", "--in-place", "--layer", "Title", "--content", "Hello"]);
    let after = ok(d, &["mask", "list", "doc.pxo"]);
    let cov = |v: &Value| {
        v["result"]["layer_masks"].as_array().unwrap().iter().find(|m| m["layer"] == "Top").unwrap()["area"]
            .as_f64()
            .unwrap()
    };
    assert!(
        cov(&after) > cov(&before) * 1.5,
        "the live mask follows the longer text: {} -> {}",
        cov(&before),
        cov(&after)
    );
    ok(d, &["export", "doc.pxo", "-o", "out.png"]);
    let v = ok(d, &["replay", "out.png.pixelox.json", "-o", "again.png"]);
    assert_eq!(v["result"]["identical"], true);
    // flags that conflict or point at the wrong kind
    let (code, v) = run_in(d, &["layer", "add", "doc.pxo", "-o", "x.pxo", "--into", "Title"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX2330")));
    let (code, v) = run_in(d, &["layer", "add", "doc.pxo", "-o", "x.pxo", "--above", "Title", "--below", "Spot"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX1005")));
    let (code, v) = run_in(d, &["adjust", "invert", "doc.pxo", "-o", "x.pxo", "--layer", "Spot", "--below", "Spot"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX1005")));
}

/// Runs only when CI (or you) points PIXELOX_TEST_U2NETP at the 5 MB u2netp.onnx; the
/// file is copied into a private models directory, so nothing is downloaded here.
#[test]
fn local_models_end_to_end() {
    let Some(model) = std::env::var_os("PIXELOX_TEST_U2NETP") else {
        eprintln!("skipped: set PIXELOX_TEST_U2NETP to a u2netp.onnx to run the model test");
        return;
    };
    let t = TempDir::new("cli-ml");
    let d = t.0.as_path();
    sample(d);
    let v = ok(d, &["models", "list"]);
    if v["result"]["build_supports_ml"] != true {
        eprintln!("skipped: this build has no ML runtime");
        return;
    }
    let dir = std::path::PathBuf::from(v["result"]["models_dir"].as_str().unwrap());
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::copy(&model, dir.join("u2netp.onnx")).unwrap();
    // side files without a document; the model file is adopted after its hash is checked
    let v = ok(d, &["detect", "subject", "photo.png", "--fast", "--cutout", "cut.png", "--mask-out", "mask.png"]);
    assert_eq!(v["result"]["model"], "u2netp");
    let cov = v["result"]["mask"]["coverage"].as_f64().unwrap();
    assert!(cov > 0.5 && cov < 99.5, "coverage {cov}");
    for f in ["cut.png", "mask.png"] {
        let r = ok(d, &["replay", &format!("{f}.pixelox.json"), "-o", &format!("again-{f}")]);
        assert_eq!(r["result"]["identical"], true, "{f}");
    }
    // into a document, then the classic export --apply-mask: the same bytes as --cutout
    ok(d, &["open", "photo.png", "-o", "doc.pxo"]);
    ok(d, &["detect", "subject", "doc.pxo", "--in-place", "--fast", "--save-mask", "subject"]);
    ok(d, &["export", "doc.pxo", "-o", "doc-cut.png", "--apply-mask", "subject"]);
    assert_eq!(std::fs::read(t.path("cut.png")).unwrap(), std::fs::read(t.path("doc-cut.png")).unwrap());
    let r = ok(d, &["replay", "doc-cut.png.pixelox.json", "-o", "again-doc-cut.png"]);
    assert_eq!(r["result"]["identical"], true);
    // salient into the selection; registry bookkeeping
    let v = ok(d, &["detect", "salient", "doc.pxo", "-o", "sal.pxo"]);
    assert_eq!(v["result"]["selection"], true);
    let v = ok(d, &["models", "verify"]);
    assert_eq!(v["result"]["models"][0]["ok"], true);
    let v = ok(d, &["models", "rm", "u2netp"]);
    assert_eq!(v["result"]["removed"], true);
    let (code, v) = run_in(d, &["detect", "salient", "doc.pxo", "-o", "sal2.pxo"]);
    assert_eq!((code, v["error"]["code"].as_str()), (6, Some("PX1502")));
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

fn run_env(dir: &Path, args: &[&str], env: &[(&str, &str)]) -> (i32, Value) {
    let mut c = Command::new(BIN);
    c.args(args).arg("--json").current_dir(dir).env("PIXELOX_HOME", dir.join(".home")).env("PIXELOX_SYSTEM_FONTS", "0");
    for (k, v) in env {
        c.env(k, v);
    }
    let out = c.output().expect("run pixelox");
    let v: Value = serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|e| panic!("{args:?}: not JSON ({e}): {}", String::from_utf8_lossy(&out.stdout)));
    (out.status.code().unwrap_or(-1), v)
}

/// Font classes, live type settings (fit, case, scale, faux bold, warp, ink anchoring), a
/// non-uniform transform that keeps text live, and a bit-identical replay of all of it.
#[test]
fn live_type_replays_bit_for_bit() {
    let t = TempDir::new("cli-type");
    let d = t.0.as_path();
    ok(d, &["new", "-o", "t.pxo", "--size", "600x400", "--fill", "#f1ede4"]);
    let v = ok(
        d,
        &[
            "text",
            "add",
            "t.pxo",
            "--in-place",
            "--name",
            "L1",
            "--content",
            "make",
            "--case",
            "upper",
            "--font",
            "@condensed-heavy",
            "--fit-width",
            "500",
            "--at",
            "50,40",
            "--anchor-to",
            "ink",
            "--color",
            "#111111",
        ],
    );
    // no class member on a bare machine: a reported substitute, never an error
    assert_eq!(v["result"]["font"]["resolved"]["class"], "@condensed-heavy");
    assert_eq!(v["result"]["fit"]["ink"][0], 500.0);
    assert_eq!(&v["result"]["bounds"].as_array().unwrap()[..2], &[50, 40]);
    ok(
        d,
        &[
            "text",
            "add",
            "t.pxo",
            "--in-place",
            "--name",
            "W",
            "--content",
            "WARPED",
            "--font",
            "sans-serif",
            "--size",
            "60px",
            "--at",
            "300,250",
            "--anchor",
            "c",
            "--scale-x",
            "80%",
            "--faux-bold",
            "20",
        ],
    );
    let v = ok(d, &["text", "warp", "t.pxo", "--in-place", "--layer", "W", "--preset", "arc", "--bend", "40"]);
    assert_eq!(v["result"]["warp"]["preset"], "arc");
    let v =
        ok(d, &["transform", "scale", "t.pxo", "--in-place", "--layer", "L1", "--size", "500x200", "--anchor", "tl"]);
    assert!(v["warnings"].as_array().unwrap().iter().all(|w| w["code"] != "PXW2003"), "{}", v["warnings"]);
    let v = ok(d, &["layer", "list", "t.pxo"]);
    let kinds: Vec<&str> =
        v["result"]["layers"].as_array().unwrap().iter().map(|l| l["kind"].as_str().unwrap()).collect();
    assert_eq!(kinds.iter().filter(|k| **k == "text").count(), 2, "{kinds:?}");
    // the words change, the fit and warp follow
    ok(d, &["text", "edit", "t.pxo", "--in-place", "--layer", "W", "--content", "BENT"]);
    ok(d, &["export", "t.pxo", "-o", "t.png"]);
    let v = ok(d, &["replay", "t.png.pixelox.json", "-o", "again.png", "--strict"]);
    assert_eq!(v["result"]["identical"], true);
    assert_eq!(v["result"]["missing_fonts"], serde_json::json!([]));
    ok(d, &["verify", "t.png.pixelox.json", "--full"]);
    // strict mode keeps the old error for a missing family
    let (code, v) = run_env(
        d,
        &["text", "add", "t.pxo", "--in-place", "--content", "x", "--font", "Zzqx Gone"],
        &[("PIXELOX_STRICT_FONTS", "1")],
    );
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX4102")));
    let (code, v) = run_in(d, &["text", "add", "t.pxo", "--in-place", "--content", "x", "--font", "@nope"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX4110")));
}

/// Serve `files` (path, bytes) over HTTP on a local port, one request per connection.
/// Every response says `Connection: close`: otherwise the client keeps the socket for its next
/// request, races the close below, and gets a reset (seen on Linux CI).
fn serve(files: Vec<(String, Vec<u8>)>) -> String {
    use std::io::{BufRead, BufReader, Write};
    let l = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = l.local_addr().unwrap();
    std::thread::spawn(move || {
        for s in l.incoming() {
            let Ok(mut s) = s else { break };
            let mut reader = BufReader::new(s.try_clone().unwrap());
            let mut first = String::new();
            let _ = reader.read_line(&mut first);
            loop {
                let mut line = String::new();
                if reader.read_line(&mut line).unwrap_or(0) == 0 || line == "\r\n" {
                    break;
                }
            }
            let path = first.split_whitespace().nth(1).unwrap_or("").to_string();
            match files.iter().find(|(p, _)| *p == path) {
                Some((_, body)) => {
                    let head =
                        format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
                    let _ = s.write_all(head.as_bytes());
                    let _ = s.write_all(body);
                }
                None => {
                    let _ = s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n");
                }
            }
        }
    });
    format!("http://{addr}")
}

/// `fonts get` needs consent, plans without it, refuses offline, and downloads a family of
/// the pinned Google Fonts commit (here from a local mirror) with its license, once.
#[test]
fn fonts_get_consent_and_download() {
    let t = TempDir::new("cli-fonts");
    let d = t.0.as_path();
    let (code, v) = run_in(d, &["fonts", "get", "Bebas Neue"]);
    assert_eq!((code, v["error"]["code"].as_str()), (6, Some("PX1540")));
    assert!(v["error"]["hint"].as_str().unwrap().contains("pixelox fonts get \"Bebas Neue\" -y"));
    let v = ok(d, &["fonts", "get", "Bebas Neue", "--dry-run"]);
    assert_eq!(v["result"]["families"][0]["status"], "planned");
    let (code, v) = run_env(d, &["fonts", "get", "Bebas Neue", "-y"], &[("PIXELOX_OFFLINE", "1")]);
    assert_eq!((code, v["error"]["code"].as_str()), (1, Some("PX1504")));
    // an uncurated family through a local mirror of the pinned commit
    let commit = "06bfe0d62a98375cb09c942fa18e68bd32cff057";
    let font = std::fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("../pixelox-text/assets/Tuffy.ttf")).unwrap();
    let meta = "name: \"Test Grotesk\"\nlicense: \"OFL\"\nfonts {\n  name: \"Test Grotesk\"\n  style: \"normal\"\n  weight: 400\n  filename: \"TestGrotesk-Regular.ttf\"\n}\n";
    let base = serve(vec![
        (format!("/{commit}/ofl/testgrotesk/METADATA.pb"), meta.as_bytes().to_vec()),
        (format!("/{commit}/ofl/testgrotesk/TestGrotesk-Regular.ttf"), font.clone()),
        (format!("/{commit}/ofl/testgrotesk/OFL.txt"), b"SIL Open Font License".to_vec()),
    ]);
    let env = [("PIXELOX_FONTS_URL", base.as_str())];
    let (code, v) = run_env(d, &["fonts", "get", "Test Grotesk", "-y"], &env);
    assert_eq!(code, 0, "{v}");
    let fam = &v["result"]["families"][0];
    assert_eq!(fam["status"], "downloaded");
    assert_eq!(fam["curated"], false);
    assert_eq!(fam["files"][0]["bytes"], font.len());
    assert!(Path::new(fam["license_file"].as_str().unwrap()).exists());
    let (_, v) = run_env(d, &["fonts", "get", "Test Grotesk", "-y"], &env);
    assert_eq!(v["result"]["families"][0]["status"], "installed");
    let (code, v) = run_env(d, &["fonts", "get", "No Such Family", "-y"], &env);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX4111")));
}

/// `pixelox --update` is `pixelox update`, before the command word only.
#[test]
fn update_flag_is_the_update_command() {
    use pixelox_cli::cli::expand_update_flag;
    let x = |a: &[&str]| expand_update_flag(a.iter().map(|s| s.to_string()).collect());
    assert_eq!(x(&["pixelox", "--update"]), ["pixelox", "update"]);
    assert_eq!(x(&["pixelox", "--json", "--update", "--check"]), ["pixelox", "update", "--json", "--check"]);
    assert_eq!(x(&["pixelox", "--seed", "5", "--update"]), ["pixelox", "update", "--seed", "5"]);
    assert_eq!(x(&["pixelox", "-o", "a.png", "--update"]), ["pixelox", "update", "-o", "a.png"]);
    // after a command word it stays that command's argument
    assert_eq!(x(&["pixelox", "info", "--update"]), ["pixelox", "info", "--update"]);
    assert_eq!(x(&["pixelox", "--", "--update"]), ["pixelox", "--", "--update"]);
    // it runs the update command (offline, so it stops before any network)
    let t = TempDir::new("cli-update-flag");
    let d = t.0.as_path();
    for args in [&["--update", "--check"][..], &["--quiet", "--update"][..]] {
        let (code, v) = run_env(d, args, &[("PIXELOX_OFFLINE", "1")]);
        assert_eq!(
            (code, v["command"].as_str(), v["error"]["code"].as_str()),
            (1, Some("update"), Some("PX1504")),
            "{v}"
        );
    }
    let (code, v) = run_in(d, &["info", "--update"]);
    assert_eq!((code, v["error"]["code"].as_str()), (2, Some("PX1002")), "{v}");
}
