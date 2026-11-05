//! Behavior tests: the op contract (summaries, help, examples), every command's effect,
//! identity settings, masks, transparent / 1×1 / locked / non-raster layers, adjustment
//! layers rendered through the installed hook, and the analyzers.

use crate::{install, ops};
use pixelox_core::ctx::{Globals, ImageInfo, ImageIo, NoIo, NoProgress, OpenOptions};
use pixelox_core::envelope::Warning;
use pixelox_core::op::{json_to_pairs, Runner};
use pixelox_core::prelude::*;
use pixelox_core::testing::{self, doc_from, max_diff, mean, raster, run, TempDir};
use std::path::{Path, PathBuf};

fn spec(name: &str) -> OpSpec {
    testing::find(&ops(), name)
}

fn grad() -> Image {
    testing::gradient(32, 16)
}

/// Run a doc op on a one-layer document; returns the new pixels and the op's result.
fn apply(name: &str, pairs: &[(&str, &str)], img: Image) -> (Image, Value) {
    let mut d = doc_from(img);
    let v = run(&spec(name), pairs, &mut d).unwrap_or_else(|e| panic!("{name} {pairs:?}: {e}"));
    (raster(&d, "Background").clone(), v)
}

fn err(name: &str, pairs: &[(&str, &str)], img: Image) -> String {
    let mut d = doc_from(img);
    run(&spec(name), pairs, &mut d).expect_err("expected an error").code
}

/// Run a doc op with a context we can inspect (resolved params, warnings).
fn run_ctx(name: &str, pairs: &[(&str, &str)], doc: &mut Document) -> (Value, Map<String, Value>, Vec<String>) {
    let s = spec(name);
    let a = testing::args(&s, pairs).unwrap();
    let mut ctx = OpCtx::new(s.name, vec![], Globals::default(), &NoIo, &NoProgress);
    let v = match s.runner {
        Runner::Doc(f) => f(&a, doc, &mut ctx).unwrap(),
        _ => panic!("not a doc op"),
    };
    (v, ctx.resolved.clone(), ctx.warnings.iter().map(|w| w.code.clone()).collect())
}

fn px(img: &Image, x: u32, y: u32) -> [f32; 4] {
    img.get(x, y)
}

fn solid(r: f32, g: f32, b: f32) -> Image {
    Image::filled(4, 4, Rgba::new(r, g, b, 1.0))
}

fn close(a: f32, b: f32, tol: f32) -> bool {
    (a - b).abs() <= tol
}

/// A 2-point LUT that swaps red and blue.
fn swap_cube() -> String {
    let mut s = String::from("TITLE \"swap\"\nLUT_3D_SIZE 2\n");
    for b in 0..2 {
        for g in 0..2 {
            for r in 0..2 {
                s.push_str(&format!("{b} {g} {r}\n"));
            }
        }
    }
    s
}

/// One non-identity setting per adjust command.
fn cases() -> Vec<(&'static str, Vec<(&'static str, String)>)> {
    let s = |v: &str| v.to_string();
    vec![
        ("adjust brightness-contrast", vec![("brightness", s("30")), ("contrast", s("40"))]),
        ("adjust levels", vec![("in-black", s("20")), ("in-white", s("220")), ("gamma", s("1.3"))]),
        ("adjust curves", vec![("points", s("0,0 64,90 192,200 255,255"))]),
        ("adjust exposure", vec![("ev", s("0.8")), ("offset", s("-0.01")), ("gamma", s("1.1"))]),
        ("adjust shadows-highlights", vec![("shadows", s("60")), ("highlights", s("40")), ("radius", s("4"))]),
        ("adjust tone-map", vec![("strength", s("70"))]),
        ("adjust hsl", vec![("hue", s("40")), ("saturation", s("30")), ("lightness", s("-10"))]),
        (
            "adjust color-balance",
            vec![("shadows", s("-20,0,20")), ("midtones", s("15,-5,0")), ("highlights", s("0,10,-10"))],
        ),
        ("adjust vibrance", vec![("vibrance", s("60")), ("saturation", s("10"))]),
        ("adjust white-balance", vec![("auto", s("true"))]),
        ("adjust selective-color", vec![("target", s("neutrals")), ("c", s("-30")), ("m", s("20")), ("k", s("10"))]),
        ("adjust channel-mixer", vec![("out", s("r")), ("r", s("60")), ("g", s("40"))]),
        ("adjust gradient-map", vec![("stops", s("#102030@0,#ff8800@0.5,#ffffff@1")), ("interp", s("oklab"))]),
        ("adjust lut", vec![("cube-text", swap_cube())]),
        ("adjust bw", vec![("preset", s("red-filter"))]),
        ("adjust invert", vec![]),
        ("adjust equalize", vec![]),
        ("adjust posterize", vec![("levels", s("3"))]),
        ("adjust threshold", vec![("level", s("100"))]),
        ("adjust auto", vec![("color", s("true"))]),
    ]
}

