//! `fill noise`: seeded procedural textures — clouds (Photoshop's Render Clouds, and
//! Difference Clouds through `--blend difference`), smoke, turbulence, ridged noise,
//! and Worley cells — painted onto a layer or onto a new layer.
//!
//! The noise is evaluated in canvas coordinates (two layers filled with the same seed
//! line up), normalized so contrast and bias mean the same for every kind
//! (`pixelox_core::noise`), shaped, and mapped through `--stops` with premultiplied
//! interpolation (a `transparent` end fades without a dark fringe).

use crate::smoke::Smoke;
use pixelox_core::blend::{composite_pixel, BLEND_MODES};
use pixelox_core::color::Stop;
use pixelox_core::noise::{self, Basis, Field, Pattern, SPREAD};
use pixelox_core::prelude::*;
use pixelox_core::units::Length;
use rand_core::RngCore;
use rayon::prelude::*;

pub const KINDS: &[&str] = &["clouds", "smoke", "turbulence", "ridged", "cells", "cell-edges"];

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("fill noise", Category::Retouch, "Clouds, smoke, fractal or cell noise", fill_noise)
        .about(
            "Paints seeded procedural noise through --stops (low → high) onto the target raster layer, or onto a new \
             canvas-sized layer with --as-layer. Kinds: clouds (fractal Brownian motion: Photoshop's Render Clouds), \
             smoke (thin curling wisps over soft haze), turbulence (billows with creases), ridged (sharp veins and \
             ridges: marble, lightning), cells (Worley: round cells), cell-edges (the network between cells: cracks, \
             crackle, scales). --scale is the size of the largest features; --octaves adds finer detail, each octave \
             --lacunarity times finer and --persistence times weaker. --warp swirls the pattern (domain warp), \
             --stretch elongates it along --angle (90 = rising). --contrast steepens and --bias shifts the ramp toward \
             the high color. --tile makes the texture repeat seamlessly across the canvas (the scale is rounded to a \
             whole number of cells). --blend composites onto the existing pixels (difference = Photoshop's \
             Difference Clouds); with --as-layer it becomes the new layer's blend mode and --opacity its opacity. \
             The noise is anchored to the canvas, so the same --seed lines up across layers.",
        )
        .param(param("kind", ParamKind::Choice).choices(KINDS).default("clouds").help("Pattern: clouds, smoke, turbulence, ridged, cells, or cell-edges"))
        .param(param("stops", ParamKind::Stops).default("#000000,#ffffff").help("Colors from low to high noise: two colors ('#000,#fff', 'transparent,#fff') or stops with positions"))
        .param(param("scale", ParamKind::Length).auto().default("auto").help("Size of the largest features in px, or % of the canvas's shorter side; auto = 70% for smoke, else 40%"))
        .param(param("octaves", ParamKind::Int).auto().default("auto").range("1..12").help("Detail layers summed (1 = smooth blobs, 8 = fine detail); auto = 1 for cells, 2 for cell-edges, 5 for smoke, else 6"))
        .param(param("persistence", ParamKind::Float).default("0.5").range("0..1").help("Strength of each finer octave relative to the last (higher = rougher)"))
        .param(param("lacunarity", ParamKind::Float).default("2").range("1..4").help("How much finer each octave is (a whole number with --tile)"))
        .param(param("basis", ParamKind::Choice).choices(&["perlin", "simplex", "value"]).default("perlin").help("Noise function for clouds, smoke, turbulence, ridged: perlin, simplex, or value"))
        .param(param("contrast", ParamKind::Float).default("0").range("-100..100").help("Steepen (+) or flatten (−) the ramp; 100 gives hard-edged shapes"))
        .param(param("bias", ParamKind::Float).default("0").range("-100..100").help("Shift toward the high color (+) or the low color (−): more or less coverage"))
        .param(param("warp", ParamKind::Float).auto().default("auto").range("0..300").help("Swirl (domain warp) in % of a feature; auto = 90 for smoke, else 0"))
        .param(param("stretch", ParamKind::Float).auto().default("auto").range("0.1..10").help("Elongate features along --angle by this factor; auto = 1.8 for smoke, else 1"))
        .param(param("angle", ParamKind::Angle).default("90").help("Direction of --stretch in degrees (0 = horizontal, 90 = vertical)"))
        .param(param("tile", ParamKind::Flag).help("Repeat seamlessly across the canvas edges (for pattern tiles)"))
        .param(param("blend", ParamKind::Choice).choices(&BLEND_MODES[..BLEND_MODES.len() - 1]).default("normal").help("How the noise combines with the pixels (difference = Difference Clouds); with --as-layer, the new layer's mode"))
        .param(param("opacity", ParamKind::Float).default("100").range("0..100").help("Opacity of the noise in percent (with --as-layer, the layer's opacity)"))
        .param(param("as-layer", ParamKind::Flag).help("Paint onto a new canvas-sized layer above --layer (or the active layer) instead"))
        .param(common::name())
        .params(common::region())
        .example("pixelox fill noise doc.pxo --in-place --layer Sky --stops '#1b2a4a,#f2f4f8'", "Render Clouds into a layer")
        .example("pixelox fill noise doc.pxo --in-place --as-layer --name Smoke --kind smoke --stops 'transparent,#c9d2e0' --blend screen --opacity 70", "Rising smoke on its own screen layer")
        .example("pixelox fill noise doc.pxo --in-place --layer Paper --scale 25% --stops '#8a7a62,#f6eedc' --blend multiply --opacity 60", "Mottled paper")
        .example("pixelox fill noise doc.pxo --in-place --kind clouds --blend difference", "Difference Clouds")
        .example("pixelox fill noise tile.png -o marble.png --kind ridged --warp 120 --scale 60% --stops '#2b2b30,#f4f1ea' --tile", "A seamless marble tile")
        .example("pixelox fill noise doc.pxo --in-place --kind cell-edges --scale 8% --octaves 2 --stops '#ffffff,#222222' --as-layer --blend multiply", "Crackle texture")]
}

