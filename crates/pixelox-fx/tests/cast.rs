//! Behavior tests for the cast styles (`fx long-shadow`, `fx extrude`, `fx reflection`) and
//! for content cut by the canvas edge, which every style treats as continuing past it.

use pixelox_core::prelude::*;
use pixelox_core::testing::{self, find, run};

/// A `w`×`h` document with one raster layer `Box`: an opaque square of side `s` at (x, y).
fn doc_with_square(w: u32, h: u32, x: u32, y: u32, s: u32, c: Rgba) -> Document {
    pixelox_fx::install();
    let img =
        Image::from_fn(
            w,
            h,
            move |px, py| {
                if px >= x && px < x + s && py >= y && py < y + s {
                    c.to_array()
                } else {
                    [0.0; 4]
                }
            },
        );
    testing::doc_layers(w, h, vec![("Box", img)])
}

fn fx(name: &str, pairs: &[(&str, &str)], d: &mut Document) -> Value {
    run(&find(&pixelox_fx::ops(), name), pairs, d).unwrap_or_else(|e| panic!("{name}: {e:?}"))
}

fn canvas(d: &Document) -> Image {
    compose::composite(d).unwrap()
}

fn bounds(d: &Document) -> RectI {
    let l = d.layer(&d.resolve("Box").unwrap());
    compose::layer_bounds(d, l).unwrap().unwrap()
}

const RED: Rgba = Rgba { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };

#[test]
fn long_shadows_run_to_the_canvas_edge() {
    let mut d = doc_with_square(200, 160, 20, 20, 30, RED);
    let r = fx("fx long-shadow", &[("layer", "Box"), ("blend", "normal"), ("opacity", "100")], &mut d);
    assert_eq!(r["action"], "added");
    let b = bounds(&d);
    assert_eq!((b.x, b.y), (20, 20), "nothing up or left of the square");
    // the diagonal band leaves through the bottom edge (it would reach x = 193 there)
    assert!(b.y + b.h as i32 >= 160, "down-right all the way to the canvas edge: {b:?}");
    assert!((193..=194).contains(&(b.x + b.w as i32)), "{b:?}");
    let img = canvas(&d);
    // along the 315° diagonal from the square, black; beside it, clear
    assert!(img.get(120, 120)[3] > 0.99 && img.get(120, 120)[0] < 0.01);
    assert_eq!(img.get(150, 30)[3], 0.0);
    assert_eq!(img.get(10, 10)[3], 0.0);
    // the shadow's diagonal edge is anti-aliased, not stair-stepped: some partial pixels
    let partial = (0..160).filter(|y| (0..200).any(|x| (0.05..0.95).contains(&img.get(x, *y)[3]))).count();
    assert!(partial > 100, "{partial} rows with soft edges");
}

#[test]
fn long_shadow_length_and_fade() {
    let mut d = doc_with_square(300, 60, 10, 10, 20, RED);
    fx(
        "fx long-shadow",
        &[
            ("layer", "Box"),
            ("angle", "0"),
            ("length", "100"),
            ("fade", "100"),
            ("blend", "normal"),
            ("opacity", "100"),
        ],
        &mut d,
    );
    let img = canvas(&d);
    assert!(img.get(31, 20)[3] > 0.95, "full at the trailing edge");
    let mid = img.get(80, 20)[3];
    assert!((mid - 0.5).abs() < 0.05, "half way along it is half gone: {mid}");
    assert_eq!(img.get(135, 20)[3], 0.0, "ends at the length");
    let end = bounds(&d).x + bounds(&d).w as i32;
    assert!((128..=130).contains(&end), "{end}");
}

#[test]
fn extrusions_take_the_face_color_and_cast_the_shadow() {
    let mut d = doc_with_square(160, 160, 40, 40, 40, Rgba::new(1.0, 0.5, 0.0, 1.0));
    fx("fx extrude", &[("layer", "Box"), ("depth", "20"), ("darken", "50")], &mut d);
    let img = canvas(&d);
    let side = img.get(85, 85);
    assert!((side[0] - 0.5).abs() < 0.03 && (side[1] - 0.25).abs() < 0.03, "half as bright as the face: {side:?}");
    assert_eq!(img.get(60, 60), [1.0, 0.5, 0.0, 1.0], "the face is untouched");
    assert_eq!(img.get(35, 60)[3], 0.0, "nothing behind the light side");
    let b = bounds(&d);
    assert!(
        (55..=56).contains(&b.w) && (55..=56).contains(&b.h),
        "the sides reach 20 px along the diagonal (~14 px each way): {b:?}"
    );
    // solid color, outline around the whole block
    fx(
        "fx extrude",
        &[("layer", "Box"), ("depth", "20"), ("color", "#0000ff"), ("outline", "3"), ("outline-color", "#00ff00")],
        &mut d,
    );
    let img = canvas(&d);
    assert_eq!(img.get(85, 85), [0.0, 0.0, 1.0, 1.0]);
    let ring = img.get(38, 60);
    assert!(ring[1] > 0.9 && ring[3] > 0.9, "outline left of the face: {ring:?}");
    // a long shadow on the same layer starts from the block, not from the face
    fx("fx long-shadow", &[("layer", "Box"), ("length", "40"), ("blend", "normal"), ("opacity", "100")], &mut d);
    let img = canvas(&d);
    let beyond_block = img.get(108, 108);
    assert!(beyond_block[3] > 0.9 && beyond_block[2] < 0.1, "shadow beyond the block's far corner: {beyond_block:?}");
}

