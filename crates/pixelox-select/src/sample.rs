//! Resampling for geometric transforms: point samplers for the four filters (premultiplied
//! so transparent neighbours never bleed), affine resampling of rasters and masks with
//! exact fast paths (lattice permutations, axis-aligned scales), and pre-shrinking so large
//! reductions do not alias.

use crate::util::Affine2;
use pixelox_core::kernels::{self, Filter};
use pixelox_core::prelude::*;
use std::borrow::Cow;

/// Largest raster a transform may produce (pixels); beyond this the result is almost
/// certainly a mistake (a near-degenerate perspective, a huge scale).
pub const MAX_PIXELS: u64 = 1 << 28;

pub fn check_size(w: f64, h: f64) -> Result<(u32, u32)> {
    if !(w.is_finite() && h.is_finite()) || w * h > MAX_PIXELS as f64 || w > 65535.0 || h > 65535.0 {
        return Err(Error::new(
            "PX1013",
            format!("the transformed layer would be {}x{} px, beyond the limit", w.round(), h.round()),
        )
        .with_hint("check the transform values; very large scales or nearly degenerate quads explode the bounds")
        .with_context("size", [w.round(), h.round()]));
    }
    Ok((w.max(0.0) as u32, h.max(0.0) as u32))
}

fn kernel(f: Filter, x: f64) -> f64 {
    let x = x.abs();
    match f {
        Filter::Nearest => {
            if x < 0.5 {
                1.0
            } else {
                0.0
            }
        }
        Filter::Bilinear => (1.0 - x).max(0.0),
        Filter::Bicubic => {
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

fn support(f: Filter) -> i64 {
    match f {
        Filter::Nearest | Filter::Bilinear => 1,
        Filter::Bicubic => 2,
        Filter::Lanczos3 => 3,
    }
}

/// Samples an image at continuous coordinates (pixel centers at +0.5). Outside the image
/// is transparent.
pub struct Sampler<'a> {
    src: &'a Image,
    pm: Option<Image>,
    filter: Filter,
}

impl<'a> Sampler<'a> {
    pub fn new(src: &'a Image, filter: Filter) -> Sampler<'a> {
        let pm = if filter == Filter::Nearest {
            None
        } else {
            let mut p = src.clone();
            p.premultiply();
            Some(p)
        };
        Sampler { src, pm, filter }
    }

    pub fn sample(&self, x: f64, y: f64) -> [f32; 4] {
        let (w, h) = (self.src.width as i64, self.src.height as i64);
        let Some(pm) = &self.pm else {
            let (px, py) = (x.floor() as i64, y.floor() as i64);
            if px < 0 || py < 0 || px >= w || py >= h {
                return [0.0; 4];
            }
            return self.src.get(px as u32, py as u32);
        };
        let s = support(self.filter);
        let (fx, fy) = (x - 0.5, y - 0.5);
        let (x0, y0) = (fx.floor() as i64, fy.floor() as i64);
        if x0 + s < 0 || y0 + s < 0 || x0 - s + 1 >= w || y0 - s + 1 >= h {
            return [0.0; 4];
        }
        let n = (2 * s) as usize;
        let mut wx = [0f64; 6];
        let mut wy = [0f64; 6];
        for i in 0..n {
            wx[i] = kernel(self.filter, fx - (x0 - s + 1 + i as i64) as f64);
            wy[i] = kernel(self.filter, fy - (y0 - s + 1 + i as i64) as f64);
        }
        let wsum: f64 = wx[..n].iter().sum::<f64>() * wy[..n].iter().sum::<f64>();
        if wsum.abs() < 1e-12 {
            return [0.0; 4];
        }
        let mut acc = [0f64; 4];
        for (j, &wyj) in wy[..n].iter().enumerate() {
            let py = y0 - s + 1 + j as i64;
            if wyj == 0.0 || py < 0 || py >= h {
                continue;
            }
            for (i, &wxi) in wx[..n].iter().enumerate() {
                let px = x0 - s + 1 + i as i64;
                if wxi == 0.0 || px < 0 || px >= w {
                    continue;
                }
                let wt = wxi * wyj;
                let p = pm.get(px as u32, py as u32);
                for c in 0..4 {
                    acc[c] += p[c] as f64 * wt;
                }
            }
        }
        let mut a = (acc[3] / wsum).clamp(0.0, 1.0) as f32;
        if a > 0.99999 {
            a = 1.0;
        } else if a < 1e-5 {
            return [0.0; 4];
        }
        let inv = 1.0 / (wsum * a as f64);
        [
            (acc[0] * inv).clamp(0.0, 1.0) as f32,
            (acc[1] * inv).clamp(0.0, 1.0) as f32,
            (acc[2] * inv).clamp(0.0, 1.0) as f32,
            a,
        ]
    }
}

/// Bilinear mask sample with `fill` outside the mask.
pub fn mask_bilinear(m: &Mask, x: f64, y: f64, fill: f32) -> f32 {
    let (fx, fy) = (x - 0.5, y - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (tx, ty) = ((fx - x0) as f32, (fy - y0) as f32);
    let at = |px: i64, py: i64| -> f32 {
        if px < 0 || py < 0 || px >= m.width as i64 || py >= m.height as i64 {
            fill
        } else {
            m.get(px as u32, py as u32)
        }
    };
    let (ix, iy) = (x0 as i64, y0 as i64);
    let top = at(ix, iy) * (1.0 - tx) + at(ix + 1, iy) * tx;
    let bot = at(ix, iy + 1) * (1.0 - tx) + at(ix + 1, iy + 1) * tx;
    top * (1.0 - ty) + bot * ty
}

/// The value a layer mask has beyond its edges: 1 when the whole border is revealed
/// (a reveal-all style mask), else 0 — so moving content in does not hide it.
pub fn border_fill(m: &Mask) -> f32 {
    let (w, h) = (m.width, m.height);
    if w == 0 || h == 0 {
        return 0.0;
    }
    let row_ok = |y: u32| (0..w).all(|x| m.get(x, y) >= 0.999);
    let col_ok = |x: u32| (0..h).all(|y| m.get(x, y) >= 0.999);
    if row_ok(0) && row_ok(h - 1) && col_ok(0) && col_ok(w - 1) {
        1.0
    } else {
        0.0
    }
}

/// Transform a raster placed at (`lx`, `ly`) by `m` (canvas → canvas). Returns the new
/// pixels and their canvas offset. Lattice maps are exact; axis-aligned scales use the
/// separable resampler; anything else samples through the inverse map, pre-shrinking the
/// source when the transform reduces it.
pub fn affine_raster(img: &Image, lx: i32, ly: i32, m: &Affine2, filter: Option<Filter>) -> Result<(Image, i32, i32)> {
    if img.is_empty() || m.is_identity() {
        return Ok((img.clone(), lx, ly));
    }
    let (w, h) = (img.width as f64, img.height as f64);
    let (lxf, lyf) = (lx as f64, ly as f64);
    if m.is_translation() && (m.e - m.e.round()).abs() < 1e-9 && (m.f - m.f.round()).abs() < 1e-9 {
        return Ok((img.clone(), lx + m.e.round() as i32, ly + m.f.round() as i32));
    }
    let inv = m.inverse().ok_or_else(|| Error::new("PX1010", "the transform is degenerate (zero area)"))?;
    if m.is_lattice() {
        let (x0, y0, x1, y1) = m.map_rect(lxf, lyf, w, h);
        let (bx, by) = (x0.round(), y0.round());
        let (nw, nh) = check_size((x1 - x0).round(), (y1 - y0).round())?;
        let out = Image::from_fn(nw, nh, |px, py| {
            let (sx, sy) = inv.apply(bx + px as f64 + 0.5, by + py as f64 + 0.5);
            let (ix, iy) = ((sx - lxf).floor() as i64, (sy - lyf).floor() as i64);
            if ix < 0 || iy < 0 || ix >= img.width as i64 || iy >= img.height as i64 {
                [0.0; 4]
            } else {
                img.get(ix as u32, iy as u32)
            }
        });
        return Ok((out, bx as i32, by as i32));
    }
    if let Some((sx, sy)) = m.axis_scale() {
        let x0 = (sx * lxf + m.e).round();
        let x1 = (sx * (lxf + w) + m.e).round();
        let y0 = (sy * lyf + m.f).round();
        let y1 = (sy * (lyf + h) + m.f).round();
        let (nw, nh) = check_size((x1 - x0).max(1.0), (y1 - y0).max(1.0))?;
        if nw == img.width && nh == img.height {
            return Ok((img.clone(), x0 as i32, y0 as i32));
        }
        let f = filter.unwrap_or_else(|| Filter::auto(img.width.max(img.height), nw.max(nh)));
        return Ok((kernels::resize(img, nw, nh, f), x0 as i32, y0 as i32));
    }
    let (x0, y0, x1, y1) = m.map_rect(lxf, lyf, w, h);
    let (bx, by) = ((x0 + 1e-6).floor(), (y0 + 1e-6).floor());
    let (nw, nh) = check_size((x1 - 1e-6).ceil() - bx, (y1 - 1e-6).ceil() - by)?;
    let f = filter.unwrap_or(Filter::Bicubic);
    let (src, kx, ky) = preshrink(img, m.singular_values().1, f);
    let sampler = Sampler::new(&src, f);
    let out = Image::from_fn(nw.max(1), nh.max(1), |px, py| {
        let (sx, sy) = inv.apply(bx + px as f64 + 0.5, by + py as f64 + 0.5);
        sampler.sample((sx - lxf) * kx, (sy - lyf) * ky)
    });
    Ok((out, bx as i32, by as i32))
}

/// Shrink the source first when every direction of the transform reduces it, so the point
/// sampler does not alias. Returns the source and the coordinate scale into it.
pub fn preshrink(img: &Image, max_scale: f64, f: Filter) -> (Cow<'_, Image>, f64, f64) {
    if max_scale >= 0.999 || f == Filter::Nearest || img.is_empty() {
        return (Cow::Borrowed(img), 1.0, 1.0);
    }
    let nw = ((img.width as f64 * max_scale).ceil() as u32).max(1);
    let nh = ((img.height as f64 * max_scale).ceil() as u32).max(1);
    let small = kernels::resize(img, nw, nh, Filter::Lanczos3);
    (Cow::Owned(small), nw as f64 / img.width as f64, nh as f64 / img.height as f64)
}

/// Transform a canvas mask by `m` into a `nw`×`nh` canvas; `fill` is the value beyond the
/// old canvas.
pub fn warp_mask(mask: &Mask, m: &Affine2, nw: u32, nh: u32, fill: f32) -> Mask {
    if m.is_identity() && mask.width == nw && mask.height == nh {
        return mask.clone();
    }
    let Some(inv) = m.inverse() else {
        return Mask::filled(nw, nh, fill);
    };
    if m.is_lattice() {
        return Mask::from_fn(nw, nh, |x, y| {
            let (sx, sy) = inv.apply(x as f64 + 0.5, y as f64 + 0.5);
            let (ix, iy) = (sx.floor() as i64, sy.floor() as i64);
            if ix < 0 || iy < 0 || ix >= mask.width as i64 || iy >= mask.height as i64 {
                fill
            } else {
                mask.get(ix as u32, iy as u32)
            }
        });
    }
    if let Some((sx, sy)) = m.axis_scale() {
        let exact = m.e.abs() < 1e-9
            && m.f.abs() < 1e-9
            && (sx * mask.width as f64 - nw as f64).abs() < 1e-6
            && (sy * mask.height as f64 - nh as f64).abs() < 1e-6;
        if exact {
            let f = Filter::auto(mask.width.max(mask.height), nw.max(nh));
            return kernels::resize_mask(mask, nw, nh, f);
        }
    }
    let smax = m.singular_values().1;
    let (src, kx, ky): (Cow<'_, Mask>, f64, f64) = if smax < 0.999 && !mask.data.is_empty() {
        let w2 = ((mask.width as f64 * smax).ceil() as u32).max(1);
        let h2 = ((mask.height as f64 * smax).ceil() as u32).max(1);
        let small = kernels::resize_mask(mask, w2, h2, Filter::Lanczos3);
        (Cow::Owned(small), w2 as f64 / mask.width as f64, h2 as f64 / mask.height as f64)
    } else {
        (Cow::Borrowed(mask), 1.0, 1.0)
    };
    Mask::from_fn(nw, nh, |x, y| {
        let (sx, sy) = inv.apply(x as f64 + 0.5, y as f64 + 0.5);
        mask_bilinear(&src, sx * kx, sy * ky, fill).clamp(0.0, 1.0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing;

    #[test]
    fn lattice_rotations_are_exact() {
        let img = testing::gradient(7, 5);
        let rot = Affine2::rotate(90.0).then(Affine2::translate(5.0, 0.0));
        let (r, x, y) = affine_raster(&img, 0, 0, &rot, None).unwrap();
        assert_eq!((r.width, r.height, x, y), (5, 7, 0, 0));
        // (0,0) goes to the top-right corner
        assert_eq!(r.get(4, 0), img.get(0, 0));
        let mut back = r;
        for _ in 0..3 {
            back = affine_raster(
                &back,
                0,
                0,
                &Affine2::rotate(90.0).then(Affine2::translate(back.height as f64, 0.0)),
                None,
            )
            .unwrap()
            .0;
        }
        assert_eq!(back, img);
    }

    #[test]
    fn identity_like_sampling_preserves_pixels() {
        let img = testing::gradient(16, 16);
        let s = Sampler::new(&img, Filter::Bicubic);
        let p = s.sample(5.5, 7.5);
        let q = img.get(5, 7);
        for c in 0..4 {
            assert!((p[c] - q[c]).abs() < 1e-5);
        }
        assert_eq!(s.sample(-10.0, 3.0), [0.0; 4]);
    }

    #[test]
    fn scaled_raster_has_scaled_size() {
        let img = testing::checker(40, 20);
        let (r, x, y) = affine_raster(&img, 10, 10, &Affine2::scale(0.5, 0.5), None).unwrap();
        assert_eq!((r.width, r.height, x, y), (20, 10, 5, 5));
    }

    #[test]
    fn mask_warp_paths() {
        let m = Mask::from_fn(8, 4, |x, _| if x < 4 { 1.0 } else { 0.0 });
        let flipped = warp_mask(&m, &Affine2::new(-1.0, 0.0, 0.0, 1.0, 8.0, 0.0), 8, 4, 0.0);
        assert_eq!(flipped.get(7, 0), 1.0);
        assert_eq!(flipped.get(0, 0), 0.0);
        let shifted = warp_mask(&m, &Affine2::translate(2.0, 0.0), 10, 4, 0.5);
        assert_eq!(shifted.get(0, 0), 0.5);
        assert_eq!(shifted.get(2, 0), 1.0);
        let half = warp_mask(&m, &Affine2::scale(0.5, 0.5), 4, 2, 0.0);
        assert_eq!((half.width, half.height), (4, 2));
        assert_eq!(border_fill(&Mask::filled(3, 3, 1.0)), 1.0);
        assert_eq!(border_fill(&m), 0.0);
    }
}