/// A premultiplied-interpolation lookup table of the stops (straight RGBA out).
pub(crate) fn stops_lut(stops: &[Stop], n: usize) -> Vec<[f32; 4]> {
    (0..n)
        .map(|i| {
            let t = i as f32 / (n - 1) as f32;
            if t <= stops[0].pos {
                return stops[0].color.to_array();
            }
            for w in stops.windows(2) {
                if t <= w[1].pos {
                    let span = (w[1].pos - w[0].pos).max(1e-6);
                    let k = ((t - w[0].pos) / span).clamp(0.0, 1.0);
                    let (a, b) = (w[0].color.to_array(), w[1].color.to_array());
                    let al = a[3] + (b[3] - a[3]) * k;
                    if al <= 1e-6 {
                        return [0.0; 4];
                    }
                    let mut o = [0f32; 4];
                    for c in 0..3 {
                        o[c] = ((a[c] * a[3] + (b[c] * b[3] - a[c] * a[3]) * k) / al).clamp(0.0, 1.0);
                    }
                    o[3] = al;
                    return o;
                }
            }
            stops[stops.len() - 1].color.to_array()
        })
        .collect()
}

#[inline]
fn lut_at(lut: &[[f32; 4]], t: f64) -> [f32; 4] {
    let f = t.clamp(0.0, 1.0) as f32 * (lut.len() - 1) as f32;
    let i = (f as usize).min(lut.len() - 2);
    let k = f - i as f32;
    let (a, b) = (lut[i], lut[i + 1]);
    [a[0] + (b[0] - a[0]) * k, a[1] + (b[1] - a[1]) * k, a[2] + (b[2] - a[2]) * k, a[3] + (b[3] - a[3]) * k]
}

/// A domain warp: two fields displacing the coordinates by `amt` cells per unit.
#[derive(Clone, Copy)]
pub(crate) struct Warp {
    fx: Field,
    fy: Field,
    amt: f64,
}

