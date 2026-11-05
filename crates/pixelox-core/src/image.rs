//! Pixel containers: [`Image`] (RGBA `f32`, straight alpha, sRGB-encoded) and [`Mask`]
//! (single-channel coverage 0..1), plus [`Rendered`] (an image placed on the canvas).

use crate::color::Rgba;
use crate::units::RectI;
use rayon::prelude::*;

/// An RGBA image: `f32` components in 0..1, sRGB-encoded, straight (un-premultiplied) alpha,
/// row-major, 4 floats per pixel.
#[derive(Clone, PartialEq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f32>,
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Image({}x{})", self.width, self.height)
    }
}

impl Image {
    /// A fully transparent image.
    pub fn new(width: u32, height: u32) -> Image {
        Image { width, height, data: vec![0.0; width as usize * height as usize * 4] }
    }

    pub fn filled(width: u32, height: u32, c: Rgba) -> Image {
        let mut data = Vec::with_capacity(width as usize * height as usize * 4);
        for _ in 0..(width as usize * height as usize) {
            data.extend_from_slice(&[c.r, c.g, c.b, c.a]);
        }
        Image { width, height, data }
    }

    pub fn from_fn(width: u32, height: u32, f: impl Fn(u32, u32) -> [f32; 4] + Sync) -> Image {
        let mut img = Image::new(width, height);
        img.data.par_chunks_mut(width as usize * 4).enumerate().for_each(|(y, row)| {
            for x in 0..width as usize {
                let p = f(x as u32, y as u32);
                row[x * 4..x * 4 + 4].copy_from_slice(&p);
            }
        });
        img
    }

    pub fn from_rgba8(width: u32, height: u32, bytes: &[u8]) -> Image {
        Image { width, height, data: bytes.par_iter().map(|&b| b as f32 / 255.0).collect() }
    }

    pub fn from_rgba16(width: u32, height: u32, words: &[u16]) -> Image {
        Image { width, height, data: words.par_iter().map(|&w| w as f32 / 65535.0).collect() }
    }

    pub fn to_rgba8(&self) -> Vec<u8> {
        self.data.par_iter().map(|&v| crate::color::to_u8(v)).collect()
    }

    pub fn to_rgba16(&self) -> Vec<u16> {
        self.data.par_iter().map(|&v| crate::color::to_u16(v)).collect()
    }

    pub fn pixels(&self) -> usize {
        self.width as usize * self.height as usize
    }

    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    #[inline]
    pub fn idx(&self, x: u32, y: u32) -> usize {
        (y as usize * self.width as usize + x as usize) * 4
    }

    #[inline]
    pub fn get(&self, x: u32, y: u32) -> [f32; 4] {
        let i = self.idx(x, y);
        [self.data[i], self.data[i + 1], self.data[i + 2], self.data[i + 3]]
    }

    #[inline]
    pub fn set(&mut self, x: u32, y: u32, p: [f32; 4]) {
        let i = self.idx(x, y);
        self.data[i..i + 4].copy_from_slice(&p);
    }

    /// Pixel with clamp-to-edge addressing.
    #[inline]
    pub fn get_clamped(&self, x: i64, y: i64) -> [f32; 4] {
        let x = x.clamp(0, self.width as i64 - 1) as u32;
        let y = y.clamp(0, self.height as i64 - 1) as u32;
        self.get(x, y)
    }

    /// Bilinear sample at continuous coordinates (pixel centers at +0.5), premultiplied
    /// internally so transparent neighbours do not bleed color. Outside: transparent.
    pub fn sample_bilinear(&self, fx: f64, fy: f64) -> [f32; 4] {
        let x = fx - 0.5;
        let y = fy - 0.5;
        let x0 = x.floor();
        let y0 = y.floor();
        let tx = (x - x0) as f32;
        let ty = (y - y0) as f32;
        let mut acc = [0f32; 4];
        for (dx, dy, w) in
            [(0, 0, (1.0 - tx) * (1.0 - ty)), (1, 0, tx * (1.0 - ty)), (0, 1, (1.0 - tx) * ty), (1, 1, tx * ty)]
        {
            if w == 0.0 {
                continue;
            }
            let px = x0 as i64 + dx;
            let py = y0 as i64 + dy;
            if px < 0 || py < 0 || px >= self.width as i64 || py >= self.height as i64 {
                continue;
            }
            let p = self.get(px as u32, py as u32);
            acc[0] += p[0] * p[3] * w;
            acc[1] += p[1] * p[3] * w;
            acc[2] += p[2] * p[3] * w;
            acc[3] += p[3] * w;
        }
        if acc[3] > 1e-6 {
            [acc[0] / acc[3], acc[1] / acc[3], acc[2] / acc[3], acc[3].min(1.0)]
        } else {
            [0.0; 4]
        }
    }

