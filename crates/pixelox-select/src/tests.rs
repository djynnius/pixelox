//! Behavior tests for every command, run on in-memory documents.

use crate::ops;
use pixelox_core::ctx::{Globals, ImageInfo, ImageIo, NoProgress, OpenOptions};
use pixelox_core::envelope::Warning;
use pixelox_core::op::{Args, Runner};
use pixelox_core::prelude::*;
use pixelox_core::testing::{self, find, TempDir};
use std::path::Path;
use std::sync::{Arc, Once};

// ---------------------------------------------------------------------------------------
// harness

fn fake_text(l: &Layer, _: &Document) -> Result<Option<Rendered>> {
    let LayerKind::Text { spec } = &l.content else { return Ok(None) };
    let (x, y) = (spec["x"].as_f64().unwrap_or(0.0), spec["y"].as_f64().unwrap_or(0.0));
    Ok(Some(Rendered { image: Image::filled(10, 5, Rgba::BLACK), x: x.round() as i32, y: y.round() as i32 }))
}

fn fake_shape(l: &Layer, doc: &Document) -> Result<Option<Rendered>> {
    let LayerKind::Shape { spec } = &l.content else { return Ok(None) };
    let p = crate::raster::parse_path(spec["d"].as_str().unwrap_or(""))?;
    let m = crate::raster::fill_mask(doc.width, doc.height, &p, false);
    let img = Image::from_fn(doc.width, doc.height, |x, y| [0.0, 0.0, 1.0, m.get(x, y)]);
    Ok(Some(Rendered { image: img, x: 0, y: 0 }))
}

fn hooks() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        compose::set_text_renderer(fake_text);
        compose::set_shape_renderer(fake_shape);
    });
}

/// A minimal codec: 8-byte header (w, h as u32 LE) + RGBA8.
struct TestIo;

fn encode_test(img: &Image) -> Vec<u8> {
    let mut b = img.width.to_le_bytes().to_vec();
    b.extend(img.height.to_le_bytes());
    b.extend(img.to_rgba8());
    b
}

fn decode_test(b: &[u8]) -> Image {
    let w = u32::from_le_bytes(b[0..4].try_into().unwrap());
    let h = u32::from_le_bytes(b[4..8].try_into().unwrap());
    Image::from_rgba8(w, h, &b[8..])
}

impl ImageIo for TestIo {
    fn open(&self, p: &Path, _: &OpenOptions) -> Result<(Document, Vec<Warning>)> {
        Err(Error::unsupported(format!("test io cannot open {}", p.display())))
    }
    fn decode(&self, bytes: &[u8], _: Option<&str>, _: &OpenOptions) -> Result<(Image, ImageInfo)> {
        let img = decode_test(bytes);
        let info = ImageInfo { width: img.width, height: img.height, ..Default::default() };
        Ok((img, info))
    }
    fn probe(&self, p: &Path) -> Result<ImageInfo> {
        Err(Error::unsupported(format!("test io cannot probe {}", p.display())))
    }
    fn encode(&self, image: &Image, _: Option<&Document>, path: &Path, _: &EncodeOptions) -> Result<Vec<Warning>> {
        std::fs::write(path, encode_test(image))?;
        Ok(vec![])
    }
    fn save_pxo(&self, _: &Document, _: &Path) -> Result<()> {
        Err(Error::new("PX1106", "test io"))
    }
    fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec![], vec![])
    }
}

static IO: TestIo = TestIo;

fn spec(name: &str) -> OpSpec {
    find(&ops(), name)
}

fn run(name: &str, pairs: &[(&str, &str)], doc: &mut Document) -> Result<Value> {
    hooks();
    testing::run(&spec(name), pairs, doc)
}

fn run_w(name: &str, pairs: &[(&str, &str)], doc: &mut Document) -> Result<(Value, Vec<String>)> {
    hooks();
    testing::run_with_warnings(&spec(name), pairs, doc)
}

struct Ran {
    value: Value,
    resolved: Map<String, Value>,
    warnings: Vec<String>,
}

/// Run with the test codec and an optional `-o`.
fn run_io(name: &str, pairs: &[(&str, &str)], doc: &mut Document, output: Option<&Path>) -> Result<Ran> {
    hooks();
    let s = spec(name);
    let a = Args::build(&s, pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect())?;
    let g = Globals { output: output.map(|p| p.display().to_string()), ..Default::default() };
    let mut ctx = OpCtx::new(s.name, vec![], g, &IO, &NoProgress);
    let value = match s.runner {
        Runner::Doc(f) => f(&a, doc, &mut ctx)?,
        _ => panic!("not a doc op"),
    };
    Ok(Ran { value, resolved: ctx.resolved.clone(), warnings: ctx.warnings.iter().map(|w| w.code.clone()).collect() })
}

fn code(r: Result<impl std::fmt::Debug>) -> String {
    r.expect_err("expected an error").code
}

fn sel(doc: &Document) -> &Mask {
    doc.selection.as_ref().expect("selection")
}

fn area(m: &Mask) -> f64 {
    m.data.iter().map(|&v| v as f64).sum()
}

fn layer<'a>(doc: &'a Document, name: &str) -> &'a Layer {
    doc.layer(&doc.resolve(name).unwrap())
}

fn spec_of<'a>(doc: &'a Document, name: &str) -> &'a Value {
    match &layer(doc, name).content {
        LayerKind::Text { spec } | LayerKind::Shape { spec } => spec,
        other => panic!("{name} is {}", other.name()),
    }
}

fn placed_rect(doc: &Document, name: &str) -> ([f64; 4], f64) {
    match &layer(doc, name).content {
        LayerKind::Placed { rect, angle, .. } => (*rect, *angle),
        other => panic!("{name} is {}", other.name()),
    }
}

fn path_bbox(doc: &Document, name: &str) -> (f64, f64, f64, f64) {
    let (x0, y0, x1, y1) = geom::path_bbox(spec_of(doc, name)["d"].as_str().unwrap()).unwrap();
    (x0.round(), y0.round(), x1.round(), y1.round())
}

fn white(w: u32, h: u32) -> Image {
    Image::filled(w, h, Rgba::WHITE)
}

/// 100×80: Background (gradient), Pic (20×10 red at 30,20, left-half layer mask), Title
/// (text at 40,50), Box (shape 10..30), Placed (rect 50,5,20×10), group G with Inner
/// (10×10 at 60,40); a selection, a named mask, and two guides.
fn mixed_doc() -> Document {
    let mut d = testing::doc_layers(100, 80, vec![("Background", testing::gradient(100, 80))]);
    let mut pic = d.new_layer("Pic", LayerKind::Raster { image: Image::filled(20, 10, Rgba::new(1.0, 0.0, 0.0, 1.0)) });
    pic.x = 30;
    pic.y = 20;
    pic.mask = Some(LayerMask::new(testing::half_mask(100, 80)));
    d.insert_above(None, pic);
    let t = d.new_layer("Title", LayerKind::Text { spec: json!({"content": "Hi", "x": 40, "y": 50, "size": "20px"}) });
    d.insert_above(None, t);
    let b = d.new_layer(
        "Box",
        LayerKind::Shape { spec: json!({"d": "M 10 10 L 30 10 L 30 30 L 10 30 Z", "fill": "#00f", "stroke_width": 4}) },
    );
    d.insert_above(None, b);
    let src = Arc::new(Image::filled(4, 2, Rgba::new(0.0, 0.0, 1.0, 1.0)));
    let p = d.new_layer(
        "Placed",
        LayerKind::Placed {
            asset: "k".into(),
            path: "p.png".into(),
            linked: false,
            rect: [50.0, 5.0, 20.0, 10.0],
            angle: 0.0,
            source: Some(src),
        },
    );
    d.insert_above(None, p);
    let mut g = d.new_layer("G", LayerKind::Group { children: vec![] });
    let mut inner =
        d.new_layer("Inner", LayerKind::Raster { image: Image::filled(10, 10, Rgba::new(0.0, 1.0, 0.0, 1.0)) });
    inner.x = 60;
    inner.y = 40;
    g.children_mut().unwrap().push(inner);
    d.insert_above(None, g);
    d.selection =
        Some(Mask::from_fn(100, 80, |x, y| if (20..40).contains(&x) && (10..30).contains(&y) { 1.0 } else { 0.0 }));
    d.masks.insert("m".into(), Mask::from_fn(100, 80, |x, _| if x >= 50 { 1.0 } else { 0.0 }));
    d.guides.push(pixelox_core::doc::Guide { orientation: "v".into(), pos: 50.0 });
    d.guides.push(pixelox_core::doc::Guide { orientation: "h".into(), pos: 40.0 });
    d
}

