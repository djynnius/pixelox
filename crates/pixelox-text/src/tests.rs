//! Behavior tests. They use the embedded Tuffy face explicitly, so they pass whatever fonts
//! the machine has installed.

use super::*;
use pixelox_core::ctx::{Globals, NoIo, NoProgress};
use pixelox_core::testing::{find, run, run_with_warnings, TempDir};
use pixelox_core::Runner;

fn add(doc: &mut Document, content: &str, extra: &[(&str, &str)]) -> Value {
    let mut pairs = vec![("content", content), ("font", "Tuffy"), ("size", "48px")];
    pairs.extend_from_slice(extra);
    run(&find(&ops(), "text add"), &pairs, doc).unwrap()
}

fn spec(doc: &Document, name: &str) -> Value {
    match &doc.layer(&doc.resolve(name).unwrap()).content {
        LayerKind::Text { spec } => spec.clone(),
        other => panic!("{name} is a {} layer", other.name()),
    }
}

fn rect(v: &Value) -> [f64; 4] {
    let a = v.as_array().expect("rect");
    [a[0].as_f64().unwrap(), a[1].as_f64().unwrap(), a[2].as_f64().unwrap(), a[3].as_f64().unwrap()]
}

fn run_custom(name: &str, pairs: &[(&str, &str)], dry_run: bool) -> Result<Value> {
    let spec = find(&ops(), name);
    let a = pixelox_core::testing::args(&spec, pairs)?;
    let g = Globals { dry_run, ..Globals::default() };
    let mut ctx = OpCtx::new(spec.name, vec![], g, &NoIo, &NoProgress);
    match spec.runner {
        Runner::Custom(f) => f(&a, &mut ctx),
        _ => panic!("{name} is not a custom op"),
    }
}

