//! Helpers shared by the filter, retouch, and fill commands: interleaved float buffers,
//! premultiplication, fast sRGB transfer functions, argument helpers, and bilinear
//! sampling. The heavy kernels live in `kern.rs`.

use pixelox_core::color;
use pixelox_core::kernels::Edge;
use pixelox_core::prelude::*;
use rayon::prelude::*;
use std::sync::OnceLock;

// ---------------------------------------------------------------------------------------
// buffers

/// An interleaved `f32` buffer with `c` channels per pixel. Holds premultiplied RGBA when
/// built with [`Buf::premul`].
#[derive(Clone, Debug)]
pub struct Buf {
    pub w: usize,
    pub h: usize,
    pub c: usize,
    pub data: Vec<f32>,
}

impl Buf {
    pub fn new(w: usize, h: usize, c: usize) -> Buf {
        Buf { w, h, c, data: vec![0.0; w * h * c] }
    }

    /// Premultiplied RGBA copy of an image.
    pub fn premul(img: &Image) -> Buf {
        let mut data = img.data.clone();
        data.par_chunks_mut(4).for_each(|p| {
            p[0] *= p[3];
            p[1] *= p[3];
            p[2] *= p[3];
        });
        Buf { w: img.width as usize, h: img.height as usize, c: 4, data }
    }

    /// Back to a straight-alpha image from the first four (premultiplied) channels,
    /// clamped to 0..1.
    pub fn unpremul(&self) -> Image {
        if self.c == 4 {
            return self.clone().into_unpremul();
        }
        let mut out = Image::new(self.w as u32, self.h as u32);
        out.data.par_chunks_mut(4).zip(self.data.par_chunks(self.c)).for_each(|(o, p)| {
            o.copy_from_slice(&p[..4]);
            unpremul_px(o);
        });
        out
    }

    /// [`Buf::unpremul`] in place for a four-channel buffer (no allocation).
    pub fn into_unpremul(mut self) -> Image {
        assert_eq!(self.c, 4, "into_unpremul needs RGBA");
        self.data.par_chunks_mut(4).for_each(unpremul_px);
        Image { width: self.w as u32, height: self.h as u32, data: self.data }
    }

    /// Rows become columns (blocked, parallel).
    pub fn transpose(&self) -> Buf {
        let (w, h, c) = (self.w, self.h, self.c);
        if self.data.is_empty() {
            return Buf { w: h, h: w, c, data: Vec::new() };
        }
        let mut out = vec![0f32; self.data.len()];
        const B: usize = 32;
        out.par_chunks_mut(h * c * B).enumerate().for_each(|(bx, chunk)| {
            let x0 = bx * B;
            let x1 = (x0 + B).min(w);
            for y0 in (0..h).step_by(B) {
                let y1 = (y0 + B).min(h);
                for x in x0..x1 {
                    for y in y0..y1 {
                        let s = (y * w + x) * c;
                        let d = ((x - x0) * h + y) * c;
                        chunk[d..d + c].copy_from_slice(&self.data[s..s + c]);
                    }
                }
            }
        });
        Buf { w: h, h: w, c, data: out }
    }
}

#[inline]
fn unpremul_px(o: &mut [f32]) {
    let a = o[3];
    if a > 1e-6 {
        let inv = 1.0 / a;
        for v in o.iter_mut().take(3) {
            *v = (*v * inv).clamp(0.0, 1.0);
        }
        o[3] = a.min(1.0);
    } else {
        o.fill(0.0);
    }
}

/// A raw pointer that parallel tasks use to access provably disjoint elements.
#[derive(Clone, Copy)]
pub(crate) struct SharedMut<T>(*mut T);

// SAFETY: only used where every task touches a disjoint set of indices (documented at
// each use), and the buffer outlives the parallel scope.
unsafe impl<T: Send> Send for SharedMut<T> {}
unsafe impl<T: Send> Sync for SharedMut<T> {}

impl<T: Copy> SharedMut<T> {
    pub(crate) fn new(v: &mut [T]) -> SharedMut<T> {
        SharedMut(v.as_mut_ptr())
    }
    /// # Safety
    /// `i..i + dst.len()` must be in bounds and not written concurrently.
    #[inline]
    pub(crate) unsafe fn read(self, i: usize, dst: &mut [T]) {
        std::ptr::copy_nonoverlapping(self.0.add(i), dst.as_mut_ptr(), dst.len());
    }
    /// # Safety
    /// `i..i + src.len()` must be in bounds and not accessed concurrently.
    #[inline]
    pub(crate) unsafe fn write(self, i: usize, src: &[T]) {
        std::ptr::copy_nonoverlapping(src.as_ptr(), self.0.add(i), src.len());
    }
}

