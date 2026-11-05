//! Sharpening: `filter unsharp`, `filter smart-sharpen`, `filter high-pass`.

use crate::kern::{self, Shape};
use crate::util::Buf;
use pixelox_core::kernels;
use pixelox_core::prelude::*;
use rayon::prelude::*;

const CAT: Category = Category::Filter;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("filter unsharp", CAT, "Unsharp mask sharpening", unsharp)
            .about("Classic unsharp mask: adds --amount % of the difference between each pixel and its Gaussian blur (sigma = --radius). Differences smaller than --threshold levels (0..255) are left alone, which keeps smooth areas and noise from being sharpened.")
            .param(param("amount", ParamKind::Float).default("100").range("0..500").help("Strength in % of the detail added back (0 = no change)"))
            .param(param("radius", ParamKind::Float).default("1").range("0..1000").help("Blur radius (Gaussian sigma) in px that defines the detail scale"))
            .param(param("threshold", ParamKind::Float).default("0").range("0..255").help("Minimum difference in levels 0..255 before a pixel is sharpened"))
            .params(common::region())
            .example("pixelox filter unsharp photo.jpg -o crisp.jpg --amount 120 --radius 1.2", "General sharpening")
            .example("pixelox filter unsharp portrait.jpg -o crisp.jpg --amount 80 --radius 2 --threshold 6", "Sharpen but spare skin texture")
            .example("pixelox filter unsharp doc.pxo --amount 60 --radius 30 --in-place", "Local contrast boost (large radius)"),
        OpSpec::doc("filter smart-sharpen", CAT, "Smart sharpen with halo and noise control", smart)
            .about("Removes a modeled blur (--kind lens: a defocus disc of --radius; gaussian: a Gaussian of sigma --radius; motion: a streak of 2×--radius px at --angle) by adding back the difference, with noise coring (--reduce-noise softens small differences) and halo suppression (results are clamped near the local min/max of a (2R+1)² window, R ≤ 8).")
            .param(param("amount", ParamKind::Float).default("100").range("0..500").help("Strength in % (0 = no change)"))
            .param(param("radius", ParamKind::Float).default("1").range("0..64").help("Size of the blur being removed, in px"))
            .param(param("reduce-noise", ParamKind::Float).default("10").range("0..100").help("Noise reduction 0..100: small differences are sharpened less"))
            .param(param("kind", ParamKind::Choice).choices(&["lens", "gaussian", "motion"]).default("lens").help("Blur to remove: lens (defocus disc), gaussian, or motion"))
            .param(param("angle", ParamKind::Angle).default("0").help("Direction of the motion blur in degrees (with --kind motion)"))
            .params(common::region())
            .example("pixelox filter smart-sharpen photo.jpg -o crisp.jpg --amount 150 --radius 1.5", "Recover slight defocus")
            .example("pixelox filter smart-sharpen shake.jpg -o fixed.jpg --kind motion --angle 20 --radius 3", "Reduce camera-shake blur")
            .example("pixelox filter smart-sharpen doc.pxo --amount 120 --radius 2 --reduce-noise 40 --mask subject --in-place", "Sharpen the subject, not the noise"),
        OpSpec::doc("filter high-pass", CAT, "High-pass filter for frequency separation", high_pass)
            .about("Keeps detail finer than --radius on a 50% gray base (0.5 + pixel − Gaussian blur). Blend the result in overlay or soft-light mode to sharpen, or use it as the high band of a frequency separation.")
            .param(param("radius", ParamKind::Float).default("10").range("0..1000").help("Detail scale (Gaussian sigma) in px; 0 gives flat gray"))
            .params(common::region())
            .example("pixelox filter high-pass photo.jpg -o detail.png --radius 4", "Extract fine detail")
            .example("pixelox filter high-pass doc.pxo --layer Detail --radius 3 --in-place", "High band of a frequency separation")
            .example("pixelox filter high-pass doc.pxo --layer Texture --radius 12 --mask @sel --in-place", "Detail inside the selection"),
    ]
}

