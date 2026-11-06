//! `retouch frequency-split`: frequency separation that reconstructs the source exactly.
//!
//! Low is the Gaussian-blurred source; High holds the rest as 0.5 + (source − Low)/2, so a
//! linear-light blend (base + 2·blend − 1) of High over Low gives the source back. Both are
//! stored at the document's depth with Low rounded to whichever neighboring level makes
//! (source − Low) odd in levels: then High lands exactly on a level too and the pair
//! reproduces the source to float rounding instead of drifting by a level.

use crate::kern;
use crate::retouch::push_pull;
use crate::util::Buf;
use pixelox_core::kernels::Edge;
use pixelox_core::prelude::*;
use rayon::prelude::*;

const DEFAULT_GROUP: &str = "Frequency Separation";

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("retouch frequency-split", Category::Retouch, "Frequency separation into Low and High layers", split)
        .about("Splits a layer into a group above it holding Low (normal: the layer blurred with a Gaussian of --radius, which carries tone and color) and High (linear light: 0.5 + (layer − Low)/2 per channel, which carries texture such as pores). This is Photoshop's Apply Image Subtract with scale 2 and offset 128; `filter high-pass` (0.5 + layer − blur) does not reconstruct under linear light. The group blends normally (isolated) and the source stays unchanged underneath. Values are stored at the document's depth, with Low rounded to the neighboring level that makes the pair exact, so the composite reproduces the source to float rounding at 16 and 8 bits (never more than 1 level off at 8 bits); the measured maximum difference is reported as reconstruction.max_error_levels. Low becomes the active layer: smooth it (for example `filter surface-blur` or `filter gaussian` with --layer Low and a skin mask) to even out tone and blotches while High keeps the texture; heal or clone on High to remove texture only. Heal blemishes before splitting (`retouch blemishes --heal`): Low and High are copies, so later edits to the source do not show through. A source that is not fully opaque at 100% normal (transparency, opacity, blend mode, layer mask, or Blend If) gets opaque Low and High (colors extended past its edges), a group mask equal to its alpha times its layer mask, its opacity, blend mode, and Blend If on the group, and is hidden so its pixels are not counted twice. Groups are rendered isolated; adjustment layers have no pixels (PX2303).")
        .param(param("layer", ParamKind::Layer).help("Source layer, any kind (rendered with its styles): name, #N (1 = top), @top, @bottom, or @active; default: the only layer, else the active one"))
        .param(
            param("radius", ParamKind::Float)
                .default("auto")
                .auto()
                .range("0.5..1000")
                .help("Gaussian sigma in px between the bands: detail finer than this goes to High (auto = 0.15% of the long side, at least 2)"),
        )
        .param(param("name", ParamKind::Str).default(DEFAULT_GROUP).help("Name of the new group (made unique; its layers are Low and High)"))
        .example("pixelox retouch frequency-split portrait.pxo --in-place", "Split the only layer at the automatic radius")
        .example("pixelox retouch frequency-split doc.pxo --layer Photo --radius 6 --in-place", "Split at sigma 6 for a high-resolution face")
        .example("pixelox retouch frequency-split photo.jpg -o split.pxo --name 'FS skin'", "Start a layered document with a named split")
        .example("pixelox retouch frequency-split doc.pxo --layer Photo --dry-run --json", "See the radius and layer names it would use")]
}

/// The source layer: `--layer`, else the only layer, else the active one.
fn source_path(a: &Args, doc: &Document) -> Result<Vec<usize>> {
    if let Some(r) = a.raw("layer") {
        return doc.resolve(r);
    }
    match doc.resolve_or_only(None) {
        Err(e) if e.code == "PX2302" => doc.active.as_deref().and_then(|id| doc.find_by_id(id)).ok_or(e),
        other => other,
    }
}

/// A layer mask's effective per-pixel coverage (feather and density applied), as the
/// compositor uses it.
fn mask_coverage(lm: &LayerMask) -> Mask {
    let mut m = if lm.feather > 0.0 { kernels::blur_mask(&lm.mask, lm.feather as f64 / 2.0) } else { lm.mask.clone() };
    let d = lm.density / 100.0;
    m.data.par_iter_mut().for_each(|v| *v = 1.0 - d * (1.0 - *v));
    m
}