    /// Multiply color by alpha in place.
    pub fn premultiply(&mut self) {
        self.data.par_chunks_mut(4).for_each(|p| {
            p[0] *= p[3];
            p[1] *= p[3];
            p[2] *= p[3];
        });
    }

    /// Divide color by alpha in place (fully transparent pixels become 0).
    pub fn unpremultiply(&mut self) {
        self.data.par_chunks_mut(4).for_each(|p| {
            if p[3] > 1e-7 {
                let inv = 1.0 / p[3];
                p[0] = (p[0] * inv).min(1.0);
                p[1] = (p[1] * inv).min(1.0);
                p[2] = (p[2] * inv).min(1.0);
            } else {
                p[0] = 0.0;
                p[1] = 0.0;
                p[2] = 0.0;
            }
        });
    }

    /// Apply `f` to every pixel (`[r,g,b,a]`, straight alpha), in parallel.
    pub fn map_pixels(&mut self, f: impl Fn(&mut [f32]) + Sync + Send) {
        self.data.par_chunks_mut(4).for_each(f);
    }

    /// Clamp every component to 0..1; returns true when anything was out of range.
    pub fn clamp(&mut self) -> bool {
        let clipped = self.data.par_iter().any(|v| !(0.0..=1.0).contains(v) || v.is_nan());
        if clipped {
            self.data.par_iter_mut().for_each(|v| *v = if v.is_nan() { 0.0 } else { v.clamp(0.0, 1.0) });
        }
        clipped
    }

    /// A copy of a region (parts outside the image are transparent).
    pub fn crop(&self, r: RectI) -> Image {
        let mut out = Image::new(r.w, r.h);
        for y in 0..r.h {
            let sy = r.y + y as i32;
            if sy < 0 || sy >= self.height as i32 {
                continue;
            }
            for x in 0..r.w {
                let sx = r.x + x as i32;
                if sx < 0 || sx >= self.width as i32 {
                    continue;
                }
                out.set(x, y, self.get(sx as u32, sy as u32));
            }
        }
        out
    }

    /// Bounding box of pixels with alpha above `threshold` (None when fully transparent).
    pub fn alpha_bbox(&self, threshold: f32) -> Option<RectI> {
        let (w, h) = (self.width as usize, self.height as usize);
        let rows: Vec<Option<(usize, usize)>> = (0..h)
            .into_par_iter()
            .map(|y| {
                let row = &self.data[y * w * 4..(y + 1) * w * 4];
                let mut lo = None;
                let mut hi = 0;
                for x in 0..w {
                    if row[x * 4 + 3] > threshold {
                        if lo.is_none() {
                            lo = Some(x);
                        }
                        hi = x;
                    }
                }
                lo.map(|l| (l, hi))
            })
            .collect();
        let mut bbox: Option<(usize, usize, usize, usize)> = None;
        for (y, r) in rows.iter().enumerate() {
            if let Some((l, hgh)) = r {
                bbox = Some(match bbox {
                    None => (*l, y, *hgh, y),
                    Some((x0, y0, x1, _)) => (x0.min(*l), y0, x1.max(*hgh), y),
                });
            }
        }
        bbox.map(|(x0, y0, x1, y1)| RectI::new(x0 as i32, y0 as i32, (x1 - x0 + 1) as u32, (y1 - y0 + 1) as u32))
    }

    /// Alpha channel as a mask.
    pub fn alpha_mask(&self) -> Mask {
        Mask { width: self.width, height: self.height, data: self.data.par_chunks(4).map(|p| p[3]).collect() }
    }

    /// True when every pixel is fully opaque.
    pub fn is_opaque(&self) -> bool {
        self.data.par_chunks(4).all(|p| p[3] >= 0.999)
    }

