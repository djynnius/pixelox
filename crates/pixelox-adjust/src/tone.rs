//! Tonal adjustments: brightness/contrast, levels, curves, exposure, shadows/highlights,
//! tone mapping, equalize, posterize, threshold, invert. Each `prepare` function reads its
//! (already resolved) params and returns the job that changes an image; all math is f32
//! on 0..1 values, so 8- and 16-bit documents keep their precision. Alpha is untouched.

use crate::adjust::{pixel_job, Job};
use crate::curve::Curve;
use crate::util::{self, clamp01, map_rgb_indexed, per_row, sum_rows, HBINS};
use pixelox_core::kernels::{gaussian_planar, Edge};
use pixelox_core::prelude::*;

/// Which of R, G, B a `--channel rgb|r|g|b` value selects.
pub fn channels(ch: &str) -> [bool; 3] {
    match ch {
        "r" => [true, false, false],
        "g" => [false, true, false],
        "b" => [false, false, true],
        _ => [true, true, true],
    }
}

fn smooth_contrast(v: f32, c: f32) -> f32 {
    if c >= 0.0 {
        // endpoint-preserving S-curve with slope 1 + 2c at mid gray
        let p = 1.0 + 2.0 * c;
        if v < 0.5 {
            0.5 * (2.0 * v.max(0.0)).powf(p)
        } else {
            1.0 - 0.5 * (2.0 * (1.0 - v).max(0.0)).powf(p)
        }
    } else {
        0.5 + (v - 0.5) * (1.0 + 0.75 * c)
    }
}

pub fn brightness_contrast(a: &Args) -> Result<Job> {
    let b = a.f32("brightness")? / 100.0;
    let c = a.f32("contrast")? / 100.0;
    // brightness bends the midtones like Photoshop's (non-legacy) slider: black and
    // white stay where they are
    let e = 2f32.powf(-b);
    Ok(pixel_job(move |p| {
        p.map(|v| {
            let v = if b != 0.0 { clamp01(v).powf(e) } else { v };
            if c != 0.0 {
                smooth_contrast(v, c)
            } else {
                v
            }
        })
    }))
}

/// One channel's levels mapping (all values 0..1).
#[derive(Clone, Copy, Debug)]
pub struct LevelMap {
    pub in_black: f32,
    pub in_white: f32,
    pub inv_gamma: f32,
    pub out_black: f32,
    pub out_white: f32,
}

impl LevelMap {
    pub const IDENTITY: LevelMap =
        LevelMap { in_black: 0.0, in_white: 1.0, inv_gamma: 1.0, out_black: 0.0, out_white: 1.0 };

    #[inline]
    pub fn map(&self, v: f32) -> f32 {
        let t = ((v - self.in_black) / (self.in_white - self.in_black)).clamp(0.0, 1.0);
        let t = if self.inv_gamma != 1.0 { t.powf(self.inv_gamma) } else { t };
        self.out_black + (self.out_white - self.out_black) * t
    }
}

pub fn levels(a: &Args) -> Result<Job> {
    let ib = a.f32("in-black")?;
    let iw = a.f32("in-white")?;
    if iw <= ib {
        return Err(Error::new("PX1010", format!("--in-white ({iw}) must be greater than --in-black ({ib})"))
            .with_context("in_black", ib)
            .with_context("in_white", iw));
    }
    let m = LevelMap {
        in_black: ib / 255.0,
        in_white: iw / 255.0,
        inv_gamma: 1.0 / a.f32("gamma")?,
        out_black: a.f32("out-black")? / 255.0,
        out_white: a.f32("out-white")? / 255.0,
    };
    let sel = channels(&a.choice("channel"));
    Ok(pixel_job(move |p| [0, 1, 2].map(|c| if sel[c] { m.map(p[c]) } else { p[c] })))
}

