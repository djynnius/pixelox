//! The compositor: renders each layer's content and blends the stack (PRD §7.3).
//!
//! Layer kinds whose rendering lives in other crates plug in through renderer hooks,
//! installed once at startup (`pixelox-cli` installs all of them):
//! text (`pixelox-text`), shapes (`pixelox-vector`), adjustments (`pixelox-adjust`), and
//! layer styles (`pixelox-fx`). The order of operations follows Photoshop:
//! content → fx → clipped layers → layer mask → opacity → blend mode onto the backdrop.

use crate::blend::{composite_pixel, BlendMode};
use crate::doc::{Document, FxSpec, Layer, LayerKind};
use crate::error::{Error, Result};
use crate::image::{Image, Mask, Rendered};
use crate::kernels;
use crate::units::RectI;
use rayon::prelude::*;
use serde_json::{Map, Value};
use std::borrow::Cow;
use std::sync::RwLock;

/// Renders a text or shape layer to pixels placed on the canvas (None when empty).
pub type LayerRenderFn = fn(&Layer, &Document) -> Result<Option<Rendered>>;
/// Applies a named adjustment (`levels`, `curves`, ...) with params to an image in place.
pub type AdjustFn = fn(&str, &Map<String, Value>, &mut Image) -> Result<()>;
/// Applies a layer's style stack to its rendered content (may grow the bounds).
pub type FxFn = fn(&[FxSpec], Rendered, &Document) -> Result<Rendered>;

#[derive(Default, Clone, Copy)]
struct Hooks {
    text: Option<LayerRenderFn>,
    shape: Option<LayerRenderFn>,
    adjust: Option<AdjustFn>,
    fx: Option<FxFn>,
}

static HOOKS: RwLock<Hooks> = RwLock::new(Hooks { text: None, shape: None, adjust: None, fx: None });

fn hooks() -> Hooks {
    *HOOKS.read().unwrap_or_else(|e| e.into_inner())
}

pub fn set_text_renderer(f: LayerRenderFn) {
    HOOKS.write().unwrap_or_else(|e| e.into_inner()).text = Some(f);
}
pub fn set_shape_renderer(f: LayerRenderFn) {
    HOOKS.write().unwrap_or_else(|e| e.into_inner()).shape = Some(f);
}
pub fn set_adjuster(f: AdjustFn) {
    HOOKS.write().unwrap_or_else(|e| e.into_inner()).adjust = Some(f);
}
pub fn set_fx_renderer(f: FxFn) {
    HOOKS.write().unwrap_or_else(|e| e.into_inner()).fx = Some(f);
}

/// Decodes embedded asset bytes (installed by `pixelox-io`).
pub type DecodeFn = fn(&[u8], Option<&str>) -> Result<Image>;

static DECODER: RwLock<Option<DecodeFn>> = RwLock::new(None);
static ASSET_CACHE: std::sync::Mutex<Vec<(String, std::sync::Arc<Image>)>> = std::sync::Mutex::new(Vec::new());

pub fn set_decoder(f: DecodeFn) {
    *DECODER.write().unwrap_or_else(|e| e.into_inner()) = Some(f);
}

/// Decode an embedded asset (`doc.assets[key]`) to pixels, with a small cache. Used by
/// renderers that need images stored in the document (pattern overlays).
pub fn decode_asset(doc: &Document, key: &str) -> Result<std::sync::Arc<Image>> {
    if let Some((_, img)) = ASSET_CACHE.lock().unwrap_or_else(|e| e.into_inner()).iter().find(|(k, _)| k == key) {
        return Ok(img.clone());
    }
    let asset =
        doc.assets.get(key).ok_or_else(|| Error::new("PX1210", format!("asset {key} is missing from the document")))?;
    let f = (*DECODER.read().unwrap_or_else(|e| e.into_inner())).ok_or_else(|| missing("image decoder"))?;
    let img = std::sync::Arc::new(f(&asset.bytes, Some(&asset.ext))?);
    let mut cache = ASSET_CACHE.lock().unwrap_or_else(|e| e.into_inner());
    if cache.len() >= 16 {
        cache.remove(0);
    }
    cache.push((key.to_string(), img.clone()));
    Ok(img)
}

/// Which renderers are installed (for `doctor`).
pub fn installed_renderers() -> Vec<&'static str> {
    let h = hooks();
    let mut v = Vec::new();
    if h.text.is_some() {
        v.push("text");
    }
    if h.shape.is_some() {
        v.push("shape");
    }
    if h.adjust.is_some() {
        v.push("adjust");
    }
    if h.fx.is_some() {
        v.push("fx");
    }
    v
}