/// Low and High for the rendered source `src`, at `levels` (255 or 65535) per channel.
/// Both are opaque; for pixels the source does not cover, Low's colors are extended from
/// the nearest content and High is neutral.
pub(crate) fn bands(src: &Image, sigma: f64, levels: u32, opaque: bool) -> (Image, Image) {
    let (w, h) = (src.width as usize, src.height as usize);
    // Low before rounding: a premultiplied blur, renormalized by the blurred alpha
    let mut b = Buf::premul(src);
    kern::gaussian(&mut b, sigma, sigma, if opaque { Edge::Clamp } else { Edge::Zero });
    let mut low = b.data;
    let mut defined = vec![1f32; w * h];
    low.par_chunks_mut(4).zip(defined.par_iter_mut()).for_each(|(p, d)| {
        if p[3] > 1e-4 {
            let inv = 1.0 / p[3];
            for c in 0..3 {
                p[c] = (p[c] * inv).clamp(0.0, 1.0);
            }
        } else {
            p[..3].fill(0.0);
            *d = 0.0;
        }
        p[3] = 1.0;
    });
    if defined.iter().any(|&d| d < 1.0) {
        push_pull(&mut low, &defined, w, h);
    }
    let n = levels as i64;
    let nf = levels as f64;
    let mut high = vec![0f32; w * h * 4];
    low.par_chunks_mut(4).zip(high.par_chunks_mut(4)).zip(src.data.par_chunks(4)).for_each(|((l, hi), s)| {
        for c in 0..3 {
            let lr = (l[c] as f64).clamp(0.0, 1.0) * nf;
            // where the source is fully transparent its color is meaningless: keep High neutral
            let iv = if s[3] > 0.0 { s[c] as f64 } else { l[c] as f64 };
            let iq = (iv.clamp(0.0, 1.0) * nf).round() as i64;
            let f = (lr.floor() as i64).clamp(0, n - 1);
            // (iq − ll) must be odd so that (iq − ll + n) / 2 is a whole level (n is odd)
            let ll = if (iq - f).rem_euclid(2) == 1 { f } else { f + 1 };
            let hl = (iq - ll + n) / 2;
            l[c] = (ll as f64 / nf) as f32;
            hi[c] = (hl as f64 / nf) as f32;
        }
        l[3] = 1.0;
        hi[3] = 1.0;
    });
    (
        Image { width: src.width, height: src.height, data: low },
        Image { width: src.width, height: src.height, data: high },
    )
}