impl Warp {
    #[inline]
    fn offset(&self, u: f64, v: f64) -> (f64, f64) {
        let qx = (self.fx.at(u + 5.2, v + 1.3) - 0.5) / SPREAD;
        let qy = (self.fy.at(u + 1.7, v + 9.2) - 0.5) / SPREAD;
        (self.amt * qx, self.amt * qy)
    }
}

/// A noise texture over canvas pixels.
pub(crate) struct Texture {
    pub field: Field,
    pub warp: Option<Warp>,
    /// The smoke kind draws its own wisps (`crate::smoke`).
    pub smoke: Option<Smoke>,
    /// Pixels per cell along the stretch direction and across it.
    pub cell: (f64, f64),
    /// Unit vector of the stretch direction (screen coordinates), when rotated.
    pub dir: (f64, f64),
    pub axis_aligned: bool,
    pub contrast: f64,
    pub bias: f64,
}

impl Texture {
    /// Noise coordinates (cells) of a canvas point.
    #[inline]
    fn coords(&self, x: f64, y: f64) -> (f64, f64) {
        if self.axis_aligned {
            (x / self.cell.0, y / self.cell.1)
        } else {
            let a = x * self.dir.0 + y * self.dir.1;
            let b = -x * self.dir.1 + y * self.dir.0;
            (a / self.cell.0, b / self.cell.1)
        }
    }

    /// The unshaped value at canvas point (x, y) (about 0..1).
    fn raw(&self, x: f64, y: f64) -> f64 {
        let (mut u, mut v) = self.coords(x, y);
        if let Some(w) = &self.warp {
            let (du, dv) = w.offset(u, v);
            u += du;
            v += dv;
        }
        self.field.at(u, v)
    }

    /// The shaped values (0..1) of the `w`×`h` canvas pixels whose top-left is (x0, y0).
    pub fn plane(&self, x0: i64, y0: i64, w: usize, h: usize) -> Vec<f32> {
        let raw = match &self.smoke {
            Some(sm) => sm.plane(x0, y0, w, h),
            None => {
                let mut raw = vec![0f32; w * h];
                raw.par_chunks_mut(w.max(1)).enumerate().for_each(|(j, row)| {
                    let cy = (y0 + j as i64) as f64 + 0.5;
                    for (i, v) in row.iter_mut().enumerate() {
                        *v = self.raw((x0 + i as i64) as f64 + 0.5, cy) as f32;
                    }
                });
                raw
            }
        };
        raw.par_iter().map(|&t| noise::shape_value(t as f64, self.contrast, self.bias) as f32).collect()
    }
}

/// Settings shared by `fill noise` and the noise map of `filter displace`.
pub(crate) struct NoiseSettings {
    pub kind: String,
    pub basis: Basis,
    pub scale: f64,
    pub octaves: u32,
    pub persistence: f64,
    pub lacunarity: f64,
    pub warp: f64,
    pub stretch: f64,
    pub angle: f64,
    pub tile: bool,
    pub contrast: f64,
    pub bias: f64,
}