fn missing(what: &str) -> Error {
    Error::internal(format!("no {what} renderer is installed in this build"))
}

/// Apply a named adjustment through the installed adjuster.
pub fn apply_adjustment(op: &str, params: &Map<String, Value>, img: &mut Image) -> Result<()> {
    let f = hooks().adjust.ok_or_else(|| missing("adjustment"))?;
    f(op, params, img)
}

/// A layer's own pixels (before mask, opacity, and blending), with its fx applied.
/// `None` for adjustment layers and empty content.
pub fn content<'a>(doc: &Document, layer: &'a Layer) -> Result<Option<(Cow<'a, Image>, i32, i32)>> {
    let base: Option<(Cow<'a, Image>, i32, i32)> = match &layer.content {
        LayerKind::Raster { image } => {
            if image.is_empty() {
                None
            } else {
                Some((Cow::Borrowed(image), layer.x, layer.y))
            }
        }
        LayerKind::Text { .. } => {
            let f = hooks().text.ok_or_else(|| missing("text"))?;
            f(layer, doc)?.map(|r| (Cow::Owned(r.image), r.x, r.y))
        }
        LayerKind::Shape { .. } => {
            let f = hooks().shape.ok_or_else(|| missing("shape"))?;
            f(layer, doc)?.map(|r| (Cow::Owned(r.image), r.x, r.y))
        }
        LayerKind::Placed { source, rect, angle, .. } => match source {
            None => None,
            Some(src) => render_placed(src, rect, *angle).map(|r| (Cow::Owned(r.image), r.x, r.y)),
        },
        LayerKind::Group { children } => {
            let mut buf = Image::new(doc.width, doc.height);
            composite_layers(doc, children, &mut buf)?;
            Some((Cow::Owned(buf), 0, 0))
        }
        LayerKind::Adjustment { .. } => None,
    };
    let enabled_fx: Vec<FxSpec> = layer.fx.iter().filter(|f| f.enabled).cloned().collect();
    match (base, enabled_fx.is_empty()) {
        (Some(b), true) => Ok(Some(b)),
        (Some((img, x, y)), false) => {
            let f = hooks().fx.ok_or_else(|| missing("layer style (fx)"))?;
            let r = f(&enabled_fx, Rendered { image: img.into_owned(), x, y }, doc)?;
            Ok(Some((Cow::Owned(r.image), r.x, r.y)))
        }
        (None, _) => Ok(None),
    }
}

/// Owned variant of [`content`].
pub fn render_layer(doc: &Document, layer: &Layer) -> Result<Option<Rendered>> {
    Ok(content(doc, layer)?.map(|(i, x, y)| Rendered { image: i.into_owned(), x, y }))
}

/// Draw a placed source into its destination rectangle (rotated about the center).
pub fn render_placed(src: &Image, rect: &[f64; 4], angle: f64) -> Option<Rendered> {
    let [x, y, w, h] = *rect;
    if w < 0.5 || h < 0.5 || src.is_empty() {
        return None;
    }
    let (tw, th) = (w.round().max(1.0) as u32, h.round().max(1.0) as u32);
    let filter = kernels::Filter::auto(src.width.max(src.height), tw.max(th));
    let scaled = kernels::resize(src, tw, th, filter);
    if angle.abs() < 1e-9 {
        return Some(Rendered { image: scaled, x: x.round() as i32, y: y.round() as i32 });
    }
    // rotate about the rect center into an expanded box
    let (s, c) = angle.to_radians().sin_cos();
    let (cw, ch) = (tw as f64, th as f64);
    let bw = (cw * c.abs() + ch * s.abs()).ceil();
    let bh = (cw * s.abs() + ch * c.abs()).ceil();
    let (bwu, bhu) = (bw as u32, bh as u32);
    let out = Image::from_fn(bwu, bhu, |px, py| {
        let dx = px as f64 + 0.5 - bw / 2.0;
        let dy = py as f64 + 0.5 - bh / 2.0;
        // inverse rotation
        let sx = dx * c + dy * s + cw / 2.0;
        let sy = -dx * s + dy * c + ch / 2.0;
        scaled.sample_bilinear(sx, sy)
    });
    let cx = x + w / 2.0;
    let cy = y + h / 2.0;
    Some(Rendered { image: out, x: (cx - bw / 2.0).round() as i32, y: (cy - bh / 2.0).round() as i32 })
}

