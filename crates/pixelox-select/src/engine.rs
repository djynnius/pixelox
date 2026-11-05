//! The geometry engine behind `transform *`: applies one map to layers of every kind and to
//! every mask, following the shared-keys contract in `docs/dev/AUTHORING.md`:
//!
//! * raster pixels are resampled (exact for lattice maps);
//! * text: `x`/`y` (anchor) are mapped, `size` scales, `rotation` accumulates, `path` maps;
//! * shapes: `d` maps through `geom::transform_path_data`, `stroke_width` (and dashes) scale;
//! * placed: `rect` and `angle` follow the map;
//! * groups recurse; adjustment layers have no geometry;
//! * layer masks, vector masks, the selection, library masks, and guides follow too.
//!
//! What a spec cannot express (text under a non-uniform scale or a mirror, any live layer
//! under skew/perspective/warp) is rasterized first with `compose::render_layer`, with a
//! warning.

use crate::sample::{affine_raster, border_fill, warp_mask};
use crate::util::{fit_mask, r4, Affine2, SIMILAR_TOL};
use crate::warp::CoordField;
use pixelox_core::doc::LayerPath;
use pixelox_core::kernels::Filter;
use pixelox_core::prelude::*;
use pixelox_core::units::Length;

/// Warning raised when a live layer had to be rasterized to apply a transform.
pub const RASTERIZED: &str = "PXW2003";

/// A map from old canvas coordinates to new ones.
pub enum Geo<'a> {
    Affine(Affine2),
    /// A non-affine map sampled per destination pixel (perspective, mesh warp).
    Field(&'a CoordField),
}

impl Geo<'_> {
    fn affine(&self) -> Option<&Affine2> {
        match self {
            Geo::Affine(m) => Some(m),
            Geo::Field(_) => None,
        }
    }
}

fn angle_is_flat(angle: f64) -> bool {
    let r = angle.rem_euclid(180.0);
    !(1e-9..=180.0 - 1e-9).contains(&r)
}

/// Whether `l` itself (not its children) must become pixels before `geo` can apply.
/// `force` rasterizes every live kind (skew, as the contract asks).
pub fn needs_raster(l: &Layer, geo: &Geo, force: bool) -> bool {
    let live = matches!(l.content, LayerKind::Text { .. } | LayerKind::Shape { .. } | LayerKind::Placed { .. });
    let Some(m) = geo.affine() else { return live };
    if force {
        return live;
    }
    match &l.content {
        LayerKind::Text { .. } => m.similarity(SIMILAR_TOL).is_none(),
        LayerKind::Placed { angle, .. } => {
            !(m.similarity(SIMILAR_TOL).is_some() || (m.axis_scale().is_some() && angle_is_flat(*angle)))
        }
        _ => false,
    }
}

/// Replace a live layer with its rendered pixels (fx stay live on the new raster).
pub fn rasterize_layer(doc: &mut Document, path: &[usize]) -> Result<()> {
    let mut c = doc.layer(path).clone();
    c.fx.clear();
    let r = compose::render_layer(doc, &c)?;
    let l = doc.layer_mut(path);
    match r {
        Some(r) => {
            l.content = LayerKind::Raster { image: r.image };
            l.x = r.x;
            l.y = r.y;
        }
        None => {
            l.content = LayerKind::Raster { image: Image::new(1, 1) };
            l.x = 0;
            l.y = 0;
        }
    }
    Ok(())
}

/// Rasterize every layer under `root` (all layers when `None`) that `geo` needs as pixels.
pub fn rasterize_where(
    doc: &mut Document,
    ctx: &mut OpCtx,
    root: Option<&[usize]>,
    geo: &Geo,
    force: bool,
) -> Result<Vec<String>> {
    let todo: Vec<(LayerPath, String)> = doc
        .walk()
        .into_iter()
        .filter(|(p, l)| root.is_none_or(|r| p.starts_with(r)) && needs_raster(l, geo, force))
        .map(|(p, l)| (p, l.name.clone()))
        .collect();
    for (p, _) in &todo {
        rasterize_layer(doc, p)?;
    }
    let names: Vec<String> = todo.into_iter().map(|(_, n)| n).collect();
    if !names.is_empty() {
        ctx.warn_ctx(
            RASTERIZED,
            format!("rasterized {} to apply this transform: {}", plural(names.len(), "layer"), names.join(", ")),
            json!({"layers": names}),
        );
    }
    Ok(names)
}

fn plural(n: usize, w: &str) -> String {
    if n == 1 {
        format!("1 {w}")
    } else {
        format!("{n} {w}s")
    }
}

