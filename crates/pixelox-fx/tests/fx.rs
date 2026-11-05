//! Behavior tests for the `fx *` commands and the layer-style renderer.

use pixelox_core::doc::FxSpec;
use pixelox_core::prelude::*;
use pixelox_core::testing::{self, find, run, run_with_warnings, TempDir};

const W: u32 = 48;
const H: u32 = 48;
/// The square's box: x 16..32, y 12..28.
const SQ: (u32, u32, u32, u32) = (16, 12, 16, 16);

fn square(c: Rgba) -> Image {
    let (x0, y0, w, h) = SQ;
    Image::from_fn(
        W,
        H,
        move |x, y| if x >= x0 && x < x0 + w && y >= y0 && y < y0 + h { c.to_array() } else { [0.0; 4] },
    )
}

/// A one-layer document: a gray square named `Box` on transparency.
fn doc() -> Document {
    pixelox_fx::install();
    testing::doc_layers(W, H, vec![("Box", square(Rgba::new(0.5, 0.5, 0.5, 1.0)))])
}

fn fx(name: &str, pairs: &[(&str, &str)], d: &mut Document) -> Value {
    run(&find(&pixelox_fx::ops(), name), pairs, d).unwrap_or_else(|e| panic!("{name}: {e:?}"))
}

fn fx_err(name: &str, pairs: &[(&str, &str)], d: &mut Document) -> String {
    run(&find(&pixelox_fx::ops(), name), pairs, d).expect_err("expected an error").code
}

fn bounds(d: &Document, name: &str) -> RectI {
    let l = d.layer(&d.resolve(name).unwrap());
    compose::layer_bounds(d, l).unwrap().expect("visible content")
}

fn inside(x: u32, y: u32) -> bool {
    let (x0, y0, w, h) = SQ;
    x >= x0 && x < x0 + w && y >= y0 && y < y0 + h
}

#[test]
fn every_command_is_documented() {
    let ops = pixelox_fx::ops();
    let names: Vec<&str> = ops.iter().map(|o| o.name).collect();
    for n in [
        "fx drop-shadow",
        "fx inner-shadow",
        "fx outer-glow",
        "fx inner-glow",
        "fx bevel",
        "fx satin",
        "fx overlay",
        "fx stroke",
        "fx list",
        "fx rm",
        "fx copy",
        "fx scale",
    ] {
        assert!(names.contains(&n), "missing {n}");
    }
    let globals = ["--in-place", "--json", "--dry-run", "--overwrite"];
    for o in &ops {
        assert!(o.summary.len() <= 60 && !o.summary.ends_with('.'), "{}: summary", o.name);
        assert!(!o.about.is_empty(), "{}: about", o.name);
        assert!(o.examples.len() >= 3, "{}: examples", o.name);
        for p in &o.params {
            assert!(!p.help.is_empty(), "{} --{}: help", o.name, p.name);
        }
        for ex in &o.examples {
            assert!(ex.cmd.starts_with(&format!("pixelox {} ", o.name)), "{}", ex.cmd);
            for tok in ex.cmd.split_whitespace().filter(|t| t.starts_with("--")) {
                assert!(globals.contains(&tok) || o.find_param(&tok[2..]).is_some(), "{}: unknown flag {tok}", o.name);
            }
        }
    }
    for n in &names[..8] {
        let o = find(&ops, n);
        for p in ["layer", "blend", "opacity", "enabled"] {
            assert!(o.find_param(p).is_some(), "{n} lacks --{p}");
        }
    }
}