fn split(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let path = source_path(a, doc)?;
    let sigma = if a.is_auto("radius") {
        let s = (doc.width.max(doc.height) as f64 * 15.0 / 10_000.0).max(2.0);
        // full precision: a replay must rebuild exactly the same bands
        ctx.resolve_param("radius", s.to_string());
        s
    } else {
        a.f64("radius")?
    };
    let src = doc.layer(&path);
    let src_name = src.name.clone();
    if !src.visible {
        ctx.warn("PXW2001", format!("layer '{src_name}' is hidden; the split is visible"));
    }
    if matches!(src.content, LayerKind::Adjustment { .. }) {
        return Err(Error::new(
            "PX2303",
            format!("layer '{src_name}' is an adjustment layer; frequency separation needs pixels"),
        )
        .with_hint("split a pixel, text, shape, placed, or group layer: pass --layer <name>")
        .with_context("layer", &src_name)
        .with_context("kind", "adjustment"));
    }
    let rendered = compose::render_layer(doc, src)?.filter(|r| !r.image.is_empty()).ok_or_else(|| {
        Error::new("PX3320", format!("layer '{src_name}' has no pixels to separate")).with_context("layer", &src_name)
    })?;
    let levels: u32 = if doc.depth == 16 { 65535 } else { 255 };
    let opaque = rendered.image.data.par_chunks(4).all(|p| p[3] >= 1.0 - 0.5 / levels as f32);
    let src_mask = src.mask.as_ref().filter(|m| m.enabled).map(mask_coverage);
    let plain = opaque
        && src.opacity >= 100.0
        && matches!(src.blend, BlendMode::Normal | BlendMode::PassThrough)
        && src_mask.is_none()
        && src.blend_if.is_none();
    let (opacity, blend, blend_if, clip) = (src.opacity, src.blend, src.blend_if.clone(), src.clip);

    let (low_img, high_img) = bands(&rendered.image, sigma, levels, opaque);

    // the group mask carries the source's coverage when Low and High (opaque) cannot
    let group_mask = (!opaque || src_mask.is_some()).then(|| {
        let (cw, ch) = (doc.width, doc.height);
        let nf = levels as f32;
        let (rx, ry, rw, rh) =
            (rendered.x as i64, rendered.y as i64, rendered.image.width as i64, rendered.image.height as i64);
        Mask::from_fn(cw, ch, |x, y| {
            let (lx, ly) = (x as i64 - rx, y as i64 - ry);
            if lx < 0 || ly < 0 || lx >= rw || ly >= rh {
                return 0.0;
            }
            let a = rendered.image.data[((ly * rw + lx) * 4 + 3) as usize];
            let m = src_mask.as_ref().map(|m| m.get(x, y)).unwrap_or(1.0);
            ((a * m).clamp(0.0, 1.0) * nf).round() / nf
        })
    });

    let gname = a.str("name").filter(|s| !s.trim().is_empty()).unwrap_or_else(|| DEFAULT_GROUP.to_string());
    let mut group = doc.new_layer(&gname, LayerKind::Group { children: Vec::new() });
    group.blend = BlendMode::Normal;
    group.clip = clip;
    if !plain {
        group.opacity = opacity;
        group.blend = if blend == BlendMode::PassThrough { BlendMode::Normal } else { blend };
        group.blend_if = blend_if;
    }
    let has_group_mask = group_mask.is_some();
    group.mask = group_mask.map(LayerMask::new);
    let group_name = group.name.clone();
    let gpath = doc.insert_above(Some(&path), group);
    let mut low = doc.new_layer("Low", LayerKind::Raster { image: low_img });
    (low.x, low.y) = (rendered.x, rendered.y);
    let (low_id, low_name) = (low.id.clone(), low.name.clone());
    doc.layer_mut(&gpath).children_mut().expect("group").push(low);
    let mut high = doc.new_layer("High", LayerKind::Raster { image: high_img });
    (high.x, high.y) = (rendered.x, rendered.y);
    high.blend = BlendMode::LinearLight;
    let high_name = high.name.clone();
    doc.layer_mut(&gpath).children_mut().expect("group").push(high);
    if !plain {
        doc.layer_mut(&path).visible = false;
    }
    doc.active = Some(low_id);

    let max_error = reconstruction_error(doc, &gpath, &rendered, src_mask.as_ref(), levels)?;
    Ok(json!({
        "layer": src_name,
        "group": group_name,
        "low": low_name,
        "high": high_name,
        "radius": sigma,
        "reconstruction": {"max_error_levels": max_error, "depth": doc.depth},
        "group_mask": has_group_mask,
        "source_hidden": !plain,
        "active": low_name,
    }))
}