/// Bounds of a layer's visible content on the canvas (alpha > 0), None when empty.
pub fn layer_bounds(doc: &Document, layer: &Layer) -> Result<Option<RectI>> {
    if let LayerKind::Raster { image } = &layer.content {
        if layer.fx.is_empty() {
            return Ok(image.alpha_bbox(0.0).map(|r| RectI::new(r.x + layer.x, r.y + layer.y, r.w, r.h)));
        }
    }
    Ok(content(doc, layer)?.and_then(|(img, x, y)| img.alpha_bbox(0.0).map(|r| RectI::new(r.x + x, r.y + y, r.w, r.h))))
}

/// Flatten the whole document to one canvas-sized image.
pub fn composite(doc: &Document) -> Result<Image> {
    // fast path: one plain canvas-sized raster layer is its own composite
    if let [l] = doc.layers.as_slice() {
        if let LayerKind::Raster { image } = &l.content {
            let plain = l.visible
                && l.opacity >= 100.0
                && matches!(l.blend, BlendMode::Normal | BlendMode::PassThrough)
                && l.mask.as_ref().map(|m| !m.enabled).unwrap_or(true)
                && l.fx.iter().all(|f| !f.enabled)
                && !l.clip
                && (l.x, l.y, image.width, image.height) == (0, 0, doc.width, doc.height);
            if plain {
                return Ok(image.clone());
            }
        }
    }
    let mut canvas = Image::new(doc.width, doc.height);
    composite_layers(doc, &doc.layers, &mut canvas)?;
    Ok(canvas)
}

/// Composite only some layers (by id) — used by `export --layer` and `layer merge`.
pub fn composite_subset(doc: &Document, ids: &[String]) -> Result<Image> {
    fn keep(layers: &[Layer], ids: &[String]) -> Vec<Layer> {
        layers
            .iter()
            .filter_map(|l| {
                if ids.contains(&l.id) {
                    Some(l.clone())
                } else if let Some(ch) = l.children() {
                    let kept = keep(ch, ids);
                    if kept.is_empty() {
                        None
                    } else {
                        let mut g = l.clone();
                        *g.children_mut().unwrap() = kept;
                        Some(g)
                    }
                } else {
                    None
                }
            })
            .collect()
    }
    let layers = keep(&doc.layers, ids);
    let mut canvas = Image::new(doc.width, doc.height);
    composite_layers(doc, &layers, &mut canvas)?;
    Ok(canvas)
}

/// Composite a stack (bottom → top) onto `backdrop` (canvas-sized).
pub fn composite_layers(doc: &Document, layers: &[Layer], backdrop: &mut Image) -> Result<()> {
    let mut i = 0;
    while i < layers.len() {
        let base = &layers[i];
        let mut j = i + 1;
        while j < layers.len() && layers[j].clip {
            j += 1;
        }
        let clipped = &layers[i + 1..j];
        if base.visible {
            draw_layer(doc, base, clipped, backdrop)?;
        }
        i = j;
    }
    Ok(())
}

/// Per-pixel coverage multiplier from a layer's mask (None = all 1).
fn mask_factor(layer: &Layer) -> Option<Mask> {
    let lm = layer.mask.as_ref().filter(|m| m.enabled)?;
    let mut m = if lm.feather > 0.0 { kernels::blur_mask(&lm.mask, lm.feather as f64 / 2.0) } else { lm.mask.clone() };
    let d = lm.density / 100.0;
    m.data.par_iter_mut().for_each(|v| *v = 1.0 - d * (1.0 - *v));
    Some(m)
}

fn layer_seed(layer: &Layer) -> u64 {
    // FNV-1a of the layer id: dissolve noise is deterministic per layer
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in layer.id.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

#[inline]
fn hash01(seed: u64, x: u32, y: u32) -> f32 {
    let mut z = seed ^ ((x as u64) << 32 | y as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z >> 40) as f32 / (1u64 << 24) as f32
}

/// Blend `src` (canvas-sized, straight alpha) onto `dst` with opacity, mask, and mode.
pub fn blend_onto(dst: &mut Image, src: &Image, mode: BlendMode, opacity: f32, mask: Option<&Mask>, seed: u64) {
    let w = dst.width as usize;
    dst.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let si = (y * w + x) * 4;
            let mut a = src.data[si + 3] * opacity;
            if let Some(m) = mask {
                a *= m.data[y * w + x];
            }
            if a <= 0.0 {
                continue;
            }
            if mode == BlendMode::Dissolve {
                a = if hash01(seed, x as u32, y as u32) < a { 1.0 } else { 0.0 };
                if a == 0.0 {
                    continue;
                }
            }
            let p = [src.data[si], src.data[si + 1], src.data[si + 2], a];
            composite_pixel(mode, &mut row[x * 4..x * 4 + 4], p);
        }
    });
}

