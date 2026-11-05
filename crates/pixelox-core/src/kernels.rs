//! Shared pixel kernels used by the compositor and by every op crate: Gaussian and box
//! blur (images and masks), separable resampling (nearest, bilinear, bicubic, Lanczos-3),
//! grayscale morphology, median smoothing, 2-D convolution, and SSIM.
//!
//! Complexity (PRD §7.3): blurs are separable, switching to three box passes (O(1) per
//! pixel) above σ = 4; resampling is separable two-pass; morphology is O(pixels·radius).

use crate::image::{Image, Mask};
use rayon::prelude::*;

// ---------------------------------------------------------------------------------------
// planar helpers

/// How pixels beyond the edge are treated by blurs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    /// Repeat the edge pixel (right for opaque photos: no dark borders).
    Clamp,
    /// Treat outside as transparent / zero (right for layers with transparency).
    Zero,
}

fn transpose(src: &[f32], w: usize, h: usize, c: usize) -> Vec<f32> {
    let mut out = vec![0.0f32; src.len()];
    const B: usize = 32;
    // blocked transpose, parallel over destination row blocks
    out.par_chunks_mut(h * c * B).enumerate().for_each(|(bx, chunk)| {
        let x0 = bx * B;
        let x1 = (x0 + B).min(w);
        for y0 in (0..h).step_by(B) {
            let y1 = (y0 + B).min(h);
            for x in x0..x1 {
                for y in y0..y1 {
                    let s = (y * w + x) * c;
                    let d = ((x - x0) * h + y) * c;
                    chunk[d..d + c].copy_from_slice(&src[s..s + c]);
                }
            }
        }
    });
    out
}

fn gaussian_kernel(sigma: f64) -> Vec<f32> {
    let r = (sigma * 3.0).ceil().max(1.0) as i64;
    let mut k: Vec<f64> = (-r..=r).map(|i| (-(i * i) as f64 / (2.0 * sigma * sigma)).exp()).collect();
    let s: f64 = k.iter().sum();
    k.iter_mut().for_each(|v| *v /= s);
    k.into_iter().map(|v| v as f32).collect()
}

/// Copy a row of pixels (`c` channels) into `out` with `r` pixels of padding per side.
fn pad_row(row: &[f32], c: usize, r: usize, edge: Edge, out: &mut Vec<f32>) {
    let w = row.len() / c;
    out.clear();
    out.reserve((w + 2 * r) * c);
    for _ in 0..r {
        match edge {
            Edge::Clamp => out.extend_from_slice(&row[..c]),
            Edge::Zero => out.extend(std::iter::repeat_n(0.0, c)),
        }
    }
    out.extend_from_slice(row);
    for _ in 0..r {
        match edge {
            Edge::Clamp => out.extend_from_slice(&row[(w - 1) * c..]),
            Edge::Zero => out.extend(std::iter::repeat_n(0.0, c)),
        }
    }
}

/// Convolve every row with a symmetric odd kernel: branch-free inner loops over
/// contiguous slices (they auto-vectorize).
fn conv_rows(data: &mut [f32], w: usize, c: usize, k: &[f32], edge: Edge) {
    if w == 0 || k.len() <= 1 {
        return;
    }
    let r = k.len() / 2;
    let n = w * c;
    data.par_chunks_mut(n).for_each_init(Vec::new, |pad, row| {
        pad_row(row, c, r, edge, pad);
        row.fill(0.0);
        for (j, &kj) in k.iter().enumerate() {
            for (o, &s) in row.iter_mut().zip(&pad[j * c..j * c + n]) {
                *o += kj * s;
            }
        }
    });
}

/// Running-sum box filter of radius `r` on every row.
fn box_rows(data: &mut [f32], w: usize, c: usize, r: usize, edge: Edge) {
    let norm = 1.0 / (2 * r + 1) as f32;
    data.par_chunks_mut(w * c).for_each(|row| {
        let src = row.to_vec();
        let at = |i: i64, ch: usize| -> f32 {
            if i < 0 || i >= w as i64 {
                match edge {
                    Edge::Clamp => src[(i.clamp(0, w as i64 - 1) as usize) * c + ch],
                    Edge::Zero => 0.0,
                }
            } else {
                src[i as usize * c + ch]
            }
        };
        for ch in 0..c {
            let mut acc: f64 = 0.0;
            for i in -(r as i64)..=(r as i64) {
                acc += at(i, ch) as f64;
            }
            for x in 0..w {
                row[x * c + ch] = acc as f32 * norm;
                acc += at(x as i64 + r as i64 + 1, ch) as f64 - at(x as i64 - r as i64, ch) as f64;
            }
        }
    });
}

