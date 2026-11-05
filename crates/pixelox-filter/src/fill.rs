//! Fills: `fill solid`, `fill gradient`, `fill pattern` (`fill content-aware` lives in
//! pixelox-ml).

use crate::util::{self, Buf};
use pixelox_core::color::{self, Interp};
use pixelox_core::kernels;
use pixelox_core::prelude::*;
use rand_core::RngCore;
use rayon::prelude::*;

const CAT: Category = Category::Retouch;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("fill solid", CAT, "Fill a layer or region with a color", solid)
            .about("Paints --color over the target raster layer (normal blending, so a translucent color tints), limited by --mask. --preserve-alpha keeps the layer's transparency and only recolors visible pixels. Fills cover the layer's own pixel area.")
            .param(param("color", ParamKind::Color).default("#000000").help("Fill color (alpha < 1 blends over the existing pixels)"))
            .params(common::region())
            .param(param("preserve-alpha", ParamKind::Flag).help("Keep the layer's alpha; only recolor existing pixels"))
            .example("pixelox fill solid doc.pxo --layer Backdrop --color '#0ea5e9' --in-place", "Fill a layer with blue")
            .example("pixelox fill solid doc.pxo --color white --mask @sel --in-place", "Fill the selection with white")
            .example("pixelox fill solid logo.png -o red-logo.png --color '#e11d48' --preserve-alpha", "Recolor a logo keeping its shape"),
        OpSpec::doc("fill gradient", CAT, "Fill with a linear, radial, angular, or reflected gradient", gradient)
            .about("Paints a gradient from --from to --to (canvas coordinates) with --stops, interpolated in --interp space: perceptual (sRGB values, Photoshop's default), linear light, or OKLab. linear runs along the line, radial grows from --from with radius |to − from|, angular sweeps around --from starting at --to, reflected mirrors the linear ramp about --from. --dither adds seeded ±1-level triangular noise to hide banding.")
            .param(param("kind", ParamKind::Choice).choices(&["linear", "radial", "angular", "reflected"]).default("linear").help("Geometry: linear, radial, angular, or reflected"))
            .param(param("from", ParamKind::Point).default("0,50%").help("Start point X,Y in canvas coordinates (center for radial/angular)"))
            .param(param("to", ParamKind::Point).default("100%,50%").help("End point X,Y in canvas coordinates"))
            .param(param("stops", ParamKind::Stops).default("#000@0,#fff@1").help("Color stops 'color@pos,...' with positions 0..1 or %"))
            .param(param("interp", ParamKind::Choice).choices(&["perceptual", "linear", "oklab"]).default("perceptual").help("Interpolation space: perceptual (sRGB), linear light, or oklab"))
            .param(param("dither", ParamKind::Flag).help("Add seeded dither noise to prevent banding"))
            .params(common::region())
            .example("pixelox fill gradient doc.pxo --layer Sky --from 0,0 --to 0,100% --stops '#0b1d51@0,#f97316@1' --in-place", "Vertical sunset sky")
            .example("pixelox fill gradient doc.pxo --kind radial --from 50%,50% --to 100%,100% --stops '#fff@0,#000@1' --dither --in-place", "Radial spotlight without banding")
            .example("pixelox fill gradient doc.pxo --kind angular --from 50%,50% --to 100%,50% --stops 'red@0,yellow@0.33,blue@0.66,red@1' --interp oklab --in-place", "Color wheel"),
        OpSpec::doc("fill pattern", CAT, "Fill with a tiled image pattern", pattern)
            .about("Tiles the image --from across the target layer, aligned to the canvas origin, scaled by --scale % and rotated by --angle degrees (counter-clockwise), with wrap-around bilinear sampling. Limited by --mask; the tile's transparency blends over the layer.")
            .param(param("from", ParamKind::Path).required().help("Tile image to repeat"))
            .param(param("scale", ParamKind::Float).default("100").range("1..10000").help("Tile scale in %"))
            .param(param("angle", ParamKind::Angle).default("0").help("Tile rotation in degrees (counter-clockwise)"))
            .params(common::region())
            .example("pixelox fill pattern doc.pxo --layer Paper --from texture.png --in-place", "Tile a texture across a layer")
            .example("pixelox fill pattern doc.pxo --from dots.png --scale 50 --angle 45 --in-place", "Small diagonal dots")
            .example("pixelox fill pattern doc.pxo --from brick.png --mask wall --in-place", "Fill a masked wall with bricks"),
    ]
}

