//! Artistic ("photo to art") filters: `filter pencil`, `filter charcoal`, `filter ink`
//! (sketch.rs), `filter oil-paint`, `filter watercolor` (paint.rs), `filter cartoon`,
//! `filter cutout`, `filter halftone` (graphic.rs).
//!
//! Sizes scale with the layer: the unit is √(w·h)/1000 px (1 px at one megapixel), so a
//! 1 MP and a 24 MP photo look alike. `auto` sizes are recorded with
//! `ctx.resolve_param` so a replay reproduces them.
//!
//! Cost: every filter is O(pixels). Gaussians and line (motion) blurs are radius
//! independent; the anisotropic Kuwahara filter, whose cost grows with the radius squared,
//! runs on a copy downsampled by an integer factor so its radius stays at most
//! [`AKF_MAX_R`] px there, and is upsampled afterwards.
//!
//! Paper, grain, and stroke textures are procedural: a hash of the op's seed (`ctx.rng()`)
//! and the canvas position, so textures line up across layers, replay exactly, and do not
//! depend on the thread count. Transparent pixels get colors extended from their
//! neighbors before processing (alpha edges are not drawn as image edges), and the output
//! keeps the layer's alpha.

use crate::kern;
use crate::util::Buf;
use pixelox_core::kernels::Edge;
use pixelox_core::prelude::*;
use rand_core::RngCore;
use rayon::prelude::*;

mod graphic;
mod paint;
mod sketch;

pub(crate) const CAT: Category = Category::Filter;

pub fn ops() -> Vec<OpSpec> {
    let mut v = sketch::ops();
    v.extend(paint::ops());
    v.extend(graphic::ops());
    v
}

// ---------------------------------------------------------------------------------------
// scale and parameters

/// The size unit of a `w`×`h` layer in px: √(w·h)/1000, at least 0.4.
pub(crate) fn unit(w: usize, h: usize) -> f64 {
    ((w as f64 * h as f64).sqrt() / 1000.0).max(0.4)
}

/// A px size parameter: its value, or for `auto` `auto_px` rounded to 0.01 and recorded
/// for replay.
pub(crate) fn size_param(a: &Args, ctx: &mut OpCtx, name: &str, auto_px: f64) -> Result<f64> {
    if a.is_auto(name) {
        let v = (auto_px * 100.0).round().max(1.0) / 100.0;
        ctx.resolve_param(name, v.to_string());
        Ok(v)
    } else {
        a.f64(name)
    }
}

/// A 0..100 parameter as 0..1.
pub(crate) fn pct(a: &Args, name: &str) -> Result<f32> {
    Ok((a.f64(name)? / 100.0) as f32)
}

/// Integer downsampling factor that brings a radius of `r` px to at most `max` px.
pub(crate) fn work_factor(r: f64, max: f64) -> usize {
    (r / max).ceil().max(1.0) as usize
}

// ---------------------------------------------------------------------------------------
// procedural noise

/// Seeded, canvas-aligned hash noise: the same canvas pixel always gets the same value.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Tex {
    pub key: u64,
    pub ox: i64,
    pub oy: i64,
}

impl Tex {
    pub fn new(ctx: &OpCtx, ox: i32, oy: i32) -> Tex {
        Tex { key: ctx.rng().next_u64(), ox: ox as i64, oy: oy as i64 }
    }

    /// An independent stream.
    pub fn salt(self, s: u64) -> Tex {
        Tex { key: mix(self.key ^ s.wrapping_mul(0xD6E8_FEB8_6659_FD93)), ..self }
    }

    /// Uniform value in [0, 1) at layer pixel (x, y).
    #[inline]
    pub fn at(self, x: i64, y: i64) -> f32 {
        unit_f(hash2(self.key, x + self.ox, y + self.oy))
    }
}

#[inline]
pub(crate) fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

#[inline]
pub(crate) fn hash2(key: u64, x: i64, y: i64) -> u64 {
    mix(key ^ mix((x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)))
}

#[inline]
pub(crate) fn unit_f(h: u64) -> f32 {
    (h >> 40) as f32 * (1.0 / 16_777_216.0)
}

/// Uniform white noise in [0, 1).
pub(crate) fn white(w: usize, h: usize, t: Tex) -> Buf {
    let mut b = Buf::new(w, h, 1);
    b.data.par_chunks_mut(w.max(1)).enumerate().for_each(|(y, row)| {
        for (x, v) in row.iter_mut().enumerate() {
            *v = t.at(x as i64, y as i64);
        }
    });
    b
}