/// Largest per-channel difference, in levels, between the group as the compositor draws it
/// (its isolated content times its mask) and the rendered source times its layer mask
/// coverage `src_cov`, premultiplied so fully transparent pixels do not count.
fn reconstruction_error(
    doc: &Document,
    gpath: &[usize],
    src: &Rendered,
    src_cov: Option<&Mask>,
    levels: u32,
) -> Result<f64> {
    let g = doc.layer(gpath);
    let Some(out) = compose::render_layer(doc, g)? else { return Ok(0.0) };
    let out = out.to_canvas(doc.width, doc.height);
    let mask = g.mask.as_ref().map(|m| &m.mask);
    let (cw, ch) = (doc.width as i64, doc.height as i64);
    let (sw, sh) = (src.image.width as i64, src.image.height as i64);
    let x0 = (src.x as i64).max(0);
    let x1 = (src.x as i64 + sw).min(cw);
    let rows: Vec<f64> = ((src.y as i64).max(0)..(src.y as i64 + sh).min(ch))
        .into_par_iter()
        .map(|y| {
            let mut worst = 0f64;
            for x in x0..x1 {
                let si = (((y - src.y as i64) * sw + (x - src.x as i64)) * 4) as usize;
                let s = &src.image.data[si..si + 4];
                let oi = ((y * cw + x) * 4) as usize;
                let o = &out.data[oi..oi + 4];
                let oa = o[3] * mask.map(|m| m.get(x as u32, y as u32)).unwrap_or(1.0);
                let sa = s[3] * src_cov.map(|m| m.get(x as u32, y as u32)).unwrap_or(1.0);
                worst = worst.max((oa - sa).abs() as f64);
                for c in 0..3 {
                    worst = worst.max((o[c] * oa - s[c] * sa).abs() as f64);
                }
            }
            worst
        })
        .collect();
    let worst = rows.into_iter().fold(0.0, f64::max);
    Ok((worst * levels as f64 * 1e4).round() / 1e4)
}