/// Per-channel levels from `black`/`white` (0–255) and `gamma` triples (`adjust auto`).
pub fn levels_rgb(black: [f32; 3], white: [f32; 3], gamma: [f32; 3]) -> Job {
    let maps = [0, 1, 2].map(|c| LevelMap {
        in_black: black[c] / 255.0,
        in_white: (white[c] / 255.0).max(black[c] / 255.0 + 1e-4),
        inv_gamma: 1.0 / gamma[c].max(0.01),
        out_black: 0.0,
        out_white: 1.0,
    });
    pixel_job(move |p| [0, 1, 2].map(|c| maps[c].map(p[c])))
}

pub fn curves(a: &Args) -> Result<Job> {
    let pts = a.points("points")?;
    let curve = Curve::new(&pts)?;
    let sel = channels(&a.choice("channel"));
    Ok(pixel_job(move |p| [0, 1, 2].map(|c| if sel[c] { curve.map01(p[c]) } else { p[c] })))
}

pub fn exposure(a: &Args) -> Result<Job> {
    let gain = 2f32.powf(a.f32("ev")?);
    let offset = a.f32("offset")?;
    let inv_gamma = 1.0 / a.f32("gamma")?;
    Ok(pixel_job(move |p| {
        p.map(|v| {
            let lin = color::srgb_to_linear(v) * gain + offset;
            let lin = lin.max(0.0);
            let lin = if inv_gamma != 1.0 { lin.powf(inv_gamma) } else { lin };
            color::linear_to_srgb(lin.min(1.0))
        })
    }))
}

/// Alpha-weighted Gaussian blur of a per-pixel value: transparent pixels do not pull the
/// local average toward zero.
fn local_mean(img: &Image, values: &[f32], sigma: f64) -> Vec<f32> {
    let (w, h) = (img.width as usize, img.height as usize);
    if sigma <= 0.0 {
        return values.to_vec();
    }
    let mut buf: Vec<f32> = Vec::with_capacity(w * h * 2);
    for (i, v) in values.iter().enumerate() {
        let a = img.data[i * 4 + 3];
        buf.push(v * a);
        buf.push(a);
    }
    gaussian_planar(&mut buf, w, h, 2, sigma, sigma, Edge::Clamp);
    buf.chunks_exact(2).zip(values).map(|(p, v)| if p[1] > 1e-6 { p[0] / p[1] } else { *v }).collect()
}

fn luma_of(img: &Image) -> Vec<f32> {
    img.data.chunks_exact(4).map(|p| color::luma(p[0], p[1], p[2])).collect()
}

pub fn shadows_highlights(a: &Args) -> Result<Job> {
    let s = a.f32("shadows")? / 100.0;
    let h = a.f32("highlights")? / 100.0;
    let radius = a.f64("radius")?;
    Ok(Box::new(move |img: &mut Image| {
        if img.is_empty() || (s == 0.0 && h == 0.0) {
            return;
        }
        let base = local_mean(img, &luma_of(img), radius);
        map_rgb_indexed(img, |i, p| {
            let lb = clamp01(base[i]);
            let es = 1.0 / (1.0 + 1.5 * s * (1.0 - lb).powi(3));
            let eh = 1.0 / (1.0 + 1.5 * h * lb.powi(3));
            p.map(|v| {
                let v1 = clamp01(v).powf(es);
                1.0 - (1.0 - v1).powf(eh)
            })
        });
    }))
}