/// Smooth noise with features of about `scale` px, standardized to mean 0 and standard
/// deviation 1. Large scales are generated on a coarse canvas-aligned grid and upsampled.
pub(crate) fn smooth_noise(w: usize, h: usize, t: Tex, scale: f64) -> Buf {
    if w == 0 || h == 0 {
        return Buf::new(w, h, 1);
    }
    let f = (scale / 2.0).floor().max(1.0) as usize;
    let mut b = if f == 1 {
        let mut b = white(w, h, t);
        b.data.iter_mut().for_each(|v| *v -= 0.5);
        gauss_in(&mut b, scale);
        b
    } else {
        // coarse cells aligned to the canvas: cell (i, j) covers canvas px [i·f, (i+1)·f)
        let (cx0, cy0) = (t.ox.div_euclid(f as i64), t.oy.div_euclid(f as i64));
        let (cx1, cy1) = ((t.ox + w as i64 - 1).div_euclid(f as i64), (t.oy + h as i64 - 1).div_euclid(f as i64));
        let pad = 4i64;
        let (gw, gh) = ((cx1 - cx0 + 1 + 2 * pad) as usize, (cy1 - cy0 + 1 + 2 * pad) as usize);
        let mut g = Buf::new(gw, gh, 1);
        g.data.par_chunks_mut(gw).enumerate().for_each(|(j, row)| {
            for (i, v) in row.iter_mut().enumerate() {
                *v = unit_f(hash2(t.key, cx0 - pad + i as i64, cy0 - pad + j as i64)) - 0.5;
            }
        });
        gauss_in(&mut g, scale / f as f64);
        let mut b = Buf::new(w, h, 1);
        let ff = f as f32;
        b.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            let gy = ((y as i64 + t.oy) as f32 + 0.5) / ff - 0.5 - (cy0 - pad) as f32;
            for (x, v) in row.iter_mut().enumerate() {
                let gx = ((x as i64 + t.ox) as f32 + 0.5) / ff - 0.5 - (cx0 - pad) as f32;
                let mut o = [0f32];
                crate::util::sample(&g, gx, gy, Edge::Clamp, &mut o);
                *v = o[0];
            }
        });
        b
    };
    standardize(&mut b);
    b
}

/// Shift and scale a plane to mean 0 and standard deviation 1 (no change when flat).
pub(crate) fn standardize(b: &mut Buf) {
    let (m, sd) = mean_std(&b.data, b.w.max(1));
    let inv = if sd > 1e-12 { 1.0 / sd } else { 0.0 };
    b.data.par_iter_mut().for_each(|v| *v = ((*v as f64 - m) * inv) as f32);
}

/// Mean and standard deviation, summed per row in parallel and across rows in order (the
/// result does not depend on the thread count).
pub(crate) fn mean_std(data: &[f32], row: usize) -> (f64, f64) {
    if data.is_empty() {
        return (0.0, 0.0);
    }
    let parts: Vec<(f64, f64)> = data
        .par_chunks(row)
        .map(|r| r.iter().fold((0.0, 0.0), |(s, q), &v| (s + v as f64, q + v as f64 * v as f64)))
        .collect();
    let (mut s, mut q) = (0.0, 0.0);
    for (a, b) in parts {
        s += a;
        q += b;
    }
    let n = data.len() as f64;
    let m = s / n;
    (m, (q / n - m * m).max(0.0).sqrt())
}

/// A field of hatching strokes along `angle` (degrees counter-clockwise from the right):
/// sparse seeded dabs of width `width` px drawn into tapered lines `len` px long, with
/// `coverage` strokes over an average pixel. Normalized to mean 1 (0 = bare paper).
pub(crate) fn stroke_field(w: usize, h: usize, t: Tex, angle: f64, len: f64, width: f64, coverage: f64) -> Buf {
    let mut b = Buf::new(w, h, 1);
    if w == 0 || h == 0 {
        return b;
    }
    let density = (coverage / (len.max(1.0) * width.max(0.5))).min(1.0) as f32;
    let (t1, t2) = (t.salt(1), t.salt(2));
    b.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, v) in row.iter_mut().enumerate() {
            let (xi, yi) = (x as i64, y as i64);
            if t1.at(xi, yi) < density {
                *v = 0.35 + 0.65 * t2.at(xi, yi);
            }
        }
    });
    gauss_in(&mut b, (width * 0.45).max(0.35));
    // two half-length boxes: a triangular profile, so strokes taper at both ends
    let b = kern::motion_blur(&b, angle, len * 0.5, Edge::Clamp);
    let mut b = kern::motion_blur(&b, angle, len * 0.5, Edge::Clamp);
    let (m, _) = mean_std(&b.data, w);
    let inv = if m > 1e-12 { (1.0 / m) as f32 } else { 0.0 };
    b.data.par_iter_mut().for_each(|v| *v *= inv);
    b
}

