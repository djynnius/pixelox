//! End-to-end tests of the vector primitives (`shape icon`, `shape qr`, `shape barcode`)
//! and the cast styles (`fx long-shadow`, `fx extrude`, `fx reflection`): envelopes and
//! exit codes, replay bit for bit, and the same bytes at every thread count.

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
    v
}

/// A document using every new command, built with `threads` worker threads.
fn build(dir: &Path, doc: &str, threads: &str) {
    let t = ["--threads", threads];
    let step = |args: &[&str]| {
        let mut a: Vec<&str> = args.to_vec();
        a.extend_from_slice(&t);
        ok(dir, &a)
    };
    step(&["new", "-o", doc, "--size", "360x300", "--fill", "#19b5a5"]);
    step(&["shape", "icon", doc, "calendar", "--in-place", "--size", "48", "--at", "12,12", "--color", "white"]);
    step(&[
        "shape",
        "icon",
        doc,
        "heart",
        "--in-place",
        "--size",
        "36",
        "--at",
        "70,18",
        "--fill",
        "#e11d48",
        "--rotation",
        "-12",
    ]);
    step(&[
        "shape",
        "qr",
        doc,
        "--in-place",
        "--data",
        "https://example.com/t",
        "--size",
        "120",
        "--style",
        "dots",
        "--background-color",
        "white",
        "--radius",
        "10",
        "--logo-gap",
        "20",
        "--anchor",
        "br",
        "--at",
        "-8,-8",
    ]);
    step(&[
        "shape",
        "barcode",
        doc,
        "--in-place",
        "--type",
        "ean8",
        "--data",
        "9638507",
        "--width",
        "130",
        "--at",
        "8,190",
    ]);
    step(&[
        "text",
        "add",
        doc,
        "--in-place",
        "--name",
        "Title",
        "--content",
        "GO",
        "--font",
        "sans-serif",
        "--size",
        "90px",
        "--color",
        "#ffd23c",
        "--at",
        "150,70",
        "--anchor",
        "c",
    ]);
    step(&["fx", "extrude", doc, "--in-place", "--layer", "Title", "--depth", "14", "--shade", "30", "--outline", "2"]);
    step(&["fx", "long-shadow", doc, "--in-place", "--layer", "Title", "--fade", "60"]);
    step(&["fx", "reflection", doc, "--in-place", "--layer", "Title", "--gap", "4", "--blur", "6"]);
    step(&[
        "shape",
        "rect",
        doc,
        "--in-place",
        "--name",
        "Pill",
        "--rect",
        "190,20,150x40",
        "--radius",
        "20",
        "--fill",
        "white",
        "--stroke",
        "#111827",
        "--stroke-width",
        "4",
    ]);
    step(&["fx", "overlay", doc, "--in-place", "--layer", "Pill", "--stops", "#ffcf56@0,#e4572e@1", "--angle", "0"]);
}

#[test]
fn primitives_and_cast_styles_replay_and_ignore_thread_counts() {
    let t = TempDir::new("cli-prims");
    let mut outs = Vec::new();
    for (i, threads) in ["1", "4"].iter().enumerate() {
        let doc = format!("d{i}.pxo");
        let png = format!("d{i}.png");
        build(&t.0, &doc, threads);
        ok(&t.0, &["export", &doc, "-o", &png, "--threads", threads]);
        outs.push(std::fs::read(t.path(&png)).unwrap());
    }
    assert!(outs[0] == outs[1], "PNG bytes differ across thread counts");
    let v = ok(&t.0, &["replay", "d0.png.pixelox.json", "-o", "again.png", "--strict"]);
    assert_eq!(v["result"]["identical"], true);
    ok(&t.0, &["verify", "d0.png.pixelox.json", "--full"]);
    let l = ok(&t.0, &["fx", "list", "d0.pxo", "--layer", "Title"]);
    assert_eq!(l["result"]["count"], 3);
}

#[test]
fn primitive_envelopes_and_exit_codes() {
    let t = TempDir::new("cli-prims-env");
    ok(&t.0, &["new", "-o", "d.pxo", "--size", "200x200", "--fill", "#101010"]);
    // a QR code without a plate on a dark ground warns but succeeds
    let v = ok(&t.0, &["shape", "qr", "d.pxo", "--in-place", "--data", "HELLO", "--size", "150"]);
    assert_eq!(v["warnings"][0]["code"], "PXW6010");
    assert_eq!(v["result"]["ec"], "m");
    // usage errors exit 2 with a hint
    for (args, code) in [
        (vec!["shape", "icon", "d.pxo", "calender", "--in-place"], "PX6030"),
        (vec!["shape", "barcode", "d.pxo", "--in-place", "--type", "ean13", "--data", "4006381333932"], "PX6032"),
        (vec!["shape", "barcode", "d.pxo", "--in-place", "--type", "code39", "--data", "lower"], "PX6031"),
        (
            vec![
                "shape",
                "qr",
                "d.pxo",
                "--in-place",
                "--data",
                "https://example.com",
                "--logo-gap",
                "35",
                "--ec",
                "l",
            ],
            "PX6034",
        ),
    ] {
        let (exit, v) = run_in(&t.0, &args);
        assert_eq!((exit, v["error"]["code"].as_str()), (2, Some(code)), "{args:?}");
        assert!(v["error"]["hint"].as_str().is_some_and(|h| !h.is_empty()), "{args:?}");
    }
    // the icon list and search
    let v = ok(&t.0, &["shape", "icons", "--search", "ticket", "--limit", "3"]);
    assert_eq!(v["result"]["icons"][0]["name"], "ticket");
    let v = ok(&t.0, &["shape", "icons"]);
    assert!(v["result"]["count"].as_u64().unwrap() > 2000);
}
