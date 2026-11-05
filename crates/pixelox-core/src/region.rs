//! Helpers shared by ops that change a raster layer's pixels: resolve `--layer`, honor
//! locks, and limit the effect with `--mask` / `--invert-mask`.

use crate::ctx::OpCtx;
use crate::doc::{Document, LayerKind, LayerPath, SmartFilter};
use crate::error::{Error, Result};
use crate::image::{Image, Mask};
use crate::op::Args;
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
/// On a placed layer, `filter` and `adjust` commands are recorded as smart filters
/// instead (non-destructive, re-run whenever the layer renders); other commands fail with
/// PX2303 and a hint to rasterize.
pub fn apply_to_raster(
    args: &Args,
    doc: &mut Document,
    ctx: &mut OpCtx,
    f: impl FnOnce(&Image, &mut OpCtx) -> Result<Image>,
) -> Result<Value> {
    let path = target_layer(args, doc)?;
    if matches!(doc.layer(&path).content, LayerKind::Placed { .. })
        && crate::compose::smart_filter_eligible(&ctx.id, args)
    {
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

/// Record the current command as a smart filter on the placed layer at `path`.
fn add_smart_filter(args: &Args, doc: &mut Document, ctx: &mut OpCtx, path: &LayerPath) -> Result<Value> {
    let name = doc.layer(path).name.clone();
    if args.str("mask").is_some() {
        return Err(Error::new("PX2303", format!("smart filters on placed layer '{name}' cannot take --mask"))
            .with_hint(format!(
                "rasterize first (pixelox layer rasterize <doc> --layer '{name}' --in-place), or mask the layer itself"
            ))
            .with_context("layer", name));
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
