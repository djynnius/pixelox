//! Film grain shared by `filter grain`, the grain adjustment layer, and the `fx grain`
//! layer style.
//!
//! Grain is band-limited Gaussian noise: white noise on a lattice anchored to absolute
//! coordinates (so a layer's grain does not change when a region of it is redrawn),
//! blurred to the grain size, and scaled back to unit variance with the exact discrete
//! kernel norm, so `--amount` means the same strength at every size. Large grain is built
//! on a coarser lattice and interpolated (the cost stays O(pixels)). `roughness` mixes in a
//! second, finer field; `color` mixes independent per-channel fields with the shared
//! (monochrome) one. The grain is added to the sRGB-encoded values, weighted by a film-like
//! response: strongest in the midtones, fading toward pure black and pure white (`tone`).

use crate::error::{Error, Result};
use crate::image::Image;
use crate::noise::{gauss, lane};
use rayon::prelude::*;
use serde_json::{Map, Value};

/// Grain settings. Every field is plain data so the same grain renders anywhere.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Grain {
    /// Strength, 0..100: the grain's standard deviation in the midtones is
    /// `amount / 100 × 0.12` of full scale.
    pub amount: f64,
    /// Grain size in px (about the diameter of one clump).
    pub size: f64,
    /// 0..100: 0 = soft round clumps only, 100 = crisp, mixed with a fine layer.
    pub roughness: f64,
    /// 0..100: 0 = monochrome (luminance) grain, 100 = independent per channel.
    pub color: f64,
    /// 0..100: how much the strength follows the tone (100 = none in pure black or white).
    pub tone: f64,
    /// The grain pattern.
    pub seed: u64,
}

impl Default for Grain {
    fn default() -> Grain {
        Grain { amount: 30.0, size: 1.5, roughness: 50.0, color: 0.0, tone: 60.0, seed: 0 }
    }
}

/// The automatic grain size for an image whose shorter side is `short` px: about 1.5 px
/// on a 1080 px poster, growing with the resolution so prints look alike.
pub fn auto_size(short: f64) -> f64 {
    (short / 720.0).clamp(1.0, 6.0)
}

/// A normalized 1-D Gaussian kernel and its radius.
fn kernel(sigma: f64) -> (Vec<f32>, usize) {
    let r = (3.0 * sigma).ceil().max(1.0) as usize;
    let mut k: Vec<f64> = (0..=2 * r)
        .map(|i| {
            let d = i as f64 - r as f64;
            (-(d * d) / (2.0 * sigma * sigma)).exp()
        })
        .collect();
    let s: f64 = k.iter().sum();
    k.iter_mut().for_each(|v| *v /= s);
    (k.into_iter().map(|v| v as f32).collect(), r)
}

/// A unit-variance band-limited noise plane for the `w`×`h` pixels whose (0, 0) sits at
/// `origin`, with blur `sigma` px.
fn plane(seed: u64, origin: (i64, i64), w: usize, h: usize, sigma: f64) -> Vec<f32> {
    if sigma < 0.3 {
        // white noise: one draw per pixel
        let mut out = vec![0f32; w * h];
        out.par_chunks_mut(w.max(1)).enumerate().for_each(|(y, row)| {
            for (x, v) in row.iter_mut().enumerate() {
                *v = gauss(seed, origin.0 + x as i64, origin.1 + y as i64);
            }
        });
        return out;
    }
    // lattice step: keep at least 1.5 lattice cells per sigma
    let step = ((sigma / 1.5).floor() as i64).max(1);
    let sg = sigma / step as f64;
    let (k, r) = kernel(sg);
    let norm = k.iter().map(|v| v * v).sum::<f32>(); // std of the 2-D blurred field
    let ri = r as i64;
    let gx0 = origin.0.div_euclid(step) - ri - 1;
    let gy0 = origin.1.div_euclid(step) - ri - 1;
    let gx1 = (origin.0 + w as i64).div_euclid(step) + ri + 2;
    let gy1 = (origin.1 + h as i64).div_euclid(step) + ri + 2;
    let (gw, gh) = ((gx1 - gx0) as usize, (gy1 - gy0) as usize);
    // white noise on the lattice, blurred horizontally (valid columns r..gw-r)
    let mut white = vec![0f32; gw * gh];
    white.par_chunks_mut(gw).enumerate().for_each(|(j, row)| {
        for (i, v) in row.iter_mut().enumerate() {
            *v = gauss(seed, gx0 + i as i64, gy0 + j as i64);
        }
    });
    let mut hb = vec![0f32; gw * gh];
    hb.par_chunks_mut(gw).enumerate().for_each(|(j, row)| {
        let src = &white[j * gw..(j + 1) * gw];
        for i in r..gw.saturating_sub(r) {
            let mut s = 0f32;
            for (t, kv) in k.iter().enumerate() {
                s += src[i + t - r] * kv;
            }
            row[i] = s;
        }
    });
    let mut g = vec![0f32; gw * gh];
    g.par_chunks_mut(gw).enumerate().for_each(|(j, row)| {
        if j < r || j + r >= gh {
            return;
        }
        for i in r..gw.saturating_sub(r) {
            let mut s = 0f32;
            for (t, kv) in k.iter().enumerate() {
                s += hb[(j + t - r) * gw + i] * kv;
            }
            row[i] = s / norm;
        }
    });
    if step == 1 {
        let mut out = vec![0f32; w * h];
        let (ox, oy) = ((origin.0 - gx0) as usize, (origin.1 - gy0) as usize);
        out.par_chunks_mut(w.max(1)).enumerate().for_each(|(y, row)| {
            let src = &g[(oy + y) * gw + ox..(oy + y) * gw + ox + w];
            row.copy_from_slice(src);
        });
        return out;
    }
    // bilinear from the lattice (lattice point i sits at absolute (gx0 + i + 0.5) × step)
    let st = step as f64;
    let mut out = vec![0f32; w * h];
    out.par_chunks_mut(w.max(1)).enumerate().for_each(|(y, row)| {
        let fy = (origin.1 as f64 + y as f64 + 0.5) / st - 0.5 - gy0 as f64;
        let j0 = fy.floor();
        let ty = (fy - j0) as f32;
        let j0 = j0 as usize;
        for (x, v) in row.iter_mut().enumerate() {
            let fx = (origin.0 as f64 + x as f64 + 0.5) / st - 0.5 - gx0 as f64;
            let i0 = fx.floor();
            let tx = (fx - i0) as f32;
            let i0 = i0 as usize;
            let a = g[j0 * gw + i0] + (g[j0 * gw + i0 + 1] - g[j0 * gw + i0]) * tx;
            let b = g[(j0 + 1) * gw + i0] + (g[(j0 + 1) * gw + i0 + 1] - g[(j0 + 1) * gw + i0]) * tx;
            // bilinear interpolation of a smooth field keeps nearly all of its variance
            *v = a + (b - a) * ty;
        }
    });
    out
}

