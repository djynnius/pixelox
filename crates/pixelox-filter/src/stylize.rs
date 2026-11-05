//! Stylize and lens: `filter emboss`, `filter find-edges`, `filter pixelate`,
//! `filter vignette`, `filter lens-correct`.

use crate::util::{self, Buf};
use pixelox_core::kernels::{self, Edge};
use pixelox_core::prelude::*;
use rayon::prelude::*;

const CAT: Category = Category::Filter;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("filter emboss", CAT, "Emboss into a gray relief", emboss)
            .about("Turns the layer into a gray relief lit from --angle: each pixel becomes 50% gray plus the luma difference between points --height px apart along the light direction, scaled by --amount. Alpha is kept.")
            .param(param("angle", ParamKind::Angle).default("135").help("Light direction in degrees counter-clockwise from the right"))
            .param(param("height", ParamKind::Float).default("3").range("0..100").help("Relief height: distance in px between the compared points"))
            .param(param("amount", ParamKind::Float).default("100").range("0..500").help("Relief contrast in %"))
            .params(common::region())
            .example("pixelox filter emboss logo.png -o relief.png", "Default emboss")
            .example("pixelox filter emboss photo.jpg -o relief.jpg --angle 45 --height 5 --amount 200", "Strong relief lit from the top right")
            .example("pixelox filter emboss doc.pxo --layer Texture --height 2 --in-place", "Emboss a texture layer"),
        OpSpec::doc("filter find-edges", CAT, "Highlight edges (Sobel, inverted)", find_edges)
            .about("Photoshop-style Find Edges: the Sobel gradient magnitude of each color channel, inverted, so edges are dark lines on white. Alpha is kept.")
            .params(common::region())
            .example("pixelox filter find-edges photo.jpg -o edges.png", "Line-drawing look")
            .example("pixelox filter find-edges doc.pxo --layer Background --in-place", "Edges of one layer")
            .example("pixelox filter find-edges doc.pxo --mask @sel --in-place", "Edges inside the selection"),
        OpSpec::doc("filter pixelate", CAT, "Mosaic into square or hexagonal cells", pixelate)
            .about("Replaces each cell by its average color (premultiplied, so transparency is respected). Cells are aligned to the canvas origin so layers line up; hex cells are pointy-top hexagons --cell px wide.")
            .param(param("cell", ParamKind::Int).default("8").range("1..4000").help("Cell size in px (1 = no change)"))
            .param(param("kind", ParamKind::Choice).choices(&["square", "hex"]).default("square").help("Cell shape: square or hex"))
            .params(common::region())
            .example("pixelox filter pixelate photo.jpg -o mosaic.png --cell 16", "Classic mosaic")
            .example("pixelox filter pixelate photo.jpg -o hex.png --cell 24 --kind hex", "Hexagonal mosaic")
            .example("pixelox filter pixelate doc.pxo --cell 12 --mask faces --in-place", "Anonymize faces"),
        OpSpec::doc("filter vignette", CAT, "Add (+) or remove (−) a vignette", vignette)
            .about("Darkens (positive --amount, up to 2 stops at the corners) or brightens (negative) the layer toward its edges in linear light. --midpoint moves where the falloff starts (0 = at the center, 100 = near the corners); --roundness morphs the shape from rectangular (−100) through an ellipse fitted to the layer (0) to a circle (+100).")
            .param(param("amount", ParamKind::Float).default("50").range("-100..100").help("Strength: + darkens the edges (adds a vignette), − brightens them (removes one)"))
            .param(param("midpoint", ParamKind::Float).default("50").range("0..100").help("Where the falloff begins, 0 (center) .. 100 (near the corners)"))
            .param(param("roundness", ParamKind::Float).default("0").range("-100..100").help("Shape: −100 rectangular, 0 elliptical, +100 circular"))
            .params(common::region())
            .example("pixelox filter vignette photo.jpg -o moody.jpg --amount 40", "Classic darkened corners")
            .example("pixelox filter vignette scan.jpg -o even.jpg --amount -30 --midpoint 30", "Remove lens falloff")
            .example("pixelox filter vignette portrait.jpg -o round.jpg --amount 60 --roundness 100 --midpoint 20", "Strong circular vignette"),
        OpSpec::doc("filter lens-correct", CAT, "Fix distortion, chromatic aberration, falloff", lens_correct)
            .about("Simple radial lens corrections about the layer center. --distortion: positive values correct barrel distortion, negative values correct pincushion (scaled so no empty corners appear). --ca R,B shifts the red and blue channels radially by that many px at the corners (positive = outward) to cancel color fringes. --vignette: positive values brighten the corners (correct lens falloff, up to 1 stop), negative darken them.")
            .param(param("distortion", ParamKind::Float).default("0").range("-100..100").help("Barrel (+) or pincushion (−) correction strength"))
            .param(param("ca", ParamKind::List).default("0,0").help("Radial shift R,B of the red and blue channels at the corners, in px"))
            .param(param("vignette", ParamKind::Float).default("0").range("-100..100").help("Corner brightness correction: + brightens, − darkens"))
            .params(common::region())
            .example("pixelox filter lens-correct wide.jpg -o straight.jpg --distortion 30", "Straighten barrel distortion")
            .example("pixelox filter lens-correct photo.jpg -o fixed.jpg --ca 1.5,-1", "Cancel red/cyan fringes")
            .example("pixelox filter lens-correct photo.jpg -o fixed.jpg --distortion -10 --vignette 40", "Fix pincushion and corner falloff"),
    ]
}