/// Three box radii whose successive application approximates a Gaussian of `sigma`.
fn boxes_for_gauss(sigma: f64) -> [usize; 3] {
    let n = 3.0;
    let w_ideal = (12.0 * sigma * sigma / n + 1.0).sqrt();
    let mut wl = w_ideal.floor() as i64;
    if wl % 2 == 0 {
        wl -= 1;
    }
    let wu = wl + 2;
    let m_ideal =
        (12.0 * sigma * sigma - n * (wl * wl) as f64 - 4.0 * n * wl as f64 - 3.0 * n) / (-4.0 * wl as f64 - 4.0);
    let m = m_ideal.round() as i64;
    let mut out = [0usize; 3];
    for (i, o) in out.iter_mut().enumerate() {
        let size = if (i as i64) < m { wl } else { wu };
        *o = ((size - 1) / 2).max(0) as usize;
    }
    out
}

/// Gaussian blur of planar-interleaved data with `c` channels.
pub fn gaussian_planar(data: &mut Vec<f32>, w: usize, h: usize, c: usize, sigma_x: f64, sigma_y: f64, edge: Edge) {
    if w == 0 || h == 0 {
        return;
    }
    let pass = |d: &mut Vec<f32>, len: usize, sigma: f64| {
        if sigma <= 0.0 {
            return;
        }
        // direct taps up to sigma 4 (at most 25), three running boxes above: O(1) per pixel
        if sigma > 4.0 {
            for r in boxes_for_gauss(sigma) {
                box_rows(d, len, c, r, edge);
            }
        } else {
            conv_rows(d, len, c, &gaussian_kernel(sigma), edge);
        }
    };
    pass(data, w, sigma_x);
    if sigma_y > 0.0 {
        let mut t = transpose(data, w, h, c);
        pass(&mut t, h, sigma_y);
        *data = transpose(&t, h, w, c);
    }
}

/// Gaussian blur of an image (premultiplied internally, so transparent pixels do not
/// bleed black). `Edge::Clamp` suits opaque photos; `Edge::Zero` lets layers fade out.
pub fn gaussian_blur(img: &Image, sigma: f64, edge: Edge) -> Image {
    gaussian_blur_xy(img, sigma, sigma, edge)
}

pub fn gaussian_blur_xy(img: &Image, sx: f64, sy: f64, edge: Edge) -> Image {
    let mut out = img.clone();
    if sx <= 0.0 && sy <= 0.0 {
        return out;
    }
    out.premultiply();
    gaussian_planar(&mut out.data, img.width as usize, img.height as usize, 4, sx, sy, edge);
    out.unpremultiply();
    out
}

/// Box blur of an image with `passes` repetitions.
pub fn box_blur(img: &Image, radius: usize, passes: usize, edge: Edge) -> Image {
    let mut out = img.clone();
    if radius == 0 {
        return out;
    }
    out.premultiply();
    let (w, h) = (img.width as usize, img.height as usize);
    for _ in 0..passes.max(1) {
        box_rows(&mut out.data, w, 4, radius, edge);
        let mut t = transpose(&out.data, w, h, 4);
        box_rows(&mut t, h, 4, radius, edge);
        out.data = transpose(&t, h, w, 4);
    }
    out.unpremultiply();
    out
}

/// Gaussian blur of a mask (edges clamp).
pub fn blur_mask(m: &Mask, sigma: f64) -> Mask {
    let mut out = m.clone();
    gaussian_planar(&mut out.data, m.width as usize, m.height as usize, 1, sigma, sigma, Edge::Clamp);
    out
}

/// Pick the edge mode a blur should use for this image.
pub fn auto_edge(img: &Image) -> Edge {
    if img.is_opaque() {
        Edge::Clamp
    } else {
        Edge::Zero
    }
}

// ---------------------------------------------------------------------------------------
// resampling

/// Resampling filter (`--interp nearest|bilinear|bicubic|lanczos3`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    Nearest,
    Bilinear,
    Bicubic,
    Lanczos3,
}

pub const FILTERS: &[&str] = &["nearest", "bilinear", "bicubic", "lanczos3"];