// ---------------------------------------------------------------------------------------
// the op catalog

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

#[test]
fn every_command_is_documented() {
    let all = ops();
    let names: Vec<&str> = all.iter().map(|o| o.name).collect();
    for want in [
        "select rect",
        "select ellipse",
        "select poly",
        "select path",
        "select color",
        "select luma",
        "select all",
        "select none",
        "select invert",
        "select save",
        "select load",
        "select bounds",
        "mask list",
        "mask import",
        "mask export",
        "mask rm",
        "mask add",
        "mask remove",
        "mask enable",
        "mask disable",
        "mask density",
        "mask feather",
        "mask expand",
        "mask contract",
        "mask smooth",
        "mask invert",
        "transform move",
        "transform scale",
        "transform rotate",
        "transform flip",
        "transform skew",
        "transform perspective",
        "transform warp",
        "transform crop",
        "transform straighten",
        "transform canvas",
        "transform resize",
        "transform trim",
    ] {
        assert!(names.contains(&want), "missing {want}");
    }
    assert_eq!(names.len(), 38);
    assert!(!names.contains(&"mask refine"));
    for o in &all {
        assert!(o.summary.len() <= 60 && !o.summary.ends_with('.'), "{}: summary '{}'", o.name, o.summary);
        assert!(!o.about.is_empty(), "{}: no about", o.name);
        assert!(o.examples.len() >= 3, "{}: {} examples", o.name, o.examples.len());
        for p in &o.params {
            assert!(!p.help.is_empty(), "{} --{}: no help", o.name, p.name);
            assert!(!GLOBALS.contains(&p.name), "{} declares global --{}", o.name, p.name);
        }
        for e in &o.examples {
            assert!(e.cmd.starts_with(&format!("pixelox {} ", o.name)), "{}: example '{}'", o.name, e.cmd);
            for tok in e.cmd.split_whitespace() {
                let digit = tok.chars().nth(1).is_some_and(|c| c.is_ascii_digit() || c == '.');
                assert!(!(tok.starts_with('-') && digit), "{}: '{tok}' would parse as a flag", o.name);
            }
            for tok in e.cmd.split_whitespace().filter(|t| t.starts_with("--")) {
                let flag = tok.trim_start_matches("--").split('=').next().unwrap();
                assert!(
                    o.find_param(flag).is_some() || GLOBALS.contains(&flag),
                    "{}: example uses unknown --{flag}",
                    o.name
                );
            }
            let mutating = o.mode == DocMode::Mutate;
            if mutating {
                assert!(
                    e.cmd.contains(" -o ") || e.cmd.contains("--in-place") || e.cmd.contains("--dry-run"),
                    "{}: '{}' writes nowhere",
                    o.name,
                    e.cmd
                );
            }
        }
    }
}

use pixelox_core::op::DocMode;

// ---------------------------------------------------------------------------------------
// select