fn pairs_of<'a>(v: &'a [(&'static str, String)]) -> Vec<(&'static str, &'a str)> {
    v.iter().map(|(k, val)| (*k, val.as_str())).collect()
}

// ---------------------------------------------------------------------------------------
// contract

const GLOBALS: &[&str] = &[
    "output",
    "format",
    "json",
    "dry-run",
    "overwrite",
    "in-place",
    "seed",
    "yes",
    "quiet",
    "verbose",
    "threads",
    "mem-budget",
    "progress",
    "no-sidecar",
    "background",
    "config",
    "redact-paths",
    "limit-pixels",
];
const GLOBAL_FLAGS: &[&str] =
    &["json", "dry-run", "overwrite", "in-place", "yes", "quiet", "verbose", "no-sidecar", "background"];

/// Split a command line on spaces outside single quotes.
fn shell_split(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    let mut any = false;
    for c in s.chars() {
        match c {
            '\'' => {
                quoted = !quoted;
                any = true;
            }
            ' ' if !quoted => {
                if any {
                    out.push(std::mem::take(&mut cur));
                    any = false;
                }
            }
            _ => {
                cur.push(c);
                any = true;
            }
        }
    }
    if any {
        out.push(cur);
    }
    out
}

#[test]
fn every_command_is_defined() {
    let names: Vec<&str> = ops().iter().map(|o| o.name).collect();
    for (op, _) in crate::ADJUSTMENTS {
        assert!(names.contains(&format!("adjust {op}").as_str()), "missing adjust {op}");
    }
    for a in ["analyze histogram", "analyze stats", "analyze colors", "analyze diff"] {
        assert!(names.contains(&a), "missing {a}");
    }
    assert_eq!(names.len(), 24);
    assert!(!names.contains(&"analyze exif"));
}

#[test]
fn op_contract_and_examples_parse() {
    for op in ops() {
        let n = op.summary.chars().count();
        assert!(n <= 60 && n > 0, "{}: summary is {n} chars", op.name);
        assert!(!op.summary.ends_with('.'), "{}: summary ends with a period", op.name);
        assert!(!op.about.is_empty(), "{}: no about", op.name);
        assert!(op.examples.len() >= 3, "{}: fewer than 3 examples", op.name);
        for p in &op.params {
            assert!(!p.help.is_empty(), "{} --{}: no help", op.name, p.name);
            assert!(!GLOBALS.contains(&p.name), "{} --{} shadows a global flag", op.name, p.name);
        }
        if op.name.starts_with("adjust ") {
            for p in ["layer", "mask", "invert-mask", "as-layer", "name"] {
                assert!(op.find_param(p).is_some(), "{} lacks --{p}", op.name);
            }
        }
        for ex in &op.examples {
            let toks = shell_split(ex.cmd);
            let words: Vec<&str> = op.name.split(' ').collect();
            assert_eq!(toks[0], "pixelox", "{}", ex.cmd);
            assert_eq!(&toks[1..1 + words.len()], words.as_slice(), "{}", ex.cmd);
            let mut pairs: Vec<(String, String)> = Vec::new();
            let mut inputs = 0;
            let mut has_output = false;
            let mut i = 1 + words.len();
            while i < toks.len() {
                let t = &toks[i];
                if t == "-o" {
                    has_output = true;
                    i += 2;
                    continue;
                }
                if let Some(flag) = t.strip_prefix("--") {
                    let (name, inline) = match flag.split_once('=') {
                        Some((n, v)) => (n, Some(v.to_string())),
                        None => (flag, None),
                    };
                    if name == "in-place" {
                        has_output = true;
                    }
                    if GLOBALS.contains(&name) {
                        i += if GLOBAL_FLAGS.contains(&name) || inline.is_some() { 1 } else { 2 };
                        continue;
                    }
                    let p = op.find_param(name).unwrap_or_else(|| panic!("{}: unknown flag --{name}", ex.cmd));
                    let value = match (inline, p.kind) {
                        (Some(v), _) => v,
                        (None, ParamKind::Flag) => "true".into(),
                        (None, _) => {
                            i += 1;
                            toks.get(i).cloned().unwrap_or_else(|| panic!("{}: --{name} needs a value", ex.cmd))
                        }
                    };
                    pairs.push((name.to_string(), value));
                } else {
                    assert!(!t.starts_with('-'), "{}: stray token {t}", ex.cmd);
                    inputs += 1;
                }
                i += 1;
            }
            assert!(
                inputs >= op.inputs.min && op.inputs.max.is_none_or(|m| inputs <= m),
                "{}: {inputs} inputs",
                ex.cmd
            );
            if op.is_doc() && op.mode == DocMode::Mutate {
                assert!(has_output, "{}: a mutating example needs -o or --in-place", ex.cmd);
            }
            Args::build(&op, pairs).unwrap_or_else(|e| panic!("{}: {e}", ex.cmd));
        }
    }
}

use pixelox_core::op::DocMode;

// ---------------------------------------------------------------------------------------
// identity settings

#[test]
fn identity_settings_change_nothing() {
    let cube = crate::lut::identity_cube(3);
    let ids: Vec<(&str, Vec<(&str, &str)>)> = vec![
        ("adjust brightness-contrast", vec![]),
        ("adjust levels", vec![]),
        ("adjust curves", vec![("points", "0,0 255,255")]),
        ("adjust exposure", vec![]),
        ("adjust shadows-highlights", vec![("shadows", "0")]),
        ("adjust tone-map", vec![("strength", "0")]),
        ("adjust hsl", vec![]),
        ("adjust color-balance", vec![]),
        ("adjust vibrance", vec![]),
        ("adjust white-balance", vec![("temp", "0"), ("tint", "0")]),
        ("adjust selective-color", vec![]),
        ("adjust channel-mixer", vec![]),
        ("adjust channel-mixer", vec![("out", "b")]),
        ("adjust lut", vec![("cube-text", cube.as_str())]),
        ("adjust lut", vec![("cube-text", cube.as_str()), ("interp", "trilinear")]),
        (
            "adjust lut",
            vec![
                ("cube-text", "LUT_3D_SIZE 2\n0 0 0\n1 1 1\n1 1 1\n1 1 1\n1 1 1\n1 1 1\n1 1 1\n1 1 1\n"),
                ("strength", "0"),
            ],
        ),
        ("adjust gradient-map", vec![("stops", "#000@0,#fff@1")]),
    ];
    for (name, pairs) in ids {
        // gradient maps are only an identity on neutrals
        let img = if name == "adjust gradient-map" {
            Image::from_fn(16, 4, |x, _| [x as f32 / 15.0, x as f32 / 15.0, x as f32 / 15.0, 1.0])
        } else {
            grad()
        };
        let (out, _) = apply(name, &pairs, img.clone());
        let d = max_diff(&out, &img);
        assert!(d < 1e-4, "{name} {pairs:?} changed pixels by {d}");
    }
    // double invert is the identity
    let (once, _) = apply("adjust invert", &[], grad());
    let (twice, _) = apply("adjust invert", &[], once);
    assert!(max_diff(&twice, &grad()) < 1e-6);
}

// ---------------------------------------------------------------------------------------
// behavior per command

#[test]
fn brightness_contrast_behaves() {
    let ramp = Image::from_fn(64, 1, |x, _| {
        let v = x as f32 / 63.0;
        [v, v, v, 1.0]
    });
    let (b, _) = apply("adjust brightness-contrast", &[("brightness", "50")], ramp.clone());
    assert!(mean(&b, 0) > mean(&ramp, 0) + 0.05);
    assert_eq!(px(&b, 0, 0)[0], 0.0);
    assert!(close(px(&b, 63, 0)[0], 1.0, 1e-6));
    let (c, _) = apply("adjust brightness-contrast", &[("contrast", "60")], ramp.clone());
    assert!(px(&c, 10, 0)[0] < px(&ramp, 10, 0)[0] && px(&c, 53, 0)[0] > px(&ramp, 53, 0)[0]);
    let (lo, _) = apply("adjust brightness-contrast", &[("contrast", "-100")], ramp.clone());
    assert!(px(&lo, 0, 0)[0] > 0.3 && px(&lo, 63, 0)[0] < 0.7);
    assert_eq!(err("adjust brightness-contrast", &[("brightness", "101")], grad()), "PX1013");
}

#[test]
fn levels_behave() {
    let img = Image::from_fn(3, 1, |x, _| {
        let v = [0.25, 0.5, 0.75][x as usize];
        [v, v, v, 1.0]
    });
    let (o, _) = apply("adjust levels", &[("in-black", "63.75"), ("in-white", "191.25")], img.clone());
    assert!(
        close(px(&o, 0, 0)[0], 0.0, 1e-5) && close(px(&o, 1, 0)[0], 0.5, 1e-5) && close(px(&o, 2, 0)[0], 1.0, 1e-5)
    );
    let (g, _) = apply("adjust levels", &[("gamma", "2")], img.clone());
    assert!(close(px(&g, 1, 0)[0], 0.5f32.powf(0.5), 1e-5));
    let (b, _) = apply("adjust levels", &[("channel", "b"), ("out-white", "127.5")], img.clone());
    assert!(close(px(&b, 2, 0)[2], 0.375, 1e-5) && close(px(&b, 2, 0)[0], 0.75, 1e-6));
    let (inv, _) = apply("adjust levels", &[("out-black", "255"), ("out-white", "0")], img.clone());
    assert!(close(px(&inv, 0, 0)[1], 0.75, 1e-5));
    assert_eq!(err("adjust levels", &[("in-black", "200"), ("in-white", "100")], grad()), "PX1010");
}

#[test]
fn curves_behave_and_validate() {
    let gray = solid(0.5, 0.5, 0.5);
    let (o, v) = apply("adjust curves", &[("points", "0,0 127.5,191.25 255,255")], gray.clone());
    assert!(close(px(&o, 0, 0)[0], 0.75, 1e-4), "{:?}", px(&o, 0, 0));
    assert_eq!(v["layer"], "Background");
    let (r, _) = apply("adjust curves", &[("points", "0,0 127.5,191.25 255,255"), ("channel", "r")], gray.clone());
    assert!(close(px(&r, 0, 0)[0], 0.75, 1e-4) && px(&r, 0, 0)[1] == 0.5);
    assert_eq!(err("adjust curves", &[("points", "0,0 128,100 128,200 255,255")], gray.clone()), "PX3310");
    assert_eq!(err("adjust curves", &[("points", "0,0 200,100 100,200")], gray.clone()), "PX3310");
    assert_eq!(err("adjust curves", &[], gray.clone()), "PX1003");
    assert_eq!(err("adjust curves", &[("points", "0,0 255,255"), ("curve", "x.json")], gray.clone()), "PX1005");
    // a curve file gives the same result as the same points
    let dir = TempDir::new("adjust-curve");
    let f = dir.path("film.json");
    std::fs::write(&f, r#"{"points": [[0,0],[127.5,191.25],[255,255]]}"#).unwrap();
    let (fo, _) = apply("adjust curves", &[("curve", f.to_str().unwrap())], gray.clone());
    assert!(max_diff(&fo, &o) < 1e-6);
    std::fs::write(&f, "[[0,0],[100,10],[90,20]]").unwrap();
    assert_eq!(err("adjust curves", &[("curve", f.to_str().unwrap())], gray), "PX3310");
}

#[test]
fn exposure_works_in_linear_light() {
    let (o, _) = apply("adjust exposure", &[("ev", "1")], solid(0.2, 0.4, 0.6));
    for (c, v) in [0.2f32, 0.4, 0.6].iter().enumerate() {
        let want = color::linear_to_srgb((color::srgb_to_linear(*v) * 2.0).min(1.0));
        assert!(close(px(&o, 0, 0)[c], want, 1e-5));
    }
    let (d, _) = apply("adjust exposure", &[("offset", "-0.05")], solid(0.2, 0.4, 0.6));
    assert!(px(&d, 0, 0)[0] < 0.2 && px(&d, 0, 0)[0] >= 0.0);
    let (w, _) = apply("adjust exposure", &[("ev", "5")], solid(0.9, 0.9, 0.9));
    assert!(close(px(&w, 0, 0)[0], 1.0, 1e-6));
}

fn two_tone() -> Image {
    Image::from_fn(32, 8, |x, _| if x < 16 { [0.08, 0.08, 0.08, 1.0] } else { [0.92, 0.92, 0.92, 1.0] })
}

#[test]
fn shadows_highlights_lift_and_recover() {
    let (s, _) = apply("adjust shadows-highlights", &[("shadows", "80"), ("radius", "2")], two_tone());
    assert!(px(&s, 2, 2)[0] > 0.12, "shadows lifted: {:?}", px(&s, 2, 2));
    assert!(px(&s, 30, 2)[0] >= 0.919, "highlights barely touched");
    let (h, _) =
        apply("adjust shadows-highlights", &[("shadows", "0"), ("highlights", "80"), ("radius", "2")], two_tone());
    assert!(px(&h, 30, 2)[0] < 0.9 && close(px(&h, 2, 2)[0], 0.08, 0.005));
    let (r0, _) = apply("adjust shadows-highlights", &[("radius", "0")], two_tone());
    assert!(px(&r0, 2, 2)[0] > 0.08);
}

#[test]
fn tone_map_compresses_large_scale_contrast() {
    let img = two_tone();
    let (o, _) = apply("adjust tone-map", &[("strength", "100")], img.clone());
    let (dark, bright) = (px(&o, 2, 4)[0], px(&o, 29, 4)[0]);
    assert!(dark > 0.08 && bright < 0.92, "dark {dark} bright {bright}");
    // keeps hue: a colored pixel's channel ratios survive
    let (c, _) = apply("adjust tone-map", &[], Image::filled(8, 8, Rgba::new(0.2, 0.1, 0.05, 1.0)));
    let p = px(&c, 4, 4);
    assert!(p[0] > p[1] && p[1] > p[2]);
}

#[test]
fn hsl_behaves() {
    let red = solid(1.0, 0.0, 0.0);
    let (g, _) = apply("adjust hsl", &[("hue", "120")], red.clone());
    assert!(close(px(&g, 0, 0)[1], 1.0, 1e-4) && close(px(&g, 0, 0)[0], 0.0, 1e-4));
    let (gray, _) = apply("adjust hsl", &[("saturation", "-100")], solid(0.8, 0.2, 0.4));
    let p = px(&gray, 0, 0);
    assert!(close(p[0], p[1], 1e-5) && close(p[1], p[2], 1e-5));
    let (white, _) = apply("adjust hsl", &[("lightness", "100")], red.clone());
    assert_eq!(&px(&white, 0, 0)[..3], &[1.0, 1.0, 1.0]);
    // a range spares other hues
    let both = Image::from_fn(2, 1, |x, _| if x == 0 { [0.9, 0.1, 0.1, 1.0] } else { [0.1, 0.8, 0.1, 1.0] });
    let (o, _) = apply("adjust hsl", &[("range", "greens"), ("saturation", "-100")], both.clone());
    assert_eq!(px(&o, 0, 0), px(&both, 0, 0));
    let q = px(&o, 1, 0);
    assert!(close(q[0], q[1], 1e-4));
    let (cz, _) =
        apply("adjust hsl", &[("colorize", "true"), ("hue", "-120"), ("saturation", "100")], solid(0.5, 0.5, 0.5));
    let p = px(&cz, 0, 0);
    assert!(p[2] > p[0] + 0.3 && close(p[0], p[1], 1e-4));
    assert_eq!(err("adjust hsl", &[("colorize", "true"), ("range", "reds")], red), "PX1005");
}

#[test]
fn color_balance_behaves() {
    let (m, _) = apply("adjust color-balance", &[("midtones", "50,0,0")], solid(0.5, 0.5, 0.5));
    assert!(px(&m, 0, 0)[0] > 0.6 && px(&m, 0, 0)[1] == 0.5);
    let (s, _) = apply("adjust color-balance", &[("midtones", "50,0,0")], solid(0.02, 0.02, 0.02));
    assert!(px(&s, 0, 0)[0] < 0.05, "midtone shifts spare the shadows");
    let (h, _) = apply("adjust color-balance", &[("highlights", "0,0,-60")], solid(0.9, 0.9, 0.9));
    assert!(px(&h, 0, 0)[2] < 0.6);
    let (p, _) =
        apply("adjust color-balance", &[("midtones", "60,0,0"), ("preserve-luma", "true")], solid(0.5, 0.5, 0.5));
    let q = px(&p, 0, 0);
    let (_, _, l) = color::rgb_to_hsl(q[0], q[1], q[2]);
    assert!(close(l, 0.5, 1e-4) && q[0] > q[1]);
    assert_eq!(err("adjust color-balance", &[("midtones", "1,2")], grad()), "PX1010");
    assert_eq!(err("adjust color-balance", &[("midtones", "1,2,300")], grad()), "PX1013");
}

fn chroma(p: [f32; 4]) -> f32 {
    p[0].max(p[1]).max(p[2]) - p[0].min(p[1]).min(p[2])
}

#[test]
fn vibrance_favors_muted_colors() {
    let img = Image::from_fn(2, 1, |x, _| if x == 0 { [0.5, 0.45, 0.6, 1.0] } else { [0.9, 0.1, 0.5, 1.0] });
    let (o, _) = apply("adjust vibrance", &[("vibrance", "100")], img.clone());
    let gain_muted = chroma(px(&o, 0, 0)) / chroma(px(&img, 0, 0));
    let gain_vivid = chroma(px(&o, 1, 0)) / chroma(px(&img, 1, 0));
    assert!(gain_muted > 1.3 && gain_vivid < gain_muted, "{gain_muted} {gain_vivid}");
    let (pure, _) = apply("adjust vibrance", &[("vibrance", "100"), ("saturation", "100")], solid(1.0, 0.0, 0.0));
    assert_eq!(&px(&pure, 0, 0)[..3], &[1.0, 0.0, 0.0]);
    let (d, _) = apply("adjust vibrance", &[("vibrance", "-100")], img.clone());
    assert!(chroma(px(&d, 0, 0)) < 0.05);
}

fn cast_image() -> Image {
    Image::from_fn(16, 16, |x, y| {
        let v = 0.2 + 0.6 * (x + y) as f32 / 30.0;
        [(v * 1.15).min(1.0), v, v * 0.75, 1.0]
    })
}

#[test]
fn white_balance_auto_point_and_manual() {
    let (w, _) = apply("adjust white-balance", &[("temp", "50")], solid(0.5, 0.5, 0.5));
    assert!(px(&w, 0, 0)[0] > px(&w, 0, 0)[2]);
    let (t, _) = apply("adjust white-balance", &[("tint", "50")], solid(0.5, 0.5, 0.5));
    assert!(px(&t, 0, 0)[1] < px(&t, 0, 0)[0]);

    let img = cast_image();
    let ratio = |i: &Image| mean(i, 0) / mean(i, 2);
    let mut doc = doc_from(img.clone());
    let (v, resolved, _) = run_ctx("adjust white-balance", &[("auto", "true")], &mut doc);
    let out = raster(&doc, "Background").clone();
    assert!((ratio(&out) - 1.0).abs() < (ratio(&img) - 1.0).abs() / 3.0, "{} vs {}", ratio(&out), ratio(&img));
    assert_eq!(v["white_balance"]["method"], "auto");
    assert!(v["white_balance"]["temp"].as_f64().unwrap() < 0.0);
    assert_eq!(resolved["auto"], "false");
    assert!(resolved["from_point"].is_null());
    // zero flags mean --auto
    let (z, _) = apply("adjust white-balance", &[], img.clone());
    assert!(max_diff(&z, &out) < 1e-6);
    // replay: the recorded params reproduce the result without detecting again
    let s = spec("adjust white-balance");
    let mut params = testing::args(&s, &[("auto", "true")]).unwrap().to_json();
    for (k, val) in resolved {
        params.insert(k, val);
    }
    let a = Args::build(&s, json_to_pairs(&Value::Object(params)).unwrap()).unwrap();
    let mut d2 = doc_from(img.clone());
    let mut ctx = OpCtx::new(s.name, vec![], Globals::default(), &NoIo, &NoProgress);
    let Runner::Doc(f) = s.runner else { panic!() };
    let v2 = f(&a, &mut d2, &mut ctx).unwrap();
    assert!(v2.get("white_balance").is_none(), "replay must not re-detect");
    assert!(max_diff(raster(&d2, "Background"), &out) < 1e-6);
    // gray point: the sampled pixel becomes neutral
    let (gp, gv) = apply("adjust white-balance", &[("from-point", "8,8")], img.clone());
    let p = px(&gp, 8, 8);
    assert!(close(p[0], p[2], 0.01) && close(p[0], p[1], 0.01), "{p:?}");
    assert_eq!(gv["white_balance"]["method"], "point");
    assert_eq!(err("adjust white-balance", &[("from-point", "100,8")], img.clone()), "PX1010");
    assert_eq!(err("adjust white-balance", &[("auto", "true"), ("temp", "10")], img.clone()), "PX1005");
    assert_eq!(err("adjust white-balance", &[("auto", "true"), ("from-point", "1,1")], img), "PX1005");
}

#[test]
fn white_balance_clamps_extreme_casts() {
    for pairs in [vec![("auto", "true")], vec![("from-point", "1,1")]] {
        let mut doc = doc_from(solid(0.9, 0.05, 0.02));
        let (v, resolved, warnings) = run_ctx("adjust white-balance", &pairs, &mut doc);
        assert!(warnings.contains(&"PXW3001".to_string()), "{pairs:?}: {warnings:?}");
        assert_eq!(v["white_balance"]["temp"], -100.0);
        assert_eq!(resolved["temp"], "-100");
    }
}

#[test]
fn selective_color_targets_one_family() {
    let img = Image::from_fn(3, 1, |x, _| match x {
        0 => [0.9, 0.2, 0.1, 1.0],
        1 => [0.1, 0.2, 0.9, 1.0],
        _ => [0.5, 0.5, 0.5, 1.0],
    });
    let (o, _) = apply("adjust selective-color", &[("target", "reds"), ("c", "100")], img.clone());
    assert!(px(&o, 0, 0)[0] < 0.4, "cyan removes red from reds: {:?}", px(&o, 0, 0));
    assert_eq!(px(&o, 1, 0), px(&img, 1, 0));
    assert_eq!(px(&o, 2, 0), px(&img, 2, 0));
    let (n, _) = apply("adjust selective-color", &[("target", "neutrals"), ("k", "50")], img.clone());
    assert!(px(&n, 2, 0)[0] < 0.1);
    let (rel, _) = apply("adjust selective-color", &[("target", "neutrals"), ("k", "50"), ("relative", "true")], img);
    assert!(px(&rel, 2, 0)[0] > px(&n, 2, 0)[0] && px(&rel, 2, 0)[0] < 0.5);
}

#[test]
fn channel_mixer_behaves() {
    let img = solid(0.8, 0.4, 0.2);
    let (m, _) = apply("adjust channel-mixer", &[("monochrome", "true")], img.clone());
    let want = 0.4 * 0.8 + 0.4 * 0.4 + 0.2 * 0.2;
    assert!(px(&m, 0, 0)[..3].iter().all(|v| close(*v, want, 1e-5)));
    let (g, _) = apply("adjust channel-mixer", &[("out", "g"), ("r", "100"), ("g", "0")], img.clone());
    assert!(close(px(&g, 0, 0)[1], 0.8, 1e-6) && px(&g, 0, 0)[0] == 0.8 && px(&g, 0, 0)[2] == 0.2);
    let (k, _) = apply("adjust channel-mixer", &[("out", "b"), ("constant", "50")], img);
    assert!(close(px(&k, 0, 0)[2], 0.7, 1e-6));
}

#[test]
fn gradient_map_follows_luma() {
    let img = Image::from_fn(2, 1, |x, _| if x == 0 { [0.0, 0.0, 0.0, 1.0] } else { [1.0, 1.0, 1.0, 1.0] });
    let (o, _) = apply("adjust gradient-map", &[("stops", "#ff0000@0,#0000ff@1")], img);
    assert_eq!(&px(&o, 0, 0)[..3], &[1.0, 0.0, 0.0]);
    assert_eq!(&px(&o, 1, 0)[..3], &[0.0, 0.0, 1.0]);
    let (h, _) = apply("adjust gradient-map", &[("stops", "#ff000080@0,#ff000080@1")], solid(0.0, 0.0, 1.0));
    assert!(close(px(&h, 0, 0)[0], 0.5, 0.01) && close(px(&h, 0, 0)[2], 0.5, 0.01));
}

#[test]
fn lut_applies_files_and_strength() {
    let img = solid(0.9, 0.5, 0.1);
    let dir = TempDir::new("adjust-lut");
    let f = dir.path("swap.cube");
    std::fs::write(&f, swap_cube()).unwrap();
    let (o, _) = apply("adjust lut", &[("cube", f.to_str().unwrap())], img.clone());
    let p = px(&o, 0, 0);
    assert!(close(p[0], 0.1, 1e-5) && close(p[1], 0.5, 1e-5) && close(p[2], 0.9, 1e-5), "{p:?}");
    let (h, _) = apply("adjust lut", &[("cube", f.to_str().unwrap()), ("strength", "50")], img.clone());
    assert!(close(px(&h, 0, 0)[0], 0.5, 1e-5));
    std::fs::write(&f, "LUT_3D_SIZE 2\n0 0 0\n").unwrap();
    assert_eq!(err("adjust lut", &[("cube", f.to_str().unwrap())], img.clone()), "PX3311");
    assert_eq!(err("adjust lut", &[], img.clone()), "PX1003");
    assert_eq!(err("adjust lut", &[("cube", "missing.cube")], img), "PX1101");
}

#[test]
fn bw_presets_and_weights() {
    let sky = solid(0.2, 0.4, 0.9);
    let (d, _) = apply("adjust bw", &[], sky.clone());
    let p = px(&d, 0, 0);
    assert!(close(p[0], p[1], 1e-6) && close(p[1], p[2], 1e-6));
    let (rf, _) = apply("adjust bw", &[("preset", "red-filter")], sky.clone());
    let (bf, _) = apply("adjust bw", &[("preset", "blue-filter")], sky.clone());
    assert!(px(&rf, 0, 0)[0] < px(&d, 0, 0)[0] && px(&bf, 0, 0)[0] > px(&d, 0, 0)[0]);
    let (custom, _) = apply("adjust bw", &[("b", "100"), ("c", "100")], sky);
    assert!(px(&custom, 0, 0)[0] > px(&d, 0, 0)[0]);
    let (gray, _) = apply("adjust bw", &[("preset", "infrared")], solid(0.3, 0.3, 0.3));
    assert!(close(px(&gray, 0, 0)[0], 0.3, 1e-6), "neutrals stay put");
}

#[test]
fn invert_equalize_posterize_threshold() {
    let (i, _) = apply("adjust invert", &[], solid(0.2, 0.4, 1.0));
    assert!(close(px(&i, 0, 0)[0], 0.8, 1e-6) && px(&i, 0, 0)[2] == 0.0);
    let flat = Image::from_fn(64, 4, |x, _| {
        let v = 0.4 + 0.2 * x as f32 / 63.0;
        [v, v, v, 1.0]
    });
    let (e, _) = apply("adjust equalize", &[], flat);
    assert!(px(&e, 0, 0)[0] < 0.02 && px(&e, 63, 0)[0] > 0.98);
    assert!(close(px(&e, 32, 0)[0], 0.5, 0.05));
    let ramp = Image::from_fn(256, 1, |x, _| {
        let v = x as f32 / 255.0;
        [v, v, v, 1.0]
    });
    let (p2, _) = apply("adjust posterize", &[("levels", "2")], ramp.clone());
    assert!(p2.data.chunks(4).all(|p| p[0] == 0.0 || p[0] == 1.0));
    assert_eq!(px(&p2, 127, 0)[0], 0.0);
    assert_eq!(px(&p2, 128, 0)[0], 1.0);
    let (p4, _) = apply("adjust posterize", &[("levels", "4")], ramp.clone());
    let mut vals: Vec<u8> = p4.to_rgba8().chunks(4).map(|p| p[0]).collect();
    vals.dedup();
    assert_eq!(vals, vec![0, 85, 170, 255]);
    let (t, _) = apply("adjust threshold", &[("level", "128")], ramp);
    assert_eq!(px(&t, 127, 0)[0], 0.0);
    assert_eq!(px(&t, 128, 0)[0], 1.0);
    assert_eq!(err("adjust posterize", &[("levels", "1")], grad()), "PX1013");
}

#[test]
fn auto_corrections() {
    let dull = Image::from_fn(64, 8, |x, _| {
        let v = 0.3 + 0.4 * x as f32 / 63.0;
        [v, v * 0.95, v * 0.85, 1.0]
    });
    let mut doc = doc_from(dull.clone());
    let (v, resolved, _) = run_ctx("adjust auto", &[], &mut doc);
    let out = raster(&doc, "Background");
    // tone: one shared stretch (darkest channel to black, brightest to white) ...
    assert!(px(out, 0, 0)[2] < 0.05 && px(out, 63, 0)[0] > 0.95, "{:?} {:?}", px(out, 0, 0), px(out, 63, 0));
    assert_eq!(v["auto"]["mode"], "tone");
    // ... plus neutral midtones: less cast at mid gray than the plain contrast stretch
    let (ct_only, _) = apply("adjust auto", &[("contrast", "true")], dull.clone());
    let cast = |p: [f32; 4]| (p[0] - p[2]).abs();
    assert!(cast(px(out, 32, 0)) < cast(px(&ct_only, 32, 0)) / 2.0);
    assert!(resolved.contains_key("black") && resolved.contains_key("white") && resolved.contains_key("gamma"));
    let (lv, _) = apply("adjust auto", &[("levels", "true")], dull.clone());
    let p = px(&lv, 63, 0);
    assert!(p[0] > 0.95 && p[2] > 0.95, "per-channel white points: {p:?}");
    let (ct, _) = apply("adjust auto", &[("contrast", "true")], dull.clone());
    assert!(px(&ct, 63, 0)[2] < 0.9, "contrast keeps the color cast");
    assert_eq!(err("adjust auto", &[("levels", "true"), ("color", "true")], dull.clone()), "PX1005");
    // explicit endpoints skip detection
    let (fixed, fv) =
        apply("adjust auto", &[("black", "0,0,0"), ("white", "255,255,255"), ("gamma", "1,1,1")], dull.clone());
    assert!(max_diff(&fixed, &dull) < 1e-6 && fv.get("auto").is_none());
}

// ---------------------------------------------------------------------------------------
// masks, transparency, tiny images, locks, layer kinds, determinism

#[test]
fn masks_limit_every_adjustment() {
    for (name, pairs) in cases() {
        let pairs = pairs_of(&pairs);
        let img = grad();
        let mut d = doc_from(img.clone());
        d.put_mask("left", testing::half_mask(32, 16), false).unwrap();
        let mut p = pairs.clone();
        p.push(("mask", "left"));
        run(&spec(name), &p, &mut d).unwrap_or_else(|e| panic!("{name}: {e}"));
        let out = raster(&d, "Background");
        let right_same = (16..32).all(|x| (0..16).all(|y| px(out, x, y) == px(&img, x, y)));
        assert!(right_same, "{name}: changed pixels outside the mask");
        let left_changed = (0..16).any(|x| (0..16).any(|y| px(out, x, y) != px(&img, x, y)));
        assert!(left_changed, "{name}: did nothing inside the mask");
        // inverted mask: the other half
        let mut d = doc_from(img.clone());
        d.put_mask("left", testing::half_mask(32, 16), false).unwrap();
        p.push(("invert-mask", "true"));
        run(&spec(name), &p, &mut d).unwrap();
        let out = raster(&d, "Background");
        assert!((0..16).all(|x| (0..16).all(|y| px(out, x, y) == px(&img, x, y))), "{name}: --invert-mask");
    }
}

#[test]
fn alpha_is_untouched_and_tiny_images_work() {
    let transparent = Image::from_fn(16, 8, |x, y| [x as f32 / 15.0, y as f32 / 7.0, 0.3, (x as f32 / 15.0) * 0.9]);
    for (name, pairs) in cases() {
        let pairs = pairs_of(&pairs);
        let (o, _) = apply(name, &pairs, transparent.clone());
        let same_alpha = o.data.chunks(4).zip(transparent.data.chunks(4)).all(|(a, b)| a[3] == b[3]);
        assert!(same_alpha, "{name}: alpha changed");
        assert!(o.data.iter().all(|v| (0.0..=1.0).contains(v)), "{name}: values out of range");
        let (one, _) = apply(name, &pairs, Image::filled(1, 1, Rgba::new(0.3, 0.6, 0.2, 1.0)));
        assert!(one.data.iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)), "{name}: 1x1");
        let (clear, _) = apply(name, &pairs, Image::new(3, 3));
        assert!(clear.data.chunks(4).all(|p| p[3] == 0.0), "{name}: fully transparent");
    }
}