#[test]
fn ops_are_well_formed() {
    let globals = [
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
    let names: Vec<&str> = ops().iter().map(|o| o.name).collect();
    for want in
        ["text add", "text edit", "text style", "text on-path", "text to-path", "text list", "fonts list", "fonts add"]
    {
        assert!(names.contains(&want), "missing {want}");
    }
    for op in ops() {
        assert!(op.summary.len() <= 60 && !op.summary.ends_with('.'), "{}: summary", op.name);
        assert!(!op.about.is_empty(), "{}: about", op.name);
        assert!(op.examples.len() >= 3, "{}: examples", op.name);
        for e in &op.examples {
            let head = format!("pixelox {}", op.name);
            assert!(e.cmd == head || e.cmd.starts_with(&format!("{head} ")), "{}: {}", op.name, e.cmd);
        }
        for p in &op.params {
            assert!(!p.help.is_empty(), "{} --{}: help", op.name, p.name);
            assert!(!globals.contains(&p.name), "{} --{} shadows a global flag", op.name, p.name);
        }
    }
}

#[test]
fn hello_renders_non_empty_and_deterministic() {
    install();
    let mut d = Document::new(320, 120);
    let r = add(&mut d, "Hello", &[("at", "10,10"), ("color", "#ff0000")]);
    assert_eq!(r["lines"], 1);
    assert_eq!(r["glyphs"], 5);
    let a = compose::composite(&d).unwrap();
    let b = compose::composite(&d).unwrap();
    assert!(a.data.iter().zip(&b.data).all(|(x, y)| x.to_bits() == y.to_bits()), "render is not deterministic");
    let ink: f64 = a.data.chunks(4).map(|p| p[3] as f64).sum();
    assert!(ink > 300.0, "too little ink: {ink}");
    // fully covered pixels carry the text color
    let solid = a.data.chunks(4).find(|p| p[3] >= 0.999).expect("a solid pixel");
    assert!((solid[0] - 1.0).abs() < 1e-6 && solid[1].abs() < 1e-6 && solid[2].abs() < 1e-6);
    // the reported bounds hold all the ink
    let [bx, by, bw, bh] = rect(&r["bounds"]);
    for y in 0..a.height {
        for x in 0..a.width {
            if a.get(x, y)[3] > 0.0 {
                let (x, y) = (x as f64, y as f64);
                assert!(x >= bx && x < bx + bw && y >= by && y < by + bh, "ink at {x},{y} outside {bx},{by},{bw}x{bh}");
            }
        }
    }
    // the face and its hash are recorded
    let s = spec(&d, "Hello");
    assert_eq!(s["font_sha256"], json!(pixelox_core::envelope::sha256_bytes(fonts::TUFFY)));
    assert_eq!(s["font_face"]["family"], "Tuffy");
}

#[test]
fn stored_spec_has_every_key() {
    let mut d = Document::new(200, 100);
    add(&mut d, "Keys", &[]);
    let s = spec(&d, "Keys");
    for k in [
        "content",
        "font",
        "font_sha256",
        "size",
        "weight",
        "style",
        "variation",
        "color",
        "stroke",
        "stroke_width",
        "align",
        "tracking",
        "leading",
        "kerning",
        "x",
        "y",
        "anchor",
        "box",
        "rotation",
        "path",
    ] {
        assert!(s.get(k).is_some(), "spec lacks {k}");
    }
    assert_eq!(s["size"], "48px");
    assert_eq!(s["anchor"], "tl");
    assert_eq!((s["x"].as_f64(), s["y"].as_f64(), s["rotation"].as_f64()), (Some(0.0), Some(0.0), Some(0.0)));
    assert!(s["path"].is_null());
    // pt sizes go through the document DPI
    let mut d = Document::new(400, 200);
    d.dpi = 144.0;
    let r = add(&mut d, "Pt", &[("size", "24pt")]);
    assert_eq!(r["size_px"], 48.0);
}

#[test]
fn anchors_place_the_text_box() {
    let mut d = Document::new(400, 300);
    let r = add(&mut d, "Anchor", &[("at", "20,30"), ("anchor", "tl"), ("name", "TL")]);
    let [x, y, w, h] = rect(&r["box"]);
    assert_eq!((x, y), (20.0, 30.0));
    assert!(w > 50.0 && h > 40.0);
    let r = add(&mut d, "Anchor", &[("at", "50%,50%"), ("anchor", "c"), ("name", "C")]);
    let [x, y, w, h] = rect(&r["box"]);
    assert!((x + w / 2.0 - 200.0).abs() < 0.02 && (y + h / 2.0 - 150.0).abs() < 0.02, "{x},{y},{w}x{h}");
    // the ink is centered horizontally too (within a glyph side bearing)
    let [bx, _, bw, _] = rect(&r["bounds"]);
    assert!((bx + bw / 2.0 - 200.0).abs() < 6.0);
    let r = add(&mut d, "Anchor", &[("at", "-10,-10"), ("anchor", "br"), ("name", "BR")]);
    let [x, y, w, h] = rect(&r["box"]);
    assert!((x + w - 390.0).abs() < 0.02 && (y + h - 290.0).abs() < 0.02, "{x},{y},{w}x{h}");
    // same text, same size everywhere
    assert!((rect(&add(&mut d, "Anchor", &[("anchor", "b"), ("name", "B")])["box"])[2] - w).abs() < 0.02);
}

#[test]
fn box_wraps_into_lines() {
    let mut d = Document::new(400, 600);
    let text = "The quick brown fox jumps over the lazy dog again and again";
    let r = add(&mut d, text, &[("box", "180x500"), ("at", "10,10"), ("name", "P")]);
    let lines = r["lines"].as_u64().unwrap();
    assert!(lines >= 3, "only {lines} lines");
    assert_eq!(r["overflow"], false);
    let [bx, _, bw, bh] = rect(&r["bounds"]);
    // (side bearings may reach a few px past the box edges)
    assert!(bx >= 5.0 && bx + bw <= 10.0 + 180.0 + 4.0, "ink {bx}+{bw} exceeds the box");
    assert!(bh > 2.5 * 48.0);
    // an explicit newline starts a paragraph; a short box height overflows
    let r = add(&mut d, "one\ntwo", &[("box", "300x40"), ("name", "Q")]);
    assert_eq!(r["lines"], 2);
    assert_eq!(r["overflow"], true);
    // a word longer than the box breaks between letters
    let r = add(&mut d, "Supercalifragilistic", &[("box", "120x"), ("name", "R")]);
    assert!(r["lines"].as_u64().unwrap() >= 3);
    // justify stretches the first lines to the box width
    let r = add(&mut d, text, &[("box", "180x500"), ("align", "justify"), ("name", "J")]);
    let [jx, _, jw, _] = rect(&r["bounds"]);
    assert!(jx + jw > 10.0 + 180.0 - 6.0 || jx + jw > bx + bw, "justified ink {jx}+{jw}");
}

#[test]
fn tracking_widens() {
    let mut d = Document::new(800, 200);
    let plain = rect(&add(&mut d, "Tracking", &[("name", "A")])["box"])[2];
    let wide = rect(&add(&mut d, "Tracking", &[("name", "B"), ("tracking", "200")])["box"])[2];
    // 7 gaps × 0.2 em × 48 px
    assert!((wide - plain - 7.0 * 0.2 * 48.0).abs() < 0.5, "{plain} → {wide}");
    let tight = rect(&add(&mut d, "Tracking", &[("name", "C"), ("tracking", "-50")])["box"])[2];
    assert!(tight < plain);
    let unkerned = rect(&add(&mut d, "AVAWAVATA", &[("name", "K"), ("kerning", "none")])["box"])[2];
    let kerned = rect(&add(&mut d, "AVAWAVATA", &[("name", "L")])["box"])[2];
    assert!(unkerned >= kerned);
}

#[test]
fn edit_and_style_change_the_layer() {
    let mut d = Document::new(400, 200);
    add(&mut d, "Before", &[("name", "T"), ("color", "#000000")]);
    let edit = find(&ops(), "text edit");
    let r = run(&edit, &[("layer", "T"), ("content", "After words")], &mut d).unwrap();
    assert_eq!(r["content"], "After words");
    assert_eq!(spec(&d, "T")["color"], "#000000");
    let style = find(&ops(), "text style");
    run(&style, &[("layer", "T"), ("color", "#00ff00"), ("align", "center")], &mut d).unwrap();
    let s = spec(&d, "T");
    assert_eq!(
        (s["color"].as_str(), s["align"].as_str(), s["content"].as_str()),
        (Some("#00ff00"), Some("center"), Some("After words"))
    );
    // style never takes --content
    assert_eq!(run(&style, &[("layer", "T"), ("content", "x")], &mut d).unwrap_err().code, "PX1002");
    // nothing to change
    assert_eq!(run(&style, &[("layer", "T")], &mut d).unwrap_err().code, "PX1003");
    // the only text layer is the default target
    run(&edit, &[("content", "Default")], &mut d).unwrap();
    assert_eq!(spec(&d, "T")["content"], "Default");
    // locked and non-text layers are refused
    let p = d.resolve("T").unwrap();
    d.layer_mut(&p).locked = true;
    assert_eq!(run(&edit, &[("layer", "T"), ("content", "x")], &mut d).unwrap_err().code, "PX2304");
    let mut img = pixelox_core::testing::doc_from(pixelox_core::testing::solid(8, 8, Rgba::WHITE));
    assert_eq!(run(&edit, &[("layer", "Background"), ("content", "x")], &mut img).unwrap_err().code, "PX2303");
}

#[test]
fn unknown_family_is_px4102_in_strict_mode() {
    let mut d = Document::new(200, 100);
    let strict = ("strict-font", "true");
    let e = run(&find(&ops(), "text add"), &[("content", "x"), ("font", "Zzqx Nonexistent Family 9"), strict], &mut d)
        .unwrap_err();
    assert_eq!(e.code, "PX4102");
    assert!(e.context.contains_key("candidates"));
    assert!(e.hint.unwrap().contains("fonts"));
    assert_eq!(d.layers.len(), 0);
    // a misspelling suggests the real family
    let e = run(&find(&ops(), "text add"), &[("content", "x"), ("font", "Tufy"), strict], &mut d).unwrap_err();
    assert!(e.context["candidates"].as_array().unwrap().iter().any(|c| c == "Tuffy"), "{:?}", e.context);
    // a curated Google Fonts family that is not installed asks for consent (exit 6)
    if !fonts::has_family("Saira Stencil One") {
        let e = run(&find(&ops(), "text add"), &[("content", "x"), ("font", "Saira Stencil One"), strict], &mut d)
            .unwrap_err();
        assert_eq!(e.code, "PX1540");
        assert_eq!(e.exit_code, 6);
        assert!(e.hint.unwrap().contains("pixelox fonts get \"Saira Stencil One\" -y"));
    }
    // generic families always resolve
    run(&find(&ops(), "text add"), &[("content", "x"), ("font", "monospace"), strict], &mut d).unwrap();
}

#[test]
fn unknown_family_is_substituted_with_a_warning() {
    let mut d = Document::new(400, 100);
    let (r, w) = run_with_warnings(
        &find(&ops(), "text add"),
        &[("content", "HELLO"), ("font", "Zzqx Nonexistent Condensed Black"), ("size", "40px")],
        &mut d,
    )
    .unwrap();
    assert!(w.contains(&"PXW4010".to_string()), "{w:?}");
    assert_eq!(r["font"]["resolved"]["how"], "substitute");
    assert_eq!(r["font"]["resolved"]["class"], "@condensed-heavy");
    assert_eq!(d.layers.len(), 1);
    // the layer keeps the requested family, so a machine that has it draws it
    assert_eq!(spec(&d, "HELLO")["font"], "Zzqx Nonexistent Condensed Black");
    // an unknown class is an error
    let e = run(&find(&ops(), "text add"), &[("content", "x"), ("font", "@nope")], &mut d).unwrap_err();
    assert_eq!(e.code, "PX4110");
}

#[test]
fn synthetic_styles_and_fallback_warn() {
    let mut d = Document::new(400, 200);
    let spec_add = find(&ops(), "text add");
    let base = [("content", "Bold"), ("font", "Tuffy"), ("size", "48px")];
    let (r, w) =
        run_with_warnings(&spec_add, &[&base[..], &[("weight", "bold"), ("name", "B")]].concat(), &mut d).unwrap();
    assert!(w.contains(&"PXW4002".to_string()), "{w:?}");
    assert_eq!(r["synthetic"], json!(["bold"]));
    let plain = rect(&add(&mut d, "Bold", &[("name", "P")])["bounds"]);
    assert!(rect(&r["bounds"])[2] > plain[2], "synthetic bold should be wider");
    let (r, w) =
        run_with_warnings(&spec_add, &[&base[..], &[("style", "italic"), ("name", "I")]].concat(), &mut d).unwrap();
    assert!(w.contains(&"PXW4002".to_string()));
    assert_eq!(r["synthetic"], json!(["italic"]));
    // a character Tuffy lacks goes through the fallback chain (or is reported missing)
    let (_, w) =
        run_with_warnings(&spec_add, &[("content", "A\u{4E2D}"), ("font", "Tuffy"), ("name", "F")], &mut d).unwrap();
    assert!(w.contains(&"PXW4001".to_string()), "{w:?}");
}

#[test]
fn to_path_makes_a_shape_layer() {
    let mut d = pixelox_core::testing::doc_from(pixelox_core::testing::solid(300, 120, Rgba::WHITE));
    add(
        &mut d,
        "Outline",
        &[("name", "Word"), ("at", "10,20"), ("color", "#123456"), ("stroke", "#ff0000"), ("stroke-width", "3")],
    );
    add(&mut d, "Top", &[("name", "Above")]);
    let before = d.resolve("Word").unwrap();
    let id = d.layer(&before).id.clone();
    let r = run(&find(&ops(), "text to-path"), &[("layer", "Word")], &mut d).unwrap();
    assert_eq!(r["kind"], "shape");
    let after = d.resolve("Word").unwrap();
    assert_eq!(before, after, "stack position changed");
    let l = d.layer(&after);
    assert_eq!(l.id, id);
    let LayerKind::Shape { spec } = &l.content else { panic!("not a shape layer") };
    let dd = spec["d"].as_str().unwrap();
    assert!(dd.starts_with('M') && dd.len() > 100);
    assert_eq!(spec["fill"], "#123456");
    assert_eq!(spec["stroke"], "#ff0000");
    assert_eq!(spec["stroke_width"], 3.0);
    // the outlines sit where the text was drawn
    let (x0, y0, x1, _) = geom::path_bbox(dd).unwrap();
    assert!((9.0..20.0).contains(&x0) && (20.0..40.0).contains(&y0) && x1 > x0 + 100.0, "{x0},{y0},{x1}");
    // text layers only
    assert_eq!(run(&find(&ops(), "text to-path"), &[("layer", "Word")], &mut d).unwrap_err().code, "PX2303");
}

fn ink_radii(r: &Rendered, cx: f64, cy: f64) -> (f64, f64, [usize; 4]) {
    let (mut lo, mut hi) = (f64::MAX, 0.0f64);
    let mut quadrants = [0usize; 4];
    for y in 0..r.image.height {
        for x in 0..r.image.width {
            if r.image.get(x, y)[3] > 0.5 {
                let (px, py) = (r.x as f64 + x as f64 + 0.5 - cx, r.y as f64 + y as f64 + 0.5 - cy);
                let d = (px * px + py * py).sqrt();
                lo = lo.min(d);
                hi = hi.max(d);
                quadrants[usize::from(px >= 0.0) + 2 * usize::from(py >= 0.0)] += 1;
            }
        }
    }
    (lo, hi, quadrants)
}

#[test]
fn on_path_follows_a_circle() {
    let mut d = Document::new(400, 400);
    let content = "AROUND AND AROUND THE CIRCLE WE GO ";
    add(&mut d, content, &[("name", "Arc"), ("size", "24px")]);
    let on = find(&ops(), "text on-path");
    let circle = "M 200,80 A 120,120 0 1 1 199.9,80";
    let r = run(&on, &[("layer", "Arc"), ("d", circle)], &mut d).unwrap();
    let len = r["path_length"].as_f64().unwrap();
    assert!((len - 2.0 * std::f64::consts::PI * 120.0).abs() < 2.0, "length {len}");
    assert_eq!(spec(&d, "Arc")["path"], circle);
    let out = compose::render_layer(&d, d.layer(&d.resolve("Arc").unwrap()));
    let rendered = match out {
        Ok(Some(r)) => r,
        _ => {
            install();
            compose::render_layer(&d, d.layer(&d.resolve("Arc").unwrap())).unwrap().unwrap()
        }
    };
    let (lo, hi, q) = ink_radii(&rendered, 200.0, 200.0);
    // outside: glyphs stand on the circle, pointing away from the center
    assert!(lo > 116.0 && hi < 120.0 + 24.0 + 4.0, "radii {lo}..{hi}");
    assert!(q.iter().filter(|&&n| n > 0).count() >= 3, "ink should go most of the way around: {q:?}");
    // clockwise from the top: the top-right quarter is reached first
    assert!(q[1] > 0);
    // inside: the same text hangs inside the circle
    run(&on, &[("layer", "Arc"), ("side", "inside")], &mut d).unwrap();
    let rendered = compose::render_layer(&d, d.layer(&d.resolve("Arc").unwrap())).unwrap().unwrap();
    let (lo, hi, _) = ink_radii(&rendered, 200.0, 200.0);
    assert!(lo > 120.0 - 24.0 - 4.0 && hi < 124.0, "inside radii {lo}..{hi}");
    // --follow takes a shape layer's path; both sources at once conflict
    let ring = d.new_layer("Ring", LayerKind::Shape { spec: json!({"d": circle, "fill": null}) });
    d.insert_above(None, ring);
    run(&on, &[("layer", "Arc"), ("follow", "Ring"), ("offset", "50%")], &mut d).unwrap();
    assert_eq!(spec(&d, "Arc")["path_offset"], "50%");
    assert_eq!(run(&on, &[("layer", "Arc"), ("follow", "Ring"), ("d", circle)], &mut d).unwrap_err().code, "PX1005");
    // --d none takes the text off the path again
    let r = run(&on, &[("layer", "Arc"), ("d", "none")], &mut d).unwrap();
    assert!(spec(&d, "Arc")["path"].is_null());
    assert!(r["box"].is_array());
}

#[test]
fn rotation_and_moves_keep_text_live() {
    let mut d = Document::new(400, 400);
    add(&mut d, "Rotate me", &[("name", "R"), ("at", "200,200"), ("anchor", "c")]);
    let flat = rect(&add(&mut d, "Rotate me", &[("name", "S"), ("at", "200,200"), ("anchor", "c")])["bounds"]);
    let p = d.resolve("R").unwrap();
    let mut s = spec(&d, "R");
    s["rotation"] = json!(90.0);
    d.layer_mut(&p).content = LayerKind::Text { spec: s.clone() };
    let c = layout::compose(&spec::TextSpec::from_value(&s), &d, false).unwrap();
    let r = c.ink_rect().unwrap();
    assert!(r.h as f64 > flat[2] - 4.0 && (r.w as f64) < flat[3] + 4.0, "rotated {r:?} vs {flat:?}");
    // rotation is about the anchor point: the ink stays centered on it
    assert!(((r.x as f64 + r.w as f64 / 2.0) - 200.0).abs() < 30.0);
    // moving the layer moves the ink by the same amount
    let before = layout::compose(&spec::TextSpec::from_value(&spec(&d, "S")), &d, false).unwrap().ink_rect().unwrap();
    let ps = d.resolve("S").unwrap();
    geom::translate_layer(&mut d, &ps, 15.0, -20.0).unwrap();
    let after = layout::compose(&spec::TextSpec::from_value(&spec(&d, "S")), &d, false).unwrap().ink_rect().unwrap();
    assert_eq!((after.x - before.x, after.y - before.y, after.w, after.h), (15, -20, before.w, before.h));
}

#[test]
fn text_list_reports_layers_and_fonts() {
    let mut d = Document::new(400, 200);
    add(&mut d, "First", &[("name", "One")]);
    add(&mut d, "Second", &[("name", "Two"), ("weight", "bold")]);
    let list = find(&ops(), "text list");
    let r = run(&list, &[], &mut d).unwrap();
    assert_eq!(r["count"], 2);
    assert_eq!(r["layers"][0]["name"], "Two"); // top first
    assert_eq!(r["layers"][1]["content"], "First");
    assert!(r.get("fonts").is_none());
    let r = run(&list, &[("fonts", "true")], &mut d).unwrap();
    let fonts = r["fonts"].as_array().unwrap();
    assert_eq!(fonts[0]["family"], "Tuffy");
    assert!(fonts[0]["sha256"].as_str().unwrap().len() == 64);
    assert_eq!(fonts[0]["embedded"], true);
}

#[test]
fn fonts_list_and_add() {
    let r = run_custom("fonts list", &[("family", "tuffy")], false).unwrap();
    let fam = r["families"].as_array().unwrap().iter().find(|f| f["family"] == "Tuffy").expect("Tuffy listed").clone();
    assert!(fam["faces"].as_array().unwrap().iter().any(|f| f["embedded"] == true));
    assert_eq!(r["fallback"]["family"], "Tuffy");
    assert_eq!(r["fallback_chain"][0], "Tuffy");
    for g in ["sans-serif", "serif", "monospace"] {
        assert!(r["generic"][g].is_string());
    }
    let none = run_custom("fonts list", &[("family", "Zzqx Nonexistent")], false).unwrap();
    assert_eq!(none["count"], 0);
    assert!(none["candidates"].is_array());
    // fonts add scans the folder; a dry run registers nothing
    let tmp = TempDir::new("fonts-add");
    std::fs::write(tmp.path("Tuffy-copy.ttf"), fonts::TUFFY).unwrap();
    let r = run_custom("fonts add", &[("dir", tmp.0.to_str().unwrap())], true).unwrap();
    assert_eq!(r["faces"], 1);
    assert_eq!(r["families"], json!(["Tuffy"]));
    assert_eq!(r["added"], false);
    let e = run_custom("fonts add", &[("dir", tmp.path("missing").to_str().unwrap())], true).unwrap_err();
    assert_eq!(e.code, "PX1101");
    let e = run_custom("fonts add", &[("dir", tmp.path("Tuffy-copy.ttf").to_str().unwrap())], true).unwrap_err();
    assert_eq!(e.code, "PX1010");
}

#[test]
fn rtl_paragraph_lays_out() {
    // Hebrew is not in Tuffy: it comes from the fallback chain or draws missing-glyph boxes;
    // either way the line is shaped and laid out without failing
    let mut d = Document::new(400, 200);
    let r = add(&mut d, "abc \u{5D0}\u{5D1}\u{5D2} 123", &[("name", "Mixed")]);
    assert_eq!(r["lines"], 1);
    assert!(rect(&r["box"])[2] > 50.0);
}

fn ink_size(r: &Value) -> (f64, f64) {
    let f = &r["fit"]["ink"];
    (f[0].as_f64().unwrap(), f[1].as_f64().unwrap())
}

#[test]
fn fit_width_and_box_size_the_visible_letters() {
    let mut d = Document::new(1000, 600);
    let r = add(&mut d, "MAKE", &[("name", "A"), ("fit-width", "920"), ("stroke", "#000000"), ("stroke-width", "6")]);
    let (w, _) = ink_size(&r);
    assert!((w - 920.0).abs() < 1e-3, "ink width {w}");
    // the fit follows new words
    let ed = find(&ops(), "text edit");
    let r = run(&ed, &[("layer", "A"), ("content", "TONIGHT")], &mut d).unwrap();
    assert!((ink_size(&r).0 - 920.0).abs() < 1e-3);
    // a box: the binding side matches
    let r = add(&mut d, "WIDE WORDS", &[("name", "B"), ("fit-box", "400x60")]);
    let (w, h) = ink_size(&r);
    assert!(w <= 400.0 + 1e-3 && h <= 60.0 + 1e-3 && ((w - 400.0).abs() < 1e-3 || (h - 60.0).abs() < 1e-3), "{w}x{h}");
    // an explicit size later replaces the fit
    let st = find(&ops(), "text style");
    let r = run(&st, &[("layer", "B"), ("size", "20px")], &mut d).unwrap();
    assert!(r.get("fit").is_none());
    assert!(spec(&d, "B").get("fit_box").is_none());
    // nothing to fit
    let e = run(&find(&ops(), "text add"), &[("content", "   "), ("font", "Tuffy"), ("fit-width", "100")], &mut d)
        .unwrap_err();
    assert_eq!(e.code, "PX4113");
}

#[test]
fn anchor_to_ink_places_the_visible_letters() {
    let mut d = Document::new(600, 300);
    let r = add(&mut d, "Hxg", &[("at", "100,50"), ("anchor-to", "ink"), ("size", "80px")]);
    let b = rect(&r["bounds"]);
    assert_eq!((b[0], b[1]), (100.0, 50.0), "{b:?}");
    let r = add(
        &mut d,
        "Hxg",
        &[("name", "C"), ("at", "300,150"), ("anchor", "c"), ("anchor-to", "ink"), ("size", "80px")],
    );
    let b = rect(&r["bounds"]);
    assert!((b[0] + b[2] / 2.0 - 300.0).abs() <= 1.0 && (b[1] + b[3] / 2.0 - 150.0).abs() <= 1.0, "{b:?}");
}

#[test]
fn case_scale_and_faux_bold_stay_live() {
    let mut d = Document::new(800, 300);
    let plain = rect(&add(&mut d, "type", &[("name", "P")])["bounds"]);
    let upper = add(&mut d, "type", &[("name", "U"), ("case", "upper")]);
    assert_eq!(spec(&d, "U")["content"], "type", "the content keeps what was typed");
    assert_eq!(upper["case"], "upper");
    assert!(rect(&upper["bounds"])[2] > plain[2]);
    let narrow = rect(&add(&mut d, "type", &[("name", "N"), ("scale-x", "50%")])["bounds"]);
    assert!((narrow[2] - plain[2] / 2.0).abs() < 3.0, "{narrow:?} vs {plain:?}");
    let tall = rect(&add(&mut d, "type", &[("name", "T"), ("scale-y", "2")])["bounds"]);
    assert!(tall[3] > plain[3] * 1.8, "{tall:?}");
    let bold = add(&mut d, "type", &[("name", "B"), ("faux-bold", "40")]);
    assert!(rect(&bold["bounds"])[2] > plain[2] + 4.0);
    assert_eq!(spec(&d, "B")["faux_bold"], 40.0);
    // text to-path bakes the faux bold into the outlines
    let r = run(&find(&ops(), "text to-path"), &[("layer", "B")], &mut d).unwrap();
    assert!(r["d_bytes"].as_u64().unwrap() > 100);
    // removing settings removes their keys
    let st = find(&ops(), "text style");
    run(&st, &[("layer", "U"), ("case", "none")], &mut d).unwrap();
    assert!(spec(&d, "U").get("case").is_none());
}

#[test]
fn warps_bend_the_outlines_and_stay_editable() {
    install();
    let mut d = Document::new(800, 400);
    let flat = rect(&add(&mut d, "WARPED TYPE", &[("name", "F"), ("at", "400,200"), ("anchor", "c")])["bounds"]);
    let w = find(&ops(), "text warp");
    let outline = |d: &Document, n: &str| {
        let s = crate::spec::TextSpec::from_value(&spec(d, n));
        crate::render::svg_path_data(&crate::layout::compose(&s, d, false).unwrap())
    };
    let flat_d = outline(&d, "F");
    let mut seen = vec![flat_d];
    for p in crate::warp::PRESETS {
        let name = format!("W-{p}");
        add(&mut d, "WARPED TYPE", &[("name", &name), ("at", "400,200"), ("anchor", "c")]);
        let r = run(&w, &[("layer", &name), ("preset", p), ("bend", "60")], &mut d).unwrap();
        assert_eq!(r["warp"]["preset"], *p);
        let o = outline(&d, &name);
        assert!(!seen.contains(&o), "{p} looks like another preset or like flat text");
        seen.push(o);
        assert!(matches!(d.layer(&d.resolve(&name).unwrap()).content, LayerKind::Text { .. }));
    }
    // the arc's ends drop below the flat text (bend 60 = a 108° ring sector)
    let arc = rect(&run(&w, &[("layer", "W-arc"), ("preset", "arc"), ("bend", "60")], &mut d).unwrap()["bounds"]);
    assert!(arc[3] > flat[3] * 1.5, "{arc:?} vs {flat:?}");
    // editing the words keeps the warp; none removes it
    let r = run(&find(&ops(), "text edit"), &[("layer", "W-arc"), ("content", "NEW WORDS")], &mut d).unwrap();
    assert_eq!(r["warp"]["preset"], "arc");
    let r = run(&w, &[("layer", "W-arc"), ("preset", "none")], &mut d).unwrap();
    assert!(r["warp"].is_null());
    assert!(spec(&d, "W-arc").get("warp").is_none());
    // text style --bend without a warp is an error
    let st = find(&ops(), "text style");
    assert_eq!(run(&st, &[("layer", "F"), ("bend", "20")], &mut d).unwrap_err().code, "PX1003");
    run(&st, &[("layer", "F"), ("warp", "flag"), ("bend", "-30"), ("orient", "vertical")], &mut d).unwrap();
    assert_eq!(spec(&d, "F")["warp"]["orient"], "vertical");
    // rendering is deterministic
    let a = compose::composite(&d).unwrap();
    let b = compose::composite(&d).unwrap();
    assert!(a.data.iter().zip(&b.data).all(|(x, y)| x.to_bits() == y.to_bits()));
}

#[test]
fn on_path_overflow_warns_and_shrinks() {
    let mut d = Document::new(400, 400);
    add(&mut d, "A LONG LINE THAT CANNOT POSSIBLY FIT AROUND", &[("name", "Arc"), ("size", "40px")]);
    let on = find(&ops(), "text on-path");
    let circle = "M 200,140 A 60,60 0 1 1 199.9,140";
    let (r, w) = run_with_warnings(&on, &[("layer", "Arc"), ("d", circle)], &mut d).unwrap();
    assert!(w.contains(&"PXW4012".to_string()), "{w:?}");
    assert!(r["overflow_glyphs"].as_u64().unwrap() > 5);
    let (r, w) = run_with_warnings(&on, &[("layer", "Arc"), ("fit", "shrink")], &mut d).unwrap();
    assert!(!w.contains(&"PXW4012".to_string()), "{w:?}");
    assert_eq!(r["overflow_glyphs"], 0);
    assert!(r["text_length"].as_f64().unwrap() <= r["path_length"].as_f64().unwrap());
    assert!(r["size_px"].as_f64().unwrap() < 40.0);
    assert_eq!(spec(&d, "Arc")["path_fit"], "shrink");
    // an open path: letters past the end are counted too
    add(&mut d, "TOO MANY LETTERS HERE", &[("name", "Line"), ("size", "30px")]);
    let (r, w) = run_with_warnings(&on, &[("layer", "Line"), ("d", "M 10,300 L 150,300")], &mut d).unwrap();
    assert!(w.contains(&"PXW4012".to_string()) && r["overflow_glyphs"].as_u64().unwrap() > 0);
}

#[test]
fn stretch_and_weight_requests_are_recorded() {
    let mut d = Document::new(400, 100);
    let r = add(&mut d, "Wide", &[("name", "S"), ("stretch", "condensed"), ("weight", "700")]);
    assert_eq!(r["stretch"], "condensed");
    let s = spec(&d, "S");
    assert_eq!((s["stretch"].as_f64(), s["weight"].as_u64()), (Some(75.0), Some(700)));
    // no --weight: automatic, not 400
    add(&mut d, "Auto", &[("name", "A")]);
    assert!(spec(&d, "A")["weight"].is_null());
    assert!(run(&find(&ops(), "text add"), &[("content", "x"), ("stretch", "squashed")], &mut d).is_err());
}