// ---------------------------------------------------------------------------------------
// planes and color

/// Gaussian blur of a buffer in place (edges clamp).
pub(crate) fn gauss_in(b: &mut Buf, sigma: f64) {
    if sigma > 0.0 && b.w > 0 && b.h > 0 {
        kern::gaussian(b, sigma, sigma, Edge::Clamp);
    }
}

pub(crate) fn gauss(b: &Buf, sigma: f64) -> Buf {
    let mut o = b.clone();
    gauss_in(&mut o, sigma);
    o
}

/// Straight RGB of `img` as a three-channel buffer, with the colors under transparent
/// pixels filled from their surroundings (so alpha edges are not image edges).
pub(crate) fn solid_rgb(img: &Image) -> Buf {
    let (w, h) = (img.width as usize, img.height as usize);
    let mut out = Buf::new(w, h, 3);
    out.data.par_chunks_mut(3).zip(img.data.par_chunks(4)).for_each(|(o, p)| o.copy_from_slice(&p[..3]));
    if img.is_empty() || img.is_opaque() {
        return out;
    }
    let pm = Buf::premul(img);
    // extension color per pixel, from the smallest blur that reaches it
    let mut ext = vec![f32::NAN; w * h * 3];
    let u = unit(w, h);
    for sigma in [2.0 * u, 12.0 * u, 60.0 * u] {
        let b = gauss(&pm, sigma);
        ext.par_chunks_mut(3).zip(b.data.par_chunks(4)).for_each(|(e, p)| {
            if e[0].is_nan() && p[3] > 0.02 {
                for c in 0..3 {
                    e[c] = (p[c] / p[3]).clamp(0.0, 1.0);
                }
            }
        });
    }
    // anything still unreached: the mean color (gray for a fully transparent layer)
    let (mut s, mut a) = ([0f64; 3], 0f64);
    for p in pm.data.chunks(4) {
        for c in 0..3 {
            s[c] += p[c] as f64;
        }
        a += p[3] as f64;
    }
    let fill = if a > 1e-9 { [(s[0] / a) as f32, (s[1] / a) as f32, (s[2] / a) as f32] } else { [0.5; 3] };
    out.data.par_chunks_mut(3).zip(ext.par_chunks(3)).zip(img.data.par_chunks(4)).for_each(|((o, e), p)| {
        let al = p[3].clamp(0.0, 1.0);
        if al >= 0.999 {
            return;
        }
        for c in 0..3 {
            let x = if e[c].is_nan() { fill[c] } else { e[c] };
            o[c] = p[c] * al + x * (1.0 - al);
        }
    });
    out
}

/// Rec. 709 luma of a three-channel buffer.
pub(crate) fn luma(rgb: &Buf) -> Buf {
    let data = rgb.data.par_chunks(3).map(|p| color::luma(p[0], p[1], p[2])).collect();
    Buf { w: rgb.w, h: rgb.h, c: 1, data }
}

/// An RGB result as an image with the alpha of `orig`, clamped to 0..1 (fully
/// transparent pixels stay 0, as everywhere in pixelox).
pub(crate) fn with_alpha(orig: &Image, rgb: &Buf) -> Image {
    let mut out = orig.clone();
    out.data.par_chunks_mut(4).zip(rgb.data.par_chunks(3)).for_each(|(o, p)| {
        if o[3] > 0.0 {
            for c in 0..3 {
                o[c] = p[c].clamp(0.0, 1.0);
            }
        } else {
            o.fill(0.0);
        }
    });
    out
}

/// Value at quantile `q` (0..1) of a plane, from a 4096-bin histogram over 0..1 (exact
/// integer counts, so independent of the thread count).
pub(crate) fn quantile(p: &Buf, q: f64) -> f32 {
    let hist = histogram(&p.data, 4096);
    let n: u64 = hist.iter().sum();
    if n == 0 {
        return 0.5;
    }
    let target = (q.clamp(0.0, 1.0) * n as f64) as u64;
    let mut acc = 0u64;
    for (i, &c) in hist.iter().enumerate() {
        acc += c;
        if acc > target {
            return (i as f32 + 0.5) / 4096.0;
        }
    }
    1.0
}

