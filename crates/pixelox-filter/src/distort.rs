//! Distortions: `filter distort` (wave, ripple, twirl, pinch, spherize, polar
//! coordinates), `filter displace` (a displacement map from an image, a layer, or
//! generated noise), and `filter glitch` (channel split, slice shifts, scanlines).
//!
//! Every command maps each output pixel back to a source position and samples the layer
//! bilinearly with premultiplied alpha, so edges never darken. Centers and the polar box
//! default to the layer's visible bounds, which also makes them follow a placed layer's
//! content when they run as smart filters.

use crate::procedural::{self, NoiseSettings};
use crate::util::{self, Buf};
use pixelox_core::kernels::Edge;
use pixelox_core::noise::Basis;
use pixelox_core::op::ParamSpec;
use pixelox_core::prelude::*;
use pixelox_core::region::{apply_to_raster_grow, takes_smart_filters, Reach};
use pixelox_core::units::Length;
use rand_core::RngCore;
use rayon::prelude::*;
use std::f64::consts::{PI, TAU};

const CAT: Category = Category::Filter;

pub const KINDS: &[&str] = &["wave", "ripple", "twirl", "pinch", "spherize", "polar", "polar-inverse"];

fn edge_param() -> ParamSpec {
    param("edge", ParamKind::Choice)
        .choices(&["auto", "clamp", "wrap", "transparent"])
        .default("auto")
        .help("Pixels pulled from outside the layer: auto (clamp on opaque layers, transparent otherwise), clamp, wrap, or transparent")
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("filter distort", CAT, "Wave, ripple, twirl, pinch, spherize, or polar", distort)
            .about(
                "Photoshop's classic distortions. wave: parallel waves of --amplitude and --wavelength traveling along \
                 --angle (0 = left to right: horizontal lines become waves), with --shape sine, triangle, or square. \
                 ripple: concentric water ripples around --center. twirl: turns pixels around --center by --angle \
                 degrees at the center, fading to none at --radius (positive turns clockwise). pinch: --amount > 0 \
                 squeezes toward the center, < 0 bulges out. spherize: --amount > 0 wraps the layer around a sphere \
                 (magnified middle), < 0 hollows it. polar: rectangular to polar coordinates (the layer's top edge \
                 becomes the center, the left and right edges meet at the top); polar-inverse undoes it. The center \
                 and radius default to the layer's visible bounds (its center, half its shorter side). Works as a smart \
                 filter on placed layers.",
            )
            .param(param("kind", ParamKind::Choice).choices(KINDS).required().help("wave, ripple, twirl, pinch, spherize, polar, or polar-inverse"))
            .param(param("amount", ParamKind::Float).default("50").range("-100..100").help("pinch and spherize strength (negative reverses)"))
            .param(param("angle", ParamKind::Angle).auto().default("auto").help("twirl: degrees at the center (auto 90); wave: direction of travel (auto 0)"))
            .param(param("amplitude", ParamKind::Length).auto().default("auto").help("wave and ripple height in px or % of the shorter side (auto 2%)"))
            .param(param("wavelength", ParamKind::Length).auto().default("auto").help("wave and ripple length in px or % of the shorter side (auto 12%)"))
            .param(param("shape", ParamKind::Choice).choices(&["sine", "triangle", "square"]).default("sine").help("wave profile"))
            .param(param("center", ParamKind::Point).help("Center X,Y on the canvas (default: the center of the layer's visible bounds)"))
            .param(param("radius", ParamKind::Length).auto().default("auto").help("twirl, pinch, spherize, ripple: reach in px or % of the shorter side (auto: half the bounds' shorter side; ripple: everywhere)"))
            .param(edge_param())
            .params(common::region())
            .example("pixelox filter distort photo.jpg -o twirl.jpg --kind twirl --angle 180", "Twirl the middle of a photo")
            .example("pixelox filter distort doc.pxo --in-place --layer Flag --kind wave --amplitude 14 --wavelength 160", "Wave a flag layer")
            .example("pixelox filter distort doc.pxo --in-place --layer Ball --kind spherize --amount 80", "Wrap a layer around a sphere")
            .example("pixelox filter distort doc.pxo --in-place --layer Water --kind ripple --center 50%,70% --amplitude 6 --wavelength 40", "Pond ripples")
            .example("pixelox filter distort pattern.png -o ring.png --kind polar", "Bend a stripe pattern into a ring"),
        OpSpec::doc("filter displace", CAT, "Displace by an image, layer, or noise map", displace)
            .about(
                "Moves each pixel by a displacement map. --map reads an image file and --map-layer another layer of the \
                 document (both stretched over the layer's visible bounds, or repeated from the canvas origin with \
                 --fit tile); with neither, a seeded noise map is generated (--scale, --octaves). --mode rg follows \
                 Photoshop: the red channel moves pixels sideways and the green channel up and down, 50% gray staying \
                 put, by up to --amount px (a grayscale map moves pixels diagonally; the noise map has independent \
                 channels: a turbulent wobble for heat haze or water). --mode slope moves pixels along the map's \
                 brightness slopes, as if seen through glass or printed on fabric (the steepest slopes move by \
                 --amount). On a placed, text, or shape layer the noise map is recorded as a smart filter; a map file \
                 or layer is read only once, so rasterize such a layer first to use one.",
            )
            .param(param("map", ParamKind::Path).help("Displacement map image (grayscale or color)"))
            .param(param("map-layer", ParamKind::Layer).help("Use another layer of the document as the map"))
            .param(param("mode", ParamKind::Choice).choices(&["rg", "slope"]).default("rg").help("rg: red/green channels move x/y (Photoshop); slope: move along brightness slopes (glass, fabric)"))
            .param(param("amount", ParamKind::Length).default("2%").help("Largest move in px or % of the shorter canvas side"))
            .param(param("fit", ParamKind::Choice).choices(&["stretch", "tile"]).default("stretch").help("Map image or layer: stretch over the layer's bounds, or tile from the canvas origin"))
            .param(param("scale", ParamKind::Length).default("12%").help("Noise map: feature size in px or % of the shorter canvas side"))
            .param(param("octaves", ParamKind::Int).default("3").range("1..10").help("Noise map: detail layers"))
            .param(edge_param())
            .params(common::region())
            .example("pixelox filter displace doc.pxo --in-place --layer Photo --amount 1.5% --scale 4%", "Heat-haze wobble from generated noise")
            .example("pixelox filter displace doc.pxo --in-place --layer Logo --map-layer Folds --mode slope --amount 18", "Print a logo onto a fabric layer's folds")
            .example("pixelox filter displace photo.jpg -o glass.jpg --map glass.png --mode slope --amount 25", "Look through textured glass")
            .example("pixelox filter displace doc.pxo --in-place --layer Title --map ripple.png --fit tile --amount 8", "Ripple a title with a tiled map"),
        OpSpec::doc("filter glitch", CAT, "RGB split, slice shifts, scanlines", glitch)
            .about(
                "Digital damage, seeded: the red channel is pulled --split px away from green and blue (red and cyan \
                 fringes; the alpha spreads with them, so type gets colored edges), --slices horizontal bands of random \
                 heights (2 to 10% of the layer's visible content) are shifted sideways by up to --slice-shift with extra split inside \
                 them, and dark --scanlines run every --line-gap px. --seed picks another arrangement. Works as a smart \
                 filter on placed layers.",
            )
            .param(param("split", ParamKind::Length).default("1.6%").help("Red/cyan channel offset in px or % of the content width"))
            .param(param("slices", ParamKind::Int).default("6").range("0..200").help("Number of shifted horizontal bands"))
            .param(param("slice-shift", ParamKind::Length).default("6%").help("Largest sideways shift of a band in px or % of the content width"))
            .param(param("scanlines", ParamKind::Float).default("30").range("0..100").help("Darkness of the scanlines in percent (0 = none)"))
            .param(param("line-gap", ParamKind::Float).auto().default("auto").range("2..200").help("Scanline spacing in px (auto: 0.4% of the shorter canvas side, at least 3)"))
            .params(common::region())
            .example("pixelox filter glitch photo.jpg -o glitch.jpg", "Default glitch")
            .example("pixelox filter glitch doc.pxo --in-place --layer Title --split 8 --slices 3 --scanlines 0", "Split a white title into colored fringes")
            .example("pixelox filter glitch doc.pxo --in-place --layer Photo --slices 12 --slice-shift 10% --seed 7", "Heavier, reproducible damage"),
    ]
}

