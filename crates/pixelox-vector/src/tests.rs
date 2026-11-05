//! Behavior tests: shapes render where expected, booleans obey set laws, SVG edits round
//! trip through the parser, rasterize/trace/from-doc produce the expected pixels, and every
//! op honors the authoring contract (summaries, help, examples).

use crate::xml::{Selector, XDoc};
use crate::{ops, pathops, spec, svg};
use pixelox_core::ctx::{Globals, ImageInfo, ImageIo, NoProgress, OpenOptions};
use pixelox_core::envelope::Warning;
use pixelox_core::prelude::*;
use pixelox_core::testing::{self, find, TempDir};
use pixelox_core::Runner;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------------------
// harness

/// PNG-only IO through tiny-skia (enough for rasterize/trace tests).
struct TestIo;

fn decode_png(bytes: &[u8]) -> Result<Image> {
    let pm = tiny_skia::Pixmap::decode_png(bytes).map_err(|e| Error::new("PX1103", e.to_string()))?;
    Ok(svg::pixmap_to_image(&pm))
}

impl ImageIo for TestIo {
    fn open(&self, path: &Path, _: &OpenOptions) -> Result<(Document, Vec<Warning>)> {
        Ok((Document::from_image(decode_png(&std::fs::read(path)?)?), vec![]))
    }
    fn decode(&self, bytes: &[u8], _: Option<&str>, _: &OpenOptions) -> Result<(Image, ImageInfo)> {
        Ok((decode_png(bytes)?, ImageInfo::default()))
    }
    fn probe(&self, _: &Path) -> Result<ImageInfo> {
        Ok(ImageInfo::default())
    }
    fn encode(&self, image: &Image, _: Option<&Document>, path: &Path, _: &EncodeOptions) -> Result<Vec<Warning>> {
        std::fs::write(path, crate::fromdoc::encode_png(image)?)?;
        Ok(vec![])
    }
    fn save_pxo(&self, _: &Document, _: &Path) -> Result<()> {
        Err(Error::new("PX1106", "no .pxo in tests"))
    }
    fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["png"], vec!["png"])
    }
}

/// Run a custom op with inputs, `-o`, and `--in-place`.
fn custom(name: &str, inputs: &[&Path], out: Option<&Path>, in_place: bool, pairs: &[(&str, &str)]) -> Result<Value> {
    let spec = find(&ops(), name);
    let a = testing::args(&spec, pairs)?;
    let g = Globals { output: out.map(|p| p.display().to_string()), in_place, ..Default::default() };
    let mut ctx = OpCtx::new(spec.name, inputs.iter().map(|p| p.to_path_buf()).collect(), g, &TestIo, &NoProgress);
    match spec.runner {
        Runner::Custom(f) => f(&a, &mut ctx),
        _ => panic!("{name} is not a custom op"),
    }
}

fn shape_op(doc: &mut Document, name: &str, pairs: &[(&str, &str)]) -> Value {
    testing::run(&find(&ops(), name), pairs, doc).unwrap_or_else(|e| panic!("{name}: {e}"))
}

fn canvas(w: u32, h: u32) -> Document {
    crate::install();
    Document::new(w, h)
}

fn px(img: &Image, x: u32, y: u32) -> [f32; 4] {
    img.get(x, y)
}

fn close(a: [f32; 4], b: [f32; 4]) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() < 0.02)
}

const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const CLEAR: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

fn opaque_count(img: &Image) -> usize {
    img.data.chunks(4).filter(|p| p[3] > 0.5).count()
}

fn spec_of<'a>(doc: &'a Document, layer: &str) -> &'a Value {
    match &doc.layer(&doc.resolve(layer).unwrap()).content {
        LayerKind::Shape { spec } => spec,
        _ => panic!("{layer} is not a shape"),
    }
}

fn read(p: &Path) -> String {
    std::fs::read_to_string(p).unwrap()
}

fn write(dir: &TempDir, name: &str, text: &str) -> PathBuf {
    let p = dir.path(name);
    std::fs::write(&p, text).unwrap();
    p
}

fn rasterize_text(svg_text: &str, w: u32, h: u32) -> Image {
    let tree = usvg::Tree::from_str(svg_text, &svg::usvg_options(false)).unwrap();
    svg::render_svg(&tree, w, h, None).unwrap()
}

// ---------------------------------------------------------------------------------------
// contract

const GLOBAL_VALUE: &[&str] =
    &["-o", "--output", "--seed", "--threads", "--mem-budget", "--progress", "--config", "--format", "--limit-pixels"];
const GLOBAL_FLAG: &[&str] = &[
    "--json",
    "--dry-run",
    "--overwrite",
    "--in-place",
    "-y",
    "--yes",
    "-q",
    "--quiet",
    "-v",
    "--verbose",
    "--no-sidecar",
    "--background",
    "--redact-paths",
];
const RESERVED: &[&str] = &[
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

fn shell_words(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quote: Option<char> = None;
    let mut started = false;
    for c in s.chars() {
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            (Some(_), c) => cur.push(c),
            (None, '\'' | '"') => {
                quote = Some(c);
                started = true;
            }
            (None, c) if c.is_whitespace() => {
                if started || !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                    started = false;
                }
            }
            (None, c) => cur.push(c),
        }
    }
    if started || !cur.is_empty() {
        out.push(cur);
    }
    out
}