// ---------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::ctx::{Globals, NoIo, NoProgress};
    use pixelox_core::testing::{self, find, raster, run};
    use pixelox_core::Runner;

    fn op() -> OpSpec {
        find(&crate::ops(), "retouch frequency-split")
    }

    /// Skin-like test image: smooth shading plus fine deterministic texture, quantized to
    /// the storage depth like every raster the executor hands an op.
    fn textured(w: u32, h: u32, levels: f32) -> Image {
        Image::from_fn(w, h, |x, y| {
            let n = (x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(2_246_822_519)).wrapping_mul(3_266_489_917);
            let t = ((n >> 8) % 1000) as f32 / 1000.0 - 0.5;
            let shade = 0.45 + 0.3 * (x as f32 / w as f32) + 0.1 * ((y as f32) * 0.05).sin();
            let q = |v: f32| (v.clamp(0.0, 1.0) * levels).round() / levels;
            [q(shade + 0.08 * t), q(shade * 0.75 + 0.06 * t), q(shade * 0.6 + 0.05 * t), 1.0]
        })
    }

    /// High-frequency energy: mean absolute Laplacian of the red channel.
    fn detail(img: &Image) -> f64 {
        let (w, h) = (img.width, img.height);
        let mut s = 0.0;
        let mut n = 0.0;
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let c = img.get(x, y)[0] * 4.0;
                let l = img.get(x - 1, y)[0] + img.get(x + 1, y)[0] + img.get(x, y - 1)[0] + img.get(x, y + 1)[0];
                s += (l - c).abs() as f64;
                n += 1.0;
            }
        }
        s / n
    }

    fn quantize(img: &mut Image, levels: f32) {
        img.data.iter_mut().for_each(|v| *v = (v.clamp(0.0, 1.0) * levels).round() / levels);
    }

    #[test]
    fn reconstructs_at_8_and_16_bit() {
        for depth in [8u8, 16] {
            let levels = if depth == 16 { 65535.0 } else { 255.0 };
            let src = textured(80, 60, levels);
            let mut doc = testing::doc_from(src.clone());
            doc.depth = depth;
            let before = compose::composite(&doc).unwrap();
            let v = run(&op(), &[("radius", "3")], &mut doc).unwrap();
            assert_eq!(v["low"], "Low");
            assert_eq!(v["high"], "High");
            assert_eq!(v["source_hidden"], false);
            assert_eq!(v["group_mask"], false);
            let err = v["reconstruction"]["max_error_levels"].as_f64().unwrap();
            assert!(err < 0.01, "{depth}-bit reported error {err} levels");
            // what the executor stores (a no-op: the bands are already on the grid)
            doc.quantize_to_storage();
            let after = compose::composite(&doc).unwrap();
            let worst = testing::max_diff(&before, &after) * levels;
            assert!(worst < 0.01, "{depth}-bit composite differs by {worst} levels");
            // and exported at the document depth it is bit-identical
            let enc =
                |i: &Image| if depth == 16 { i.to_rgba16() } else { i.to_rgba8().into_iter().map(u16::from).collect() };
            assert_eq!(enc(&before), enc(&after), "{depth}-bit export differs");
            // the layer structure: group above the source with Low (normal) under High (linear light)
            let g = doc.layer(&doc.resolve("Frequency Separation").unwrap());
            assert_eq!(g.blend, BlendMode::Normal);
            let ch = g.children().unwrap();
            assert_eq!((ch[0].name.as_str(), ch[1].name.as_str()), ("Low", "High"));
            assert_eq!((ch[0].blend, ch[1].blend), (BlendMode::Normal, BlendMode::LinearLight));
            assert_eq!(doc.layers[0].name, "Background");
            assert!(doc.layers[0].visible);
            assert_eq!(doc.layer(&doc.resolve("@active").unwrap()).name, "Low");
        }
    }

    #[test]
    fn unrounded_sources_stay_within_one_level() {
        // a source off the 8-bit grid (a live layer's rendering, say) still reconstructs to
        // within half a level, from rounding the source itself
        let src = Image::from_fn(40, 30, |x, y| {
            let v = 0.3 + 0.4 * ((x as f32 * 0.37).sin() * (y as f32 * 0.23).cos()).abs();
            [v, v * 0.8, v * 0.7, 1.0]
        });
        let mut doc = testing::doc_from(src);
        let v = run(&op(), &[("radius", "2")], &mut doc).unwrap();
        let err = v["reconstruction"]["max_error_levels"].as_f64().unwrap();
        assert!(err <= 0.5001, "error {err}");
    }

    #[test]
    fn low_is_smooth_and_high_holds_the_detail() {
        let src = textured(96, 64, 255.0);
        let mut doc = testing::doc_from(src.clone());
        run(&op(), &[("radius", "3")], &mut doc).unwrap();
        let low = raster(&doc, "Low").clone();
        let high = raster(&doc, "High").clone();
        // Low is the blur to within one level, so it has lost the fine texture
        let blur = kern::gaussian_image(&src, 3.0);
        assert!(testing::max_diff(&low, &blur) <= 1.0 / 255.0 + 1e-5, "Low is not the blur");
        assert!(detail(&low) < 0.2 * detail(&src), "Low keeps detail: {} vs {}", detail(&low), detail(&src));
        // High is centered on 0.5 and carries (source − Low)/2
        let mean = testing::mean(&high, 0);
        assert!((mean - 0.5).abs() < 0.01, "High mean {mean}");
        assert!(detail(&high) > 0.4 * detail(&src), "High lost detail");
        for (s, (l, hp)) in src.data.chunks(4).zip(low.data.chunks(4).zip(high.data.chunks(4))) {
            for c in 0..3 {
                assert!((hp[c] - (0.5 + (s[c] - l[c]) / 2.0)).abs() <= 0.5 / 255.0 + 1e-5);
            }
        }
    }

    #[test]
    fn smoothing_low_keeps_the_texture() {
        let src = textured(96, 64, 255.0);
        let mut doc = testing::doc_from(src.clone());
        run(&op(), &[("radius", "2")], &mut doc).unwrap();
        // the active layer is Low, so a following smoothing command hits it
        let gauss = find(&crate::ops(), "filter gaussian");
        run(&gauss, &[("layer", "@active"), ("sigma", "8")], &mut doc).unwrap();
        let out = compose::composite(&doc).unwrap();
        let fine = |i: &Image| {
            let b = kern::gaussian_image(i, 2.0);
            Image { width: i.width, height: i.height, data: i.data.iter().zip(&b.data).map(|(a, b)| a - b).collect() }
        };
        let (fs, fo) = (fine(&src), fine(&out));
        // the fine band is what it was (texture intact) away from the borders...
        let mut worst = 0f32;
        for y in 12..52 {
            for x in 12..84 {
                worst = worst.max((fs.get(x, y)[0] - fo.get(x, y)[0]).abs());
            }
        }
        assert!(worst < 0.02, "texture changed by {worst}");
        assert!(detail(&out) > 0.8 * detail(&src));
        // ...while the coarse tone was smoothed
        let coarse = |i: &Image| kern::gaussian_image(i, 2.0);
        assert!(testing::max_diff(&coarse(&out), &coarse(&src)) > 0.01, "tone did not change");
    }

    #[test]
    fn names_are_unique_and_radius_auto_is_recorded() {
        let spec = op();
        let src = textured(64, 48, 255.0);
        let mut doc = testing::doc_from(src);
        let a = testing::args(&spec, &[]).unwrap();
        let mut ctx = OpCtx::new(spec.name, vec![], Globals::default(), &NoIo, &NoProgress);
        let Runner::Doc(f) = spec.runner else { panic!() };
        let v = f(&a, &mut doc, &mut ctx).unwrap();
        assert_eq!(v["radius"], 2.0, "auto radius is at least 2");
        assert_eq!(ctx.resolved["radius"], "2");
        // splitting again (the active layer, Low, by default) makes new unique names
        let v = run(&spec, &[("name", "Low")], &mut doc).unwrap();
        assert_eq!(v["layer"], "Low");
        assert_eq!(v["group"], "Low 2");
        assert_eq!(v["low"], "Low 3");
        assert_eq!(v["high"], "High 2");
        assert_eq!(doc.layer(&doc.resolve("@active").unwrap()).name, "Low 3");
        let names: Vec<String> = doc.walk().iter().map(|(_, l)| l.name.clone()).collect();
        let set: std::collections::BTreeSet<&String> = names.iter().collect();
        assert_eq!(set.len(), names.len(), "{names:?}");
        // auto radius scales with the image: 0.15% of the long side
        let mut big = testing::doc_from(testing::solid(4000, 2, Rgba::WHITE));
        assert_eq!(run(&spec, &[], &mut big).unwrap()["radius"], 6.0);
    }

    #[test]
    fn replay_and_thread_count_give_identical_bands() {
        let src = textured(120, 90, 255.0);
        let go = |threads: usize, pairs: &[(&str, &str)]| {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
            let mut doc = testing::doc_from(src.clone());
            pool.install(|| run(&op(), pairs, &mut doc)).unwrap();
            (raster(&doc, "Low").clone(), raster(&doc, "High").clone())
        };
        let a = go(1, &[]);
        let b = go(3, &[]);
        let c = go(2, &[("radius", "2")]);
        for (x, y) in [(&a, &b), (&a, &c)] {
            assert!(x.0.data.iter().zip(&y.0.data).all(|(p, q)| p.to_bits() == q.to_bits()));
            assert!(x.1.data.iter().zip(&y.1.data).all(|(p, q)| p.to_bits() == q.to_bits()));
        }
    }

    #[test]
    fn transparent_sources_keep_the_composite() {
        // a soft-edged disc with a hole, over a background layer
        let w = 48;
        let mut fg = Image::from_fn(w, w, |x, y| {
            let d = ((x as f32 - 24.0).powi(2) + (y as f32 - 24.0).powi(2)).sqrt();
            let a = (18.0 - d).clamp(0.0, 1.0) * if d < 4.0 { 0.0 } else { 1.0 } * (0.6 + 0.4 * (x as f32 / 48.0));
            [0.9 - 0.01 * (x % 5) as f32, 0.4, 0.2 + 0.01 * (y % 3) as f32, a]
        });
        quantize(&mut fg, 255.0);
        let bg = testing::solid(w, w, Rgba::new(0.1, 0.5, 0.8, 1.0));
        let mut doc = testing::doc_layers(w, w, vec![("Back", bg), ("Disc", fg)]);
        doc.quantize_to_storage();
        let before = compose::composite(&doc).unwrap();
        let v = run(&op(), &[("layer", "Disc"), ("radius", "2")], &mut doc).unwrap();
        assert_eq!(v["source_hidden"], true);
        assert_eq!(v["group_mask"], true);
        assert!(v["reconstruction"]["max_error_levels"].as_f64().unwrap() < 0.01);
        assert!(!doc.layer(&doc.resolve("Disc").unwrap()).visible);
        doc.quantize_to_storage();
        let after = compose::composite(&doc).unwrap();
        assert!(testing::max_diff(&before, &after) * 255.0 < 0.01, "composite changed");
        // Low and High are opaque, so smoothing Low cannot pull in black from outside
        assert!(raster(&doc, "Low").is_opaque() && raster(&doc, "High").is_opaque());
        let lo = raster(&doc, "Low").get(0, 0);
        assert!(lo[0] > 0.5, "Low's color is extended past the edge: {lo:?}");

        // half opacity, multiply: the group takes the look over
        let mut fg2 = textured(32, 32, 255.0);
        quantize(&mut fg2, 255.0);
        let mut doc = testing::doc_layers(32, 32, vec![("Back", testing::gradient(32, 32)), ("Top", fg2)]);
        quantize(doc.layers[0].raster_mut().unwrap(), 255.0);
        let p = doc.resolve("Top").unwrap();
        doc.layer_mut(&p).opacity = 50.0;
        doc.layer_mut(&p).blend = BlendMode::Multiply;
        let before = compose::composite(&doc).unwrap();
        let v = run(&op(), &[("layer", "Top"), ("radius", "2")], &mut doc).unwrap();
        assert_eq!(v["source_hidden"], true);
        assert_eq!(v["group_mask"], false);
        let after = compose::composite(&doc).unwrap();
        assert!(testing::max_diff(&before, &after) * 255.0 < 0.01);

        // an opaque layer with a feathered layer mask: the group mask carries it
        let mut doc =
            testing::doc_layers(32, 32, vec![("Back", testing::gradient(32, 32)), ("Top", textured(32, 32, 255.0))]);
        quantize(doc.layers[0].raster_mut().unwrap(), 255.0);
        let p = doc.resolve("Top").unwrap();
        let mut lm = LayerMask::new(testing::half_mask(32, 32));
        lm.feather = 4.0;
        lm.density = 80.0;
        doc.layer_mut(&p).mask = Some(lm);
        let before = compose::composite(&doc).unwrap();
        let v = run(&op(), &[("layer", "Top"), ("radius", "2")], &mut doc).unwrap();
        assert_eq!((v["source_hidden"].clone(), v["group_mask"].clone()), (json!(true), json!(true)));
        assert!(v["reconstruction"]["max_error_levels"].as_f64().unwrap() <= 0.5001);
        doc.quantize_to_storage();
        let after = compose::composite(&doc).unwrap();
        assert!(testing::max_diff(&before, &after) * 255.0 <= 1.0, "masked composite changed");
    }

    #[test]
    fn edge_cases() {
        let spec = op();
        // 1×1 and fully transparent layers
        let mut doc = testing::doc_from(testing::solid(1, 1, Rgba::new(0.8, 0.2, 0.2, 1.0)));
        let v = run(&spec, &[], &mut doc).unwrap();
        assert!(v["reconstruction"]["max_error_levels"].as_f64().unwrap() < 0.01);
        let mut doc = testing::doc_from(Image::new(4, 4));
        run(&spec, &[("radius", "1")], &mut doc).unwrap();
        assert!(compose::composite(&doc).unwrap().alpha_bbox(0.0).is_none(), "transparent stays transparent");
        // several layers: default to the active one; adjustment layers have no pixels
        let mut doc = testing::doc_layers(8, 8, vec![("A", testing::gradient(8, 8)), ("B", testing::checker(8, 8))]);
        assert_eq!(run(&spec, &[], &mut doc).unwrap()["layer"], "B");
        let adj = doc.new_layer("Curves", LayerKind::Adjustment { op: "levels".into(), params: Map::new() });
        doc.insert_above(None, adj);
        assert_eq!(run(&spec, &[("layer", "Curves")], &mut doc).unwrap_err().code, "PX2303");
        // locked sources are only read
        let mut doc = testing::doc_from(testing::gradient(8, 8));
        doc.layers[0].locked = true;
        run(&spec, &[], &mut doc).unwrap();
        assert_eq!(doc.layers.len(), 2);
    }
}