/// Mix `adjusted` into `dst` by `t·mask` (colors only; alpha kept).
fn mix_into(dst: &mut Image, adjusted: &Image, t: f32, mask: Option<&Mask>) {
    let w = dst.width as usize;
    dst.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let mut k = t;
            if let Some(m) = mask {
                k *= m.data[y * w + x];
            }
            if k <= 0.0 {
                continue;
            }
            let i = (y * w + x) * 4;
            for c in 0..3 {
                let d = &mut row[x * 4 + c];
                *d += (adjusted.data[i + c] - *d) * k;
            }
        }
    });
}

fn apply_adjustment_layer(layer: &Layer, target: &mut Image) -> Result<()> {
    if let LayerKind::Adjustment { op, params } = &layer.content {
        let mut adjusted = target.clone();
        apply_adjustment(op, params, &mut adjusted)?;
        let m = mask_factor(layer);
        mix_into(target, &adjusted, layer.opacity / 100.0, m.as_ref());
    }
    Ok(())
}

fn draw_layer(doc: &Document, base: &Layer, clipped: &[Layer], backdrop: &mut Image) -> Result<()> {
    let (w, h) = (doc.width, doc.height);
    match &base.content {
        LayerKind::Adjustment { .. } => {
            apply_adjustment_layer(base, backdrop)?;
            // layers clipped to an adjustment layer have nothing to clip to: draw normally
            for c in clipped.iter().filter(|c| c.visible) {
                let mut c = c.clone();
                c.clip = false;
                draw_layer(doc, &c, &[], backdrop)?;
            }
            return Ok(());
        }
        LayerKind::Group { children }
            if base.blend == BlendMode::PassThrough && clipped.is_empty() && base.fx.is_empty() =>
        {
            let m = mask_factor(base);
            if base.opacity >= 100.0 && m.is_none() {
                return composite_layers(doc, children, backdrop);
            }
            let mut tmp = backdrop.clone();
            composite_layers(doc, children, &mut tmp)?;
            // mix the group's result into the backdrop by opacity and mask (alpha too)
            let t = base.opacity / 100.0;
            let wu = w as usize;
            backdrop.data.par_chunks_mut(wu * 4).enumerate().for_each(|(y, row)| {
                for x in 0..wu {
                    let mut k = t;
                    if let Some(m) = &m {
                        k *= m.data[y * wu + x];
                    }
                    let i = (y * wu + x) * 4;
                    for c in 0..4 {
                        row[x * 4 + c] += (tmp.data[i + c] - row[x * 4 + c]) * k;
                    }
                }
            });
            return Ok(());
        }
        _ => {}
    }
    let Some((img, x, y)) = content(doc, base)? else {
        return Ok(());
    };
    let mut layer_img = Rendered { image: img.into_owned(), x, y }.to_canvas(w, h);
    for c in clipped.iter().filter(|c| c.visible) {
        if matches!(c.content, LayerKind::Adjustment { .. }) {
            let alpha: Vec<f32> = layer_img.data.chunks(4).map(|p| p[3]).collect();
            apply_adjustment_layer(c, &mut layer_img)?;
            for (p, a) in layer_img.data.chunks_mut(4).zip(alpha) {
                p[3] = a;
            }
            continue;
        }
        let Some((cimg, cx, cy)) = content(doc, c)? else { continue };
        let csrc = Rendered { image: cimg.into_owned(), x: cx, y: cy }.to_canvas(w, h);
        let cm = mask_factor(c);
        let alpha: Vec<f32> = layer_img.data.chunks(4).map(|p| p[3]).collect();
        blend_onto(&mut layer_img, &csrc, c.blend, c.opacity / 100.0, cm.as_ref(), layer_seed(c));
        // clipping keeps the base's coverage
        for (p, a) in layer_img.data.chunks_mut(4).zip(alpha) {
            p[3] = a;
        }
    }
    let m = mask_factor(base);
    let mode = if base.blend == BlendMode::PassThrough { BlendMode::Normal } else { base.blend };
    blend_onto(backdrop, &layer_img, mode, base.opacity / 100.0, m.as_ref(), layer_seed(base));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Rgba;
    use crate::doc::LayerMask;

    fn two_layer(top: Rgba, mode: BlendMode, opacity: f32) -> Document {
        let mut d = Document::from_image(Image::filled(4, 4, Rgba::new(0.5, 0.5, 0.5, 1.0)));
        let mut l = d.new_layer("Top", LayerKind::Raster { image: Image::filled(4, 4, top) });
        l.blend = mode;
        l.opacity = opacity;
        d.insert_above(None, l);
        d
    }

    #[test]
    fn normal_and_multiply() {
        let d = two_layer(Rgba::new(1.0, 0.0, 0.0, 1.0), BlendMode::Normal, 50.0);
        let c = composite(&d).unwrap();
        let p = c.get(0, 0);
        assert!((p[0] - 0.75).abs() < 1e-5 && (p[1] - 0.25).abs() < 1e-5 && p[3] == 1.0);
        let d = two_layer(Rgba::new(0.5, 1.0, 0.0, 1.0), BlendMode::Multiply, 100.0);
        let p = composite(&d).unwrap().get(1, 1);
        assert!((p[0] - 0.25).abs() < 1e-5 && (p[1] - 0.5).abs() < 1e-5 && p[2].abs() < 1e-5);
    }

    #[test]
    fn layer_mask_and_density() {
        let mut d = two_layer(Rgba::new(1.0, 1.0, 1.0, 1.0), BlendMode::Normal, 100.0);
        let p = d.resolve("Top").unwrap();
        let mut lm = LayerMask::new(Mask::from_fn(4, 4, |x, _| if x < 2 { 1.0 } else { 0.0 }));
        lm.density = 50.0;
        d.layer_mut(&p).mask = Some(lm);
        let c = composite(&d).unwrap();
        assert!((c.get(0, 0)[0] - 1.0).abs() < 1e-5);
        // hidden half shows at 50% density: 0.5 + 0.5*(1-0.5)... coverage 0.5
        assert!((c.get(3, 0)[0] - 0.75).abs() < 1e-5);
    }

    #[test]
    fn clipping_mask_limits_to_base_alpha() {
        let mut d = Document::new(4, 1);
        let base = d.new_layer(
            "Base",
            LayerKind::Raster {
                image: Image::from_fn(4, 1, |x, _| if x < 2 { [0.0, 0.0, 1.0, 1.0] } else { [0.0; 4] }),
            },
        );
        d.insert_above(None, base);
        let mut c =
            d.new_layer("Clip", LayerKind::Raster { image: Image::filled(4, 1, Rgba::new(1.0, 0.0, 0.0, 1.0)) });
        c.clip = true;
        d.insert_above(None, c);
        let out = composite(&d).unwrap();
        assert_eq!(out.get(0, 0), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(out.get(3, 0)[3], 0.0);
    }

    #[test]
    fn groups_pass_through_and_isolated() {
        let mut d = Document::from_image(Image::filled(2, 2, Rgba::new(0.5, 0.5, 0.5, 1.0)));
        let mut inner =
            d.new_layer("Inner", LayerKind::Raster { image: Image::filled(2, 2, Rgba::new(0.5, 0.5, 0.5, 1.0)) });
        inner.blend = BlendMode::Multiply;
        let g = {
            let mut g = d.new_layer("G", LayerKind::Group { children: vec![] });
            g.children_mut().unwrap().push(inner);
            g
        };
        d.insert_above(None, g);
        // pass-through: multiply reaches the background → 0.25
        assert!((composite(&d).unwrap().get(0, 0)[0] - 0.25).abs() < 1e-5);
        // isolated (normal): multiply over transparency inside the group → 0.5
        let p = d.resolve("G").unwrap();
        d.layer_mut(&p).blend = BlendMode::Normal;
        assert!((composite(&d).unwrap().get(0, 0)[0] - 0.5).abs() < 1e-5);
    }

    #[test]
    fn placed_layers_render_into_rect() {
        let mut d = Document::new(10, 10);
        let l = d.new_layer(
            "P",
            LayerKind::Placed {
                asset: "x".into(),
                path: "x.png".into(),
                linked: false,
                rect: [2.0, 2.0, 4.0, 4.0],
                angle: 0.0,
                source: Some(std::sync::Arc::new(Image::filled(8, 8, Rgba::WHITE))),
            },
        );
        d.insert_above(None, l);
        let c = composite(&d).unwrap();
        assert_eq!(c.get(3, 3)[3], 1.0);
        assert_eq!(c.get(0, 0)[3], 0.0);
        assert_eq!(layer_bounds(&d, &d.layers[0]).unwrap(), Some(RectI::new(2, 2, 4, 4)));
    }
}
