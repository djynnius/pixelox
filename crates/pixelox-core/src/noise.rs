//! Seeded procedural noise shared by fills, displacement maps, grain, and generators:
//! gradient (Perlin), simplex, and value noise; fractal sums (fBm, turbulence, ridged);
//! Worley cellular noise (F1 and F2 − F1); periodic lattices for tileable output.
//!
//! Every value is a pure function of the seed and the coordinates (no state, no visiting
//! order), so renders are identical for any thread count. Fractal sums are normalized by
//! the basis statistics (measured once from a fixed sample), so [`Field::at`] has about the
//! same mean (0.5) and spread (standard deviation [`SPREAD`]) for every pattern, octave
//! count, and persistence: contrast and bias settings mean the same thing everywhere.

use std::f64::consts::TAU;
use std::sync::OnceLock;

/// Standard deviation of a normalized [`Field`] value (mean 0.5): ±2.8 σ spans 0..1.
pub const SPREAD: f64 = 0.18;

/// SplitMix64: a fast, well-mixed 64-bit hash step.
#[inline]
pub fn mix64(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Hash of a seed and an integer lattice point.
#[inline]
pub fn hash2(seed: u64, x: i64, y: i64) -> u64 {
    mix64(seed ^ (x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F))
}

/// A uniform value in [0, 1) for a lattice point.
#[inline]
pub fn hash01(seed: u64, x: i64, y: i64) -> f64 {
    (hash2(seed, x, y) >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
}

/// A standard normal value for a lattice point (Box–Muller on two hashes).
#[inline]
pub fn gauss(seed: u64, x: i64, y: i64) -> f32 {
    let h = hash2(seed, x, y);
    let u1 = ((h >> 40) as f32 + 1.0) * (1.0 / 16_777_217.0); // (0, 1]
    let u2 = ((h & 0xFF_FFFF) as f32) * (1.0 / 16_777_216.0);
    (-2.0 * u1.ln()).sqrt() * (std::f32::consts::TAU * u2).cos()
}

/// A derived, independent seed (`lane` k of `seed`).
#[inline]
pub fn lane(seed: u64, k: u64) -> u64 {
    mix64(seed ^ (k + 1).wrapping_mul(0xA076_1D64_78BD_642F))
}

/// The noise function summed by a fractal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Basis {
    /// Gradient noise on a square lattice (improved Perlin, quintic fade).
    Perlin,
    /// Simplex gradient noise (fewer axis-aligned artifacts; not tileable).
    Simplex,
    /// Smoothly interpolated random values (softer, blockier).
    Value,
}

impl Basis {
    pub fn parse(s: &str) -> Option<Basis> {
        Some(match s {
            "perlin" => Basis::Perlin,
            "simplex" => Basis::Simplex,
            "value" => Basis::Value,
            _ => return None,
        })
    }
}

/// Lattice period in cells (`Some((px, py))` makes the noise repeat every px × py cells).
pub type Period = Option<(i64, i64)>;

#[inline]
fn wrap(i: i64, p: Option<i64>) -> i64 {
    match p {
        Some(p) if p > 0 => i.rem_euclid(p),
        _ => i,
    }
}

#[inline]
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Gradient noise at (x, y), roughly −1..1, zero on the lattice.
#[inline]
pub fn perlin(seed: u64, x: f64, y: f64, period: Period) -> f64 {
    let (x0, y0) = (x.floor(), y.floor());
    let (tx, ty) = (x - x0, y - y0);
    let (ix, iy) = (x0 as i64, y0 as i64);
    let (px, py) = (period.map(|p| p.0), period.map(|p| p.1));
    let dot = |i: i64, j: i64| {
        let h = hash2(seed, wrap(ix + i, px), wrap(iy + j, py));
        // one of 16 unit gradients
        let a = (h >> 60) as f64 * (TAU / 16.0);
        a.cos() * (tx - i as f64) + a.sin() * (ty - j as f64)
    };
    let (u, v) = (fade(tx), fade(ty));
    let (d00, d10, d01, d11) = (dot(0, 0), dot(1, 0), dot(0, 1), dot(1, 1));
    let a = d00 + (d10 - d00) * u;
    let b = d01 + (d11 - d01) * u;
    (a + (b - a) * v) * std::f64::consts::SQRT_2
}

/// Value noise at (x, y), −1..1.
#[inline]
pub fn value(seed: u64, x: f64, y: f64, period: Period) -> f64 {
    let (x0, y0) = (x.floor(), y.floor());
    let (ix, iy) = (x0 as i64, y0 as i64);
    let (px, py) = (period.map(|p| p.0), period.map(|p| p.1));
    let at = |i: i64, j: i64| hash01(seed, wrap(ix + i, px), wrap(iy + j, py)) * 2.0 - 1.0;
    let (u, v) = (fade(x - x0), fade(y - y0));
    let a = at(0, 0) + (at(1, 0) - at(0, 0)) * u;
    let b = at(0, 1) + (at(1, 1) - at(0, 1)) * u;
    a + (b - a) * v
}

/// 2-D simplex noise at (x, y), roughly −1..1.
#[inline]
pub fn simplex(seed: u64, x: f64, y: f64) -> f64 {
    const F2: f64 = 0.366_025_403_784_438_6; // (√3 − 1) / 2
    const G2: f64 = 0.211_324_865_405_187_1; // (3 − √3) / 6
    let s = (x + y) * F2;
    let (i, j) = ((x + s).floor(), (y + s).floor());
    let t = (i + j) * G2;
    let (x0, y0) = (x - (i - t), y - (j - t));
    let (i1, j1) = if x0 > y0 { (1.0, 0.0) } else { (0.0, 1.0) };
    let corners = [
        (x0, y0, 0.0, 0.0),
        (x0 - i1 + G2, y0 - j1 + G2, i1, j1),
        (x0 - 1.0 + 2.0 * G2, y0 - 1.0 + 2.0 * G2, 1.0, 1.0),
    ];
    let mut n = 0.0;
    for (dx, dy, oi, oj) in corners {
        let tt = 0.5 - dx * dx - dy * dy;
        if tt > 0.0 {
            let h = hash2(seed, (i + oi) as i64, (j + oj) as i64);
            let a = (h >> 58) as f64 * (TAU / 64.0);
            let t2 = tt * tt;
            n += t2 * t2 * (a.cos() * dx + a.sin() * dy);
        }
    }
    70.0 * n
}

/// Worley (cellular) noise: distances (in cells) to the nearest and second-nearest of one
/// jittered feature point per cell.
#[inline]
pub fn worley(seed: u64, x: f64, y: f64, period: Period) -> (f64, f64) {
    let (ix, iy) = (x.floor() as i64, y.floor() as i64);
    let (px, py) = (period.map(|p| p.0), period.map(|p| p.1));
    let (mut f1, mut f2) = (f64::INFINITY, f64::INFINITY);
    for j in -1..=1 {
        for i in -1..=1 {
            let (cx, cy) = (ix + i, iy + j);
            let h = hash2(seed, wrap(cx, px), wrap(cy, py));
            let fx = cx as f64 + (h >> 40) as f64 / (1u64 << 24) as f64;
            let fy = cy as f64 + (h & 0xFF_FFFF) as f64 / (1u64 << 24) as f64;
            let d = ((fx - x) * (fx - x) + (fy - y) * (fy - y)).sqrt();
            if d < f1 {
                f2 = f1;
                f1 = d;
            } else if d < f2 {
                f2 = d;
            }
        }
    }
    (f1, f2)
}

/// How the octaves of a [`Field`] are shaped and summed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pattern {
    /// Fractal Brownian motion: soft clouds (Photoshop's Render Clouds).
    Fbm,
    /// Sum of |noise|: billowy, puffy shapes with creases (smoke, fire, puffs).
    Turbulence,
    /// Sum of (1 − |noise|)²: sharp ridges and veins (marble, lightning, mountains).
    Ridged,
    /// Worley F1: round cells, darkest at their centers.
    Cells,
    /// Worley F2 − F1: a network of cell borders (cracks, crackle, scales).
    CellEdges,
}

