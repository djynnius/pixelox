//! Classical tilt detection for `transform straighten --angle auto`, and the largest
//! axis-aligned rectangle inside rotated content for `--crop auto`.
//!
//! Detection: the image is reduced to at most 512 px, Sobel gradients are taken, and the
//! strongest edge pixels (top 15%) whose gradient is within 20° of vertical (horizontal
//! edges) or horizontal (vertical edges) vote. For each candidate tilt in ±15° the votes
//! are projected onto the normal of the tilted horizontal/vertical directions and binned
//! (1 px bins, linear soft binning); the tilt that makes the projections most concentrated
//! (largest sum of squared bin weights, a Hough-style projection profile) wins. A coarse
//! 0.25° sweep is refined at 0.025° and then by a parabola through the best three.

use pixelox_core::kernels::{self, Filter};
use pixelox_core::prelude::*;
use rayon::prelude::*;

/// The largest search range (degrees each way).
pub const MAX_TILT: f64 = 15.0;

/// A detected tilt: clockwise degrees (content rotated clockwise reads positive) and a
/// 0..1 confidence (how much the best angle stands out from the median).
#[derive(Clone, Copy, Debug)]
pub struct Tilt {
    pub degrees: f64,
    pub confidence: f64,
}

struct Vote {
    x: f32,
    y: f32,
    w: f32,
    horizontal: bool,
}

fn votes(img: &Image) -> (Vec<Vote>, f64) {
    let long = img.width.max(img.height);
    let small = if long > 512 {
        let k = 512.0 / long as f64;
        let (w, h) = (((img.width as f64 * k).round() as u32).max(3), ((img.height as f64 * k).round() as u32).max(3));
        kernels::resize(img, w, h, Filter::Lanczos3)
    } else {
        img.clone()
    };
    let (w, h) = (small.width as usize, small.height as usize);
    if w < 3 || h < 3 {
        return (Vec::new(), 0.0);
    }
    let gray: Vec<f32> = small.data.chunks(4).map(|p| color::luma(p[0], p[1], p[2]) * p[3]).collect();
    let rows: Vec<Vec<(usize, f32, f32)>> = (1..h - 1)
        .into_par_iter()
        .map(|y| {
            let at = |x: usize, y: usize| gray[y * w + x];
            (1..w - 1)
                .map(|x| {
                    let gx = (at(x + 1, y - 1) + 2.0 * at(x + 1, y) + at(x + 1, y + 1))
                        - (at(x - 1, y - 1) + 2.0 * at(x - 1, y) + at(x - 1, y + 1));
                    let gy = (at(x - 1, y + 1) + 2.0 * at(x, y + 1) + at(x + 1, y + 1))
                        - (at(x - 1, y - 1) + 2.0 * at(x, y - 1) + at(x + 1, y - 1));
                    (x, gx, gy)
                })
                .collect()
        })
        .collect();
    let mut mags: Vec<f32> =
        rows.iter().flatten().map(|&(_, gx, gy)| (gx * gx + gy * gy).sqrt()).filter(|&m| m > 0.0).collect();
    if mags.is_empty() {
        return (Vec::new(), 0.0);
    }
    mags.sort_by(|a, b| a.total_cmp(b));
    let thr = mags[(mags.len() as f64 * 0.85) as usize].max(0.02);
    let tan20 = 20f32.to_radians().tan();
    let (cx, cy) = (w as f32 / 2.0, h as f32 / 2.0);
    let mut out = Vec::new();
    for (ri, row) in rows.iter().enumerate() {
        let y = (ri + 1) as f32 + 0.5 - cy;
        for &(x, gx, gy) in row {
            let m = (gx * gx + gy * gy).sqrt();
            if m < thr {
                continue;
            }
            let x = x as f32 + 0.5 - cx;
            if gx.abs() <= tan20 * gy.abs() {
                out.push(Vote { x, y, w: m, horizontal: true });
            } else if gy.abs() <= tan20 * gx.abs() {
                out.push(Vote { x, y, w: m, horizontal: false });
            }
        }
    }
    let diag = ((w * w + h * h) as f64).sqrt();
    (out, diag)
}

fn score(v: &[Vote], diag: f64, deg: f64) -> f64 {
    let (s, c) = deg.to_radians().sin_cos();
    let (s, c) = (s as f32, c as f32);
    let half = (diag / 2.0).ceil() as i64 + 2;
    let n = (2 * half + 1) as usize;
    let mut hh = vec![0f64; n];
    let mut hv = vec![0f64; n];
    for p in v {
        // horizontal lines tilted clockwise by t keep -x·sin t + y·cos t constant;
        // vertical lines keep x·cos t + y·sin t constant
        let (rho, hist) = if p.horizontal { (-p.x * s + p.y * c, &mut hh) } else { (p.x * c + p.y * s, &mut hv) };
        let r = rho as f64 + half as f64;
        let i = r.floor();
        let t = r - i;
        let i = i as usize;
        if i + 1 < n {
            hist[i] += p.w as f64 * (1.0 - t);
            hist[i + 1] += p.w as f64 * t;
        }
    }
    hh.iter().chain(hv.iter()).map(|b| b * b).sum()
}

