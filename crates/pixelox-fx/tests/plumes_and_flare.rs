//! Behavior tests for `fx fire`, `fx smoke`, and `render lens-flare`.

use pixelox_core::prelude::*;
use pixelox_core::testing::{self, find, run};

const W: u32 = 96;
const H: u32 = 120;
/// The subject: a square at x 36..60, y 70..94.
const SQ: (u32, u32, u32, u32) = (36, 70, 24, 24);
const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

fn square() -> Image {
    let (x0, y0, w, h) = SQ;
    Image::from_fn(W, H, move |x, y| if x >= x0 && x < x0 + w && y >= y0 && y < y0 + h { GRAY } else { [0.0; 4] })
}

fn inside(x: u32, y: u32) -> bool {
    let (x0, y0, w, h) = SQ;
    x >= x0 && x < x0 + w && y >= y0 && y < y0 + h
}

/// A one-layer document: a gray square named `Box` on transparency.
fn doc() -> Document {
    pixelox_fx::install();
    testing::doc_layers(W, H, vec![("Box", square())])
}

fn fx(name: &str, pairs: &[(&str, &str)], d: &mut Document) -> Value {
    run(&find(&pixelox_fx::ops(), name), pairs, d).unwrap_or_else(|e| panic!("{name}: {e:?}"))
}

fn fx_err(name: &str, pairs: &[(&str, &str)], d: &mut Document) -> String {
    run(&find(&pixelox_fx::ops(), name), pairs, d).expect_err("expected an error").code
}

/// Sum of the composite's alpha above, below, left of, and right of the square
/// (each region is the full strip beyond that side).
fn energy(img: &Image) -> [f64; 4] {
    let (x0, y0, w, h) = SQ;
    let mut e = [0.0; 4];
    for y in 0..img.height {
        for x in 0..img.width {
            let a = img.get(x, y)[3] as f64;
            if y < y0 {
                e[0] += a;
            }
            if y >= y0 + h {
                e[1] += a;
            }
            if x < x0 {
                e[2] += a;
            }
            if x >= x0 + w {
                e[3] += a;
            }
        }
    }
    e
}

/// The box around every pixel with any coverage.
fn painted(img: &Image) -> RectI {
    img.alpha_bbox(0.0).expect("something drawn")
}

#[test]
fn fire_and_smoke_rise_from_the_outline() {
    for (name, extra) in [("fx fire", vec![]), ("fx smoke", vec![("density", "80")])] {
        let mut d = doc();
        let mut pairs = vec![("height", "30"), ("variant", "11")];
        pairs.extend(extra);
        let r = fx(name, &pairs, &mut d);
        assert_eq!(r["action"], "added");
        let spec = &d.layers[0].fx[0];
        assert_eq!(spec.kind, name.trim_start_matches("fx "));
        assert_eq!(spec.params["variant"], "11");
        let c = compose::composite(&d).unwrap();
        let [up, down, ..] = energy(&c);
        assert!(up > 20.0, "{name}: visible above the subject ({up})");
        assert!(up > 8.0 * down, "{name}: rises (above {up}, below {down})");
        // nothing far from the subject: within the height above, a little to the sides
        // and below
        let b = painted(&c);
        let (x0, y0, w, h) = (SQ.0 as i32, SQ.1 as i32, SQ.2 as i32, SQ.3 as i32);
        assert!(b.y >= y0 - 30 - 8, "{name}: top {b:?}");
        assert!(b.bottom() <= y0 + h + 8, "{name}: bottom {b:?}");
        let side = if name == "fx fire" { 16 } else { 32 };
        assert!(b.x >= x0 - side && b.right() <= x0 + w + side, "{name}: sides {b:?}");
        // the layer's bounds grew to hold it
        let lb = compose::layer_bounds(&d, &d.layers[0]).unwrap().unwrap();
        assert!(lb.y < y0 - 8 && lb.h > SQ.3 + 8, "{name}: bounds {lb:?}");
    }
}

