//! Helpers shared by ops that change a raster layer's pixels: resolve `--layer`, honor
//! locks, and limit the effect with `--mask` / `--invert-mask`.

use crate::ctx::OpCtx;
use crate::doc::{Document, LayerKind, LayerPath, SmartFilter};
use crate::error::{Error, Result};
use crate::image::{Image, Mask};
use crate::op::Args;
use crate::units::RectI;
use rayon::prelude::*;
use serde_json::{json, Value};

/// Resolve `--layer` (default: the only layer) and check that it is unlocked.
pub fn target_layer(args: &Args, doc: &Document) -> Result<LayerPath> {
    let p = doc.resolve_or_only(args.raw("layer"))?;
    doc.layer(&p).check_unlocked()?;
    Ok(p)
}

/// Mix `processed` into `orig` by mask coverage. `mask` is canvas-sized; the image sits
/// at (`ox`, `oy`) on the canvas. The mix is premultiplied, so a soft mask edge over
/// transparent pixels never darkens colors.
pub fn mix_by_mask(orig: &Image, processed: &Image, mask: &Mask, ox: i32, oy: i32) -> Image {
    let mut out = orig.clone();
    let w = orig.width as usize;
    out.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let k = mask.at(x as i64 + ox as i64, y as i64 + oy as i64);
            if k <= 0.0 {
                continue;
            }
            let i = (y * w + x) * 4;
            let o = &mut row[x * 4..x * 4 + 4];
            if k >= 1.0 {
                o.copy_from_slice(&processed.data[i..i + 4]);
                continue;
            }
            let p = &processed.data[i..i + 4];
            let a = o[3] + (p[3] - o[3]) * k;
            if a <= 1e-7 {
                o.copy_from_slice(&[0.0; 4]);
                continue;
            }
            for c in 0..3 {
                o[c] = (o[c] * o[3] + (p[c] * p[3] - o[c] * o[3]) * k) / a;
            }
            o[3] = a;
        }
    });
    out
}

/// The common shape of a pixel op: resolve the raster layer, run `f` on its pixels, and
/// keep the result only inside `--mask` (all of it when no mask is given). `f` must return
/// an image of the same size. Returns `{"layer": name}` for the envelope.
///
/// On a placed, text, or shape layer, `filter` and `adjust` commands are recorded as smart
/// filters instead (non-destructive, re-run whenever the layer renders); other commands
/// fail with PX2303 and a hint to rasterize.
pub fn apply_to_raster(
    args: &Args,
    doc: &mut Document,
    ctx: &mut OpCtx,
    f: impl FnOnce(&Image, &mut OpCtx) -> Result<Image>,
) -> Result<Value> {
    let path = target_layer(args, doc)?;
    if takes_smart_filters(&doc.layer(&path).content) && crate::compose::smart_filter_eligible(&ctx.id, args) {
        return add_smart_filter(args, doc, ctx, &path);
    }
    let mask = ctx.mask_arg(args, doc)?;
    let layer = doc.layer(&path);
    if !layer.visible {
        ctx.warn("PXW2001", format!("layer '{}' is hidden", layer.name));
    }
    let (ox, oy) = (layer.x, layer.y);
    let name = layer.name.clone();
    let orig = layer.raster()?;
    let processed = f(orig, ctx)?;
    debug_assert_eq!((processed.width, processed.height), (orig.width, orig.height));
    let result = match &mask {
        Some(m) => mix_by_mask(orig, &processed, m, ox, oy),
        None => processed,
    };
    *doc.layer_mut(&path).raster_mut()? = result;
    Ok(json!({"layer": name}))
}

/// How far a spatial filter can carry content past where it was, in px per side. A blur
/// of a shape on a small layer needs this much room around the shape, or it is cut off
/// at the layer's edge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Reach {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl Reach {
    /// No growth: the filter keeps content where it is.
    pub const NONE: Reach = Reach { left: 0, top: 0, right: 0, bottom: 0 };

    fn px(v: f64) -> u32 {
        if v.is_finite() && v > 0.0 {
            v.ceil().min(1_000_000.0) as u32
        } else {
            0
        }
    }

    /// The same reach on every side.
    pub fn uniform(px: f64) -> Reach {
        Reach::xy(px, px)
    }

    /// `x` px to the left and right, `y` px up and down.
    pub fn xy(x: f64, y: f64) -> Reach {
        let (x, y) = (Reach::px(x), Reach::px(y));
        Reach { left: x, top: y, right: x, bottom: y }
    }

    pub fn is_none(&self) -> bool {
        *self == Reach::NONE
    }
}