impl Filter {
    pub fn parse(s: &str) -> crate::Result<Filter> {
        Ok(match s {
            "nearest" => Filter::Nearest,
            "bilinear" | "linear" | "triangle" => Filter::Bilinear,
            "bicubic" | "cubic" => Filter::Bicubic,
            "lanczos3" | "lanczos" => Filter::Lanczos3,
            _ => return Err(crate::Error::usage(format!("--interp must be one of {}, got '{s}'", FILTERS.join(", ")))),
        })
    }

    /// PRD default: Lanczos-3 when shrinking, bicubic when enlarging.
    pub fn auto(src: u32, dst: u32) -> Filter {
        if dst < src {
            Filter::Lanczos3
        } else {
            Filter::Bicubic
        }
    }

    fn support(self) -> f64 {
        match self {
            Filter::Nearest => 0.5,
            Filter::Bilinear => 1.0,
            Filter::Bicubic => 2.0,
            Filter::Lanczos3 => 3.0,
        }
    }

    fn eval(self, x: f64) -> f64 {
        let x = x.abs();
        match self {
            Filter::Nearest => {
                if x <= 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
            Filter::Bilinear => (1.0 - x).max(0.0),
            Filter::Bicubic => {
                // Keys cubic, a = -0.5 (Catmull-Rom)
                let a = -0.5;
                if x < 1.0 {
                    (a + 2.0) * x * x * x - (a + 3.0) * x * x + 1.0
                } else if x < 2.0 {
                    a * x * x * x - 5.0 * a * x * x + 8.0 * a * x - 4.0 * a
                } else {
                    0.0
                }
            }
            Filter::Lanczos3 => {
                if x < 1e-9 {
                    1.0
                } else if x < 3.0 {
                    let px = std::f64::consts::PI * x;
                    3.0 * px.sin() * (px / 3.0).sin() / (px * px)
                } else {
                    0.0
                }
            }
        }
    }
}

struct Contrib {
    start: usize,
    weights: Vec<f32>,
}

fn contributions(src: usize, dst: usize, filter: Filter) -> Vec<Contrib> {
    let scale = dst as f64 / src as f64;
    let fscale = if scale < 1.0 { 1.0 / scale } else { 1.0 };
    let support = filter.support() * fscale;
    (0..dst)
        .map(|i| {
            let center = (i as f64 + 0.5) / scale;
            if filter == Filter::Nearest {
                let s = (center.floor() as usize).min(src - 1);
                return Contrib { start: s, weights: vec![1.0] };
            }
            let lo = ((center - support).floor() as i64).max(0) as usize;
            let hi = ((center + support).ceil() as i64).min(src as i64) as usize;
            let mut ws: Vec<f64> = (lo..hi).map(|j| filter.eval((j as f64 + 0.5 - center) / fscale)).collect();
            let sum: f64 = ws.iter().sum();
            if sum.abs() > 1e-12 {
                ws.iter_mut().for_each(|w| *w /= sum);
            }
            Contrib { start: lo, weights: ws.into_iter().map(|w| w as f32).collect() }
        })
        .collect()
}

fn resample_rows(data: &[f32], w: usize, h: usize, c: usize, nw: usize, filter: Filter) -> Vec<f32> {
    let contribs = contributions(w, nw, filter);
    let mut out = vec![0f32; nw * h * c];
    out.par_chunks_mut(nw * c).enumerate().for_each(|(y, row)| {
        let src = &data[y * w * c..(y + 1) * w * c];
        for (x, ct) in contribs.iter().enumerate() {
            let mut acc = [0f32; 4];
            for (k, &wt) in ct.weights.iter().enumerate() {
                let s = (ct.start + k) * c;
                for ch in 0..c {
                    acc[ch] += src[s + ch] * wt;
                }
            }
            row[x * c..x * c + c].copy_from_slice(&acc[..c]);
        }
    });
    out
}

/// Resize an image to `nw`×`nh` (premultiplied internally; result clamped to 0..1).
pub fn resize(img: &Image, nw: u32, nh: u32, filter: Filter) -> Image {
    if nw == img.width && nh == img.height {
        return img.clone();
    }
    if img.width == 0 || img.height == 0 || nw == 0 || nh == 0 {
        return Image::new(nw, nh);
    }
    let mut pm = img.clone();
    pm.premultiply();
    let (w, h) = (img.width as usize, img.height as usize);
    let rows = resample_rows(&pm.data, w, h, 4, nw as usize, filter);
    let t = transpose(&rows, nw as usize, h, 4);
    let cols = resample_rows(&t, h, nw as usize, 4, nh as usize, filter);
    let data = transpose(&cols, nh as usize, nw as usize, 4);
    let mut out = Image { width: nw, height: nh, data };
    out.map_pixels(|p| {
        // snap float drift so opaque stays exactly opaque and empty stays empty
        p[3] = if p[3] > 0.99999 {
            1.0
        } else if p[3] < 1e-5 {
            0.0
        } else {
            p[3]
        };
        if p[3] > 1e-7 {
            let a = p[3];
            for v in p.iter_mut().take(3) {
                *v = (*v / a).clamp(0.0, 1.0);
            }
        } else {
            p[0] = 0.0;
            p[1] = 0.0;
            p[2] = 0.0;
        }
    });
    out
}

/// Resize a mask.
pub fn resize_mask(m: &Mask, nw: u32, nh: u32, filter: Filter) -> Mask {
    if nw == m.width && nh == m.height {
        return m.clone();
    }
    let (w, h) = (m.width as usize, m.height as usize);
    let rows = resample_rows(&m.data, w, h, 1, nw as usize, filter);
    let t = transpose(&rows, nw as usize, h, 1);
    let cols = resample_rows(&t, h, nw as usize, 1, nh as usize, filter);
    let mut data = transpose(&cols, nh as usize, nw as usize, 1);
    data.par_iter_mut().for_each(|v| *v = v.clamp(0.0, 1.0));
    Mask { width: nw, height: nh, data }
}

// ---------------------------------------------------------------------------------------
// morphology and smoothing (masks)

/// Sliding maximum of width `2r+1` over a row (van Herk / Gil-Werman, O(1) per sample).
fn sliding_max(src: &[f32], r: usize, out: &mut [f32]) {
    let n = src.len();
    if r == 0 {
        out.copy_from_slice(src);
        return;
    }
    let win = 2 * r + 1;
    // pad with zeros (outside counts as empty)
    let padded: Vec<f32> =
        std::iter::repeat_n(0.0, r).chain(src.iter().copied()).chain(std::iter::repeat_n(0.0, r)).collect();
    let m = padded.len();
    let mut g = vec![0f32; m];
    let mut hh = vec![0f32; m];
    for i in 0..m {
        g[i] = if i % win == 0 { padded[i] } else { g[i - 1].max(padded[i]) };
    }
    for i in (0..m).rev() {
        hh[i] = if i == m - 1 || (i + 1) % win == 0 { padded[i] } else { hh[i + 1].max(padded[i]) };
    }
    for (x, o) in out.iter_mut().enumerate().take(n) {
        let lo = x;
        let hi = x + win - 1;
        *o = hh[lo].max(g[hi.min(m - 1)]);
    }
}

/// Grayscale dilation by a disc of radius `r` (grows covered areas).
pub fn dilate(m: &Mask, r: f64) -> Mask {
    if r <= 0.0 {
        return m.clone();
    }
    let (w, h) = (m.width as usize, m.height as usize);
    let ri = r.round() as i64;
    // horizontal half-widths of the disc per row offset
    let chords: Vec<(i64, usize)> =
        (-ri..=ri).map(|dy| (dy, ((r * r - (dy * dy) as f64).max(0.0).sqrt()).floor() as usize)).collect();
    // per distinct half-width, row-wise sliding maxima
    let mut widths: Vec<usize> = chords.iter().map(|c| c.1).collect();
    widths.sort();
    widths.dedup();
    let maxed: Vec<(usize, Vec<f32>)> = widths
        .par_iter()
        .map(|&hw| {
            let mut out = vec![0f32; w * h];
            for y in 0..h {
                sliding_max(&m.data[y * w..(y + 1) * w], hw, &mut out[y * w..(y + 1) * w]);
            }
            (hw, out)
        })
        .collect();
    let lookup = |hw: usize| &maxed.iter().find(|(k, _)| *k == hw).unwrap().1;
    let mut out = Mask::new(m.width, m.height);
    out.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for &(dy, hw) in &chords {
            let sy = y as i64 + dy;
            if sy < 0 || sy >= h as i64 {
                continue;
            }
            let src = &lookup(hw)[sy as usize * w..(sy as usize + 1) * w];
            for (o, s) in row.iter_mut().zip(src) {
                if *s > *o {
                    *o = *s;
                }
            }
        }
    });
    out
}