// ---------------------------------------------------------------------------------------
// sampling

#[derive(Clone, Copy, PartialEq)]
enum EdgeMode {
    Clamp,
    Wrap,
    Zero,
}

fn edge_mode(a: &Args, img: &Image) -> EdgeMode {
    match a.choice("edge").as_str() {
        "clamp" => EdgeMode::Clamp,
        "wrap" => EdgeMode::Wrap,
        "transparent" => EdgeMode::Zero,
        _ => match kernels::auto_edge(img) {
            Edge::Clamp => EdgeMode::Clamp,
            Edge::Zero => EdgeMode::Zero,
        },
    }
}

/// Premultiplied bilinear sample at pixel-center coordinates (centers on integers).
#[inline]
fn sample(buf: &Buf, x: f64, y: f64, edge: EdgeMode) -> [f32; 4] {
    match edge {
        EdgeMode::Wrap => {
            let mut o = [0f32; 4];
            util::sample_wrap(buf, x as f32, y as f32, &mut o);
            o
        }
        EdgeMode::Clamp => util::sample4(buf, x as f32, y as f32, Edge::Clamp),
        EdgeMode::Zero => util::sample4(buf, x as f32, y as f32, Edge::Zero),
    }
}

/// Resample `img` through `map(x, y) -> (sx, sy)` in layer pixel coordinates (centers
/// at +0.5).
fn remap(img: &Image, edge: EdgeMode, map: impl Fn(f64, f64) -> (f64, f64) + Sync) -> Image {
    let buf = Buf::premul(img);
    let w = img.width as usize;
    let mut out = Buf::new(w, img.height as usize, 4);
    out.data.par_chunks_mut(w.max(1) * 4).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let (sx, sy) = map(x as f64 + 0.5, y as f64 + 0.5);
            row[x * 4..x * 4 + 4].copy_from_slice(&sample(&buf, sx - 0.5, sy - 0.5, edge));
        }
    });
    out.into_unpremul()
}