#[test]
fn reflections_mirror_under_the_layer_and_fade() {
    // top half red, bottom half blue: the reflection starts blue at the floor line
    pixelox_fx::install();
    let img = Image::from_fn(80, 120, |x, y| match (x, y) {
        (20..60, 10..30) => [1.0, 0.0, 0.0, 1.0],
        (20..60, 30..50) => [0.0, 0.0, 1.0, 1.0],
        _ => [0.0; 4],
    });
    let mut d = testing::doc_layers(80, 120, vec![("Box", img)]);
    fx("fx reflection", &[("layer", "Box"), ("length", "100%"), ("opacity", "50"), ("fade", "0")], &mut d);
    let out = canvas(&d);
    let first = out.get(40, 50);
    assert!(first[2] > 0.99 && (first[3] - 0.5).abs() < 0.01, "blue at half opacity under the floor line: {first:?}");
    let last = out.get(40, 89);
    assert!(last[0] > 0.99 && (last[3] - 0.5).abs() < 0.01, "red at the far end: {last:?}");
    assert_eq!(out.get(40, 91)[3], 0.0);
    // a gap moves it down; a fade empties the far end
    fx("fx reflection", &[("layer", "Box"), ("gap", "6"), ("length", "40"), ("opacity", "50")], &mut d);
    let out = canvas(&d);
    assert_eq!(out.get(40, 53)[3], 0.0, "the gap");
    assert!(out.get(40, 57)[3] > 0.4);
    assert!(out.get(40, 94)[3] < 0.05, "faded out at the end");
    let b = bounds(&d);
    assert_eq!(b.y + b.h as i32, 96);
}

#[test]
fn blurred_reflections_grow_softer_with_distance() {
    let mut d = doc_with_square(120, 200, 40, 20, 40, RED);
    fx(
        "fx reflection",
        &[("layer", "Box"), ("length", "60"), ("blur", "16"), ("fade", "0"), ("opacity", "100")],
        &mut d,
    );
    let out = canvas(&d);
    // near the floor line the side edge is sharp; half way down (the square is 40 px tall,
    // so its mirror image ends 40 px below the floor) it is spread out
    let edge = |y: u32| (30..50).filter(|x| (0.05..0.95).contains(&out.get(*x, y)[3])).count();
    assert!(edge(62) <= 4, "sharp near the floor: {}", edge(62));
    assert!(edge(90) >= 10, "soft farther away: {}", edge(90));
}

#[test]
fn content_cut_by_the_canvas_continues_past_it() {
    // a square touching the left canvas edge, with an inner glow
    let mut d = doc_with_square(100, 100, 0, 30, 40, Rgba::new(0.2, 0.2, 0.2, 1.0));
    fx(
        "fx inner-glow",
        &[("layer", "Box"), ("blur", "16"), ("color", "#ffffff"), ("blend", "normal"), ("opacity", "100")],
        &mut d,
    );
    let img = canvas(&d);
    let at_canvas_edge = img.get(0, 50);
    let at_real_edge = img.get(39, 50);
    assert!(at_canvas_edge[0] < 0.25, "no glow along the canvas edge: {at_canvas_edge:?}");
    assert!(at_real_edge[0] > 0.6, "glow along the shape's own edge: {at_real_edge:?}");
    // an offset drop shadow continues to the canvas edge instead of stopping short
    let mut d = doc_with_square(100, 100, 0, 30, 40, Rgba::new(0.2, 0.2, 0.2, 1.0));
    fx(
        "fx drop-shadow",
        &[("layer", "Box"), ("offset", "12,12"), ("blur", "0"), ("blend", "normal"), ("opacity", "100")],
        &mut d,
    );
    let img = canvas(&d);
    assert!(img.get(3, 75)[3] > 0.99, "the shadow reaches the left edge: {:?}", img.get(3, 75));
}

#[test]
fn list_rm_and_scale_know_the_new_kinds() {
    let mut d = doc_with_square(100, 100, 30, 30, 20, RED);
    fx("fx long-shadow", &[("layer", "Box"), ("length", "40")], &mut d);
    fx("fx extrude", &[("layer", "Box"), ("depth", "10%"), ("outline", "2")], &mut d);
    fx("fx reflection", &[("layer", "Box"), ("gap", "4"), ("blur", "6")], &mut d);
    let l = fx("fx list", &[("layer", "Box")], &mut d);
    assert_eq!(l["count"], 3);
    let s = fx("fx scale", &[("layer", "Box"), ("by", "200")], &mut d);
    let changed = s["scaled"].to_string();
    assert!(changed.contains("\"40\",\"80\""), "px length doubles: {changed}");
    assert!(!changed.contains("10%"), "% depth is relative: {changed}");
    assert!(changed.contains("\"2\",\"4\"") && changed.contains("\"4\",\"8\""), "outline and gap: {changed}");
    for kind in ["long-shadow", "extrude", "reflection"] {
        let r = fx("fx rm", &[("layer", "Box"), ("kind", kind)], &mut d);
        assert_eq!(r["count"], 1, "{kind}");
    }
}

#[test]
fn work_on_every_layer_kind_and_replay_from_their_params() {
    // the specs hold every param as given, so a document re-renders identically
    let mut d = doc_with_square(120, 120, 30, 30, 30, RED);
    fx("fx extrude", &[("layer", "Box"), ("angle", "300"), ("depth", "15")], &mut d);
    fx("fx long-shadow", &[("layer", "Box"), ("angle", "300"), ("length", "50%"), ("fade", "40")], &mut d);
    let a = canvas(&d);
    let again = d.clone();
    assert_eq!(a.data, canvas(&again).data);
    let spec = &d.layer(&d.resolve("Box").unwrap()).fx[1];
    assert_eq!(spec.kind, "long-shadow");
    assert_eq!(spec.params["length"], "50%");
    assert_eq!(spec.params["angle"], "300");
}