#[test]
fn locked_and_non_raster_layers_are_refused() {
    let mut d = doc_from(grad());
    d.layers[0].locked = true;
    assert_eq!(run(&spec("adjust invert"), &[], &mut d).unwrap_err().code, "PX2304");
    let mut d = doc_from(grad());
    let t = d.new_layer("Title", LayerKind::Text { spec: json!({"content": "hi"}) });
    d.insert_above(None, t);
    assert_eq!(run(&spec("adjust invert"), &[("layer", "Title")], &mut d).unwrap_err().code, "PX2303");
    assert_eq!(run(&spec("adjust invert"), &[], &mut d).unwrap_err().code, "PX2302");
    assert_eq!(run(&spec("adjust invert"), &[("name", "X")], &mut d).unwrap_err().code, "PX1005");
    // a layer offset on the canvas: the mask is read in canvas coordinates
    let mut d = testing::doc_layers(8, 1, vec![("Base", Image::filled(8, 1, Rgba::BLACK))]);
    d.layers[0].x = 4;
    d.put_mask("right", Mask::from_fn(8, 1, |x, _| if x >= 6 { 1.0 } else { 0.0 }), false).unwrap();
    run(&spec("adjust invert"), &[("mask", "right")], &mut d).unwrap();
    let out = raster(&d, "Base");
    assert_eq!(px(out, 1, 0)[0], 0.0);
    assert_eq!(px(out, 2, 0)[0], 1.0);
}