pub fn tone_map(a: &Args) -> Result<Job> {
    let s = a.f32("strength")? / 100.0;
    Ok(Box::new(move |img: &mut Image| {
        if img.is_empty() || s <= 0.0 {
            return;
        }
        const EPS: f32 = 1e-4;
        let lin: Vec<[f32; 3]> = img
            .data
            .chunks_exact(4)
            .map(|p| [color::srgb_to_linear(p[0]), color::srgb_to_linear(p[1]), color::srgb_to_linear(p[2])])
            .collect();
        let ys: Vec<f32> = lin.iter().map(|c| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]).collect();
        let logs: Vec<f32> = ys.iter().map(|y| (y + EPS).ln()).collect();
        let sigma = (img.width.max(img.height) as f64 * 0.02).max(1.0);
        let base = local_mean(img, &logs, sigma);
        // pivot on the alpha-weighted mean log luminance (the image's "key")
        let w = img.width as usize;
        let rows = per_row(img, |y, row| {
            let mut acc = [0.0f64; 2];
            for (x, p) in row.chunks_exact(4).enumerate() {
                acc[0] += base[y * w + x] as f64 * p[3] as f64;
                acc[1] += p[3] as f64;
            }
            acc
        });
        let [sum, weight] = sum_rows(&rows);
        if weight <= 0.0 {
            return;
        }
        let key = (sum / weight) as f32;
        let compress = 1.0 - 0.75 * s;
        let detail = 1.0 + 0.4 * s;
        map_rgb_indexed(img, |i, _| {
            let y = ys[i];
            let c = lin[i];
            let target = (key + (base[i] - key) * compress + (logs[i] - base[i]) * detail).exp() - EPS;
            let mut ratio = if y > 1e-6 { target.max(0.0) / y } else { 1.0 };
            let mx = c[0].max(c[1]).max(c[2]);
            if mx * ratio > 1.0 {
                ratio = 1.0 / mx;
            }
            c.map(|v| color::linear_to_srgb(v * ratio))
        });
    }))
}

pub fn equalize(_: &Args) -> Result<Job> {
    Ok(Box::new(|img: &mut Image| {
        let h = &util::histograms(img, HBINS)[3];
        let total: u64 = h.iter().sum();
        let Some(first) = h.iter().position(|&c| c > 0) else { return };
        let c0 = h[first];
        if total == c0 {
            return;
        }
        let mut lut = vec![0f32; HBINS];
        let mut acc = 0u64;
        for (i, &c) in h.iter().enumerate() {
            acc += c;
            lut[i] = (acc.saturating_sub(c0)) as f32 / (total - c0) as f32;
        }
        crate::util::map_rgb(img, |p| p.map(|v| lut[util::bin_of(v, HBINS)]));
    }))
}

pub fn posterize(a: &Args) -> Result<Job> {
    let n = a.int("levels")? as f32;
    Ok(pixel_job(move |p| p.map(|v| (clamp01(v) * n).floor().min(n - 1.0) / (n - 1.0))))
}

pub fn threshold(a: &Args) -> Result<Job> {
    let level = a.int("level")? as f32 / 255.0;
    Ok(pixel_job(move |p| {
        let v = if color::luma(p[0], p[1], p[2]) >= level - 1e-6 { 1.0 } else { 0.0 };
        [v; 3]
    }))
}

pub fn invert(_: &Args) -> Result<Job> {
    Ok(pixel_job(|p| p.map(|v| 1.0 - v)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contrast_curve_keeps_endpoints_and_mid() {
        for c in [-1.0, -0.3, 0.4, 1.0] {
            assert!((smooth_contrast(0.5, c) - 0.5).abs() < 1e-6);
        }
        assert!((smooth_contrast(0.0, 1.0)).abs() < 1e-6);
        assert!((smooth_contrast(1.0, 1.0) - 1.0).abs() < 1e-6);
        assert!(smooth_contrast(0.25, 0.5) < 0.25);
        assert!(smooth_contrast(0.25, -0.5) > 0.25);
    }

    #[test]
    fn level_map_math() {
        let m = LevelMap { in_black: 0.2, in_white: 0.8, inv_gamma: 1.0, out_black: 0.0, out_white: 1.0 };
        assert!((m.map(0.5) - 0.5).abs() < 1e-6);
        assert_eq!(m.map(0.1), 0.0);
        assert_eq!(m.map(0.9), 1.0);
        let inv = LevelMap { out_black: 1.0, out_white: 0.0, ..LevelMap::IDENTITY };
        assert!((inv.map(0.25) - 0.75).abs() < 1e-6);
    }
}