/// The tone range to stretch a luma plane over: its `q` and `1 − q` quantiles, widened to
/// at least 0.2..0.8 so flat or low-contrast layers are not blown up.
pub(crate) fn tone_range(p: &Buf, q: f64) -> (f32, f32) {
    (quantile(p, q).min(0.2), quantile(p, 1.0 - q).max(0.8))
}

pub(crate) fn histogram(data: &[f32], bins: usize) -> Vec<u64> {
    data.par_chunks(65_536)
        .map(|ch| {
            let mut h = vec![0u64; bins];
            for &v in ch {
                h[((v.clamp(0.0, 1.0) * bins as f32) as usize).min(bins - 1)] += 1;
            }
            h
        })
        .reduce(
            || vec![0u64; bins],
            |mut a, b| {
                a.iter_mut().zip(b).for_each(|(x, y)| *x += y);
                a
            },
        )
}

/// Central-difference gradient of a plane (edges clamp).
pub(crate) fn gradient(p: &Buf) -> (Buf, Buf) {
    let (w, h) = (p.w, p.h);
    let mut gx = Buf::new(w, h, 1);
    let mut gy = Buf::new(w, h, 1);
    gx.data.par_chunks_mut(w.max(1)).zip(gy.data.par_chunks_mut(w.max(1))).enumerate().for_each(|(y, (rx, ry))| {
        let up = y.saturating_sub(1);
        let dn = (y + 1).min(h - 1);
        for x in 0..w {
            let l = x.saturating_sub(1);
            let r = (x + 1).min(w - 1);
            rx[x] = (p.data[y * w + r] - p.data[y * w + l]) * 0.5;
            ry[x] = (p.data[dn * w + x] - p.data[up * w + x]) * 0.5;
        }
    });
    (gx, gy)
}

// ---------------------------------------------------------------------------------------
// structure tensor and the anisotropic Kuwahara filter

/// Local orientation: the edge tangent (unit vector, image coordinates with y down) and
/// the anisotropy 0 (isotropic) .. 1 (a clean straight edge) per pixel.
pub(crate) struct Flow {
    pub tx: Vec<f32>,
    pub ty: Vec<f32>,
    pub aniso: Vec<f32>,
}

/// Smoothed structure tensor of an RGB buffer (Sobel derivatives, Gaussian `sigma`).
pub(crate) fn flow(rgb: &Buf, sigma: f64) -> Flow {
    let (w, h, c) = (rgb.w, rgb.h, rgb.c);
    let mut st = Buf::new(w, h, 3);
    let at = |x: i64, y: i64, ch: usize| {
        rgb.data[(y.clamp(0, h as i64 - 1) as usize * w + x.clamp(0, w as i64 - 1) as usize) * c + ch]
    };
    st.data.par_chunks_mut((w * 3).max(1)).enumerate().for_each(|(y, row)| {
        let y = y as i64;
        for x in 0..w {
            let xi = x as i64;
            let (mut e, mut f, mut g) = (0f32, 0f32, 0f32);
            for ch in 0..c.min(3) {
                let gx = (at(xi + 1, y - 1, ch) + 2.0 * at(xi + 1, y, ch) + at(xi + 1, y + 1, ch)
                    - at(xi - 1, y - 1, ch)
                    - 2.0 * at(xi - 1, y, ch)
                    - at(xi - 1, y + 1, ch))
                    / 4.0;
                let gy = (at(xi - 1, y + 1, ch) + 2.0 * at(xi, y + 1, ch) + at(xi + 1, y + 1, ch)
                    - at(xi - 1, y - 1, ch)
                    - 2.0 * at(xi, y - 1, ch)
                    - at(xi + 1, y - 1, ch))
                    / 4.0;
                e += gx * gx;
                f += gx * gy;
                g += gy * gy;
            }
            row[x * 3] = e;
            row[x * 3 + 1] = f;
            row[x * 3 + 2] = g;
        }
    });
    gauss_in(&mut st, sigma);
    let n = w * h;
    let mut tx = vec![0f32; n];
    let mut ty = vec![1f32; n];
    let mut aniso = vec![0f32; n];
    tx.par_iter_mut().zip(ty.par_iter_mut()).zip(aniso.par_iter_mut()).zip(st.data.par_chunks(3)).for_each(
        |(((tx, ty), an), s)| {
            let (e, f, g) = (s[0], s[1], s[2]);
            let d = ((e - g) * (e - g) + 4.0 * f * f).sqrt();
            let l1 = (e + g + d) * 0.5;
            let l2 = (e + g - d) * 0.5;
            let (vx, vy) = (l1 - e, -f);
            let len = (vx * vx + vy * vy).sqrt();
            if len > 1e-12 {
                *tx = vx / len;
                *ty = vy / len;
            }
            *an = if l1 + l2 > 1e-12 { ((l1 - l2) / (l1 + l2)).clamp(0.0, 1.0) } else { 0.0 };
        },
    );
    Flow { tx, ty, aniso }
}