/// A fractal noise field. Coordinates are in octave-0 lattice cells.
#[derive(Clone, Copy, Debug)]
pub struct Field {
    pub pattern: Pattern,
    pub basis: Basis,
    pub octaves: u32,
    /// Amplitude factor per octave (0..1; higher = rougher).
    pub persistence: f64,
    /// Frequency factor per octave (an integer when `period` is set).
    pub lacunarity: f64,
    pub seed: u64,
    /// Repeat every (px, py) octave-0 cells.
    pub period: Period,
}

/// Mean and standard deviation of one octave's shaped value, per (pattern, basis).
fn stats(pattern: Pattern, basis: Basis) -> (f64, f64) {
    static CACHE: OnceLock<Vec<((Pattern, Basis), (f64, f64))>> = OnceLock::new();
    let all = CACHE.get_or_init(|| {
        let mut v = Vec::new();
        for p in [Pattern::Fbm, Pattern::Turbulence, Pattern::Ridged, Pattern::Cells, Pattern::CellEdges] {
            for b in [Basis::Perlin, Basis::Simplex, Basis::Value] {
                // a fixed, deterministic sample (irrational steps avoid the lattice)
                let n = 40_000;
                let (mut s, mut s2) = (0.0, 0.0);
                for k in 0..n {
                    let x = k as f64 * 0.618_033_988_749_894_8 * 7.31;
                    let y = k as f64 * 0.754_877_666_246_692_7 * 5.17;
                    let val = shaped(p, b, 0x005E_ED0F_57A7, x, y, None);
                    s += val;
                    s2 += val * val;
                }
                let m = s / n as f64;
                v.push(((p, b), (m, (s2 / n as f64 - m * m).max(1e-12).sqrt())));
            }
        }
        v
    });
    all.iter().find(|(k, _)| *k == (pattern, basis)).map(|(_, s)| *s).unwrap_or((0.0, 1.0))
}