fn sweep(v: &[Vote], diag: f64, lo: f64, hi: f64, step: f64) -> Vec<(f64, f64)> {
    let n = ((hi - lo) / step).round() as usize + 1;
    (0..n).into_par_iter().map(|i| lo + i as f64 * step).map(|a| (a, score(v, diag, a))).collect()
}

fn best(s: &[(f64, f64)]) -> usize {
    let mut b = 0;
    for (i, p) in s.iter().enumerate() {
        if p.1 > s[b].1 {
            b = i;
        }
    }
    b
}

/// Detect the dominant near-horizontal / near-vertical edge tilt within ±`MAX_TILT`.
pub fn detect_tilt(img: &Image) -> Option<Tilt> {
    let (v, diag) = votes(img);
    if v.len() < 16 {
        return None;
    }
    let coarse = sweep(&v, diag, -MAX_TILT, MAX_TILT, 0.25);
    let bi = best(&coarse);
    if coarse[bi].1 <= 0.0 {
        return None;
    }
    let mut sorted: Vec<f64> = coarse.iter().map(|p| p.1).collect();
    sorted.sort_by(|a, b| a.total_cmp(b));
    let median = sorted[sorted.len() / 2];
    let c = coarse[bi].0;
    let fine = sweep(&v, diag, (c - 0.25).max(-MAX_TILT), (c + 0.25).min(MAX_TILT), 0.025);
    let fi = best(&fine);
    let mut deg = fine[fi].0;
    if fi > 0 && fi + 1 < fine.len() {
        let (y0, y1, y2) = (fine[fi - 1].1, fine[fi].1, fine[fi + 1].1);
        let den = y0 - 2.0 * y1 + y2;
        if den.abs() > 1e-12 {
            deg += 0.025 * (0.5 * (y0 - y2) / den).clamp(-0.5, 0.5);
        }
    }
    let top = fine[fi].1.max(coarse[bi].1);
    Some(Tilt { degrees: deg.clamp(-MAX_TILT, MAX_TILT), confidence: ((top - median) / top).clamp(0.0, 1.0) })
}

/// Width and height of the largest axis-aligned rectangle inside a `w`×`h` rectangle
/// rotated by `deg` (both centered).
pub fn max_inscribed(w: f64, h: f64, deg: f64) -> (f64, f64) {
    if w <= 0.0 || h <= 0.0 {
        return (0.0, 0.0);
    }
    let a = deg.to_radians();
    let (sin_a, cos_a) = (a.sin().abs(), a.cos().abs());
    if sin_a < 1e-12 {
        return (w, h);
    }
    let width_longer = w >= h;
    let (long, short) = if width_longer { (w, h) } else { (h, w) };
    if short <= 2.0 * sin_a * cos_a * long || (sin_a - cos_a).abs() < 1e-10 {
        let x = 0.5 * short;
        if width_longer {
            (x / sin_a, x / cos_a)
        } else {
            (x / cos_a, x / sin_a)
        }
    } else {
        let cos_2a = cos_a * cos_a - sin_a * sin_a;
        ((w * cos_a - h * sin_a) / cos_2a, (h * cos_a - w * sin_a) / cos_2a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Horizontal stripes (and a few vertical bars) rotated clockwise by `deg`.
    pub fn tilted(w: u32, h: u32, deg: f64) -> Image {
        let (s, c) = deg.to_radians().sin_cos();
        let (cx, cy) = (w as f64 / 2.0, h as f64 / 2.0);
        Image::from_fn(w, h, |x, y| {
            let (dx, dy) = (x as f64 + 0.5 - cx, y as f64 + 0.5 - cy);
            let v = -dx * s + dy * c; // rows of the unrotated content
            let u = dx * c + dy * s;
            let on = (v.rem_euclid(24.0) < 8.0) ^ (u.rem_euclid(64.0) < 4.0);
            let g = if on { 0.9 } else { 0.1 };
            [g, g, g, 1.0]
        })
    }

    #[test]
    fn finds_small_tilts_both_ways() {
        for t in [3.0, -5.5, 0.0] {
            let got = detect_tilt(&tilted(240, 180, t)).unwrap();
            assert!((got.degrees - t).abs() < 0.3, "expected {t}, got {}", got.degrees);
        }
    }

    #[test]
    fn flat_images_have_no_tilt() {
        assert!(detect_tilt(&Image::filled(64, 64, Rgba::WHITE)).is_none());
    }

    #[test]
    fn inscribed_rect() {
        assert_eq!(max_inscribed(100.0, 50.0, 0.0), (100.0, 50.0));
        let (w, h) = max_inscribed(100.0, 100.0, 45.0);
        // a square rotated 45° holds an axis-aligned square of side s/√2
        assert!((w - 70.7107).abs() < 1e-3 && (h - 70.7107).abs() < 1e-3, "{w} {h}");
        let (w, h) = max_inscribed(400.0, 300.0, 5.0);
        assert!(w < 400.0 && h < 300.0 && w > 340.0 && h > 250.0, "{w} {h}");
    }
}
