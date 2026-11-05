//! Local-contrast adjustments from the Lightroom / Camera Raw family: clarity (large-scale
//! midtone contrast), texture (medium-sized surface detail), and dehaze (the dark channel
//! prior of He, Sun & Tang, 2009).
//!
//! All three are spatial. Their neighborhoods scale with the image (a fraction of the
//! canvas's long side), so a setting looks alike at 1 and at 24 megapixels, and every
//! filter is a Gaussian (three running boxes above σ = 4) or a sliding-window minimum, so
//! the cost is O(pixels) whatever the radius. Clarity and texture work on perceptual
//! luminance (Rec. 709 luminance of the linear colors, sRGB-encoded) and write the new
//! luminance back by scaling the linear color, which keeps each pixel's hue; dehaze inverts
//! the haze model I = J·t + A·(1 − t) in linear light, where it holds physically.
//! Neighborhood averages are alpha-weighted, so transparent pixels never leak into visible
//! ones. Alpha is never changed. Every pass is per pixel or per row and every reduction is
//! an integer histogram or a row-ordered sum, so the thread count never changes the output.

use crate::adjust::Job;
use crate::util::{bin_of, clamp01, map_rgb_indexed, num, per_row, round_to, sum_rows, HBINS};
use pixelox_core::color::{linear_to_srgb, srgb_to_linear};
use pixelox_core::kernels::{gaussian_planar, Edge};
use pixelox_core::prelude::*;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Facts a job learns while it runs (merged into the op's JSON result).
pub type Report = Arc<Mutex<Map<String, Value>>>;

/// Clarity's base radius (Gaussian σ) as a fraction of the long side, and its floor in px.
const CLARITY_SCALE: f64 = 0.015;
const CLARITY_MIN: f64 = 2.0;
/// Texture's largest detail (Gaussian σ) as a fraction of the long side, and its floor.
const TEXTURE_SCALE: f64 = 0.0025;
const TEXTURE_MIN: f64 = 1.5;
/// Dehaze's dark-channel patch radius as a fraction of the long side (He et al. used
/// 15×15 px on 600 px images).
const PATCH_SCALE: f64 = 0.012;
/// The transmission map is refined by a guided filter this many patch radii wide (σ). The
/// opened dark channel already follows the scene's outlines, so a tight window suffices
/// (wider ones bend t across depth edges and copy texture into it).
const REFINE_SCALE: f64 = 0.5;

/// Guided-filter regularization, a variance of perceptual luminance: neighborhoods whose
/// spread is well above √eps count as edges and keep their contrast (no halos).
const CLARITY_EPS: f32 = 0.012;
const TEXTURE_EPS: f32 = 0.005;
const TRANSMISSION_EPS: f32 = 1e-3;
/// Detail gain at amount +100 (negative amounts remove up to all of the detail).
const CLARITY_GAIN: f32 = 1.0;
const TEXTURE_GAIN: f32 = 1.5;
/// Texture ignores pixel-level noise: its band is blurred by this σ (px).
const TEXTURE_FINE: f64 = 0.8;
/// He et al.'s ω at amount 100 (a little haze is kept for depth) and transmission floor t0.
const OMEGA: f32 = 0.95;
const T0: f32 = 0.1;
/// Negative dehaze: the most it blends toward A, and the share every pixel gets regardless
/// of its estimated depth (a veil even on near objects).
const HAZE_MAX: f32 = 0.9;
const HAZE_FLOOR: f32 = 0.25;

// ---------------------------------------------------------------------------------------
// scale

/// The size-dependent param of `op` (`radius` or `patch`) and its `auto` value for an
/// image whose long side is `long` px.
pub fn auto_scale(op: &str, long: f64) -> Option<(&'static str, f64)> {
    match op {
        "clarity" => Some(("radius", round_to((long * CLARITY_SCALE).max(CLARITY_MIN), 2))),
        "texture" => Some(("radius", round_to((long * TEXTURE_SCALE).max(TEXTURE_MIN), 2))),
        "dehaze" => Some(("patch", (long * PATCH_SCALE).round().max(1.0))),
        _ => None,
    }
}

/// Replace an `auto` radius or patch in `a` with px for a canvas whose long side is
/// `long`. Returns the param and the value that will be used (None for other ops).
pub fn resolve_scale(op: &str, a: &mut Args, long: f64) -> Result<Option<(&'static str, f64)>> {
    let Some((name, auto)) = auto_scale(op, long) else { return Ok(None) };
    match a.opt_f64(name)? {
        Some(v) => Ok(Some((name, v))),
        None => {
            a.set(name, num(auto, 2));
            Ok(Some((name, auto)))
        }
    }
}

/// The result entry of a local adjustment (`{"dehaze": {...}}`): the scale it used plus
/// whatever its job reported. Empty for other ops.
pub fn info(op: &str, scale: Option<(&'static str, f64)>, report: Option<Map<String, Value>>) -> Map<String, Value> {
    let mut out = Map::new();
    let Some((name, v)) = scale else { return out };
    let mut m = Map::new();
    // the patch is a whole number of px; radii may be fractional
    m.insert(name.into(), if name == "patch" { json!(v.round() as i64) } else { json!(v) });
    m.extend(report.unwrap_or_default());
    out.insert(op.into(), Value::Object(m));
    out
}

fn long_side(img: &Image) -> f64 {
    img.width.max(img.height) as f64
}

fn auto_px(op: &str, img: &Image) -> f64 {
    auto_scale(op, long_side(img)).map(|(_, v)| v).unwrap_or(1.0)
}

// ---------------------------------------------------------------------------------------
// planar helpers

#[inline]
fn lin3(p: &[f32]) -> [f32; 3] {
    [srgb_to_linear(clamp01(p[0])), srgb_to_linear(clamp01(p[1])), srgb_to_linear(clamp01(p[2]))]
}

/// Rec. 709 luminance of linear RGB.
#[inline]
fn lum709(c: [f32; 3]) -> f32 {
    0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]
}

/// Perceptual luminance per pixel: the linear luminance, sRGB-encoded (0..1).
fn luminance(img: &Image) -> Vec<f32> {
    img.data.par_chunks_exact(4).map(|p| linear_to_srgb(lum709(lin3(p)))).collect()
}

/// Alpha per pixel, or None for opaque images (plain averages are then exact and cheaper).
fn alpha_of(img: &Image) -> Option<Vec<f32>> {
    if img.is_opaque() {
        None
    } else {
        Some(img.data.par_chunks_exact(4).map(|p| p[3]).collect())
    }
}

/// Gaussian local means of each field, alpha-weighted when `alpha` is given (a pixel with
/// no visible neighbors keeps its own value). Edges clamp.
fn local_means(fields: &[&[f32]], alpha: Option<&[f32]>, w: usize, h: usize, sigma: f64) -> Vec<Vec<f32>> {
    let n = fields.len();
    let c = n + usize::from(alpha.is_some());
    let mut buf = vec![0f32; w * h * c];
    buf.par_chunks_mut(w * c).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let i = y * w + x;
            let a = alpha.map_or(1.0, |al| al[i]);
            for (k, f) in fields.iter().enumerate() {
                row[x * c + k] = f[i] * a;
            }
            if alpha.is_some() {
                row[x * c + n] = a;
            }
        }
    });
    gaussian_planar(&mut buf, w, h, c, sigma, sigma, Edge::Clamp);
    (0..n)
        .map(|k| {
            buf.par_chunks_exact(c)
                .zip(fields[k].par_iter())
                .map(|(p, own)| match alpha {
                    None => p[k],
                    Some(_) if p[n] > 1e-6 => p[k] / p[n],
                    Some(_) => *own,
                })
                .collect()
        })
        .collect()
}