/// One octave's value before normalization.
#[inline]
fn shaped(pattern: Pattern, basis: Basis, seed: u64, x: f64, y: f64, period: Period) -> f64 {
    let base = |s: u64| match basis {
        Basis::Perlin => perlin(s, x, y, period),
        Basis::Simplex => simplex(s, x, y),
        Basis::Value => value(s, x, y, period),
    };
    match pattern {
        Pattern::Fbm => base(seed),
        Pattern::Turbulence => base(seed).abs(),
        Pattern::Ridged => {
            let r = 1.0 - base(seed).abs();
            r * r
        }
        Pattern::Cells => worley(seed, x, y, period).0,
        Pattern::CellEdges => {
            let (f1, f2) = worley(seed, x, y, period);
            f2 - f1
        }
    }
}

impl Field {
    /// A plain 5-octave fBm Perlin field.
    pub fn new(seed: u64) -> Field {
        Field {
            pattern: Pattern::Fbm,
            basis: Basis::Perlin,
            octaves: 5,
            persistence: 0.5,
            lacunarity: 2.0,
            seed,
            period: None,
        }
    }

    /// The normalized value at (x, y): mean about 0.5, standard deviation [`SPREAD`];
    /// not clamped.
    pub fn at(&self, x: f64, y: f64) -> f64 {
        let (m, sd) = stats(self.pattern, self.basis);
        let (mut sum, mut amp, mut freq, mut norm2) = (0.0, 1.0, 1.0, 0.0);
        let octaves = self.octaves.max(1);
        for o in 0..octaves {
            // offset each octave so the lattice zeros of different octaves do not line up
            let off = o as f64 * 0.371 + 0.137;
            let period =
                self.period.map(|(px, py)| ((px as f64 * freq).round() as i64, (py as f64 * freq).round() as i64));
            let v = shaped(self.pattern, self.basis, lane(self.seed, o as u64), x * freq + off, y * freq - off, period);
            sum += amp * (v - m);
            norm2 += amp * amp;
            amp *= self.persistence;
            freq *= self.lacunarity;
        }
        let z = sum / (sd * norm2.sqrt());
        0.5 + z * SPREAD
    }
}

