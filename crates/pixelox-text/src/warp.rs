//! Text warp presets (Photoshop's Warp Text list), applied to glyph outlines so warped type
//! stays vector and editable (ADR 0015).
//!
//! Every preset is a map of the text's bounding box `[x, y, w, h]` (the ink of the unwarped
//! glyphs) onto the plane. With `u` = 0..1 left to right, `v` = 0..1 top to bottom,
//! `b` = bend / 100 (-1..1), and `c(t) = 1 - (2t - 1)^2` (0 at the ends, 1 in the middle),
//! offsets in box units are (positive `b`):
//!
//! | preset | effect |
//! | --- | --- |
//! | `arc` | the box bends into a ring sector of `b x 180°`; the middle line keeps its length |
//! | `arc-lower` | top stays straight; bottom sags: `y += b·h·v·c(u)` |
//! | `arc-upper` | bottom stays straight; top rises: `y -= b·h·(1-v)·c(u)` |
//! | `arch` | every column lifts without tilting: `y -= b·h·c(u)` |
//! | `bulge` | top rises and bottom sags in the middle: `y += b·h·(v-½)·c(u)` |
//! | `shell-lower` | fan: top straight, bottom spreads (`x` by up to 60%) and sags (`b·h·c(u)`) |
//! | `shell-upper` | the same, mirrored: bottom straight, top spreads and rises |
//! | `flag` | one sine period, columns stay upright: `y -= b·h/2·sin(2πu)` |
//! | `wave` | like flag, with the bottom a quarter period ahead: `y -= b·h/2·sin(2πu + πv/2)` |
//! | `fish` | height swells on the left and narrows to the right: `y += (v-½)·h·0.8b·sin(1.5πu)` |
//! | `rise` | an S-shaped climb to the right: `y -= b·h/2·sin(π(u-½))` |
//! | `fisheye` | the middle letters grow in both directions: width `1 + 0.6b·c(u)`, height the same; the box keeps its size |
//! | `inflate` | a rounder, stronger swell: height `1 + 0.8b·√c(u)`, width `1 + 0.4b·√c(u)` |
//! | `squeeze` | the middle letters are pressed flat: width `1 + 0.5b·c(u)`, height `1 - 0.5b·c(u)` |
//! | `twist` | a vortex: rotation about the center by `b·90°·(1-r)²`, r = distance / half the longer side |
//!
//! Negative bends mirror the effect (`arc` -50 is a smile instead of a frown).
//! `h_distort` (-100..100) then applies a perspective that grows the right side (left side
//! for negative values, up to ±60% at 100), `v_distort` one that grows the bottom (top).
//! `orient vertical` applies the preset along the vertical axis (x and y swapped).
//!
//! The preset math lives in `pixelox_core::warp_preset` (shared with `transform warp
//! --preset` on pixels, so both take the same shape); this module bends glyph paths with it.

use kurbo::{BezPath, CubicBez, ParamCurve, PathEl, Point, QuadBez};
use pixelox_core::prelude::*;

pub use pixelox_core::warp_preset::{Preset, Warp, PRESETS};

/// Pieces per segment so that each is at most `step` px long.
fn pieces(len: f64, step: f64) -> usize {
    ((len / step).ceil() as usize).clamp(1, 512)
}

/// Warp a path: every segment is subdivided into short pieces whose points are mapped, so
/// straight stems bend smoothly; curves stay curves.
pub fn warp_path(path: &BezPath, warp: &Warp, rect: [f64; 4]) -> BezPath {
    let step = (rect[2].min(rect[3]) / 48.0).clamp(0.75, 4.0);
    let m = |p: Point| warp.map(rect, p);
    let mut out = BezPath::new();
    let mut cur = Point::ZERO;
    let mut start = Point::ZERO;
    for el in path.elements() {
        match *el {
            PathEl::MoveTo(p) => {
                out.move_to(m(p));
                cur = p;
                start = p;
            }
            PathEl::LineTo(p) => {
                let n = pieces(cur.distance(p), step);
                for i in 1..=n {
                    out.line_to(m(cur.lerp(p, i as f64 / n as f64)));
                }
                cur = p;
            }
            PathEl::QuadTo(p1, p2) => {
                let q = QuadBez::new(cur, p1, p2);
                let n = pieces(cur.distance(p1) + p1.distance(p2), step);
                for i in 0..n {
                    let s = q.subsegment(i as f64 / n as f64..(i + 1) as f64 / n as f64);
                    out.quad_to(m(s.p1), m(s.p2));
                }
                cur = p2;
            }
            PathEl::CurveTo(p1, p2, p3) => {
                let cb = CubicBez::new(cur, p1, p2, p3);
                let n = pieces(cur.distance(p1) + p1.distance(p2) + p2.distance(p3), step);
                for i in 0..n {
                    let s = cb.subsegment(i as f64 / n as f64..(i + 1) as f64 / n as f64);
                    out.curve_to(m(s.p1), m(s.p2), m(s.p3));
                }
                cur = p3;
            }
            PathEl::ClosePath => {
                if cur.distance(start) > 1e-9 {
                    let n = pieces(cur.distance(start), step);
                    for i in 1..n {
                        out.line_to(m(cur.lerp(start, i as f64 / n as f64)));
                    }
                }
                out.close_path();
                cur = start;
            }
        }
    }
    out
}

/// Parse `--warp` / `--preset` (none clears).
pub fn parse_preset(s: &str) -> Result<Option<Preset>> {
    let t = s.trim();
    if t.is_empty() || t.eq_ignore_ascii_case("none") || t.eq_ignore_ascii_case("off") {
        return Ok(None);
    }
    Preset::parse(t).map(Some).ok_or_else(|| {
        Error::usage(format!("'{t}' is not a warp preset"))
            .with_hint(format!("use one of: {}, or none", PRESETS.join(", ")))
            .with_context("warp", t)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(p: Preset, bend: f64) -> Warp {
        Warp { preset: p, bend, h_distort: 0.0, v_distort: 0.0, vertical: false }
    }

    const R: [f64; 4] = [100.0, 50.0, 400.0, 100.0];

    #[test]
    fn presets_parse() {
        assert!(parse_preset("none").unwrap().is_none());
        assert!(parse_preset("bogus").is_err());
        assert_eq!(parse_preset("flag").unwrap(), Some(Preset::Flag));
    }

    #[test]
    fn paths_are_subdivided() {
        let mut p = BezPath::new();
        p.move_to((100.0, 50.0));
        p.line_to((500.0, 50.0));
        p.line_to((500.0, 150.0));
        p.close_path();
        let out = warp_path(&p, &w(Preset::Flag, 50.0), R);
        assert!(out.elements().len() > 50);
    }
}