fn num(v: Option<&Value>) -> Option<f64> {
    v.and_then(|v| v.as_f64().or_else(|| v.as_str().and_then(|s| s.trim().parse().ok())))
}

fn fmt_num(v: f64) -> String {
    let r = r4(v);
    if r == r.trunc() {
        format!("{}", r as i64)
    } else {
        format!("{r}")
    }
}

/// Scale a CSS-like length value (`48pt`, `64px`, `12`, or a number) by `k`.
pub fn scale_length_value(v: &Value, k: f64) -> Value {
    if let Some(n) = v.as_f64() {
        return json!(r4(n * k));
    }
    let Some(s) = v.as_str() else { return v.clone() };
    let t = s.trim();
    let Ok(l) = Length::parse(t) else { return v.clone() };
    let num_part_len = t.trim_end_matches(|c: char| c.is_ascii_alphabetic() || c == '%').len();
    let suffix = &t[num_part_len..];
    json!(format!("{}{}", fmt_num(l.value * k), suffix))
}

fn map_text(spec: &mut Value, m: &Affine2) -> Result<()> {
    let (s, deg) = m.similarity(SIMILAR_TOL).unwrap_or((m.uniform_scale(), m.b.atan2(m.a).to_degrees()));
    let x = num(spec.get("x")).unwrap_or(0.0);
    let y = num(spec.get("y")).unwrap_or(0.0);
    let (nx, ny) = m.apply(x, y);
    spec["x"] = json!(r4(nx));
    spec["y"] = json!(r4(ny));
    if (s - 1.0).abs() > 1e-12 {
        if let Some(size) = spec.get("size").cloned() {
            spec["size"] = scale_length_value(&size, s);
        }
    }
    if deg.abs() > 1e-9 || spec.get("rotation").is_some() {
        let mut r = num(spec.get("rotation")).unwrap_or(0.0) + deg;
        r = (r + 180.0).rem_euclid(360.0) - 180.0;
        spec["rotation"] = json!(r4(r));
    }
    if let Some(d) = spec.get("path").and_then(|v| v.as_str()).map(String::from) {
        spec["path"] = json!(geom::transform_path_data(&d, m.coeffs())?);
    }
    Ok(())
}

fn map_shape(spec: &mut Value, m: &Affine2) -> Result<()> {
    if let Some(d) = spec.get("d").and_then(|v| v.as_str()).map(String::from) {
        spec["d"] = json!(geom::transform_path_data(&d, m.coeffs())?);
    }
    let s = m.uniform_scale();
    if (s - 1.0).abs() > 1e-12 {
        for key in ["stroke_width", "dash_offset"] {
            if let Some(v) = num(spec.get(key)) {
                spec[key] = json!(r4(v * s));
            }
        }
        if let Some(Value::Array(dash)) = spec.get("dash").cloned() {
            spec["dash"] =
                Value::Array(dash.iter().map(|v| v.as_f64().map(|n| json!(r4(n * s))).unwrap_or(v.clone())).collect());
        }
    }
    Ok(())
}

fn map_placed(rect: &mut [f64; 4], angle: &mut f64, m: &Affine2) {
    let [x, y, mut w, mut h] = *rect;
    let (cx, cy) = m.apply(x + w / 2.0, y + h / 2.0);
    if let Some((s, deg)) = m.similarity(SIMILAR_TOL) {
        w *= s;
        h *= s;
        if deg.abs() > 1e-9 {
            *angle = r4((*angle + deg + 180.0).rem_euclid(360.0) - 180.0);
        }
    } else if let Some((sx, sy)) = m.axis_scale() {
        w *= sx;
        h *= sy;
    }
    *rect = [r4(cx - w / 2.0), r4(cy - h / 2.0), r4(w), r4(h)];
}

/// Transform a layer's content (recursively for groups), not its masks.
pub fn transform_content(l: &mut Layer, geo: &Geo, filter: Option<Filter>) -> Result<()> {
    let name = l.name.clone();
    let not_affine =
        || Error::internal(format!("layer '{name}' should have been rasterized before a non-affine transform"));
    match &mut l.content {
        LayerKind::Raster { image } => {
            let (img, x, y) = match geo {
                Geo::Affine(m) => affine_raster(image, l.x, l.y, m, filter)?,
                Geo::Field(f) => f.raster(image, l.x, l.y, filter),
            };
            *image = img;
            l.x = x;
            l.y = y;
        }
        LayerKind::Text { spec } => map_text(spec, geo.affine().ok_or_else(not_affine)?)?,
        LayerKind::Shape { spec } => map_shape(spec, geo.affine().ok_or_else(not_affine)?)?,
        LayerKind::Placed { rect, angle, .. } => map_placed(rect, angle, geo.affine().ok_or_else(not_affine)?),
        LayerKind::Group { children } => {
            for c in children.iter_mut() {
                transform_content(c, geo, filter)?;
            }
        }
        LayerKind::Adjustment { .. } => {}
    }
    Ok(())
}