fn emboss(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let angle = a.angle("angle")?.to_radians();
    let height = a.f64("height")? as f32;
    let amount = a.f64("amount")? as f32 / 100.0;
    apply_to_raster(a, doc, ctx, |img, _| {
        let (w, h) = (img.width as usize, img.height as usize);
        let lum = Buf { w, h, c: 1, data: util::luma_plane(img) };
        let (dx, dy) = ((angle.cos() as f32) * height / 2.0, -(angle.sin() as f32) * height / 2.0);
        let mut out = img.clone();
        out.data.par_chunks_mut((w * 4).max(1)).enumerate().for_each(|(y, row)| {
            let (mut a1, mut b1) = ([0f32], [0f32]);
            for x in 0..w {
                let (fx, fy) = (x as f32, y as f32);
                util::sample(&lum, fx + dx, fy + dy, Edge::Clamp, &mut a1);
                util::sample(&lum, fx - dx, fy - dy, Edge::Clamp, &mut b1);
                let e = (0.5 + (a1[0] - b1[0]) * amount * 0.5).clamp(0.0, 1.0);
                row[x * 4] = e;
                row[x * 4 + 1] = e;
                row[x * 4 + 2] = e;
            }
        });
        Ok(out)
    })
}

fn find_edges(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    apply_to_raster(a, doc, ctx, |img, _| {
        let (w, h) = (img.width as usize, img.height as usize);
        let mut out = img.clone();
        for ch in 0..3 {
            let plane: Vec<f32> = img.data.par_chunks(4).map(|p| p[ch]).collect();
            let mag = crate::noise::sobel_plane(&plane, w, h);
            out.data.par_chunks_mut(4).zip(mag.par_iter()).for_each(|(o, m)| o[ch] = 1.0 - m.clamp(0.0, 1.0));
        }
        Ok(out)
    })
}

// ---------------------------------------------------------------------------------------
// pixelate