impl SharedMut<f32> {
    /// # Safety
    /// `i` must be in bounds and not accessed concurrently.
    #[inline]
    pub(crate) unsafe fn add(self, i: usize, v: f32) {
        *self.0.add(i) += v;
    }
}

// ---------------------------------------------------------------------------------------
// color helpers

const LUT_N: usize = 16384;

fn lut(f: fn(f32) -> f32) -> Vec<f32> {
    (0..LUT_N + 1).map(|i| f((i as f32 / (LUT_N - 1) as f32).min(1.0))).collect()
}

fn to_lin_lut() -> &'static [f32] {
    static L: OnceLock<Vec<f32>> = OnceLock::new();
    L.get_or_init(|| lut(color::srgb_to_linear))
}

fn to_srgb_lut() -> &'static [f32] {
    static L: OnceLock<Vec<f32>> = OnceLock::new();
    L.get_or_init(|| lut(color::linear_to_srgb))
}

#[inline]
fn lut_eval(l: &[f32], v: f32) -> f32 {
    let t = v.clamp(0.0, 1.0) * (LUT_N - 1) as f32;
    let i = t as usize;
    let fr = t - i as f32;
    l[i] + (l[i + 1] - l[i]) * fr
}

/// sRGB transfer functions as 16K-entry tables (error < 2e-6), for per-pixel loops.
#[derive(Clone, Copy)]
pub struct Transfer {
    lin: &'static [f32],
    srgb: &'static [f32],
}

impl Transfer {
    pub fn get() -> Transfer {
        Transfer { lin: to_lin_lut(), srgb: to_srgb_lut() }
    }
    #[inline]
    pub fn to_linear(self, v: f32) -> f32 {
        lut_eval(self.lin, v)
    }
    #[inline]
    pub fn to_srgb(self, v: f32) -> f32 {
        lut_eval(self.srgb, v)
    }
}

/// Run `f` in linear light when `linear` is set (`--linear`).
pub fn maybe_linear(img: &Image, linear: bool, f: impl FnOnce(&Image) -> Result<Image>) -> Result<Image> {
    if !linear {
        return f(img);
    }
    let t = Transfer::get();
    let mut l = img.clone();
    l.map_pixels(|p| p.iter_mut().take(3).for_each(|v| *v = t.to_linear(*v)));
    let mut out = f(&l)?;
    out.map_pixels(|p| p.iter_mut().take(3).for_each(|v| *v = t.to_srgb(*v)));
    Ok(out)
}

/// `2^-t` for `t ≥ 0`, branch-free so loops over it vectorize (relative error < 2e-4).
#[inline(always)]
pub fn exp2_neg(t: f32) -> f32 {
    let t = t.clamp(0.0, 126.0);
    let ti = t as i32;
    let f = t - ti as f32;
    let p = 1.0
        + f * (-std::f32::consts::LN_2 + f * (0.240_226_5 + f * (-0.055_504_1 + f * (0.009_618_1 + f * -0.001_333_4))));
    p * f32::from_bits(((127 - ti) as u32) << 23)
}

#[inline]
pub fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Luma plane of a straight-alpha image.
pub fn luma_plane(img: &Image) -> Vec<f32> {
    img.data.par_chunks(4).map(|p| color::luma(p[0], p[1], p[2])).collect()
}

// ---------------------------------------------------------------------------------------
// argument helpers

/// Merge the keys of `extra` into the JSON object `v`.
pub fn merge(mut v: Value, extra: Value) -> Value {
    if let (Value::Object(m), Value::Object(e)) = (&mut v, extra) {
        for (k, x) in e {
            m.insert(k, x);
        }
    }
    v
}

/// A canvas point parameter resolved to canvas pixels.
pub fn canvas_point(a: &Args, name: &str, doc: &Document) -> Result<Option<(f64, f64)>> {
    Ok(a.point(name)?.map(|p| p.resolve(doc.width as f64, doc.height as f64, doc.dpi())))
}

/// The `(x, y)` offset of the `--layer` target (checks that it is unlocked).
pub fn layer_offset(a: &Args, doc: &Document) -> Result<(i32, i32)> {
    let p = target_layer(a, doc)?;
    let l = doc.layer(&p);
    Ok((l.x, l.y))
}

// ---------------------------------------------------------------------------------------
// sampling and resampling

