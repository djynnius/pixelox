//! Rasterizing laid-out text (anti-aliased tiny-skia coverage masks composited in `f32`),
//! SVG path data for `text to-path`, and the compositor hook.

use crate::layout::{compose, Composed};
use crate::spec::TextSpec;
use kurbo::{BezPath, PathEl};
use pixelox_core::prelude::*;
use rayon::prelude::*;
use tiny_skia::{FillRule, LineJoin, Mask, PathBuilder, Stroke, Transform};

/// How far outside the canvas text is still rasterized (room for layer styles).
const MARGIN: f64 = 1024.0;

fn to_skia(p: &BezPath) -> Option<tiny_skia::Path> {
    let mut b = PathBuilder::new();
    for el in p.elements() {
        match *el {
            PathEl::MoveTo(a) => b.move_to(a.x as f32, a.y as f32),
            PathEl::LineTo(a) => b.line_to(a.x as f32, a.y as f32),
            PathEl::QuadTo(a, c) => b.quad_to(a.x as f32, a.y as f32, c.x as f32, c.y as f32),
            PathEl::CurveTo(a, c, d) => {
                b.cubic_to(a.x as f32, a.y as f32, c.x as f32, c.y as f32, d.x as f32, d.y as f32)
            }
            PathEl::ClosePath => b.close(),
        }
    }
    b.finish()
}

fn stroked(p: &tiny_skia::Path, width: f64) -> Option<tiny_skia::Path> {
    if width <= 0.0 {
        return None;
    }
    p.stroke(&Stroke { width: width as f32, line_join: LineJoin::Round, ..Stroke::default() }, 1.0)
}

/// The filled shapes and the stroke band of laid-out text, as tiny-skia paths.
struct Paths {
    fill: Vec<tiny_skia::Path>,
    stroke: Vec<tiny_skia::Path>,
}

fn paths(c: &Composed) -> Paths {
    let regular = to_skia(&c.regular);
    let bold = to_skia(&c.bold);
    let mut fill: Vec<tiny_skia::Path> = Vec::new();
    fill.extend(regular.clone());
    if let Some(b) = &bold {
        fill.push(b.clone());
        fill.extend(stroked(b, c.bold_width));
    }
    let mut stroke = Vec::new();
    if let Some((_, w)) = c.stroke {
        stroke.extend(regular.as_ref().and_then(|p| stroked(p, w)));
        stroke.extend(bold.as_ref().and_then(|p| stroked(p, w + c.bold_width)));
    }
    Paths { fill, stroke }
}

fn coverage(paths: &[tiny_skia::Path], x0: i32, y0: i32, w: u32, h: u32) -> Option<Mask> {
    if paths.is_empty() {
        return None;
    }
    let mut m = Mask::new(w, h)?;
    let ts = Transform::from_translate(-x0 as f32, -y0 as f32);
    for p in paths {
        if let Some(p) = p.clone().transform(ts) {
            m.fill_path(&p, FillRule::Winding, true, Transform::identity());
        }
    }
    Some(m)
}

/// Rasterize laid-out text: an image cropped to the ink bounds (padded for anti-aliasing),
/// clipped to the canvas plus a margin. `None` when nothing is drawn.
pub fn rasterize(c: &Composed, canvas_w: u32, canvas_h: u32) -> Option<Rendered> {
    if c.is_empty() {
        return None;
    }
    let [ix0, iy0, ix1, iy1] = c.ink()?;
    let x0 = (ix0.floor() - 1.0).max(-MARGIN) as i32;
    let y0 = (iy0.floor() - 1.0).max(-MARGIN) as i32;
    let x1 = (ix1.ceil() + 1.0).min(canvas_w as f64 + MARGIN) as i32;
    let y1 = (iy1.ceil() + 1.0).min(canvas_h as f64 + MARGIN) as i32;
    if x1 <= x0 || y1 <= y0 {
        return None;
    }
    let (w, h) = ((x1 - x0) as u32, (y1 - y0) as u32);
    let p = paths(c);
    let fill = coverage(&p.fill, x0, y0, w, h);
    let stroke = coverage(&p.stroke, x0, y0, w, h);
    let fc = c.color;
    let sc = c.stroke.map(|(s, _)| s).unwrap_or(Rgba::TRANSPARENT);
    let mut img = Image::new(w, h);
    let wu = w as usize;
    img.data.par_chunks_mut(wu * 4).enumerate().for_each(|(y, row)| {
        for x in 0..wu {
            let i = y * wu + x;
            let fa = fill.as_ref().map_or(0.0, |m| m.data()[i] as f32 / 255.0) * fc.a;
            let sa = stroke.as_ref().map_or(0.0, |m| m.data()[i] as f32 / 255.0) * sc.a;
            let a = sa + fa * (1.0 - sa);
            if a <= 0.0 {
                continue;
            }
            let k = fa * (1.0 - sa);
            let px = &mut row[x * 4..x * 4 + 4];
            px[0] = (sc.r * sa + fc.r * k) / a;
            px[1] = (sc.g * sa + fc.g * k) / a;
            px[2] = (sc.b * sa + fc.b * k) / a;
            px[3] = a;
        }
    });
    Some(Rendered { image: img, x: x0, y: y0 })
}