fn pixelate_square(img: &Image, cell: usize, lx: i64, ly: i64) -> Image {
    let (w, h) = (img.width as usize, img.height as usize);
    let c = cell as i64;
    let col_of = |x: usize| ((x as i64 + lx).div_euclid(c) - lx.div_euclid(c)) as usize;
    let band_of = |y: usize| ((y as i64 + ly).div_euclid(c) - ly.div_euclid(c)) as usize;
    let ncols = col_of(w - 1) + 1;
    let nbands = band_of(h - 1) + 1;
    // per band: sums of premultiplied RGBA and counts per cell
    let sums: Vec<Vec<[f64; 5]>> = (0..nbands)
        .into_par_iter()
        .map(|b| {
            let mut s = vec![[0f64; 5]; ncols];
            for y in (0..h).filter(|&y| band_of(y) == b) {
                for x in 0..w {
                    let p = img.get(x as u32, y as u32);
                    let e = &mut s[col_of(x)];
                    let a = p[3] as f64;
                    e[0] += p[0] as f64 * a;
                    e[1] += p[1] as f64 * a;
                    e[2] += p[2] as f64 * a;
                    e[3] += a;
                    e[4] += 1.0;
                }
            }
            s
        })
        .collect();
    let mut out = img.clone();
    out.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        let s = &sums[band_of(y)];
        for x in 0..w {
            row[x * 4..x * 4 + 4].copy_from_slice(&average(&s[col_of(x)]));
        }
    });
    out
}

fn average(e: &[f64; 5]) -> [f32; 4] {
    if e[3] <= 1e-9 || e[4] <= 0.0 {
        return [0.0; 4];
    }
    [(e[0] / e[3]) as f32, (e[1] / e[3]) as f32, (e[2] / e[3]) as f32, (e[3] / e[4]) as f32]
}

/// Axial hex coordinates (pointy-top, circumradius `s`) of a canvas point.
fn hex_of(px: f64, py: f64, s: f64) -> (i64, i64) {
    let q = (3f64.sqrt() / 3.0 * px - py / 3.0) / s;
    let r = (2.0 / 3.0 * py) / s;
    // cube rounding
    let (x, z) = (q, r);
    let y = -x - z;
    let (mut rx, ry, mut rz) = (x.round(), y.round(), z.round());
    let (dx, dy, dz) = ((rx - x).abs(), (ry - y).abs(), (rz - z).abs());
    if dx > dy && dx > dz {
        rx = -ry - rz;
    } else if dy <= dz {
        rz = -rx - ry;
    }
    (rx as i64, rz as i64)
}

fn pixelate_hex(img: &Image, cell: usize, lx: i64, ly: i64) -> Image {
    let (w, h) = (img.width as usize, img.height as usize);
    let s = cell as f64 / 3f64.sqrt();
    let id = |x: usize, y: usize| hex_of((x as i64 + lx) as f64 + 0.5, (y as i64 + ly) as f64 + 0.5, s);
    // dense index over the axial range of the layer
    let corners = [id(0, 0), id(w - 1, 0), id(0, h - 1), id(w - 1, h - 1)];
    let (q0, q1) = (corners.iter().map(|c| c.0).min().unwrap() - 2, corners.iter().map(|c| c.0).max().unwrap() + 2);
    let (r0, r1) = (corners.iter().map(|c| c.1).min().unwrap() - 2, corners.iter().map(|c| c.1).max().unwrap() + 2);
    let nq = (q1 - q0 + 1) as usize;
    let nr = (r1 - r0 + 1) as usize;
    let ids: Vec<u32> = (0..w * h)
        .into_par_iter()
        .map(|i| {
            let (q, r) = id(i % w, i / w);
            ((r - r0) as usize * nq + (q - q0) as usize) as u32
        })
        .collect();
    let mut sums = vec![[0f64; 5]; nq * nr];
    for (i, p) in img.data.chunks(4).enumerate() {
        let e = &mut sums[ids[i] as usize];
        let a = p[3] as f64;
        e[0] += p[0] as f64 * a;
        e[1] += p[1] as f64 * a;
        e[2] += p[2] as f64 * a;
        e[3] += a;
        e[4] += 1.0;
    }
    let mut out = img.clone();
    out.data.par_chunks_mut(4).zip(ids.par_iter()).for_each(|(o, &i)| o.copy_from_slice(&average(&sums[i as usize])));
    out
}

