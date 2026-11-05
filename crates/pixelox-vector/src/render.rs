//! The shape renderer hook (`compose::set_shape_renderer`): anti-aliased fill and stroke
//! (dashes, caps, joins) through tiny-skia coverage masks, composited in `f32` so colors
//! stay exact. The result is cropped to the shape's bounds (stroke included) on the canvas.

use crate::spec::{to_skia, ShapeSpec};
use pixelox_core::prelude::*;
use rayon::prelude::*;

/// Renderer installed by [`crate::install`].
pub fn render(layer: &Layer, doc: &Document) -> Result<Option<Rendered>> {
    let LayerKind::Shape { spec } = &layer.content else {
        return Ok(None);
    };
    let s = ShapeSpec::from_value(spec)
        .map_err(|e| Error::new("PX3001", format!("shape layer '{}': {}", layer.name, e.message)))?;
    render_spec(&s, doc.width, doc.height).map_err(|e| {
        Error::new("PX3001", format!("shape layer '{}': {}", layer.name, e.message)).with_context("layer", &layer.name)
    })
}

fn union(a: Option<tiny_skia::Rect>, b: tiny_skia::Rect) -> Option<tiny_skia::Rect> {
    Some(match a {
        None => b,
        Some(a) => tiny_skia::Rect::from_ltrb(
            a.left().min(b.left()),
            a.top().min(b.top()),
            a.right().max(b.right()),
            a.bottom().max(b.bottom()),
        )?,
    })
}

/// Render a spec onto a `cw`×`ch` canvas (None when nothing is visible on it).
pub fn render_spec(s: &ShapeSpec, cw: u32, ch: u32) -> Result<Option<Rendered>> {
    let bez = s.bez()?;
    let Some(path) = to_skia(&bez) else {
        return Ok(None);
    };
    let fill = s.fill.filter(|c| c.a > 0.0);
    let stroke_color = s.stroke.filter(|c| c.a > 0.0 && s.stroke_width > 0.0);
    let outline = match stroke_color {
        None => None,
        Some(_) => {
            let dashed = match s.dash_even() {
                Some(d) => tiny_skia::StrokeDash::new(d.iter().map(|v| *v as f32).collect(), s.dash_offset as f32)
                    .and_then(|sd| path.dash(&sd, 1.0)),
                None => Some(path.clone()),
            };
            dashed.and_then(|p| p.stroke(&s.skia_stroke(), 1.0))
        }
    };
    let mut bounds = None;
    if fill.is_some() {
        bounds = union(bounds, path.bounds());
    }
    if let Some(o) = &outline {
        bounds = union(bounds, o.bounds());
    }
    let Some(b) = bounds else {
        return Ok(None);
    };
    let x0 = ((b.left() as f64).floor() as i64 - 1).max(0);
    let y0 = ((b.top() as f64).floor() as i64 - 1).max(0);
    let x1 = ((b.right() as f64).ceil() as i64 + 1).min(cw as i64);
    let y1 = ((b.bottom() as f64).ceil() as i64 + 1).min(ch as i64);
    if x1 <= x0 || y1 <= y0 {
        return Ok(None);
    }
    let (w, h) = ((x1 - x0) as u32, (y1 - y0) as u32);
    let tf = tiny_skia::Transform::from_translate(-x0 as f32, -y0 as f32);
    let coverage = |p: &tiny_skia::Path, rule: tiny_skia::FillRule| -> Result<Vec<u8>> {
        let mut m = tiny_skia::Mask::new(w, h)
            .ok_or_else(|| Error::new("PX3001", format!("cannot allocate a {w}x{h} shape mask")))?;
        m.fill_path(p, rule, true, tf);
        Ok(m.data().to_vec())
    };
    let rule = if s.fill_rule == "evenodd" { tiny_skia::FillRule::EvenOdd } else { tiny_skia::FillRule::Winding };
    let fm = match fill {
        Some(_) => Some(coverage(&path, rule)?),
        None => None,
    };
    let sm = match &outline {
        Some(o) => Some(coverage(o, tiny_skia::FillRule::Winding)?),
        None => None,
    };
    let fc = fill.unwrap_or(Rgba::TRANSPARENT);
    let sc = stroke_color.unwrap_or(Rgba::TRANSPARENT);
    let mut img = Image::new(w, h);
    let wu = w as usize;
    img.data.par_chunks_mut(wu * 4).enumerate().for_each(|(y, row)| {
        for x in 0..wu {
            let i = y * wu + x;
            let fa = fm.as_ref().map(|m| m[i] as f32 / 255.0).unwrap_or(0.0) * fc.a;
            let sa = sm.as_ref().map(|m| m[i] as f32 / 255.0).unwrap_or(0.0) * sc.a;
            let a = sa + fa * (1.0 - sa);
            if a <= 0.0 {
                continue;
            }
            let k = fa * (1.0 - sa);
            let px = &mut row[x * 4..x * 4 + 4];
            px[0] = (sc.r * sa + fc.r * k) / a;
            px[1] = (sc.g * sa + fc.g * k) / a;
            px[2] = (sc.b * sa + fc.b * k) / a;
            px[3] = a.min(1.0);
        }
    });
    Ok(Some(Rendered { image: img, x: x0 as i32, y: y0 as i32 }))
}