/// Build the texture; `tile` is the canvas size for seamless output. Adjusted settings
/// are reported through `ctx` (PXW3010).
pub(crate) fn texture(mut s: NoiseSettings, seed: u64, canvas: (u32, u32), ctx: &mut OpCtx) -> Texture {
    let pattern = match s.kind.as_str() {
        "turbulence" => Pattern::Turbulence,
        "ridged" => Pattern::Ridged,
        "cells" => Pattern::Cells,
        "cell-edges" => Pattern::CellEdges,
        _ => Pattern::Fbm,
    };
    let mut notes = Vec::new();
    let (mut cell_a, mut cell_b) = (s.scale * s.stretch, s.scale);
    let mut axis_aligned = false;
    let th = s.angle.to_radians();
    let dir = (th.cos(), -th.sin());
    let mut period = None;
    if s.tile {
        if s.basis == Basis::Simplex {
            s.basis = Basis::Perlin;
            notes.push("simplex noise cannot tile: used perlin".to_string());
        }
        let lac = s.lacunarity.round().max(1.0);
        if lac != s.lacunarity {
            notes.push(format!("lacunarity {} rounded to {lac} to tile", s.lacunarity));
            s.lacunarity = lac;
        }
        let quarter = (s.angle / 90.0).round();
        if (s.angle - quarter * 90.0).abs() > 1e-6 && (s.stretch - 1.0).abs() > 1e-9 {
            notes.push(format!("--angle {} snapped to {} to tile", s.angle, quarter * 90.0));
        }
        // stretch along x (0°, 180°) or y (90°, 270°); cells fit the canvas exactly
        let vertical = (quarter as i64).rem_euclid(2) == 1;
        let (sx, sy) = if vertical { (s.scale, s.scale * s.stretch) } else { (s.scale * s.stretch, s.scale) };
        let (w, h) = (canvas.0 as f64, canvas.1 as f64);
        let (nx, ny) = ((w / sx).round().max(1.0), (h / sy).round().max(1.0));
        cell_a = w / nx;
        cell_b = h / ny;
        axis_aligned = true;
        period = Some((nx as i64, ny as i64));
    } else if (s.stretch - 1.0).abs() < 1e-9 {
        axis_aligned = true;
        cell_a = s.scale;
        cell_b = s.scale;
    }
    if !notes.is_empty() {
        ctx.warn_ctx("PXW3010", format!("--tile adjusted the noise: {}", notes.join("; ")), json!({"notes": notes}));
    }
    let field = Field {
        pattern,
        basis: s.basis,
        octaves: s.octaves,
        persistence: s.persistence,
        lacunarity: s.lacunarity,
        seed: noise::lane(seed, 1),
        period,
    };
    let smoke = (s.kind == "smoke").then(|| {
        Smoke::new(
            seed,
            s.scale,
            s.angle,
            s.stretch,
            s.warp / 90.0,
            s.octaves,
            s.persistence,
            s.basis,
            s.tile.then_some(canvas),
        )
    });
    let wf = Field { pattern: Pattern::Fbm, basis: s.basis, octaves: s.octaves.min(4), ..field };
    let make = |k: u64, amt: f64| Warp {
        fx: Field { seed: noise::lane(seed, k), ..wf },
        fy: Field { seed: noise::lane(seed, k + 1), ..wf },
        amt,
    };
    let warp = (s.warp > 0.0 && smoke.is_none()).then(|| make(2, s.warp / 100.0 * 0.35));
    Texture { field, warp, smoke, cell: (cell_a, cell_b), dir, axis_aligned, contrast: s.contrast, bias: s.bias }
}

/// Read the noise settings shared with `filter displace` (resolving `auto` values).
pub(crate) fn settings(a: &Args, doc: &Document, kind: &str, ctx: &mut OpCtx) -> Result<NoiseSettings> {
    let short = doc.width.min(doc.height).max(1) as f64;
    let scale = match a.length("scale")? {
        Some(l) => l.to_px(short, doc.dpi()),
        None => {
            let pct = if kind == "smoke" { 70.0 } else { 40.0 };
            ctx.resolve_param("scale", format!("{pct}%"));
            Length { value: pct, unit: units::Unit::Pct, plus: None }.to_px(short, doc.dpi())
        }
    };
    if scale.is_nan() || scale <= 0.0 {
        return Err(Error::new("PX1013", format!("--scale {scale} px must be positive"))
            .with_hint("give a size such as 120 or 25%")
            .with_context("scale_px", scale));
    }
    // features smaller than a pixel would alias into white noise
    let scale = scale.max(1.0);
    let smoke = kind == "smoke";
    let warp = match a.opt_f64("warp")? {
        Some(v) => v,
        None => {
            let v = if smoke { 90.0 } else { 0.0 };
            ctx.resolve_param("warp", v);
            v
        }
    };
    let stretch = match a.opt_f64("stretch")? {
        Some(v) => v,
        None => {
            let v = if smoke { 1.8 } else { 1.0 };
            ctx.resolve_param("stretch", v);
            v
        }
    };
    Ok(NoiseSettings {
        kind: kind.to_string(),
        basis: Basis::parse(&a.str_or("basis", "perlin")).unwrap_or(Basis::Perlin),
        scale,
        octaves: match a.opt_int("octaves")? {
            Some(n) => n.clamp(1, 12) as u32,
            None => {
                let n = match kind {
                    "cells" => 1,
                    "cell-edges" => 2,
                    "smoke" => 5,
                    _ => 6,
                };
                ctx.resolve_param("octaves", n);
                n
            }
        },
        persistence: a.f64("persistence")?,
        lacunarity: a.f64("lacunarity")?,
        warp,
        stretch,
        angle: a.angle("angle")?,
        tile: a.flag("tile"),
        contrast: a.f64("contrast")?,
        bias: a.f64("bias")?,
    })
}

