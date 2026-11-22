//! Photoshop's warp styles (Edit > Transform > Warp, and Warp Text) as one shared map, so
//! raster and placed layers (`transform warp --preset`) and live text bend identically.
//!
//! A preset maps a point of a box (`x` along the length, `y` across, in px) to where it
//! goes. Parameters, as in Photoshop: `bend` −100..100 (the style's strength and
//! direction), `h_distort` and `v_distort` −100..100 (a perspective that makes one end
//! recede), and the orientation.
//!
//! * Horizontal orientation: the length runs left → right; a positive bend lifts the
//!   middle (arc, arch, bulge, ...) toward the top.
//! * Vertical orientation: the preset is turned a quarter turn clockwise, as if applied to
//!   text running top → bottom whose letter tops face right: the length runs top → bottom
//!   and a positive bend pushes toward the right.
//! * A positive `h_distort` makes the left end recede (smaller), a negative one the right
//!   end; a positive `v_distort` makes the top recede, a negative one the bottom. The
//!   receding edge shrinks to `1 − 0.85 |d|` of its size, with perspective foreshortening.
//!
//! The styles (positive bend; a negative bend mirrors the motion):
//! `arc` (a band bent around a circle: the ends turn radially), `arc-lower` (only the
//! bottom bows down), `arc-upper` (only the top bows up), `arch` (the whole band rises in
//! the middle, sides stay vertical), `bulge` (top up and bottom down: a lens), `shell-lower`
//! (the bottom bows down while its corners rise: a fan), `shell-upper` (the top bows up
//! while its corners drop), `flag` (one full sine wave, both edges together), `wave`
//! (a sine wave whose top and bottom are out of phase, so the thickness changes), `fish`
//! (swells then pinches toward a flared tail), `rise` (an S-curve: the left end low, the
//! right end high), `fisheye` (a magnifying bulge at the center), `inflate` (the whole
//! band puffs out on every side), `squeeze` (pinched in from all four sides), and `twist`
//! (the ends rotate in opposite directions about the center line).

use crate::error::{Error, Result};
use std::f64::consts::PI;

/// The preset names in Photoshop's order.
pub const PRESETS: &[&str] = &[
    "arc",
    "arc-lower",
    "arc-upper",
    "arch",
    "bulge",
    "shell-lower",
    "shell-upper",
    "flag",
    "wave",
    "fish",
    "rise",
    "fisheye",
    "inflate",
    "squeeze",
    "twist",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Style {
    Arc,
    ArcLower,
    ArcUpper,
    Arch,
    Bulge,
    ShellLower,
    ShellUpper,
    Flag,
    Wave,
    Fish,
    Rise,
    Fisheye,
    Inflate,
    Squeeze,
    Twist,
}

impl Style {
    pub fn parse(s: &str) -> Result<Style> {
        Ok(match s.trim().to_ascii_lowercase().as_str() {
            "arc" => Style::Arc,
            "arc-lower" => Style::ArcLower,
            "arc-upper" => Style::ArcUpper,
            "arch" => Style::Arch,
            "bulge" => Style::Bulge,
            "shell-lower" => Style::ShellLower,
            "shell-upper" => Style::ShellUpper,
            "flag" => Style::Flag,
            "wave" => Style::Wave,
            "fish" => Style::Fish,
            "rise" => Style::Rise,
            "fisheye" => Style::Fisheye,
            "inflate" => Style::Inflate,
            "squeeze" => Style::Squeeze,
            "twist" => Style::Twist,
            other => {
                return Err(Error::new("PX1010", format!("'{other}' is not a warp preset"))
                    .with_hint(format!("use one of: {}", PRESETS.join(", ")))
                    .with_context("preset", other))
            }
        })
    }

    pub fn name(self) -> &'static str {
        PRESETS[self as usize]
    }
}

/// A warp preset with its settings (bend and distortions as fractions −1..1).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Warp {
    pub style: Style,
    pub bend: f64,
    pub h_distort: f64,
    pub v_distort: f64,
    pub vertical: bool,
}

impl Warp {
    /// From the command-line values (−100..100 each).
    pub fn new(style: Style, bend: f64, h_distort: f64, v_distort: f64, vertical: bool) -> Warp {
        let f = |v: f64| (v / 100.0).clamp(-1.0, 1.0);
        Warp { style, bend: f(bend), h_distort: f(h_distort), v_distort: f(v_distort), vertical }
    }

    /// Where the point (`x`, `y`) of a `w`×`h` box (origin at its top-left) goes, in the
    /// same coordinates. Points may land outside the box.
    pub fn map(&self, x: f64, y: f64, w: f64, h: f64) -> (f64, f64) {
        if self.vertical {
            // a quarter turn clockwise: length runs down, the "top" side faces right
            let (s, t) = (y, w - x);
            let (s2, t2) = self.map_h(s, t, h, w);
            (w - t2, s2)
        } else {
            self.map_h(x, y, w, h)
        }
    }