#[test]
fn the_angle_turns_the_plume() {
    let mut d = doc();
    fx("fx fire", &[("height", "24"), ("angle", "0"), ("variant", "2")], &mut d);
    let [up, down, left, right] = energy(&compose::composite(&d).unwrap());
    assert!(right > 8.0 * left, "fire to the right: right {right} left {left}");
    assert!(right > up && right > down);

    let mut d = doc();
    fx("fx smoke", &[("height", "24"), ("drift", "180"), ("density", "80"), ("variant", "2")], &mut d);
    let [_, _, left, right] = energy(&compose::composite(&d).unwrap());
    assert!(left > 8.0 * right, "smoke to the left: left {left} right {right}");

    // straight down: below, not above
    let mut d = doc();
    fx("fx fire", &[("height", "24"), ("angle", "270"), ("variant", "2")], &mut d);
    let [up, down, ..] = energy(&compose::composite(&d).unwrap());
    assert!(down > 8.0 * up, "fire downward: down {down} up {up}");
}

#[test]
fn height_bounds_the_flames() {
    let top = |h: &str| {
        let mut d = doc();
        fx("fx fire", &[("height", h), ("intensity", "200"), ("variant", "4")], &mut d);
        painted(&compose::composite(&d).unwrap()).y
    };
    let (short, tall) = (top("8"), top("40"));
    let y0 = SQ.1 as i32;
    assert!(short >= y0 - 8 - 6, "8 px flames reach {short}");
    assert!(tall >= y0 - 40 - 8, "40 px flames reach {tall}");
    assert!(tall < short - 12, "taller flames reach higher: {tall} vs {short}");
    // percent heights are of the layer's height (24 px): 100% ~ 24 px
    assert_eq!(top("100%"), top("24"));
}

#[test]
fn front_zero_leaves_the_subject_untouched() {
    let mut d = doc();
    fx("fx fire", &[("front", "0"), ("intensity", "200"), ("variant", "5")], &mut d);
    let c = compose::composite(&d).unwrap();
    for y in 0..H {
        for x in 0..W {
            if inside(x, y) {
                assert_eq!(c.get(x, y), GRAY, "subject pixel {x},{y} changed");
            }
        }
    }
    // with --front, flames lick over the subject's edges
    fx("fx fire", &[("front", "100"), ("intensity", "200"), ("variant", "5")], &mut d);
    assert_eq!(d.layers[0].fx.len(), 1, "replaces the fire");
    let c = compose::composite(&d).unwrap();
    let changed = (0..H)
        .flat_map(|y| (0..W).map(move |x| (x, y)))
        .filter(|&(x, y)| inside(x, y) && (c.get(x, y)[0] - GRAY[0]).abs() > 0.1);
    assert!(changed.count() > 10, "front flames cover some of the subject");
}

#[test]
fn variants_are_stored_deterministic_and_thread_invariant() {
    let render = |pairs: &[(&str, &str)]| {
        let mut d = doc();
        fx("fx fire", pairs, &mut d);
        (compose::composite(&d).unwrap(), d)
    };
    let (a, _) = render(&[("variant", "7")]);
    let (b, _) = render(&[("variant", "7")]);
    assert_eq!(a, b, "same variant, same flames");
    let (c, _) = render(&[("variant", "8")]);
    assert!(testing::max_diff(&a, &c) > 0.1, "another variant draws other flames");

    // auto draws a pattern from the op's RNG (the same on every run) and stores it
    let (_, d1) = render(&[]);
    let (_, d2) = render(&[]);
    let v = d1.layers[0].fx[0].params["variant"].as_str().unwrap().to_string();
    assert!(v.parse::<u64>().is_ok(), "{v}");
    assert_eq!(d2.layers[0].fx[0].params["variant"], v.as_str());
    // replacing the style with auto keeps the pattern; an explicit number changes it
    let mut d = d1;
    fx("fx fire", &[("height", "20")], &mut d);
    assert_eq!(d.layers[0].fx[0].params["variant"], v.as_str());
    fx("fx fire", &[("height", "20"), ("variant", "99")], &mut d);
    assert_eq!(d.layers[0].fx[0].params["variant"], "99");

    // thread count never changes the bytes
    for kind in ["fx fire", "fx smoke"] {
        let mut d = doc();
        fx(
            kind,
            &[("variant", "3"), ("angle", "75"), ("turbulence", "90")].map(|(k, v)| {
                if kind == "fx smoke" && k == "angle" {
                    ("drift", v)
                } else {
                    (k, v)
                }
            }),
            &mut d,
        );
        let pool = |n| rayon::ThreadPoolBuilder::new().num_threads(n).build().unwrap();
        let one = pool(1).install(|| compose::composite(&d).unwrap());
        let many = pool(5).install(|| compose::composite(&d).unwrap());
        assert_eq!(one, many, "{kind}: 1 vs 5 threads");
    }
}