/// He et al.'s guided filter with a Gaussian window of `sigma`: `src` (the guide itself when
/// None) smoothed by the local linear model q = a·guide + b, which follows the guide's
/// edges wherever its local variance is well above `eps`.
fn guided(
    guide: &[f32],
    src: Option<&[f32]>,
    alpha: Option<&[f32]>,
    w: usize,
    h: usize,
    sigma: f64,
    eps: f32,
) -> Vec<f32> {
    let sq: Vec<f32> = guide.par_iter().map(|g| g * g).collect();
    let (ca, cb): (Vec<f32>, Vec<f32>) = match src {
        None => {
            let m = local_means(&[guide, &sq], alpha, w, h, sigma);
            m[0].par_iter()
                .zip(m[1].par_iter())
                .map(|(&mg, &mgg)| {
                    let var = (mgg - mg * mg).max(0.0);
                    let a = var / (var + eps);
                    (a, mg * (1.0 - a))
                })
                .unzip()
        }
        Some(p) => {
            let gp: Vec<f32> = guide.par_iter().zip(p.par_iter()).map(|(g, v)| g * v).collect();
            let m = local_means(&[guide, p, &gp, &sq], alpha, w, h, sigma);
            (0..w * h)
                .into_par_iter()
                .map(|i| {
                    let var = (m[3][i] - m[0][i] * m[0][i]).max(0.0);
                    let cov = m[2][i] - m[0][i] * m[1][i];
                    let a = cov / (var + eps);
                    (a, m[1][i] - a * m[0][i])
                })
                .unzip()
        }
    };
    let m = local_means(&[&ca, &cb], alpha, w, h, sigma);
    guide.par_iter().enumerate().map(|(i, g)| m[0][i] * g + m[1][i]).collect()
}