    /// Flatten onto a solid background color (result is opaque).
    pub fn flatten_onto(&self, bg: Rgba) -> Image {
        let mut out = self.clone();
        out.map_pixels(|p| {
            let a = p[3];
            p[0] = p[0] * a + bg.r * (1.0 - a);
            p[1] = p[1] * a + bg.g * (1.0 - a);
            p[2] = p[2] * a + bg.b * (1.0 - a);
            p[3] = 1.0;
        });
        out
    }
}

/// A single-channel coverage map (0 = outside, 1 = fully inside), in canvas coordinates
/// unless stated otherwise. Used for selections, named masks, and layer masks.
#[derive(Clone, PartialEq)]
pub struct Mask {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f32>,
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mask({}x{})", self.width, self.height)
    }
}

impl Mask {
    pub fn new(width: u32, height: u32) -> Mask {
        Mask { width, height, data: vec![0.0; width as usize * height as usize] }
    }

    pub fn filled(width: u32, height: u32, v: f32) -> Mask {
        Mask { width, height, data: vec![v; width as usize * height as usize] }
    }

    pub fn from_fn(width: u32, height: u32, f: impl Fn(u32, u32) -> f32 + Sync) -> Mask {
        let mut m = Mask::new(width, height);
        m.data.par_chunks_mut(width.max(1) as usize).enumerate().for_each(|(y, row)| {
            for (x, v) in row.iter_mut().enumerate() {
                *v = f(x as u32, y as u32);
            }
        });
        m
    }

    pub fn from_gray8(width: u32, height: u32, bytes: &[u8]) -> Mask {
        Mask { width, height, data: bytes.iter().map(|&b| b as f32 / 255.0).collect() }
    }

    pub fn to_gray8(&self) -> Vec<u8> {
        self.data.iter().map(|&v| crate::color::to_u8(v)).collect()
    }

    pub fn to_gray16(&self) -> Vec<u16> {
        self.data.iter().map(|&v| crate::color::to_u16(v)).collect()
    }

    #[inline]
    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.data[y as usize * self.width as usize + x as usize]
    }

    #[inline]
    pub fn set(&mut self, x: u32, y: u32, v: f32) {
        let w = self.width as usize;
        self.data[y as usize * w + x as usize] = v;
    }

    /// Value at a signed coordinate; 0 outside.
    #[inline]
    pub fn at(&self, x: i64, y: i64) -> f32 {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            0.0
        } else {
            self.get(x as u32, y as u32)
        }
    }

    pub fn invert(&mut self) {
        self.data.par_iter_mut().for_each(|v| *v = 1.0 - *v);
    }

    pub fn inverted(&self) -> Mask {
        let mut m = self.clone();
        m.invert();
        m
    }

    /// Fraction of the canvas covered (sum of coverage / pixel count).
    pub fn coverage(&self) -> f64 {
        if self.data.is_empty() {
            return 0.0;
        }
        self.data.par_iter().map(|&v| v as f64).sum::<f64>() / self.data.len() as f64
    }

    pub fn is_empty(&self) -> bool {
        self.data.par_iter().all(|&v| v <= 1e-6)
    }

    /// Bounding box of coverage above `threshold`.
    pub fn bbox(&self, threshold: f32) -> Option<RectI> {
        let w = self.width as usize;
        let mut bbox: Option<(usize, usize, usize, usize)> = None;
        for y in 0..self.height as usize {
            let row = &self.data[y * w..(y + 1) * w];
            let first = row.iter().position(|&v| v > threshold);
            if let Some(l) = first {
                let r = row.iter().rposition(|&v| v > threshold).unwrap_or(l);
                bbox = Some(match bbox {
                    None => (l, y, r, y),
                    Some((x0, y0, x1, _)) => (x0.min(l), y0, x1.max(r), y),
                });
            }
        }
        bbox.map(|(x0, y0, x1, y1)| RectI::new(x0 as i32, y0 as i32, (x1 - x0 + 1) as u32, (y1 - y0 + 1) as u32))
    }

    /// Boolean combination used by selections (`--mode replace|add|subtract|intersect`).
    pub fn combine(&mut self, other: &Mask, mode: SelectMode) {
        match mode {
            SelectMode::Replace => self.data.clone_from(&other.data),
            SelectMode::Add => self.data.par_iter_mut().zip(&other.data).for_each(|(a, b)| *a = a.max(*b)),
            SelectMode::Subtract => {
                self.data.par_iter_mut().zip(&other.data).for_each(|(a, b)| *a = (*a * (1.0 - *b)).max(0.0))
            }
            SelectMode::Intersect => self.data.par_iter_mut().zip(&other.data).for_each(|(a, b)| *a = a.min(*b)),
        }
    }
}

