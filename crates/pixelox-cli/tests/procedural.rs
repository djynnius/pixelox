//! End-to-end tests of the procedural, distortion, grain, warp-preset, and liquid commands
//! (ADR 0016): envelopes, live layers, smart filters, replay, and thread independence.

use pixelox_core::testing::TempDir;
use serde_json::Value;
use std::path::Path;
use std::process::Command;

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

fn layer<'a>(list: &'a Value, name: &str) -> &'a Value {
    list["result"]["layers"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["name"] == name)
        .unwrap_or_else(|| panic!("no layer {name}"))
}

fn replays(d: &Path, out: &str) {
    let v = ok(d, &["replay", &format!("{out}.pixelox.json"), "-o", "again.png", "--overwrite"]);
    assert_eq!(v["result"]["identical"], true, "{out} did not replay bit for bit");
}

/// A small document with a photo-like placed layer and a raster layer.
fn setup(d: &Path) {
    ok(d, &["new", "-o", "src.pxo", "--size", "120x90", "--fill", "#336699"]);
    ok(
        d,
        &["shape", "ellipse", "src.pxo", "--in-place", "--rect", "20,10,70x60", "--fill", "#ffcc00", "--name", "Sun"],
    );
    ok(d, &["export", "src.pxo", "-o", "photo.png"]);
    ok(d, &["new", "-o", "doc.pxo", "--size", "160x120", "--fill", "#101018"]);
    ok(d, &["comp", "place", "doc.pxo", "--in-place", "photo.png", "--as", "Photo", "--size", "100x", "--at", "30,20"]);
}

#[test]
fn noise_grain_and_distortions_replay() {
    let t = TempDir::new("ws3-noise");
    let d = t.0.as_path();
    setup(d);
    let v = ok(
        d,
        &[
            "fill",
            "noise",
            "doc.pxo",
            "--in-place",
            "--as-layer",
            "--name",
            "Smoke",
            "--kind",
            "smoke",
            "--stops",
            "transparent,#c9d2e0",
            "--blend",
            "screen",
            "--opacity",
            "70",
        ],
    );
    assert_eq!(v["result"]["layer"], "Smoke");
    ok(
        d,
        &[
            "fill",
            "noise",
            "doc.pxo",
            "--in-place",
            "--as-layer",
            "--name",
            "Cracks",
            "--kind",
            "cell-edges",
            "--scale",
            "10%",
            "--blend",
            "multiply",
            "--opacity",
            "30",
        ],
    );
    let v = ok(d, &["filter", "grain", "doc.pxo", "--in-place", "--as-layer", "--amount", "30"]);
    assert_eq!(v["result"]["adjustment"]["op"], "grain");
    let list = ok(d, &["layer", "list", "doc.pxo"]);
    assert_eq!(layer(&list, "Smoke")["blend"], "screen");
    assert_eq!(layer(&list, "Grain")["kind"], "adjustment");
    // distortions on a placed layer are smart filters: it stays placed
    for args in [
        vec!["filter", "displace", "doc.pxo", "--in-place", "--layer", "Photo", "--amount", "3", "--scale", "8"],
        vec!["filter", "glitch", "doc.pxo", "--in-place", "--layer", "Photo", "--split", "3"],
        vec!["filter", "distort", "doc.pxo", "--in-place", "--layer", "Photo", "--kind", "twirl", "--angle", "120"],
    ] {
        let v = ok(d, &args);
        assert!(v["result"]["smart_filter"].as_str().unwrap().starts_with("filter."), "{}", v["result"]);
    }
    let list = ok(d, &["layer", "list", "doc.pxo"]);
    assert_eq!(layer(&list, "Photo")["kind"], "placed");
    assert_eq!(layer(&list, "Photo")["smart_filters"].as_array().unwrap().len(), 3);
    // a map layer cannot drive a smart filter
    let (code, v) =
        run_in(d, &["filter", "displace", "doc.pxo", "--in-place", "--layer", "Photo", "--map-layer", "Smoke"]);
    assert_eq!((code, v["error"]["code"].as_str().unwrap()), (2, "PX3342"));
    ok(d, &["export", "doc.pxo", "-o", "out.png"]);
    replays(d, "out.png");
    let v = ok(d, &["verify", "out.png.pixelox.json"]);
    assert_eq!(v["ok"], true);
}