#[test]
fn every_op_meets_the_authoring_contract() {
    let all = ops();
    let names: Vec<&str> = all.iter().map(|o| o.name).collect();
    for want in [
        "shape rect",
        "shape ellipse",
        "shape polygon",
        "shape star",
        "shape line",
        "shape arrow",
        "shape path",
        "shape set",
        "path unite",
        "path subtract",
        "path intersect",
        "path exclude",
        "path simplify",
        "path outline-stroke",
        "svg new",
        "svg el add",
        "svg el set",
        "svg el rm",
        "svg defs add",
        "svg use",
        "svg query",
        "svg optimize",
        "svg rasterize",
        "svg from-doc",
        "trace",
    ] {
        assert!(names.contains(&want), "missing {want}");
    }
    let mut seen = std::collections::BTreeSet::new();
    for op in &all {
        assert!(seen.insert(op.name), "duplicate {}", op.name);
        assert!(
            op.summary.chars().count() <= 60 && !op.summary.ends_with('.'),
            "summary of {}: {}",
            op.name,
            op.summary
        );
        assert!(!op.about.is_empty(), "{} has no about", op.name);
        assert!(op.examples.len() >= 3, "{} has {} examples", op.name, op.examples.len());
        for p in &op.params {
            assert!(!p.help.is_empty(), "{} --{} has no help", op.name, p.name);
            assert!(!RESERVED.contains(&p.name), "{} declares reserved --{}", op.name, p.name);
            if let Some(d) = p.default {
                p.validate(d).unwrap_or_else(|e| panic!("{} --{} default: {e}", op.name, p.name));
            }
        }
        for ex in &op.examples {
            let words = shell_words(ex.cmd);
            let head = format!("pixelox {}", op.name);
            assert!(ex.cmd.starts_with(&format!("{head} ")), "example of {}: {}", op.name, ex.cmd);
            let mut i = op.name.split(' ').count() + 1;
            let mut positional = 0;
            while i < words.len() {
                let w = &words[i];
                if GLOBAL_FLAG.contains(&w.as_str()) {
                    i += 1;
                } else if GLOBAL_VALUE.contains(&w.as_str()) {
                    i += 2;
                } else if let Some(flag) = w.strip_prefix("--") {
                    let p = op
                        .find_param(flag)
                        .unwrap_or_else(|| panic!("{}: unknown flag --{flag} in `{}`", op.name, ex.cmd));
                    if p.kind == ParamKind::Flag {
                        i += 1;
                    } else {
                        let v = words.get(i + 1).unwrap_or_else(|| panic!("{}: --{flag} needs a value", op.name));
                        p.validate(v).unwrap_or_else(|e| panic!("{}: `{}`: {e}", op.name, ex.cmd));
                        i += 2;
                    }
                } else {
                    positional += 1;
                    i += 1;
                }
            }
            assert!(positional >= op.inputs.min, "{}: too few inputs in `{}`", op.name, ex.cmd);
            if let Some(max) = op.inputs.max {
                assert!(positional <= max, "{}: too many inputs in `{}`", op.name, ex.cmd);
            }
            let req: Vec<&str> = op.params.iter().filter(|p| p.required).map(|p| p.name).collect();
            for r in req {
                assert!(words.iter().any(|w| w == &format!("--{r}")), "{}: `{}` lacks required --{r}", op.name, ex.cmd);
            }
        }
    }
}

#[test]
fn install_registers_the_shape_renderer() {
    crate::install();
    assert!(compose::installed_renderers().contains(&"shape"));
}

// ---------------------------------------------------------------------------------------
// shapes

#[test]
fn rect_renders_inside_its_box_only() {
    let mut d = canvas(100, 100);
    let r = shape_op(&mut d, "shape rect", &[("rect", "10,10,30x20"), ("fill", "red")]);
    assert_eq!(r["layer"], "Rectangle");
    assert_eq!(r["bbox"], json!([10.0, 10.0, 30.0, 20.0]));
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 10, 10), RED) && close(px(&c, 39, 29), RED) && close(px(&c, 20, 20), RED));
    assert!(close(px(&c, 9, 15), CLEAR) && close(px(&c, 40, 15), CLEAR) && close(px(&c, 20, 30), CLEAR));
    assert_eq!(opaque_count(&c), 600);
    let s = spec_of(&d, "Rectangle");
    assert_eq!(s["geometry"]["type"], "rect");
    assert_eq!(s["geometry"]["rect"], "10,10,30x20");
    assert_eq!(s["stroke"], Value::Null);
}

#[test]
fn rounded_rect_cuts_corners_and_default_fill_is_black() {
    let mut d = canvas(100, 100);
    shape_op(&mut d, "shape rect", &[("rect", "0,0,100x100"), ("radius", "30")]);
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 1, 1), CLEAR), "corner should be cut");
    assert!(close(px(&c, 50, 50), BLACK) && close(px(&c, 50, 1), BLACK));
    // per-corner radii: only the top-left is rounded
    let mut d = canvas(100, 100);
    shape_op(&mut d, "shape rect", &[("rect", "0,0,100x100"), ("radius", "30,0,0,0")]);
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 1, 1), CLEAR) && close(px(&c, 98, 1), BLACK) && close(px(&c, 1, 98), BLACK));
}

#[test]
fn ellipse_polygon_star_and_path_fill_where_expected() {
    let mut d = canvas(100, 100);
    shape_op(&mut d, "shape ellipse", &[("rect", "20,20,60x60"), ("fill", "#0000ff")]);
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 50, 50), [0.0, 0.0, 1.0, 1.0]) && close(px(&c, 50, 22), [0.0, 0.0, 1.0, 1.0]));
    assert!(close(px(&c, 22, 22), CLEAR));
    let area = opaque_count(&c) as f64;
    assert!((area - std::f64::consts::PI * 900.0).abs() / (std::f64::consts::PI * 900.0) < 0.02, "{area}");

    let mut d = canvas(100, 100);
    shape_op(&mut d, "shape polygon", &[("center", "50,50"), ("radius", "40"), ("sides", "4")]);
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 50, 50), BLACK) && close(px(&c, 50, 13), BLACK) && close(px(&c, 15, 15), CLEAR));
    assert!((opaque_count(&c) as f64 - 3200.0).abs() < 80.0);

    let mut d = canvas(100, 100);
    shape_op(&mut d, "shape star", &[("points", "5"), ("outer", "40"), ("inner", "15")]);
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 50, 50), BLACK) && close(px(&c, 50, 14), BLACK), "tip up");
    assert!(close(px(&c, 50, 72), CLEAR), "valley straight down");

    let mut d = canvas(100, 100);
    shape_op(&mut d, "shape path", &[("d", "M 10 90 L 50 10 L 90 90 Z"), ("fill", "red")]);
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 50, 70), RED) && close(px(&c, 15, 20), CLEAR));
    // even-odd ring
    let mut d = canvas(100, 100);
    shape_op(
        &mut d,
        "shape path",
        &[("d", "M 0 0 H 100 V 100 H 0 Z M 25 25 H 75 V 75 H 25 Z"), ("fill-rule", "evenodd")],
    );
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 50, 50), CLEAR) && close(px(&c, 10, 10), BLACK));
}