/// The compositor hook (`compose::set_text_renderer`). Missing fonts fall back to the
/// embedded face so a document always renders.
pub fn render_text_layer(layer: &Layer, doc: &Document) -> Result<Option<Rendered>> {
    let LayerKind::Text { spec } = &layer.content else {
        return Ok(None);
    };
    let spec = TextSpec::from_value(spec);
    if spec.content.is_empty() {
        return Ok(None);
    }
    let c = compose(&spec, doc, false)?;
    Ok(rasterize(&c, doc.width, doc.height))
}

fn fmt(v: f64) -> String {
    let r = (v * 100.0).round() / 100.0;
    let s = format!("{r:.2}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" {
        "0".to_string()
    } else {
        s.to_string()
    }
}

fn bez_svg(p: &BezPath, out: &mut String) {
    for el in p.elements() {
        if !out.is_empty() {
            out.push(' ');
        }
        match *el {
            PathEl::MoveTo(a) => out.push_str(&format!("M{} {}", fmt(a.x), fmt(a.y))),
            PathEl::LineTo(a) => out.push_str(&format!("L{} {}", fmt(a.x), fmt(a.y))),
            PathEl::QuadTo(a, b) => out.push_str(&format!("Q{} {} {} {}", fmt(a.x), fmt(a.y), fmt(b.x), fmt(b.y))),
            PathEl::CurveTo(a, b, c) => {
                out.push_str(&format!("C{} {} {} {} {} {}", fmt(a.x), fmt(a.y), fmt(b.x), fmt(b.y), fmt(c.x), fmt(c.y)))
            }
            PathEl::ClosePath => out.push('Z'),
        }
    }
}

fn skia_svg(p: &tiny_skia::Path, out: &mut String) {
    use tiny_skia::PathSegment as S;
    let f = |v: f32| fmt(v as f64);
    for seg in p.segments() {
        if !out.is_empty() {
            out.push(' ');
        }
        match seg {
            S::MoveTo(a) => out.push_str(&format!("M{} {}", f(a.x), f(a.y))),
            S::LineTo(a) => out.push_str(&format!("L{} {}", f(a.x), f(a.y))),
            S::QuadTo(a, b) => out.push_str(&format!("Q{} {} {} {}", f(a.x), f(a.y), f(b.x), f(b.y))),
            S::CubicTo(a, b, c) => {
                out.push_str(&format!("C{} {} {} {} {} {}", f(a.x), f(a.y), f(b.x), f(b.y), f(c.x), f(c.y)))
            }
            S::Close => out.push('Z'),
        }
    }
}

/// Glyph outlines as SVG path data in canvas px (nonzero fill). Synthetic bold is baked in
/// as the outline of its stroke.
pub fn svg_path_data(c: &Composed) -> String {
    let mut out = String::new();
    bez_svg(&c.regular, &mut out);
    bez_svg(&c.bold, &mut out);
    if c.bold_width > 0.0 {
        if let Some(b) = to_skia(&c.bold).and_then(|b| stroked(&b, c.bold_width)) {
            skia_svg(&b, &mut out);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::fmt;

    #[test]
    fn numbers_are_compact() {
        assert_eq!(fmt(12.0), "12");
        assert_eq!(fmt(12.345), "12.35");
        assert_eq!(fmt(-0.001), "0");
        assert_eq!(fmt(0.5), "0.5");
    }
}