/// `t` (0..1 field value) with contrast (−100..100) and bias (−100..100) applied, clamped.
#[inline]
pub fn shape_value(t: f64, contrast: f64, bias: f64) -> f64 {
    let k = if contrast >= 0.0 { 1.0 + contrast / 100.0 * 4.0 } else { 1.0 + contrast / 100.0 };
    (0.5 + (t - 0.5) * k + bias / 100.0 * 0.5).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bases_are_smooth_bounded_and_seeded() {
        for (name, f) in [
            ("perlin", (|s, x, y| perlin(s, x, y, None)) as fn(u64, f64, f64) -> f64),
            ("simplex", |s, x, y| simplex(s, x, y)),
            ("value", |s, x, y| value(s, x, y, None)),
        ] {
            let (mut lo, mut hi) = (0.0f64, 0.0f64);
            for k in 0..5000 {
                let (x, y) = (k as f64 * 0.173, k as f64 * 0.091);
                let v = f(7, x, y);
                lo = lo.min(v);
                hi = hi.max(v);
                assert!((f(7, x, y) - f(7, x + 1e-4, y)).abs() < 0.01, "{name} is continuous");
            }
            assert!(lo < -0.4 && hi > 0.4 && lo > -1.3 && hi < 1.3, "{name}: {lo}..{hi}");
            assert_eq!(f(7, 1.3, 2.7), f(7, 1.3, 2.7));
            assert_ne!(f(7, 1.3, 2.7), f(8, 1.3, 2.7), "{name} depends on the seed");
        }
    }

    #[test]
    fn periodic_lattices_repeat() {
        let p = Some((5, 3));
        for &(x, y) in &[(0.3, 0.7), (2.9, 1.1), (4.99, 2.5)] {
            assert!((perlin(3, x, y, p) - perlin(3, x + 5.0, y + 3.0, p)).abs() < 1e-12);
            assert!((value(3, x, y, p) - value(3, x - 10.0, y, p)).abs() < 1e-12);
            let (a, b) = (worley(3, x, y, p), worley(3, x + 5.0, y - 3.0, p));
            assert!((a.0 - b.0).abs() < 1e-9 && (a.1 - b.1).abs() < 1e-9);
        }
        let f = Field { period: Some((4, 2)), ..Field::new(11) };
        for &(x, y) in &[(0.25, 0.5), (3.7, 1.9)] {
            assert!((f.at(x, y) - f.at(x + 4.0, y + 2.0)).abs() < 1e-9);
        }
    }

    #[test]
    fn fields_are_normalized_for_every_pattern() {
        for pattern in [Pattern::Fbm, Pattern::Turbulence, Pattern::Ridged, Pattern::Cells, Pattern::CellEdges] {
            for (octaves, persistence) in [(1, 0.5), (6, 0.5), (4, 0.8)] {
                let f = Field { pattern, octaves, persistence, ..Field::new(5) };
                let n = 6000;
                let vals: Vec<f64> = (0..n).map(|k| f.at(k as f64 * 0.377 + 0.11, k as f64 * 0.213 + 0.07)).collect();
                let m = vals.iter().sum::<f64>() / n as f64;
                let sd = (vals.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / n as f64).sqrt();
                assert!((m - 0.5).abs() < 0.06, "{pattern:?} {octaves}: mean {m}");
                assert!((sd - SPREAD).abs() < 0.06, "{pattern:?} {octaves}: sd {sd}");
            }
        }
    }

    #[test]
    fn gauss_is_standard_normal() {
        let n = 20_000;
        let v: Vec<f32> = (0..n).map(|k| gauss(9, k, k * 7 + 1)).collect();
        let m = v.iter().sum::<f32>() / n as f32;
        let sd = (v.iter().map(|x| (x - m) * (x - m)).sum::<f32>() / n as f32).sqrt();
        assert!(m.abs() < 0.03 && (sd - 1.0).abs() < 0.03, "{m} {sd}");
    }

    #[test]
    fn shaping_contrast_and_bias() {
        assert_eq!(shape_value(0.5, 0.0, 0.0), 0.5);
        assert!(shape_value(0.6, 50.0, 0.0) > 0.7);
        assert!((shape_value(0.6, -100.0, 0.0) - 0.5).abs() < 1e-12);
        assert!(shape_value(0.5, 0.0, 40.0) > 0.65);
        assert_eq!(shape_value(2.0, 0.0, 0.0), 1.0);
    }
}