fn pixelate(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let cell = a.usize("cell")?.max(1);
    let hex = a.choice("kind") == "hex";
    let (lx, ly) = util::layer_offset(a, doc)?;
    let v = apply_to_raster(a, doc, ctx, |img, _| {
        if (cell <= 1 && !hex) || img.is_empty() {
            return Ok(img.clone());
        }
        Ok(if hex {
            pixelate_hex(img, cell, lx as i64, ly as i64)
        } else {
            pixelate_square(img, cell, lx as i64, ly as i64)
        })
    })?;
    Ok(util::merge(v, json!({"cell": cell})))
}

// ---------------------------------------------------------------------------------------
// vignette

/// Normalized distance from the layer center (1 at the corners) for the vignette shape.
fn vignette_dist(w: f64, h: f64, roundness: f64) -> impl Fn(f64, f64) -> f64 + Sync {
    let (hw, hh) = (w / 2.0, h / 2.0);
    let t = (roundness.max(0.0) / 100.0).min(1.0);
    let p = 2.0 + (-roundness).max(0.0) / 100.0 * 6.0;
    let s = (hw * hw + hh * hh).sqrt() / 2f64.sqrt();
    let raw = move |dx: f64, dy: f64| -> f64 {
        let (u, v) = (dx / hw.max(1e-9), dy / hh.max(1e-9));
        let (uc, vc) = (dx / s.max(1e-9), dy / s.max(1e-9));
        let (u, v) = (u + (uc - u) * t, v + (vc - v) * t);
        (u.abs().powf(p) + v.abs().powf(p)).powf(1.0 / p)
    };
    let corner = raw(hw, hh).max(1e-9);
    move |dx, dy| raw(dx, dy) / corner
}

fn vignette(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let amount = a.f64("amount")? / 100.0;
    let mid = a.f64("midpoint")? / 100.0;
    let roundness = a.f64("roundness")?;
    apply_to_raster(a, doc, ctx, |img, _| {
        if amount == 0.0 || img.is_empty() {
            return Ok(img.clone());
        }
        let (w, h) = (img.width as usize, img.height as usize);
        let dist = vignette_dist(w as f64, h as f64, roundness);
        let start = (0.7 * mid) as f32;
        let tf = util::Transfer::get();
        let mut out = img.clone();
        out.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
            let dy = y as f64 + 0.5 - h as f64 / 2.0;
            for x in 0..w {
                let d = dist(x as f64 + 0.5 - w as f64 / 2.0, dy) as f32;
                let f = util::smoothstep(start, 1.0, d);
                if f <= 0.0 {
                    continue;
                }
                let gain = (-2.0 * amount as f32 * f).exp2();
                for v in row[x * 4..x * 4 + 3].iter_mut() {
                    *v = tf.to_srgb(tf.to_linear(*v) * gain);
                }
            }
        });
        Ok(out)
    })
}

// ---------------------------------------------------------------------------------------
// lens correction