#[test]
fn line_dash_and_arrow_render_strokes() {
    let mut d = canvas(100, 100);
    let r = shape_op(&mut d, "shape line", &[("from", "10,50"), ("to", "90,50"), ("stroke-width", "4")]);
    assert_eq!(r["stroke"], "#000000");
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 50, 49), BLACK) && close(px(&c, 50, 50), BLACK));
    assert!(close(px(&c, 50, 53), CLEAR) && close(px(&c, 8, 50), CLEAR), "butt caps");
    assert!((opaque_count(&c) as f64 - 320.0).abs() < 10.0);

    let mut d = canvas(100, 100);
    shape_op(
        &mut d,
        "shape line",
        &[("from", "10,50"), ("to", "90,50"), ("stroke-width", "4"), ("dash", "10,10"), ("stroke", "red")],
    );
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 15, 50), RED) && close(px(&c, 25, 50), CLEAR) && close(px(&c, 35, 50), RED));

    let mut d = canvas(100, 100);
    let r = shape_op(&mut d, "shape arrow", &[("from", "10,50"), ("to", "90,50")]);
    assert_eq!(r["geometry"]["head_size"], 8.0);
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 40, 50), BLACK) && close(px(&c, 40, 53), CLEAR), "shaft");
    assert!(close(px(&c, 84, 52), BLACK) && close(px(&c, 88, 50), BLACK), "filled head");
    let mut d = canvas(100, 100);
    shape_op(&mut d, "shape arrow", &[("from", "10,50"), ("to", "90,50"), ("tail-head", "true"), ("stroke", "red")]);
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 16, 52), RED), "tail head");
}

#[test]
fn fill_and_stroke_composite_and_opacity_applies() {
    let mut d = canvas(100, 100);
    shape_op(
        &mut d,
        "shape rect",
        &[("rect", "20,20,60x60"), ("fill", "red"), ("stroke", "#0000ff"), ("stroke-width", "4"), ("opacity", "50")],
    );
    let c = compose::composite(&d).unwrap();
    let p = px(&c, 50, 50);
    assert!((p[0] - 1.0).abs() < 0.01 && (p[3] - 0.5).abs() < 0.01);
    let e = px(&c, 19, 50);
    assert!((e[2] - 1.0).abs() < 0.01 && e[0] < 0.01 && (e[3] - 0.5).abs() < 0.01, "{e:?}");
}