#[test]
fn styles_survive_save_reload_and_moves() {
    let mut d = doc();
    fx("fx smoke", &[("height", "40"), ("variant", "21")], &mut d);
    fx("fx fire", &[("height", "25"), ("variant", "9"), ("front", "30")], &mut d);
    d.quantize_to_storage();
    let before = compose::composite(&d).unwrap();
    let back = pixelox_io::pxo::read(&pixelox_io::pxo::write(&d).unwrap()).unwrap();
    assert_eq!(back.layers[0].fx, d.layers[0].fx);
    assert_eq!(compose::composite(&back).unwrap(), before, "same render after .pxo save and reload");

    // moving the layer moves the flames and smoke with it, unchanged
    let r0 = compose::render_layer(&d, &d.layers[0]).unwrap().unwrap();
    geom::translate_layer(&mut d, &vec![0], 7.0, -5.0).unwrap();
    let r1 = compose::render_layer(&d, &d.layers[0]).unwrap().unwrap();
    assert_eq!((r1.x, r1.y), (r0.x + 7, r0.y - 5));
    assert_eq!(r1.image, r0.image);
}

#[test]
fn rm_scale_list_and_edge_cases() {
    let mut d = doc();
    fx("fx fire", &[("height", "24")], &mut d);
    fx("fx smoke", &[("height", "50%")], &mut d);
    let r = fx("fx list", &[], &mut d);
    let labels: Vec<&str> = r["fx"].as_array().unwrap().iter().map(|f| f["fx"].as_str().unwrap()).collect();
    assert_eq!(labels, ["fire", "smoke"]);
    // px heights scale, percent heights are relative
    fx("fx scale", &[("by", "200")], &mut d);
    assert_eq!(d.layers[0].fx[0].params["height"], "48");
    assert_eq!(d.layers[0].fx[1].params["height"], "50%");
    let r = fx("fx rm", &[("kind", "fire")], &mut d);
    assert_eq!(r["count"], 1);
    assert_eq!(d.layers[0].fx[0].kind, "smoke");
    // --enabled alone toggles and keeps the pattern
    let v = d.layers[0].fx[0].params["variant"].clone();
    let r = fx("fx smoke", &[("enabled", "off")], &mut d);
    assert_eq!(r["action"], "toggled");
    assert_eq!(d.layers[0].fx[0].params["variant"], v);

    // intensity 0 draws nothing; out-of-range values are refused
    let mut d = doc();
    let plain = compose::composite(&d).unwrap();
    fx("fx fire", &[("intensity", "0")], &mut d);
    assert_eq!(compose::composite(&d).unwrap(), plain);
    assert_eq!(fx_err("fx fire", &[("front", "101")], &mut d), "PX1013");
    assert_eq!(fx_err("fx smoke", &[("density", "-1")], &mut d), "PX1013");
    assert_eq!(fx_err("fx fire", &[("height", "tall")], &mut d), "PX1011");

    // transparent and 1x1 layers do not panic
    let mut d = testing::doc_layers(W, H, vec![("Empty", Image::new(W, H))]);
    fx("fx fire", &[], &mut d);
    fx("fx smoke", &[], &mut d);
    assert!(compose::composite(&d).unwrap().data.chunks(4).all(|p| p[3] == 0.0));
    let mut d = testing::doc_layers(1, 1, vec![("Dot", Image::filled(1, 1, Rgba::WHITE))]);
    fx("fx fire", &[("angle", "33")], &mut d);
    fx("fx smoke", &[], &mut d);
    assert!(compose::composite(&d).is_ok());
}

// --- render lens-flare -------------------------------------------------------------

const FW: u32 = 160;
const FH: u32 = 100;