/// Bilinear sample of `buf` at continuous coordinates where pixel centers are integers.
#[inline]
pub fn sample(buf: &Buf, x: f32, y: f32, edge: Edge, out: &mut [f32]) {
    let c = buf.c;
    out[..c].fill(0.0);
    if !x.is_finite() || !y.is_finite() {
        return;
    }
    let (w, h) = (buf.w as i64, buf.h as i64);
    let fx = x.floor();
    let fy = y.floor();
    let tx = x - fx;
    let ty = y - fy;
    let (x0, y0) = (fx as i64, fy as i64);
    for (dx, dy, wt) in
        [(0, 0, (1.0 - tx) * (1.0 - ty)), (1, 0, tx * (1.0 - ty)), (0, 1, (1.0 - tx) * ty), (1, 1, tx * ty)]
    {
        if wt <= 0.0 {
            continue;
        }
        let (mut sx, mut sy) = (x0 + dx, y0 + dy);
        if sx < 0 || sy < 0 || sx >= w || sy >= h {
            match edge {
                Edge::Zero => continue,
                Edge::Clamp => {
                    sx = sx.clamp(0, w - 1);
                    sy = sy.clamp(0, h - 1);
                }
            }
        }
        let i = (sy as usize * buf.w + sx as usize) * c;
        for ch in 0..c {
            out[ch] += buf.data[i + ch] * wt;
        }
    }
}

/// Bilinear sample of a four-channel buffer, with a fast path away from the edges.
#[inline(always)]
pub fn sample4(buf: &Buf, x: f32, y: f32, edge: Edge) -> [f32; 4] {
    let fx = x.floor();
    let fy = y.floor();
    let (x0, y0) = (fx as i64, fy as i64);
    if x0 >= 0 && y0 >= 0 && x0 + 1 < buf.w as i64 && y0 + 1 < buf.h as i64 {
        let (tx, ty) = (x - fx, y - fy);
        let i = (y0 as usize * buf.w + x0 as usize) * 4;
        let j = i + buf.w * 4;
        let (a, b) = (&buf.data[i..i + 8], &buf.data[j..j + 8]);
        let mut o = [0f32; 4];
        for c in 0..4 {
            let top = a[c] + (a[4 + c] - a[c]) * tx;
            let bot = b[c] + (b[4 + c] - b[c]) * tx;
            o[c] = top + (bot - top) * ty;
        }
        return o;
    }
    let mut o = [0f32; 4];
    sample(buf, x, y, edge, &mut o);
    o
}

/// Bilinear sample with wrap-around addressing (tiling).
#[inline]
pub fn sample_wrap(buf: &Buf, x: f32, y: f32, out: &mut [f32]) {
    let c = buf.c;
    out[..c].fill(0.0);
    if !x.is_finite() || !y.is_finite() || buf.w == 0 || buf.h == 0 {
        return;
    }
    let (w, h) = (buf.w as i64, buf.h as i64);
    let fx = x.floor();
    let fy = y.floor();
    let tx = x - fx;
    let ty = y - fy;
    let (x0, y0) = (fx as i64, fy as i64);
    for (dx, dy, wt) in
        [(0, 0, (1.0 - tx) * (1.0 - ty)), (1, 0, tx * (1.0 - ty)), (0, 1, (1.0 - tx) * ty), (1, 1, tx * ty)]
    {
        if wt <= 0.0 {
            continue;
        }
        let sx = (x0 + dx).rem_euclid(w) as usize;
        let sy = (y0 + dy).rem_euclid(h) as usize;
        let i = (sy * buf.w + sx) * c;
        for ch in 0..c {
            out[ch] += buf.data[i + ch] * wt;
        }
    }
}

/// Area-average downsample by an integer factor (partial blocks average what they hold).
pub fn downsample(src: &Buf, f: usize) -> Buf {
    let (nw, nh, c) = (src.w.div_ceil(f), src.h.div_ceil(f), src.c);
    let mut out = Buf::new(nw, nh, c);
    out.data.par_chunks_mut(nw * c).enumerate().for_each(|(oy, row)| {
        let y0 = oy * f;
        let y1 = (y0 + f).min(src.h);
        for ox in 0..nw {
            let x0 = ox * f;
            let x1 = (x0 + f).min(src.w);
            let o = &mut row[ox * c..ox * c + c];
            for y in y0..y1 {
                let s = &src.data[(y * src.w + x0) * c..(y * src.w + x1) * c];
                for p in s.chunks(c) {
                    for ch in 0..c {
                        o[ch] += p[ch];
                    }
                }
            }
            let n = ((y1 - y0) * (x1 - x0)) as f32;
            o.iter_mut().for_each(|v| *v /= n);
        }
    });
    out
}

