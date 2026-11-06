//! Deterministic 2-D gradient noise and fractal sums for the procedural effects (fire,
//! smoke). Every value is a pure function of the seed and the coordinates, so renders do
//! not depend on the thread count or on the order pixels are visited.

use std::f64::consts::FRAC_1_SQRT_2;

/// Sixteen unit gradients, 22.5° apart.
const GRADS: [(f64, f64); 16] = {
    // cos/sin of k·22.5° (const fn trigonometry is not available)
    const C: [f64; 16] = [
        1.0,
        0.923_879_532_511_286_7,
        FRAC_1_SQRT_2,
        0.382_683_432_365_089_8,
        0.0,
        -0.382_683_432_365_089_8,
        -FRAC_1_SQRT_2,
        -0.923_879_532_511_286_7,
        -1.0,
        -0.923_879_532_511_286_7,
        -FRAC_1_SQRT_2,
        -0.382_683_432_365_089_8,
        0.0,
        0.382_683_432_365_089_8,
        FRAC_1_SQRT_2,
        0.923_879_532_511_286_7,
    ];
    let mut g = [(0.0, 0.0); 16];
    let mut k = 0;
    while k < 16 {
        g[k] = (C[k], C[(k + 12) % 16]);
        k += 1;
    }
    g
};

/// SplitMix64 finalizer.
#[inline]
pub(crate) fn mix64(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[inline]
fn hash(seed: u64, x: i64, y: i64) -> u64 {
    mix64(seed ^ (x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F))
}

#[inline]
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// A seeded gradient-noise field.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Noise {
    seed: u64,
}

impl Noise {
    pub(crate) fn new(seed: u64) -> Noise {
        Noise { seed: mix64(seed ^ 0x6A09_E667_F3BC_C909) }
    }

    /// An independent field derived from this one.
    pub(crate) fn lane(self, k: u64) -> Noise {
        Noise { seed: mix64(self.seed ^ (k + 1).wrapping_mul(0xA076_1D64_78BD_642F)) }
    }

    /// Gradient noise at (x, y): smooth, zero on the integer lattice, roughly -1..1.
    #[inline]
    pub(crate) fn at(self, x: f64, y: f64) -> f64 {
        let (x0, y0) = (x.floor(), y.floor());
        let (tx, ty) = (x - x0, y - y0);
        let (ix, iy) = (x0 as i64, y0 as i64);
        let dot = |i: i64, j: i64| {
            let g = GRADS[(hash(self.seed, ix + i, iy + j) >> 60) as usize];
            g.0 * (tx - i as f64) + g.1 * (ty - j as f64)
        };
        let (u, v) = (fade(tx), fade(ty));
        let a = dot(0, 0) + (dot(1, 0) - dot(0, 0)) * u;
        let b = dot(0, 1) + (dot(1, 1) - dot(0, 1)) * u;
        ((a + (b - a) * v) * 1.45).clamp(-1.0, 1.0)
    }

    /// Fractal sum of `octaves` octaves (each twice the frequency, half the amplitude),
    /// roughly -1..1.
    pub(crate) fn fbm(self, x: f64, y: f64, octaves: u32) -> f64 {
        let (mut sum, mut amp, mut norm, mut f) = (0.0, 1.0, 0.0, 1.0);
        for o in 0..octaves {
            // offset each octave so the lattice zeros do not line up at the origin
            let off = o as f64 * 17.31;
            sum += amp * self.lane(o as u64).at(x * f + off, y * f - off);
            norm += amp;
            amp *= 0.5;
            f *= 2.0;
        }
        if norm > 0.0 {
            sum / norm
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_is_smooth_bounded_and_seeded() {
        let n = Noise::new(7);
        assert_eq!(n.at(3.0, 4.0), 0.0, "zero on the lattice");
        let mut lo: f64 = 0.0;
        let mut hi: f64 = 0.0;
        for i in 0..2000 {
            let (x, y) = (i as f64 * 0.137, i as f64 * 0.071);
            let v = n.fbm(x, y, 3);
            lo = lo.min(v);
            hi = hi.max(v);
            // continuity: a tiny step changes the value a little
            assert!((n.at(x, y) - n.at(x + 1e-3, y)).abs() < 0.01);
        }
        assert!(lo < -0.3 && hi > 0.3 && lo >= -1.0 && hi <= 1.0, "{lo}..{hi}");
        assert_eq!(n.at(1.3, 2.7), Noise::new(7).at(1.3, 2.7));
        assert_ne!(n.at(1.3, 2.7), Noise::new(8).at(1.3, 2.7));
        assert_ne!(n.at(1.3, 2.7), n.lane(1).at(1.3, 2.7));
    }
}
