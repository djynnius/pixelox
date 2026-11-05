//! Geometry helpers shared by every crate that moves layers: SVG path-data transforms and
//! `translate_layer`, which moves any kind of layer consistently (raster offset, text
//! anchor, shape path, placed rect, group children, and the layer mask with it).

use crate::doc::{Document, Layer, LayerKind, LayerPath};
use crate::error::{Error, Result};
use crate::image::Mask;
use kurbo::{Affine, BezPath};
use serde_json::{json, Value};

/// Apply an affine transform `[a, b, c, d, e, f]` (x' = a·x + c·y + e, y' = b·x + d·y + f)
/// to SVG path data. Arcs become cubic Béziers.
pub fn transform_path_data(d: &str, m: [f64; 6]) -> Result<String> {
    let mut p = BezPath::from_svg(d)
        .map_err(|e| Error::new("PX1010", format!("invalid SVG path data: {e}")).with_context("d", d))?;
    p.apply_affine(Affine::new(m));
    Ok(p.to_svg())
}

pub fn translate_path_data(d: &str, dx: f64, dy: f64) -> Result<String> {
    transform_path_data(d, [1.0, 0.0, 0.0, 1.0, dx, dy])
}

/// Bounding box `(x0, y0, x1, y1)` of SVG path data (control points included).
pub fn path_bbox(d: &str) -> Result<(f64, f64, f64, f64)> {
    use kurbo::Shape;
    let p = BezPath::from_svg(d).map_err(|e| Error::new("PX1010", format!("invalid SVG path data: {e}")))?;
    let r = p.bounding_box();
    Ok((r.x0, r.y0, r.x1, r.y1))
}

/// Shift a mask's content by whole pixels. The vacated area takes the mask's border value
/// when the border is uniform (a fully revealing mask stays revealing), else 0.
pub fn shift_mask(m: &Mask, dx: i32, dy: i32) -> Mask {
    let fill = border_fill(m);
    Mask::from_fn(m.width, m.height, |x, y| {
        let (sx, sy) = (x as i64 - dx as i64, y as i64 - dy as i64);
        if sx < 0 || sy < 0 || sx >= m.width as i64 || sy >= m.height as i64 {
            fill
        } else {
            m.get(sx as u32, sy as u32)
        }
    })
}

fn border_fill(m: &Mask) -> f32 {
    if m.width == 0 || m.height == 0 {
        return 0.0;
    }
    let (w, h) = (m.width, m.height);
    let mut all_one = true;
    for x in 0..w {
        all_one &= m.get(x, 0) >= 0.999 && m.get(x, h - 1) >= 0.999;
    }
    for y in 0..h {
        all_one &= m.get(0, y) >= 0.999 && m.get(w - 1, y) >= 0.999;
    }
    if all_one {
        1.0
    } else {
        0.0
    }
}

fn num(v: Option<&Value>) -> f64 {
    v.and_then(|v| v.as_f64().or_else(|| v.as_str().and_then(|s| s.parse().ok()))).unwrap_or(0.0)
}

fn translate_one(l: &mut Layer, dx: f64, dy: f64) -> Result<()> {
    match &mut l.content {
        LayerKind::Raster { .. } => {
            l.x += dx.round() as i32;
            l.y += dy.round() as i32;
        }
        LayerKind::Text { spec } => {
            let x = num(spec.get("x")) + dx;
            let y = num(spec.get("y")) + dy;
            spec["x"] = json!(x);
            spec["y"] = json!(y);
            if let Some(d) = spec.get("path").and_then(|v| v.as_str()).map(String::from) {
                spec["path"] = json!(translate_path_data(&d, dx, dy)?);
            }
        }
        LayerKind::Shape { spec } => {
            if let Some(d) = spec.get("d").and_then(|v| v.as_str()).map(String::from) {
                spec["d"] = json!(translate_path_data(&d, dx, dy)?);
            }
        }
        LayerKind::Placed { rect, .. } => {
            rect[0] += dx;
            rect[1] += dy;
        }
        LayerKind::Group { children } => {
            for c in children.iter_mut() {
                translate_one(c, dx, dy)?;
            }
        }
        LayerKind::Adjustment { .. } => {}
    }
    if let Some(lm) = &mut l.mask {
        lm.mask = shift_mask(&lm.mask, dx.round() as i32, dy.round() as i32);
    }
    if let Some(vm) = &l.vector_mask {
        l.vector_mask = Some(translate_path_data(vm, dx, dy)?);
    }
    Ok(())
}

/// Move a layer (and everything it contains, and its mask) by (`dx`, `dy`) canvas pixels.
pub fn translate_layer(doc: &mut Document, path: &LayerPath, dx: f64, dy: f64) -> Result<()> {
    let l = doc.layer_mut(path);
    l.check_unlocked()?;
    translate_one(l, dx, dy)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::Image;

    #[test]
    fn path_translation_and_bbox() {
        let d = translate_path_data("M 0 0 L 10 0 L 10 10 Z", 5.0, -2.0).unwrap();
        let (x0, y0, x1, y1) = path_bbox(&d).unwrap();
        assert_eq!((x0, y0, x1, y1), (5.0, -2.0, 15.0, 8.0));
        assert!(transform_path_data("nonsense", [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]).is_err());
    }

    #[test]
    fn translate_every_kind() {
        let mut d = Document::new(20, 20);
        let r = d.new_layer("R", LayerKind::Raster { image: Image::new(4, 4) });
        d.insert_above(None, r);
        let t = d.new_layer("T", LayerKind::Text { spec: json!({"content": "hi", "x": 1, "y": 2}) });
        d.insert_above(None, t);
        let s = d.new_layer("S", LayerKind::Shape { spec: json!({"d": "M 0 0 L 4 0 L 4 4 Z"}) });
        d.insert_above(None, s);
        for name in ["R", "T", "S"] {
            let p = d.resolve(name).unwrap();
            translate_layer(&mut d, &p, 3.0, 4.0).unwrap();
        }
        assert_eq!((d.layers[0].x, d.layers[0].y), (3, 4));
        if let LayerKind::Text { spec } = &d.layers[1].content {
            assert_eq!((spec["x"].as_f64(), spec["y"].as_f64()), (Some(4.0), Some(6.0)));
        }
        if let LayerKind::Shape { spec } = &d.layers[2].content {
            assert_eq!(path_bbox(spec["d"].as_str().unwrap()).unwrap(), (3.0, 4.0, 7.0, 8.0));
        }
    }
}