/// Bilinear upsample of a buffer made by [`downsample`] with factor `f` back to `w`×`h`.
pub fn upsample(src: &Buf, w: usize, h: usize, f: usize) -> Buf {
    let c = src.c;
    let mut out = Buf::new(w, h, c);
    let ff = f as f32;
    out.data.par_chunks_mut(w * c).enumerate().for_each(|(y, row)| {
        let sy = (y as f32 + 0.5) / ff - 0.5;
        for x in 0..w {
            let sx = (x as f32 + 0.5) / ff - 0.5;
            sample(src, sx, sy, Edge::Clamp, &mut row[x * c..x * c + c]);
        }
    });
    out
}

// ---------------------------------------------------------------------------------------
// tests

#[cfg(test)]
pub(crate) mod test_io {
    //! A tiny image IO for tests: `PXT1` + width + height (u32 LE) + RGBA8 pixels.
    use pixelox_core::ctx::{EncodeOptions, Globals, ImageInfo, ImageIo, NoProgress, OpCtx, OpenOptions};
    use pixelox_core::envelope::Warning;
    use pixelox_core::{Args, Document, Image, OpSpec, Result, Runner};
    use serde_json::Value;
    use std::path::Path;

    pub struct StubIo;

    impl ImageIo for StubIo {
        fn open(&self, _: &Path, _: &OpenOptions) -> Result<(Document, Vec<Warning>)> {
            Err(pixelox_core::Error::unsupported("stub"))
        }
        fn decode(&self, bytes: &[u8], _: Option<&str>, _: &OpenOptions) -> Result<(Image, ImageInfo)> {
            let w = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
            let h = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
            let img = Image::from_rgba8(w, h, &bytes[12..]);
            Ok((img, ImageInfo { width: w, height: h, ..Default::default() }))
        }
        fn probe(&self, _: &Path) -> Result<ImageInfo> {
            Err(pixelox_core::Error::unsupported("stub"))
        }
        fn encode(&self, _: &Image, _: Option<&Document>, _: &Path, _: &EncodeOptions) -> Result<Vec<Warning>> {
            Err(pixelox_core::Error::unsupported("stub"))
        }
        fn save_pxo(&self, _: &Document, _: &Path) -> Result<()> {
            Err(pixelox_core::Error::unsupported("stub"))
        }
        fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>) {
            (vec![], vec![])
        }
    }

    pub fn encode(img: &Image) -> Vec<u8> {
        let mut v = b"PXT1".to_vec();
        v.extend(img.width.to_le_bytes());
        v.extend(img.height.to_le_bytes());
        v.extend(img.to_rgba8());
        v
    }

    /// Run a doc op with the stub IO (for ops that read auxiliary images).
    pub fn run_io(spec: &OpSpec, pairs: &[(&str, &str)], doc: &mut Document) -> Result<Value> {
        let a = Args::build(spec, pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect())?;
        let mut ctx = OpCtx::new(spec.name, vec![], Globals::default(), &StubIo, &NoProgress);
        match spec.runner {
            Runner::Doc(f) => f(&a, doc, &mut ctx),
            _ => panic!("not a doc op"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_tables_and_exp2() {
        let t = Transfer::get();
        for i in 0..=1000 {
            let v = i as f32 / 1000.0;
            assert!((t.to_linear(v) - color::srgb_to_linear(v)).abs() < 2e-6, "{v}");
            assert!((t.to_srgb(v) - color::linear_to_srgb(v)).abs() < 2e-6, "{v}");
        }
        for i in 0..=400 {
            let x = i as f32 / 10.0;
            let e = exp2_neg(x);
            assert!((e - (-x).exp2()).abs() <= 2e-4 * (-x).exp2() + 1e-30, "{x}: {e}");
        }
    }

    #[test]
    fn sample4_matches_sample() {
        let mut b = Buf::new(5, 4, 4);
        for (i, v) in b.data.iter_mut().enumerate() {
            *v = (i % 7) as f32 / 7.0;
        }
        for &(x, y) in &[(1.3f32, 2.6f32), (0.0, 0.0), (3.99, 2.01), (-0.5, 1.0), (4.5, 3.5)] {
            let mut o = [0f32; 4];
            sample(&b, x, y, Edge::Clamp, &mut o);
            let f = sample4(&b, x, y, Edge::Clamp);
            for c in 0..4 {
                assert!((o[c] - f[c]).abs() < 1e-6);
            }
        }
    }
}