#[test]
fn thread_count_never_changes_results() {
    let img = cast_image();
    let pool1 = rayon::ThreadPoolBuilder::new().num_threads(1).build().unwrap();
    let pool4 = rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();
    for (name, pairs) in cases() {
        let pairs = pairs_of(&pairs);
        let a = pool1.install(|| apply(name, &pairs, img.clone()));
        let b = pool4.install(|| apply(name, &pairs, img.clone()));
        assert!(a.0 == b.0 && a.1 == b.1, "{name} depends on the thread count");
    }
    for name in ["analyze stats", "analyze histogram", "analyze colors"] {
        let mut d1 = doc_from(img.clone());
        let mut d4 = doc_from(img.clone());
        let a = pool1.install(|| run(&spec(name), &[], &mut d1).unwrap());
        let b = pool4.install(|| run(&spec(name), &[], &mut d4).unwrap());
        assert_eq!(a, b, "{name} depends on the thread count");
    }
}

// ---------------------------------------------------------------------------------------
// adjustment layers

#[test]
fn adjustment_layers_render_like_the_destructive_op() {
    install();
    for (name, pairs) in cases() {
        let pairs = pairs_of(&pairs);
        let (destructive, _) = apply(name, &pairs, grad());
        let mut d = doc_from(grad());
        let mut p = pairs.clone();
        p.push(("as-layer", "true"));
        let v = run(&spec(name), &p, &mut d).unwrap_or_else(|e| panic!("{name}: {e}"));
        let op = name.strip_prefix("adjust ").unwrap();
        assert_eq!(d.layers.len(), 2, "{name}");
        let top = &d.layers[1];
        match &top.content {
            LayerKind::Adjustment { op: o, params } => {
                assert_eq!(o, op);
                for k in ["layer", "mask", "invert_mask", "as_layer", "name"] {
                    assert!(!params.contains_key(k), "{name}: params keep {k}");
                }
                assert!(params.values().all(|v| v.is_string()), "{name}: params must be strings");
            }
            other => panic!("{name}: made a {} layer", other.name()),
        }
        assert_eq!(v["adjustment"]["op"], op);
        assert_eq!(v["layer"], top.name.as_str());
        // the source pixels are untouched
        assert_eq!(raster(&d, "Background"), &grad());
        let comp = compose::composite(&d).unwrap();
        let diff = max_diff(&comp, &destructive);
        assert!(diff < 1e-4, "{name}: layer renders {diff} away from the destructive op");
    }
}