/// Largest Kuwahara radius computed directly; larger brushes run downsampled.
pub(crate) const AKF_MAX_R: f64 = 6.0;

/// Anisotropic Kuwahara filter (Kyprianidis et al. 2009, polynomial sector weights of
/// 2010) of a three-channel buffer: each pixel becomes the variance-weighted mean of eight
/// overlapping sectors of an ellipse of radius `radius` stretched along the edge tangent.
/// `q` sets how strongly the lowest-variance sectors win (sharper edges). O(radius²) per
/// pixel: callers keep `radius` ≤ [`AKF_MAX_R`].
pub(crate) fn akf(src: &Buf, fl: &Flow, radius: f32, q: f32) -> Buf {
    let (w, h) = (src.w, src.h);
    let mut out = Buf::new(w, h, 3);
    if w == 0 || h == 0 {
        return out;
    }
    if radius < 0.5 {
        return src.clone();
    }
    const NG: usize = 512;
    let glut: Vec<f32> = (0..=NG).map(|i| (-3.125 * (i as f32 / NG as f32 * 0.25)).exp()).collect();
    let (zeta, eta) = (0.12f32, 3.0f32);
    let s2 = std::f32::consts::FRAC_1_SQRT_2;
    out.data.par_chunks_mut(w * 3).enumerate().for_each(|(y, row)| {
        let mut m = [[0f32; 4]; 8];
        let mut sq = [[0f32; 3]; 8];
        for x in 0..w {
            let i = y * w + x;
            let (c, s) = (fl.tx[i], fl.ty[i]);
            let an = fl.aniso[i];
            let a = radius * (1.0 + an).clamp(0.1, 2.0);
            let b = radius * (1.0 / (1.0 + an)).clamp(0.1, 2.0);
            // v = S·R(−φ)·offset maps the ellipse onto the disc of radius 1/2
            let (r00, r01, r10, r11) = (0.5 / a * c, 0.5 / a * s, -0.5 / b * s, 0.5 / b * c);
            let max_x = (a * a * c * c + b * b * s * s).sqrt().floor() as i64;
            let max_y = (a * a * s * s + b * b * c * c).sqrt().floor() as i64;
            for k in 0..8 {
                m[k] = [0.0; 4];
                sq[k] = [0.0; 3];
            }
            for j in -max_y..=max_y {
                let yy = (y as i64 + j).clamp(0, h as i64 - 1) as usize;
                let base = yy * w;
                let jf = j as f32;
                for ii in -max_x..=max_x {
                    let fi = ii as f32;
                    let vx = r00 * fi + r01 * jf;
                    let vy = r10 * fi + r11 * jf;
                    let d2 = vx * vx + vy * vy;
                    if d2 > 0.25 {
                        continue;
                    }
                    let xx = (x as i64 + ii).clamp(0, w as i64 - 1) as usize;
                    let p = &src.data[(base + xx) * 3..(base + xx) * 3 + 3];
                    let mut wt = [0f32; 8];
                    let vxx = zeta - eta * vx * vx;
                    let vyy = zeta - eta * vy * vy;
                    let z0 = (vy + vxx).max(0.0);
                    let z2 = (-vx + vyy).max(0.0);
                    let z4 = (-vy + vxx).max(0.0);
                    let z6 = (vx + vyy).max(0.0);
                    wt[0] = z0 * z0;
                    wt[2] = z2 * z2;
                    wt[4] = z4 * z4;
                    wt[6] = z6 * z6;
                    let (ux, uy) = (s2 * (vx - vy), s2 * (vx + vy));
                    let uxx = zeta - eta * ux * ux;
                    let uyy = zeta - eta * uy * uy;
                    let z1 = (uy + uxx).max(0.0);
                    let z3 = (-ux + uyy).max(0.0);
                    let z5 = (-uy + uxx).max(0.0);
                    let z7 = (ux + uyy).max(0.0);
                    wt[1] = z1 * z1;
                    wt[3] = z3 * z3;
                    wt[5] = z5 * z5;
                    wt[7] = z7 * z7;
                    let sum: f32 = wt.iter().sum();
                    if sum <= 1e-12 {
                        continue;
                    }
                    let g = glut[((d2 * 4.0 * NG as f32) as usize).min(NG)] / sum;
                    let (pr, pg, pb) = (p[0], p[1], p[2]);
                    let (qr, qg, qb) = (pr * pr, pg * pg, pb * pb);
                    for k in 0..8 {
                        let wk = wt[k] * g;
                        let mk = &mut m[k];
                        mk[0] += pr * wk;
                        mk[1] += pg * wk;
                        mk[2] += pb * wk;
                        mk[3] += wk;
                        let sk = &mut sq[k];
                        sk[0] += qr * wk;
                        sk[1] += qg * wk;
                        sk[2] += qb * wk;
                    }
                }
            }
            let mut o = [0f32; 4];
            for k in 0..8 {
                let wk = m[k][3];
                if wk <= 1e-12 {
                    continue;
                }
                let inv = 1.0 / wk;
                let mut sig = 0f32;
                let mut mean = [0f32; 3];
                for ch in 0..3 {
                    mean[ch] = m[k][ch] * inv;
                    sig += (sq[k][ch] * inv - mean[ch] * mean[ch]).abs().sqrt();
                }
                let a = 1.0 / (1.0 + (255.0 * sig).powf(0.5 * q));
                for ch in 0..3 {
                    o[ch] += mean[ch] * a;
                }
                o[3] += a;
            }
            let px = &mut row[x * 3..x * 3 + 3];
            if o[3] > 0.0 {
                for ch in 0..3 {
                    px[ch] = o[ch] / o[3];
                }
            } else {
                px.copy_from_slice(&src.data[i * 3..i * 3 + 3]);
            }
        }
    });
    out
}