#[test]
fn select_rect_is_exact_and_reports_stats() {
    let mut d = testing::doc_from(white(100, 80));
    let v = run("select rect", &[("rect", "10,10,30x20")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 600.0);
    assert_eq!(v["selection"]["bounds"], json!([10, 10, 30, 20]));
    assert_eq!(v["selection"]["area"], json!(600.0));
    assert!((v["selection"]["coverage"].as_f64().unwrap() - 7.5).abs() < 1e-9);
    assert_eq!(code(run("select rect", &[("rect", "0,0,0x10")], &mut d)), "PX1011");
}

#[test]
fn select_modes_combine() {
    let mut d = testing::doc_from(white(40, 40));
    run("select rect", &[("rect", "0,0,20x20")], &mut d).unwrap();
    run("select rect", &[("rect", "10,10,20x20"), ("mode", "add")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 700.0);
    run("select rect", &[("rect", "0,0,20x20"), ("mode", "subtract")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 300.0);
    run("select rect", &[("rect", "20,20,20x20"), ("mode", "intersect")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 100.0);
    // subtract with no selection leaves an empty one (and warns)
    d.selection = None;
    let (_, w) = run_w("select rect", &[("rect", "0,0,5x5"), ("mode", "subtract")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 0.0);
    assert!(w.contains(&"PXW2002".to_string()));
}

#[test]
fn select_feather_zero_is_identity_and_positive_softens() {
    let mut a = testing::doc_from(white(60, 60));
    let mut b = a.clone();
    run("select rect", &[("rect", "20,20,20x20")], &mut a).unwrap();
    run("select rect", &[("rect", "20,20,20x20"), ("feather", "0")], &mut b).unwrap();
    assert_eq!(sel(&a), sel(&b));
    run("select rect", &[("rect", "20,20,20x20"), ("feather", "8")], &mut b).unwrap();
    assert!(sel(&b).get(18, 30) > 0.05 && sel(&b).get(18, 30) < 0.5);
    assert!((area(sel(&b)) - 400.0).abs() < 4.0, "feathering keeps the area: {}", area(sel(&b)));
}

#[test]
fn select_ellipse_poly_and_path_areas() {
    let mut d = testing::doc_from(white(100, 100));
    run("select ellipse", &[("rect", "10,20,60x40")], &mut d).unwrap();
    let want = std::f64::consts::PI * 30.0 * 20.0;
    assert!((area(sel(&d)) - want).abs() / want < 0.01, "{}", area(sel(&d)));
    assert_eq!(sel(&d).get(40, 40), 1.0);
    assert_eq!(sel(&d).get(11, 21), 0.0);
    run("select poly", &[("points", "0,0 40,0 0,40")], &mut d).unwrap();
    // tiny-skia's 4x4 supersampling gives 10/16 on a sample-aligned 45° edge: within 1%
    assert!((area(sel(&d)) - 800.0).abs() < 8.0, "{}", area(sel(&d)));
    assert_eq!(code(run("select poly", &[("points", "0,0 4,4")], &mut d)), "PX1010");
    run("select path", &[("d", "M 10 10 H 30 V 30 H 10 Z")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 400.0);
    run("select path", &[("d", "M0 0H100V100H0Z M25 25H75V75H25Z"), ("fill-rule", "evenodd")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 7500.0);
    assert_eq!(code(run("select path", &[], &mut d)), "PX1003");
    assert_eq!(code(run("select path", &[("d", "M0 0Z"), ("points-file", "x.json")], &mut d)), "PX1005");
    assert_eq!(code(run("select path", &[("d", "garbage")], &mut d)), "PX1010");
}

#[test]
fn select_path_reads_points_files() {
    let dir = TempDir::new("select-path");
    let f = dir.path("pts.json");
    std::fs::write(&f, r#"{"points": [[0,0],[20,0],[20,10],{"x":0,"y":10}]}"#).unwrap();
    let mut d = testing::doc_from(white(50, 50));
    let r = run_io("select path", &[("points-file", f.to_str().unwrap())], &mut d, None).unwrap();
    assert_eq!(area(sel(&d)), 200.0);
    assert_eq!(r.value["selection"]["bounds"], json!([0, 0, 20, 10]));
    std::fs::write(&f, r#"[[[0,0],[10,0],[10,10]], [[20,20],[30,20],[30,30]]]"#).unwrap();
    run_io("select path", &[("points-file", f.to_str().unwrap())], &mut d, None).unwrap();
    assert!((area(sel(&d)) - 100.0).abs() < 3.0, "{}", area(sel(&d)));
    std::fs::write(&f, r#"{"nope": 1}"#).unwrap();
    assert_eq!(
        code(run_io("select path", &[("points-file", f.to_str().unwrap())], &mut d, None).map(|r| r.value)),
        "PX1010"
    );
}

fn two_red_squares() -> Document {
    let img = Image::from_fn(40, 20, |x, y| {
        let inside = (2..10).contains(&y) && ((2..10).contains(&x) || (22..30).contains(&x));
        if inside {
            [1.0, 0.0, 0.0, 1.0]
        } else {
            [1.0, 1.0, 1.0, 1.0]
        }
    });
    testing::doc_from(img)
}

#[test]
fn select_color_global_and_magic_wand() {
    let mut d = two_red_squares();
    let v = run("select color", &[("ref", "red"), ("tolerance", "10")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 128.0);
    assert_eq!(v["ref"], json!("#ff0000"));
    run("select color", &[("at", "4,4"), ("contiguous", "true")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 64.0);
    assert_eq!(sel(&d).get(25, 5), 0.0);
    // tolerance 255 selects everything reachable
    run("select color", &[("at", "0,0"), ("tolerance", "255")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 800.0);
    assert_eq!(code(run("select color", &[("contiguous", "true"), ("ref", "red")], &mut d)), "PX1003");
    assert_eq!(code(run("select color", &[], &mut d)), "PX1003");
    assert_eq!(code(run("select color", &[("at", "100,4")], &mut d)), "PX1011");
}

#[test]
fn select_color_samples_a_layer() {
    let mut d = testing::doc_layers(
        10,
        10,
        vec![("A", white(10, 10)), ("B", Image::filled(10, 10, Rgba::new(0.0, 0.0, 1.0, 1.0)))],
    );
    run("select color", &[("ref", "white"), ("tolerance", "0")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 0.0, "the composite is blue");
    run("select color", &[("ref", "white"), ("tolerance", "0"), ("layer", "A")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 100.0);
}

#[test]
fn select_luma_ranges() {
    let mut d = testing::doc_from(testing::gradient(101, 10));
    run("select luma", &[("channel", "r"), ("range", "0.5..1"), ("softness", "0")], &mut d).unwrap();
    assert_eq!(sel(&d).get(49, 0), 0.0);
    assert_eq!(sel(&d).get(51, 0), 1.0);
    run("select luma", &[("channel", "r"), ("range", "0.5..1"), ("softness", "0.1")], &mut d).unwrap();
    let soft = sel(&d).get(45, 0);
    assert!(soft > 0.3 && soft < 0.7, "{soft}");
    run("select luma", &[("channel", "a"), ("range", "0.99..1")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 1010.0);
    assert_eq!(code(run("select luma", &[("range", "0.8..0.2")], &mut d)), "PX1010");
}

#[test]
fn select_all_none_invert() {
    let mut d = testing::doc_from(white(20, 10));
    run("select all", &[], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 200.0);
    run("select none", &[], &mut d).unwrap();
    assert!(d.selection.is_none());
    run("select invert", &[], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 200.0);
    run("select ellipse", &[("rect", "2,1,15x7"), ("feather", "2")], &mut d).unwrap();
    let before = sel(&d).clone();
    run("select invert", &[], &mut d).unwrap();
    assert!((area(sel(&d)) - (200.0 - area(&before))).abs() < 1e-3);
    run("select invert", &[], &mut d).unwrap();
    let back = sel(&d);
    assert!(back.data.iter().zip(&before.data).all(|(a, b)| (a - b).abs() < 1e-6), "invert∘invert = id");
    run("select all", &[("mode", "intersect")], &mut d).unwrap();
    assert!(sel(&d).data.iter().zip(&before.data).all(|(a, b)| (a - b).abs() < 1e-6));
}

#[test]
fn select_save_load_round_trip() {
    let mut d = testing::doc_from(white(30, 30));
    assert_eq!(code(run("select save", &[("as", "x")], &mut d)), "PX2312");
    run("select rect", &[("rect", "0,0,10x10")], &mut d).unwrap();
    let v = run("select save", &[("as", "box")], &mut d).unwrap();
    assert_eq!(v["mask"], json!("box"));
    assert_eq!(code(run("select save", &[("as", "box")], &mut d)), "PX2310");
    run("select save", &[("as", "box"), ("replace", "true")], &mut d).unwrap();
    assert_eq!(code(run("select save", &[("as", "@bad")], &mut d)), "PX1010");
    run("select none", &[], &mut d).unwrap();
    run("select load", &[("name", "box")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 100.0);
    run("select rect", &[("rect", "20,20,10x10")], &mut d).unwrap();
    run("select load", &[("name", "box"), ("mode", "add")], &mut d).unwrap();
    assert_eq!(area(sel(&d)), 200.0);
    assert_eq!(code(run("select load", &[("name", "missing")], &mut d)), "PX2311");
    let v = run("select load", &[("name", "@alpha:Background")], &mut d).unwrap();
    assert_eq!(v["selection"]["area"], json!(900.0));
}

#[test]
fn select_load_reads_a_mask_image() {
    let dir = TempDir::new("select-load");
    let f = dir.path("m.raw");
    std::fs::write(
        &f,
        encode_test(&Image::from_fn(10, 10, |x, _| if x < 5 { [1.0; 4] } else { [0.0, 0.0, 0.0, 1.0] })),
    )
    .unwrap();
    let mut d = testing::doc_from(white(10, 10));
    run_io("select load", &[("name", f.to_str().unwrap())], &mut d, None).unwrap();
    assert_eq!(area(sel(&d)), 50.0);
}

#[test]
fn select_bounds_reports_or_says_empty() {
    let mut d = testing::doc_from(white(50, 40));
    let v = run("select bounds", &[], &mut d).unwrap();
    assert_eq!((v["empty"].clone(), v["active"].clone()), (json!(true), json!(false)));
    run("select rect", &[("rect", "5,6,10x4")], &mut d).unwrap();
    let v = run("select bounds", &[], &mut d).unwrap();
    assert_eq!(v["bounds"], json!([5, 6, 10, 4]));
    assert_eq!(v["area"], json!(40.0));
    assert_eq!(v["coverage"], json!(2.0));
    assert!(spec("select bounds").is_read_only());
}

// ---------------------------------------------------------------------------------------
// mask

#[test]
fn mask_list_reports_library_and_layer_masks() {
    let mut d = mixed_doc();
    let v = run("mask list", &[], &mut d).unwrap();
    assert_eq!(v["masks"][0]["name"], json!("m"));
    assert_eq!(v["masks"][0]["coverage"], json!(50.0));
    assert_eq!(v["layer_masks"][0]["layer"], json!("Pic"));
    assert_eq!(v["layer_masks"][0]["enabled"], json!(true));
    assert_eq!(v["selection"]["area"], json!(400.0));
    assert_eq!(v["count"], json!(2));
    assert!(spec("mask list").is_read_only());
}

#[test]
fn mask_import_and_export_round_trip() {
    let dir = TempDir::new("mask-io");
    let f = dir.path("matte.raw");
    let matte = Image::from_fn(20, 10, |x, _| if x < 5 { [1.0; 4] } else { [0.0, 0.0, 0.0, 1.0] });
    std::fs::write(&f, encode_test(&matte)).unwrap();
    let mut d = testing::doc_from(white(20, 10));
    let r = run_io("mask import", &[("file", f.to_str().unwrap()), ("as", "subject")], &mut d, None).unwrap();
    assert_eq!(r.value["stats"]["area"], json!(50.0));
    assert_eq!(area(&d.masks["subject"]), 50.0);
    assert_eq!(
        code(run_io("mask import", &[("file", f.to_str().unwrap()), ("as", "subject")], &mut d, None).map(|r| r.value)),
        "PX2310"
    );
    run_io("mask import", &[("file", f.to_str().unwrap()), ("as", "subject"), ("replace", "true")], &mut d, None)
        .unwrap();
    // size mismatch
    let small = dir.path("small.raw");
    std::fs::write(&small, encode_test(&white(5, 5))).unwrap();
    assert_eq!(
        code(run_io("mask import", &[("file", small.to_str().unwrap()), ("as", "s")], &mut d, None).map(|r| r.value)),
        "PX2313"
    );
    // export
    let out = dir.path("out.raw");
    let r = run_io("mask export", &[("mask", "subject")], &mut d, Some(&out)).unwrap();
    assert_eq!(r.value["size"], json!([20, 10]));
    let back = decode_test(&std::fs::read(&out).unwrap());
    assert_eq!(back.get(0, 0), [1.0, 1.0, 1.0, 1.0]);
    assert_eq!(back.get(10, 0), [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(code(run_io("mask export", &[("mask", "subject")], &mut d, None).map(|r| r.value)), "PX1004");
    assert_eq!(spec("mask export").mode, DocMode::Export);
}

#[test]
fn mask_rm_removes_or_reports() {
    let mut d = mixed_doc();
    run("mask rm", &[("mask", "m")], &mut d).unwrap();
    assert!(d.masks.is_empty());
    assert_eq!(code(run("mask rm", &[("mask", "m")], &mut d)), "PX2311");
    run("mask rm", &[("mask", "@sel")], &mut d).unwrap();
    assert!(d.selection.is_none());
}

#[test]
fn mask_add_every_source() {
    let mut d = mixed_doc();
    let v = run("mask add", &[("layer", "Inner")], &mut d).unwrap();
    assert_eq!(v["source"], json!("@sel"), "defaults to the selection");
    assert_eq!(area(&layer(&d, "Inner").mask.as_ref().unwrap().mask), 400.0);
    run("mask add", &[("layer", "Inner"), ("hide-all", "true")], &mut d).unwrap();
    assert_eq!(area(&layer(&d, "Inner").mask.as_ref().unwrap().mask), 0.0);
    run("mask add", &[("layer", "Inner"), ("reveal-all", "true")], &mut d).unwrap();
    assert_eq!(area(&layer(&d, "Inner").mask.as_ref().unwrap().mask), 8000.0);
    run("mask add", &[("layer", "Inner"), ("from", "m")], &mut d).unwrap();
    assert_eq!(area(&layer(&d, "Inner").mask.as_ref().unwrap().mask), 4000.0);
    let v = run("mask add", &[("layer", "Title"), ("from-path", "M 0 0 H 10 V 10 H 0 Z")], &mut d).unwrap();
    assert_eq!(v["vector"], json!(true));
    assert_eq!(layer(&d, "Title").vector_mask.as_deref(), Some("M 0 0 H 10 V 10 H 0 Z"));
    assert_eq!(area(&layer(&d, "Title").mask.as_ref().unwrap().mask), 100.0);
    assert_eq!(code(run("mask add", &[("layer", "Inner"), ("hide-all", "true"), ("from", "m")], &mut d)), "PX1005");
    let p = d.resolve("Inner").unwrap();
    d.layer_mut(&p).locked = true;
    assert_eq!(code(run("mask add", &[("layer", "Inner"), ("reveal-all", "true")], &mut d)), "PX2304");
}

#[test]
fn mask_add_from_file() {
    let dir = TempDir::new("mask-add");
    let f = dir.path("m.raw");
    std::fs::write(
        &f,
        encode_test(&Image::from_fn(10, 10, |_, y| if y < 3 { [1.0; 4] } else { [0.0, 0.0, 0.0, 1.0] })),
    )
    .unwrap();
    let mut d = testing::doc_from(white(10, 10));
    run_io("mask add", &[("from-file", f.to_str().unwrap())], &mut d, None).unwrap();
    assert_eq!(area(&d.layers[0].mask.as_ref().unwrap().mask), 30.0);
}

#[test]
fn mask_remove_and_apply_bakes_alpha() {
    let mut d = testing::doc_from(white(10, 10));
    d.layers[0].mask = Some(LayerMask::new(testing::half_mask(10, 10)));
    let v = run("mask remove", &[("apply", "true")], &mut d).unwrap();
    assert_eq!(v["applied"], json!(true));
    assert!(d.layers[0].mask.is_none());
    let img = testing::raster(&d, "Background");
    assert_eq!((img.get(2, 2)[3], img.get(8, 2)[3]), (1.0, 0.0));
    assert_eq!(code(run("mask remove", &[], &mut d)), "PX2311");
    // density 50 bakes half strength
    let mut lm = LayerMask::new(Mask::new(10, 10));
    lm.density = 50.0;
    d.layers[0].mask = Some(lm);
    d.layers[0].content = LayerKind::Raster { image: white(10, 10) };
    run("mask remove", &[("apply", "true")], &mut d).unwrap();
    assert!((testing::raster(&d, "Background").get(5, 5)[3] - 0.5).abs() < 1e-6);
    // non-raster layers cannot bake
    let mut m = mixed_doc();
    run("mask add", &[("layer", "Placed"), ("hide-all", "true")], &mut m).unwrap();
    assert_eq!(code(run("mask remove", &[("layer", "Placed"), ("apply", "true")], &mut m)), "PX2303");
    run("mask remove", &[("layer", "Placed")], &mut m).unwrap();
    assert!(layer(&m, "Placed").mask.is_none());
}

#[test]
fn mask_enable_disable_toggle() {
    let mut d = mixed_doc();
    let v = run("mask disable", &[("layer", "Pic")], &mut d).unwrap();
    assert_eq!(v["changed"], json!(true));
    assert!(!layer(&d, "Pic").mask.as_ref().unwrap().enabled);
    run("mask enable", &[("layer", "Pic")], &mut d).unwrap();
    assert!(layer(&d, "Pic").mask.as_ref().unwrap().enabled);
    assert_eq!(code(run("mask enable", &[("layer", "Inner")], &mut d)), "PX2311");
}

#[test]
fn mask_density_live_and_destructive() {
    let mut d = mixed_doc();
    let v = run("mask density", &[("layer", "Pic"), ("value", "40")], &mut d).unwrap();
    assert_eq!(v["live"], json!(true));
    assert_eq!(layer(&d, "Pic").mask.as_ref().unwrap().density, 40.0);
    run("mask density", &[("mask", "m"), ("value", "30")], &mut d).unwrap();
    assert!((d.masks["m"].get(0, 0) - 0.7).abs() < 1e-6);
    assert_eq!(d.masks["m"].get(60, 0), 1.0);
    assert_eq!(code(run("mask density", &[("mask", "m"), ("value", "130")], &mut d)), "PX1013");
    assert_eq!(code(run("mask density", &[("mask", "m"), ("layer", "Pic"), ("value", "3")], &mut d)), "PX1005");
}

#[test]
fn mask_feather_identity_and_effect() {
    let mut d = mixed_doc();
    let before = d.masks["m"].clone();
    run("mask feather", &[("mask", "m"), ("radius", "0")], &mut d).unwrap();
    assert_eq!(d.masks["m"], before, "radius 0 = identity");
    run("mask feather", &[("mask", "m"), ("radius", "6")], &mut d).unwrap();
    assert!(d.masks["m"].get(48, 10) > 0.05 && d.masks["m"].get(48, 10) < 0.5);
    // no flag: edits the selection
    let v = run("mask feather", &[("radius", "4px")], &mut d).unwrap();
    assert_eq!(v["mask"], json!("@sel"));
    assert!(sel(&d).get(19, 20) > 0.0);
    let v = run("mask feather", &[("layer", "Pic"), ("radius", "5")], &mut d).unwrap();
    assert_eq!(v["live"], json!(true));
    assert_eq!(layer(&d, "Pic").mask.as_ref().unwrap().feather, 5.0);
    assert_eq!(code(run("mask feather", &[("mask", "nope"), ("radius", "2")], &mut d)), "PX2311");
    assert_eq!(code(run("mask feather", &[("mask", "@alpha:Pic"), ("radius", "2")], &mut d)), "PX1010");
    assert_eq!(code(run("mask feather", &[("mask", "m"), ("radius", "-2")], &mut d)), "PX1013");
    d.selection = None;
    assert_eq!(code(run("mask feather", &[("radius", "2")], &mut d)), "PX1003");
}

#[test]
fn mask_expand_contract_and_smooth() {
    let mut d = testing::doc_from(white(40, 40));
    d.masks.insert(
        "sq".into(),
        Mask::from_fn(40, 40, |x, y| if (10..30).contains(&x) && (10..30).contains(&y) { 1.0 } else { 0.0 }),
    );
    run("mask expand", &[("mask", "sq"), ("by", "2")], &mut d).unwrap();
    assert_eq!(d.masks["sq"].get(8, 20), 1.0);
    assert_eq!(d.masks["sq"].get(7, 20), 0.0);
    run("mask contract", &[("mask", "sq"), ("by", "2")], &mut d).unwrap();
    assert_eq!(d.masks["sq"].get(9, 20), 0.0);
    assert_eq!(d.masks["sq"].get(10, 20), 1.0);
    run("mask contract", &[("mask", "sq"), ("by", "3")], &mut d).unwrap();
    assert!(area(&d.masks["sq"]) < 400.0);
    let mut speck = Mask::new(40, 40);
    speck.set(5, 5, 1.0);
    d.masks.insert("speck".into(), speck);
    run("mask smooth", &[("mask", "speck"), ("radius", "1")], &mut d).unwrap();
    assert_eq!(area(&d.masks["speck"]), 0.0);
}

#[test]
fn mask_invert_twice_is_identity_for_every_target() {
    let mut d = mixed_doc();
    let (m0, s0, l0) = (d.masks["m"].clone(), sel(&d).clone(), layer(&d, "Pic").mask.as_ref().unwrap().mask.clone());
    for t in [("mask", "m"), ("mask", "@sel"), ("layer", "Pic")] {
        run("mask invert", &[t], &mut d).unwrap();
    }
    assert_eq!(d.masks["m"], m0.inverted());
    for t in [("mask", "m"), ("mask", "@sel"), ("layer", "Pic")] {
        run("mask invert", &[t], &mut d).unwrap();
    }
    assert_eq!(d.masks["m"], m0);
    assert_eq!(sel(&d), &s0);
    assert_eq!(layer(&d, "Pic").mask.as_ref().unwrap().mask, l0);
    let p = d.resolve("Pic").unwrap();
    d.layer_mut(&p).locked = true;
    assert_eq!(code(run("mask invert", &[("layer", "Pic")], &mut d)), "PX2304");
}

// ---------------------------------------------------------------------------------------
// transform: layers

#[test]
fn move_every_layer_kind() {
    let mut d = mixed_doc();
    run("transform move", &[("layer", "Pic"), ("by", "5,-3")], &mut d).unwrap();
    assert_eq!((layer(&d, "Pic").x, layer(&d, "Pic").y), (35, 17));
    assert_eq!(layer(&d, "Pic").mask.as_ref().unwrap().mask.get(53, 10), 1.0, "the mask moves along");
    run("transform move", &[("layer", "Title"), ("by", "5,5")], &mut d).unwrap();
    assert_eq!((spec_of(&d, "Title")["x"].as_f64(), spec_of(&d, "Title")["y"].as_f64()), (Some(45.0), Some(55.0)));
    let v = run("transform move", &[("layer", "Box"), ("to", "0,0")], &mut d).unwrap();
    assert_eq!(path_bbox(&d, "Box"), (0.0, 0.0, 20.0, 20.0));
    assert_eq!(v["bounds"], json!([0, 0, 20, 20]));
    run("transform move", &[("layer", "Pic"), ("to", "50%,50%"), ("anchor", "c")], &mut d).unwrap();
    assert_eq!((layer(&d, "Pic").x, layer(&d, "Pic").y), (40, 35));
    run("transform move", &[("layer", "Placed"), ("to", "-1,-1"), ("anchor", "br")], &mut d).unwrap();
    assert_eq!(placed_rect(&d, "Placed").0, [79.0, 69.0, 20.0, 10.0]);
    run("transform move", &[("layer", "G"), ("by", "1,1")], &mut d).unwrap();
    assert_eq!((layer(&d, "Inner").x, layer(&d, "Inner").y), (61, 41));
    assert_eq!(code(run("transform move", &[("layer", "Pic")], &mut d)), "PX1003");
    assert_eq!(code(run("transform move", &[("layer", "Pic"), ("by", "1,1"), ("to", "1,1")], &mut d)), "PX1005");
    assert_eq!(code(run("transform move", &[("by", "1,1")], &mut d)), "PX2302");
    let p = d.resolve("Pic").unwrap();
    d.layer_mut(&p).locked = true;
    assert_eq!(code(run("transform move", &[("layer", "Pic"), ("by", "1,1")], &mut d)), "PX2304");
}

#[test]
fn move_carries_vector_masks() {
    let mut d = mixed_doc();
    run("mask add", &[("layer", "Box"), ("from-path", "M 0 0 H 10 V 10 Z")], &mut d).unwrap();
    run("transform move", &[("layer", "Box"), ("by", "3,4")], &mut d).unwrap();
    let vm = layer(&d, "Box").vector_mask.clone().unwrap();
    assert_eq!(geom::path_bbox(&vm).unwrap(), (3.0, 4.0, 13.0, 14.0));
}

#[test]
fn scale_layers_keep_them_live() {
    let mut d = mixed_doc();
    let v = run("transform scale", &[("layer", "Pic"), ("to", "50%")], &mut d).unwrap();
    let pic = layer(&d, "Pic");
    assert_eq!((pic.raster().unwrap().width, pic.raster().unwrap().height), (10, 5));
    assert_eq!(v["scale"], json!([0.5, 0.5]));
    run("transform scale", &[("layer", "Title"), ("to", "2")], &mut d).unwrap();
    let t = spec_of(&d, "Title");
    assert_eq!(t["size"], json!("40px"));
    assert_eq!((t["x"].as_f64(), t["y"].as_f64()), (Some(35.0), Some(47.5)));
    run("transform scale", &[("layer", "Box"), ("to", "0.5"), ("anchor", "tl")], &mut d).unwrap();
    assert_eq!(path_bbox(&d, "Box"), (10.0, 10.0, 20.0, 20.0));
    assert_eq!(spec_of(&d, "Box")["stroke_width"], json!(2.0));
    run("transform scale", &[("layer", "Placed"), ("size", "40x"), ("anchor", "tl")], &mut d).unwrap();
    assert_eq!(placed_rect(&d, "Placed").0, [50.0, 5.0, 40.0, 20.0]);
    // a non-uniform scale cannot stay live text
    let (v, w) = run_w("transform scale", &[("layer", "Title"), ("size", "80x10")], &mut d).unwrap();
    assert!(w.contains(&"PXW2003".to_string()));
    assert_eq!(v["rasterized"], json!(["Title"]));
    assert!(matches!(layer(&d, "Title").content, LayerKind::Raster { .. }));
    assert_eq!(code(run("transform scale", &[("layer", "Pic"), ("to", "-2")], &mut d)), "PX1013");
    assert_eq!(code(run("transform scale", &[("layer", "Pic")], &mut d)), "PX1003");
}

#[test]
fn rotate_layer_quarter_turns_are_lossless() {
    let mut d = mixed_doc();
    let p = d.resolve("Pic").unwrap();
    d.layer_mut(&p).content = LayerKind::Raster { image: testing::gradient(20, 10) };
    let orig = testing::raster(&d, "Pic").clone();
    let v = run("transform rotate", &[("layer", "Pic"), ("degrees", "90")], &mut d).unwrap();
    assert_eq!(v["bounds"], json!([35, 15, 10, 20]));
    for _ in 0..3 {
        run("transform rotate", &[("layer", "Pic"), ("degrees", "90")], &mut d).unwrap();
    }
    assert_eq!(testing::raster(&d, "Pic"), &orig);
    assert_eq!((layer(&d, "Pic").x, layer(&d, "Pic").y), (30, 20));
    run("transform rotate", &[("layer", "Title"), ("degrees", "30"), ("around", "40,50")], &mut d).unwrap();
    assert_eq!(spec_of(&d, "Title")["rotation"], json!(30.0));
    assert_eq!(spec_of(&d, "Title")["x"], json!(40.0));
    run("transform rotate", &[("layer", "Placed"), ("degrees", "-15")], &mut d).unwrap();
    assert_eq!(placed_rect(&d, "Placed").1, -15.0);
    // odd-sized content still rotates exactly
    let mut o = testing::doc_from(testing::gradient(7, 4));
    let src = testing::raster(&o, "Background").clone();
    for _ in 0..4 {
        run("transform rotate", &[("layer", "Background"), ("degrees", "90")], &mut o).unwrap();
    }
    assert_eq!(testing::raster(&o, "Background"), &src);
}

#[test]
fn flip_layer_twice_is_identity_and_mirrors_live_text_as_pixels() {
    let mut d = mixed_doc();
    let p = d.resolve("Pic").unwrap();
    d.layer_mut(&p).content = LayerKind::Raster { image: testing::gradient(20, 10) };
    let orig = testing::raster(&d, "Pic").clone();
    run("transform flip", &[("layer", "Pic"), ("h", "true")], &mut d).unwrap();
    assert_eq!(testing::raster(&d, "Pic").get(0, 0), orig.get(19, 0));
    run("transform flip", &[("layer", "Pic"), ("h", "true")], &mut d).unwrap();
    assert_eq!(testing::raster(&d, "Pic"), &orig);
    run("transform flip", &[("layer", "Box"), ("v", "true")], &mut d).unwrap();
    assert_eq!(path_bbox(&d, "Box"), (10.0, 10.0, 30.0, 30.0));
    assert!(matches!(layer(&d, "Box").content, LayerKind::Shape { .. }), "shapes mirror live");
    let (_, w) = run_w("transform flip", &[("layer", "Title"), ("h", "true")], &mut d).unwrap();
    assert!(w.contains(&"PXW2003".to_string()));
    assert_eq!(code(run("transform flip", &[("layer", "Pic")], &mut d)), "PX1003");
}

#[test]
fn skew_widens_and_rasterizes_live_layers() {
    let mut d = mixed_doc();
    let v = run("transform skew", &[("layer", "Pic"), ("x", "20")], &mut d).unwrap();
    let b = v["bounds"].as_array().unwrap();
    assert!(b[2].as_u64().unwrap() >= 23 && b[3].as_u64().unwrap() == 10, "{v}");
    let (v, w) = run_w("transform skew", &[("layer", "Placed"), ("y", "10")], &mut d).unwrap();
    assert!(w.contains(&"PXW2003".to_string()));
    assert_eq!(v["rasterized"], json!(["Placed"]));
    assert!(matches!(layer(&d, "Placed").content, LayerKind::Raster { .. }));
    assert_eq!(code(run("transform skew", &[("layer", "Pic"), ("x", "89")], &mut d)), "PX1013");
}

#[test]
fn perspective_identity_quad_keeps_pixels() {
    let mut d = testing::doc_layers(60, 40, vec![("A", Image::new(60, 40))]);
    let p = d.resolve("A").unwrap();
    d.layer_mut(&p).content = LayerKind::Raster { image: testing::gradient(20, 10) };
    d.layer_mut(&p).x = 10;
    d.layer_mut(&p).y = 5;
    let orig = testing::raster(&d, "A").clone();
    run("transform perspective", &[("layer", "A"), ("quad", "10,5 30,5 30,15 10,15")], &mut d).unwrap();
    let got = testing::raster(&d, "A");
    assert_eq!((got.width, got.height, layer(&d, "A").x, layer(&d, "A").y), (20, 10, 10, 5));
    assert!(testing::max_diff(got, &orig) < 1e-4);
    let v = run("transform perspective", &[("layer", "A"), ("quad", "10,5 50,0 50,40 10,15")], &mut d).unwrap();
    assert_eq!(v["bounds"].as_array().unwrap()[2], json!(40));
    assert_eq!(code(run("transform perspective", &[("layer", "A"), ("quad", "0,0 10,0 10,10")], &mut d)), "PX1010");
    assert_eq!(
        code(run("transform perspective", &[("layer", "A"), ("quad", "0,0 10,10 10,0 0,10")], &mut d)),
        "PX1010"
    );
}

#[test]
fn warp_identity_grid_and_bends() {
    let dir = TempDir::new("warp");
    let f = dir.path("warp.json");
    let mut d = testing::doc_layers(60, 40, vec![("A", Image::new(1, 1))]);
    let p = d.resolve("A").unwrap();
    d.layer_mut(&p).content = LayerKind::Raster { image: testing::gradient(32, 16) };
    let orig = testing::raster(&d, "A").clone();
    std::fs::write(&f, r#"{"points": [[0,0],[16,0],[32,0],[0,8],[16,8],[32,8],[0,16],[16,16],[32,16]]}"#).unwrap();
    run_io("transform warp", &[("layer", "A"), ("points", f.to_str().unwrap())], &mut d, None).unwrap();
    assert!(testing::max_diff(testing::raster(&d, "A"), &orig) < 1e-3);
    std::fs::write(&f, r#"{"grid": "2x2", "relative": true, "points": [[0,0],[0,0],[0,10],[0,10]]}"#).unwrap();
    let r = run_io("transform warp", &[("layer", "A"), ("points", f.to_str().unwrap())], &mut d, None).unwrap();
    assert_eq!(r.value["grid"], json!("2x2"));
    assert_eq!(testing::raster(&d, "A").height, 26);
    std::fs::write(&f, "[[0,0],[1,1]]").unwrap();
    assert_eq!(
        code(
            run_io("transform warp", &[("layer", "A"), ("points", f.to_str().unwrap())], &mut d, None).map(|r| r.value)
        ),
        "PX1010"
    );
    assert_eq!(code(run("transform warp", &[("layer", "A"), ("points", "w.json"), ("grid", "1x5")], &mut d)), "PX1010");
}

// ---------------------------------------------------------------------------------------
// transform: document

#[test]
fn crop_shifts_every_layer_kind_and_mask() {
    let mut d = mixed_doc();
    let bg = testing::raster(&d, "Background").clone();
    let pic_mask = layer(&d, "Pic").mask.as_ref().unwrap().mask.clone();
    let (sel0, m0) = (sel(&d).clone(), d.masks["m"].clone());
    let p = d.resolve("Background").unwrap();
    d.layer_mut(&p).locked = true;
    let v = run("transform crop", &[("rect", "10,5,60x50")], &mut d).unwrap();
    assert_eq!(v["size"], json!([60, 50]));
    assert_eq!((d.width, d.height), (60, 50));
    let b = testing::raster(&d, "Background");
    assert_eq!((b.width, b.height, layer(&d, "Background").x), (60, 50, 0));
    assert_eq!(b.get(0, 0), bg.get(10, 5));
    assert!(layer(&d, "Background").locked, "locks survive document geometry");
    assert_eq!((layer(&d, "Pic").x, layer(&d, "Pic").y), (20, 15));
    assert_eq!((spec_of(&d, "Title")["x"].as_f64(), spec_of(&d, "Title")["y"].as_f64()), (Some(30.0), Some(45.0)));
    assert_eq!(path_bbox(&d, "Box"), (0.0, 5.0, 20.0, 25.0));
    assert_eq!(placed_rect(&d, "Placed").0, [40.0, 0.0, 20.0, 10.0]);
    assert_eq!((layer(&d, "Inner").x, layer(&d, "Inner").y), (50, 35));
    let lm = &layer(&d, "Pic").mask.as_ref().unwrap().mask;
    assert_eq!((lm.width, lm.height), (60, 50));
    for (x, y) in [(0, 0), (39, 10), (40, 10), (59, 49)] {
        assert_eq!(lm.get(x, y), pic_mask.get(x + 10, y + 5));
        assert_eq!(sel(&d).get(x, y), sel0.get(x + 10, y + 5));
        assert_eq!(d.masks["m"].get(x, y), m0.get(x + 10, y + 5));
    }
    assert_eq!((d.guides[0].pos, d.guides[1].pos), (40.0, 35.0));
    assert_eq!(code(run("transform crop", &[("rect", "500,500,10x10")], &mut d)), "PX3320");
}

#[test]
fn crop_ratio_auto_and_gates() {
    let mut d = testing::doc_from(testing::gradient(100, 80));
    run("transform crop", &[("ratio", "4:5")], &mut d).unwrap();
    assert_eq!((d.width, d.height), (64, 80));
    let mut d = testing::doc_from(testing::gradient(100, 80));
    run("transform crop", &[("ratio", "2"), ("gravity", "t")], &mut d).unwrap();
    assert_eq!((d.width, d.height), (100, 50));
    let img = Image::from_fn(50, 40, |x, y| {
        if (10..30).contains(&x) && (8..18).contains(&y) {
            [0.0, 0.0, 0.0, 1.0]
        } else {
            [1.0; 4]
        }
    });
    let mut d = testing::doc_from(img);
    let v = run("transform crop", &[("auto", "trim")], &mut d).unwrap();
    assert_eq!((v["rect"].clone(), d.width, d.height), (json!([10, 8, 20, 10]), 20, 10));
    assert_eq!(code(run("transform crop", &[("content-aware", "true"), ("rect", "0,0,5x5")], &mut d)), "PX1520");
    assert_eq!(code(run("transform crop", &[], &mut d)), "PX1003");
    assert_eq!(code(run("transform crop", &[("rect", "0,0,5x5"), ("ratio", "1:1")], &mut d)), "PX1005");
    assert_eq!(code(run("transform crop", &[("ratio", "x:y")], &mut d)), "PX1010");
}

#[test]
fn rotate_document_quarter_turns() {
    let mut d = mixed_doc();
    let bg = testing::raster(&d, "Background").clone();
    let v = run("transform rotate", &[("degrees", "90")], &mut d).unwrap();
    assert_eq!(v["size"], json!([80, 100]));
    // pixel (x, y) goes to (H-1-y, x)
    assert_eq!(testing::raster(&d, "Background").get(79, 0), bg.get(0, 0));
    assert_eq!(testing::raster(&d, "Background").get(0, 99), bg.get(99, 79));
    let pic = layer(&d, "Pic");
    assert_eq!((pic.x, pic.y, pic.raster().unwrap().width, pic.raster().unwrap().height), (50, 30, 10, 20));
    let t = spec_of(&d, "Title");
    assert_eq!((t["x"].as_f64(), t["y"].as_f64(), t["rotation"].as_f64()), (Some(30.0), Some(40.0), Some(90.0)));
    assert_eq!(path_bbox(&d, "Box"), (50.0, 10.0, 70.0, 30.0));
    assert_eq!(placed_rect(&d, "Placed"), ([60.0, 55.0, 20.0, 10.0], 90.0));
    assert_eq!((d.guides[0].orientation.as_str(), d.guides[0].pos), ("h", 50.0));
    assert_eq!((d.guides[1].orientation.as_str(), d.guides[1].pos), ("v", 40.0));
    assert_eq!((sel(&d).width, sel(&d).height), (80, 100));
    for _ in 0..3 {
        run("transform rotate", &[("degrees", "90")], &mut d).unwrap();
    }
    let fresh = mixed_doc();
    assert_eq!((d.width, d.height), (100, 80));
    assert_eq!(testing::raster(&d, "Background"), &bg);
    assert_eq!(sel(&d), sel(&fresh));
    assert_eq!(layer(&d, "Pic").mask.as_ref().unwrap().mask, layer(&fresh, "Pic").mask.as_ref().unwrap().mask);
    assert_eq!(spec_of(&d, "Title")["rotation"].as_f64(), Some(0.0));
    assert_eq!(placed_rect(&d, "Placed"), ([50.0, 5.0, 20.0, 10.0], 0.0));
    run("transform rotate", &[("degrees", "180")], &mut d).unwrap();
    assert_eq!(testing::raster(&d, "Background").get(0, 0), bg.get(99, 79));
}

#[test]
fn rotate_document_arbitrary_with_expand() {
    let mut d = testing::doc_from(white(100, 50));
    d.selection = Some(Mask::filled(100, 50, 1.0));
    let v = run("transform rotate", &[("degrees", "30"), ("expand", "true")], &mut d).unwrap();
    let (w, h) = (
        100.0 * 30f64.to_radians().cos() + 50.0 * 30f64.to_radians().sin(),
        100.0 * 30f64.to_radians().sin() + 50.0 * 30f64.to_radians().cos(),
    );
    assert_eq!(v["size"], json!([w.ceil() as u32, h.ceil() as u32]));
    assert_eq!(sel(&d).get(0, 0), 0.0, "corners are outside the old canvas");
    assert!(sel(&d).get(d.width / 2, d.height / 2) > 0.99);
    let mut k = testing::doc_from(white(100, 50));
    run("transform rotate", &[("degrees", "30")], &mut k).unwrap();
    assert_eq!((k.width, k.height), (100, 50));
}

#[test]
fn flip_document_twice_is_identity() {
    let mut d = mixed_doc();
    let p = d.resolve("Title").unwrap();
    d.remove(&p);
    let p = d.resolve("Placed").unwrap();
    d.remove(&p);
    let fresh = d.clone();
    run("transform flip", &[("h", "true"), ("v", "true")], &mut d).unwrap();
    assert_eq!(testing::raster(&d, "Background").get(0, 0), testing::raster(&fresh, "Background").get(99, 79));
    assert_eq!((d.guides[0].pos, d.guides[1].pos), (50.0, 40.0));
    run("transform flip", &[("h", "true"), ("v", "true")], &mut d).unwrap();
    assert_eq!(testing::raster(&d, "Background"), testing::raster(&fresh, "Background"));
    assert_eq!(testing::raster(&d, "Pic"), testing::raster(&fresh, "Pic"));
    assert_eq!((layer(&d, "Pic").x, layer(&d, "Pic").y), (30, 20));
    assert_eq!(sel(&d), sel(&fresh));
    assert_eq!(path_bbox(&d, "Box"), path_bbox(&fresh, "Box"));
}

#[test]
fn canvas_grows_with_anchor_and_fill() {
    let mut d = mixed_doc();
    run("transform canvas", &[("size", "+20x+10"), ("anchor", "tl")], &mut d).unwrap();
    assert_eq!((d.width, d.height, layer(&d, "Pic").x), (120, 90, 30));
    assert_eq!((sel(&d).width, sel(&d).height), (120, 90));
    let mut d = mixed_doc();
    let bg = testing::raster(&d, "Background").clone();
    let v = run("transform canvas", &[("size", "+20x+10"), ("fill", "white")], &mut d).unwrap();
    assert_eq!(v["offset"], json!([10.0, 5.0]));
    assert_eq!((layer(&d, "Pic").x, layer(&d, "Pic").y), (40, 25));
    let b = testing::raster(&d, "Background");
    assert_eq!((b.width, b.height), (120, 90));
    assert_eq!(b.get(0, 0), [1.0; 4]);
    assert_eq!(b.get(10, 5), bg.get(0, 0));
    assert_eq!(spec_of(&d, "Title")["x"].as_f64(), Some(50.0));
    assert_eq!(d.guides[0].pos, 60.0);
    // the layer mask's reveal-all border extends as revealed
    let mut r = testing::doc_from(white(10, 10));
    r.layers[0].mask = Some(LayerMask::new(Mask::filled(10, 10, 1.0)));
    run("transform canvas", &[("size", "20x20")], &mut r).unwrap();
    assert_eq!(r.layers[0].mask.as_ref().unwrap().mask.get(0, 0), 1.0);
    // absolute and shrinking sizes, bad specs
    run("transform canvas", &[("size", "50%x")], &mut r).unwrap();
    assert_eq!((r.width, r.height), (10, 20));
    assert_eq!(code(run("transform canvas", &[("size", "-30x")], &mut r)), "PX1013");
    assert_eq!(code(run("transform canvas", &[("size", "big")], &mut r)), "PX1011");
    // a non-raster bottom layer gets a fill layer
    let mut s = Document::new(10, 10);
    let t = s.new_layer("T", LayerKind::Text { spec: json!({"content": "x", "x": 0, "y": 0}) });
    s.insert_above(None, t);
    run("transform canvas", &[("size", "+4x+4"), ("fill", "black")], &mut s).unwrap();
    assert_eq!(s.layers[0].name, "Canvas Fill");
    assert_eq!(testing::raster(&s, "Canvas Fill").get(0, 0), [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(testing::raster(&s, "Canvas Fill").get(5, 5)[3], 0.0);
}

#[test]
fn resize_scales_everything() {
    let mut d = mixed_doc();
    let v = run("transform resize", &[("size", "50%")], &mut d).unwrap();
    assert_eq!(v["size"], json!([50, 40]));
    assert_eq!(v["interp"], json!("lanczos3"));
    let pic = layer(&d, "Pic");
    assert_eq!((pic.x, pic.y, pic.raster().unwrap().width, pic.raster().unwrap().height), (15, 10, 10, 5));
    let t = spec_of(&d, "Title");
    assert_eq!((t["x"].as_f64(), t["y"].as_f64(), t["size"].clone()), (Some(20.0), Some(25.0), json!("10px")));
    assert_eq!(path_bbox(&d, "Box"), (5.0, 5.0, 15.0, 15.0));
    assert_eq!(spec_of(&d, "Box")["stroke_width"], json!(2.0));
    assert_eq!(placed_rect(&d, "Placed").0, [25.0, 2.5, 10.0, 5.0]);
    assert_eq!((layer(&d, "Inner").x, layer(&d, "Inner").raster().unwrap().width), (30, 5));
    assert_eq!((sel(&d).width, d.masks["m"].width, layer(&d, "Pic").mask.as_ref().unwrap().mask.height), (50, 50, 40));
    assert!((area(sel(&d)) - 100.0).abs() < 1.0);
    assert_eq!((d.guides[0].pos, d.guides[1].pos), (25.0, 20.0));
    let mut g = testing::doc_from(white(40, 20));
    g.grid = Some(pixelox_core::doc::GridSpec { size: [10.0, 4.0], visible: true });
    run("transform resize", &[("size", "20x")], &mut g).unwrap();
    assert_eq!(g.grid.as_ref().unwrap().size, [5.0, 2.0]);
    run("transform rotate", &[("degrees", "90")], &mut g).unwrap();
    assert_eq!(g.grid.as_ref().unwrap().size, [2.0, 5.0]);
    let mut c = testing::doc_from(testing::checker(16, 16));
    let src = testing::raster(&c, "Background").clone();
    run("transform resize", &[("size", "32x"), ("interp", "nearest")], &mut c).unwrap();
    let big = testing::raster(&c, "Background");
    assert_eq!((big.width, big.height), (32, 32));
    assert_eq!((big.get(15, 15), big.get(16, 16)), (src.get(7, 7), src.get(8, 8)));
    let mut s = testing::doc_from(testing::checker(64, 64));
    let r = run_io("transform resize", &[("size", "32x32"), ("sharpen", "auto")], &mut s, None).unwrap();
    assert_eq!(r.resolved["sharpen"], json!(0.3));
}

#[test]
fn trim_borders() {
    let img = Image::from_fn(50, 40, |x, y| {
        if (5..25).contains(&x) && (4..14).contains(&y) {
            [0.2, 0.3, 0.4, 1.0]
        } else {
            [1.0; 4]
        }
    });
    let mut d = testing::doc_from(img);
    let v = run("transform trim", &[], &mut d).unwrap();
    assert_eq!((v["rect"].clone(), d.width, d.height), (json!([5, 4, 20, 10]), 20, 10));
    let v = run("transform trim", &[], &mut d).unwrap();
    assert_eq!(v["trimmed"], json!(false));
    let mut t = testing::doc_from(Image::from_fn(30, 30, |x, y| {
        if (10..12).contains(&x) && (3..20).contains(&y) {
            [1.0; 4]
        } else {
            [0.0; 4]
        }
    }));
    run("transform trim", &[], &mut t).unwrap();
    assert_eq!((t.width, t.height), (2, 17));
    let mut u = testing::doc_from(white(8, 8));
    assert_eq!(run("transform trim", &[], &mut u).unwrap()["trimmed"], json!(false));
}

/// Horizontal stripes (with a few vertical bars) rotated clockwise by `deg`.
fn tilted(w: u32, h: u32, deg: f64) -> Image {
    let (s, c) = deg.to_radians().sin_cos();
    let (cx, cy) = (w as f64 / 2.0, h as f64 / 2.0);
    Image::from_fn(w, h, |x, y| {
        let (dx, dy) = (x as f64 + 0.5 - cx, y as f64 + 0.5 - cy);
        let v = -dx * s + dy * c;
        let u = dx * c + dy * s;
        let on = (v.rem_euclid(24.0) < 8.0) ^ (u.rem_euclid(64.0) < 4.0);
        let g = if on { 0.9 } else { 0.1 };
        [g, g, g, 1.0]
    })
}

#[test]
fn straighten_detects_records_and_crops() {
    let mut d = testing::doc_from(tilted(240, 180, 4.0));
    let r = run_io("transform straighten", &[], &mut d, None).unwrap();
    let applied = r.resolved["angle"].as_f64().unwrap();
    assert!((applied + 4.0).abs() < 0.3, "applied {applied}");
    assert_eq!(r.value["angle"].as_f64(), Some(applied));
    assert!(r.value["detected"]["tilt"].as_f64().is_some());
    assert!(d.width < 240 && d.height < 180 && d.width > 200 && d.height > 150, "{}x{}", d.width, d.height);
    // the result is level: detection now finds ~0
    let again = crate::straighten::detect_tilt(&compose::composite(&d).unwrap()).unwrap();
    assert!(again.degrees.abs() < 0.4, "{}", again.degrees);
    let mut k = testing::doc_from(tilted(120, 90, 0.0));
    let r = run_io("transform straighten", &[("angle", "2"), ("crop", "none")], &mut k, None).unwrap();
    assert!(r.resolved.is_empty(), "explicit angles are not re-recorded");
    assert_eq!((k.width, k.height), (120, 90));
    assert_eq!(code(run("transform straighten", &[("angle", "60")], &mut k)), "PX1013");
    let mut flat = testing::doc_from(white(32, 32));
    let r = run_io("transform straighten", &[], &mut flat, None).unwrap();
    assert_eq!((r.value["angle"].as_f64(), flat.width), (Some(0.0), 32));
    assert!(r.warnings.is_empty());
}