#[test]
fn adjustment_layers_embed_files_and_detected_values() {
    install();
    let dir = TempDir::new("adjust-layer");
    let curve = dir.path("c.json");
    std::fs::write(&curve, "[[0,0],[128,180],[255,255]]").unwrap();
    let cube = dir.path("swap.cube");
    std::fs::write(&cube, swap_cube()).unwrap();
    let mut d = doc_from(grad());
    run(&spec("adjust curves"), &[("curve", curve.to_str().unwrap()), ("as-layer", "true")], &mut d).unwrap();
    run(&spec("adjust lut"), &[("cube", cube.to_str().unwrap()), ("as-layer", "true"), ("name", "Grade")], &mut d)
        .unwrap();
    run(&spec("adjust white-balance"), &[("as-layer", "true")], &mut d).unwrap();
    let params = |n: &str| match &d.layer(&d.resolve(n).unwrap()).content {
        LayerKind::Adjustment { params, .. } => params.clone(),
        _ => panic!(),
    };
    let c = params("Curves");
    assert_eq!(c["points"], "0,0 128,180 255,255");
    assert!(!c.contains_key("curve"));
    let l = params("Grade");
    assert!(l["cube_text"].as_str().unwrap().contains("LUT_3D_SIZE 2"));
    assert!(!l.contains_key("cube"));
    let w = params("White Balance");
    assert!(w.contains_key("temp") && w.contains_key("tint") && !w.contains_key("auto"));
    // the files can go away: the layers still render
    drop(dir);
    assert!(compose::composite(&d).is_ok());
    // name collisions and the default unique naming
    assert_eq!(
        run(&spec("adjust invert"), &[("as-layer", "true"), ("name", "Grade")], &mut d).unwrap_err().code,
        "PX2305"
    );
    run(&spec("adjust invert"), &[("as-layer", "true")], &mut d).unwrap();
    run(&spec("adjust invert"), &[("as-layer", "true")], &mut d).unwrap();
    assert!(d.resolve("Invert 2").is_ok());
}