/// Painterly abstraction of `rgb` at brush radius `radius` px: the anisotropic Kuwahara
/// filter, run at a reduced resolution when the radius is large. Returns the result at
/// the working resolution with its downsampling factor.
pub(crate) fn abstraction(rgb: &Buf, radius: f64, q: f32, passes: usize) -> (Buf, usize) {
    let f = work_factor(radius, AKF_MAX_R);
    let mut cur = if f > 1 { crate::util::downsample(rgb, f) } else { rgb.clone() };
    let r = (radius / f as f64) as f32;
    for _ in 0..passes.max(1) {
        let fl = flow(&cur, 2.0);
        cur = akf(&cur, &fl, r, q);
    }
    (cur, f)
}

// ---------------------------------------------------------------------------------------
// OKLab helpers

pub(crate) fn to_oklab(rgb: &Buf) -> Buf {
    let mut out = rgb.clone();
    out.data.par_chunks_mut(3).for_each(|p| {
        let (l, a, b) = color::rgb_to_oklab(p[0].clamp(0.0, 1.0), p[1].clamp(0.0, 1.0), p[2].clamp(0.0, 1.0));
        p[0] = l;
        p[1] = a;
        p[2] = b;
    });
    out
}

pub(crate) fn from_oklab(lab: &Buf) -> Buf {
    let mut out = lab.clone();
    out.data.par_chunks_mut(3).for_each(|p| {
        let (r, g, b) = color::oklab_to_rgb(p[0], p[1], p[2]);
        p[0] = r.clamp(0.0, 1.0);
        p[1] = g.clamp(0.0, 1.0);
        p[2] = b.clamp(0.0, 1.0);
    });
    out
}

// ---------------------------------------------------------------------------------------
// test fixtures

#[cfg(test)]
pub(crate) mod fixture {
    use super::*;
    use pixelox_core::testing;

