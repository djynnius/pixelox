//! Photoshop's warp presets as one shared map (ADR 0015, ADR 0016), so live text
//! (`text warp`, which bends glyph outlines in pixelox-text) and pixels (`transform warp
//! --preset` on raster, placed, and shape layers) take the same shape for the same preset,
//! `--bend`, `--h-distort`, `--v-distort`, and `--orient`. Pure math with `libm`
//! trigonometry, so the shape is identical on every platform.
//!
//! Every preset is a map of a bounding box `[x, y, w, h]` onto the plane. With `u` = 0..1 left to right, `v` = 0..1 top to bottom,
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
//! The map works on a box `[x, y, w, h]`: text uses the ink bounds of the unwarped glyphs,
//! `transform warp` the layer's visible bounds.

use kurbo::Point;
use serde_json::{json, Value};
use std::f64::consts::PI;

/// The preset names, in Photoshop's order.
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
pub enum Preset {
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

impl Preset {
    pub fn parse(s: &str) -> Option<Preset> {
        let n: String = s.trim().to_ascii_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect();
        Some(match n.as_str() {
            "arc" => Preset::Arc,
            "arclower" => Preset::ArcLower,
            "arcupper" => Preset::ArcUpper,
            "arch" => Preset::Arch,
            "bulge" => Preset::Bulge,
            "shelllower" => Preset::ShellLower,
            "shellupper" => Preset::ShellUpper,
            "flag" => Preset::Flag,
            "wave" => Preset::Wave,
            "fish" => Preset::Fish,
            "rise" => Preset::Rise,
            "fisheye" => Preset::Fisheye,
            "inflate" => Preset::Inflate,
            "squeeze" => Preset::Squeeze,
            "twist" => Preset::Twist,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        PRESETS[self as usize]
    }
}

/// A warp: preset, bend, distortions (all -100..100), and orientation.
#[derive(Clone, Debug, PartialEq)]
pub struct Warp {
    pub preset: Preset,
    pub bend: f64,
    pub h_distort: f64,
    pub v_distort: f64,
    pub vertical: bool,
}

fn c(t: f64) -> f64 {
    let k = 2.0 * t - 1.0;
    1.0 - k * k
}

/// Horizontal position after magnifying the middle by `1 + a·profile(u)` (box width kept):
/// the normalized integral of the profile, parabolic `c(u)` or semicircular `sqrt(c(u))`.
fn magnify(u: f64, a: f64, round: bool) -> f64 {
    if round {
        let x = (2.0 * u - 1.0).clamp(-1.0, 1.0);
        let g = 0.25 * (x * (1.0 - x * x).max(0.0).sqrt() + libm::asin(x) + PI / 2.0);
        (u + a * g) / (1.0 + a * PI / 4.0)
    } else {
        (u + a * (2.0 * u * u - 4.0 * u * u * u / 3.0)) / (1.0 + 2.0 * a / 3.0)
    }
}

impl Warp {
    pub fn to_json(&self) -> Value {
        json!({
            "preset": self.preset.name(),
            "bend": self.bend,
            "h_distort": self.h_distort,
            "v_distort": self.v_distort,
            "orient": if self.vertical { "vertical" } else { "horizontal" },
        })
    }

    pub fn from_json(v: &Value) -> Option<Warp> {
        let preset = Preset::parse(v.get("preset")?.as_str()?)?;
        let n =
            |k: &str| v.get(k).and_then(|x| x.as_f64()).filter(|x| x.is_finite()).unwrap_or(0.0).clamp(-100.0, 100.0);
        Some(Warp {
            preset,
            bend: n("bend"),
            h_distort: n("h_distort"),
            v_distort: n("v_distort"),
            vertical: v.get("orient").and_then(|o| o.as_str()) == Some("vertical"),
        })
    }

    /// Whether the warp changes anything.
    pub fn is_identity(&self) -> bool {
        self.bend.abs() < 1e-9 && self.h_distort.abs() < 1e-9 && self.v_distort.abs() < 1e-9
    }

    /// Map point `p` of the box `[x, y, w, h]`.
    pub fn map(&self, rect: [f64; 4], p: Point) -> Point {
        let [x0, y0, w, h] = rect;
        if w <= 0.0 || h <= 0.0 {
            return p;
        }
        let (lx, ly) = (p.x - x0, p.y - y0);
        let (lx, ly, w, h) = if self.vertical { (ly, lx, h, w) } else { (lx, ly, w, h) };
        let (mx, my) = self.style_map(lx / w, ly / h, w, h);
        let (mx, my) = self.distort(mx / w, my / h, w, h);
        let (mx, my) = if self.vertical { (my, mx) } else { (mx, my) };
        Point::new(x0 + mx, y0 + my)
    }

    /// The preset itself, in box px (horizontal orientation).
    fn style_map(&self, u: f64, v: f64, w: f64, h: f64) -> (f64, f64) {
        let b = self.bend / 100.0;
        let (x, y) = (u * w, v * h);
        match self.preset {
            Preset::Arc => {
                let theta = b.abs() * PI;
                if theta < 1e-9 {
                    return (x, y);
                }
                let r0 = w / theta;
                let phi = (u - 0.5) * theta;
                let (s, co) = (libm::sin(phi), libm::cos(phi));
                if b > 0.0 {
                    let r = r0 + (0.5 - v) * h;
                    (w / 2.0 + r * s, h / 2.0 + r0 - r * co)
                } else {
                    let r = r0 + (v - 0.5) * h;
                    (w / 2.0 + r * s, h / 2.0 - r0 + r * co)
                }
            }
            Preset::ArcLower => (x, y + b * h * v * c(u)),
            Preset::ArcUpper => (x, y - b * h * (1.0 - v) * c(u)),
            Preset::Arch => (x, y - b * h * c(u)),
            Preset::Bulge => (x, y + b * h * (v - 0.5) * c(u)),
            Preset::ShellLower => (w / 2.0 + (u - 0.5) * w * (1.0 + 0.6 * b * v), v * h * (1.0 + b * c(u))),
            Preset::ShellUpper => {
                (w / 2.0 + (u - 0.5) * w * (1.0 + 0.6 * b * (1.0 - v)), h - (1.0 - v) * h * (1.0 + b * c(u)))
            }
            Preset::Flag => (x, y - b * h / 2.0 * libm::sin(2.0 * PI * u)),
            Preset::Wave => (x, y - b * h / 2.0 * libm::sin(2.0 * PI * u + PI / 2.0 * v)),
            Preset::Fish => (x, y + (v - 0.5) * h * b * 0.8 * libm::sin(1.5 * PI * u)),
            Preset::Rise => (x, y - b * h / 2.0 * libm::sin(PI * (u - 0.5))),
            Preset::Fisheye => {
                // the middle letters grow in both directions, the ends shrink; the box stays
                let a = 0.6 * b;
                (w * magnify(u, a, false), h / 2.0 + (y - h / 2.0) * (1.0 + a * c(u)))
            }
            Preset::Inflate => {
                // a rounder, stronger swell than fisheye: the word puffs up like a balloon
                let r = c(u).max(0.0).sqrt();
                (w * magnify(u, 0.4 * b, true), h / 2.0 + (y - h / 2.0) * (1.0 + 0.8 * b * r))
            }
            Preset::Squeeze => {
                // the middle letters are pressed flat: wider and shorter
                (w * magnify(u, 0.5 * b, false), h / 2.0 + (y - h / 2.0) * (1.0 - 0.5 * b * c(u)))
            }
            Preset::Twist => {
                // a vortex: the middle turns by up to b x 90°, the ends stay
                let (px, py) = (x - w / 2.0, y - h / 2.0);
                let r = ((px * px + py * py).sqrt() / (w.max(h) / 2.0)).min(1.0);
                let a = b * PI / 2.0 * (1.0 - r) * (1.0 - r);
                let (s, co) = (libm::sin(a), libm::cos(a));
                (w / 2.0 + px * co - py * s, h / 2.0 + px * s + py * co)
            }
        }
    }

    /// Horizontal / vertical distortion: a perspective of the box (unit coordinates in).
    fn distort(&self, u: f64, v: f64, w: f64, h: f64) -> (f64, f64) {
        let (hd, vd) = (self.h_distort / 100.0 * 0.6, self.v_distort / 100.0 * 0.6);
        if hd.abs() < 1e-12 && vd.abs() < 1e-12 {
            return (u * w, v * h);
        }
        let (cx, cy) = (w / 2.0, h / 2.0);
        // corners: TL, TR, BR, BL
        let q = [
            (cx - cx * (1.0 - vd), cy - cy * (1.0 - hd)),
            (cx + cx * (1.0 - vd), cy - cy * (1.0 + hd)),
            (cx + cx * (1.0 + vd), cy + cy * (1.0 + hd)),
            (cx - cx * (1.0 + vd), cy + cy * (1.0 - hd)),
        ];
        homography(q, u, v)
    }
}

/// Projective map of the unit square onto quad `q` (TL, TR, BR, BL), Heckbert's form.
fn homography(q: [(f64, f64); 4], u: f64, v: f64) -> (f64, f64) {
    let [(x0, y0), (x1, y1), (x2, y2), (x3, y3)] = q;
    let (dx1, dx2, dx3) = (x1 - x2, x3 - x2, x0 - x1 + x2 - x3);
    let (dy1, dy2, dy3) = (y1 - y2, y3 - y2, y0 - y1 + y2 - y3);
    let (a, b, cc, d, e, f, g, hh);
    if dx3.abs() < 1e-12 && dy3.abs() < 1e-12 {
        a = x1 - x0;
        b = x2 - x1;
        cc = x0;
        d = y1 - y0;
        e = y2 - y1;
        f = y0;
        g = 0.0;
        hh = 0.0;
    } else {
        let den = dx1 * dy2 - dx2 * dy1;
        g = (dx3 * dy2 - dx2 * dy3) / den;
        hh = (dx1 * dy3 - dx3 * dy1) / den;
        a = x1 - x0 + g * x1;
        b = x3 - x0 + hh * x3;
        cc = x0;
        d = y1 - y0 + g * y1;
        e = y3 - y0 + hh * y3;
        f = y0;
    }
    let wq = g * u + hh * v + 1.0;
    let wq = if wq.abs() < 1e-9 { 1e-9 } else { wq };
    ((a * u + b * v + cc) / wq, (d * u + e * v + f) / wq)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(p: Preset, bend: f64) -> Warp {
        Warp { preset: p, bend, h_distort: 0.0, v_distort: 0.0, vertical: false }
    }

    const R: [f64; 4] = [100.0, 50.0, 400.0, 100.0];

    fn at(wp: &Warp, u: f64, v: f64) -> Point {
        wp.map(R, Point::new(R[0] + u * R[2], R[1] + v * R[3]))
    }

    #[test]
    fn names_roundtrip() {
        for (i, n) in PRESETS.iter().enumerate() {
            let p = Preset::parse(n).unwrap();
            assert_eq!(p as usize, i);
            assert_eq!(p.name(), *n);
        }
        assert_eq!(Preset::parse("Arc Lower"), Some(Preset::ArcLower));
        assert_eq!(Preset::parse("bogus"), None);
    }

    #[test]
    fn zero_bend_is_identity() {
        for n in PRESETS {
            let wp = w(Preset::parse(n).unwrap(), 0.0);
            for (u, v) in [(0.0, 0.0), (0.3, 0.7), (1.0, 1.0), (0.5, 0.5)] {
                let p = at(&wp, u, v);
                let q = Point::new(R[0] + u * R[2], R[1] + v * R[3]);
                assert!(p.distance(q) < 1e-6, "{n} at {u},{v}: {p:?}");
            }
        }
    }

    #[test]
    fn presets_move_the_right_edges() {
        // arc: the middle of the top edge stays, the ends drop (positive bend = frown)
        let a = w(Preset::Arc, 50.0);
        assert!(at(&a, 0.5, 0.0).distance(Point::new(300.0, 50.0)) < 1e-6);
        assert!(at(&a, 0.0, 0.0).y > 60.0 && at(&a, 1.0, 0.0).y > 60.0);
        // negative bend mirrors it (smile)
        let s = w(Preset::Arc, -50.0);
        assert!(at(&s, 0.0, 1.0).y < 140.0);
        // arc-lower: the top edge does not move, the bottom middle sags by b*h
        let l = w(Preset::ArcLower, 50.0);
        assert!((at(&l, 0.5, 0.0).y - 50.0).abs() < 1e-9);
        assert!((at(&l, 0.5, 1.0).y - 200.0).abs() < 1e-9);
        // arch lifts columns without tilting them
        let ar = w(Preset::Arch, 40.0);
        let (t, b) = (at(&ar, 0.3, 0.0), at(&ar, 0.3, 1.0));
        assert!((t.x - b.x).abs() < 1e-9 && (b.y - t.y - 100.0).abs() < 1e-9);
        // flag is periodic: ends unmoved
        let f = w(Preset::Flag, 80.0);
        assert!((at(&f, 0.0, 0.5).y - 100.0).abs() < 1e-9 && (at(&f, 1.0, 0.5).y - 100.0).abs() < 1e-6);
        // rise climbs to the right
        let r = w(Preset::Rise, 50.0);
        assert!(at(&r, 1.0, 0.5).y < at(&r, 0.0, 0.5).y);
        // fisheye magnifies the middle and keeps the corners
        let fe = w(Preset::Fisheye, 100.0);
        assert!(at(&fe, 1.0, 1.0).distance(Point::new(500.0, 150.0)) < 1e-6);
        assert!(at(&fe, 0.75, 0.5).x > 400.0);
        // twist keeps the center
        let tw = w(Preset::Twist, 100.0);
        assert!(at(&tw, 0.5, 0.5).distance(Point::new(300.0, 100.0)) < 1e-9);
    }

    #[test]
    fn distortion_and_orientation() {
        let mut d = w(Preset::Arc, 0.0);
        d.h_distort = 100.0;
        // the right edge grows, the left shrinks
        let (rt, rb) = (at(&d, 1.0, 0.0), at(&d, 1.0, 1.0));
        let (lt, lb) = (at(&d, 0.0, 0.0), at(&d, 0.0, 1.0));
        assert!(rb.y - rt.y > 150.0 && lb.y - lt.y < 50.0);
        // a vertical arc bends a column sideways
        let v = Warp { preset: Preset::ArcLower, bend: 50.0, h_distort: 0.0, v_distort: 0.0, vertical: true };
        let p = v.map(R, Point::new(R[0] + R[2], R[1] + R[3] / 2.0));
        assert!(p.x > R[0] + R[2] + 1.0);
    }

    #[test]
    fn json_round_trip() {
        let json = w(Preset::ShellLower, -20.0).to_json();
        assert_eq!(Warp::from_json(&json), Some(w(Preset::ShellLower, -20.0)));
    }
}