#[test]
fn shapes_stack_above_the_active_layer() {
    let mut d = testing::doc_layers(50, 50, vec![("Bottom", Image::new(50, 50)), ("Top", Image::new(50, 50))]);
    crate::install();
    d.active = Some(d.layer(&d.resolve("Bottom").unwrap()).id.clone());
    shape_op(&mut d, "shape rect", &[("rect", "0,0,10x10"), ("name", "Box")]);
    assert_eq!(d.layers[1].name, "Box");
    assert_eq!(d.active.as_deref(), Some(d.layers[1].id.as_str()));
    let e = testing::run(&find(&ops(), "shape rect"), &[("rect", "0,0,10x10"), ("name", "Box")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX2305");
}

#[test]
fn shape_set_restyles_and_rejects_other_kinds() {
    let mut d = canvas(60, 60);
    shape_op(&mut d, "shape ellipse", &[("rect", "0,0,60x60"), ("fill", "red")]);
    let r = shape_op(
        &mut d,
        "shape set",
        &[("layer", "Ellipse"), ("fill", "none"), ("stroke", "#00ff00"), ("stroke-width", "6"), ("name", "Ring")],
    );
    assert_eq!(r["changed"], json!(["fill", "stroke", "stroke_width", "name"]));
    let s = spec_of(&d, "Ring");
    assert_eq!(s["fill"], Value::Null);
    assert_eq!(s["stroke"], "#00ff00");
    assert_eq!(s["geometry"]["type"], "ellipse");
    let c = compose::composite(&d).unwrap();
    assert!(close(px(&c, 30, 30), CLEAR) && close(px(&c, 30, 1), [0.0, 1.0, 0.0, 1.0]));
    let e = testing::run(&find(&ops(), "shape set"), &[("layer", "Ring")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX1003");
    let mut r = testing::doc_from(testing::solid(4, 4, Rgba::WHITE));
    let e = testing::run(&find(&ops(), "shape set"), &[("fill", "red")], &mut r).unwrap_err();
    assert_eq!(e.code, "PX2303");
    let p = d.resolve("Ring").unwrap();
    d.layer_mut(&p).locked = true;
    let e = testing::run(&find(&ops(), "shape set"), &[("layer", "Ring"), ("fill", "red")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX2304");
}

#[test]
fn invalid_paint_and_path_data_fail_cleanly() {
    let mut d = canvas(10, 10);
    let e = testing::run(&find(&ops(), "shape rect"), &[("rect", "0,0,5x5"), ("fill", "nope")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX1012");
    let e = testing::run(&find(&ops(), "shape path"), &[("d", "M 0 0 Q")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX1010");
    let e = testing::run(&find(&ops(), "shape rect"), &[("rect", "0,0,5x5"), ("dash", "1,-2")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX1010");
}

#[test]
fn shapes_outside_the_canvas_render_nothing_and_1x1_works() {
    let mut d = canvas(10, 10);
    shape_op(&mut d, "shape rect", &[("rect", "50,50,10x10")]);
    assert_eq!(opaque_count(&compose::composite(&d).unwrap()), 0);
    let mut d = canvas(1, 1);
    shape_op(&mut d, "shape rect", &[("rect", "0,0,1x1"), ("fill", "red")]);
    assert!(close(px(&compose::composite(&d).unwrap(), 0, 0), RED));
}

// ---------------------------------------------------------------------------------------
// path ops

fn two_squares() -> Document {
    let mut d = canvas(40, 30);
    shape_op(&mut d, "shape path", &[("d", "M 10 10 H 20 V 20 H 10 Z"), ("name", "A"), ("fill", "red")]);
    shape_op(&mut d, "shape path", &[("d", "M 15 10 H 25 V 20 H 15 Z"), ("name", "B")]);
    d
}

fn area_of(op: &str) -> f64 {
    let mut d = two_squares();
    let r = shape_op(&mut d, op, &[("layers", "A,B")]);
    r["area"].as_f64().unwrap()
}

#[test]
fn booleans_obey_set_laws_on_squares() {
    let (u, i, s, x) =
        (area_of("path unite"), area_of("path intersect"), area_of("path subtract"), area_of("path exclude"));
    let near = |a: f64, b: f64| (a - b).abs() < 1e-6;
    assert!(near(u, 150.0) && near(i, 50.0) && near(s, 50.0) && near(x, 100.0), "{u} {i} {s} {x}");
    assert!(near(u + i, 100.0 + 100.0), "inclusion-exclusion");
    assert!(near(s, 100.0 - i), "A - B = A - (A ∩ B)");
    assert!(near(x, u - i), "A ⊕ B = (A ∪ B) - (A ∩ B)");
    // B − A has the same area by symmetry
    let mut d = two_squares();
    let r = shape_op(&mut d, "path subtract", &[("layers", "B,A")]);
    assert!(near(r["area"].as_f64().unwrap(), 50.0));
}

#[test]
fn boolean_replaces_a_or_writes_into_a_new_layer() {
    let mut d = two_squares();
    let r = shape_op(&mut d, "path unite", &[("layers", "A,B")]);
    assert_eq!(r["removed"], json!(["B"]));
    assert_eq!(d.layer_count(), 1);
    let s = spec_of(&d, "A");
    assert_eq!(s["fill"], "#ff0000");
    assert_eq!(s["geometry"]["op"], "unite");
    let c = compose::composite(&d).unwrap();
    assert_eq!(opaque_count(&c), 150);
    assert!(close(px(&c, 22, 15), RED));

    let mut d = two_squares();
    let r = shape_op(&mut d, "path intersect", &[("layers", "A,B"), ("into", "Lens")]);
    assert_eq!(r["layer"], "Lens");
    assert_eq!(d.layer_count(), 3);
    assert_eq!(d.layers.last().unwrap().name, "Lens");
    let only = compose::composite_subset(&d, &[d.layers[2].id.clone()]).unwrap();
    assert_eq!(opaque_count(&only), 50);

    // disjoint intersect is empty but valid
    let mut d = canvas(40, 40);
    shape_op(&mut d, "shape rect", &[("rect", "0,0,5x5"), ("name", "P")]);
    shape_op(&mut d, "shape rect", &[("rect", "20,20,5x5"), ("name", "Q")]);
    let r = shape_op(&mut d, "path intersect", &[("layers", "P,Q")]);
    assert_eq!(r["empty"], true);
    assert_eq!(opaque_count(&compose::composite(&d).unwrap()), 0);
}

#[test]
fn boolean_checks_operands() {
    let mut d = two_squares();
    let e = testing::run(&find(&ops(), "path unite"), &[("layers", "A")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX1003");
    let e = testing::run(&find(&ops(), "path unite"), &[("layers", "A,A")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX1010");
    let bg = d.new_layer("Pixels", LayerKind::Raster { image: Image::new(4, 4) });
    d.insert_above(None, bg);
    let e = testing::run(&find(&ops(), "path unite"), &[("layers", "A,Pixels")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX2303");
}

#[test]
fn curved_booleans_are_accurate_to_the_flattening_tolerance() {
    let mut d = canvas(200, 200);
    shape_op(&mut d, "shape ellipse", &[("rect", "20,20,100x100"), ("name", "C1")]);
    shape_op(&mut d, "shape ellipse", &[("rect", "70,20,100x100"), ("name", "C2")]);
    let i = shape_op(&mut d, "path intersect", &[("layers", "C1,C2"), ("into", "I")])["area"].as_f64().unwrap();
    // lens of two r=50 circles at distance 50: 2r²acos(d/2r) − (d/2)√(4r²−d²)
    let (r, dd) = (50.0f64, 50.0f64);
    let lens = 2.0 * r * r * (dd / (2.0 * r)).acos() - dd / 2.0 * (4.0 * r * r - dd * dd).sqrt();
    assert!((i - lens).abs() / lens < 0.01, "{i} vs {lens}");
}

#[test]
fn simplify_reduces_anchors_within_tolerance() {
    let mut d = canvas(100, 100);
    shape_op(&mut d, "shape path", &[("d", "M 0 0 L 10 0 L 20 0 L 30 0 L 30 30 L 0 30 Z"), ("name", "Poly")]);
    let r = shape_op(&mut d, "path simplify", &[("layer", "Poly"), ("tolerance", "0.5")]);
    assert_eq!((r["anchors_before"].as_u64(), r["anchors_after"].as_u64()), (Some(6), Some(4)));
    // a densely sampled circle keeps its area
    let pts: Vec<String> = (0..128)
        .map(|i| {
            let a = i as f64 / 128.0 * std::f64::consts::TAU;
            format!("{} {}", 50.0 + 40.0 * a.cos(), 50.0 + 40.0 * a.sin())
        })
        .collect();
    let dd = format!("M {} Z", pts.join(" L "));
    let mut d = canvas(100, 100);
    shape_op(&mut d, "shape path", &[("d", &dd), ("name", "Circle")]);
    let before = opaque_count(&compose::composite(&d).unwrap()) as f64;
    let r = shape_op(&mut d, "path simplify", &[("layer", "Circle"), ("tolerance", "0.5")]);
    assert!(r["anchors_after"].as_u64().unwrap() < 64, "{r}");
    let after = opaque_count(&compose::composite(&d).unwrap()) as f64;
    assert!((after - before).abs() / before < 0.02, "{before} {after}");
    // curves: a cubic run splits/merges but stays close
    let p = spec::parse_d("M 0 0 C 10 0 20 0 30 0 C 40 0 50 0 60 0").unwrap();
    assert!(spec::anchor_count(&pathops::simplify_path(&p, 0.5)) <= spec::anchor_count(&p));
}

#[test]
fn outline_stroke_matches_the_stroked_area() {
    let mut d = canvas(100, 100);
    shape_op(&mut d, "shape line", &[("from", "10,50"), ("to", "90,50"), ("stroke-width", "10"), ("name", "L")]);
    let before = compose::composite(&d).unwrap();
    let r = shape_op(&mut d, "path outline-stroke", &[("layer", "L")]);
    assert_eq!(r["replaced"], true);
    let s = spec::ShapeSpec::from_value(spec_of(&d, "L")).unwrap();
    assert!(s.stroke.is_none() && s.fill.is_some());
    let area = pathops::shapes_area(&pathops::normalize(&s).unwrap());
    assert!((area - 800.0).abs() < 1.0, "{area}");
    let after = compose::composite(&d).unwrap();
    assert!(testing::max_diff(&before, &after) < 0.05);
    // with a fill, the stroke becomes a second layer and the look is kept
    let mut d = canvas(100, 100);
    shape_op(
        &mut d,
        "shape rect",
        &[("rect", "20,20,60x60"), ("fill", "red"), ("stroke", "blue"), ("stroke-width", "6"), ("name", "R")],
    );
    let before = compose::composite(&d).unwrap();
    let r = shape_op(&mut d, "path outline-stroke", &[("layer", "R")]);
    assert_eq!(r["layer"], "R Stroke");
    assert_eq!(d.layer_count(), 2);
    assert!(testing::max_diff(&before, &compose::composite(&d).unwrap()) < 0.05);
    let e = testing::run(&find(&ops(), "path outline-stroke"), &[("layer", "R")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX1010", "no stroke left");
}

// ---------------------------------------------------------------------------------------
// svg

const BASE: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="50" height="40" viewBox="0 0 50 40">
  <rect id="box" x="10" y="10" width="20" height="10" fill="#ff0000"/>
</svg>
"##;

#[test]
fn svg_new_writes_a_parseable_document() {
    let dir = TempDir::new("vec-new");
    let out = dir.path("n.svg");
    let r = custom("svg new", &[], Some(&out), false, &[("size", "320x200")]).unwrap();
    assert_eq!(r["viewbox"], "0 0 320 200");
    let d = XDoc::parse_svg(&read(&out)).unwrap();
    assert_eq!(d.root.get("width"), Some("320"));
    assert!(read(&out).starts_with("<?xml"));
    let e = custom("svg new", &[], None, false, &[]).unwrap_err();
    assert_eq!(e.code, "PX1004");
    let e = custom("svg new", &[], Some(&dir.path("x.svg")), false, &[("viewbox", "0 0 -1 5")]).unwrap_err();
    assert_eq!(e.code, "PX1010");
    let e = custom("svg new", &[], Some(&out), false, &[]).unwrap_err();
    assert_eq!(e.code, "PX1401", "no overwrite without --overwrite");
}

#[test]
fn svg_el_add_set_rm_round_trip() {
    let dir = TempDir::new("vec-el");
    let f = write(&dir, "f.svg", BASE);
    // add (in place), with a parent group
    custom("svg el add", &[&f], None, true, &[("kind", "g"), ("id", "grp"), ("attr", "class=axis")]).unwrap();
    let r = custom(
        "svg el add",
        &[&f],
        None,
        true,
        &[
            ("kind", "circle"),
            ("parent", "grp"),
            ("attr", "cx=5"),
            ("attr", "cy=6"),
            ("attr", "r=3"),
            ("attr", "style=fill:red;stroke:blue"),
        ],
    )
    .unwrap();
    assert_eq!(r["id"], "circle-1");
    assert_eq!(r["parent"], "#grp");
    custom(
        "svg el add",
        &[&f],
        None,
        true,
        &[("kind", "text"), ("id", "t"), ("text", "a < b & c"), ("attr", "x=1"), ("attr", "y=2")],
    )
    .unwrap();
    custom("svg el add", &[&f], None, true, &[("kind", "path"), ("id", "p"), ("d", "M 0 0 L 5 5")]).unwrap();
    let d = XDoc::parse_svg(&read(&f)).unwrap();
    let c = d.el(&d.find_id("circle-1").unwrap());
    assert_eq!((c.get("cx"), c.get("r")), (Some("5"), Some("3")));
    assert_eq!(d.el(&d.find_id("t").unwrap()).text_content(), "a < b & c");
    assert_eq!(d.select(&Selector::parse("g.axis circle").unwrap()).len(), 1);
    // set by selector; the style property loses to the new attribute
    let r = custom("svg el set", &[&f], None, true, &[("where", "g.axis circle"), ("attr", "fill=#00ff00")]).unwrap();
    assert_eq!(r["matched"], 1);
    let d = XDoc::parse_svg(&read(&f)).unwrap();
    let c = d.el(&d.find_id("circle-1").unwrap());
    assert_eq!((c.get("fill"), c.get("style")), (Some("#00ff00"), Some("stroke:blue")));
    // set text by id, unset an attribute, to a new file
    let out = dir.path("g.svg");
    custom("svg el set", &[&f], Some(&out), false, &[("id", "t"), ("text", "hello"), ("unset", "x")]).unwrap();
    let g = XDoc::parse_svg(&read(&out)).unwrap();
    let t = g.el(&g.find_id("t").unwrap());
    assert_eq!((t.text_content().as_str(), t.get("x")), ("hello", None));
    // rm a group with its child
    let r = custom("svg el rm", &[&f], None, true, &[("id", "grp")]).unwrap();
    assert_eq!(r["removed"], 1);
    let d = XDoc::parse_svg(&read(&f)).unwrap();
    assert!(d.find_id("grp").is_none() && d.find_id("circle-1").is_none() && d.find_id("box").is_some());
    // the written file is stable under parse → serialize
    assert_eq!(d.serialize(crate::xml::WriteOpts::PRETTY), read(&f));
}

#[test]
fn svg_el_errors_are_specific() {
    let dir = TempDir::new("vec-el-err");
    let f = write(&dir, "f.svg", BASE);
    let e = custom("svg el add", &[&f], None, true, &[("kind", "rect"), ("id", "box")]).unwrap_err();
    assert_eq!(e.code, "PX1010", "duplicate id");
    let e = custom("svg el add", &[&f], None, true, &[("kind", "path")]).unwrap_err();
    assert_eq!(e.code, "PX1003", "path needs d");
    let e = custom("svg el add", &[&f], None, true, &[("kind", "rect"), ("parent", "nope")]).unwrap_err();
    assert_eq!(e.code, "PX1010");
    let e = custom("svg el set", &[&f], None, true, &[("id", "box"), ("where", "rect"), ("attr", "x=1")]).unwrap_err();
    assert_eq!(e.code, "PX1005");
    let e = custom("svg el set", &[&f], None, true, &[("where", "circle"), ("attr", "x=1")]).unwrap_err();
    assert_eq!(e.code, "PX1010", "no match");
    let e = custom("svg el rm", &[&f], None, true, &[("where", "svg")]).unwrap_err();
    assert_eq!(e.code, "PX1010", "root");
    let e = custom("svg el rm", &[&f], None, false, &[("id", "box")]).unwrap_err();
    assert_eq!(e.code, "PX1004");
    let e = custom("svg el rm", &[&f], Some(&f), false, &[("id", "box")]).unwrap_err();
    assert_eq!(e.code, "PX1403", "output equals input without --in-place");
    let bad = write(&dir, "bad.svg", "<svg><rect></svg>");
    let e = custom("svg el rm", &[&bad], None, true, &[("id", "box")]).unwrap_err();
    assert_eq!(e.code, "PX1103");
    assert_eq!(read(&f), BASE, "failed edits leave the file alone");
}

#[test]
fn svg_defs_and_use_render() {
    let dir = TempDir::new("vec-defs");
    let f = write(&dir, "f.svg", BASE);
    let spec_file = write(
        &dir,
        "blur.json",
        r#"{"x": "-20%", "width": "140%", "primitives": [{"type": "gaussian-blur", "stdDeviation": 2, "result": "b"}, {"type": "feMerge", "children": [{"tag": "feMergeNode", "in": "b"}, {"tag": "feMergeNode", "in": "SourceGraphic"}]}]}"#,
    );
    let r = custom(
        "svg defs add",
        &[&f],
        None,
        true,
        &[("kind", "filter"), ("id", "soft"), ("spec", spec_file.to_str().unwrap())],
    )
    .unwrap();
    assert_eq!(r["children"], 2);
    custom(
        "svg defs add",
        &[&f],
        None,
        true,
        &[("kind", "linear-gradient"), ("id", "g"), ("spec", r##"{"x2": 1, "stops": "#000000@0,#ffffff80@1"}"##)],
    )
    .unwrap();
    custom(
        "svg defs add",
        &[&f],
        None,
        true,
        &[
            ("kind", "symbol"),
            ("id", "dot"),
            ("spec", r##"{"children": [{"tag": "rect", "width": 4, "height": 4, "fill": "#0000ff"}]}"##),
        ],
    )
    .unwrap();
    custom("svg use", &[&f], None, true, &[("ref", "#dot"), ("at", "40,30")]).unwrap();
    let d = XDoc::parse_svg(&read(&f)).unwrap();
    assert_eq!(d.root.elements().next().unwrap().local(), "defs");
    let blur = d.el(&d.find_id("soft").unwrap());
    let kids: Vec<&str> = blur.elements().map(|e| e.local()).collect();
    assert_eq!(kids, ["feGaussianBlur", "feMerge"]);
    let g = d.el(&d.find_id("g").unwrap());
    assert_eq!(g.elements().nth(1).unwrap().get("stop-opacity"), Some("0.502"));
    let img = rasterize_text(&read(&f), 50, 40);
    assert!(close(px(&img, 41, 31), [0.0, 0.0, 1.0, 1.0]), "symbol instance renders");
    let e = custom("svg use", &[&f], None, true, &[("ref", "#missing")]).unwrap_err();
    assert_eq!(e.code, "PX1010");
    let e = custom(
        "svg defs add",
        &[&f],
        None,
        true,
        &[("kind", "filter"), ("id", "x"), ("spec", r#"{"primitives": [{"type": "nope"}]}"#)],
    )
    .unwrap_err();
    assert_eq!(e.code, "PX1010");
    let e = custom("svg defs add", &[&f], None, true, &[("kind", "linear-gradient"), ("id", "y"), ("spec", "{}")])
        .unwrap_err();
    assert_eq!(e.code, "PX1003");
}

#[test]
fn svg_query_reports_attrs_and_bbox() {
    let dir = TempDir::new("vec-q");
    let f = write(
        &dir,
        "f.svg",
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="50" height="40"><g transform="translate(5,0)"><rect x="10" y="10" width="20" height="10" stroke="#000" stroke-width="2"/></g><defs><rect id="hidden" width="5" height="5"/></defs></svg>"##,
    );
    let r = custom("svg query", &[&f], None, false, &[("where", "rect")]).unwrap();
    assert_eq!(r["count"], 2);
    let e0 = &r["elements"][0];
    assert_eq!(e0["id"], Value::Null);
    assert_eq!(e0["attrs"]["width"], "20");
    assert_eq!(e0["bbox"], json!([15.0, 10.0, 20.0, 10.0]));
    assert_eq!(e0["stroke_bbox"], json!([14.0, 9.0, 22.0, 12.0]));
    assert_eq!(r["elements"][1]["bbox"], Value::Null, "defs do not render");
    let r = custom("svg query", &[&f], None, false, &[]).unwrap();
    assert_eq!(r["elements"][0]["bbox"], json!([0.0, 0.0, 50.0, 40.0]));
    assert!(!f.with_extension("svg.pixelox.json").exists());
}

#[test]
fn svg_optimize_minifies_without_changing_the_picture() {
    let dir = TempDir::new("vec-opt");
    let src = r##"<?xml version="1.0"?>
<!-- Created with Inkscape -->
<svg xmlns="http://www.w3.org/2000/svg" xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape" xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd" xmlns:xlink="http://www.w3.org/1999/xlink" width="50" height="40" viewBox="0 0 50.000 40.000" inkscape:version="1.3">
  <metadata><rdf>x</rdf></metadata>
  <sodipodi:namedview id="nv" pagecolor="#ffffff"/>
  <g inkscape:label="Layer 1" inkscape:groupmode="layer" id="layer1">
    <path id="p" d="M 10.000004 10.123456 L 30.000 10 L 30 30.5 Z" fill="#ff0000" transform="translate(0.000001, 0)"/>
    <g/>
    <g>
    </g>
  </g>
</svg>
"##;
    let f = write(&dir, "in.svg", src);
    let out = dir.path("min.svg");
    let r = custom("svg optimize", &[&f], Some(&out), false, &[("precision", "2")]).unwrap();
    let text = read(&out);
    assert!(r["bytes_after"].as_u64().unwrap() < r["bytes_before"].as_u64().unwrap());
    assert_eq!(r["removed"]["empty_groups"], 2);
    for gone in ["<!--", "metadata", "inkscape", "sodipodi", "xlink", "\n  "] {
        assert!(!text.contains(gone), "{gone} survived: {text}");
    }
    assert!(text.contains(r#"d="M10 10.12L30 10 30 30.5Z""#), "{text}");
    assert!(text.contains(r#"viewBox="0 0 50 40""#) && text.contains(r#"id="layer1""#), "{text}");
    let a = rasterize_text(src, 50, 40);
    let b = rasterize_text(&text, 50, 40);
    // rounding to 2 decimals may move an anti-aliased edge pixel by one coverage step
    assert!(testing::max_diff(&a, &b) < 0.1);
}

#[test]
fn path_minifier_keeps_semantics() {
    assert_eq!(
        svg::minify_path("M 10.000 20.5 L 30 40 L 50 60 a 5 5 0 0 1 10 10 z", 3).unwrap(),
        "M10 20.5L30 40 50 60a5 5 0 0 1 10 10z"
    );
    assert_eq!(svg::minify_path("M0 0 M1 1", 3).unwrap(), "M0 0M1 1");
    assert_eq!(svg::minify_path("M0,0 l-0.5,0.25 .5.5", 3).unwrap(), "M0 0l-.5.25.5.5");
    assert_eq!(svg::minify_path("a1 1 0 0110 10", 3).unwrap(), "a1 1 0 0 1 10 10");
    assert!(svg::minify_path("M 0 0 X 1", 3).is_none());
}

#[test]
fn svg_rasterize_matches_a_known_rect() {
    let dir = TempDir::new("vec-ras");
    let f = write(&dir, "f.svg", BASE);
    let out = dir.path("o.png");
    let r = custom("svg rasterize", &[&f], Some(&out), false, &[]).unwrap();
    assert_eq!(r["size"], json!([50, 40]));
    let img = decode_png(&std::fs::read(&out).unwrap()).unwrap();
    assert!(close(px(&img, 10, 10), RED) && close(px(&img, 29, 19), RED) && close(px(&img, 5, 5), CLEAR));
    assert_eq!(opaque_count(&img), 200);
    let out2 = dir.path("o2.png");
    custom("svg rasterize", &[&f], Some(&out2), false, &[("scale", "2"), ("background-color", "white")]).unwrap();
    let img = decode_png(&std::fs::read(&out2).unwrap()).unwrap();
    assert_eq!((img.width, img.height), (100, 80));
    assert!(close(px(&img, 21, 21), RED) && close(px(&img, 5, 5), [1.0; 4]));
    let out3 = dir.path("o3.png");
    let r = custom("svg rasterize", &[&f], Some(&out3), false, &[("size", "25x")]).unwrap();
    assert_eq!(r["size"], json!([25, 20]));
    let e = custom("svg rasterize", &[&f], Some(&dir.path("o4.png")), false, &[("size", "10x10"), ("scale", "2")])
        .unwrap_err();
    assert_eq!(e.code, "PX1005");
}

#[test]
fn external_hrefs_stay_disabled() {
    let dir = TempDir::new("vec-href");
    let red = dir.path("red.png");
    std::fs::write(&red, crate::fromdoc::encode_png(&Image::filled(4, 4, Rgba::new(1.0, 0.0, 0.0, 1.0))).unwrap())
        .unwrap();
    let f = write(
        &dir,
        "f.svg",
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="4" height="4"><image href="red.png" width="4" height="4"/></svg>"#,
    );
    let out = dir.path("o.png");
    custom("svg rasterize", &[&f], Some(&out), false, &[]).unwrap();
    assert_eq!(opaque_count(&decode_png(&std::fs::read(&out).unwrap()).unwrap()), 0);
}

// ---------------------------------------------------------------------------------------
// from-doc

#[test]
fn from_doc_exports_structured_svg() {
    let dir = TempDir::new("vec-fromdoc");
    let mut d = canvas(60, 40);
    let bg = d.new_layer("Background", LayerKind::Raster { image: Image::filled(60, 40, Rgba::WHITE) });
    d.insert_above(None, bg);
    shape_op(
        &mut d,
        "shape rect",
        &[
            ("rect", "10,10,20x10"),
            ("fill", "red"),
            ("stroke", "#0000ff80"),
            ("stroke-width", "2"),
            ("dash", "4,2"),
            ("name", "Red Box"),
        ],
    );
    let mut g = d.new_layer("Group", LayerKind::Group { children: vec![] });
    let mut star =
        d.new_layer("Star", LayerKind::Shape { spec: json!({"d": "M 40 5 L 50 30 L 30 30 Z", "fill": "#00ff00"}) });
    star.blend = BlendMode::Multiply;
    star.opacity = 50.0;
    g.children_mut().unwrap().push(star);
    d.insert_above(None, g);
    let t = d.new_layer("Title", LayerKind::Text { spec: json!({"content": "Hi & bye", "font": "Inter", "size": "12px", "color": "#111111", "x": 30, "y": 5, "anchor": "t"}) });
    d.insert_above(None, t);
    let adj = d.new_layer("Levels", LayerKind::Adjustment { op: "levels".into(), params: Map::new() });
    d.insert_above(None, adj);

    let spec = find(&ops(), "svg from-doc");
    let out = dir.path("doc.svg");
    let a = testing::args(&spec, &[]).unwrap();
    let g = Globals { output: Some(out.display().to_string()), ..Default::default() };
    let mut ctx = OpCtx::new(spec.name, vec![], g, &TestIo, &NoProgress);
    let Runner::Doc(f) = spec.runner else { panic!() };
    let r = f(&a, &mut d, &mut ctx).unwrap();
    assert_eq!(
        (r["shapes"].as_u64(), r["texts"].as_u64(), r["images"].as_u64(), r["groups"].as_u64()),
        (Some(2), Some(1), Some(1), Some(1))
    );
    assert_eq!(r["skipped"][0]["layer"], "Levels");
    assert_eq!(ctx.outputs[0].recipe.as_ref().unwrap().last().unwrap().op, "svg.from-doc");
    let text = read(&out);
    let x = XDoc::parse_svg(&text).unwrap();
    let rb = x.el(&x.find_id("Red-Box").unwrap());
    assert_eq!(
        (rb.get("data-name"), rb.get("fill"), rb.get("stroke"), rb.get("stroke-opacity"), rb.get("stroke-dasharray")),
        (Some("Red Box"), Some("#ff0000"), Some("#0000ff"), Some("0.502"), Some("4,2"))
    );
    let st = x.el(&x.find_id("Star").unwrap());
    assert_eq!((st.get("opacity"), st.get("style")), (Some("0.5"), Some("mix-blend-mode:multiply")));
    assert_eq!(x.el(&x.find_id("Star").unwrap()).local(), "path");
    let tl = x.el(&x.find_id("Title").unwrap());
    assert_eq!(
        (tl.text_content().as_str(), tl.get("text-anchor"), tl.get("font-family")),
        ("Hi & bye", Some("middle"), Some("Inter"))
    );
    assert!(x.el(&x.find_id("Background").unwrap()).get("href").unwrap().starts_with("data:image/png;base64,"));

    // vector-only drops the raster; the vector part renders like the document
    let out2 = dir.path("vec.svg");
    let p = d.resolve("Title").unwrap();
    d.remove(&p);
    let a = testing::args(&spec, &[("vector-only", "true")]).unwrap();
    let g = Globals { output: Some(out2.display().to_string()), ..Default::default() };
    let mut ctx = OpCtx::new(spec.name, vec![], g, &TestIo, &NoProgress);
    let r = f(&a, &mut d, &mut ctx).unwrap();
    assert_eq!(r["images"], 0);
    let v = read(&out2);
    assert!(!v.contains("data:image"));
    let mut vec_only = d.clone();
    for name in ["Background", "Levels"] {
        let p = vec_only.resolve(name).unwrap();
        vec_only.remove(&p);
    }
    let want = compose::composite(&vec_only).unwrap();
    let got = rasterize_text(&v, 60, 40);
    let diff = want.data.iter().zip(&got.data).filter(|(a, b)| (*a - *b).abs() > 0.1).count();
    assert!(diff < 60, "{diff} components differ");
}

// ---------------------------------------------------------------------------------------
// trace

fn square_png(dir: &TempDir) -> PathBuf {
    let img = Image::from_fn(64, 64, |x, y| {
        if (16..48).contains(&x) && (16..48).contains(&y) {
            [0.0, 0.0, 0.0, 1.0]
        } else {
            [1.0; 4]
        }
    });
    let p = dir.path("square.png");
    std::fs::write(&p, crate::fromdoc::encode_png(&img).unwrap()).unwrap();
    p
}

#[test]
fn trace_binary_square_is_one_path_with_the_right_area() {
    let dir = TempDir::new("vec-trace");
    let src = square_png(&dir);
    let out = dir.path("sq.svg");
    let r = custom("trace", &[&src], Some(&out), false, &[("mode", "binary")]).unwrap();
    assert_eq!(r["paths"], 1);
    let text = read(&out);
    let x = XDoc::parse_svg(&text).unwrap();
    assert_eq!(x.select(&Selector::parse("path").unwrap()).len(), 1);
    let img = rasterize_text(&text, 64, 64);
    let area = img.data.chunks(4).map(|p| p[3] as f64).sum::<f64>();
    assert!((area - 1024.0).abs() / 1024.0 < 0.03, "{area}");
    assert!(close(px(&img, 32, 32), BLACK));
    // deterministic
    let out2 = dir.path("sq2.svg");
    custom("trace", &[&src], Some(&out2), false, &[("mode", "binary")]).unwrap();
    assert_eq!(read(&out2), text);
}

#[test]
fn trace_color_and_holes() {
    let dir = TempDir::new("vec-trace-c");
    let src = square_png(&dir);
    let out = dir.path("c.svg");
    let r = custom("trace", &[&src], Some(&out), false, &[("colors", "4")]).unwrap();
    assert_eq!(r["paths"], 2, "{r}");
    let img = rasterize_text(&read(&out), 64, 64);
    assert!(close(px(&img, 32, 32), BLACK) && close(px(&img, 4, 4), [1.0; 4]));
    // a ring keeps its hole
    let ring = Image::from_fn(64, 64, |x, y| {
        let (dx, dy) = (x as f32 - 31.5, y as f32 - 31.5);
        let r = (dx * dx + dy * dy).sqrt();
        if (12.0..26.0).contains(&r) {
            [0.0, 0.0, 0.0, 1.0]
        } else {
            [1.0; 4]
        }
    });
    let p = dir.path("ring.png");
    std::fs::write(&p, crate::fromdoc::encode_png(&ring).unwrap()).unwrap();
    let out = dir.path("ring.svg");
    let r = custom("trace", &[&p], Some(&out), false, &[("mode", "binary")]).unwrap();
    assert_eq!(r["paths"], 1);
    let img = rasterize_text(&read(&out), 64, 64);
    assert!(close(px(&img, 32, 32), CLEAR) && close(px(&img, 32, 13), BLACK));
    let want = std::f64::consts::PI * (26.0f64.powi(2) - 12.0f64.powi(2));
    let area = img.data.chunks(4).map(|p| p[3] as f64).sum::<f64>();
    assert!((area - want).abs() / want < 0.05, "{area} vs {want}");
}

#[test]
fn trace_needs_an_output() {
    let dir = TempDir::new("vec-trace-o");
    let src = square_png(&dir);
    assert_eq!(custom("trace", &[&src], None, false, &[]).unwrap_err().code, "PX1004");
}