/// Transform a layer's raster mask and vector mask (recursively) into a `nw`×`nh` canvas.
pub fn transform_masks(l: &mut Layer, geo: &Geo, nw: u32, nh: u32) -> Result<()> {
    if let Some(lm) = &mut l.mask {
        let fill = border_fill(&lm.mask);
        lm.mask = match geo {
            Geo::Affine(m) => {
                lm.feather = (lm.feather as f64 * m.uniform_scale()) as f32;
                warp_mask(&lm.mask, m, nw, nh, fill)
            }
            Geo::Field(f) => f.mask(&lm.mask, nw, nh, fill),
        };
    }
    if let Some(d) = l.vector_mask.take() {
        // a non-affine map cannot keep path data; the raster mask carries the shape
        if let Geo::Affine(m) = geo {
            l.vector_mask = Some(geom::transform_path_data(&d, m.coeffs())?);
        }
    }
    if let Some(ch) = l.children_mut() {
        for c in ch.iter_mut() {
            transform_masks(c, geo, nw, nh)?;
        }
    }
    Ok(())
}

/// Apply `geo` to one layer (and its children and masks) in place. PX2304 when locked.
pub fn layer_geo(
    doc: &mut Document,
    ctx: &mut OpCtx,
    path: &[usize],
    geo: &Geo,
    filter: Option<Filter>,
    force: bool,
) -> Result<Vec<String>> {
    doc.layer(path).check_unlocked()?;
    let rasterized = rasterize_where(doc, ctx, Some(path), geo, force)?;
    let (w, h) = (doc.width, doc.height);
    let l = doc.layer_mut(path);
    transform_content(l, geo, filter)?;
    transform_masks(l, geo, w, h)?;
    Ok(rasterized)
}

fn take_masks(l: &mut Layer, out: &mut Vec<Option<LayerMask>>) {
    out.push(l.mask.take());
    if let Some(ch) = l.children_mut() {
        for c in ch.iter_mut() {
            take_masks(c, out);
        }
    }
}

fn put_masks(l: &mut Layer, it: &mut impl Iterator<Item = Option<LayerMask>>) {
    l.mask = it.next().flatten();
    if let Some(ch) = l.children_mut() {
        for c in ch.iter_mut() {
            put_masks(c, it);
        }
    }
}

/// Move every layer by whole pixels with `geom::translate_layer` (document-wide, so locks
/// do not block it). Layer masks are handled by the caller.
fn translate_all(doc: &mut Document, dx: f64, dy: f64) -> Result<()> {
    for i in 0..doc.layers.len() {
        let mut saved = Vec::new();
        take_masks(&mut doc.layers[i], &mut saved);
        let locked = std::mem::replace(&mut doc.layers[i].locked, false);
        let r = geom::translate_layer(doc, &vec![i], dx, dy);
        doc.layers[i].locked = locked;
        put_masks(&mut doc.layers[i], &mut saved.into_iter());
        r?;
    }
    Ok(())
}

/// Apply an affine map to the whole document: every layer (locked ones too: document
/// geometry keeps layers registered), the selection, library masks, layer masks, and
/// guides; the canvas becomes `nw`×`nh`. Returns the names of rasterized layers.
pub fn doc_affine(
    doc: &mut Document,
    ctx: &mut OpCtx,
    m: &Affine2,
    nw: u32,
    nh: u32,
    filter: Option<Filter>,
) -> Result<Vec<String>> {
    let geo = Geo::Affine(*m);
    let rasterized = rasterize_where(doc, ctx, None, &geo, false)?;
    let (ow, oh) = (doc.width, doc.height);
    let whole = |v: f64| (v - v.round()).abs() < 1e-9;
    if m.is_translation() && whole(m.e) && whole(m.f) {
        translate_all(doc, m.e.round(), m.f.round())?;
    } else {
        for l in doc.layers.iter_mut() {
            transform_content(l, &geo, filter)?;
        }
    }
    for l in doc.layers.iter_mut() {
        transform_masks(l, &geo, nw, nh)?;
    }
    if let Some(s) = doc.selection.take() {
        doc.selection = Some(warp_mask(&fit_mask(s, ow, oh), m, nw, nh, 0.0));
    }
    let names: Vec<String> = doc.masks.keys().cloned().collect();
    for n in names {
        if let Some(mk) = doc.masks.remove(&n) {
            doc.masks.insert(n, warp_mask(&fit_mask(mk, ow, oh), m, nw, nh, 0.0));
        }
    }
    map_guides(doc, m, ow, oh);
    if let Some(g) = &mut doc.grid {
        if m.b == 0.0 && m.c == 0.0 {
            g.size = [r4(g.size[0] * m.a.abs()), r4(g.size[1] * m.d.abs())];
        } else if m.a == 0.0 && m.d == 0.0 {
            g.size = [r4(g.size[1] * m.c.abs()), r4(g.size[0] * m.b.abs())];
        }
    }
    doc.width = nw;
    doc.height = nh;
    Ok(rasterized)
}