#[test]
fn adjustment_layer_masks_position_and_hook() {
    install();
    let mut d = testing::doc_layers(
        32,
        16,
        vec![
            ("Base", grad()),
            ("Top", Image::from_fn(32, 16, |x, _| if x < 8 { [1.0, 0.0, 0.0, 1.0] } else { [0.0; 4] })),
        ],
    );
    d.put_mask("left", testing::half_mask(32, 16), false).unwrap();
    let v = run(&spec("adjust invert"), &[("as-layer", "true"), ("layer", "Base"), ("mask", "left")], &mut d).unwrap();
    assert_eq!(v["above"], "Base");
    assert_eq!(v["masked"], true);
    let names: Vec<&str> = d.layers.iter().map(|l| l.name.as_str()).collect();
    assert_eq!(names, vec!["Base", "Invert", "Top"]);
    let comp = compose::composite(&d).unwrap();
    // inside the mask, under Top's transparency: inverted base
    let g = grad();
    assert!(close(px(&comp, 10, 3)[0], 1.0 - px(&g, 10, 3)[0], 1e-5));
    // outside the mask: the base as is
    assert!(close(px(&comp, 20, 3)[0], px(&g, 20, 3)[0], 1e-5));
    // Top sits above the adjustment and is not inverted
    assert_eq!(&px(&comp, 2, 2)[..3], &[1.0, 0.0, 0.0]);

    // the hook directly, with string params as Args::to_json writes them
    let mut img = grad();
    let mut params = Map::new();
    params.insert("in_black".into(), json!("51"));
    compose::apply_adjustment("levels", &params, &mut img).unwrap();
    assert_eq!(px(&img, 0, 0)[0], 0.0);
    assert!(compose::apply_adjustment("nope", &Map::new(), &mut img).is_err());
    let mut bad = Map::new();
    bad.insert("curve".into(), json!("c.json"));
    assert_eq!(compose::apply_adjustment("curves", &bad, &mut img).unwrap_err().code, "PX3001");
    // unresolved auto values are detected at render time (hand-made layers)
    let mut auto = Map::new();
    auto.insert("auto".into(), json!(true));
    let mut c = cast_image();
    compose::apply_adjustment("white-balance", &auto, &mut c).unwrap();
    assert!((mean(&c, 0) / mean(&c, 2) - 1.0).abs() < 0.1);
}