fn transpose(src: &[f32], w: usize, h: usize) -> Vec<f32> {
    let mut out = vec![0f32; w * h];
    out.par_chunks_mut(h).enumerate().for_each(|(x, col)| {
        for (y, v) in col.iter_mut().enumerate() {
            *v = src[y * w + x];
        }
    });
    out
}

/// Sliding minimum of `2r + 1` samples along each row (van Herk / Gil–Werman: three
/// comparisons per sample whatever `r`); samples beyond the ends do not count.
fn min_rows(data: &mut [f32], w: usize, r: usize) {
    let k = 2 * r + 1;
    let n = w + 2 * r;
    data.par_chunks_mut(w).for_each_init(
        || (vec![0f32; n], vec![0f32; n], vec![0f32; n]),
        |(pad, fwd, bwd), row| {
            pad.fill(f32::INFINITY);
            pad[r..r + w].copy_from_slice(row);
            for i in 0..n {
                fwd[i] = if i % k == 0 { pad[i] } else { fwd[i - 1].min(pad[i]) };
            }
            for i in (0..n).rev() {
                bwd[i] = if i == n - 1 || (i + 1) % k == 0 { pad[i] } else { bwd[i + 1].min(pad[i]) };
            }
            for (x, o) in row.iter_mut().enumerate() {
                *o = bwd[x].min(fwd[x + k - 1]);
            }
        },
    );
}

/// Minimum over a (2r + 1)² square, separably.
fn min_filter(data: &mut [f32], w: usize, h: usize, r: usize) {
    if r == 0 || w == 0 || h == 0 {
        return;
    }
    min_rows(data, w, r);
    let mut t = transpose(data, w, h);
    min_rows(&mut t, h, r);
    data.copy_from_slice(&transpose(&t, h, w));
}

/// Give pixel `i` the perceptual luminance `target(i)` (currently `l[i]`) by scaling its
/// linear color, which keeps its hue; a color pushed out of gamut moves toward the gray of
/// its new luminance instead of clipping channel by channel.
fn set_luminance(img: &mut Image, l: &[f32], target: impl Fn(usize) -> f32 + Sync) {
    // below this linear luminance the change becomes additive (so black can lift to gray)
    const E: f32 = 1e-3;
    map_rgb_indexed(img, |i, p| {
        let t = clamp01(target(i));
        if (t - l[i]).abs() < 1e-6 {
            return p;
        }
        let c = lin3(&p);
        let (y, y2) = (lum709(c), srgb_to_linear(t));
        let k = (y2 + E) / (y + E);
        let add = E * (y2 - y) / (y + E);
        let mut o = c.map(|v| v * k + add);
        let (hi, lo) = (o[0].max(o[1]).max(o[2]), o[0].min(o[1]).min(o[2]));
        let mut s = 1.0f32;
        if hi > 1.0 {
            s = s.min((1.0 - y2) / (hi - y2));
        }
        if lo < 0.0 {
            s = s.min(y2 / (y2 - lo));
        }
        if s < 1.0 {
            let s = s.max(0.0);
            o = o.map(|v| y2 + (v - y2) * s);
        }
        o.map(|v| linear_to_srgb(clamp01(v)))
    });
}