    /// The horizontal map: `len` along x, `thick` across y.
    fn map_h(&self, x: f64, y: f64, len: f64, thick: f64) -> (f64, f64) {
        let (len, thick) = (len.max(1e-9), thick.max(1e-9));
        let (x1, y1) = bend(self.style, self.bend, x, y, len, thick);
        distort(x1, y1, len, thick, self.h_distort, self.v_distort)
    }
}

/// The bend of one style, horizontal orientation.
fn bend(style: Style, b: f64, x: f64, y: f64, w: f64, h: f64) -> (f64, f64) {
    if b == 0.0 {
        return (x, y);
    }
    let s = x / w; // 0..1 along
    let t = y / h; // 0..1 across (0 = top)
    let c = 2.0 * s - 1.0; // −1..1
    let hump = 1.0 - c * c; // 0 at the ends, 1 in the middle
    match style {
        Style::Arc => {
            // a band bent around a circle; the middle column stays put
            let theta = b.abs() * PI;
            let r_mid = w / theta;
            let (tt, flip) = if b > 0.0 { (t, false) } else { (1.0 - t, true) };
            let phi = (s - 0.5) * theta;
            let r = (r_mid + (0.5 - tt) * h).max(0.0);
            let nx = w / 2.0 + r * phi.sin();
            let ny = h / 2.0 + r_mid - r * phi.cos();
            (nx, if flip { h - ny } else { ny })
        }
        Style::ArcLower => (x, y + t * b * 0.75 * h * hump),
        Style::ArcUpper => (x, y - (1.0 - t) * b * 0.75 * h * hump),
        Style::Arch => (x, y - b * 0.6 * h * hump),
        Style::Bulge => (x, y + (t - 0.5) * 2.0 * b * 0.45 * h * hump),
        Style::ShellLower => (x, y + t * b * 0.75 * h * (hump - 0.6 * c * c)),
        Style::ShellUpper => (x, y - (1.0 - t) * b * 0.75 * h * (hump - 0.6 * c * c)),
        Style::Flag => (x, y - b * 0.4 * h * (2.0 * PI * s).sin()),
        Style::Wave => (x, y - b * 0.4 * h * (2.0 * PI * s - t * PI / 2.0).sin()),
        Style::Fish => (x, y + (t - 0.5) * 2.0 * b * 0.45 * h * (2.0 * PI * s).sin()),
        Style::Rise => (x, y + b * 0.5 * h * (PI * s).cos()),
        Style::Fisheye | Style::Inflate => {
            let (dx, dy) = (c, 2.0 * t - 1.0);
            let rho2 = (dx * dx + dy * dy) / 2.0; // 0 at the center, 1 at the corners
            let f = if style == Style::Fisheye {
                0.9 * (1.0 - rho2).max(0.0).powi(2)
            } else {
                0.6 * (1.0 - rho2).max(0.0).sqrt()
            };
            (x + dx * w / 2.0 * b * f, y + dy * h / 2.0 * b * f)
        }
        Style::Squeeze => {
            let v = 2.0 * t - 1.0;
            (x - c * w / 2.0 * b * 0.35 * (1.0 - v * v), y - v * h / 2.0 * b * 0.35 * hump)
        }
        Style::Twist => {
            let a = b * PI / 3.0 * c;
            let d = (t - 0.5) * h;
            (x - a.sin() * d, h / 2.0 + a.cos() * d)
        }
    }
}