/// Paint rows produced by `src` over the target raster layer, limited by `--mask`.
/// `src(canvas_y, canvas_x0, row)` writes straight RGBA for one layer row.
fn paint(
    a: &Args,
    doc: &mut Document,
    ctx: &mut OpCtx,
    preserve_alpha: bool,
    src: &(dyn Fn(i64, i64, &mut [f32]) + Sync),
) -> Result<Value> {
    let path = target_layer(a, doc)?;
    let mask = ctx.mask_arg(a, doc)?;
    let layer = doc.layer(&path);
    if !layer.visible {
        ctx.warn("PXW2001", format!("layer '{}' is hidden", layer.name));
    }
    let (lx, ly, name) = (layer.x as i64, layer.y as i64, layer.name.clone());
    let img = doc.layer_mut(&path).raster_mut()?;
    let w = img.width as usize;
    if w == 0 || img.height == 0 {
        return Ok(json!({"layer": name, "pixels_changed": 0}));
    }
    let counts: Vec<usize> = img
        .data
        .par_chunks_mut(w * 4)
        .enumerate()
        .map(|(y, row)| {
            let cy = y as i64 + ly;
            let mut colors = vec![0f32; w * 4];
            src(cy, lx, &mut colors);
            let mut n = 0;
            for (x, (d, s)) in row.chunks_mut(4).zip(colors.chunks(4)).enumerate() {
                let k = mask.as_ref().map_or(1.0, |m| m.at(x as i64 + lx, cy));
                let a_s = (s[3] * k).clamp(0.0, 1.0);
                if a_s <= 0.0 {
                    continue;
                }
                if preserve_alpha {
                    if d[3] <= 0.0 {
                        continue;
                    }
                    for c in 0..3 {
                        d[c] += (s[c] - d[c]) * a_s;
                    }
                } else {
                    let a_d = d[3];
                    let a_o = a_s + a_d * (1.0 - a_s);
                    for c in 0..3 {
                        d[c] = ((s[c] * a_s + d[c] * a_d * (1.0 - a_s)) / a_o).clamp(0.0, 1.0);
                    }
                    d[3] = a_o;
                }
                n += 1;
            }
            n
        })
        .collect();
    Ok(json!({"layer": name, "pixels_changed": counts.iter().sum::<usize>()}))
}

fn solid(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let c = a.color("color")?;
    let px = c.to_array();
    let v = paint(a, doc, ctx, a.flag("preserve-alpha"), &|_, _, row: &mut [f32]| {
        for p in row.chunks_mut(4) {
            p.copy_from_slice(&px);
        }
    })?;
    Ok(util::merge(v, json!({"color": c.to_hex()})))
}

// ---------------------------------------------------------------------------------------
// gradient

#[inline]
fn splitmix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn gradient(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let kind = a.choice("kind");
    let from = util::canvas_point(a, "from", doc)?.unwrap_or((0.0, doc.height as f64 / 2.0));
    let to = util::canvas_point(a, "to", doc)?.unwrap_or((doc.width as f64, doc.height as f64 / 2.0));
    let stops = a.stops("stops")?;
    let interp = Interp::parse(&a.choice("interp"))?;
    let (vx, vy) = (to.0 - from.0, to.1 - from.1);
    let len2 = vx * vx + vy * vy;
    if len2 < 1e-12 {
        return Err(Error::new("PX1010", "--from and --to are the same point; a gradient needs a direction")
            .with_context("from", [from.0, from.1])
            .with_context("to", [to.0, to.1]));
    }
    // color lookup table along t
    const N: usize = 4096;
    let lut: Vec<[f32; 4]> =
        (0..N).map(|i| color::sample_stops(&stops, i as f32 / (N - 1) as f32, interp).to_array()).collect();
    let key = if a.flag("dither") { Some(ctx.rng().next_u64()) } else { None };
    let len = len2.sqrt();
    let base_angle = vy.atan2(vx);
    let t_of = |px: f64, py: f64| -> f64 {
        let (dx, dy) = (px - from.0, py - from.1);
        match kind.as_str() {
            "radial" => (dx * dx + dy * dy).sqrt() / len,
            "angular" => ((dy.atan2(dx) - base_angle) / std::f64::consts::TAU).rem_euclid(1.0),
            "reflected" => ((dx * vx + dy * vy) / len2).abs(),
            _ => (dx * vx + dy * vy) / len2,
        }
    };
    let v = paint(a, doc, ctx, false, &|cy, cx0, row: &mut [f32]| {
        let py = cy as f64 + 0.5;
        for (x, p) in row.chunks_mut(4).enumerate() {
            let cx = cx0 + x as i64;
            let t = t_of(cx as f64 + 0.5, py).clamp(0.0, 1.0) as f32 * (N - 1) as f32;
            let i = (t as usize).min(N - 2);
            let fr = t - i as f32;
            for c in 0..4 {
                p[c] = lut[i][c] + (lut[i + 1][c] - lut[i][c]) * fr;
            }
            if let Some(k) = key {
                let h1 = splitmix(k ^ ((cx as u64) << 32 | (cy as u64 & 0xffff_ffff)));
                let h2 = splitmix(h1);
                for c in 0..3 {
                    let u = ((h1 >> (16 * c)) & 0xffff) as f32 + ((h2 >> (16 * c)) & 0xffff) as f32;
                    let tpdf = u / 65536.0 - 1.0;
                    p[c] = (p[c] + tpdf / 255.0).clamp(0.0, 1.0);
                }
            }
        }
    })?;
    Ok(util::merge(v, json!({"kind": kind, "from": [from.0, from.1], "to": [to.0, to.1]})))
}