// ---------------------------------------------------------------------------------------
// clarity and texture

/// Clarity's tonal weight: 1 through the midtones, falling to 0 at black and white.
#[inline]
fn midtones(l: f32) -> f32 {
    let d = 2.0 * clamp01(l) - 1.0;
    let d2 = d * d;
    1.0 - d2 * d2
}

fn gain(amount: f32, boost: f32) -> f32 {
    if amount > 0.0 {
        amount * boost
    } else {
        amount
    }
}

pub fn clarity(a: &Args) -> Result<Job> {
    let amount = a.f32("amount")? / 100.0;
    let radius = a.opt_f64("radius")?;
    Ok(Box::new(move |img: &mut Image| {
        if img.is_empty() || amount == 0.0 {
            return;
        }
        let sigma = radius.unwrap_or_else(|| auto_px("clarity", img));
        let (w, h) = (img.width as usize, img.height as usize);
        let alpha = alpha_of(img);
        let l = luminance(img);
        let base = guided(&l, None, alpha.as_deref(), w, h, sigma, CLARITY_EPS);
        let g = gain(amount, CLARITY_GAIN);
        set_luminance(img, &l, |i| l[i] + g * midtones(base[i]) * (l[i] - base[i]));
    }))
}

pub fn texture(a: &Args) -> Result<Job> {
    let amount = a.f32("amount")? / 100.0;
    let radius = a.opt_f64("radius")?;
    Ok(Box::new(move |img: &mut Image| {
        if img.is_empty() || amount == 0.0 {
            return;
        }
        let sigma = radius.unwrap_or_else(|| auto_px("texture", img));
        let (w, h) = (img.width as usize, img.height as usize);
        let alpha = alpha_of(img);
        let l = luminance(img);
        // detail below the medium scale, minus strong edges (the guided base keeps them) ...
        let base = guided(&l, None, alpha.as_deref(), w, h, sigma, TEXTURE_EPS);
        let detail: Vec<f32> = l.par_iter().zip(base.par_iter()).map(|(v, b)| v - b).collect();
        // ... minus pixel-level noise
        let band = local_means(&[&detail], alpha.as_deref(), w, h, TEXTURE_FINE.min(sigma / 2.0)).remove(0);
        let g = gain(amount, TEXTURE_GAIN);
        set_luminance(img, &l, |i| l[i] + g * band[i]);
    }))
}

// ---------------------------------------------------------------------------------------
// dehaze

/// The dark channel: each pixel's smallest linear channel times `scale` (1/A per channel
/// for dark(I/A)), min-filtered over a (2r + 1)² square. With `open`, a max filter of the
/// same size follows (a morphological opening): each pixel then takes the darkest value of
/// the best patch that contains it rather than of the patch centered on it, so the dark
/// channel keeps the scene's outlines instead of spreading dark objects r px into the sky.
/// Fully transparent pixels do not count (a window with no visible pixel stays ±∞).
fn dark_channel(img: &Image, scale: [f32; 3], r: usize, open: bool) -> Vec<f32> {
    let (w, h) = (img.width as usize, img.height as usize);
    let mut m: Vec<f32> = img
        .data
        .par_chunks_exact(4)
        .map(|p| {
            if p[3] <= 0.0 {
                f32::INFINITY
            } else {
                let c = lin3(p);
                (c[0] * scale[0]).min(c[1] * scale[1]).min(c[2] * scale[2])
            }
        })
        .collect();
    min_filter(&mut m, w, h, r);
    if open {
        // max filter = min filter of the negation; transparent pixels stay out of it
        m.par_iter_mut().zip(img.data.par_chunks_exact(4)).for_each(|(v, p)| {
            *v = if p[3] <= 0.0 { f32::INFINITY } else { -*v };
        });
        min_filter(&mut m, w, h, r);
        m.par_iter_mut().for_each(|v| *v = -*v);
    }
    m
}