#[test]
fn drop_shadow_appears_offset_below() {
    let mut d = doc();
    let r = fx("fx drop-shadow", &[("offset", "0,8"), ("blur", "0"), ("opacity", "100")], &mut d);
    assert_eq!(r["action"], "added");
    let spec = &d.layers[0].fx[0];
    assert_eq!(spec.kind, "drop-shadow");
    assert!(spec.enabled);
    assert_eq!(spec.params["offset"], "0,8");
    assert!(!spec.params.contains_key("layer") && !spec.params.contains_key("enabled"));

    let c = compose::composite(&d).unwrap();
    // below the square: the shadow (black, opaque); above it: nothing
    let below = c.get(24, 32);
    assert!(below[3] > 0.99 && below[0] < 0.01, "{below:?}");
    assert_eq!(c.get(24, 8)[3], 0.0);
    // the square itself is untouched
    assert!((c.get(24, 20)[0] - 0.5).abs() < 1e-5);
    assert_eq!(bounds(&d, "Box"), RectI::new(16, 12, 16, 24));

    // a soft shadow is still stronger below than above
    fx("fx drop-shadow", &[("offset", "0,6"), ("blur", "8")], &mut d);
    assert_eq!(d.layers[0].fx.len(), 1, "replaces the existing drop shadow");
    let c = compose::composite(&d).unwrap();
    assert!(c.get(24, 30)[3] > c.get(24, 10)[3] + 0.2);
    assert!(c.get(24, 30)[3] < 0.75 + 1e-4, "75% default opacity");
}

#[test]
fn outside_stroke_grows_the_bounds() {
    let mut d = doc();
    assert_eq!(bounds(&d, "Box"), RectI::new(16, 12, 16, 16));
    fx("fx stroke", &[("width", "3"), ("color", "#ff0000")], &mut d);
    assert_eq!(bounds(&d, "Box"), RectI::new(13, 9, 22, 22));
    let c = compose::composite(&d).unwrap();
    assert_eq!(c.get(14, 20), [1.0, 0.0, 0.0, 1.0], "the ring is red and opaque");
    assert!((c.get(17, 20)[0] - 0.5).abs() < 1e-5, "the content stays on top");
    assert_eq!(c.get(11, 20)[3], 0.0);

    // center: half outside; inside: no growth
    fx("fx stroke", &[("width", "4"), ("position", "center"), ("color", "#00ff00")], &mut d);
    assert_eq!(d.layers[0].fx.len(), 1);
    assert_eq!(bounds(&d, "Box"), RectI::new(14, 10, 20, 20));
    let c = compose::composite(&d).unwrap();
    assert_eq!(c.get(17, 20), [0.0, 1.0, 0.0, 1.0]);
    fx("fx stroke", &[("width", "4"), ("position", "inside"), ("color", "#0000ff")], &mut d);
    assert_eq!(bounds(&d, "Box"), RectI::new(16, 12, 16, 16));
    let c = compose::composite(&d).unwrap();
    assert_eq!(c.get(17, 20), [0.0, 0.0, 1.0, 1.0]);
    assert!((c.get(24, 20)[0] - 0.5).abs() < 1e-5, "the middle is not stroked");
}

#[test]
fn inner_effects_stay_within_alpha() {
    let cases: Vec<(&str, Vec<(&str, &str)>)> = vec![
        ("fx inner-shadow", vec![("offset", "3,3"), ("blur", "6"), ("opacity", "100")]),
        ("fx inner-glow", vec![("blur", "6"), ("color", "#ff0000"), ("blend", "normal")]),
        ("fx inner-glow", vec![("blur", "6"), ("source", "center"), ("blend", "normal"), ("color", "#ff0000")]),
        ("fx satin", vec![("offset", "4,4"), ("blur", "6"), ("opacity", "100")]),
        ("fx overlay", vec![("color", "#ff0000")]),
        ("fx overlay", vec![("stops", "#ff0000@0,#0000ff@1"), ("angle", "0")]),
        ("fx bevel", vec![("style", "inner"), ("size", "5")]),
        ("fx stroke", vec![("width", "2"), ("position", "inside"), ("color", "#ff0000")]),
    ];
    for (name, pairs) in cases {
        let mut d = doc();
        let before = compose::composite(&d).unwrap();
        fx(name, &pairs, &mut d);
        assert_eq!(bounds(&d, "Box"), RectI::new(16, 12, 16, 16), "{name} {pairs:?} grew the bounds");
        let c = compose::composite(&d).unwrap();
        for y in 0..H {
            for x in 0..W {
                if !inside(x, y) {
                    assert_eq!(c.get(x, y)[3], 0.0, "{name} {pairs:?} painted outside at {x},{y}");
                } else {
                    assert_eq!(c.get(x, y)[3], 1.0, "{name} changed coverage at {x},{y}");
                }
            }
        }
        assert!(testing::max_diff(&before, &c) > 0.1, "{name} {pairs:?} had no visible effect");
    }
}

