//! Performance-critical kernels. Everything here is O(pixels) per pass and written so the
//! inner loops run over contiguous slices (they auto-vectorize); column passes work in
//! strips of columns with a ring buffer of rows, so nothing is transposed and the large
//! buffers are updated in place.
//!
//! Bounds that are not radius-independent:
//! * direct Gaussian convolution up to sigma 4 (≤ 25 taps); above, three running boxes;
//! * disc/hex bokeh: O(r) per pixel up to r = 8, then computed at reduced resolution;
//! * surface blur: brute force over the (2r+1)² window for r ≤ 3 (≤ 49 taps), else a
//!   piecewise-linear bilateral filter on a grid downsampled by max(2, r/4), with at most
//!   32 range levels;
//! * non-local means: fixed 5×5 search window and 3×3 patches.

use crate::util::{self, Buf, SharedMut};
use pixelox_core::kernels::Edge;
use pixelox_core::prelude::*;
use rayon::prelude::*;

/// Width in floats of the column strips used by vertical passes.
const STRIP: usize = 1024;

// ---------------------------------------------------------------------------------------
// Gaussian and box

/// Normalized Gaussian taps of radius ceil(3σ).
pub fn gauss_kernel(sigma: f64) -> Vec<f32> {
    let r = (sigma * 3.0).ceil().max(1.0) as i64;
    let k: Vec<f64> = (-r..=r).map(|i| (-((i * i) as f64) / (2.0 * sigma * sigma)).exp()).collect();
    let s: f64 = k.iter().sum();
    k.iter().map(|v| (v / s) as f32).collect()
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

/// In-place convolution of every row with a symmetric odd kernel.
pub fn conv_rows(data: &mut [f32], w: usize, c: usize, k: &[f32], edge: Edge) {
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

/// In-place convolution of every column with a symmetric odd kernel.
pub fn conv_cols(data: &mut [f32], w: usize, h: usize, c: usize, k: &[f32], edge: Edge) {
    if w == 0 || h == 0 || k.len() <= 1 {
        return;
    }
    let r = k.len() / 2;
    let stride = w * c;
    let ksz = (2 * r + 1).min(h);
    let ptr = SharedMut::new(data);
    (0..stride.div_ceil(STRIP)).into_par_iter().for_each(|s| {
        let a = s * STRIP;
        let n = (a + STRIP).min(stride) - a;
        let mut ring = vec![0f32; ksz * n];
        let mut out = vec![0f32; n];
        // SAFETY (all accesses below): each task reads and writes only its own column
        // strip [a, a + n) of each row; rows are loaded into the ring before being overwritten.
        for row in 0..r.min(h) {
            unsafe { ptr.read(row * stride + a, &mut ring[(row % ksz) * n..(row % ksz + 1) * n]) };
        }
        for y in 0..h {
            if y + r < h {
                let row = y + r;
                unsafe { ptr.read(row * stride + a, &mut ring[(row % ksz) * n..(row % ksz + 1) * n]) };
            }
            out.fill(0.0);
            for (j, &kj) in k.iter().enumerate() {
                let sy = y as i64 + j as i64 - r as i64;
                let sy = if sy < 0 || sy >= h as i64 {
                    if edge == Edge::Zero {
                        continue;
                    }
                    sy.clamp(0, h as i64 - 1)
                } else {
                    sy
                } as usize;
                let slot = sy % ksz;
                for (o, &v) in out.iter_mut().zip(&ring[slot * n..slot * n + n]) {
                    *o += kj * v;
                }
            }
            unsafe { ptr.write(y * stride + a, &out) };
        }
    });
}

/// In-place running-sum box filter of radius `r` along every row (f64 sums).
pub fn box_rows(data: &mut [f32], w: usize, c: usize, r: usize, edge: Edge) {
    if r == 0 || w == 0 {
        return;
    }
    let norm = 1.0 / (2 * r + 1) as f64;
    data.par_chunks_mut(w * c).for_each_init(
        || (Vec::new(), vec![0f64; c]),
        |(pad, acc), row| {
            pad_row(row, c, r + 1, edge, pad);
            acc.fill(0.0);
            for i in 1..=2 * r + 1 {
                for ch in 0..c {
                    acc[ch] += pad[i * c + ch] as f64;
                }
            }
            for x in 0..w {
                for ch in 0..c {
                    row[x * c + ch] = (acc[ch] * norm) as f32;
                }
                let (ai, ri) = ((x + 2 * r + 2) * c, (x + 1) * c);
                for ch in 0..c {
                    acc[ch] += pad[ai + ch] as f64 - pad[ri + ch] as f64;
                }
            }
        },
    );
}

/// In-place running-sum box filter of radius `r` along every column.
pub fn box_cols(data: &mut [f32], w: usize, h: usize, c: usize, r: usize, edge: Edge) {
    if r == 0 || w == 0 || h == 0 {
        return;
    }
    let stride = w * c;
    let norm = 1.0 / (2 * r + 1) as f64;
    let ksz = (2 * r + 2).min(h);
    let ptr = SharedMut::new(data);
    let hi = h as i64;
    (0..stride.div_ceil(STRIP)).into_par_iter().for_each(|s| {
        let a = s * STRIP;
        let n = (a + STRIP).min(stride) - a;
        let mut ring = vec![0f32; ksz * n];
        let mut first = vec![0f32; n];
        let mut last = vec![0f32; n];
        let mut acc = vec![0f64; n];
        let mut out = vec![0f32; n];
        // SAFETY (all accesses below): each task only touches its own column strip, and a
        // row is copied into the ring before its output overwrites it.
        unsafe {
            ptr.read(a, &mut first);
            ptr.read((h - 1) * stride + a, &mut last);
        }
        for row in 0..=r.min(h - 1) {
            unsafe { ptr.read(row * stride + a, &mut ring[(row % ksz) * n..(row % ksz + 1) * n]) };
        }
        let slice_of = |row: i64, ring: &[f32]| -> Option<(usize, bool)> {
            // Some((offset, from_ring)) ; offset into ring, or into first/last when !from_ring
            let _ = ring;
            if row < 0 {
                (edge == Edge::Clamp).then_some((0, false))
            } else if row >= hi {
                (edge == Edge::Clamp).then_some((1, false))
            } else {
                Some(((row as usize % ksz) * n, true))
            }
        };
        let add_row = |acc: &mut [f64], row: i64, ring: &[f32], sign: f64| {
            if let Some((off, in_ring)) = slice_of(row, ring) {
                let src: &[f32] = if in_ring {
                    &ring[off..off + n]
                } else if off == 0 {
                    &first
                } else {
                    &last
                };
                for (a, &v) in acc.iter_mut().zip(src) {
                    *a += sign * v as f64;
                }
            }
        };
        for j in -(r as i64)..=r as i64 {
            add_row(&mut acc, j, &ring, 1.0);
        }
        for y in 0..h {
            for (o, &v) in out.iter_mut().zip(&acc) {
                *o = (v * norm) as f32;
            }
            unsafe { ptr.write(y * stride + a, &out) };
            let add = y + r + 1;
            if add < h {
                unsafe { ptr.read(add * stride + a, &mut ring[(add % ksz) * n..(add % ksz + 1) * n]) };
            }
            add_row(&mut acc, add as i64, &ring, 1.0);
            add_row(&mut acc, y as i64 - r as i64, &ring, -1.0);
        }
    });
}

/// Box blur with radii `rx`, `ry` in place (O(1) per pixel).
pub fn box_blur(buf: &mut Buf, rx: usize, ry: usize, edge: Edge) {
    box_rows(&mut buf.data, buf.w, buf.c, rx, edge);
    box_cols(&mut buf.data, buf.w, buf.h, buf.c, ry, edge);
}

/// Gaussian blur in place: direct separable convolution up to sigma 4, three running
/// boxes above (O(1) per pixel for any sigma).
pub fn gaussian(buf: &mut Buf, sigma_x: f64, sigma_y: f64, edge: Edge) {
    const DIRECT_MAX: f64 = 4.0;
    let (w, h, c) = (buf.w, buf.h, buf.c);
    if sigma_x > 0.0 {
        if sigma_x <= DIRECT_MAX {
            conv_rows(&mut buf.data, w, c, &gauss_kernel(sigma_x), edge);
        } else {
            for r in boxes_for_gauss(sigma_x) {
                box_rows(&mut buf.data, w, c, r, edge);
            }
        }
    }
    if sigma_y > 0.0 {
        if sigma_y <= DIRECT_MAX {
            conv_cols(&mut buf.data, w, h, c, &gauss_kernel(sigma_y), edge);
        } else {
            for r in boxes_for_gauss(sigma_y) {
                box_cols(&mut buf.data, w, h, c, r, edge);
            }
        }
    }
}

/// Gaussian blur of an image (premultiplied; opaque layers clamp, others fade at edges).
pub fn gaussian_image(img: &Image, sigma: f64) -> Image {
    if sigma <= 0.0 || img.is_empty() {
        return img.clone();
    }
    let edge = pixelox_core::kernels::auto_edge(img);
    let mut b = Buf::premul(img);
    gaussian(&mut b, sigma, sigma, edge);
    b.into_unpremul()
}

// ---------------------------------------------------------------------------------------
// uniform bokeh kernels

/// Aperture shape of a lens blur.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Circle,
    Hex,
}

/// Largest radius blurred at full resolution; larger radii are computed on a buffer
/// downsampled so the radius becomes at most this, keeping the cost constant.
pub(crate) const SHAPE_CAP: f32 = 8.0;

/// Uniform blur with a disc or (flat-topped) hexagon of radius `r`: exact per-row segment
/// sums from prefix sums, O(r) per pixel for r ≤ 8, constant beyond (reduced resolution).
pub fn shape_blur(src: &Buf, r: f32, shape: Shape, edge: Edge) -> Buf {
    if r < 0.5 || src.w == 0 || src.h == 0 {
        return src.clone();
    }
    if r > SHAPE_CAP {
        let f = (r / SHAPE_CAP).ceil() as usize;
        let small = util::downsample(src, f);
        let b = shape_blur_direct(&small, r / f as f32, shape, edge);
        return util::upsample(&b, src.w, src.h, f);
    }
    shape_blur_direct(src, r, shape, edge)
}

/// Per-row half-widths of the kernel and its area.
fn shape_rows(r: f32, shape: Shape) -> (Vec<(i64, f32)>, f32) {
    let ri = r.ceil() as i64;
    let mut rows = Vec::new();
    for dy in -ri..=ri {
        let ady = dy.unsigned_abs() as f32;
        let hw = match shape {
            Shape::Circle => {
                if ady > r {
                    continue;
                }
                (r * r - ady * ady).sqrt()
            }
            Shape::Hex => {
                let half_h = r * 3f32.sqrt() / 2.0;
                if ady > half_h + 0.5 {
                    continue;
                }
                (r - ady.min(half_h) / 3f32.sqrt()).max(0.0)
            }
        };
        rows.push((dy, hw));
    }
    let area = rows.iter().map(|(_, hw)| 2.0 * hw + 1.0).sum();
    (rows, area)
}

/// [`shape_blur`] at the buffer's own resolution (any radius; O(r) per pixel).
pub(crate) fn shape_blur_direct(src: &Buf, r: f32, shape: Shape, edge: Edge) -> Buf {
    let (w, h, c) = (src.w, src.h, src.c);
    let (rows, area) = shape_rows(r, shape);
    let inv_area = 1.0 / area;
    let ri = rows.iter().map(|(dy, _)| dy.abs()).max().unwrap_or(0);
    const BAND: usize = 32;
    let mut out = Buf::new(w, h, c);
    let clamp = edge == Edge::Clamp;
    out.data.par_chunks_mut(BAND * w * c).enumerate().for_each(|(bi, chunk)| {
        let y0 = bi * BAND;
        let y1 = (y0 + BAND).min(h);
        let sy0 = (y0 as i64 - ri).max(0) as usize;
        let sy1 = ((y1 - 1) as i64 + ri).min(h as i64 - 1) as usize;
        let stride = (w + 1) * c;
        // prefix sums of each needed source row: pre[row][i*c + ch] = Σ_{x<i} src
        let mut pre = vec![0f32; (sy1 - sy0 + 1) * stride];
        for (k, sy) in (sy0..=sy1).enumerate() {
            let p = &mut pre[k * stride..(k + 1) * stride];
            let row = &src.data[sy * w * c..(sy + 1) * w * c];
            for x in 0..w {
                for ch in 0..c {
                    p[(x + 1) * c + ch] = p[x * c + ch] + row[x * c + ch];
                }
            }
        }
        // F(u): integral of the row from pixel edge 0 to u (pixel i spans [i, i+1))
        let wf = w as f32;
        let fval = |p: &[f32], u: f32, ch: usize| -> f32 {
            if u <= 0.0 {
                if clamp {
                    u * p[c + ch]
                } else {
                    0.0
                }
            } else if u >= wf {
                if clamp {
                    p[w * c + ch] + (u - wf) * (p[w * c + ch] - p[(w - 1) * c + ch])
                } else {
                    p[w * c + ch]
                }
            } else {
                let i = u as usize;
                let fr = u - i as f32;
                p[i * c + ch] + fr * (p[(i + 1) * c + ch] - p[i * c + ch])
            }
        };
        // per kernel row: segment [x - hw, x + hw + 1) has constant integer/fractional
        // offsets, so the interior runs over contiguous slices
        let segs: Vec<(i64, f32, i64, f32, f32)> = rows
            .iter()
            .map(|&(dy, hw)| {
                let (a, b) = (-hw, hw + 1.0);
                (a.floor() as i64, a - a.floor(), b.floor() as i64, b - b.floor(), dy as f32)
            })
            .collect();
        let mut edge_acc = vec![0f32; c];
        for y in y0..y1 {
            let orow = &mut chunk[(y - y0) * w * c..(y - y0 + 1) * w * c];
            orow.fill(0.0);
            for (&(dy, hw), &(ia, fa, ib, fb, _)) in rows.iter().zip(&segs) {
                let mut sy = y as i64 + dy;
                if sy < 0 || sy >= h as i64 {
                    if !clamp {
                        continue;
                    }
                    sy = sy.clamp(0, h as i64 - 1);
                }
                let p = &pre[(sy as usize - sy0) * stride..(sy as usize - sy0 + 1) * stride];
                let x_lo = (-ia).max(0) as usize;
                let x_hi = (w as i64 - 1 - ib).min(w as i64 - 1);
                let interior = if x_hi >= x_lo as i64 { x_lo..x_hi as usize + 1 } else { 0..0 };
                if !interior.is_empty() {
                    let n = interior.len() * c;
                    let ba = (x_lo as i64 + ia) as usize * c;
                    let bb = (x_lo as i64 + ib) as usize * c;
                    let (pa0, pa1) = (&p[ba..ba + n], &p[ba + c..ba + c + n]);
                    let (pb0, pb1) = (&p[bb..bb + n], &p[bb + c..bb + c + n]);
                    let o = &mut orow[x_lo * c..x_lo * c + n];
                    for i in 0..n {
                        o[i] += (pb0[i] + fb * (pb1[i] - pb0[i])) - (pa0[i] + fa * (pa1[i] - pa0[i]));
                    }
                }
                for x in (0..w).filter(|x| !interior.contains(x)) {
                    let (ua, ub) = (x as f32 - hw, x as f32 + hw + 1.0);
                    for ch in 0..c {
                        edge_acc[ch] = fval(p, ub, ch) - fval(p, ua, ch);
                        orow[x * c + ch] += edge_acc[ch];
                    }
                }
            }
            orow.iter_mut().for_each(|v| *v *= inv_area);
        }
    });
    out
}

// ---------------------------------------------------------------------------------------
// directional (motion) box blur

/// Box blur of length `dist` along `angle_deg` (counter-clockwise from horizontal, y up).
/// O(pixels): each line of the direction is sampled once, prefix-summed, and scattered
/// back. Clamp edges renormalize by the in-image coverage; Zero edges fade.
pub fn motion_blur(src: &Buf, angle_deg: f64, dist: f64, edge: Edge) -> Buf {
    if dist < 0.5 || src.w == 0 || src.h == 0 {
        return src.clone();
    }
    let th = angle_deg.to_radians();
    let (dx, dy) = (th.cos(), -th.sin());
    if dx.abs() >= dy.abs() {
        shear_box(src, dy / dx, dist * dx.abs(), edge)
    } else {
        shear_box(&src.transpose(), dx / dy, dist * dy.abs(), edge).transpose()
    }
}

/// Lines `y = j + s·x` (|s| ≤ 1): average a window of `len` (in x units) along each line,
/// sampling rows by linear interpolation and scattering back with the same weights.
fn shear_box(src: &Buf, s: f64, len: f64, edge: Edge) -> Buf {
    let (w, h, c) = (src.w, src.h, src.c);
    let cc = c + 1;
    let len = len.max(1.0);
    let half = len / 2.0;
    let mut out = Buf::new(w, h, c);
    let mut vsum = vec![0f32; w * h];
    let span = s * (w as f64 - 1.0);
    let j_lo = (-1.0 - span.max(0.0)).floor() as i64;
    let j_hi = (h as f64 - span.min(0.0)).ceil() as i64;
    // window [k + ca, k + cb] for output sample k; integer parts are constant along a line
    let (ca, cb) = (0.5 - half, 0.5 + half);
    let (ia, ib) = (ca.floor() as i64, cb.floor() as i64);
    let (fa, fb) = (ca - ia as f64, cb - ib as f64);
    let padl = (-ia).max(0) as usize + 1;
    let padr = ib.max(0) as usize + 2;
    let po = SharedMut::new(&mut out.data);
    let pv = SharedMut::new(&mut vsum);
    let (hf, wl) = (h as f64, w as i64 - 1);
    for parity in 0..2 {
        let js: Vec<i64> = (j_lo..=j_hi).filter(|j| j.rem_euclid(2) == parity).collect();
        js.par_iter().for_each_init(
            || (Vec::<f64>::new(), vec![0f32; cc]),
            |(pre, v), &j| {
                let jf = j as f64;
                let (xa, xb) = if s == 0.0 {
                    if jf > -1.0 && jf < hf {
                        (0, wl)
                    } else {
                        return;
                    }
                } else if s > 0.0 {
                    (((-1.0 - jf) / s).floor() as i64, ((hf - jf) / s).ceil() as i64)
                } else {
                    (((hf - jf) / s).floor() as i64, ((-1.0 - jf) / s).ceil() as i64)
                };
                let (xa, xb) = (xa.max(0), xb.min(wl));
                if xa > xb {
                    return;
                }
                let n = (xb - xa + 1) as usize;
                let total = padl + n + padr + 1;
                pre.clear();
                pre.resize(total * cc, 0.0);
                for k in 0..n {
                    let x = xa as usize + k;
                    let yf = jf + s * x as f64;
                    let y0 = yf.floor();
                    let t = (yf - y0) as f32;
                    let y0 = y0 as i64;
                    v.fill(0.0);
                    for (yy, wt) in [(y0, 1.0 - t), (y0 + 1, t)] {
                        if wt <= 0.0 || yy < 0 || yy >= h as i64 {
                            continue;
                        }
                        let i = (yy as usize * w + x) * c;
                        for ch in 0..c {
                            v[ch] += src.data[i + ch] * wt;
                        }
                        v[c] += wt;
                    }
                    let (p0, p1) = ((padl + k) * cc, (padl + k + 1) * cc);
                    for ch in 0..cc {
                        pre[p1 + ch] = pre[p0 + ch] + v[ch] as f64;
                    }
                }
                let end = (padl + n) * cc;
                for idx in padl + n + 1..total {
                    for ch in 0..cc {
                        pre[idx * cc + ch] = pre[end + ch];
                    }
                }
                for k in 0..n {
                    let x = xa as usize + k;
                    let i0 = (padl as i64 + k as i64 + ia) as usize * cc;
                    let i1 = (padl as i64 + k as i64 + ib) as usize * cc;
                    for ch in 0..cc {
                        let lo = pre[i0 + ch] + fa * (pre[i0 + cc + ch] - pre[i0 + ch]);
                        let hi = pre[i1 + ch] + fb * (pre[i1 + cc + ch] - pre[i1 + ch]);
                        v[ch] = ((hi - lo) / len) as f32;
                    }
                    let yf = jf + s * x as f64;
                    let y0 = yf.floor();
                    let t = (yf - y0) as f32;
                    let y0 = y0 as i64;
                    for (yy, wt) in [(y0, 1.0 - t), (y0 + 1, t)] {
                        if wt <= 0.0 || yy < 0 || yy >= h as i64 {
                            continue;
                        }
                        let pix = yy as usize * w + x;
                        // SAFETY: lines of one parity never touch the same pixel: line j
                        // writes rows floor(j+s·x) and +1 of column x, line j+2 rows +2, +3.
                        unsafe {
                            for ch in 0..c {
                                po.add(pix * c + ch, v[ch] * wt);
                            }
                            pv.add(pix, v[c] * wt);
                        }
                    }
                }
            },
        );
    }
    if edge == Edge::Clamp {
        out.data.par_chunks_mut(c).zip(vsum.par_iter()).for_each(|(o, &v)| {
            let norm = if v > 1e-6 { 1.0 / v } else { 0.0 };
            o.iter_mut().for_each(|x| *x *= norm);
        });
    }
    out
}

// ---------------------------------------------------------------------------------------
// local min / max

/// Sliding minimum and maximum over a (2r+1)² square with replicated edges (r is small).
pub fn min_max(plane: &[f32], w: usize, h: usize, r: usize) -> (Vec<f32>, Vec<f32>) {
    let mut hmin = vec![0f32; plane.len()];
    let mut hmax = vec![0f32; plane.len()];
    hmin.par_chunks_mut(w).zip(hmax.par_chunks_mut(w)).zip(plane.par_chunks(w)).for_each_init(
        Vec::new,
        |pad, ((mn, mx), row)| {
            pad_row(row, 1, r, Edge::Clamp, pad);
            mn.copy_from_slice(&pad[..w]);
            mx.copy_from_slice(&pad[..w]);
            for j in 1..=2 * r {
                for ((a, b), &v) in mn.iter_mut().zip(mx.iter_mut()).zip(&pad[j..j + w]) {
                    *a = a.min(v);
                    *b = b.max(v);
                }
            }
        },
    );
    let mut vmin = vec![0f32; plane.len()];
    let mut vmax = vec![0f32; plane.len()];
    vmin.par_chunks_mut(w).zip(vmax.par_chunks_mut(w)).enumerate().for_each(|(y, (mn, mx))| {
        mn.fill(f32::MAX);
        mx.fill(f32::MIN);
        for dy in -(r as i64)..=r as i64 {
            let sy = (y as i64 + dy).clamp(0, h as i64 - 1) as usize;
            let (a, b) = (&hmin[sy * w..(sy + 1) * w], &hmax[sy * w..(sy + 1) * w]);
            for (((o1, o2), &v1), &v2) in mn.iter_mut().zip(mx.iter_mut()).zip(a).zip(b) {
                *o1 = o1.min(v1);
                *o2 = o2.max(v2);
            }
        }
    });
    (vmin, vmax)
}

// ---------------------------------------------------------------------------------------
// median

#[inline(always)]
fn sort3(a: f32, b: f32, c: f32) -> (f32, f32, f32) {
    let lo = a.min(b).min(c);
    let hi = a.max(b).max(c);
    let mid = a.min(b).max(a.max(b).min(c));
    (lo, mid, hi)
}

#[inline(always)]
fn med3(a: f32, b: f32, c: f32) -> f32 {
    a.min(b).max(a.max(b).min(c))
}

/// Exact 3×3 median of a plane (replicated edges): sort each column triple once, then the
/// median is med3(max of lows, med of mids, min of highs).
pub fn median3x3(plane: &[f32], w: usize, h: usize) -> Vec<f32> {
    let mut out = vec![0f32; plane.len()];
    out.par_chunks_mut(w).enumerate().for_each_init(
        || (vec![0f32; w + 2], vec![0f32; w + 2], vec![0f32; w + 2]),
        |(lo, mi, hi), (y, orow)| {
            let up = &plane[y.saturating_sub(1) * w..][..w];
            let md = &plane[y * w..][..w];
            let dn = &plane[(y + 1).min(h - 1) * w..][..w];
            for x in 0..w {
                let (a, b, c) = sort3(up[x], md[x], dn[x]);
                lo[x + 1] = a;
                mi[x + 1] = b;
                hi[x + 1] = c;
            }
            for v in [&mut *lo, &mut *mi, &mut *hi] {
                v[0] = v[1];
                v[w + 1] = v[w];
            }
            for x in 0..w {
                let a = lo[x].max(lo[x + 1]).max(lo[x + 2]);
                let b = med3(mi[x], mi[x + 1], mi[x + 2]);
                let c = hi[x].min(hi[x + 1]).min(hi[x + 2]);
                orow[x] = med3(a, b, c);
            }
        },
    );
    out
}

/// Constant-time median (Perreault & Hébert 2007) of an 8-bit plane over a (2r+1)² window
/// with replicated edges (r ≤ 127): per-column histograms slide down, the kernel histogram
/// slides right, and the 256 fine bins are synchronized lazily per 16-bin coarse bucket.
/// The image is cut into 512×128 tiles (histograms stay in cache) processed in parallel;
/// the result is exact, so it does not depend on the thread count.
pub fn median_u8(q: &[u8], w: usize, h: usize, r: usize) -> Vec<u8> {
    let mut out = vec![0u8; w * h];
    if r == 0 || w == 0 || h == 0 {
        out.copy_from_slice(q);
        return out;
    }
    const TW: usize = 512;
    const TH: usize = 128;
    let n = 2 * r + 1;
    let rank = (n * n / 2) as u16;
    let tiles: Vec<(usize, usize)> =
        (0..h).step_by(TH).flat_map(|y0| (0..w).step_by(TW).map(move |x0| (x0, y0))).collect();
    let po = SharedMut::new(&mut out);
    tiles.par_iter().for_each(|&(x0, y0)| {
        let tw = (x0 + TW).min(w) - x0;
        let y1 = (y0 + TH).min(h);
        let pw = tw + 2 * r;
        let col: Vec<usize> =
            (0..pw).map(|j| (x0 as i64 + j as i64 - r as i64).clamp(0, w as i64 - 1) as usize).collect();
        let cy = |y: i64| y.clamp(0, h as i64 - 1) as usize;
        let mut fine = vec![0u16; pw * 256];
        let mut coarse = vec![0u16; pw * 16];
        for dy in -(r as i64)..=r as i64 {
            let row = &q[cy(y0 as i64 + dy) * w..][..w];
            for (j, &cx) in col.iter().enumerate() {
                let v = row[cx] as usize;
                fine[j * 256 + v] += 1;
                coarse[j * 16 + (v >> 4)] += 1;
            }
        }
        let mut orow = vec![0u8; tw];
        for y in y0..y1 {
            if y > y0 {
                let rem = &q[cy(y as i64 - r as i64 - 1) * w..][..w];
                let add = &q[cy(y as i64 + r as i64) * w..][..w];
                for (j, &cx) in col.iter().enumerate() {
                    let (a, b) = (rem[cx] as usize, add[cx] as usize);
                    if a != b {
                        fine[j * 256 + a] -= 1;
                        coarse[j * 16 + (a >> 4)] -= 1;
                        fine[j * 256 + b] += 1;
                        coarse[j * 16 + (b >> 4)] += 1;
                    }
                }
            }
            let mut hc = [0u16; 16];
            let mut hf = [0u16; 256];
            let mut last = [usize::MAX; 16];
            for j in 0..n {
                for b in 0..16 {
                    hc[b] = hc[b].wrapping_add(coarse[j * 16 + b]);
                }
            }
            for (x, o) in orow.iter_mut().enumerate() {
                if x > 0 {
                    let (ai, ri) = ((x + 2 * r) * 16, (x - 1) * 16);
                    for b in 0..16 {
                        hc[b] = hc[b].wrapping_add(coarse[ai + b]).wrapping_sub(coarse[ri + b]);
                    }
                }
                let mut cum = 0u16;
                let mut cb = 0usize;
                while cb < 15 && cum + hc[cb] <= rank {
                    cum += hc[cb];
                    cb += 1;
                }
                let seg = cb * 16;
                let hs = &mut hf[seg..seg + 16];
                if last[cb] == usize::MAX || x - last[cb] > n {
                    hs.fill(0);
                    for j in x..x + n {
                        let f = &fine[j * 256 + seg..j * 256 + seg + 16];
                        for b in 0..16 {
                            hs[b] = hs[b].wrapping_add(f[b]);
                        }
                    }
                } else {
                    for xx in last[cb] + 1..=x {
                        let fa = &fine[(xx + 2 * r) * 256 + seg..][..16];
                        let fr = &fine[(xx - 1) * 256 + seg..][..16];
                        for b in 0..16 {
                            hs[b] = hs[b].wrapping_add(fa[b]).wrapping_sub(fr[b]);
                        }
                    }
                }
                last[cb] = x;
                let mut fb = 0usize;
                while fb < 15 && cum + hs[fb] <= rank {
                    cum += hs[fb];
                    fb += 1;
                }
                *o = (seg + fb) as u8;
            }
            // SAFETY: tiles cover disjoint pixels.
            unsafe { po.write(y * w + x0, &orow) };
        }
    });
    out
}

/// Median of every color channel (and alpha when the image has transparency) over a
/// (2r+1)² window. r = 1 is exact in f32; larger radii rank at 8-bit precision, and a
/// pixel whose own value falls in the median bin keeps its exact value.
pub fn median_image(img: &Image, r: usize) -> Image {
    if r == 0 || img.is_empty() {
        return img.clone();
    }
    let (w, h) = (img.width as usize, img.height as usize);
    let channels = if img.is_opaque() { 3 } else { 4 };
    let mut out = img.clone();
    for ch in 0..channels {
        if r == 1 {
            let plane: Vec<f32> = img.data.par_chunks(4).map(|p| p[ch]).collect();
            let m = median3x3(&plane, w, h);
            out.data.par_chunks_mut(4).zip(m.par_iter()).for_each(|(p, &v)| p[ch] = v);
            continue;
        }
        let q: Vec<u8> = img.data.par_chunks(4).map(|p| color::to_u8(p[ch])).collect();
        let m = median_u8(&q, w, h, r);
        out.data.par_chunks_mut(4).zip(q.par_iter().zip(m.par_iter())).for_each(|(p, (&qv, &mv))| {
            if qv != mv {
                p[ch] = mv as f32 / 255.0;
            }
        });
    }
    out
}

// ---------------------------------------------------------------------------------------
// non-local means

/// Search window radius of [`nlm`] (5×5 candidates).
pub const NLM_SEARCH: i64 = 2;

/// Non-local means on premultiplied RGBA: every pixel becomes the weighted mean of the
/// pixels in its 5×5 search window, weighted by `exp(-d/h²)` where `d` is the mean squared
/// RGB difference of their 3×3 patches. Mirrored offsets share their patch distances, so
/// the cost is 12 offset pairs of two O(pixels) passes each.
pub fn nlm(p: &Buf, h_param: f32) -> Buf {
    let (w, h) = (p.w, p.h);
    if w == 0 || h == 0 || h_param <= 0.0 {
        return p.clone();
    }
    let n = w * h;
    let planes: Vec<Vec<f32>> = (0..4).map(|c| p.data.par_chunks(4).map(|px| px[c]).collect()).collect();
    // weight = 2^(-ds·k) with ds the 3×3 sum of squared RGB differences
    let k = std::f32::consts::LOG2_E / (h_param * h_param * 9.0 * 3.0);
    let (wi, hi) = (w as i64, h as i64);
    let cy = move |y: i64| y.clamp(0, hi - 1) as usize;
    // accumulator rows: acc r, g, b, a, weight sum, weight max
    let mut acc = vec![0f32; 6 * n];
    let mut hsum = vec![0f32; n];
    let mut offsets = Vec::new();
    for oy in -NLM_SEARCH..=NLM_SEARCH {
        for ox in -NLM_SEARCH..=NLM_SEARCH {
            if oy > 0 || (oy == 0 && ox > 0) {
                offsets.push((ox, oy));
            }
        }
    }
    // interior column range where x + d stays inside the row
    let span = move |d: i64| -> (usize, usize) { ((-d).max(0).min(wi) as usize, (wi - d.max(0)).max(0) as usize) };
    for &(ox, oy) in &offsets {
        // pass 1: horizontal 3-sums of the squared difference to the offset pixel
        hsum.par_chunks_mut(w).enumerate().for_each_init(
            || vec![0f32; w],
            |d, (y, row)| {
                let y2 = cy(y as i64 + oy);
                d.fill(0.0);
                let (lo, hi_) = span(ox);
                for pl in planes.iter().take(3) {
                    let a = &pl[y * w..(y + 1) * w];
                    let b = &pl[y2 * w..(y2 + 1) * w];
                    if lo < hi_ {
                        let bs = &b[(lo as i64 + ox) as usize..(hi_ as i64 + ox) as usize];
                        for ((dv, &av), &bv) in d[lo..hi_].iter_mut().zip(&a[lo..hi_]).zip(bs) {
                            let t = av - bv;
                            *dv += t * t;
                        }
                    }
                    for x in (0..lo).chain(hi_..w) {
                        let t = a[x] - b[(x as i64 + ox).clamp(0, wi - 1) as usize];
                        d[x] += t * t;
                    }
                }
                if w == 1 {
                    row[0] = 3.0 * d[0];
                    return;
                }
                row[0] = 2.0 * d[0] + d[1];
                row[w - 1] = 2.0 * d[w - 1] + d[w - 2];
                for (((o, &a), &b), &c) in row[1..w - 1].iter_mut().zip(&d[..w - 2]).zip(&d[1..w - 1]).zip(&d[2..]) {
                    *o = a + b + c;
                }
            },
        );
        // pass 2: weights for p→p+o and the mirror p→p−o, accumulated per row
        let hs = &hsum;
        acc.par_chunks_mut(6 * w).enumerate().for_each_init(
            || (vec![0f32; w], vec![0f32; w]),
            |(w1, w2), (y, arow)| {
                let (r0, r1, r2) = (cy(y as i64 - 1), y, cy(y as i64 + 1));
                for (x, o) in w1.iter_mut().enumerate() {
                    *o = util::exp2_neg((hs[r0 * w + x] + hs[r1 * w + x] + hs[r2 * w + x]) * k);
                }
                let ym = y as i64 - oy;
                let mirror = ym >= 0 && ym < hi;
                let (mlo, mhi) = span(-ox);
                if mirror {
                    let ym = ym as usize;
                    let (q0, q1, q2) = (cy(ym as i64 - 1), ym, cy(ym as i64 + 1));
                    w2.fill(0.0);
                    for x in mlo..mhi {
                        let xs = (x as i64 - ox) as usize;
                        w2[x] = util::exp2_neg((hs[q0 * w + xs] + hs[q1 * w + xs] + hs[q2 * w + xs]) * k);
                    }
                }
                let y2 = cy(y as i64 + oy);
                let (accs, rest) = arow.split_at_mut(4 * w);
                let (wsum, wmax) = rest.split_at_mut(w);
                let (flo, fhi) = span(ox);
                for c in 0..4 {
                    let a = &mut accs[c * w..(c + 1) * w];
                    let src = &planes[c][y2 * w..(y2 + 1) * w];
                    for x in 0..w {
                        let sx = if x >= flo && x < fhi {
                            (x as i64 + ox) as usize
                        } else {
                            (x as i64 + ox).clamp(0, wi - 1) as usize
                        };
                        a[x] += w1[x] * src[sx];
                    }
                    if mirror {
                        let ms = &planes[c][(ym as usize) * w..(ym as usize + 1) * w];
                        for x in mlo..mhi {
                            a[x] += w2[x] * ms[(x as i64 - ox) as usize];
                        }
                    }
                }
                for x in 0..w {
                    wsum[x] += w1[x];
                    wmax[x] = wmax[x].max(w1[x]);
                }
                if mirror {
                    for x in mlo..mhi {
                        wsum[x] += w2[x];
                        wmax[x] = wmax[x].max(w2[x]);
                    }
                }
            },
        );
    }
    let mut out = Buf::new(w, h, 4);
    out.data.par_chunks_mut(4 * w).zip(acc.par_chunks(6 * w)).enumerate().for_each(|(y, (orow, arow))| {
        for x in 0..w {
            let wm = arow[5 * w + x];
            let self_w = if wm > 0.0 { wm } else { 1.0 };
            let tw = arow[4 * w + x] + self_w;
            for c in 0..4 {
                orow[x * 4 + c] = (arow[c * w + x] + self_w * planes[c][y * w + x]) / tw;
            }
        }
    });
    out
}

// ---------------------------------------------------------------------------------------
// surface blur (bilateral with a box spatial kernel)

/// Largest surface-blur radius computed by brute force (≤ 121 taps).
const DIRECT_MAX_R: usize = 5;

/// Bilateral filter of the color channels: average of the (2r+1)² box weighted by
/// alpha × tent(|Δ| / hw). Brute force for r ≤ 5, grid-accelerated above.
pub fn surface_blur(img: &Image, r: usize, hw: f32) -> (Image, usize) {
    if img.is_empty() || r == 0 || hw <= 0.0 {
        return (img.clone(), 0);
    }
    if r <= DIRECT_MAX_R {
        (bilateral_direct(img, r, hw), 0)
    } else {
        bilateral_levels(img, r, hw)
    }
}

fn bilateral_direct(img: &Image, r: usize, hw: f32) -> Image {
    let (w, h) = (img.width as usize, img.height as usize);
    let inv = 1.0 / hw;
    let alpha: Vec<f32> = img.data.par_chunks(4).map(|p| p[3]).collect();
    let mut out = img.clone();
    for ch in 0..3 {
        let plane: Vec<f32> = img.data.par_chunks(4).map(|p| p[ch]).collect();
        let mut res = vec![0f32; w * h];
        res.par_chunks_mut(w).enumerate().for_each_init(
            || (vec![0f32; w], vec![0f32; w], Vec::new(), Vec::new()),
            |(sa, sb, pv, pa), (y, orow)| {
                let vp = &plane[y * w..(y + 1) * w];
                sa.fill(0.0);
                sb.fill(0.0);
                for dy in -(r as i64)..=r as i64 {
                    let sy = (y as i64 + dy).clamp(0, h as i64 - 1) as usize;
                    pad_row(&plane[sy * w..(sy + 1) * w], 1, r, Edge::Clamp, pv);
                    pad_row(&alpha[sy * w..(sy + 1) * w], 1, r, Edge::Clamp, pa);
                    for dx in 0..=2 * r {
                        let (vq, aq) = (&pv[dx..dx + w], &pa[dx..dx + w]);
                        for ((((a, b), &q), &al), &p) in sa.iter_mut().zip(sb.iter_mut()).zip(vq).zip(aq).zip(vp) {
                            let t = (1.0 - (q - p).abs() * inv).max(0.0) * al;
                            *a += t;
                            *b += t * q;
                        }
                    }
                }
                for (x, o) in orow.iter_mut().enumerate() {
                    *o = if sa[x] > 1e-6 { sb[x] / sa[x] } else { vp[x] };
                }
            },
        );
        out.data.par_chunks_mut(4).zip(res.par_iter()).for_each(|(p, &v)| p[ch] = v.clamp(0.0, 1.0));
    }
    out
}

/// Piecewise-linear bilateral filter (Durand & Dorsey 2002): the range is sampled at
/// ≤ 32 levels; for each level the tent-weighted sums are box-blurred on a grid
/// downsampled by max(3, r/3), and every pixel interpolates the two levels around its own
/// value. Levels are processed in groups (bounded memory) so each group reads the image
/// once; all three channels share every pass.
fn bilateral_levels(img: &Image, r: usize, hw: f32) -> (Image, usize) {
    let (w, h) = (img.width as usize, img.height as usize);
    let levels = ((2.0 / hw).ceil() as usize + 1).clamp(2, 32);
    let delta = 1.0 / (levels - 1) as f32;
    let f = (r / 3).max(3);
    let rl = ((r as f32 / f as f32).round() as usize).max(1);
    let (lw, lh) = (w.div_ceil(f), h.div_ceil(f));
    let mut used = vec![false; levels];
    for p in img.data.chunks(4) {
        for &v in &p[..3] {
            let u = v.clamp(0.0, 1.0) / delta;
            used[(u.floor() as usize).min(levels - 1)] = true;
            used[(u.ceil() as usize).min(levels - 1)] = true;
        }
    }
    let used: Vec<usize> = (0..levels).filter(|&k| used[k]).collect();
    // ≤ 48M floats of grid per group
    let group = (48_000_000 / (lw * lh * 6).max(1)).clamp(1, 8);
    let mut acc = vec![0f32; w * h * 3];
    let ff = f as f32;
    let mut slot = vec![usize::MAX; levels];
    for chunk in used.chunks(group) {
        let g = chunk.len();
        slot.fill(usize::MAX);
        for (gi, &k) in chunk.iter().enumerate() {
            slot[k] = gi;
        }
        let gc = 6 * g;
        let mut low = Buf::new(lw, lh, gc);
        low.data.par_chunks_mut(lw * gc).enumerate().for_each(|(oy, row)| {
            let y0 = oy * f;
            let y1 = (y0 + f).min(h);
            for y in y0..y1 {
                let src = &img.data[y * w * 4..(y + 1) * w * 4];
                for (x, p) in src.chunks(4).enumerate() {
                    let o = &mut row[(x / f) * gc..(x / f + 1) * gc];
                    for c in 0..3 {
                        let v = p[c].clamp(0.0, 1.0);
                        // levels whose tent reaches v
                        let k0 = (((v - hw) / delta).ceil().max(0.0)) as usize;
                        let k1 = (((v + hw) / delta).floor() as usize).min(levels - 1);
                        for k in k0..=k1 {
                            let gi = slot[k];
                            if gi == usize::MAX {
                                continue;
                            }
                            let t = p[3] * (1.0 - (v - k as f32 * delta).abs() / hw).max(0.0);
                            o[gi * 6 + 2 * c] += t;
                            o[gi * 6 + 2 * c + 1] += t * v;
                        }
                    }
                }
            }
        });
        box_blur(&mut low, rl, rl, Edge::Clamp);
        let gj = 3 * g;
        let mut j = vec![0f32; lw * lh * gj];
        j.par_chunks_mut(gj).zip(low.data.par_chunks(gc)).for_each(|(o, s)| {
            for (gi, &k) in chunk.iter().enumerate() {
                for c in 0..3 {
                    let (a, b) = (s[gi * 6 + 2 * c], s[gi * 6 + 2 * c + 1]);
                    o[gi * 3 + c] = if a > 1e-9 { b / a } else { k as f32 * delta };
                }
            }
        });
        acc.par_chunks_mut(w * 3).enumerate().for_each(|(y, arow)| {
            let sy = ((y as f32 + 0.5) / ff - 0.5).clamp(0.0, (lh - 1) as f32);
            let (y0, ty) = (sy.floor() as usize, sy - sy.floor());
            let y1 = (y0 + 1).min(lh - 1);
            for x in 0..w {
                let p = &img.data[(y * w + x) * 4..][..3];
                let sx = ((x as f32 + 0.5) / ff - 0.5).clamp(0.0, (lw - 1) as f32);
                let (x0, tx) = (sx.floor() as usize, sx - sx.floor());
                let x1 = (x0 + 1).min(lw - 1);
                let corners = [
                    (y0 * lw + x0, (1.0 - tx) * (1.0 - ty)),
                    (y0 * lw + x1, tx * (1.0 - ty)),
                    (y1 * lw + x0, (1.0 - tx) * ty),
                    (y1 * lw + x1, tx * ty),
                ];
                for c in 0..3 {
                    let u = p[c].clamp(0.0, 1.0) / delta;
                    let k0 = (u.floor() as usize).min(levels - 1);
                    let fr = u - k0 as f32;
                    for (k, hat) in [(k0, 1.0 - fr), (k0 + 1, fr)] {
                        if hat <= 0.0 || k >= levels || slot[k] == usize::MAX {
                            continue;
                        }
                        let gi = slot[k];
                        let jv: f32 = corners.iter().map(|&(ci, wt)| wt * j[ci * gj + gi * 3 + c]).sum();
                        arow[x * 3 + c] += hat * jv;
                    }
                }
            }
        });
    }
    let mut out = img.clone();
    out.data.par_chunks_mut(4).zip(acc.par_chunks(3)).for_each(|(o, a)| {
        for c in 0..3 {
            o[c] = a[c].clamp(0.0, 1.0);
        }
    });
    (out, levels)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn noise_buf(w: usize, h: usize, c: usize) -> Buf {
        let mut b = Buf::new(w, h, c);
        let mut s = 12345u32;
        for v in b.data.iter_mut() {
            s = s.wrapping_mul(1_103_515_245).wrapping_add(12345);
            *v = (s >> 16) as f32 / 65536.0;
        }
        b
    }

    /// Reference box filter: average over the clamped (or zero) window.
    fn box_ref(b: &Buf, rx: usize, ry: usize, edge: Edge) -> Buf {
        let mut o = Buf::new(b.w, b.h, b.c);
        for y in 0..b.h as i64 {
            for x in 0..b.w as i64 {
                for ch in 0..b.c {
                    let mut s = 0.0;
                    for dy in -(ry as i64)..=ry as i64 {
                        for dx in -(rx as i64)..=rx as i64 {
                            let (sx, sy) = (x + dx, y + dy);
                            let inside = sx >= 0 && sy >= 0 && sx < b.w as i64 && sy < b.h as i64;
                            if !inside && edge == Edge::Zero {
                                continue;
                            }
                            let (sx, sy) = (sx.clamp(0, b.w as i64 - 1) as usize, sy.clamp(0, b.h as i64 - 1) as usize);
                            s += b.data[(sy * b.w + sx) * b.c + ch];
                        }
                    }
                    o.data[(y as usize * b.w + x as usize) * b.c + ch] = s / ((2 * rx + 1) * (2 * ry + 1)) as f32;
                }
            }
        }
        o
    }

    #[test]
    fn box_matches_reference() {
        for (w, h, rx, ry) in [(13, 9, 2, 3), (5, 40, 7, 1), (3, 3, 4, 4), (1, 1, 2, 2)] {
            for edge in [Edge::Clamp, Edge::Zero] {
                let b = noise_buf(w, h, 2);
                let mut o = b.clone();
                box_blur(&mut o, rx, ry, edge);
                let r = box_ref(&b, rx, ry, edge);
                for (a, b) in o.data.iter().zip(&r.data) {
                    assert!((a - b).abs() < 1e-5, "{w}x{h} r{rx},{ry} {edge:?}: {a} vs {b}");
                }
            }
        }
    }

    #[test]
    fn conv_matches_reference() {
        let b = noise_buf(17, 2100, 3);
        let k = gauss_kernel(1.3);
        for edge in [Edge::Clamp, Edge::Zero] {
            let mut o = b.clone();
            conv_rows(&mut o.data, b.w, 3, &k, edge);
            conv_cols(&mut o.data, b.w, b.h, 3, &k, edge);
            let r = k.len() as i64 / 2;
            for &(x, y) in &[(0i64, 0i64), (8, 1000), (16, 2099), (3, 2098)] {
                for ch in 0..3 {
                    let mut s = 0.0;
                    for (j, &kj) in k.iter().enumerate() {
                        for (i, &ki) in k.iter().enumerate() {
                            let (sx, sy) = (x + i as i64 - r, y + j as i64 - r);
                            let inside = sx >= 0 && sy >= 0 && sx < b.w as i64 && sy < b.h as i64;
                            if !inside && edge == Edge::Zero {
                                continue;
                            }
                            let (sx, sy) = (sx.clamp(0, b.w as i64 - 1) as usize, sy.clamp(0, b.h as i64 - 1) as usize);
                            s += kj * ki * b.data[(sy * b.w + sx) * 3 + ch];
                        }
                    }
                    let got = o.data[(y as usize * b.w + x as usize) * 3 + ch];
                    assert!((got - s).abs() < 1e-5, "{edge:?} at {x},{y}: {got} vs {s}");
                }
            }
        }
    }

    #[test]
    fn gaussian_keeps_flat_fields() {
        let mut b = Buf::new(40, 30, 4);
        for p in b.data.chunks_mut(4) {
            p.copy_from_slice(&[0.2, 0.4, 0.6, 1.0]);
        }
        for sigma in [0.7, 3.0, 12.0] {
            let mut o = b.clone();
            gaussian(&mut o, sigma, sigma, Edge::Clamp);
            assert!(o.data.chunks(4).all(|p| (p[1] - 0.4).abs() < 1e-5), "sigma {sigma}");
        }
    }

    #[test]
    fn median_matches_brute_force() {
        let (w, h) = (23, 140);
        let b = noise_buf(w, h, 1);
        let q: Vec<u8> = b.data.iter().map(|v| (v * 255.0) as u8).collect();
        let brute = |r: usize, x: usize, y: usize, f: &dyn Fn(usize, usize) -> f32| {
            let mut win = Vec::new();
            for dy in -(r as i64)..=r as i64 {
                for dx in -(r as i64)..=r as i64 {
                    let yy = (y as i64 + dy).clamp(0, h as i64 - 1) as usize;
                    let xx = (x as i64 + dx).clamp(0, w as i64 - 1) as usize;
                    win.push(f(xx, yy));
                }
            }
            win.sort_by(|a, b| a.partial_cmp(b).unwrap());
            win[win.len() / 2]
        };
        for r in [1usize, 2, 5] {
            let m = median_u8(&q, w, h, r);
            for y in 0..h {
                for x in 0..w {
                    assert_eq!(
                        m[y * w + x] as f32,
                        brute(r, x, y, &|xx, yy| q[yy * w + xx] as f32),
                        "r {r} at {x},{y}"
                    );
                }
            }
        }
        let m3 = median3x3(&b.data, w, h);
        for y in 0..h {
            for x in 0..w {
                assert_eq!(m3[y * w + x], brute(1, x, y, &|xx, yy| b.data[yy * w + xx]));
            }
        }
    }

    #[test]
    fn shape_blur_keeps_flat_fields() {
        let mut b = Buf::new(40, 30, 4);
        for p in b.data.chunks_mut(4) {
            p.copy_from_slice(&[0.2, 0.4, 0.6, 1.0]);
        }
        for shape in [Shape::Circle, Shape::Hex] {
            for r in [1.5f32, 5.0, 20.0] {
                let o = shape_blur(&b, r, shape, Edge::Clamp);
                for p in o.data.chunks(4) {
                    assert!((p[1] - 0.4).abs() < 1e-4, "{shape:?} r {r}: {}", p[1]);
                }
            }
        }
    }

    #[test]
    fn motion_blur_flat_and_spreads_a_dot() {
        let mut flat = Buf::new(30, 20, 4);
        for p in flat.data.chunks_mut(4) {
            p.copy_from_slice(&[0.5, 0.5, 0.5, 1.0]);
        }
        for angle in [0.0, 30.0, 45.0, 90.0, 120.0] {
            let o = motion_blur(&flat, angle, 9.0, Edge::Clamp);
            for p in o.data.chunks(4) {
                assert!((p[0] - 0.5).abs() < 1e-4, "angle {angle}: {}", p[0]);
            }
        }
        let mut dot = Buf::new(31, 31, 1);
        dot.data[15 * 31 + 15] = 1.0;
        let o = motion_blur(&dot, 0.0, 11.0, Edge::Zero);
        assert!(o.data[15 * 31 + 20] > 0.05, "spreads along x");
        assert!(o.data[20 * 31 + 15] < 1e-6, "not along y");
        let total: f32 = o.data.iter().sum();
        assert!((total - 1.0).abs() < 1e-3);
        let o = motion_blur(&dot, 37.0, 10.5, Edge::Zero);
        let total: f32 = o.data.iter().sum();
        assert!((total - 1.0).abs() < 1e-3, "slanted lines conserve energy: {total}");
    }

    #[test]
    fn nlm_reduces_noise() {
        let mut b = noise_buf(24, 24, 4);
        for p in b.data.chunks_mut(4) {
            p[0] = 0.5 + (p[0] - 0.5) * 0.1;
            p[1] = 0.5;
            p[2] = 0.5;
            p[3] = 1.0;
        }
        let var = |b: &Buf| {
            let m: f32 = b.data.chunks(4).map(|p| p[0]).sum::<f32>() / 576.0;
            b.data.chunks(4).map(|p| (p[0] - m).powi(2)).sum::<f32>() / 576.0
        };
        let o = nlm(&b, 0.1);
        assert!(var(&o) < var(&b) * 0.5);
        assert!(o.data.chunks(4).all(|p| (p[3] - 1.0).abs() < 1e-5));
    }

    #[test]
    fn surface_paths_agree() {
        let img = Image::from_fn(96, 64, |x, y| {
            let n = ((x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503)) % 31) as f32 / 31.0 * 0.06;
            let v = if x < 48 { 0.3 } else { 0.7 } + n;
            [v, 1.0 - v, 0.5, 1.0]
        });
        let hw = 2.5 * 20.0 / 255.0;
        let a = bilateral_direct(&img, 6, hw);
        let (b, levels) = bilateral_levels(&img, 6, hw);
        assert!(levels > 2);
        let mad: f32 = a.data.iter().zip(&b.data).map(|(x, y)| (x - y).abs()).sum::<f32>() / a.data.len() as f32;
        assert!(mad < 0.01, "mean abs difference {mad}");
        // the edge survives both
        assert!(b.get(44, 30)[0] < 0.4 && b.get(51, 30)[0] > 0.6);
    }

    #[test]
    fn shape_blur_conserves_a_dot() {
        for shape in [Shape::Circle, Shape::Hex] {
            for r in [3.0f32, 7.5] {
                let mut dot = Buf::new(41, 41, 1);
                dot.data[20 * 41 + 20] = 1.0;
                let o = shape_blur(&dot, r, shape, Edge::Zero);
                let total: f32 = o.data.iter().sum();
                assert!((total - 1.0).abs() < 1e-4, "{shape:?} r {r}: {total}");
                assert_eq!(o.data[20 * 41 + 20 + r.ceil() as usize + 2], 0.0, "{shape:?} r {r}: support");
            }
        }
    }

    #[test]
    fn min_max_window() {
        let mut p = vec![0.5f32; 10 * 10];
        p[55] = 1.0;
        p[11] = 0.0;
        let (mn, mx) = min_max(&p, 10, 10, 1);
        assert_eq!(mx[44], 1.0);
        assert_eq!(mx[33], 0.5);
        assert_eq!(mn[0], 0.0);
        assert_eq!(mn[33], 0.5);
    }
}