/// He et al.'s atmospheric light A (linear RGB): the alpha-weighted mean color of the
/// pixels in the brightest 0.1% of the dark channel. None when nothing is visible.
pub fn atmosphere(img: &Image, patch: usize) -> Option<[f32; 3]> {
    if img.is_empty() {
        return None;
    }
    let w = img.width as usize;
    let dark = dark_channel(img, [1.0; 3], patch, false);
    // integer counts merge exactly: the threshold never depends on the thread split
    let hist = dark
        .par_chunks(w)
        .zip(img.data.par_chunks(w * 4))
        .fold(
            || vec![0u64; HBINS],
            |mut hst, (d, row)| {
                for (v, p) in d.iter().zip(row.chunks_exact(4)) {
                    if p[3] > 0.0 {
                        hst[bin_of(*v, HBINS)] += 1;
                    }
                }
                hst
            },
        )
        .reduce(
            || vec![0u64; HBINS],
            |mut a, b| {
                for (x, y) in a.iter_mut().zip(b) {
                    *x += y;
                }
                a
            },
        );
    let total: u64 = hist.iter().sum();
    if total == 0 {
        return None;
    }
    let need = ((total as f64 * 0.001).ceil() as u64).max(1);
    let mut acc = 0u64;
    let mut threshold = 0;
    for b in (0..HBINS).rev() {
        acc += hist[b];
        if acc >= need {
            threshold = b;
            break;
        }
    }
    let rows = per_row(img, |y, row| {
        let mut s = [0f64; 4];
        for (x, p) in row.chunks_exact(4).enumerate() {
            if p[3] > 0.0 && bin_of(dark[y * w + x], HBINS) >= threshold {
                let c = lin3(p);
                let a = p[3] as f64;
                for k in 0..3 {
                    s[k] += c[k] as f64 * a;
                }
                s[3] += a;
            }
        }
        s
    });
    let s = sum_rows(&rows);
    (s[3] > 0.0).then(|| [0, 1, 2].map(|k| (s[k] / s[3]) as f32))
}

/// A linear color as the `#rrggbb` an `--atmosphere` value takes.
pub fn hex_of_linear(c: [f32; 3]) -> String {
    Rgba::new(linear_to_srgb(c[0]), linear_to_srgb(c[1]), linear_to_srgb(c[2]), 1.0).to_hex()
}

/// Dehaze settings read from args.
#[derive(Clone, Copy)]
struct Haze {
    /// −1..1
    amount: f32,
    /// Linear RGB; None = detect.
    atmosphere: Option<[f32; 3]>,
    /// Dark-channel patch radius in px; None = from the image size.
    patch: Option<usize>,
}

/// What dehaze estimated on one image.
struct Estimate {
    atmosphere: [f32; 3],
    patch: usize,
    /// Refined transmission per pixel, 0..1.
    t: Vec<f32>,
    /// Alpha-weighted mean of `t`.
    mean_t: f64,
}

impl Estimate {
    fn report(&self) -> Map<String, Value> {
        let mut m = Map::new();
        m.insert("patch".into(), json!(self.patch));
        m.insert("atmosphere".into(), json!(hex_of_linear(self.atmosphere)));
        m.insert("mean_transmission".into(), json!(round_to(self.mean_t, 4)));
        m
    }
}

impl Haze {
    fn from_args(a: &Args) -> Result<Haze> {
        let atmosphere = if a.is_auto("atmosphere") || !a.has("atmosphere") {
            None
        } else {
            let c = a.color("atmosphere")?;
            Some([c.r, c.g, c.b].map(|v| srgb_to_linear(clamp01(v))))
        };
        let patch = a.opt_int("patch")?.map(|p| p.max(1) as usize);
        Ok(Haze { amount: a.f32("amount")? / 100.0, atmosphere, patch })
    }