/// The layer's visible bounds in its own pixel coordinates (x, y, w, h); the whole image
/// when it has none.
fn bounds(img: &Image) -> (f64, f64, f64, f64) {
    match img.alpha_bbox(0.0) {
        Some(r) => (r.x as f64, r.y as f64, r.w as f64, r.h as f64),
        None => (0.0, 0.0, img.width as f64, img.height as f64),
    }
}

// ---------------------------------------------------------------------------------------
// filter distort

fn wave_shape(shape: &str, phase: f64) -> f64 {
    let p = phase.rem_euclid(1.0);
    match shape {
        "triangle" => 1.0 - 4.0 * ((p + 0.25).rem_euclid(1.0) - 0.5).abs(),
        "square" => {
            if p < 0.5 {
                1.0
            } else {
                -1.0
            }
        }
        _ => (TAU * p).sin(),
    }
}

fn distort(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let kind = a.choice("kind");
    let short = doc.width.min(doc.height).max(1) as f64;
    let dpi = doc.dpi();
    let len = |name: &str, pct: f64| -> Result<f64> {
        Ok(a.length(name)?.unwrap_or(Length { value: pct, unit: units::Unit::Pct, plus: None }).to_px(short, dpi))
    };
    let amount = a.f64("amount")? / 100.0;
    let amplitude = len("amplitude", 2.0)?;
    let wavelength = len("wavelength", 12.0)?.max(1.0);
    let angle = a.opt_angle("angle")?;
    let shape = a.choice("shape");
    let center = util::canvas_point(a, "center", doc)?;
    let radius = a.length("radius")?.map(|l| l.to_px(short, dpi));
    // waves and ripples move content up to their amplitude past where it was
    let reach = match kind.as_str() {
        "wave" | "ripple" => Reach::uniform(amplitude.abs() + 1.0),
        _ => Reach::NONE,
    };
    let mut resolved = Map::new();
    let v = apply_to_raster_grow(a, doc, ctx, reach, |img, (ox, oy), _| {
        if img.is_empty() {
            return Ok(img.clone());
        }
        let (bx, by, bw, bh) = bounds(img);
        let c = match center {
            Some((cx, cy)) => (cx - ox as f64, cy - oy as f64),
            None => (bx + bw / 2.0, by + bh / 2.0),
        };
        let r_max = radius.unwrap_or(bw.min(bh) / 2.0).max(1.0);
        let edge = edge_mode(a, img);
        resolved.insert("center".into(), json!([c.0 + ox as f64, c.1 + oy as f64]));
        let out = match kind.as_str() {
            "twirl" => {
                let turn = angle.unwrap_or(90.0).to_radians();
                resolved.insert("radius".into(), json!(r_max));
                remap(img, edge, |x, y| {
                    let (dx, dy) = (x - c.0, y - c.1);
                    let r = dx.hypot(dy);
                    if r >= r_max {
                        return (x, y);
                    }
                    let k = 1.0 - r / r_max;
                    let phi = -turn * k * k;
                    let (s, co) = phi.sin_cos();
                    (c.0 + dx * co - dy * s, c.1 + dx * s + dy * co)
                })
            }
            "pinch" | "spherize" => {
                resolved.insert("radius".into(), json!(r_max));
                let pinch = kind == "pinch";
                remap(img, edge, |x, y| {
                    let (dx, dy) = (x - c.0, y - c.1);
                    let r = dx.hypot(dy);
                    if r >= r_max || r <= 1e-9 {
                        return (x, y);
                    }
                    let t = r / r_max;
                    let st = if pinch {
                        // > 0: show source from farther out (squeezed toward the center)
                        t.powf(1.0 / (1.0 + amount.max(-0.95) * 0.9))
                    } else if amount >= 0.0 {
                        t + amount * ((2.0 / PI) * t.asin() - t)
                    } else {
                        t + (-amount) * ((PI / 2.0 * t).sin() - t)
                    };
                    let k = st / t;
                    (c.0 + dx * k, c.1 + dy * k)
                })
            }
            "wave" => {
                let th = angle.unwrap_or(0.0).to_radians();
                let (ux, uy) = (th.cos(), -th.sin());
                resolved.insert("amplitude".into(), json!(amplitude));
                resolved.insert("wavelength".into(), json!(wavelength));
                remap(img, edge, |x, y| {
                    let along = (x + ox as f64) * ux + (y + oy as f64) * uy;
                    let d = amplitude * wave_shape(&shape, along / wavelength);
                    // displace across the direction of travel
                    (x - uy * d, y + ux * d)
                })
            }
            "ripple" => {
                resolved.insert("amplitude".into(), json!(amplitude));
                resolved.insert("wavelength".into(), json!(wavelength));
                let reach = radius.unwrap_or(f64::INFINITY);
                remap(img, edge, |x, y| {
                    let (dx, dy) = (x - c.0, y - c.1);
                    let r = dx.hypot(dy);
                    if r <= 1e-9 || r >= reach {
                        return (x, y);
                    }
                    let fade = if reach.is_finite() { 1.0 - r / reach } else { 1.0 };
                    let d = amplitude * (TAU * r / wavelength).sin() * fade;
                    (x + dx / r * d, y + dy / r * d)
                })
            }
            "polar" => remap(img, edge, |x, y| {
                // the box's top edge goes to the center, its left/right edges meet at the top
                let (dx, dy) = ((x - c.0) / (bw / 2.0), (y - c.1) / (bh / 2.0));
                let r = dx.hypot(dy);
                let phi = dx.atan2(-dy).rem_euclid(TAU);
                (bx + phi / TAU * bw, by + r * bh)
            }),
            _ => remap(img, edge, |x, y| {
                let (u, v) = ((x - bx) / bw, (y - by) / bh);
                let phi = u * TAU;
                (c.0 + phi.sin() * v * bw / 2.0, c.1 - phi.cos() * v * bh / 2.0)
            }),
        };
        Ok(out)
    })?;
    let mut v = v;
    if let Value::Object(m) = &mut v {
        m.insert("kind".into(), json!(kind));
        m.extend(resolved);
    }
    Ok(v)
}