    /// A small synthetic "photo": a light sky-to-gray gradient, a mid-gray square, a dark
    /// disc, a red bar, and a little deterministic texture.
    pub fn scene(w: u32, h: u32) -> Image {
        Image::from_fn(w, h, |x, y| {
            let (fx, fy) = (x as f32 / w as f32, y as f32 / h as f32);
            let n = ((x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503)) % 97) as f32 / 97.0 * 0.04 - 0.02;
            let mut c = [0.85 - 0.15 * fy, 0.88 - 0.15 * fy, 0.92 - 0.1 * fy];
            if (0.15..0.45).contains(&fx) && (0.2..0.7).contains(&fy) {
                c = [0.5, 0.5, 0.5];
            }
            let (dx, dy) = (fx - 0.7, fy - 0.45);
            if dx * dx + dy * dy < 0.03 {
                c = [0.15, 0.13, 0.12];
            }
            if (0.75..0.85).contains(&fy) && fx > 0.1 && fx < 0.9 {
                c = [0.8, 0.15, 0.1];
            }
            [(c[0] + n).clamp(0.0, 1.0), (c[1] + n).clamp(0.0, 1.0), (c[2] + n).clamp(0.0, 1.0), 1.0]
        })
    }

    /// Run `name` on a copy of `img` and return the result and the op's JSON.
    pub fn run(name: &str, params: &[(&str, &str)], img: &Image) -> (Image, Value) {
        let spec = testing::find(&crate::ops(), name);
        let mut doc = testing::doc_from(img.clone());
        let v = testing::run(&spec, params, &mut doc).unwrap_or_else(|e| panic!("{name}: {}", e.message));
        (testing::raster(&doc, "Background").clone(), v)
    }

    pub fn lum(img: &Image) -> Vec<f32> {
        img.data.chunks(4).map(|p| color::luma(p[0], p[1], p[2])).collect()
    }

    /// Mean luma over a rectangle.
    pub fn mean_rect(img: &Image, x0: u32, y0: u32, x1: u32, y1: u32) -> f32 {
        let mut s = 0.0;
        for y in y0..y1 {
            for x in x0..x1 {
                let p = img.get(x, y);
                s += color::luma(p[0], p[1], p[2]);
            }
        }
        s / ((x1 - x0) * (y1 - y0)) as f32
    }

    /// Mean absolute difference between horizontally (or vertically) adjacent pixels'
    /// luma inside a rectangle: a measure of fine texture.
    pub fn roughness(img: &Image, x0: u32, y0: u32, x1: u32, y1: u32) -> f32 {
        let mut s = 0.0;
        let mut n = 0;
        for y in y0..y1 - 1 {
            for x in x0..x1 - 1 {
                let p = img.get(x, y);
                let (q, r) = (img.get(x + 1, y), img.get(x, y + 1));
                let l = color::luma(p[0], p[1], p[2]);
                s += (l - color::luma(q[0], q[1], q[2])).abs() + (l - color::luma(r[0], r[1], r[2])).abs();
                n += 2;
            }
        }
        s / n as f32
    }
}

#[cfg(test)]
mod tests {
    use super::fixture::*;
    use super::*;
    use pixelox_core::testing;

    const ALL: &[&str] = &[
        "filter pencil",
        "filter charcoal",
        "filter ink",
        "filter oil-paint",
        "filter watercolor",
        "filter cartoon",
        "filter cutout",
        "filter halftone",
    ];

    #[test]
    fn alpha_is_kept_and_transparent_edges_draw_nothing() {
        // opaque scene on the right, fully transparent on the left
        let src = Image::from_fn(64, 48, |x, y| {
            let mut p = scene(64, 48).get(x, y);
            if x < 24 {
                p = [0.0, 0.0, 0.0, 0.0];
            } else if x < 28 {
                p[3] = 0.5;
            }
            p
        });
        for name in ALL {
            let (out, _) = run(name, &[], &src);
            for (o, s) in out.data.chunks(4).zip(src.data.chunks(4)) {
                assert_eq!(o[3], s[3], "{name}: alpha changed");
                if s[3] == 0.0 {
                    assert_eq!(&o[..3], &[0.0, 0.0, 0.0], "{name}: color under transparency");
                }
            }
        }
    }

    #[test]
    fn colors_extend_under_transparency() {
        let src = Image::from_fn(20, 10, |x, _| if x < 10 { [0.0; 4] } else { [0.2, 0.6, 0.9, 1.0] });
        let b = solid_rgb(&src);
        assert!((b.data[0] - 0.2).abs() < 0.02 && (b.data[2] - 0.9).abs() < 0.02, "{:?}", &b.data[..3]);
        let none = solid_rgb(&Image::new(3, 3));
        assert!(none.data.iter().all(|&v| v == 0.5));
    }