#[test]
fn inner_effects_land_where_expected() {
    // inner shadow with the light up-left darkens the top-left inner edge, not the bottom-right
    let mut d = doc();
    fx("fx inner-shadow", &[("offset", "3,3"), ("blur", "4"), ("opacity", "100")], &mut d);
    let c = compose::composite(&d).unwrap();
    assert!(c.get(17, 13)[0] < 0.3, "{:?}", c.get(17, 13));
    assert!((c.get(30, 26)[0] - 0.5).abs() < 0.02);

    // inner bevel lit from 120°: top-left edge brighter, bottom-right darker than the middle
    let mut d = doc();
    fx("fx bevel", &[("size", "6"), ("depth", "150")], &mut d);
    let c = compose::composite(&d).unwrap();
    let mid = c.get(24, 20)[0];
    assert!(c.get(17, 20)[0] > mid + 0.05, "left edge {:?} vs {mid}", c.get(17, 20));
    assert!(c.get(24, 13)[0] > mid + 0.05, "top edge");
    assert!(c.get(30, 20)[0] < mid - 0.05, "right edge {:?}", c.get(30, 20));
    assert!(c.get(24, 26)[0] < mid - 0.05, "bottom edge");

    // gradient overlay at 0°: first stop on the left
    let mut d = doc();
    fx("fx overlay", &[("stops", "#ff0000@0,#0000ff@1"), ("angle", "0")], &mut d);
    assert_eq!(d.layers[0].fx[0].params["kind"], "gradient");
    let c = compose::composite(&d).unwrap();
    assert!(c.get(16, 20)[0] > 0.9 && c.get(31, 20)[2] > 0.9);
}

#[test]
fn outer_effects_reach_outside() {
    for (name, pairs) in [
        ("fx outer-glow", vec![("blur", "8"), ("color", "#ffff00"), ("blend", "normal")]),
        ("fx outer-glow", vec![("blur", "8"), ("stops", "#ffff00@0,#ff0000@1")]),
        ("fx bevel", vec![("style", "outer"), ("size", "6")]),
        ("fx bevel", vec![("style", "emboss"), ("size", "6")]),
        ("fx bevel", vec![("style", "pillow"), ("size", "6"), ("soften", "2")]),
    ] {
        let mut d = doc();
        fx(name, &pairs, &mut d);
        let b = bounds(&d, "Box");
        assert!(b.x < 16 && b.y < 12 && b.right() > 32 && b.bottom() > 28, "{name} {pairs:?}: {b:?}");
        let c = compose::composite(&d).unwrap();
        assert_eq!(c.get(24, 20)[3], 1.0);
    }
}

#[test]
fn disabled_specs_render_nothing() {
    let mut d = doc();
    let plain = compose::composite(&d).unwrap();
    fx("fx drop-shadow", &[("offset", "0,8"), ("enabled", "off")], &mut d);
    assert!(!d.layers[0].fx[0].enabled);
    assert_eq!(testing::max_diff(&plain, &compose::composite(&d).unwrap()), 0.0);

    // --enabled alone toggles the existing effect and keeps its settings
    fx("fx drop-shadow", &[("offset", "0,8"), ("blur", "2")], &mut d);
    assert!(testing::max_diff(&plain, &compose::composite(&d).unwrap()) > 0.1);
    let r = fx("fx drop-shadow", &[("enabled", "off")], &mut d);
    assert_eq!(r["action"], "toggled");
    let spec = &d.layers[0].fx[0];
    assert!(!spec.enabled);
    assert_eq!(spec.params["offset"], "0,8");
    assert_eq!(spec.params["blur"], "2");
    assert_eq!(testing::max_diff(&plain, &compose::composite(&d).unwrap()), 0.0);
    // a disabled spec is skipped even when every other spec is disabled too
    let rendered = compose::render_layer(&d, &d.layers[0]).unwrap().unwrap();
    assert_eq!((rendered.x, rendered.y, rendered.image.width), (0, 0, W));
}