// ---------------------------------------------------------------------------------------
// filter displace

/// A displacement map over canvas pixels: two channels (x, y) in −1..1, or brightness.
enum MapSrc {
    Image { img: Image, x: f64, y: f64, w: f64, h: f64, tile: bool },
    Noise(Box<(procedural::Texture, procedural::Texture)>),
}

fn displace(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let short = doc.width.min(doc.height).max(1) as f64;
    let amount = a.px("amount", short, doc.dpi())?;
    let slope = a.choice("mode") == "slope";
    let tile = a.choice("fit") == "tile";
    if a.has("map") && a.has("map-layer") {
        return Err(Error::new("PX1005", "give --map or --map-layer, not both"));
    }
    let path = target_layer(a, doc)?;
    let is_placed = takes_smart_filters(&doc.layer(&path).content);
    if is_placed && a.has("map-layer") {
        return Err(Error::new(
            "PX3342",
            "a --map-layer cannot drive a smart filter on a placed, text, or shape layer",
        )
        .with_hint(format!(
            "rasterize first (pixelox layer rasterize <doc> --layer '{}' --in-place), or use the noise map",
            doc.layer(&path).name
        ))
        .with_context("layer", &doc.layer(&path).name));
    }
    // the map (unless the op is recorded as a smart filter, which needs no map now)
    let mut map_img: Option<Image> = None;
    let mut map_name = Value::Null;
    if !is_placed {
        if let Some(p) = a.path("map") {
            let (img, _) = ctx.decode_aux(&p, "displacement map", Some("map"))?;
            if img.is_empty() {
                return Err(Error::new("PX3341", format!("displacement map {} has no pixels", p.display()))
                    .with_hint("give an image with pixels; a blurred grayscale texture works best"));
            }
            map_name = json!(p.display().to_string());
            map_img = Some(img);
        } else if let Some(r) = a.raw("map-layer") {
            let mp = doc.resolve(r)?;
            let l = doc.layer(&mp).clone();
            map_name = json!(l.name);
            let rendered = compose::render_layer(doc, &l)?.ok_or_else(|| {
                Error::new("PX3341", format!("map layer '{}' draws nothing", l.name))
                    .with_hint("pick a layer with visible pixels")
            })?;
            map_img = Some(rendered.to_canvas(doc.width, doc.height));
        }
    }
    let seed = ctx.rng().next_u64();
    let noise_settings = |kseed: u64, ctx: &mut OpCtx| -> Result<procedural::Texture> {
        let s = NoiseSettings {
            kind: "clouds".into(),
            basis: Basis::Perlin,
            scale: a.px("scale", short, doc.dpi())?.max(1.0),
            octaves: a.int("octaves")?.clamp(1, 10) as u32,
            persistence: 0.5,
            lacunarity: 2.0,
            warp: 0.0,
            stretch: 1.0,
            angle: 0.0,
            tile: false,
            contrast: 0.0,
            bias: 0.0,
        };
        Ok(procedural::texture(s, kseed, (doc.width, doc.height), ctx))
    };
    let (lx0, ly0) = {
        let l = doc.layer(&path);
        (l.x as f64, l.y as f64)
    };
    let src = match map_img {
        Some(img) => {
            let li = doc.layer(&path);
            let b = match li.raster().ok().and_then(|r| r.alpha_bbox(0.0)) {
                Some(r) => (lx0 + r.x as f64, ly0 + r.y as f64, r.w as f64, r.h as f64),
                None => (0.0, 0.0, doc.width as f64, doc.height as f64),
            };
            MapSrc::Image { img, x: b.0, y: b.1, w: b.2, h: b.3, tile }
        }
        None => MapSrc::Noise(Box::new((noise_settings(seed, ctx)?, noise_settings(seed ^ 0x5A5A_A5A5, ctx)?))),
    };
    // pixels move up to the amount (slopes: 1.5x) past where they were
    let reach = Reach::uniform(amount.abs() * if slope { 1.5 } else { 1.0 } + 1.0);
    let v = apply_to_raster_grow(a, doc, ctx, reach, |img, (gx, gy), _| {
        let (lx, ly) = (gx as f64, gy as f64);
        let (w, h) = (img.width as usize, img.height as usize);
        if w == 0 || h == 0 {
            return Ok(img.clone());
        }
        // the map's two channels per layer pixel, -1..1
        let mut field = vec![[0f32; 2]; w * h];
        match &src {
            MapSrc::Noise(t) => {
                let (tx, ty) = &**t;
                let (px, py) = (tx.plane(lx as i64, ly as i64, w, h), ty.plane(lx as i64, ly as i64, w, h));
                let k = if slope { 1.0 } else { 1.0 / (2.5 * pixelox_core::noise::SPREAD as f32) };
                for i in 0..w * h {
                    field[i] = [(px[i] - 0.5) * k, (py[i] - 0.5) * k];
                }
            }
            MapSrc::Image { img: m, x, y, w: bw, h: bh, tile } => {
                let mb = Buf::premul(m);
                field.par_chunks_mut(w).enumerate().for_each(|(j, row)| {
                    for (i, f) in row.iter_mut().enumerate() {
                        let (cx, cy) = (lx + i as f64 + 0.5, ly + j as f64 + 0.5);
                        let p = if *tile {
                            let mut o = [0f32; 4];
                            util::sample_wrap(&mb, (cx - 0.5) as f32, (cy - 0.5) as f32, &mut o);
                            o
                        } else {
                            let u = (cx - x) / bw * m.width as f64;
                            let v = (cy - y) / bh * m.height as f64;
                            util::sample4(&mb, (u - 0.5) as f32, (v - 0.5) as f32, Edge::Clamp)
                        };
                        // flatten over 50% gray (transparent = no move)
                        let gray = 0.5 * (1.0 - p[3]);
                        *f = [(p[0] + gray) * 2.0 - 1.0, (p[1] + gray) * 2.0 - 1.0];
                    }
                });
            }
        }
        let offsets: Vec<[f32; 2]> = if slope {
            // brightness: the mean of the two channels; moves along its gradient
            let lum: Vec<f32> = field
                .iter()
                .map(|f| if matches!(src, MapSrc::Image { .. }) { 0.5 * (f[0] + f[1]) } else { f[0] })
                .collect();
            let at = |i: i64, j: i64| lum[(j.clamp(0, h as i64 - 1) as usize) * w + i.clamp(0, w as i64 - 1) as usize];
            let mut g: Vec<[f32; 2]> = (0..w * h)
                .map(|k| {
                    let (i, j) = ((k % w) as i64, (k / w) as i64);
                    [(at(i + 1, j) - at(i - 1, j)) * 0.5, (at(i, j + 1) - at(i, j - 1)) * 0.5]
                })
                .collect();
            // the steepest 2% of slopes move by the full amount
            let mut mags: Vec<f32> = g.iter().map(|v| v[0].hypot(v[1])).collect();
            let n = mags.len();
            let idx = ((n as f64 * 0.98) as usize).min(n - 1);
            mags.select_nth_unstable_by(idx, |a, b| a.total_cmp(b));
            let top = mags[idx].max(1e-6);
            for v in g.iter_mut() {
                v[0] = (v[0] / top).clamp(-1.5, 1.5);
                v[1] = (v[1] / top).clamp(-1.5, 1.5);
            }
            g
        } else {
            field
        };
        let edge = edge_mode(a, img);
        let amt = amount as f32;
        Ok(remap(img, edge, |x, y| {
            let k = (y as usize).min(h - 1) * w + (x as usize).min(w - 1);
            let o = offsets[k];
            (x + (o[0] * amt) as f64, y + (o[1] * amt) as f64)
        }))
    })?;
    let mut v = v;
    if let Value::Object(m) = &mut v {
        m.insert("amount_px".into(), json!((amount * 100.0).round() / 100.0));
        m.insert("map".into(), if map_name.is_null() { json!("noise") } else { map_name });
    }
    Ok(v)
}

