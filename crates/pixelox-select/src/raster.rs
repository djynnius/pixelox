//! Anti-aliased path rasterization into canvas-sized masks, through tiny-skia.

use kurbo::{BezPath, PathEl, Shape};
use pixelox_core::prelude::*;

/// Parse SVG path data (canvas pixels) into a path (PX1010 when invalid).
pub fn parse_path(d: &str) -> Result<BezPath> {
    BezPath::from_svg(d).map_err(|e| {
        Error::new("PX1010", format!("invalid SVG path data: {e}"))
            .with_hint("use absolute or relative SVG commands, e.g. --d 'M 10 10 L 200 10 L 120 160 Z'")
            .with_context("d", d)
    })
}

/// A closed polygon through `pts`.
pub fn polygon(pts: &[(f64, f64)]) -> BezPath {
    let mut p = BezPath::new();
    for (i, &(x, y)) in pts.iter().enumerate() {
        if i == 0 {
            p.move_to((x, y));
        } else {
            p.line_to((x, y));
        }
    }
    p.close_path();
    p
}

pub fn rect_path(x: f64, y: f64, w: f64, h: f64) -> BezPath {
    kurbo::Rect::new(x, y, x + w, y + h).to_path(0.01)
}

pub fn ellipse_path(x: f64, y: f64, w: f64, h: f64) -> BezPath {
    kurbo::Ellipse::from_rect(kurbo::Rect::new(x, y, x + w, y + h)).to_path(0.01)
}

fn to_skia(path: &BezPath) -> Option<tiny_skia::Path> {
    let mut pb = tiny_skia::PathBuilder::new();
    for el in path.elements() {
        match *el {
            PathEl::MoveTo(p) => pb.move_to(p.x as f32, p.y as f32),
            PathEl::LineTo(p) => pb.line_to(p.x as f32, p.y as f32),
            PathEl::QuadTo(a, p) => pb.quad_to(a.x as f32, a.y as f32, p.x as f32, p.y as f32),
            PathEl::CurveTo(a, b, p) => {
                pb.cubic_to(a.x as f32, a.y as f32, b.x as f32, b.y as f32, p.x as f32, p.y as f32)
            }
            PathEl::ClosePath => pb.close(),
        }
    }
    pb.finish()
}

/// Fill `path` into a `w`×`h` mask with anti-aliasing (8-bit coverage, like selections).
/// Open subpaths are closed implicitly; empty or degenerate paths give an empty mask.
pub fn fill_mask(w: u32, h: u32, path: &BezPath, even_odd: bool) -> Mask {
    let mut out = Mask::new(w, h);
    let (Some(mut m), Some(p)) = (tiny_skia::Mask::new(w, h), to_skia(path)) else {
        return out;
    };
    let rule = if even_odd { tiny_skia::FillRule::EvenOdd } else { tiny_skia::FillRule::Winding };
    m.fill_path(&p, rule, true, tiny_skia::Transform::identity());
    for (o, &b) in out.data.iter_mut().zip(m.data()) {
        *o = b as f32 / 255.0;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_is_exact_on_pixel_edges() {
        let m = fill_mask(20, 20, &rect_path(5.0, 5.0, 10.0, 6.0), false);
        assert_eq!(m.get(5, 5), 1.0);
        assert_eq!(m.get(14, 10), 1.0);
        assert_eq!(m.get(15, 10), 0.0);
        assert_eq!(m.get(4, 5), 0.0);
        assert_eq!(m.data.iter().map(|&v| v as f64).sum::<f64>(), 60.0);
    }

    #[test]
    fn half_pixel_edges_are_anti_aliased() {
        let m = fill_mask(10, 10, &rect_path(2.5, 0.0, 5.0, 10.0), false);
        assert!((m.get(2, 4) - 0.5).abs() < 0.05, "{}", m.get(2, 4));
        assert_eq!(m.get(4, 4), 1.0);
    }

    #[test]
    fn even_odd_makes_holes() {
        let mut p = rect_path(0.0, 0.0, 10.0, 10.0);
        p.extend(rect_path(3.0, 3.0, 4.0, 4.0));
        assert_eq!(fill_mask(10, 10, &p, true).get(5, 5), 0.0);
        assert_eq!(fill_mask(10, 10, &p, false).get(5, 5), 1.0);
        assert!(parse_path("nope").is_err());
        assert!(fill_mask(0, 5, &p, false).data.is_empty());
    }
}