#[test]
fn transparent_layers_do_not_panic() {
    let mut d = testing::doc_layers(W, H, vec![("Empty", Image::new(W, H))]);
    pixelox_fx::install();
    fx("fx drop-shadow", &[("blur", "10")], &mut d);
    fx("fx outer-glow", &[("blur", "10")], &mut d);
    fx("fx stroke", &[("width", "5")], &mut d);
    fx("fx bevel", &[("style", "emboss")], &mut d);
    fx("fx satin", &[], &mut d);
    fx("fx inner-glow", &[], &mut d);
    fx("fx overlay", &[], &mut d);
    let c = compose::composite(&d).unwrap();
    assert!(c.data.chunks(4).all(|p| p[3] == 0.0));
    assert_eq!(compose::layer_bounds(&d, &d.layers[0]).unwrap(), None);
    // a 1x1 layer with every effect
    let mut d = testing::doc_layers(1, 1, vec![("Dot", Image::filled(1, 1, Rgba::WHITE))]);
    for name in [
        "fx drop-shadow",
        "fx inner-shadow",
        "fx outer-glow",
        "fx inner-glow",
        "fx bevel",
        "fx satin",
        "fx overlay",
        "fx stroke",
    ] {
        fx(name, &[], &mut d);
    }
    assert_eq!(d.layers[0].fx.len(), 8);
    assert!(compose::composite(&d).is_ok());
}

#[test]
fn copy_and_scale_round_trip() {
    pixelox_fx::install();
    let mut d = testing::doc_layers(
        96,
        48,
        vec![
            ("A", square(Rgba::new(0.2, 0.4, 0.8, 1.0))),
            ("B", {
                // the same square, 48 px to the right
                let mut img = Image::new(96, 48);
                let sq = square(Rgba::new(0.2, 0.4, 0.8, 1.0));
                for y in 0..48 {
                    for x in 0..48 {
                        img.set(x + 48, y, sq.get(x, y));
                    }
                }
                img
            }),
        ],
    );
    fx("fx drop-shadow", &[("layer", "A"), ("offset", "0,6"), ("blur", "4.5")], &mut d);
    fx("fx stroke", &[("layer", "A"), ("width", "2"), ("color", "#ffffff")], &mut d);
    fx("fx overlay", &[("layer", "A"), ("stops", "#000@0,#fff@1")], &mut d);
    let r = fx("fx copy", &[("from", "A"), ("to", "B")], &mut d);
    assert_eq!(r["to"], json!(["B"]));
    assert_eq!(d.layers[0].fx, d.layers[1].fx);
    let ra = compose::render_layer(&d, &d.layers[0]).unwrap().unwrap();
    let rb = compose::render_layer(&d, &d.layers[1]).unwrap().unwrap();
    assert_eq!(rb.x - ra.x, 48);
    assert_eq!(rb.y, ra.y);
    assert_eq!(testing::max_diff(&ra.image, &rb.image), 0.0, "B looks exactly like A");

    let orig: Vec<FxSpec> = d.layers[0].fx.clone();
    let before = bounds(&d, "A");
    let r = fx("fx scale", &[("layer", "A"), ("by", "200")], &mut d);
    assert_eq!(r["scaled"].as_array().unwrap().len(), 3);
    let p = &d.layers[0].fx[0].params;
    assert_eq!((p["offset"].as_str(), p["blur"].as_str()), (Some("0,12"), Some("9")));
    assert_eq!(d.layers[0].fx[1].params["width"], "4");
    let grown = bounds(&d, "A");
    assert!(grown.w > before.w && grown.h > before.h);
    fx("fx scale", &[("layer", "A"), ("by", "50")], &mut d);
    assert_eq!(d.layers[0].fx, orig, "scaling by 200% then 50% restores the settings");
    assert_eq!(bounds(&d, "A"), before);
}