// ---------------------------------------------------------------------------------------
// filter glitch

fn glitch(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let short = doc.width.min(doc.height).max(1) as f64;
    let dpi = doc.dpi();
    let split_l = a.length("split")?.unwrap_or(Length { value: 1.6, unit: units::Unit::Pct, plus: None });
    let shift_l = a.length("slice-shift")?.unwrap_or(Length { value: 6.0, unit: units::Unit::Pct, plus: None });
    let slices = a.usize("slices")?;
    let dark = (a.f64("scanlines")? / 100.0) as f32;
    let gap = match a.opt_f64("line-gap")? {
        Some(g) => g,
        None => {
            let g = (short * 0.004).max(3.0).round();
            ctx.resolve_param("line-gap", g);
            g
        }
    };
    let mut rng = ctx.rng();
    let draws: Vec<[f64; 4]> = (0..slices)
        .map(|_| {
            let u = |r: &mut dyn RngCore| (r.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
            [u(&mut rng), u(&mut rng), u(&mut rng), u(&mut rng)]
        })
        .collect();
    let mut report = Vec::new();
    // the split and the slice shifts carry content sideways (sizes in % are of the layer)
    let lw = {
        let p = target_layer(a, doc)?;
        doc.layer(&p).raster().map(|r| r.width as f64).unwrap_or(doc.width as f64)
    };
    let side = split_l.to_px(lw, dpi) * 3.0 + shift_l.to_px(lw, dpi) + 1.0;
    let reach = Reach { left: side.ceil() as u32, right: side.ceil() as u32, top: 0, bottom: 0 };
    let v = apply_to_raster_grow(a, doc, ctx, reach, |img, (_, oy), _| {
        let (w, h) = (img.width as usize, img.height as usize);
        if w == 0 || h == 0 {
            return Ok(img.clone());
        }
        // sizes and bands follow the visible content (on a smart filter's canvas-sized
        // image too), not the layer's transparent margins
        let (_, by, bw, bh) = bounds(img);
        let split = split_l.to_px(bw.min(lw), dpi);
        let max_shift = shift_l.to_px(bw.min(lw), dpi);
        // bands: (y0, y1, shift, extra split)
        let bands: Vec<(f64, f64, f64, f64)> = draws
            .iter()
            .map(|d| {
                let hh = bh * (0.02 + 0.08 * d[1]);
                let y0 = by + d[0] * (bh - hh).max(0.0);
                // mostly clear shifts: 35..100% of the maximum, either way
                let shift =
                    (if d[2] < 0.5 { -1.0 } else { 1.0 }) * (0.35 + 0.65 * (2.0 * d[2] - 1.0).abs()) * max_shift;
                (y0, y0 + hh, shift, split * (0.5 + 1.5 * d[3]))
            })
            .collect();
        for b in &bands {
            report.push(json!({"y": (b.0 + oy as f64).round(), "height": (b.1 - b.0).round(), "shift": (b.2 * 10.0).round() / 10.0}));
        }
        let buf = Buf::premul(img);
        let mut out = Buf::new(w, h, 4);
        let _ = short;
        out.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
            let yc = y as f64 + 0.5;
            let (mut shift, mut sp) = (0.0, split);
            for b in &bands {
                if yc >= b.0 && yc < b.1 {
                    shift += b.2;
                    sp += b.3;
                }
            }
            // scanlines: a soft dark line every `gap` canvas rows
            let phase = ((y as f64 + oy as f64) / gap).fract();
            let line = 1.0 - dark * 0.9 * (0.5 + 0.5 * (TAU * phase).cos()).powi(2) as f32;
            for x in 0..w {
                let sx = x as f64 - shift;
                // red pulled one way, green and blue (cyan) staying: red and cyan fringes
                let r = util::sample4(&buf, (sx + sp) as f32, y as f32, Edge::Zero);
                let c = util::sample4(&buf, sx as f32, y as f32, Edge::Zero);
                // the alpha spreads with the channels; colors stay premultiplied per channel
                let o = &mut row[x * 4..x * 4 + 4];
                o[0] = r[0] * line;
                o[1] = c[1] * line;
                o[2] = c[2] * line;
                o[3] = r[3].max(c[3]);
            }
        });
        Ok(out.into_unpremul())
    })?;
    let mut v = v;
    if let Value::Object(m) = &mut v {
        m.insert("slices".into(), json!(report));
        m.insert("line_gap".into(), json!(gap));
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing::{self, find, raster, run};

    fn stripes(w: u32, h: u32) -> Image {
        Image::from_fn(w, h, |x, y| if (x / 4 + y / 6) % 2 == 0 { [1.0, 0.2, 0.2, 1.0] } else { [0.1, 0.1, 0.9, 1.0] })
    }

    #[test]
    fn every_kind_moves_pixels_and_keeps_size() {
        let spec = find(&ops(), "filter distort");
        for kind in KINDS {
            let mut d = testing::doc_from(stripes(48, 40));
            let v = run(&spec, &[("kind", kind)], &mut d).unwrap_or_else(|e| panic!("{kind}: {}", e.message));
            assert_eq!(v["kind"], *kind);
            let r = raster(&d, "Background");
            assert_eq!((r.width, r.height), (48, 40));
            assert!(testing::max_diff(r, &stripes(48, 40)) > 0.1, "{kind} changed nothing");
        }
        assert!(run(&spec, &[], &mut testing::doc_from(stripes(8, 8))).is_err(), "--kind is required");
    }

    #[test]
    fn twirl_keeps_the_center_and_the_outside() {
        let spec = find(&ops(), "filter distort");
        let src = stripes(41, 41);
        let mut d = testing::doc_from(src.clone());
        run(&spec, &[("kind", "twirl"), ("angle", "120"), ("radius", "10")], &mut d).unwrap();
        let r = raster(&d, "Background");
        assert_eq!(r.get(0, 0), src.get(0, 0));
        assert_eq!(r.get(40, 20), src.get(40, 20));
        // polar then polar-inverse is close to the identity in the middle band
        let mut d = testing::doc_from(stripes(64, 64));
        run(&spec, &[("kind", "polar")], &mut d).unwrap();
        run(&spec, &[("kind", "polar-inverse")], &mut d).unwrap();
        let r = raster(&d, "Background");
        let ok = (20..44).filter(|&x| (r.get(x, 40)[0] - stripes(64, 64).get(x, 40)[0]).abs() < 0.35).count();
        assert!(ok > 16, "{ok}");
    }

    #[test]
    fn displace_with_noise_is_seeded_and_amount_zero_is_identity() {
        let spec = find(&ops(), "filter displace");
        let mut a = testing::doc_from(stripes(40, 30));
        let v = run(&spec, &[("amount", "4")], &mut a).unwrap();
        assert_eq!(v["map"], "noise");
        let mut b = testing::doc_from(stripes(40, 30));
        run(&spec, &[("amount", "4")], &mut b).unwrap();
        assert_eq!(raster(&a, "Background"), raster(&b, "Background"));
        assert!(testing::max_diff(raster(&a, "Background"), &stripes(40, 30)) > 0.1);
        let mut c = testing::doc_from(stripes(40, 30));
        run(&spec, &[("amount", "0"), ("mode", "slope")], &mut c).unwrap();
        assert!(testing::max_diff(raster(&c, "Background"), &stripes(40, 30)) < 1e-5);
    }

    #[test]
    fn displace_by_a_layer_map() {
        let spec = find(&ops(), "filter displace");
        let mut d = testing::doc_layers(
            32,
            24,
            vec![("Photo", stripes(32, 24)), ("Map", Image::filled(32, 24, Rgba::new(1.0, 0.5, 0.5, 1.0)))],
        );
        run(&spec, &[("layer", "Photo"), ("map-layer", "Map"), ("amount", "4")], &mut d).unwrap();
        // red = 1: every pixel samples 4 px to the right
        let r = raster(&d, "Photo");
        assert_eq!(r.get(5, 5), stripes(32, 24).get(9, 5));
        assert_eq!(
            run(&spec, &[("layer", "Photo"), ("map", "a.png"), ("map-layer", "Map")], &mut d).unwrap_err().code,
            "PX1005"
        );
    }

    #[test]
    fn glitch_splits_channels_and_spreads_alpha() {
        let spec = find(&ops(), "filter glitch");
        // a white bar on transparency gains red and blue fringes
        let img = Image::from_fn(60, 20, |x, _| if (20..40).contains(&x) { [1.0; 4] } else { [0.0; 4] });
        let mut d = testing::doc_from(img);
        run(&spec, &[("split", "4"), ("slices", "0"), ("scanlines", "0")], &mut d).unwrap();
        let r = raster(&d, "Background");
        // red is sampled 4 px to the right: a red fringe on the left, cyan on the right
        let left = r.get(17, 10);
        let right = r.get(38, 10);
        assert!(left[3] > 0.9 && left[0] > 0.9 && left[2] < 0.1, "{left:?}");
        assert!(right[3] > 0.9 && right[2] > 0.9 && right[1] > 0.9 && right[0] < 0.1, "{right:?}");
        // seeded slices
        let mut a = testing::doc_from(stripes(60, 60));
        let va = run(&spec, &[("slices", "5")], &mut a).unwrap();
        let mut b = testing::doc_from(stripes(60, 60));
        let vb = run(&spec, &[("slices", "5")], &mut b).unwrap();
        assert_eq!(va["slices"], vb["slices"]);
        assert_eq!(raster(&a, "Background"), raster(&b, "Background"));
    }
}
