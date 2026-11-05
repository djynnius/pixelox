//! Noise: `filter median`, `filter add-noise`, `filter denoise`, `filter despeckle`,
//! `filter dust`.

use crate::kern;
use crate::util::{self, Buf};
use pixelox_core::prelude::*;
use rand_core::RngCore;
use rayon::prelude::*;

const CAT: Category = Category::Filter;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("filter median", CAT, "Median filter (constant-time histogram)", median)
            .about("Replaces each channel by the median of a (2R+1)² window: removes speckles while keeping edges. Radius 1 is an exact 3×3 sorting network; larger radii use the constant-time histogram method (Perreault & Hébert), so any radius costs O(pixels), ranking at 8-bit precision (a pixel whose own value is the median keeps its exact value).")
            .param(param("radius", ParamKind::Int).default("1").range("0..100").help("Window half-width in px (0 = no change)"))
            .params(common::region())
            .example("pixelox filter median scan.png -o clean.png --radius 1", "Remove salt-and-pepper noise")
            .example("pixelox filter median photo.jpg -o painterly.jpg --radius 6", "Painterly flattening")
            .example("pixelox filter median doc.pxo --radius 2 --mask sky --in-place", "Clean only the sky"),
        OpSpec::doc("filter add-noise", CAT, "Add seeded Gaussian or uniform noise", add_noise)
            .about("Adds noise to the color channels: uniform noise lies in ±amount, Gaussian noise has a standard deviation of amount/2 (amount in % of full scale). --monochrome adds the same value to R, G, and B. Draws come from the op's seeded stream, one lane per row, so output is identical for a given --seed regardless of thread count.")
            .param(param("amount", ParamKind::Float).default("10").range("0..400").help("Noise strength in % of full scale (0 = no change)"))
            .param(param("kind", ParamKind::Choice).choices(&["gaussian", "uniform"]).default("gaussian").help("Distribution: gaussian (bell curve) or uniform"))
            .param(param("monochrome", ParamKind::Flag).help("Same noise on every color channel (luminance grain)"))
            .params(common::region())
            .example("pixelox filter add-noise photo.jpg -o grain.jpg --amount 6 --monochrome", "Film-like grain")
            .example("pixelox filter add-noise gradient.png -o dithered.png --amount 1 --kind uniform", "Break up banding")
            .example("pixelox filter add-noise doc.pxo --amount 12 --seed 7 --mask @sel --in-place", "Reproducible noise in the selection"),
        OpSpec::doc("filter denoise", CAT, "Edge-aware noise reduction (non-local means)", denoise)
            .about("Non-local means: every pixel becomes a weighted mean of the pixels in its 5×5 search window, weighted by how similar their 3×3 neighborhoods are, so edges and texture survive. The windows are fixed (cost: 12 mirrored offset pairs, O(pixels) each). --strength sets the similarity tolerance; --preserve-details restores strong local detail removed by the filter.")
            .param(param("strength", ParamKind::Float).default("50").range("0..100").help("How much noise to remove, 0..100 (0 = no change)"))
            .param(param("preserve-details", ParamKind::Float).default("30").range("0..100").help("How much strong fine detail to restore, 0..100"))
            .params(common::region())
            .example("pixelox filter denoise night.jpg -o clean.jpg --strength 60", "Clean a noisy low-light photo")
            .example("pixelox filter denoise portrait.jpg -o clean.jpg --strength 40 --preserve-details 60", "Gentle cleanup that keeps texture")
            .example("pixelox filter denoise doc.pxo --strength 80 --mask sky --in-place", "Strong smoothing of the sky"),
        OpSpec::doc("filter despeckle", CAT, "Remove speckles while keeping edges", despeckle)
            .about("Applies a 3×3 median everywhere except along edges (detected with a Sobel operator on luma), like Photoshop's Despeckle.")
            .params(common::region())
            .example("pixelox filter despeckle scan.jpg -o clean.jpg", "Clean a scanned print")
            .example("pixelox filter despeckle doc.pxo --layer Background --in-place", "Despeckle one layer")
            .example("pixelox filter despeckle doc.pxo --mask @sel --in-place", "Despeckle inside the selection"),
        OpSpec::doc("filter dust", CAT, "Dust and scratches removal", dust)
            .about("Replaces a pixel by the median of its (2R+1)² window when it differs from that median by more than --threshold levels (0..255), so isolated dust and scratches vanish while texture below the threshold is kept.")
            .param(param("radius", ParamKind::Int).default("1").range("0..100").help("Window half-width in px: about the size of the defects (0 = no change)"))
            .param(param("threshold", ParamKind::Float).default("0").range("0..255").help("Only replace pixels differing from the median by more than this many levels"))
            .params(common::region())
            .example("pixelox filter dust scan.png -o clean.png --radius 2 --threshold 20", "Remove dust from a film scan")
            .example("pixelox filter dust photo.jpg -o clean.jpg --radius 4 --threshold 30", "Remove thin scratches")
            .example("pixelox filter dust doc.pxo --radius 3 --threshold 15 --mask sky --in-place", "Clean the sky only"),
    ]
}