/// Grayscale erosion by a disc of radius `r` (shrinks covered areas).
pub fn erode(m: &Mask, r: f64) -> Mask {
    let mut inv = m.inverted();
    // outside the canvas counts as covered for erosion, so edges do not erode inward
    inv = dilate(&inv, r);
    inv.invert();
    inv
}

/// Median filter of a mask over a square window of radius `r` (256-bin sliding histogram).
pub fn median_mask(m: &Mask, r: usize) -> Mask {
    if r == 0 {
        return m.clone();
    }
    let (w, h) = (m.width as usize, m.height as usize);
    let q: Vec<u8> = m.data.iter().map(|&v| (v.clamp(0.0, 1.0) * 255.0).round() as u8).collect();
    let mut out = Mask::new(m.width, m.height);
    out.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let y0 = y.saturating_sub(r);
        let y1 = (y + r).min(h - 1);
        let mut hist = [0u32; 256];
        let mut count = 0u32;
        let add_col = |hist: &mut [u32; 256], count: &mut u32, x: usize, sign: i32| {
            for yy in y0..=y1 {
                let v = q[yy * w + x] as usize;
                if sign > 0 {
                    hist[v] += 1;
                    *count += 1;
                } else {
                    hist[v] -= 1;
                    *count -= 1;
                }
            }
        };
        for x in 0..=r.min(w - 1) {
            add_col(&mut hist, &mut count, x, 1);
        }
        for (x, o) in row.iter_mut().enumerate() {
            if x > 0 {
                if x + r < w {
                    add_col(&mut hist, &mut count, x + r, 1);
                }
                if x > r {
                    add_col(&mut hist, &mut count, x - r - 1, -1);
                }
            }
            let half = count.div_ceil(2);
            let mut acc = 0;
            for (v, &c) in hist.iter().enumerate() {
                acc += c;
                if acc >= half {
                    *o = v as f32 / 255.0;
                    break;
                }
            }
        }
    });
    out
}