/// Add `amount` × (pixel − blurred) per color channel where the difference reaches
/// `threshold` levels.
pub fn unsharp_with(img: &Image, blurred: &Image, amount: f32, threshold: f32) -> Image {
    let mut out = img.clone();
    out.data.par_chunks_mut(4).zip(blurred.data.par_chunks(4)).for_each(|(o, b)| {
        if o[3] <= 0.0 {
            return;
        }
        for c in 0..3 {
            let d = o[c] - b[c];
            if d.abs() * 255.0 < threshold {
                continue;
            }
            o[c] = (o[c] + amount * d).clamp(0.0, 1.0);
        }
    });
    out
}

/// Unsharp mask with a Gaussian of `sigma`.
pub fn unsharp_image(img: &Image, amount: f32, sigma: f64, threshold: f32) -> Image {
    if amount <= 0.0 || sigma <= 0.0 {
        return img.clone();
    }
    let blurred = kern::gaussian_image(img, sigma);
    unsharp_with(img, &blurred, amount, threshold)
}

fn unsharp(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let amount = (a.f64("amount")? / 100.0) as f32;
    let sigma = a.f64("radius")?;
    let threshold = a.f64("threshold")? as f32;
    apply_to_raster(a, doc, ctx, |img, _| Ok(unsharp_image(img, amount, sigma, threshold)))
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Psf {
    Lens,
    Gaussian,
    Motion,
}

fn smart_sharpen(img: &Image, amount: f32, radius: f64, reduce: f32, psf: Psf, angle: f64) -> Image {
    let edge = kernels::auto_edge(img);
    let blurred = match psf {
        Psf::Gaussian => kern::gaussian_image(img, radius),
        Psf::Lens => kern::shape_blur(&Buf::premul(img), radius.max(0.5) as f32, Shape::Circle, edge).into_unpremul(),
        Psf::Motion => kern::motion_blur(&Buf::premul(img), angle, 2.0 * radius, edge).into_unpremul(),
    };
    let (w, h) = (img.width as usize, img.height as usize);
    let hr = (radius.round() as usize).clamp(1, 8);
    let k2 = (reduce / 100.0 * 0.04).powi(2);
    let mut out = img.clone();
    for ch in 0..3 {
        let plane: Vec<f32> = img.data.par_chunks(4).map(|p| p[ch]).collect();
        let (mn, mx) = kern::min_max(&plane, w, h, hr);
        out.data.par_chunks_mut(4).zip(blurred.data.par_chunks(4)).enumerate().for_each(|(i, (o, b))| {
            if o[3] <= 0.0 {
                return;
            }
            let d = o[ch] - b[ch];
            let d = if k2 > 0.0 { d * d * d / (d * d + k2) } else { d };
            let s = o[ch] + amount * d;
            // halo suppression: allow 10% overshoot of the local range
            let margin = 0.1 * (mx[i] - mn[i]);
            o[ch] = s.clamp(mn[i] - margin, mx[i] + margin).clamp(0.0, 1.0);
        });
    }
    out
}

fn smart(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let amount = (a.f64("amount")? / 100.0) as f32;
    let radius = a.f64("radius")?;
    let reduce = a.f64("reduce-noise")? as f32;
    let psf = match a.choice("kind").as_str() {
        "gaussian" => Psf::Gaussian,
        "motion" => Psf::Motion,
        _ => Psf::Lens,
    };
    let angle = a.angle("angle")?;
    apply_to_raster(a, doc, ctx, |img, _| {
        if amount <= 0.0 || radius <= 0.0 {
            return Ok(img.clone());
        }
        Ok(smart_sharpen(img, amount, radius, reduce, psf, angle))
    })
}

fn high_pass(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let sigma = a.f64("radius")?;
    apply_to_raster(a, doc, ctx, |img, _| {
        let blurred = kern::gaussian_image(img, sigma);
        let mut out = img.clone();
        out.data.par_chunks_mut(4).zip(blurred.data.par_chunks(4)).for_each(|(o, b)| {
            for c in 0..3 {
                o[c] = (0.5 + o[c] - b[c]).clamp(0.0, 1.0);
            }
        });
        Ok(out)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing::{self, find, max_diff, raster, run};

    fn edge_img() -> Image {
        Image::from_fn(24, 8, |x, _| if x < 12 { [0.3, 0.3, 0.3, 1.0] } else { [0.7, 0.7, 0.7, 1.0] })
    }

    #[test]
    fn unsharp_increases_edge_contrast() {
        let spec = find(&ops(), "filter unsharp");
        let mut doc = testing::doc_from(edge_img());
        run(&spec, &[("amount", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &edge_img()), 0.0);
        run(&spec, &[("amount", "100"), ("radius", "1.5")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.get(11, 4)[0] < 0.3 && r.get(12, 4)[0] > 0.7);
        assert!((r.get(2, 4)[0] - 0.3).abs() < 1e-4, "flat area unchanged");
        // threshold larger than the edge step leaves it alone
        let mut doc = testing::doc_from(edge_img());
        run(&spec, &[("amount", "100"), ("radius", "1.5"), ("threshold", "200")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &edge_img()), 0.0);
        // mask limits to the left half
        let mut doc = testing::doc_from(edge_img());
        doc.put_mask("left", testing::half_mask(24, 8), false).unwrap();
        run(&spec, &[("amount", "100"), ("radius", "1.5"), ("mask", "left")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").get(12, 4)[0], 0.7);
    }

    #[test]
    fn smart_sharpen_kinds_limit_halos() {
        let spec = find(&ops(), "filter smart-sharpen");
        for kind in ["lens", "gaussian", "motion"] {
            let mut doc = testing::doc_from(edge_img());
            run(&spec, &[("amount", "300"), ("radius", "2"), ("kind", kind), ("reduce-noise", "0")], &mut doc).unwrap();
            let r = raster(&doc, "Background");
            assert!(r.get(12, 4)[0] > 0.7, "{kind} sharpens");
            assert!(r.get(12, 4)[0] <= 0.7 + 0.1 * 0.4 + 1e-4, "{kind} halo clamped: {}", r.get(12, 4)[0]);
        }
        let mut doc = testing::doc_from(edge_img());
        run(&spec, &[("amount", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &edge_img()), 0.0);
        let mut doc = testing::doc_from(Image::new(4, 4));
        run(&spec, &[("amount", "200")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").alpha_bbox(0.0), None);
    }

    #[test]
    fn high_pass_gray_base() {
        let spec = find(&ops(), "filter high-pass");
        let mut doc = testing::doc_from(testing::solid(6, 6, Rgba::new(0.9, 0.1, 0.4, 1.0)));
        run(&spec, &[("radius", "3")], &mut doc).unwrap();
        let p = raster(&doc, "Background").get(2, 2);
        assert!((p[0] - 0.5).abs() < 1e-4 && (p[1] - 0.5).abs() < 1e-4);
        let mut doc = testing::doc_from(edge_img());
        run(&spec, &[("radius", "0")], &mut doc).unwrap();
        assert!((raster(&doc, "Background").get(11, 4)[0] - 0.5).abs() < 1e-6, "radius 0 is flat gray");
        let mut doc = testing::doc_from(edge_img());
        run(&spec, &[("radius", "2")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.get(11, 4)[0] < 0.5 && r.get(12, 4)[0] > 0.5);
        let mut doc = testing::doc_from(testing::solid(1, 1, Rgba::WHITE));
        run(&spec, &[("radius", "5")], &mut doc).unwrap();
        assert!((raster(&doc, "Background").get(0, 0)[0] - 0.5).abs() < 1e-4);
    }
}