#[test]
fn scale_clamps_with_a_warning() {
    let mut d = doc();
    fx("fx bevel", &[("size", "200"), ("soften", "10")], &mut d);
    let (_, warnings) = run_with_warnings(&find(&pixelox_fx::ops(), "fx scale"), &[("by", "400")], &mut d).unwrap();
    assert!(warnings.contains(&"PXW3001".to_string()));
    assert_eq!(d.layers[0].fx[0].params["size"], "250");
    assert_eq!(d.layers[0].fx[0].params["soften"], "16");
}

#[test]
fn list_and_rm() {
    let mut d = doc();
    fx("fx drop-shadow", &[], &mut d);
    fx("fx overlay", &[("color", "#ff0000")], &mut d);
    fx("fx overlay", &[("stops", "#000@0,#fff@1")], &mut d);
    fx("fx overlay", &[("color", "#00ff00")], &mut d);
    let r = fx("fx list", &[], &mut d);
    assert_eq!(r["count"], 3, "one overlay of each kind");
    let labels: Vec<&str> = r["fx"].as_array().unwrap().iter().map(|f| f["fx"].as_str().unwrap()).collect();
    assert_eq!(labels, ["drop-shadow", "color-overlay", "gradient-overlay"]);
    assert_eq!(r["fx"][1]["params"]["color"], "#00ff00");
    assert_eq!(r["fx"][0]["params"]["blend"], "multiply");

    assert_eq!(fx_err("fx rm", &[], &mut d), "PX1003");
    assert_eq!(fx_err("fx rm", &[("kind", "bevel"), ("all", "true")], &mut d), "PX1005");
    let r = fx("fx rm", &[("kind", "gradient-overlay")], &mut d);
    assert_eq!(r["count"], 1);
    assert_eq!(d.layers[0].fx.len(), 2);
    let r = fx("fx rm", &[("kind", "bevel")], &mut d);
    assert_eq!(r["count"], 0);
    fx("fx rm", &[("all", "true")], &mut d);
    assert!(d.layers[0].fx.is_empty());
}

#[test]
fn locks_kinds_and_conflicts() {
    let mut d = doc();
    d.layers[0].locked = true;
    assert_eq!(fx_err("fx drop-shadow", &[], &mut d), "PX2304");
    assert_eq!(fx_err("fx rm", &[("all", "true")], &mut d), "PX2304");
    assert_eq!(fx_err("fx scale", &[("by", "200")], &mut d), "PX2304");
    // listing a locked layer is fine
    fx("fx list", &[], &mut d);

    let mut d = doc();
    let adj = d.new_layer("Levels", LayerKind::Adjustment { op: "levels".into(), params: Map::new() });
    d.insert_above(None, adj);
    assert_eq!(fx_err("fx stroke", &[("layer", "Levels")], &mut d), "PX2303");
    assert_eq!(fx_err("fx copy", &[("from", "Box"), ("to", "Levels")], &mut d), "PX2303");
    assert_eq!(
        fx_err("fx stroke", &[("layer", "Box"), ("color", "#fff"), ("stops", "#000@0,#fff@1")], &mut d),
        "PX1005"
    );
    assert_eq!(
        fx_err("fx overlay", &[("layer", "Box"), ("kind", "color"), ("stops", "#000@0,#fff@1")], &mut d),
        "PX1005"
    );
    assert_eq!(fx_err("fx overlay", &[("layer", "Box"), ("kind", "pattern")], &mut d), "PX1003");
    assert_eq!(fx_err("fx drop-shadow", &[("layer", "Box"), ("opacity", "150")], &mut d), "PX1013");
    // gradient overlay without stops gets the default black-to-white ramp
    fx("fx overlay", &[("layer", "Box"), ("kind", "gradient")], &mut d);
    assert_eq!(d.layers[0].fx[0].params["stops"], "#000000@0,#ffffff@1");
}