/// Convolve an image with an arbitrary odd-sized kernel (row-major, `kw`×`kh`), clamping
/// at edges. Alpha is convolved too unless `keep_alpha`.
pub fn convolve(img: &Image, kernel: &[f32], kw: usize, kh: usize, keep_alpha: bool) -> Image {
    let (w, h) = (img.width as i64, img.height as i64);
    let (rx, ry) = ((kw / 2) as i64, (kh / 2) as i64);
    let mut out = img.clone();
    out.data.par_chunks_mut(w as usize * 4).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let mut acc = [0f32; 4];
            for ky in 0..kh as i64 {
                for kx in 0..kw as i64 {
                    let kv = kernel[(ky * kw as i64 + kx) as usize];
                    if kv == 0.0 {
                        continue;
                    }
                    let p = img.get_clamped(x + kx - rx, y as i64 + ky - ry);
                    for c in 0..4 {
                        acc[c] += p[c] * kv;
                    }
                }
            }
            let o = &mut row[x as usize * 4..x as usize * 4 + 4];
            o[0] = acc[0];
            o[1] = acc[1];
            o[2] = acc[2];
            if !keep_alpha {
                o[3] = acc[3];
            }
        }
    });
    let _ = h;
    out
}

// ---------------------------------------------------------------------------------------
// comparison

