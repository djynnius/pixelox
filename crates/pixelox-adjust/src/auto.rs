//! `adjust auto`: detection of per-channel black points, white points, and gammas. The
//! detected values are recorded as the hidden `--black/--white/--gamma` params, so replays
//! and adjustment layers reproduce them without detecting again.

use crate::tone::LevelMap;
use crate::util::{bin_of, histograms, per_row, quantile, sum_rows, HBINS};
use pixelox_core::prelude::*;

/// Fraction clipped at each end by the contrast stretches (Photoshop's 0.1%).
const CLIP: f64 = 0.001;
/// Fraction of darkest/lightest pixels averaged by "find dark & light colors".
const EXTREME: f64 = 0.005;
/// Minimum span (0..1) worth stretching.
const MIN_SPAN: f32 = 2.0 / 255.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Per-channel endpoints (removes casts in shadows and highlights).
    Levels,
    /// One shared stretch for all channels (keeps colors).
    Contrast,
    /// Dark & light colors mapped to black & white, neutral midtones.
    Color,
    /// Shared stretch plus neutral midtones.
    Tone,
}

/// Detected per-channel `black`, `white` (0..1), and `gamma`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Endpoints {
    pub black: [f32; 3],
    pub white: [f32; 3],
    pub gamma: [f32; 3],
}

fn guard(lo: f32, hi: f32) -> (f32, f32) {
    if hi - lo < MIN_SPAN {
        (0.0, 1.0)
    } else {
        (lo, hi)
    }
}

/// Mean color of pixels whose luma falls in histogram bins `lo..=hi`.
fn mean_color_in(img: &Image, lo: usize, hi: usize) -> Option<[f32; 3]> {
    let rows = per_row(img, |_, row| {
        let mut acc = [0.0f64; 4];
        for p in row.chunks_exact(4) {
            if p[3] <= 0.0 {
                continue;
            }
            let b = bin_of(color::luma(p[0], p[1], p[2]), HBINS);
            if b >= lo && b <= hi {
                acc[0] += p[0] as f64;
                acc[1] += p[1] as f64;
                acc[2] += p[2] as f64;
                acc[3] += 1.0;
            }
        }
        acc
    });
    let s = sum_rows(&rows);
    (s[3] > 0.0).then(|| [(s[0] / s[3]) as f32, (s[1] / s[3]) as f32, (s[2] / s[3]) as f32])
}

/// Per-channel gammas that make the mean midtone (after the endpoint mapping) neutral.
fn neutral_gamma(img: &Image, black: [f32; 3], white: [f32; 3]) -> [f32; 3] {
    let maps = [0, 1, 2].map(|c| LevelMap { in_black: black[c], in_white: white[c], ..LevelMap::IDENTITY });
    let rows = per_row(img, |_, row| {
        let mut acc = [0.0f64; 4];
        for p in row.chunks_exact(4) {
            if p[3] <= 0.0 {
                continue;
            }
            let m = [maps[0].map(p[0]), maps[1].map(p[1]), maps[2].map(p[2])];
            let l = color::luma(m[0], m[1], m[2]);
            if (0.2..=0.8).contains(&l) {
                for (a, v) in acc.iter_mut().zip(m) {
                    *a += v as f64;
                }
                acc[3] += 1.0;
            }
        }
        acc
    });
    let s = sum_rows(&rows);
    if s[3] <= 0.0 {
        return [1.0; 3];
    }
    let m = [s[0] / s[3], s[1] / s[3], s[2] / s[3]];
    let target = (m[0] + m[1] + m[2]) / 3.0;
    if target <= 0.01 || target >= 0.99 {
        return [1.0; 3];
    }
    m.map(|mc| if mc <= 0.01 || mc >= 0.99 { 1.0 } else { (mc.ln() / target.ln()).clamp(0.5, 2.0) as f32 })
}

/// Detect the corrections for `mode`. Images with nothing to stretch get identity values.
pub fn detect(img: &Image, mode: Mode) -> Endpoints {
    let h = histograms(img, HBINS);
    let identity = Endpoints { black: [0.0; 3], white: [1.0; 3], gamma: [1.0; 3] };
    let Some(_) = quantile(&h[3], 0.5) else { return identity };
    let lo = |c: usize| quantile(&h[c], CLIP).unwrap_or(0.0);
    let hi = |c: usize| quantile(&h[c], 1.0 - CLIP).unwrap_or(1.0);
    let shared = || {
        let (l, hgh) = guard(lo(0).min(lo(1)).min(lo(2)), hi(0).max(hi(1)).max(hi(2)));
        ([l; 3], [hgh; 3])
    };
    match mode {
        Mode::Levels => {
            let mut black = [0f32; 3];
            let mut white = [1f32; 3];
            for c in 0..3 {
                (black[c], white[c]) = guard(lo(c), hi(c));
            }
            Endpoints { black, white, gamma: [1.0; 3] }
        }
        Mode::Contrast => {
            let (black, white) = shared();
            Endpoints { black, white, gamma: [1.0; 3] }
        }
        Mode::Color => {
            let bin = |q: f64| bin_of(quantile(&h[3], q).unwrap_or(0.0), HBINS);
            let dark = mean_color_in(img, 0, bin(EXTREME)).unwrap_or([0.0; 3]);
            let light = mean_color_in(img, bin(1.0 - EXTREME), HBINS - 1).unwrap_or([1.0; 3]);
            let mut black = [0f32; 3];
            let mut white = [1f32; 3];
            for c in 0..3 {
                (black[c], white[c]) = guard(dark[c], light[c]);
            }
            let gamma = neutral_gamma(img, black, white);
            Endpoints { black, white, gamma }
        }
        Mode::Tone => {
            let (black, white) = shared();
            let gamma = neutral_gamma(img, black, white);
            Endpoints { black, white, gamma }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ramp(lo: f32, hi: f32, tint: [f32; 3]) -> Image {
        Image::from_fn(64, 4, |x, _| {
            let v = lo + (hi - lo) * x as f32 / 63.0;
            [v * tint[0], v * tint[1], v * tint[2], 1.0]
        })
    }

    #[test]
    fn contrast_finds_shared_endpoints() {
        let e = detect(&ramp(0.2, 0.8, [1.0; 3]), Mode::Contrast);
        assert!((e.black[0] - 0.2).abs() < 0.01 && (e.white[0] - 0.8).abs() < 0.01, "{e:?}");
        assert_eq!(e.black[0], e.black[2]);
    }

    #[test]
    fn levels_are_per_channel() {
        let e = detect(&ramp(0.2, 0.8, [1.0, 0.5, 1.0]), Mode::Levels);
        assert!(e.white[1] < e.white[0] - 0.2, "{e:?}");
    }

    #[test]
    fn flat_images_are_left_alone() {
        let flat = Image::filled(8, 8, Rgba::new(0.5, 0.5, 0.5, 1.0));
        for m in [Mode::Levels, Mode::Contrast, Mode::Color, Mode::Tone] {
            let e = detect(&flat, m);
            assert_eq!(e.black, [0.0; 3]);
            assert_eq!(e.white, [1.0; 3]);
        }
        let empty = Image::new(4, 4);
        assert_eq!(detect(&empty, Mode::Tone).gamma, [1.0; 3]);
    }

    #[test]
    fn tone_neutralizes_midtone_cast() {
        let e = detect(&ramp(0.0, 1.0, [1.0, 0.9, 0.7]), Mode::Tone);
        assert!(e.gamma[2] > e.gamma[0], "blue needs lifting: {e:?}");
    }
}