#[test]
fn blend_modes_and_dissolve_are_deterministic() {
    let mut d = doc();
    fx("fx overlay", &[("color", "#ffffff"), ("blend", "multiply")], &mut d);
    let c = compose::composite(&d).unwrap();
    assert!((c.get(24, 20)[0] - 0.5).abs() < 1e-5, "multiply by white is identity");
    fx("fx overlay", &[("color", "#ffffff"), ("blend", "screen")], &mut d);
    assert!(compose::composite(&d).unwrap().get(24, 20)[0] > 0.99);

    let mut d = doc();
    fx("fx drop-shadow", &[("blur", "6"), ("blend", "dissolve"), ("opacity", "100")], &mut d);
    let a = compose::composite(&d).unwrap();
    let b = compose::composite(&d).unwrap();
    assert_eq!(a, b);
    // dissolve makes shadow pixels fully opaque or absent
    assert!(a.data.chunks(4).all(|p| p[3] == 0.0 || p[3] == 1.0));
}

// --- pattern fills: a tiny test codec stands in for pixelox-io's decoder -----------------

/// `PXT` + width (u8) + height (u8) + RGBA8 pixels.
fn test_decode(bytes: &[u8], _ext: Option<&str>) -> Result<Image> {
    if bytes.len() < 5 || &bytes[..3] != b"PXT" {
        return Err(Error::new("PX1103", "not a test tile"));
    }
    let (w, h) = (bytes[3] as u32, bytes[4] as u32);
    Ok(Image::from_rgba8(w, h, &bytes[5..5 + (w * h * 4) as usize]))
}

fn tile_bytes() -> Vec<u8> {
    // 2x1: red, blue
    let mut b = b"PXT".to_vec();
    b.extend_from_slice(&[2, 1, 255, 0, 0, 255, 0, 0, 255, 255]);
    b
}

#[test]
fn pattern_overlay_and_stroke_embed_the_tile() {
    compose::set_decoder(test_decode);
    let tmp = TempDir::new("fx-pattern");
    let path = tmp.path("tile.pxt");
    std::fs::write(&path, tile_bytes()).unwrap();
    let from = path.to_string_lossy().to_string();

    let mut d = doc();
    let r = fx("fx overlay", &[("from", &from)], &mut d);
    assert_eq!(r["kind"], "pattern");
    let p = &d.layers[0].fx[0].params;
    let key = p["asset"].as_str().unwrap().to_string();
    assert!(d.assets.contains_key(&key));
    let c = compose::composite(&d).unwrap();
    // tiled from the canvas origin: even columns red, odd columns blue
    assert_eq!(c.get(16, 20), [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(c.get(17, 20), [0.0, 0.0, 1.0, 1.0]);
    assert_eq!(c.get(10, 20)[3], 0.0);

    // scale 200%: the pattern repeats every 4 px and still wraps seamlessly
    fx("fx overlay", &[("from", &from), ("scale", "200")], &mut d);
    assert_eq!(d.layers[0].fx.len(), 1);
    let c = compose::composite(&d).unwrap();
    for x in 16..28 {
        assert_eq!(c.get(x, 20), c.get(x + 4, 20), "period 4 at x = {x}");
    }
    assert!(c.get(16, 20)[0] > 0.7 && c.get(18, 20)[2] > 0.7);
    // 50%: the tile shrinks to one (purple) pixel
    fx("fx overlay", &[("from", &from), ("scale", "50")], &mut d);
    let c = compose::composite(&d).unwrap();
    assert_eq!(c.get(16, 20), c.get(17, 20));

    let mut d = doc();
    fx("fx stroke", &[("from", &from), ("width", "2")], &mut d);
    assert_eq!(d.layers[0].fx[0].params["fill"], "pattern");
    let c = compose::composite(&d).unwrap();
    assert_eq!(c.get(14, 20), [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(c.get(15, 20), [0.0, 0.0, 1.0, 1.0]);

    // fx scale scales a pattern's tile size too
    fx("fx scale", &[("by", "300")], &mut d);
    assert_eq!(d.layers[0].fx[0].params["scale"], "300");

    // an undecodable tile fails at once
    let bad = tmp.path("bad.pxt");
    std::fs::write(&bad, b"nope").unwrap();
    let mut d = doc();
    assert_eq!(fx_err("fx overlay", &[("from", &bad.to_string_lossy())], &mut d), "PX1103");
}