fn night() -> Document {
    testing::doc_layers(FW, FH, vec![("Background", Image::filled(FW, FH, Rgba::BLACK))])
}

fn flare(pairs: &[(&str, &str)], d: &mut Document) -> Value {
    fx("render lens-flare", pairs, d)
}

fn lum(img: &Image, x: u32, y: u32) -> f32 {
    let p = img.get(x, y);
    color::luma(p[0], p[1], p[2])
}

/// Mean luminance over a disc (clipped to the image).
fn disc(img: &Image, cx: f64, cy: f64, r: f64) -> f64 {
    let (mut s, mut n) = (0.0f64, 0.0f64);
    for y in (cy - r).floor() as i64..=(cy + r).ceil() as i64 {
        for x in (cx - r).floor() as i64..=(cx + r).ceil() as i64 {
            let inside_disc = (x as f64 + 0.5 - cx).hypot(y as f64 + 0.5 - cy) <= r;
            if inside_disc && x >= 0 && y >= 0 && x < img.width as i64 && y < img.height as i64 {
                s += lum(img, x as u32, y as u32) as f64;
                n += 1.0;
            }
        }
    }
    s / n.max(1.0)
}

fn mean_lum(img: &Image) -> f64 {
    let mut s = 0.0;
    for y in 0..img.height {
        for x in 0..img.width {
            s += lum(img, x, y) as f64;
        }
    }
    s / img.pixels() as f64
}

#[test]
fn lens_flare_adds_one_screen_layer() {
    let mut d = night();
    let r = flare(&[("at", "40,30")], &mut d);
    assert_eq!(d.layers.len(), 2);
    let l = &d.layers[1];
    assert_eq!((l.name.as_str(), l.blend), ("Lens Flare", BlendMode::Screen));
    assert_eq!(d.active.as_deref(), Some(l.id.as_str()));
    assert_eq!(r["layer"], "Lens Flare");
    assert_eq!(r["blend"], "screen");
    assert_eq!(r["above"], "Background");
    assert_eq!(r["at"], json!([40.0, 30.0]));
    assert_eq!(r["bounds"].as_array().map(|b| b.len()), Some(4));
    // a second flare gets a unique name; --name must be free
    flare(&[], &mut d);
    assert_eq!(d.layers[2].name, "Lens Flare 2");
    assert_eq!(fx_err("render lens-flare", &[("name", "Background")], &mut d), "PX2305");
    flare(&[("name", "Sun")], &mut d);
    assert_eq!(d.layers[3].name, "Sun");
    // --layer puts it above that layer instead of above the active one
    flare(&[("layer", "Background"), ("name", "Low")], &mut d);
    assert_eq!(d.layers[1].name, "Low");
    // the background's own pixels are untouched: the flare is only on its layer
    assert!(testing::raster(&d, "Background").data.chunks(4).all(|p| p == [0.0, 0.0, 0.0, 1.0]));
    assert_eq!(fx_err("render lens-flare", &[("lens", "fisheye")], &mut d), "PX1010");
    assert_eq!(fx_err("render lens-flare", &[("brightness", "500")], &mut d), "PX1013");
    assert_eq!(fx_err("render lens-flare", &[("color", "black")], &mut d), "PX1010");
}

#[test]
fn the_core_is_brightest_at_the_flare_center() {
    for lens in ["zoom", "35mm", "105mm", "movie"] {
        let mut d = night();
        flare(&[("at", "40,30"), ("lens", lens)], &mut d);
        let c = compose::composite(&d).unwrap();
        let (mut best, mut at) = (-1.0f32, (0, 0));
        for y in 0..FH {
            for x in 0..FW {
                if lum(&c, x, y) > best {
                    best = lum(&c, x, y);
                    at = (x, y);
                }
            }
        }
        let dist = (at.0 as f64 + 0.5 - 40.0).hypot(at.1 as f64 + 0.5 - 30.0);
        assert!(dist <= 1.5, "{lens}: brightest at {at:?}");
        assert!(best > 0.95, "{lens}: the core is near white ({best})");
        // and it fades away from the core
        assert!(disc(&c, 40.0, 30.0, 2.0) > disc(&c, 40.0, 30.0 + 25.0, 2.0) + 0.3, "{lens}");
    }
}