/// Guides stay axis-aligned: each is mapped through its point on the canvas center line and
/// swaps orientation when the map turns more than 45°.
fn map_guides(doc: &mut Document, m: &Affine2, ow: u32, oh: u32) {
    let (cx, cy) = (ow as f64 / 2.0, oh as f64 / 2.0);
    for g in doc.guides.iter_mut() {
        if g.orientation == "v" {
            let (x, y) = m.apply(g.pos, cy);
            if m.a.abs() >= m.b.abs() {
                g.pos = r4(x);
            } else {
                g.orientation = "h".into();
                g.pos = r4(y);
            }
        } else {
            let (x, y) = m.apply(cx, g.pos);
            if m.d.abs() >= m.c.abs() {
                g.pos = r4(y);
            } else {
                g.orientation = "v".into();
                g.pos = r4(x);
            }
        }
    }
}

/// Delete raster pixels outside the canvas (crop semantics), recursively.
pub fn crop_rasters(l: &mut Layer, w: u32, h: u32) {
    if let LayerKind::Raster { image } = &mut l.content {
        let r = RectI::new(l.x, l.y, image.width, image.height).intersect(&RectI::new(0, 0, w, h));
        if r.is_empty() {
            *image = Image::new(1, 1);
            l.x = 0;
            l.y = 0;
        } else if r != RectI::new(l.x, l.y, image.width, image.height) {
            *image = image.crop(RectI::new(r.x - l.x, r.y - l.y, r.w, r.h));
            l.x = r.x;
            l.y = r.y;
        }
    }
    if let Some(ch) = l.children_mut() {
        for c in ch.iter_mut() {
            crop_rasters(c, w, h);
        }
    }
}

/// Crop the document to `r` (canvas pixels, inside the canvas): every layer, mask, and
/// guide shifts; raster pixels outside the new canvas are deleted.
pub fn crop_doc(doc: &mut Document, ctx: &mut OpCtx, r: RectI) -> Result<Vec<String>> {
    let rasterized = doc_affine(doc, ctx, &Affine2::translate(-r.x as f64, -r.y as f64), r.w, r.h, None)?;
    for l in doc.layers.iter_mut() {
        crop_rasters(l, r.w, r.h);
    }
    Ok(rasterized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lengths_scale_keep_units() {
        assert_eq!(scale_length_value(&json!("48pt"), 2.0), json!("96pt"));
        assert_eq!(scale_length_value(&json!("10.5px"), 0.5), json!("5.25px"));
        assert_eq!(scale_length_value(&json!("12"), 1.5), json!("18"));
        assert_eq!(scale_length_value(&json!(20), 0.5), json!(10.0));
    }

    #[test]
    fn text_follows_similarity() {
        let mut spec = json!({"content": "hi", "x": 10, "y": 0, "size": "20px"});
        let m = Affine2::rotate(90.0).then(Affine2::scale(2.0, 2.0));
        map_text(&mut spec, &m).unwrap();
        assert_eq!((spec["x"].as_f64(), spec["y"].as_f64()), (Some(0.0), Some(20.0)));
        assert_eq!(spec["size"], json!("40px"));
        assert_eq!(spec["rotation"].as_f64(), Some(90.0));
    }

    #[test]
    fn placed_follows_similarity_and_axis_scale() {
        let mut rect = [0.0, 0.0, 10.0, 20.0];
        let mut angle = 0.0;
        map_placed(&mut rect, &mut angle, &Affine2::scale(2.0, 1.0));
        assert_eq!(rect, [0.0, 0.0, 20.0, 20.0]);
        map_placed(&mut rect, &mut angle, &Affine2::rotate(30.0));
        assert!((angle - 30.0).abs() < 1e-9);
    }
}