fn lens_correct(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let distortion = a.f64("distortion")?;
    let ca: Vec<f64> = a
        .list("ca")
        .iter()
        .map(|s| {
            s.parse::<f64>().map_err(|_| {
                Error::new("PX1010", format!("--ca: '{s}' is not a number"))
                    .with_hint("write --ca R,B, e.g. --ca 1.5,-1")
            })
        })
        .collect::<Result<_>>()?;
    if ca.len() != 2 {
        return Err(Error::new("PX1010", format!("--ca needs two numbers R,B, got {}", ca.len()))
            .with_hint("write --ca R,B, e.g. --ca 1.5,-1"));
    }
    let vig = a.f64("vignette")? / 100.0;
    apply_to_raster(a, doc, ctx, |img, _| {
        if (distortion == 0.0 && ca[0] == 0.0 && ca[1] == 0.0 && vig == 0.0) || img.is_empty() {
            return Ok(img.clone());
        }
        let (w, h) = (img.width as usize, img.height as usize);
        let edge = kernels::auto_edge(img);
        let src = Buf::premul(img);
        let (cx, cy) = ((w as f64 - 1.0) / 2.0, (h as f64 - 1.0) / 2.0);
        let hd = (cx * cx + cy * cy).sqrt().max(1.0);
        let k = -distortion / 100.0 * 0.25;
        let scale = if k > 0.0 { 1.0 / (1.0 + k) } else { 1.0 };
        let (fr, fb) = (1.0 - ca[0] / hd, 1.0 - ca[1] / hd);
        let tf = util::Transfer::get();
        let mut out = img.clone();
        out.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
            for x in 0..w {
                let (ux, uy) = ((x as f64 - cx) / hd, (y as f64 - cy) / hd);
                let r2 = ux * ux + uy * uy;
                let f = scale * (1.0 + k * r2);
                let at =
                    |m: f64| util::sample4(&src, (cx + ux * f * m * hd) as f32, (cy + uy * f * m * hd) as f32, edge);
                let pg = at(1.0);
                let (pr, pb) = if fr == 1.0 && fb == 1.0 { (pg, pg) } else { (at(fr), at(fb)) };
                let alpha = pg[3];
                let o = &mut row[x * 4..x * 4 + 4];
                if alpha <= 1e-6 {
                    o.fill(0.0);
                    continue;
                }
                let straight = |p: &[f32; 4], ch: usize| if p[3] > 1e-6 { p[ch] / p[3] } else { pg[ch] / alpha };
                let rgb = [straight(&pr, 0), pg[1] / alpha, straight(&pb, 2)];
                let gain = if vig != 0.0 { (vig * r2).exp2() as f32 } else { 1.0 };
                for c in 0..3 {
                    let v = rgb[c].clamp(0.0, 1.0);
                    o[c] = if gain != 1.0 { tf.to_srgb(tf.to_linear(v) * gain) } else { v };
                }
                o[3] = alpha.clamp(0.0, 1.0);
            }
        });
        Ok(out)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing::{self, find, max_diff, raster, run};

    #[test]
    fn emboss_flat_is_gray_edges_relief() {
        let spec = find(&ops(), "filter emboss");
        let mut doc = testing::doc_from(testing::solid(8, 8, Rgba::new(0.9, 0.2, 0.1, 0.7)));
        run(&spec, &[], &mut doc).unwrap();
        let p = raster(&doc, "Background").get(3, 3);
        assert!((p[0] - 0.5).abs() < 1e-5 && (p[2] - 0.5).abs() < 1e-5 && (p[3] - 0.7).abs() < 1e-6);
        let edge = Image::from_fn(20, 8, |x, _| if x < 10 { [0.0, 0.0, 0.0, 1.0] } else { [1.0; 4] });
        let mut doc = testing::doc_from(edge);
        run(&spec, &[("angle", "0"), ("height", "2")], &mut doc).unwrap();
        assert!(raster(&doc, "Background").get(9, 4)[0] > 0.6, "lit side of the step");
        let mut doc = testing::doc_from(testing::solid(1, 1, Rgba::WHITE));
        run(&spec, &[("amount", "0")], &mut doc).unwrap();
        assert!((raster(&doc, "Background").get(0, 0)[0] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn find_edges_inverted() {
        let spec = find(&ops(), "filter find-edges");
        let edge = Image::from_fn(20, 8, |x, _| if x < 10 { [0.0, 0.0, 0.0, 1.0] } else { [1.0; 4] });
        let mut doc = testing::doc_from(edge);
        doc.put_mask("left", testing::half_mask(20, 8), false).unwrap();
        run(&spec, &[], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!((r.get(3, 3)[0] - 1.0).abs() < 1e-6, "flat = white");
        assert!(r.get(9, 3)[0] < 0.1, "edge = dark");
        let mut doc =
            testing::doc_from(Image::from_fn(20, 8, |x, _| if x < 10 { [0.0, 0.0, 0.0, 1.0] } else { [1.0; 4] }));
        doc.put_mask("left", testing::half_mask(20, 8), false).unwrap();
        run(&spec, &[("mask", "left"), ("invert-mask", "true")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").get(3, 3)[0], 0.0, "outside the inverted mask unchanged");
    }

    #[test]
    fn pixelate_square_hex_identity() {
        let spec = find(&ops(), "filter pixelate");
        let src = testing::gradient(16, 16);
        let mut doc = testing::doc_from(src.clone());
        run(&spec, &[("cell", "1")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &src), 0.0);
        run(&spec, &[("cell", "4")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert_eq!(r.get(0, 0), r.get(3, 3));
        assert_ne!(r.get(3, 3), r.get(4, 3));
        assert!((r.get(0, 0)[0] - 1.5 / 15.0).abs() < 1e-5, "cell average");
        let mut doc = testing::doc_from(src.clone());
        run(&spec, &[("cell", "6"), ("kind", "hex")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        let distinct: std::collections::BTreeSet<u32> =
            r.data.chunks(4).map(|p| (p[0] * 1e4) as u32 * 10007 + (p[1] * 1e4) as u32).collect();
        assert!(distinct.len() > 4 && distinct.len() < 40, "{} cells", distinct.len());
        // transparency: a transparent cell stays transparent
        let mut doc = testing::doc_from(Image::new(8, 8));
        run(&spec, &[("cell", "4")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").alpha_bbox(0.0), None);
    }

    #[test]
    fn vignette_darkens_and_brightens() {
        let spec = find(&ops(), "filter vignette");
        let gray = testing::solid(40, 30, Rgba::new(0.5, 0.5, 0.5, 1.0));
        let mut doc = testing::doc_from(gray.clone());
        run(&spec, &[("amount", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &gray), 0.0);
        run(&spec, &[("amount", "100")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.get(0, 0)[0] < 0.3, "corner two stops darker: {}", r.get(0, 0)[0]);
        assert!((r.get(20, 15)[0] - 0.5).abs() < 1e-3, "center unchanged");
        for rd in ["-100", "100"] {
            let mut doc = testing::doc_from(gray.clone());
            run(&spec, &[("amount", "-60"), ("roundness", rd)], &mut doc).unwrap();
            assert!(raster(&doc, "Background").get(0, 0)[0] > 0.6, "corner brighter");
        }
        let mut doc = testing::doc_from(testing::solid(1, 1, Rgba::new(0.5, 0.5, 0.5, 1.0)));
        run(&spec, &[("amount", "100")], &mut doc).unwrap();
    }

    #[test]
    fn lens_correct_identity_distortion_ca() {
        let spec = find(&ops(), "filter lens-correct");
        let src = testing::checker(33, 33);
        let mut doc = testing::doc_from(src.clone());
        run(&spec, &[], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &src), 0.0);
        run(&spec, &[("distortion", "40")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert_eq!(r.get(16, 16), src.get(16, 16), "center fixed");
        assert!(max_diff(r, &src) > 0.1, "periphery moved");
        let mut doc = testing::doc_from(testing::solid(21, 21, Rgba::new(0.5, 0.5, 0.5, 1.0)));
        run(&spec, &[("ca", "2,-2"), ("vignette", "50")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.get(0, 0)[0] > 0.55 && (r.get(10, 10)[0] - 0.5).abs() < 1e-4);
        let e = run(&spec, &[("ca", "1")], &mut doc).unwrap_err();
        assert_eq!(e.code, "PX1010");
    }
}