/// Mean SSIM of two same-sized images on luma (Gaussian window σ = 1.5).
pub fn ssim(a: &Image, b: &Image) -> f64 {
    assert_eq!((a.width, a.height), (b.width, b.height));
    let (w, h) = (a.width as usize, a.height as usize);
    let la: Vec<f32> = a
        .data
        .chunks(4)
        .map(|p| crate::color::luma(p[0] * p[3] + (1.0 - p[3]), p[1] * p[3] + (1.0 - p[3]), p[2] * p[3] + (1.0 - p[3])))
        .collect();
    let lb: Vec<f32> = b
        .data
        .chunks(4)
        .map(|p| crate::color::luma(p[0] * p[3] + (1.0 - p[3]), p[1] * p[3] + (1.0 - p[3]), p[2] * p[3] + (1.0 - p[3])))
        .collect();
    let blur = |v: Vec<f32>| {
        let mut v = v;
        gaussian_planar(&mut v, w, h, 1, 1.5, 1.5, Edge::Clamp);
        v
    };
    let mu_a = blur(la.clone());
    let mu_b = blur(lb.clone());
    let saa = blur(la.iter().map(|v| v * v).collect());
    let sbb = blur(lb.iter().map(|v| v * v).collect());
    let sab = blur(la.iter().zip(&lb).map(|(x, y)| x * y).collect());
    let c1 = (0.01f64).powi(2);
    let c2 = (0.03f64).powi(2);
    // per-row partial sums, then a sequential sum: the result never depends on thread count
    let rows: Vec<f64> = (0..h)
        .into_par_iter()
        .map(|y| {
            let mut acc = 0.0f64;
            for i in y * w..(y + 1) * w {
                let (ma, mb) = (mu_a[i] as f64, mu_b[i] as f64);
                let va = (saa[i] as f64 - ma * ma).max(0.0);
                let vb = (sbb[i] as f64 - mb * mb).max(0.0);
                let cov = sab[i] as f64 - ma * mb;
                acc += ((2.0 * ma * mb + c1) * (2.0 * cov + c2)) / ((ma * ma + mb * mb + c1) * (va + vb + c2));
            }
            acc
        })
        .collect();
    let total: f64 = rows.iter().sum();
    if w * h == 0 {
        1.0
    } else {
        total / (w * h) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Rgba;

    fn checker(w: u32, h: u32) -> Image {
        Image::from_fn(w, h, |x, y| if (x / 4 + y / 4) % 2 == 0 { [1.0, 1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0] })
    }

    #[test]
    fn blur_preserves_flat_fields_and_mean() {
        let flat = Image::filled(40, 30, Rgba::new(0.3, 0.6, 0.9, 1.0));
        for sigma in [0.8, 3.0, 25.0] {
            let b = gaussian_blur(&flat, sigma, Edge::Clamp);
            for v in b.data.chunks(4) {
                assert!((v[0] - 0.3).abs() < 1e-4 && (v[2] - 0.9).abs() < 1e-4, "sigma {sigma}");
            }
        }
        let c = checker(64, 64);
        let b = gaussian_blur(&c, 2.0, Edge::Clamp);
        let mean = |i: &Image| i.data.chunks(4).map(|p| p[0] as f64).sum::<f64>() / i.pixels() as f64;
        assert!((mean(&c) - mean(&b)).abs() < 0.02);
    }

    #[test]
    fn zero_edge_fades_alpha() {
        let img = Image::filled(20, 20, Rgba::WHITE);
        let b = gaussian_blur(&img, 3.0, Edge::Zero);
        assert!(b.get(0, 0)[3] < 0.6);
        assert!(b.get(10, 10)[3] > 0.95);
        // color stays white where alpha remains (premultiplied blur)
        assert!((b.get(0, 0)[0] - 1.0).abs() < 1e-3);
    }

    #[test]
    fn resize_dimensions_and_flat_color() {
        let img = Image::filled(100, 50, Rgba::new(0.2, 0.4, 0.6, 1.0));
        for f in [Filter::Nearest, Filter::Bilinear, Filter::Bicubic, Filter::Lanczos3] {
            let r = resize(&img, 37, 81, f);
            assert_eq!((r.width, r.height), (37, 81));
            for p in r.data.chunks(4) {
                assert!((p[1] - 0.4).abs() < 1e-3, "{f:?}");
            }
        }
    }

    #[test]
    fn morphology_grows_and_shrinks() {
        let m = Mask::from_fn(21, 21, |x, y| if x == 10 && y == 10 { 1.0 } else { 0.0 });
        let d = dilate(&m, 3.0);
        assert_eq!(d.get(13, 10), 1.0);
        assert_eq!(d.get(10, 7), 1.0);
        assert_eq!(d.get(13, 13), 0.0, "disc, not square");
        let square = Mask::from_fn(21, 21, |x, y| if (5..16).contains(&x) && (5..16).contains(&y) { 1.0 } else { 0.0 });
        let e = erode(&square, 2.0);
        assert_eq!(e.get(10, 10), 1.0);
        assert_eq!(e.get(5, 10), 0.0);
        assert_eq!(e.get(7, 10), 1.0);
    }

    #[test]
    fn median_removes_speckles() {
        let mut m = Mask::filled(15, 15, 0.0);
        m.set(7, 7, 1.0);
        let s = median_mask(&m, 1);
        assert_eq!(s.get(7, 7), 0.0);
    }

    #[test]
    fn ssim_identity_and_difference() {
        let c = checker(32, 32);
        assert!((ssim(&c, &c) - 1.0).abs() < 1e-9);
        let b = gaussian_blur(&c, 2.0, Edge::Clamp);
        assert!(ssim(&c, &b) < 0.9);
    }
}
