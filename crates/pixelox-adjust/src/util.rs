//! Small helpers shared by the adjustments and the analyzers: row-parallel pixel maps,
//! fixed-order reductions, integer histograms, and parameter parsing.

use pixelox_core::prelude::*;
use rayon::prelude::*;

/// Histogram resolution used for automatic corrections (12 bits: finer than 8-bit input,
/// close enough for 16-bit input).
pub const HBINS: usize = 4096;

#[inline]
pub fn clamp01(v: f32) -> f32 {
    if v.is_nan() {
        0.0
    } else {
        v.clamp(0.0, 1.0)
    }
}

/// Apply `f` to the RGB of every pixel (alpha untouched, results clamped to 0..1), in
/// parallel by rows.
pub fn map_rgb(img: &mut Image, f: impl Fn([f32; 3]) -> [f32; 3] + Sync) {
    if img.is_empty() {
        return;
    }
    let w = img.width as usize;
    img.data.par_chunks_mut(w * 4).for_each(|row| {
        for p in row.chunks_exact_mut(4) {
            let o = f([p[0], p[1], p[2]]);
            p[0] = clamp01(o[0]);
            p[1] = clamp01(o[1]);
            p[2] = clamp01(o[2]);
        }
    });
}

/// Like [`map_rgb`], with the pixel index (for ops that read a per-pixel side buffer).
pub fn map_rgb_indexed(img: &mut Image, f: impl Fn(usize, [f32; 3]) -> [f32; 3] + Sync) {
    if img.is_empty() {
        return;
    }
    let w = img.width as usize;
    img.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for (x, p) in row.chunks_exact_mut(4).enumerate() {
            let o = f(y * w + x, [p[0], p[1], p[2]]);
            p[0] = clamp01(o[0]);
            p[1] = clamp01(o[1]);
            p[2] = clamp01(o[2]);
        }
    });
}

/// One value per row, in row order. Reduce the result sequentially: float sums then do
/// not depend on how rayon split the work (PRD §6.6).
pub fn per_row<T: Send>(img: &Image, f: impl Fn(usize, &[f32]) -> T + Sync + Send) -> Vec<T> {
    if img.is_empty() {
        return Vec::new();
    }
    let w = img.width as usize;
    img.data.par_chunks(w * 4).enumerate().map(|(y, row)| f(y, row)).collect()
}

/// Sum per-row `[f64; N]` partials in row order.
pub fn sum_rows<const N: usize>(rows: &[[f64; N]]) -> [f64; N] {
    let mut acc = [0.0; N];
    for r in rows {
        for i in 0..N {
            acc[i] += r[i];
        }
    }
    acc
}

#[inline]
pub fn bin_of(v: f32, bins: usize) -> usize {
    ((clamp01(v) * bins as f32) as usize).min(bins - 1)
}

/// Integer histograms of R, G, B, and Rec. 709 luma over pixels with alpha > 0. Integer
/// counts merge exactly, so the parallel split never changes the result.
pub fn histograms(img: &Image, bins: usize) -> [Vec<u64>; 4] {
    let empty = || [vec![0u64; bins], vec![0u64; bins], vec![0u64; bins], vec![0u64; bins]];
    if img.is_empty() {
        return empty();
    }
    let w = img.width as usize;
    img.data
        .par_chunks(w * 4)
        .fold(empty, |mut h, row| {
            for p in row.chunks_exact(4) {
                if p[3] <= 0.0 {
                    continue;
                }
                h[0][bin_of(p[0], bins)] += 1;
                h[1][bin_of(p[1], bins)] += 1;
                h[2][bin_of(p[2], bins)] += 1;
                h[3][bin_of(color::luma(p[0], p[1], p[2]), bins)] += 1;
            }
            h
        })
        .reduce(empty, |mut a, b| {
            for c in 0..4 {
                for (x, y) in a[c].iter_mut().zip(&b[c]) {
                    *x += y;
                }
            }
            a
        })
}

/// The value (bin center, 0..1) below which a fraction `q` of the counted pixels lie.
pub fn quantile(h: &[u64], q: f64) -> Option<f32> {
    let total: u64 = h.iter().sum();
    if total == 0 {
        return None;
    }
    let target = (q.clamp(0.0, 1.0) * total as f64).ceil().max(1.0) as u64;
    let mut acc = 0u64;
    for (i, &c) in h.iter().enumerate() {
        acc += c;
        if acc >= target {
            return Some((i as f32 + 0.5) / h.len() as f32);
        }
    }
    Some(1.0 - 0.5 / h.len() as f32)
}

pub fn round_to(v: f64, places: i32) -> f64 {
    let m = 10f64.powi(places);
    let r = (v * m).round() / m;
    if r == 0.0 {
        0.0
    } else {
        r
    }
}

/// Format a number compactly for params (`12.5`, `-3`, `0.333`).
pub fn num(v: f64, places: i32) -> String {
    let r = round_to(v, places);
    let s = format!("{r:.*}", places.max(0) as usize);
    if s.contains('.') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

/// Parse a comma list of exactly `n` numbers, each inside `lo..hi`.
pub fn numbers(a: &Args, name: &str, n: usize, lo: f64, hi: f64) -> Result<Vec<f64>> {
    let raw = a.str(name).unwrap_or_default();
    let parts = units::split_list(&raw);
    if parts.len() != n {
        return Err(Error::new("PX1010", format!("--{name} needs {n} comma-separated numbers, got '{raw}'"))
            .with_context("param", name)
            .with_context("value", &raw));
    }
    let mut out = Vec::with_capacity(n);
    for p in parts {
        let v: f64 = p.parse().ok().filter(|v: &f64| v.is_finite()).ok_or_else(|| {
            Error::new("PX1010", format!("--{name}: '{p}' is not a number")).with_context("param", name)
        })?;
        if v < lo || v > hi {
            return Err(Error::new("PX1013", format!("--{name}: {v} is outside {lo}..{hi}"))
                .with_context("param", name)
                .with_context("range", format!("{lo}..{hi}")));
        }
        out.push(v);
    }
    Ok(out)
}

/// Three numbers as `[f32; 3]` (see [`numbers`]).
pub fn triple(a: &Args, name: &str, lo: f64, hi: f64) -> Result<[f32; 3]> {
    let v = numbers(a, name, 3, lo, hi)?;
    Ok([v[0] as f32, v[1] as f32, v[2] as f32])
}

/// `--a` and `--b` given together.
pub fn conflict(a: &str, b: &str) -> Error {
    Error::new("PX1005", format!("--{a} and --{b} cannot be used together")).with_context("flags", [a, b])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_formatting() {
        assert_eq!(num(12.5, 2), "12.5");
        assert_eq!(num(-3.0, 2), "-3");
        assert_eq!(num(0.33333, 3), "0.333");
        assert_eq!(num(-0.0001, 2), "0");
    }

    #[test]
    fn quantiles_and_bins() {
        assert_eq!(bin_of(1.0, 256), 255);
        assert_eq!(bin_of(128.0 / 255.0, 256), 128);
        let mut h = vec![0u64; 4];
        h[1] = 10;
        h[3] = 10;
        assert_eq!(quantile(&h, 0.0), Some(0.375));
        assert_eq!(quantile(&h, 0.5), Some(0.375));
        assert_eq!(quantile(&h, 0.51), Some(0.875));
        assert_eq!(quantile(&[0, 0], 0.5), None);
    }
}