/// Perspective: `hd` > 0 shrinks the left edge, < 0 the right; `vd` > 0 the top, < 0 the
/// bottom. A projective map of the box onto that trapezoid, applied to (x, y).
fn distort(x: f64, y: f64, w: f64, h: f64, hd: f64, vd: f64) -> (f64, f64) {
    let (mut u, mut v) = (x / w, y / h);
    if hd != 0.0 {
        let k = 0.85 * hd.abs();
        let q = if hd > 0.0 {
            [(0.0, k / 2.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0 - k / 2.0)]
        } else {
            [(0.0, 0.0), (1.0, k / 2.0), (1.0, 1.0 - k / 2.0), (0.0, 1.0)]
        };
        (u, v) = square_to_quad(&q, u, v);
    }
    if vd != 0.0 {
        let k = 0.85 * vd.abs();
        let q = if vd > 0.0 {
            [(k / 2.0, 0.0), (1.0 - k / 2.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
        } else {
            [(0.0, 0.0), (1.0, 0.0), (1.0 - k / 2.0, 1.0), (k / 2.0, 1.0)]
        };
        (u, v) = square_to_quad(&q, u, v);
    }
    (u * w, v * h)
}

/// The projective map of the unit square onto the quad `q` (top-left, top-right,
/// bottom-right, bottom-left), after Heckbert.
fn square_to_quad(q: &[(f64, f64); 4], u: f64, v: f64) -> (f64, f64) {
    let [(x0, y0), (x1, y1), (x2, y2), (x3, y3)] = *q;
    let (dx1, dx2, sx) = (x1 - x2, x3 - x2, x0 - x1 + x2 - x3);
    let (dy1, dy2, sy) = (y1 - y2, y3 - y2, y0 - y1 + y2 - y3);
    let (g, hh) = if sx.abs() < 1e-12 && sy.abs() < 1e-12 {
        (0.0, 0.0)
    } else {
        let den = dx1 * dy2 - dx2 * dy1;
        ((sx * dy2 - dx2 * sy) / den, (dx1 * sy - sx * dy1) / den)
    };
    let a = x1 - x0 + g * x1;
    let b = x3 - x0 + hh * x3;
    let d = y1 - y0 + g * y1;
    let e = y3 - y0 + hh * y3;
    let den = (g * u + hh * v + 1.0).max(1e-6);
    ((a * u + b * v + x0) / den, (d * u + e * v + y0) / den)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: (f64, f64), b: (f64, f64)) -> bool {
        (a.0 - b.0).abs() < 1e-6 && (a.1 - b.1).abs() < 1e-6
    }

    #[test]
    fn names_parse_and_round_trip() {
        for n in PRESETS {
            assert_eq!(Style::parse(n).unwrap().name(), *n);
        }
        assert_eq!(Style::parse("spiral").unwrap_err().code, "PX1010");
    }

    #[test]
    fn zero_bend_is_identity() {
        for n in PRESETS {
            let w = Warp::new(Style::parse(n).unwrap(), 0.0, 0.0, 0.0, false);
            for &(x, y) in &[(0.0, 0.0), (37.0, 12.0), (200.0, 50.0)] {
                assert!(close(w.map(x, y, 200.0, 50.0), (x, y)), "{n}");
            }
            let v = Warp { vertical: true, ..w };
            assert!(close(v.map(13.0, 77.0, 50.0, 200.0), (13.0, 77.0)), "{n} vertical");
        }
    }

    #[test]
    fn positive_bends_lift_the_middle() {
        let (w, h) = (400.0, 100.0);
        let up = |n: &str| Warp::new(Style::parse(n).unwrap(), 50.0, 0.0, 0.0, false).map(200.0, 0.0, w, h).1;
        assert!(up("arch") < -1.0 && up("arc-upper") < -1.0 && up("bulge") < -1.0);
        // arc keeps the middle column and drops the ends
        assert!(Warp::new(Style::Arc, 50.0, 0.0, 0.0, false).map(0.0, 0.0, w, h).1 > 1.0);
        // arc-lower keeps the top edge, moves the bottom down
        let al = Warp::new(Style::ArcLower, 50.0, 0.0, 0.0, false);
        assert!(close(al.map(200.0, 0.0, w, h), (200.0, 0.0)));
        assert!(al.map(200.0, 100.0, w, h).1 > 101.0);
        // negative bends mirror
        let neg = Warp::new(Style::Arc, -50.0, 0.0, 0.0, false).map(0.0, 100.0, w, h).1;
        assert!(neg < 99.0, "{neg}");
        assert!(Warp::new(Style::Arch, -50.0, 0.0, 0.0, false).map(200.0, 0.0, w, h).1 > 1.0);
        // arc: the ends turn radially (the outer, top corner reaches farther out)
        let arc = Warp::new(Style::Arc, 60.0, 0.0, 0.0, false);
        let (tl, bl) = (arc.map(0.0, 0.0, w, h), arc.map(0.0, h, w, h));
        assert!(tl.0 < bl.0 && tl.1 < bl.1, "{tl:?} {bl:?}");
    }

    #[test]
    fn vertical_orientation_bends_right() {
        let v = Warp::new(Style::Arch, 50.0, 0.0, 0.0, true);
        let (x, _) = v.map(50.0, 200.0, 100.0, 400.0);
        assert!(x > 51.0, "{x}");
    }

    #[test]
    fn distortion_recedes_one_end() {
        let w = Warp::new(Style::Flag, 0.0, 50.0, 0.0, false);
        let left = w.map(0.0, 100.0, 400.0, 100.0).1 - w.map(0.0, 0.0, 400.0, 100.0).1;
        let right = w.map(400.0, 100.0, 400.0, 100.0).1 - w.map(400.0, 0.0, 400.0, 100.0).1;
        assert!(left < 70.0 && (right - 100.0).abs() < 1e-6, "{left} {right}");
        let v = Warp::new(Style::Flag, 0.0, 0.0, -50.0, false);
        let bottom = v.map(400.0, 100.0, 400.0, 100.0).0 - v.map(0.0, 100.0, 400.0, 100.0).0;
        assert!(bottom < 300.0);
    }
}