#[test]
fn ghosts_lie_on_the_line_through_the_center() {
    let mut d = night();
    let at = (30.0, 22.0);
    let r = flare(&[("at", "30,22"), ("lens", "zoom")], &mut d);
    let c = compose::composite(&d).unwrap();
    let center = (FW as f64 / 2.0, FH as f64 / 2.0);
    let axis = (center.0 - at.0, center.1 - at.1);
    let len = axis.0.hypot(axis.1);
    let ghosts = r["ghosts"].as_array().unwrap();
    assert!(ghosts.len() >= 6);
    let mut checked = 0;
    for g in ghosts {
        let (gx, gy, gr) = (g["x"].as_f64().unwrap(), g["y"].as_f64().unwrap(), g["radius"].as_f64().unwrap());
        let cross = ((gx - at.0) * axis.1 - (gy - at.1) * axis.0) / len;
        assert!(cross.abs() < 0.2, "ghost {g} is off the axis by {cross}");
        // a visible ghost is brighter than the same spot turned off the axis
        let (dx, dy) = (gx - at.0, gy - at.1);
        let th = 35f64.to_radians();
        let (ox, oy) = (at.0 + dx * th.cos() - dy * th.sin(), at.1 + dx * th.sin() + dy * th.cos());
        let on_canvas = |x: f64, y: f64| x - gr >= 0.0 && y - gr >= 0.0 && x + gr < FW as f64 && y + gr < FH as f64;
        if gr >= 4.0 && on_canvas(gx, gy) && on_canvas(ox, oy) {
            let (on, off) = (disc(&c, gx, gy, gr), disc(&c, ox, oy, gr));
            assert!(on > off + 0.02, "ghost {g}: {on} vs off-axis {off}");
            checked += 1;
        }
    }
    assert!(checked >= 2, "only {checked} ghosts checked");
}

#[test]
fn lens_types_differ_and_movie_streaks() {
    let renders: Vec<(String, Image)> = ["zoom", "35mm", "105mm", "movie"]
        .iter()
        .map(|lens| {
            let mut d = night();
            flare(&[("at", "40,40"), ("lens", lens)], &mut d);
            (lens.to_string(), compose::composite(&d).unwrap())
        })
        .collect();
    for i in 0..renders.len() {
        for j in i + 1..renders.len() {
            let diff = testing::max_diff(&renders[i].1, &renders[j].1);
            assert!(diff > 0.1, "{} and {} look alike ({diff})", renders[i].0, renders[j].0);
        }
    }
    // the movie prime's anamorphic streak runs horizontally; the zoom has none
    let streak = |img: &Image| {
        let h: f64 = (25..45).map(|d| lum(img, 40 + d, 40) as f64).sum();
        let v: f64 = (25..45).map(|d| lum(img, 40, 40 + d) as f64).sum();
        h / v.max(1e-6)
    };
    assert!(streak(&renders[3].1) > 3.0, "movie: {}", streak(&renders[3].1));
    assert!(streak(&renders[0].1) < 2.0, "zoom: {}", streak(&renders[0].1));
}

#[test]
fn brightness_scales_and_tint_colors() {
    let render = |pairs: &[(&str, &str)]| {
        let mut d = night();
        flare(pairs, &mut d);
        compose::composite(&d).unwrap()
    };
    let m: Vec<f64> = ["40", "100", "250"].iter().map(|b| mean_lum(&render(&[("brightness", b)]))).collect();
    assert!(m[0] < m[1] && m[1] < m[2], "{m:?}");
    assert!(m[2] > 1.5 * m[1], "{m:?}");
    // the same settings draw the same flare
    assert_eq!(render(&[("at", "50,20")]), render(&[("at", "50,20")]));
    // a blue tint makes it blue
    let blue = render(&[("color", "#3060ff")]);
    assert!(testing::mean(&blue, 2) > 2.0 * testing::mean(&blue, 0));
    // percent positions are of the canvas
    let mut d = night();
    let r = flare(&[("at", "50%,25%")], &mut d);
    assert_eq!(r["at"], json!([80.0, 25.0]));
}