    #[test]
    fn seeded_and_repeatable() {
        let src = scene(80, 60);
        for name in ALL {
            let (a, _) = run(name, &[], &src);
            let (b, _) = run(name, &[], &src);
            assert_eq!(a.data, b.data, "{name}: not repeatable");
        }
        // a different seed changes the procedural textures of the drawing media
        let spec = testing::find(&crate::ops(), "filter pencil");
        let a = pixelox_core::testing::args(&spec, &[]).unwrap();
        let mut doc1 = testing::doc_from(src.clone());
        let mut doc2 = testing::doc_from(src.clone());
        let g2 = pixelox_core::ctx::Globals { seed: 7, ..Default::default() };
        let mut c1 =
            OpCtx::new(spec.name, vec![], Default::default(), &pixelox_core::ctx::NoIo, &pixelox_core::ctx::NoProgress);
        let mut c2 = OpCtx::new(spec.name, vec![], g2, &pixelox_core::ctx::NoIo, &pixelox_core::ctx::NoProgress);
        if let pixelox_core::Runner::Doc(f) = spec.runner {
            f(&a, &mut doc1, &mut c1).unwrap();
            f(&a, &mut doc2, &mut c2).unwrap();
        }
        assert_ne!(testing::raster(&doc1, "Background").data, testing::raster(&doc2, "Background").data);
    }

    #[test]
    fn auto_sizes_scale_with_the_layer_and_are_recorded() {
        let spec = testing::find(&crate::ops(), "filter pencil");
        let mut doc = testing::doc_from(scene(40, 30));
        let v = testing::run(&spec, &[], &mut doc).unwrap();
        assert_eq!(v["stroke"], json!(0.44), "unit is floored at 0.4 px");
        assert!((unit(1000, 1000) - 1.0).abs() < 1e-12 && (unit(6000, 4000) - 4.899).abs() < 1e-3);
        let mut doc = testing::doc_from(scene(40, 30));
        let v = testing::run(&spec, &[("stroke", "3")], &mut doc).unwrap();
        assert_eq!(v["stroke"], json!(3.0));
    }

    /// Huge sizes must not blow up the cost: windowed kernels run downsampled, and blurs
    /// are radius independent.
    #[test]
    fn large_sizes_stay_cheap() {
        let src = scene(120, 90);
        let cases: &[(&str, &[(&str, &str)])] = &[
            ("filter pencil", &[("stroke", "150")]),
            ("filter charcoal", &[("stroke", "300")]),
            ("filter ink", &[("stroke", "150"), ("shading", "stipple")]),
            ("filter oil-paint", &[("brush", "400")]),
            ("filter watercolor", &[("brush", "400")]),
            ("filter cartoon", &[("smooth", "400"), ("stroke", "150")]),
            ("filter cutout", &[("simplicity", "100"), ("fidelity", "0")]),
            ("filter halftone", &[("cell", "900"), ("mode", "cmyk")]),
        ];
        for (name, params) in cases {
            let t = std::time::Instant::now();
            let (out, _) = run(name, params, &src);
            assert!(out.data.iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)), "{name}");
            assert!(t.elapsed().as_secs_f64() < 10.0, "{name}: {:?}", t.elapsed());
        }
    }

    #[test]
    fn akf_flattens_texture_and_keeps_a_step() {
        let (w, h) = (40usize, 20usize);
        let mut b = Buf::new(w, h, 3);
        for y in 0..h {
            for x in 0..w {
                let n = (((x * 7 + y * 13) % 5) as f32 - 2.0) * 0.03;
                let v = if x < 20 { 0.2 } else { 0.8 } + n;
                b.data[(y * w + x) * 3..(y * w + x) * 3 + 3].copy_from_slice(&[v, v, v]);
            }
        }
        let fl = flow(&b, 2.0);
        let o = akf(&b, &fl, 4.0, 8.0);
        let at = |x: usize, y: usize| o.data[(y * w + x) * 3];
        assert!((at(8, 10) - 0.2).abs() < 0.03 && (at(32, 10) - 0.8).abs() < 0.03);
        // the step stays sharp: one pixel in from the edge is still near its side's value
        assert!(at(18, 10) < 0.35 && at(21, 10) > 0.65, "{} {}", at(18, 10), at(21, 10));
    }

    #[test]
    fn noise_is_canvas_aligned() {
        let t = Tex { key: 42, ox: 0, oy: 0 };
        let shifted = Tex { key: 42, ox: 5, oy: 3 };
        assert_eq!(t.at(10, 10), shifted.at(5, 7));
        let n = smooth_noise(64, 32, t, 6.0);
        let (m, sd) = mean_std(&n.data, 64);
        assert!(m.abs() < 1e-6 && (sd - 1.0).abs() < 1e-4);
    }
}