// ---------------------------------------------------------------------------------------
// analyze

#[test]
fn histogram_counts_and_sparkline() {
    let ramp = Image::from_fn(256, 2, |x, _| {
        let v = x as f32 / 255.0;
        [v, v, v, 1.0]
    });
    let mut d = doc_from(ramp);
    let v = run(&spec("analyze histogram"), &[], &mut d).unwrap();
    assert_eq!(v["pixels"], 512);
    let r = v["counts"]["r"].as_array().unwrap();
    assert_eq!(r.len(), 256);
    assert!(r.iter().all(|c| c == 2));
    assert_eq!(v["sparkline"]["g"].as_str().unwrap().chars().count(), 64);
    assert!((v["mean"]["b"].as_f64().unwrap() - 127.5).abs() < 0.01);
    let v = run(&spec("analyze histogram"), &[("channel", "l"), ("bins", "4")], &mut d).unwrap();
    assert_eq!(v["counts"]["l"], json!([128, 128, 128, 128]));
    // alpha counts every pixel; colors skip transparent ones
    let mut d = testing::doc_layers(
        4,
        1,
        vec![("Logo", Image::from_fn(4, 1, |x, _| [1.0, 0.0, 0.0, if x < 1 { 1.0 } else { 0.0 }]))],
    );
    let a = run(&spec("analyze histogram"), &[("channel", "a"), ("bins", "2"), ("layer", "Logo")], &mut d).unwrap();
    assert_eq!(a["counts"]["a"], json!([3, 1]));
    assert_eq!(a["source"], "Logo");
    let r = run(&spec("analyze histogram"), &[("channel", "r"), ("bins", "2")], &mut d).unwrap();
    assert_eq!(r["pixels"], 1);
    let adj = d.new_layer("Adj", LayerKind::Adjustment { op: "invert".into(), params: Map::new() });
    d.insert_above(None, adj);
    assert_eq!(run(&spec("analyze histogram"), &[("layer", "Adj")], &mut d).unwrap_err().code, "PX2303");
}

#[test]
fn stats_and_eyedropper() {
    let mut d = doc_from(Image::from_fn(4, 2, |x, _| if x < 2 { [0.0, 0.5, 1.0, 1.0] } else { [1.0, 0.5, 0.0, 1.0] }));
    let v = run(&spec("analyze stats"), &[], &mut d).unwrap();
    let r = &v["channels"]["r"];
    assert_eq!(r["min"], 0.0);
    assert_eq!(r["max"], 255.0);
    assert_eq!(r["mean"], 127.5);
    assert_eq!(r["stddev"], 127.5);
    assert_eq!(r["clipped_shadows_percent"], 50.0);
    assert_eq!(v["channels"]["g"]["median"], 127.5);
    assert_eq!(v["channels"]["g"]["stddev"], 0.0);
    assert_eq!(v["alpha"]["coverage_percent"], 100.0);
    assert_eq!(v["pixels"], 8);
    let e = run(&spec("analyze stats"), &[("rect", "3,1,1x1")], &mut d).unwrap();
    assert_eq!(e["hex"], "#ff8000");
    assert_eq!(e["rgba"], json!([255, 128, 0, 255]));
    assert_eq!(e["rect"], json!([3, 1, 1, 1]));
    let q = run(&spec("analyze stats"), &[("rect", "0,0,50%x100%")], &mut d).unwrap();
    assert_eq!(q["mean_hex"], "#0080ff");
    assert!(q.get("hex").is_none());
    assert_eq!(run(&spec("analyze stats"), &[("rect", "10,10,2x2")], &mut d).unwrap_err().code, "PX3320");
    // transparency
    let mut t = doc_from(Image::from_fn(4, 1, |x, _| [0.2, 0.2, 0.2, if x == 0 { 1.0 } else { 0.0 }]));
    let tv = run(&spec("analyze stats"), &[], &mut t).unwrap();
    assert_eq!(tv["alpha"]["coverage_percent"], 25.0);
    assert_eq!(tv["alpha"]["transparent_percent"], 75.0);
    assert_eq!(tv["channels"]["r"]["mean"], 51.0);
    let mut empty = doc_from(Image::new(2, 2));
    let ev = run(&spec("analyze stats"), &[], &mut empty).unwrap();
    assert!(ev["channels"]["r"].is_null() && ev["mean_hex"].is_null());
}