// ---------------------------------------------------------------------------------------
// pattern

fn pattern(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let path = a.path("from").ok_or_else(|| Error::new("PX1003", "fill pattern needs --from tile.png"))?;
    let scale = a.f64("scale")? / 100.0;
    let angle = a.angle("angle")?.to_radians();
    // resolve the layer first so a bad --layer fails before reading the file
    target_layer(a, doc)?;
    let (tile, _) = ctx.decode_aux(&path, "pattern", Some("from"))?;
    if tile.is_empty() {
        return Err(Error::new("PX3001", format!("pattern tile {} has no pixels", path.display())));
    }
    let (tw, th) =
        (((tile.width as f64 * scale).round() as u32).max(1), ((tile.height as f64 * scale).round() as u32).max(1));
    let tile = if (tw, th) != (tile.width, tile.height) {
        kernels::resize(&tile, tw, th, kernels::Filter::auto(tile.width.max(tile.height), tw.max(th)))
    } else {
        tile
    };
    let tb = Buf::premul(&tile);
    let (s, c) = angle.sin_cos();
    let v = paint(a, doc, ctx, false, &|cy, cx0, row: &mut [f32]| {
        let py = cy as f64 + 0.5;
        let mut px4 = [0f32; 4];
        for (x, p) in row.chunks_mut(4).enumerate() {
            let px = (cx0 + x as i64) as f64 + 0.5;
            // the inverse rotation of a counter-clockwise (on screen) turn
            let (u, v) = (px * c - py * s, px * s + py * c);
            util::sample_wrap(&tb, (u - 0.5) as f32, (v - 0.5) as f32, &mut px4);
            if px4[3] > 1e-6 {
                for ch in 0..3 {
                    p[ch] = (px4[ch] / px4[3]).clamp(0.0, 1.0);
                }
                p[3] = px4[3].min(1.0);
            } else {
                p.fill(0.0);
            }
        }
    })?;
    Ok(util::merge(v, json!({"tile": [tw, th]})))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_io;
    use pixelox_core::testing::{self, find, raster, run};

    #[test]
    fn solid_fill_mask_preserve_alpha() {
        let spec = find(&ops(), "fill solid");
        let mut doc = testing::doc_from(Image::new(10, 10));
        let v = run(&spec, &[("color", "#ff0000")], &mut doc).unwrap();
        assert_eq!(v["pixels_changed"], 100);
        assert_eq!(raster(&doc, "Background").get(3, 3), [1.0, 0.0, 0.0, 1.0]);
        // mask limits; soft edge composites premultiplied (no dark fringe)
        let mut doc = testing::doc_from(Image::new(10, 10));
        doc.put_mask(
            "m",
            Mask::from_fn(10, 10, |x, _| {
                if x < 5 {
                    1.0
                } else if x == 5 {
                    0.5
                } else {
                    0.0
                }
            }),
            false,
        )
        .unwrap();
        run(&spec, &[("color", "#ff0000"), ("mask", "m")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert_eq!(r.get(8, 3)[3], 0.0);
        assert_eq!(r.get(5, 3), [1.0, 0.0, 0.0, 0.5]);
        // preserve alpha: transparent pixels stay transparent
        let half = Image::from_fn(10, 10, |x, _| if x < 5 { [0.0, 0.0, 1.0, 1.0] } else { [0.0; 4] });
        let mut doc = testing::doc_from(half);
        run(&spec, &[("color", "#00ff00"), ("preserve-alpha", "true")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert_eq!(r.get(2, 2), [0.0, 1.0, 0.0, 1.0]);
        assert_eq!(r.get(7, 2)[3], 0.0);
        // locked layer
        doc.layers[0].locked = true;
        assert_eq!(run(&spec, &[], &mut doc).unwrap_err().code, "PX2304");
    }

    #[test]
    fn gradient_kinds_and_dither() {
        let spec = find(&ops(), "fill gradient");
        let mut doc = testing::doc_from(Image::new(101, 11));
        run(&spec, &[], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.get(0, 5)[0] < 0.01 && r.get(100, 5)[0] > 0.99);
        assert!((r.get(50, 5)[0] - 0.5).abs() < 0.01);
        for kind in ["radial", "angular", "reflected"] {
            let mut doc = testing::doc_from(Image::new(41, 41));
            run(&spec, &[("kind", kind), ("from", "20.5,20.5"), ("to", "40.5,20.5")], &mut doc).unwrap();
            let r = raster(&doc, "Background");
            match kind {
                "radial" => assert!(r.get(20, 20)[0] < 0.01 && r.get(20, 0)[0] > 0.9),
                "reflected" => assert!((r.get(5, 20)[0] - r.get(35, 20)[0]).abs() < 1e-4 && r.get(5, 20)[0] > 0.5),
                _ => assert!((r.get(39, 21)[0] - r.get(39, 19)[0]).abs() > 0.8, "angular seam along the start ray"),
            }
        }
        // oklab midpoint differs from perceptual for saturated stops
        let mut d1 = testing::doc_from(Image::new(11, 1));
        let mut d2 = testing::doc_from(Image::new(11, 1));
        run(&spec, &[("stops", "#ff0000@0,#0000ff@1")], &mut d1).unwrap();
        run(&spec, &[("stops", "#ff0000@0,#0000ff@1"), ("interp", "oklab")], &mut d2).unwrap();
        assert!(testing::max_diff(raster(&d1, "Background"), raster(&d2, "Background")) > 0.02);
        // dither is seeded and deterministic
        let mut d1 = testing::doc_from(Image::new(64, 4));
        let mut d2 = testing::doc_from(Image::new(64, 4));
        run(&spec, &[("dither", "true")], &mut d1).unwrap();
        run(&spec, &[("dither", "true")], &mut d2).unwrap();
        assert_eq!(raster(&d1, "Background"), raster(&d2, "Background"));
        assert_eq!(run(&spec, &[("from", "3,3"), ("to", "3,3")], &mut d1).unwrap_err().code, "PX1010");
        let mut d = testing::doc_from(Image::new(1, 1));
        run(&spec, &[], &mut d).unwrap();
    }

    #[test]
    fn pattern_tiles_from_file() {
        let spec = find(&ops(), "fill pattern");
        let td = testing::TempDir::new("pattern");
        let tile =
            Image::from_fn(2, 2, |x, y| if (x + y) % 2 == 0 { [1.0, 1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0] });
        let p = td.path("tile.pxt");
        std::fs::write(&p, test_io::encode(&tile)).unwrap();
        let mut doc = testing::doc_from(Image::new(8, 6));
        let v = test_io::run_io(&spec, &[("from", p.to_str().unwrap())], &mut doc).unwrap();
        assert_eq!(v["pixels_changed"], 48);
        let r = raster(&doc, "Background");
        for y in 0..6 {
            for x in 0..8 {
                assert_eq!(r.get(x, y), tile.get(x % 2, y % 2));
            }
        }
        let mut doc = testing::doc_from(Image::new(8, 6));
        doc.put_mask("left", testing::half_mask(8, 6), false).unwrap();
        test_io::run_io(
            &spec,
            &[("from", p.to_str().unwrap()), ("scale", "200"), ("angle", "30"), ("mask", "left")],
            &mut doc,
        )
        .unwrap();
        assert_eq!(raster(&doc, "Background").get(6, 2)[3], 0.0);
        assert!(raster(&doc, "Background").get(1, 2)[3] > 0.99);
        let e = test_io::run_io(&spec, &[("from", td.path("missing.pxt").to_str().unwrap())], &mut doc).unwrap_err();
        assert_eq!(e.code, "PX1101");
    }
}