    /// The atmospheric light and refined transmission map (None when there is nothing to
    /// do: amount 0, or no visible pixels to estimate A from).
    fn estimate(&self, img: &Image) -> Option<Estimate> {
        if img.is_empty() || self.amount == 0.0 {
            return None;
        }
        let patch = self.patch.unwrap_or_else(|| auto_px("dehaze", img) as usize);
        let atm = match self.atmosphere {
            Some(c) => c,
            None => atmosphere(img, patch)?,
        };
        // removing haze keeps ω·amount of the estimated haze out; adding haze uses the
        // full estimate as a depth proxy
        let omega = if self.amount > 0.0 { OMEGA * self.amount } else { OMEGA };
        let (w, h) = (img.width as usize, img.height as usize);
        let inv = atm.map(|v| 1.0 / v.max(1e-3));
        let raw: Vec<f32> = dark_channel(img, inv, patch, true)
            .into_par_iter()
            .map(|d| (1.0 - omega * d.min(1.0)).clamp(0.0, 1.0))
            .collect();
        let alpha = alpha_of(img);
        let l = luminance(img);
        let sigma = (REFINE_SCALE * patch as f64).max(1.0);
        let t: Vec<f32> = guided(&l, Some(&raw), alpha.as_deref(), w, h, sigma, TRANSMISSION_EPS)
            .into_par_iter()
            .map(|v| v.clamp(0.0, 1.0))
            .collect();
        let rows = per_row(img, |y, row| {
            let mut s = [0f64; 2];
            for (x, p) in row.chunks_exact(4).enumerate() {
                s[0] += t[y * w + x] as f64 * p[3] as f64;
                s[1] += p[3] as f64;
            }
            s
        });
        let [sum, weight] = sum_rows(&rows);
        let mean_t = if weight > 0.0 { sum / weight } else { 1.0 };
        Some(Estimate { atmosphere: atm, patch, t, mean_t })
    }

    fn apply(&self, img: &mut Image, e: &Estimate) {
        let atm = e.atmosphere;
        if self.amount > 0.0 {
            // J = (I − A) / max(t, t0) + A
            map_rgb_indexed(img, |i, p| {
                let t = e.t[i].max(T0);
                let c = lin3(&p);
                [0, 1, 2].map(|k| linear_to_srgb(((c[k] - atm[k]) / t + atm[k]).clamp(0.0, 1.0)))
            });
        } else {
            // more haze: blend toward A, more where the transmission is low (far away)
            let s = -self.amount * HAZE_MAX;
            map_rgb_indexed(img, |i, p| {
                let k = s * (HAZE_FLOOR + (1.0 - HAZE_FLOOR) * (1.0 - e.t[i]));
                let c = lin3(&p);
                [0, 1, 2].map(|j| linear_to_srgb(c[j] + (atm[j] - c[j]) * k))
            });
        }
    }
}

pub fn dehaze(a: &Args) -> Result<Job> {
    dehaze_reporting(a).map(|(job, _)| job)
}

/// The dehaze job and the slot where it reports A, the patch, and the mean transmission.
pub fn dehaze_reporting(a: &Args) -> Result<(Job, Report)> {
    let haze = Haze::from_args(a)?;
    let report: Report = Arc::default();
    let slot = report.clone();
    let job: Job = Box::new(move |img: &mut Image| {
        let Some(e) = haze.estimate(img) else { return };
        if let Ok(mut m) = slot.lock() {
            *m = e.report();
        }
        haze.apply(img, &e);
    });
    Ok((job, report))
}