fn two_colors() -> Image {
    Image::from_fn(40, 20, |x, _| if x < 30 { [1.0, 0.0, 0.0, 1.0] } else { [0.0, 0.0, 1.0, 1.0] })
}

#[test]
fn colors_finds_dominant_colors_deterministically() {
    for space in ["oklab", "lab", "srgb"] {
        let mut d = doc_from(two_colors());
        let v = run(&spec("analyze colors"), &[("k", "2"), ("space", space)], &mut d).unwrap();
        let cs = v["colors"].as_array().unwrap();
        assert_eq!(cs.len(), 2, "{space}");
        assert_eq!(cs[0]["hex"], "#ff0000", "{space}");
        assert_eq!(cs[0]["share"], 75.0);
        assert_eq!(cs[1]["hex"], "#0000ff", "{space}");
    }
    let mut d = doc_from(grad());
    let a = run(&spec("analyze colors"), &[("k", "5")], &mut d).unwrap();
    let b = run(&spec("analyze colors"), &[("k", "5")], &mut d).unwrap();
    assert_eq!(a, b);
    let total: f64 = a["colors"].as_array().unwrap().iter().map(|c| c["share"].as_f64().unwrap()).sum();
    assert!((total - 100.0).abs() < 0.05);
    let mut solid_doc = doc_from(solid(0.2, 0.4, 0.6));
    let s = run(&spec("analyze colors"), &[("k", "5")], &mut solid_doc).unwrap();
    assert_eq!(s["k"], 1);
    let mut clear = doc_from(Image::new(3, 3));
    assert_eq!(run(&spec("analyze colors"), &[], &mut clear).unwrap()["colors"], json!([]));
}

#[test]
fn colors_writes_an_svg_swatch() {
    let dir = TempDir::new("adjust-swatch");
    let s = spec("analyze colors");
    let Runner::Doc(f) = s.runner else { panic!() };
    let a = testing::args(&s, &[("k", "2")]).unwrap();
    let out = dir.path("palette.svg");
    let g = Globals { output: Some(out.display().to_string()), ..Globals::default() };
    let mut ctx = OpCtx::new(s.name, vec![], g, &NoIo, &NoProgress);
    let mut d = doc_from(two_colors());
    let v = f(&a, &mut d, &mut ctx).unwrap();
    let svg = std::fs::read_to_string(&out).unwrap();
    assert!(svg.starts_with("<svg") && svg.contains("#ff0000") && svg.contains("#0000ff"));
    assert_eq!(v["swatch"], out.display().to_string());
    assert_eq!(ctx.outputs.len(), 1);
    let g = Globals { output: Some(dir.path("palette.png").display().to_string()), ..Globals::default() };
    let mut ctx = OpCtx::new(s.name, vec![], g, &NoIo, &NoProgress);
    assert_eq!(f(&a, &mut d, &mut ctx).unwrap_err().code, "PX1106");
}

/// A test IO layer: images are stored as JSON `{"w", "h", "data"}`.
struct MemIo;

fn write_img(path: &Path, img: &Image) {
    let v = json!({"w": img.width, "h": img.height, "data": img.data});
    std::fs::write(path, serde_json::to_vec(&v).unwrap()).unwrap();
}

fn read_img(path: &Path) -> Image {
    let v: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let data: Vec<f32> = v["data"].as_array().unwrap().iter().map(|x| x.as_f64().unwrap() as f32).collect();
    Image { width: v["w"].as_u64().unwrap() as u32, height: v["h"].as_u64().unwrap() as u32, data }
}

impl ImageIo for MemIo {
    fn open(&self, path: &Path, _: &OpenOptions) -> Result<(Document, Vec<Warning>)> {
        Ok((Document::from_image(read_img(path)), vec![]))
    }
    fn decode(&self, _: &[u8], _: Option<&str>, _: &OpenOptions) -> Result<(Image, ImageInfo)> {
        Err(Error::unsupported("MemIo decodes nothing"))
    }
    fn probe(&self, _: &Path) -> Result<ImageInfo> {
        Err(Error::unsupported("MemIo probes nothing"))
    }
    fn encode(&self, image: &Image, _: Option<&Document>, path: &Path, _: &EncodeOptions) -> Result<Vec<Warning>> {
        write_img(path, image);
        Ok(vec![])
    }
    fn save_pxo(&self, _: &Document, _: &Path) -> Result<()> {
        Err(Error::unsupported("MemIo saves no documents"))
    }
    fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec![], vec![])
    }
}

fn run_diff(a: &Path, b: &Path, pairs: &[(&str, &str)]) -> Result<(Value, usize)> {
    let s = spec("analyze diff");
    let Runner::Custom(f) = s.runner else { panic!("diff is a custom op") };
    let args = testing::args(&s, pairs)?;
    let mut ctx = OpCtx::new(s.name, vec![a.to_path_buf(), b.to_path_buf()], Globals::default(), &MemIo, &NoProgress);
    let v = f(&args, &mut ctx)?;
    Ok((v, ctx.outputs.len()))
}

#[test]
fn diff_measures_changes() {
    let dir = TempDir::new("adjust-diff");
    let (a, b, c): (PathBuf, PathBuf, PathBuf) = (dir.path("a.json"), dir.path("b.json"), dir.path("c.json"));
    let base = grad();
    write_img(&a, &base);
    let mut changed = base.clone();
    changed.set(3, 4, [1.0, 0.0, 1.0, 1.0]);
    changed.set(5, 5, [base.get(5, 5)[0] + 0.001, base.get(5, 5)[1], 0.5, 1.0]);
    write_img(&b, &changed);
    let (same, _) = run_diff(&a, &a, &[]).unwrap();
    assert_eq!(same["identical"], true);
    assert_eq!(same["ssim"], 1.0);
    assert_eq!(same["changed_pixels"], 0);
    assert_eq!(same["delta_e00"]["max"], 0.0);
    let (v, outputs) = run_diff(&a, &b, &[]).unwrap();
    assert_eq!(v["identical"], false);
    assert_eq!(v["changed_pixels"], 1, "a sub-threshold change does not count: {v}");
    assert!(v["delta_e00"]["max"].as_f64().unwrap() > 10.0);
    assert!(v["ssim"].as_f64().unwrap() < 1.0);
    assert!((v["changed_percent"].as_f64().unwrap() - 100.0 / 512.0).abs() < 1e-3);
    assert_eq!(outputs, 0);
    // heatmap
    let heat = dir.path("heat.png");
    let (hv, outputs) = run_diff(&a, &b, &[("out", heat.to_str().unwrap()), ("threshold", "0")]).unwrap();
    assert_eq!(outputs, 1);
    assert_eq!(hv["changed_pixels"], 2);
    let h = read_img(&heat);
    assert_eq!((h.width, h.height), (32, 16));
    assert!(h.get(3, 4)[0] > 0.9 && h.get(0, 0)[0] < 0.3);
    // sizes must match
    write_img(&c, &Image::filled(8, 8, Rgba::WHITE));
    let e = run_diff(&a, &c, &[]).unwrap_err();
    assert_eq!(e.code, "PX1010");
    assert_eq!(e.context["a_size"], json!([32, 16]));
    assert_eq!(e.context["b_size"], json!([8, 8]));
}