/// A new, empty, canvas-sized raster layer above `--layer` (or the active layer, or on
/// top), named `--name` or `default`. Returns its path.
pub(crate) fn new_layer(a: &Args, doc: &mut Document, default: &str) -> Result<pixelox_core::doc::LayerPath> {
    let above = match a.raw("layer") {
        Some(r) => Some(doc.resolve(r)?),
        None => doc.active.clone().and_then(|id| doc.find_by_id(&id)),
    };
    let name = match a.str("name").map(|n| n.trim().to_string()).filter(|n| !n.is_empty()) {
        Some(n) => {
            doc.check_name_free(&n, None)?;
            n
        }
        None => default.to_string(),
    };
    let layer = doc.new_layer(&name, LayerKind::Raster { image: Image::new(doc.width.max(1), doc.height.max(1)) });
    Ok(doc.insert_above(above.as_deref(), layer))
}

fn fill_noise(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let kind = a.choice("kind");
    let settings = settings(a, doc, &kind, ctx)?;
    let stops = a.stops("stops")?;
    let mode = match BlendMode::parse(&a.choice("blend"))? {
        BlendMode::PassThrough => BlendMode::Normal,
        m => m,
    };
    let opacity = (a.f64("opacity")? / 100.0) as f32;
    let as_layer = a.flag("as-layer");
    if !as_layer && a.has("name") {
        return Err(Error::new("PX1005", "--name names the layer made by --as-layer")
            .with_hint("add --as-layer to paint onto a new layer, or drop --name"));
    }
    let seed = ctx.rng().next_u64();
    let (scale, tile) = (settings.scale, settings.tile);
    let tex = texture(settings, seed, (doc.width, doc.height), ctx);
    let lut = stops_lut(&stops, 1024);
    let mask = ctx.mask_arg(a, doc)?;
    let path = if as_layer {
        if let Some(r) = a.raw("layer") {
            doc.resolve(r)?;
        }
        new_layer(a, doc, "Noise")?
    } else {
        target_layer(a, doc)?
    };
    let layer = doc.layer(&path);
    if !layer.visible {
        ctx.warn("PXW2001", format!("layer '{}' is hidden", layer.name));
    }
    let (lx, ly, name) = (layer.x as i64, layer.y as i64, layer.name.clone());
    // on a new layer the noise is painted plainly; the layer carries the mode and opacity
    let (paint_mode, paint_opacity) = if as_layer { (BlendMode::Normal, 1.0) } else { (mode, opacity) };
    let img = doc.layer_mut(&path).raster_mut()?;
    let w = img.width as usize;
    if w > 0 && img.height > 0 {
        let plane = tex.plane(lx, ly, w, img.height as usize);
        img.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
            let cy = y as i64 + ly;
            for x in 0..w {
                let cx = x as i64 + lx;
                let k = mask.as_ref().map_or(1.0, |m| m.at(cx, cy)) * paint_opacity;
                if k <= 0.0 {
                    continue;
                }
                let c = lut_at(&lut, plane[y * w + x] as f64);
                let src = [c[0], c[1], c[2], (c[3] * k).min(1.0)];
                if src[3] <= 0.0 {
                    continue;
                }
                composite_pixel(paint_mode, &mut row[x * 4..x * 4 + 4], src);
            }
        });
    }
    if as_layer {
        let l = doc.layer_mut(&path);
        l.blend = mode;
        l.opacity = opacity * 100.0;
    }
    let mut v = json!({
        "layer": name,
        "kind": kind,
        "scale_px": (scale * 100.0).round() / 100.0,
        "blend": mode.name(),
        "as_layer": as_layer,
    });
    if tile {
        v["tile_cells"] = json!(tex.field.period.map(|(x, y)| [x, y]));
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing::{self, find, raster, run};

    #[test]
    fn clouds_span_the_ramp_and_are_seeded() {
        let spec = find(&ops(), "fill noise");
        let mut d1 = testing::doc_from(Image::new(96, 64));
        let v = run(&spec, &[], &mut d1).unwrap();
        assert_eq!(v["kind"], "clouds");
        let r = raster(&d1, "Background");
        let vals: Vec<f32> = r.data.chunks(4).map(|p| p[0]).collect();
        let (lo, hi) = vals.iter().fold((1f32, 0f32), |(l, h), &v| (l.min(v), h.max(v)));
        assert!(lo < 0.35 && hi > 0.65, "{lo}..{hi}");
        assert!(r.data.chunks(4).all(|p| p[3] == 1.0 && p[0] == p[1]));
        let mut d2 = testing::doc_from(Image::new(96, 64));
        run(&spec, &[], &mut d2).unwrap();
        assert_eq!(raster(&d1, "Background"), raster(&d2, "Background"));
    }

    #[test]
    fn every_kind_renders_and_tiles() {
        let spec = find(&ops(), "fill noise");
        for kind in KINDS {
            let mut d = testing::doc_from(Image::new(64, 48));
            run(&spec, &[("kind", kind), ("tile", "true"), ("scale", "16")], &mut d).unwrap();
            let r = raster(&d, "Background");
            // seamless: the left and right columns continue each other like inner neighbors
            let edge: f32 = (0..48).map(|y| (r.get(0, y)[0] - r.get(63, y)[0]).abs()).sum::<f32>() / 48.0;
            let inner: f32 = (0..48).map(|y| (r.get(30, y)[0] - r.get(31, y)[0]).abs()).sum::<f32>() / 48.0;
            assert!(edge < inner * 2.5 + 0.02, "{kind}: edge step {edge} vs inner {inner}");
        }
    }

    #[test]
    fn transparent_stops_fade_without_fringe_and_as_layer() {
        let spec = find(&ops(), "fill noise");
        let mut d = testing::doc_from(Image::filled(40, 30, Rgba::new(0.1, 0.1, 0.1, 1.0)));
        let v = run(
            &spec,
            &[
                ("stops", "transparent,#ffffff"),
                ("as-layer", "true"),
                ("name", "Smoke"),
                ("blend", "screen"),
                ("opacity", "50"),
            ],
            &mut d,
        )
        .unwrap();
        assert_eq!(v["layer"], "Smoke");
        let l = d.layer(&d.resolve("Smoke").unwrap());
        assert_eq!((l.blend, l.opacity), (BlendMode::Screen, 50.0));
        let r = raster(&d, "Smoke");
        // every visible pixel is white: the fade is in alpha only
        assert!(r.data.chunks(4).filter(|p| p[3] > 0.01).all(|p| p[0] > 0.999));
        assert!(raster(&d, "Background").get(3, 3)[0] < 0.11, "the original layer is untouched");
        assert_eq!(run(&spec, &[("name", "X")], &mut d).unwrap_err().code, "PX1005");
    }

    #[test]
    fn difference_clouds_and_mask() {
        let spec = find(&ops(), "fill noise");
        let mut d = testing::doc_from(Image::filled(24, 16, Rgba::new(1.0, 1.0, 1.0, 1.0)));
        d.put_mask("left", testing::half_mask(24, 16), false).unwrap();
        run(&spec, &[("blend", "difference"), ("mask", "left")], &mut d).unwrap();
        let r = raster(&d, "Background");
        assert_eq!(r.get(20, 5), [1.0, 1.0, 1.0, 1.0]);
        assert!(r.get(3, 5)[0] < 1.0);
    }
}