/// How a new selection combines with the existing one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectMode {
    Replace,
    Add,
    Subtract,
    Intersect,
}

pub const SELECT_MODES: &[&str] = &["replace", "add", "subtract", "intersect"];

impl SelectMode {
    pub fn parse(s: &str) -> crate::Result<SelectMode> {
        Ok(match s {
            "replace" => SelectMode::Replace,
            "add" | "union" => SelectMode::Add,
            "subtract" => SelectMode::Subtract,
            "intersect" => SelectMode::Intersect,
            _ => {
                return Err(crate::Error::usage(format!(
                    "--mode must be replace, add, subtract, or intersect, got '{s}'"
                )))
            }
        })
    }
}

/// An image placed on the canvas: its top-left is at (`x`, `y`) in document pixels.
#[derive(Clone, Debug)]
pub struct Rendered {
    pub image: Image,
    pub x: i32,
    pub y: i32,
}

impl Rendered {
    pub fn rect(&self) -> RectI {
        RectI::new(self.x, self.y, self.image.width, self.image.height)
    }

    /// Expand to a canvas-sized image (content outside the canvas is dropped).
    pub fn to_canvas(&self, w: u32, h: u32) -> Image {
        if self.x == 0 && self.y == 0 && self.image.width == w && self.image.height == h {
            return self.image.clone();
        }
        let mut out = Image::new(w, h);
        let r = self.rect().intersect(&RectI::new(0, 0, w, h));
        for y in r.y..r.bottom() {
            let sy = (y - self.y) as usize;
            let src =
                &self.image.data[(sy * self.image.width as usize + (r.x - self.x) as usize) * 4..][..r.w as usize * 4];
            let dst_i = (y as usize * w as usize + r.x as usize) * 4;
            out.data[dst_i..dst_i + r.w as usize * 4].copy_from_slice(src);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bbox_and_crop() {
        let mut img = Image::new(10, 10);
        img.set(3, 4, [1.0, 0.0, 0.0, 1.0]);
        img.set(6, 7, [1.0, 0.0, 0.0, 0.5]);
        assert_eq!(img.alpha_bbox(0.0), Some(RectI::new(3, 4, 4, 4)));
        let c = img.crop(RectI::new(3, 4, 2, 2));
        assert_eq!(c.get(0, 0), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(Image::new(4, 4).alpha_bbox(0.0), None);
    }

    #[test]
    fn premultiply_round_trip() {
        let mut img = Image::filled(2, 2, Rgba::new(0.8, 0.4, 0.2, 0.5));
        let orig = img.clone();
        img.premultiply();
        img.unpremultiply();
        for (a, b) in img.data.iter().zip(&orig.data) {
            assert!((a - b).abs() < 1e-6);
        }
    }

    #[test]
    fn mask_boolean_ops() {
        let mut a = Mask::from_fn(4, 1, |x, _| if x < 2 { 1.0 } else { 0.0 });
        let b = Mask::from_fn(4, 1, |x, _| if x % 2 == 0 { 1.0 } else { 0.0 });
        let mut add = a.clone();
        add.combine(&b, SelectMode::Add);
        assert_eq!(add.data, vec![1.0, 1.0, 1.0, 0.0]);
        a.combine(&b, SelectMode::Intersect);
        assert_eq!(a.data, vec![1.0, 0.0, 0.0, 0.0]);
        let mut inv = b.clone();
        inv.invert();
        inv.invert();
        assert_eq!(inv, b);
    }

    #[test]
    fn rendered_to_canvas_clips() {
        let r = Rendered { image: Image::filled(4, 4, Rgba::WHITE), x: -2, y: 1 };
        let c = r.to_canvas(4, 4);
        assert_eq!(c.get(0, 1)[3], 1.0);
        assert_eq!(c.get(2, 1)[3], 0.0);
        assert_eq!(c.get(0, 0)[3], 0.0);
    }
}