/// Film response: 1 in the midtones, falling to `1 − tone` at pure black and white.
#[inline]
fn response(l: f32, tone: f32) -> f32 {
    let bump = (4.0 * l * (1.0 - l)).max(0.0).powf(0.6);
    1.0 - tone + tone * bump
}

impl Grain {
    /// The grain field for one lane (0 = shared, 1..3 = R, G, B), unit variance.
    fn field(&self, k: u64, origin: (i64, i64), w: usize, h: usize) -> Vec<f32> {
        let sigma = 0.42 * self.size.max(0.1);
        let r = (self.roughness / 100.0).clamp(0.0, 1.0) as f32;
        let coarse = plane(lane(self.seed, 2 * k), origin, w, h, sigma);
        if r <= 0.0 {
            return coarse;
        }
        let fine = plane(lane(self.seed, 2 * k + 1), origin, w, h, sigma * 0.4);
        let n = ((1.0 - r) * (1.0 - r) + r * r).sqrt();
        coarse.iter().zip(&fine).map(|(c, f)| ((1.0 - r) * c + r * f) / n).collect()
    }

    /// Add the grain to `img`, whose pixel (0, 0) sits at `origin` in grain coordinates.
    /// Only color changes; transparent pixels stay untouched.
    pub fn apply(&self, img: &mut Image, origin: (i64, i64)) {
        let (w, h) = (img.width as usize, img.height as usize);
        if self.amount <= 0.0 || w == 0 || h == 0 {
            return;
        }
        let amp = (self.amount / 100.0 * 0.12) as f32;
        let c = (self.color / 100.0).clamp(0.0, 1.0) as f32;
        let tone = (self.tone / 100.0).clamp(0.0, 1.0) as f32;
        let mono = if c < 1.0 { Some(self.field(0, origin, w, h)) } else { None };
        let chroma: Vec<Vec<f32>> =
            if c > 0.0 { (1..=3).map(|k| self.field(k, origin, w, h)).collect() } else { vec![] };
        let norm = ((1.0 - c) * (1.0 - c) + c * c).sqrt();
        img.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
            for x in 0..w {
                let p = &mut row[x * 4..x * 4 + 4];
                if p[3] <= 0.0 {
                    continue;
                }
                let i = y * w + x;
                let l = 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2];
                let k = amp * response(l, tone);
                let m = mono.as_ref().map_or(0.0, |m| m[i]);
                for ch in 0..3 {
                    let n = if c > 0.0 { ((1.0 - c) * m + c * chroma[ch][i]) / norm } else { m };
                    p[ch] = (p[ch] + n * k).clamp(0.0, 1.0);
                }
            }
        });
    }

    /// Read grain settings from stored params (an adjustment layer's or a style's):
    /// `amount`, `size` (px; `auto` uses `short`), `roughness`, `color`, `tone`, and
    /// `variant` (the pattern). Strings and numbers are both accepted.
    pub fn from_params(params: &Map<String, Value>, short: f64) -> Result<Grain> {
        let d = Grain::default();
        let num = |key: &str, def: f64| -> Result<f64> {
            match params.get(key) {
                None | Some(Value::Null) => Ok(def),
                Some(Value::Number(n)) => Ok(n.as_f64().unwrap_or(def)),
                Some(Value::String(s)) if s.trim() == "auto" => Ok(def),
                Some(Value::String(s)) => s.trim().trim_end_matches('%').parse::<f64>().map_err(|_| {
                    Error::new("PX1010", format!("grain: {key} '{s}' is not a number")).with_context("param", key)
                }),
                Some(other) => Err(Error::new("PX1010", format!("grain: {key} {other} is not a number"))),
            }
        };
        Ok(Grain {
            amount: num("amount", d.amount)?.clamp(0.0, 400.0),
            size: num("size", auto_size(short))?.clamp(0.1, 200.0),
            roughness: num("roughness", d.roughness)?.clamp(0.0, 100.0),
            color: num("color", d.color)?.clamp(0.0, 100.0),
            tone: num("tone", d.tone)?.clamp(0.0, 100.0),
            seed: num("variant", 0.0)?.max(0.0) as u64,
        })
    }

    /// The params that [`Grain::from_params`] reads back.
    pub fn to_params(&self) -> Map<String, Value> {
        let mut m = Map::new();
        let f = |v: f64| Value::String(format!("{}", (v * 1e6).round() / 1e6));
        m.insert("amount".into(), f(self.amount));
        m.insert("size".into(), f(self.size));
        m.insert("roughness".into(), f(self.roughness));
        m.insert("color".into(), f(self.color));
        m.insert("tone".into(), f(self.tone));
        m.insert("variant".into(), Value::String(self.seed.to_string()));
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Rgba;

    fn std_of(img: &Image, ch: usize) -> f32 {
        let v: Vec<f32> = img.data.chunks(4).map(|p| p[ch]).collect();
        let m = v.iter().sum::<f32>() / v.len() as f32;
        (v.iter().map(|x| (x - m) * (x - m)).sum::<f32>() / v.len() as f32).sqrt()
    }

    #[test]
    fn strength_does_not_depend_on_size() {
        for size in [0.5, 1.5, 4.0, 12.0] {
            let mut img = Image::filled(160, 160, Rgba::new(0.5, 0.5, 0.5, 1.0));
            Grain { size, seed: 3, ..Grain::default() }.apply(&mut img, (0, 0));
            let sd = std_of(&img, 1);
            assert!((sd - 0.036).abs() < 0.009, "size {size}: std {sd}");
        }
    }

    #[test]
    fn grain_is_anchored_and_tone_weighted() {
        let g = Grain { seed: 9, size: 3.0, ..Grain::default() };
        let mut a = Image::filled(40, 30, Rgba::new(0.5, 0.5, 0.5, 1.0));
        g.apply(&mut a, (100, 50));
        // the same region drawn as part of a larger image matches
        let mut b = Image::filled(60, 50, Rgba::new(0.5, 0.5, 0.5, 1.0));
        g.apply(&mut b, (90, 40));
        for y in 0..30 {
            for x in 0..40 {
                assert!((a.get(x, y)[0] - b.get(x + 10, y + 10)[0]).abs() < 1e-5);
            }
        }
        // near-black gets much less grain than midtones; monochrome keeps gray gray
        let mut dark = Image::filled(80, 80, Rgba::new(0.02, 0.02, 0.02, 1.0));
        Grain { tone: 100.0, ..g }.apply(&mut dark, (0, 0));
        assert!(std_of(&dark, 0) < 0.012, "{}", std_of(&dark, 0));
        let p = a.get(5, 5);
        assert_eq!(p[0], p[1]);
        // color grain separates the channels; transparent pixels stay untouched
        let mut c = Image::filled(20, 20, Rgba::new(0.5, 0.5, 0.5, 1.0));
        c.set(0, 0, [0.0; 4]);
        Grain { color: 100.0, ..g }.apply(&mut c, (0, 0));
        let p = c.get(7, 7);
        assert!(p[0] != p[1] || p[1] != p[2]);
        assert_eq!(c.get(0, 0), [0.0; 4]);
    }

    #[test]
    fn params_round_trip() {
        let g = Grain { amount: 42.0, size: 2.5, roughness: 10.0, color: 30.0, tone: 80.0, seed: 77 };
        assert_eq!(Grain::from_params(&g.to_params(), 1000.0).unwrap(), g);
        let auto = Grain::from_params(&Map::new(), 1440.0).unwrap();
        assert_eq!(auto.size, 2.0);
    }
}