/// True when `filter`/`adjust` commands on this layer kind are recorded as smart filters
/// (placed, text, and shape layers keep their source live).
pub fn takes_smart_filters(kind: &LayerKind) -> bool {
    matches!(kind, LayerKind::Placed { .. } | LayerKind::Text { .. } | LayerKind::Shape { .. })
}

/// [`apply_to_raster`] for filters that spread content (blurs, pixelate): before `f`
/// runs, the layer's raster grows by `reach` on every side that lies inside the canvas,
/// so a blurred shape on a small layer is not cut off at the layer's edge; afterwards the
/// growth is trimmed back to the pixels that are not transparent. A layer that covers the
/// canvas does not grow (nothing outside the canvas is visible), and a side that already
/// reaches the canvas edge keeps its edge behavior. `f` receives the (possibly grown)
/// image and the canvas position of its top-left pixel, and must return an image of the
/// same size.
pub fn apply_to_raster_grow(
    args: &Args,
    doc: &mut Document,
    ctx: &mut OpCtx,
    reach: Reach,
    f: impl FnOnce(&Image, (i32, i32), &mut OpCtx) -> Result<Image>,
) -> Result<Value> {
    let path = target_layer(args, doc)?;
    if takes_smart_filters(&doc.layer(&path).content) && crate::compose::smart_filter_eligible(&ctx.id, args) {
        return add_smart_filter(args, doc, ctx, &path);
    }
    let (cw, ch) = (doc.width as i64, doc.height as i64);
    let (pads, offset) = {
        let layer = doc.layer(&path);
        let img = layer.raster()?;
        let pads = grow_pads(reach, layer.x as i64, layer.y as i64, img.width as i64, img.height as i64, cw, ch);
        (pads, (layer.x, layer.y))
    };
    if pads == [0; 4] {
        return apply_to_raster(args, doc, ctx, |img, ctx| f(img, offset, ctx));
    }
    let mask = ctx.mask_arg(args, doc)?;
    let layer = doc.layer(&path);
    if !layer.visible {
        ctx.warn("PXW2001", format!("layer '{}' is hidden", layer.name));
    }
    let name = layer.name.clone();
    let (lx, ly) = (layer.x as i64, layer.y as i64);
    let orig = layer.raster()?;
    let (w, h) = (orig.width as i64, orig.height as i64);
    let [pl, pt, pr, pb] = pads;
    // sides that already reach the canvas edge keep their old edge behavior: an opaque
    // layer is clamped there, so replicate its edge pixels for the filter to read (at most
    // REP_MAX px: beyond that only extreme kernels reach, and memory would grow quadratically)
    const REP_MAX: u32 = 1024;
    let opaque = orig.is_opaque();
    let rep = |touches: bool, r: u32| if opaque && touches { r.min(REP_MAX) as i64 } else { 0 };
    let (rl, rt) = (rep(lx <= 0, reach.left), rep(ly <= 0, reach.top));
    let (rr, rb) = (rep(lx + w >= cw, reach.right), rep(ly + h >= ch, reach.bottom));
    let (gw, gh) = (rl + pl + w + pr + rr, rt + pt + h + pb + rb);
    let map = |v: i64, rep_lo: i64, pad_lo: i64, n: i64, pad_hi: i64| -> Option<i64> {
        if v < rep_lo {
            Some(0)
        } else if v < rep_lo + pad_lo {
            None
        } else if v < rep_lo + pad_lo + n {
            Some(v - rep_lo - pad_lo)
        } else if v < rep_lo + pad_lo + n + pad_hi {
            None
        } else {
            Some(n - 1)
        }
    };
    let grown = Image::from_fn(gw as u32, gh as u32, |x, y| {
        match (map(x as i64, rl, pl, w, pr), map(y as i64, rt, pt, h, pb)) {
            (Some(sx), Some(sy)) => orig.get(sx as u32, sy as u32),
            _ => [0.0; 4],
        }
    });
    let (gx, gy) = (lx - pl - rl, ly - pt - rt);
    let processed = f(&grown, (gx as i32, gy as i32), ctx)?;
    if (processed.width, processed.height) != (grown.width, grown.height) {
        return Err(Error::internal(format!("{} changed the layer size", ctx.id)));
    }
    drop(grown);
    // the replicated margins lie outside the canvas: drop them
    let inner = RectI::new(rl as i32, rt as i32, (pl + w + pr) as u32, (pt + h + pb) as u32);
    let processed = processed.crop(inner);
    let (qx, qy) = (lx - pl, ly - pt);
    let result = match &mask {
        Some(m) => {
            let before = orig.crop(RectI::new(-(pl as i32), -(pt as i32), inner.w, inner.h));
            mix_by_mask(&before, &processed, m, qx as i32, qy as i32)
        }
        None => processed,
    };
    // trim the growth to what is visible at the storage depth; the original extent stays
    let threshold = if doc.depth == 16 { 0.5 / 65535.0 } else { 0.5 / 255.0 };
    let keep = RectI::new(pl as i32, pt as i32, w as u32, h as u32);
    let keep = match result.alpha_bbox(threshold) {
        Some(b) => keep.union(&b),
        None => keep,
    };
    let result = if keep == RectI::new(0, 0, result.width, result.height) { result } else { result.crop(keep) };
    let grew =
        [pl as i32 - keep.x, pt as i32 - keep.y, keep.right() - (pl + w) as i32, keep.bottom() - (pt + h) as i32];
    let l = doc.layer_mut(&path);
    *l.raster_mut()? = result;
    l.x = qx as i32 + keep.x;
    l.y = qy as i32 + keep.y;
    let mut v = json!({"layer": name});
    if grew.iter().any(|&g| g > 0) {
        v["grown"] = json!(grew);
    }
    Ok(v)
}