#[test]
fn warp_presets_on_raster_and_live_on_placed() {
    let t = TempDir::new("ws3-warp");
    let d = t.0.as_path();
    setup(d);
    ok(d, &["layer", "add", "doc.pxo", "--in-place", "--name", "Bar"]);
    ok(d, &["select", "rect", "doc.pxo", "--in-place", "--rect", "20,50,120x20"]);
    ok(d, &["fill", "solid", "doc.pxo", "--in-place", "--layer", "Bar", "--color", "#ff0000", "--mask", "@sel"]);
    let before = ok(d, &["layer", "list", "doc.pxo"]);
    let v = ok(d, &["transform", "warp", "doc.pxo", "--in-place", "--layer", "Bar", "--preset", "arc", "--bend", "60"]);
    assert_eq!(v["result"]["warp"]["preset"], "arc");
    let after = ok(d, &["layer", "list", "doc.pxo"]);
    assert_ne!(layer(&before, "Bar")["bounds"], layer(&after, "Bar")["bounds"], "the arc grows the bounds");
    // on a placed layer: recorded as a live smart filter
    let v = ok(
        d,
        &[
            "transform",
            "warp",
            "doc.pxo",
            "--in-place",
            "--layer",
            "Photo",
            "--preset",
            "flag",
            "--bend",
            "40",
            "--h-distort",
            "30",
        ],
    );
    assert_eq!(v["result"]["smart_filter"], "transform.warp");
    let list = ok(d, &["layer", "list", "doc.pxo"]);
    assert_eq!(layer(&list, "Photo")["kind"], "placed");
    // errors: both or neither of --preset and --points
    let (code, v) = run_in(d, &["transform", "warp", "doc.pxo", "--in-place", "--layer", "Bar"]);
    assert_eq!((code, v["error"]["code"].as_str().unwrap()), (2, "PX1003"));
    let (code, v) = run_in(
        d,
        &["transform", "warp", "doc.pxo", "--in-place", "--layer", "Bar", "--preset", "arc", "--points", "w.json"],
    );
    assert_eq!((code, v["error"]["code"].as_str().unwrap()), (2, "PX1005"));
    ok(d, &["export", "doc.pxo", "-o", "out.png"]);
    replays(d, "out.png");
}

#[test]
fn liquids_and_grain_style_are_layers_that_replay() {
    let t = TempDir::new("ws3-liquid");
    let d = t.0.as_path();
    setup(d);
    let v = ok(d, &["render", "splash", "doc.pxo", "--in-place", "--part", "back", "--at", "50%,85%", "--size", "90%"]);
    assert_eq!(v["result"]["layer"], "Splash");
    assert!(!v["result"]["bounds"].is_null());
    ok(d, &["render", "splash", "doc.pxo", "--in-place", "--kind", "sheet", "--liquid", "milk", "--name", "Milk"]);
    let (code, v) = run_in(d, &["render", "splash", "doc.pxo", "--in-place", "--kind", "sheet", "--part", "front"]);
    assert_eq!((code, v["error"]["code"].as_str().unwrap()), (2, "PX1005"));
    let v = ok(d, &["render", "droplets", "doc.pxo", "--in-place", "--layer", "Photo", "--size", "4"]);
    assert_eq!(v["result"]["clipped"], true);
    assert!(v["result"]["beads"].as_u64().unwrap() > 5, "{}", v["result"]);
    // a grain style on a live shape layer keeps it live
    ok(d, &["shape", "rect", "doc.pxo", "--in-place", "--rect", "10,10,40x30", "--fill", "#ffffff", "--name", "Card"]);
    ok(d, &["fx", "grain", "doc.pxo", "--in-place", "--layer", "Card", "--amount", "60"]);
    let list = ok(d, &["layer", "list", "doc.pxo"]);
    assert_eq!(layer(&list, "Card")["kind"], "shape");
    assert_eq!(layer(&list, "Droplets")["clip"], true);
    ok(d, &["render", "lens-flare", "doc.pxo", "--in-place", "--lens", "movie", "--color", "#ff9a3c"]);
    ok(d, &["export", "doc.pxo", "-o", "out.png"]);
    replays(d, "out.png");
}

#[test]
fn outputs_do_not_depend_on_thread_count() {
    let t = TempDir::new("ws3-threads");
    let d = t.0.as_path();
    setup(d);
    let mut hashes = Vec::new();
    for threads in ["1", "4"] {
        let out = format!("t{threads}.png");
        ok(
            d,
            &[
                "fill",
                "noise",
                "doc.pxo",
                "-o",
                "n.pxo",
                "--overwrite",
                "--as-layer",
                "--kind",
                "smoke",
                "--threads",
                threads,
            ],
        );
        ok(d, &["render", "splash", "n.pxo", "--in-place", "--threads", threads]);
        ok(d, &["filter", "grain", "n.pxo", "--in-place", "--as-layer", "--threads", threads]);
        let v = ok(d, &["export", "n.pxo", "-o", &out, "--threads", threads]);
        hashes.push(v["outputs"][0]["sha256"].as_str().unwrap().to_string());
    }
    assert_eq!(hashes[0], hashes[1]);
}