fn median(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let r = a.usize("radius")?;
    apply_to_raster(a, doc, ctx, |img, _| Ok(kern::median_image(img, r)))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum NoiseKind {
    Gaussian,
    Uniform,
}

/// Uniform draw in [0, 1).
#[inline]
pub(crate) fn uni(rng: &mut impl RngCore) -> f32 {
    (rng.next_u32() >> 8) as f32 * (1.0 / 16_777_216.0)
}

/// One noise draw from `rng`; `spare` caches the second Box–Muller value.
#[inline]
fn draw(rng: &mut impl RngCore, kind: NoiseKind, amount: f32, spare: &mut Option<f32>) -> f32 {
    match kind {
        NoiseKind::Uniform => (uni(rng) * 2.0 - 1.0) * amount,
        NoiseKind::Gaussian => {
            if let Some(v) = spare.take() {
                return v * amount * 0.5;
            }
            let u1 = 1.0 - uni(rng); // (0, 1]
            let u2 = uni(rng);
            let m = (-2.0 * u1.ln()).sqrt();
            let (s, c) = (std::f32::consts::TAU * u2).sin_cos();
            *spare = Some(m * s);
            m * c * amount * 0.5
        }
    }
}

fn add_noise(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let amount = (a.f64("amount")? / 100.0) as f32;
    let kind = if a.choice("kind") == "uniform" { NoiseKind::Uniform } else { NoiseKind::Gaussian };
    let mono = a.flag("monochrome");
    apply_to_raster(a, doc, ctx, |img, ctx| {
        if amount <= 0.0 || img.is_empty() {
            return Ok(img.clone());
        }
        let w = img.width as usize;
        let mut rngs: Vec<_> = (0..img.height as u64).map(|y| ctx.rng_lane(y)).collect();
        let mut out = img.clone();
        out.data.par_chunks_mut(w * 4).zip(rngs.par_iter_mut()).for_each(|(row, rng)| {
            let mut spare = None;
            for p in row.chunks_mut(4) {
                if mono {
                    let n = draw(rng, kind, amount, &mut spare);
                    for v in p.iter_mut().take(3) {
                        *v = (*v + n).clamp(0.0, 1.0);
                    }
                } else {
                    for v in p.iter_mut().take(3) {
                        *v = (*v + draw(rng, kind, amount, &mut spare)).clamp(0.0, 1.0);
                    }
                }
            }
        });
        Ok(out)
    })
}

fn denoise(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let strength = a.f64("strength")? as f32;
    let preserve = a.f64("preserve-details")? as f32 / 100.0;
    apply_to_raster(a, doc, ctx, |img, _| {
        if strength <= 0.0 {
            return Ok(img.clone());
        }
        let h = 0.01 + strength / 100.0 * 0.12;
        let p = Buf::premul(img);
        let mut d = kern::nlm(&p, h);
        if preserve > 0.0 {
            // restore large (structural) differences, not small (noise) ones
            let k2 = (h * 0.5).powi(2);
            d.data.par_iter_mut().zip(p.data.par_iter()).for_each(|(o, &orig)| {
                let det = orig - *o;
                *o += preserve * det * det * det / (det * det + k2);
            });
        }
        Ok(d.into_unpremul())
    })
}

/// Sobel gradient magnitude of a plane (clamped edges).
fn sobel(plane: &[f32], w: usize, h: usize) -> Vec<f32> {
    let (wi, hi) = (w as i64, h as i64);
    let at = |x: i64, y: i64| plane[y.clamp(0, hi - 1) as usize * w + x.clamp(0, wi - 1) as usize];
    let mut out = vec![0f32; w * h];
    out.par_chunks_mut(w.max(1)).enumerate().for_each(|(y, row)| {
        let y = y as i64;
        for (x, o) in row.iter_mut().enumerate() {
            let x = x as i64;
            let gx = at(x + 1, y - 1) + 2.0 * at(x + 1, y) + at(x + 1, y + 1)
                - at(x - 1, y - 1)
                - 2.0 * at(x - 1, y)
                - at(x - 1, y + 1);
            let gy = at(x - 1, y + 1) + 2.0 * at(x, y + 1) + at(x + 1, y + 1)
                - at(x - 1, y - 1)
                - 2.0 * at(x, y - 1)
                - at(x + 1, y - 1);
            *o = (gx * gx + gy * gy).sqrt() / 4.0;
        }
    });
    out
}

pub(crate) fn sobel_plane(plane: &[f32], w: usize, h: usize) -> Vec<f32> {
    sobel(plane, w, h)
}

fn despeckle(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    apply_to_raster(a, doc, ctx, |img, _| {
        let (w, h) = (img.width as usize, img.height as usize);
        if w == 0 || h == 0 {
            return Ok(img.clone());
        }
        let med = kern::median_image(img, 1);
        let edges = sobel(&util::luma_plane(img), w, h);
        let mut out = med;
        out.data.par_chunks_mut(4).zip(img.data.par_chunks(4)).zip(edges.par_iter()).for_each(|((o, s), &e)| {
            let keep = util::smoothstep(0.08, 0.3, e);
            for c in 0..4 {
                o[c] += (s[c] - o[c]) * keep;
            }
        });
        Ok(out)
    })
}

fn dust(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let r = a.usize("radius")?;
    let threshold = a.f64("threshold")? as f32;
    let mut changed = 0usize;
    let v = apply_to_raster(a, doc, ctx, |img, _| {
        if r == 0 {
            return Ok(img.clone());
        }
        let med = kern::median_image(img, r);
        let mut out = img.clone();
        let counts: Vec<usize> = out
            .data
            .par_chunks_mut(4)
            .zip(med.data.par_chunks(4))
            .map(|(o, m)| {
                let mut hit = false;
                for c in 0..4 {
                    if (o[c] - m[c]).abs() * 255.0 > threshold {
                        o[c] = m[c];
                        hit = true;
                    }
                }
                hit as usize
            })
            .collect();
        changed = counts.iter().sum();
        Ok(out)
    })?;
    Ok(util::merge(v, json!({"pixels_changed": changed})))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing::{self, find, max_diff, raster, run};

    fn speckled() -> Image {
        Image::from_fn(
            20,
            20,
            |x, y| if (x * 7 + y * 3) % 11 == 0 { [1.0, 1.0, 1.0, 1.0] } else { [0.2, 0.3, 0.4, 1.0] },
        )
    }

    #[test]
    fn median_removes_speckles_and_identity() {
        let spec = find(&ops(), "filter median");
        let mut doc = testing::doc_from(speckled());
        run(&spec, &[("radius", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &speckled()), 0.0);
        run(&spec, &[("radius", "1")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.data.chunks(4).all(|p| (p[0] - 0.2).abs() < 1e-6), "speckles gone, exact value kept");
        let mut doc = testing::doc_from(testing::solid(1, 1, Rgba::new(0.5, 0.5, 0.5, 0.5)));
        run(&spec, &[("radius", "3")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").get(0, 0)[3], 0.5);
    }

    #[test]
    fn add_noise_seeded_and_mono() {
        let spec = find(&ops(), "filter add-noise");
        let base = testing::solid(16, 16, Rgba::new(0.5, 0.5, 0.5, 1.0));
        let mut d1 = testing::doc_from(base.clone());
        let mut d2 = testing::doc_from(base.clone());
        run(&spec, &[("amount", "10")], &mut d1).unwrap();
        run(&spec, &[("amount", "10")], &mut d2).unwrap();
        assert_eq!(max_diff(raster(&d1, "Background"), raster(&d2, "Background")), 0.0, "deterministic");
        assert!(max_diff(raster(&d1, "Background"), &base) > 0.01);
        let sd = {
            let r = raster(&d1, "Background");
            (r.data.chunks(4).map(|p| (p[0] as f64 - 0.5).powi(2)).sum::<f64>() / 256.0).sqrt()
        };
        assert!((sd - 0.05).abs() < 0.015, "gaussian sd = amount/2: {sd}");
        let mut d = testing::doc_from(base.clone());
        run(&spec, &[("amount", "20"), ("monochrome", "true"), ("kind", "uniform")], &mut d).unwrap();
        let r = raster(&d, "Background");
        assert!(r.data.chunks(4).all(|p| p[0] == p[1] && p[1] == p[2] && (p[0] - 0.5).abs() <= 0.2 + 1e-6));
        let mut d = testing::doc_from(base.clone());
        run(&spec, &[("amount", "0")], &mut d).unwrap();
        assert_eq!(max_diff(raster(&d, "Background"), &base), 0.0);
        // mask: right half untouched
        let mut d = testing::doc_from(base.clone());
        d.put_mask("left", testing::half_mask(16, 16), false).unwrap();
        run(&spec, &[("amount", "30"), ("mask", "left")], &mut d).unwrap();
        assert_eq!(raster(&d, "Background").get(12, 3)[0], 0.5);
    }

    #[test]
    fn denoise_reduces_noise_keeps_edge() {
        let spec = find(&ops(), "filter denoise");
        let noisy = Image::from_fn(32, 32, |x, y| {
            let n = (((x.wrapping_mul(2654435761u32) ^ y.wrapping_mul(40503)) % 97) as f32 / 97.0 - 0.5) * 0.08;
            let v = if x < 16 { 0.25 } else { 0.75 } + n;
            [v, v, v, 1.0]
        });
        let mut doc = testing::doc_from(noisy.clone());
        run(&spec, &[("strength", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &noisy), 0.0);
        run(&spec, &[("strength", "70"), ("preserve-details", "0")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        let dev = |i: &Image| (2..14).map(|x| (i.get(x, 10)[0] - 0.25).abs()).sum::<f32>();
        assert!(dev(r) < dev(&noisy) * 0.7, "{} vs {}", dev(r), dev(&noisy));
        assert!(r.get(13, 10)[0] < 0.4 && r.get(18, 10)[0] > 0.6, "edge kept");
        let mut doc = testing::doc_from(Image::new(5, 5));
        run(&spec, &[("strength", "50")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").alpha_bbox(0.0), None);
    }

    #[test]
    fn despeckle_and_dust() {
        let spec = find(&ops(), "filter despeckle");
        let mut doc = testing::doc_from(speckled());
        run(&spec, &[], &mut doc).unwrap();
        let before = speckled().data.chunks(4).filter(|p| p[0] > 0.9).count();
        let after = raster(&doc, "Background").data.chunks(4).filter(|p| p[0] > 0.9).count();
        assert!(after < before);
        let mut doc = testing::doc_from(testing::solid(1, 1, Rgba::WHITE));
        run(&spec, &[], &mut doc).unwrap();

        let spec = find(&ops(), "filter dust");
        let mut doc = testing::doc_from(speckled());
        let v = run(&spec, &[("radius", "1"), ("threshold", "40")], &mut doc).unwrap();
        assert!(v["pixels_changed"].as_u64().unwrap() > 0);
        assert!(raster(&doc, "Background").data.chunks(4).all(|p| p[0] < 0.3));
        let mut doc = testing::doc_from(speckled());
        let v = run(&spec, &[("radius", "1"), ("threshold", "255")], &mut doc).unwrap();
        assert_eq!(v["pixels_changed"], 0);
        let mut doc = testing::doc_from(speckled());
        run(&spec, &[("radius", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &speckled()), 0.0);
    }
}