/// Growth per side (left, top, right, bottom) of a `w`×`h` raster at (`x`, `y`): `reach`,
/// limited to the canvas; 0 on sides that already reach or cross the canvas edge.
fn grow_pads(reach: Reach, x: i64, y: i64, w: i64, h: i64, cw: i64, ch: i64) -> [i64; 4] {
    if reach.is_none() || w == 0 || h == 0 {
        return [0; 4];
    }
    let lo = |pos: i64, r: u32| if pos <= 0 { 0 } else { (r as i64).min(pos) };
    let hi = |end: i64, extent: i64, r: u32| if end >= extent { 0 } else { (r as i64).min(extent - end) };
    [lo(x, reach.left), lo(y, reach.top), hi(x + w, cw, reach.right), hi(y + h, ch, reach.bottom)]
}

/// Record the current command as a smart filter on the placed, text, or shape layer at
/// `path`.
fn add_smart_filter(args: &Args, doc: &mut Document, ctx: &mut OpCtx, path: &LayerPath) -> Result<Value> {
    let name = doc.layer(path).name.clone();
    if args.str("mask").is_some() {
        let kind = doc.layer(path).kind_name();
        return Err(Error::new("PX2303", format!("smart filters on {kind} layer '{name}' cannot take --mask"))
            .with_hint(format!(
                "mask the layer itself (pixelox mask add <doc> --layer '{name}' --from <mask> --in-place), or rasterize first (pixelox layer rasterize <doc> --layer '{name}' --in-place)"
            ))
            .with_context("layer", name)
            .with_context("kind", kind));
    }
    let mut params = args.to_json();
    for k in ["layer", "mask", "invert-mask", "invert_mask"] {
        params.remove(k);
    }
    let filter =
        SmartFilter { op: ctx.id.clone(), enabled: true, params, seed: ctx.globals.seed, op_index: ctx.op_index };
    // render once now so a bad filter fails here, not at export
    let layer = doc.layer(path).clone();
    let mut probe = layer.clone();
    probe.smart_filters.push(filter.clone());
    crate::compose::render_layer(doc, &probe)?;
    let l = doc.layer_mut(path);
    l.smart_filters.push(filter);
    Ok(json!({"layer": name, "smart_filter": ctx.id, "smart_filters": l.smart_filters.len()}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Rgba;

    #[test]
    fn mask_limits_and_offsets() {
        let orig = Image::filled(4, 1, Rgba::BLACK);
        let proc_ = Image::filled(4, 1, Rgba::WHITE);
        let mask = Mask::from_fn(6, 1, |x, _| if x >= 3 { 1.0 } else { 0.0 });
        // the layer sits at x = 2: its pixels 1.. are inside the mask
        let out = mix_by_mask(&orig, &proc_, &mask, 2, 0);
        assert_eq!(out.get(0, 0)[0], 0.0);
        assert_eq!(out.get(1, 0)[0], 1.0);
    }

    #[test]
    fn soft_edges_over_transparency_keep_color() {
        // half-strength mask painting white onto a transparent pixel: white at half alpha,
        // not gray (the straight-alpha mix would give 0.5 gray)
        let orig = Image::new(1, 1);
        let proc_ = Image::filled(1, 1, Rgba::WHITE);
        let out = mix_by_mask(&orig, &proc_, &Mask::filled(1, 1, 0.5), 0, 0);
        assert_eq!(out.get(0, 0), [1.0, 1.0, 1.0, 0.5]);
    }
}