/// A, the patch, and the mean transmission dehaze would use on `img`, without changing it
/// (for the result of `--as-layer`, which only adds a layer).
pub fn dehaze_summary(a: &Args, img: &Image) -> Result<Option<Map<String, Value>>> {
    Ok(Haze::from_args(a)?.estimate(img).map(|e| e.report()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_filter_matches_brute_force() {
        let (w, h) = (13, 7);
        let src: Vec<f32> = (0..w * h).map(|i| ((i * 37 + 11) % 23) as f32).collect();
        for r in [1, 2, 5, 9] {
            let mut fast = src.clone();
            min_filter(&mut fast, w, h, r);
            for y in 0..h {
                for x in 0..w {
                    let mut m = f32::INFINITY;
                    for yy in y.saturating_sub(r)..(y + r + 1).min(h) {
                        for xx in x.saturating_sub(r)..(x + r + 1).min(w) {
                            m = m.min(src[yy * w + xx]);
                        }
                    }
                    assert_eq!(fast[y * w + x], m, "r {r} at {x},{y}");
                }
            }
        }
    }

    #[test]
    fn opened_dark_channel_keeps_outlines() {
        // dark ground on the left, bright sky on the right, a 1 px bright speck in the ground
        let img = Image::from_fn(24, 9, |x, y| {
            let v = if x >= 12 || (x == 4 && y == 4) { 1.0 } else { 0.1 };
            [v, v, v, 1.0]
        });
        let r = 3;
        let eroded = dark_channel(&img, [1.0; 3], r, false);
        let opened = dark_channel(&img, [1.0; 3], r, true);
        let low = srgb_to_linear(0.1);
        for y in 0..9 {
            // erosion spreads the ground r px into the sky; the opening does not
            assert_eq!(eroded[y * 24 + 13], low);
            assert_eq!(opened[y * 24 + 12], 1.0);
            // both drop the speck (smaller than a patch)
            assert_eq!(opened[y * 24 + 4], low);
        }
        // transparent pixels never count
        let mut holey = img.clone();
        holey.set(20, 4, [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(dark_channel(&holey, [1.0; 3], r, true)[4 * 24 + 21], 1.0);
    }

    #[test]
    fn guided_filter_keeps_edges_and_flattens_ripples() {
        let (w, h) = (40, 4);
        // a big step at x = 20 with a small ripple on the left
        let g: Vec<f32> = (0..w * h)
            .map(|i| {
                let x = i % w;
                if x < 20 {
                    0.3 + if x % 2 == 0 { 0.02 } else { -0.02 }
                } else {
                    0.9
                }
            })
            .collect();
        let q = guided(&g, None, None, w, h, 3.0, 0.01);
        // the ripple is smoothed away ...
        assert!((q[w + 5] - q[w + 6]).abs() < 0.01, "{} {}", q[w + 5], q[w + 6]);
        // ... the step survives
        assert!(q[w + 22] - q[w + 17] > 0.5, "{} {}", q[w + 17], q[w + 22]);
    }

    #[test]
    fn luminance_round_trips_and_keeps_hue() {
        let mut img = Image::from_fn(4, 1, |x, _| [0.6, 0.3 + 0.1 * x as f32, 0.2, 1.0]);
        let l = luminance(&img);
        let before = img.clone();
        set_luminance(&mut img, &l, |i| l[i] + 0.1);
        let after = luminance(&img);
        for i in 0..4 {
            assert!((after[i] - (l[i] + 0.1)).abs() < 2e-3, "{} vs {}", after[i], l[i] + 0.1);
            let (p, q) = (before.get(i as u32, 0), img.get(i as u32, 0));
            let (h0, _, _) = color::rgb_to_hsl(p[0], p[1], p[2]);
            let (h1, _, _) = color::rgb_to_hsl(q[0], q[1], q[2]);
            assert!((h0 - h1).abs() < 2.0, "hue {h0} → {h1}");
        }
        // black lifts to neutral gray
        let mut black = Image::filled(1, 1, Rgba::BLACK);
        set_luminance(&mut black, &[0.0], |_| 0.5);
        let p = black.get(0, 0);
        assert!((p[0] - 0.5).abs() < 2e-3 && p[0] == p[1] && p[1] == p[2], "{p:?}");
    }

    #[test]
    fn auto_scales_follow_the_long_side() {
        assert_eq!(auto_scale("clarity", 4000.0), Some(("radius", 60.0)));
        assert_eq!(auto_scale("clarity", 32.0), Some(("radius", 2.0)));
        assert_eq!(auto_scale("texture", 6000.0), Some(("radius", 15.0)));
        assert_eq!(auto_scale("dehaze", 600.0), Some(("patch", 7.0)));
        assert_eq!(auto_scale("dehaze", 10.0), Some(("patch", 1.0)));
        assert_eq!(auto_scale("levels", 100.0), None);
    }
}
