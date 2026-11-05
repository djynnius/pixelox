//! Non-affine maps for `transform perspective` and `transform warp`, evaluated once into a
//! per-pixel field of source coordinates that rasters (with any filter) and masks sample.
//!
//! * Perspective: the homography that maps the layer's bounds corners onto four points.
//! * Mesh warp: a grid of control points over the layer's bounds; the displacement of each
//!   node is interpolated with a Catmull-Rom tensor spline (linear data are reproduced
//!   exactly), evaluated on a fine 8 px mesh, and each mesh cell is inverted bilinearly.

use crate::sample::{check_size, mask_bilinear, preshrink, Sampler};
use pixelox_core::kernels::Filter;
use pixelox_core::prelude::*;
use rayon::prelude::*;

/// Source canvas coordinates for each pixel of a destination box (NaN = unmapped).
pub struct CoordField {
    pub x0: i32,
    pub y0: i32,
    pub w: u32,
    pub h: u32,
    src: Vec<[f32; 2]>,
    /// Estimated scale of the map (for pre-shrinking sources).
    pub scale: f64,
}

impl CoordField {
    fn new(x0: i32, y0: i32, w: u32, h: u32, scale: f64) -> CoordField {
        CoordField { x0, y0, w, h, src: vec![[f32::NAN; 2]; w as usize * h as usize], scale }
    }

    #[inline]
    pub fn get(&self, px: u32, py: u32) -> Option<(f64, f64)> {
        let v = self.src[py as usize * self.w as usize + px as usize];
        if v[0].is_nan() {
            None
        } else {
            Some((v[0] as f64, v[1] as f64))
        }
    }

    /// Resample a raster placed at (`lx`, `ly`) through the field.
    pub fn raster(&self, img: &Image, lx: i32, ly: i32, filter: Option<Filter>) -> (Image, i32, i32) {
        let f = filter.unwrap_or(Filter::Bicubic);
        let (src, kx, ky) = preshrink(img, self.scale, f);
        let sampler = Sampler::new(&src, f);
        let (lx, ly) = (lx as f64, ly as f64);
        let out = Image::from_fn(self.w.max(1), self.h.max(1), |px, py| {
            if px >= self.w || py >= self.h {
                return [0.0; 4];
            }
            match self.get(px, py) {
                Some((sx, sy)) => sampler.sample((sx - lx) * kx, (sy - ly) * ky),
                None => [0.0; 4],
            }
        });
        (out, self.x0, self.y0)
    }

    /// Resample a canvas mask through the field; `fill` outside the mapped region.
    pub fn mask(&self, m: &Mask, nw: u32, nh: u32, fill: f32) -> Mask {
        Mask::from_fn(nw, nh, |x, y| {
            let (px, py) = (x as i64 - self.x0 as i64, y as i64 - self.y0 as i64);
            if px < 0 || py < 0 || px >= self.w as i64 || py >= self.h as i64 {
                return fill;
            }
            match self.get(px as u32, py as u32) {
                Some((sx, sy)) => mask_bilinear(m, sx, sy, fill).clamp(0.0, 1.0),
                None => fill,
            }
        })
    }
}

fn bbox(pts: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    pts.iter().fold((f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY), |b, &(x, y)| {
        (b.0.min(x), b.1.min(y), b.2.max(x), b.3.max(y))
    })
}

fn field_box(pts: &[(f64, f64)]) -> Result<(i32, i32, u32, u32)> {
    let (x0, y0, x1, y1) = bbox(pts);
    let (bx, by) = ((x0 + 1e-6).floor(), (y0 + 1e-6).floor());
    let (w, h) = check_size((x1 - 1e-6).ceil() - bx, (y1 - 1e-6).ceil() - by)?;
    Ok((bx as i32, by as i32, w.max(1), h.max(1)))
}

fn poly_area(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    (0..n).map(|i| p[i].0 * p[(i + 1) % n].1 - p[(i + 1) % n].0 * p[i].1).sum::<f64>().abs() / 2.0
}

// ---------------------------------------------------------------------------------------
// perspective

/// A 3×3 projective map (row-major, h[8] normalized to 1 where possible).
#[derive(Clone, Copy, Debug)]
pub struct Homography(pub [f64; 9]);

impl Homography {
    /// The map sending `src[i]` to `dst[i]` (four correspondences).
    pub fn from_points(src: &[(f64, f64); 4], dst: &[(f64, f64); 4]) -> Option<Homography> {
        let mut a = [[0f64; 9]; 8];
        for i in 0..4 {
            let (x, y) = src[i];
            let (u, v) = dst[i];
            a[2 * i] = [x, y, 1.0, 0.0, 0.0, 0.0, -x * u, -y * u, u];
            a[2 * i + 1] = [0.0, 0.0, 0.0, x, y, 1.0, -x * v, -y * v, v];
        }
        // Gaussian elimination with partial pivoting on the 8×8 system
        for col in 0..8 {
            let piv = (col..8).max_by(|&r1, &r2| a[r1][col].abs().total_cmp(&a[r2][col].abs()))?;
            if a[piv][col].abs() < 1e-12 {
                return None;
            }
            a.swap(col, piv);
            let pivot_row = a[col];
            for (r, row) in a.iter_mut().enumerate() {
                if r != col {
                    let k = row[col] / pivot_row[col];
                    if k != 0.0 {
                        for (v, p) in row[col..].iter_mut().zip(&pivot_row[col..]) {
                            *v -= k * p;
                        }
                    }
                }
            }
        }
        let mut h = [0f64; 9];
        for i in 0..8 {
            h[i] = a[i][8] / a[i][i];
        }
        h[8] = 1.0;
        Some(Homography(h))
    }

    /// `(x', y', w)` before the perspective divide.
    #[inline]
    pub fn raw(&self, x: f64, y: f64) -> (f64, f64, f64) {
        let h = &self.0;
        (h[0] * x + h[1] * y + h[2], h[3] * x + h[4] * y + h[5], h[6] * x + h[7] * y + h[8])
    }

    #[cfg(test)]
    pub fn apply(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        let (u, v, w) = self.raw(x, y);
        if w.abs() < 1e-12 {
            None
        } else {
            Some((u / w, v / w))
        }
    }

    pub fn inverse(&self) -> Option<Homography> {
        let h = &self.0;
        let (a, b, c, d, e, f, g, k, l) = (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7], h[8]);
        let det = a * (e * l - f * k) - b * (d * l - f * g) + c * (d * k - e * g);
        if det.abs() < 1e-15 {
            return None;
        }
        let inv = [
            (e * l - f * k) / det,
            (c * k - b * l) / det,
            (b * f - c * e) / det,
            (f * g - d * l) / det,
            (a * l - c * g) / det,
            (c * d - a * f) / det,
            (d * k - e * g) / det,
            (b * g - a * k) / det,
            (a * e - b * d) / det,
        ];
        Some(Homography(inv))
    }
}

/// The field mapping the rectangle `r` (x, y, w, h) onto the quad `q` (tl, tr, br, bl).
pub fn perspective_field(r: (f64, f64, f64, f64), q: &[(f64, f64); 4]) -> Result<CoordField> {
    let (x, y, w, h) = r;
    let src = [(x, y), (x + w, y), (x + w, y + h), (x, y + h)];
    let bad = |why: &str| {
        Error::new("PX1010", format!("--quad cannot be used: {why}"))
            .with_hint(
                "give four corners clockwise from top-left: --quad 'x1,y1 x2,y2 x3,y3 x4,y4' forming a convex quad",
            )
            .with_context("quad", q.iter().map(|p| [p.0, p.1]).collect::<Vec<_>>())
    };
    let hm = Homography::from_points(&src, q).ok_or_else(|| bad("the points are degenerate"))?;
    // every source corner must stay on the same side of the horizon (a convex, ordered quad)
    let ws: Vec<f64> = src.iter().map(|&(sx, sy)| hm.raw(sx, sy).2).collect();
    if !(ws.iter().all(|&w| w > 1e-9) || ws.iter().all(|&w| w < -1e-9)) {
        return Err(bad("the quad folds over (points out of order or not convex)"));
    }
    let inv = hm.inverse().ok_or_else(|| bad("the map is not invertible"))?;
    let (cx, cy) = (q.iter().map(|p| p.0).sum::<f64>() / 4.0, q.iter().map(|p| p.1).sum::<f64>() / 4.0);
    let sign = inv.raw(cx, cy).2.signum();
    let (bx, by, bw, bh) = field_box(q)?;
    let scale = (poly_area(q) / (w * h).max(1e-9)).sqrt();
    let mut field = CoordField::new(bx, by, bw, bh, scale);
    let margin = 1.0;
    field.src.par_chunks_mut(bw as usize).enumerate().for_each(|(py, row)| {
        for (px, o) in row.iter_mut().enumerate() {
            let (dx, dy) = (bx as f64 + px as f64 + 0.5, by as f64 + py as f64 + 0.5);
            let (u, v, wq) = inv.raw(dx, dy);
            if wq.signum() != sign || wq.abs() < 1e-12 {
                continue;
            }
            let (sx, sy) = (u / wq, v / wq);
            if sx >= x - margin && sx <= x + w + margin && sy >= y - margin && sy <= y + h + margin {
                *o = [sx as f32, sy as f32];
            }
        }
    });
    Ok(field)
}

// ---------------------------------------------------------------------------------------
// mesh warp

/// Catmull-Rom through `p` at parameter `t` in `0..=p.len()-1`, with linearly extrapolated
/// end points (so straight data stay straight).
fn catmull(p: &[f64], t: f64) -> f64 {
    let n = p.len();
    if n == 1 {
        return p[0];
    }
    let seg = (t.floor() as usize).min(n - 2);
    let s = t - seg as f64;
    let at = |i: i64| -> f64 {
        if i < 0 {
            2.0 * p[0] - p[1]
        } else if i as usize >= n {
            2.0 * p[n - 1] - p[n - 2]
        } else {
            p[i as usize]
        }
    };
    let i = seg as i64;
    let (p0, p1, p2, p3) = (at(i - 1), at(i), at(i + 1), at(i + 2));
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * s
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * s * s
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * s * s * s)
}

fn cross(a: (f64, f64), b: (f64, f64)) -> f64 {
    a.0 * b.1 - a.1 * b.0
}

/// Inverse bilinear: the `(s, t)` in the unit square that the quad `a b c d`
/// (00, 10, 11, 01) maps to `p`, if any.
pub fn inverse_bilinear(
    p: (f64, f64),
    a: (f64, f64),
    b: (f64, f64),
    c: (f64, f64),
    d: (f64, f64),
) -> Option<(f64, f64)> {
    let e = (b.0 - a.0, b.1 - a.1);
    let f = (d.0 - a.0, d.1 - a.1);
    let g = (a.0 - b.0 + c.0 - d.0, a.1 - b.1 + c.1 - d.1);
    let h = (p.0 - a.0, p.1 - a.1);
    let k2 = cross(g, f);
    let k1 = cross(e, f) + cross(h, g);
    let k0 = cross(h, e);
    let solve_u = |v: f64| -> Option<f64> {
        let (dx, dy) = (e.0 + g.0 * v, e.1 + g.1 * v);
        if dx.abs() >= dy.abs() {
            (dx.abs() > 1e-12).then(|| (h.0 - f.0 * v) / dx)
        } else {
            (dy.abs() > 1e-12).then(|| (h.1 - f.1 * v) / dy)
        }
    };
    const EPS: f64 = 1e-6;
    let ok = |u: f64, v: f64| (-EPS..=1.0 + EPS).contains(&u) && (-EPS..=1.0 + EPS).contains(&v);
    if k2.abs() < 1e-10 * (k1.abs() + 1.0) {
        if k1.abs() < 1e-14 {
            return None;
        }
        let v = -k0 / k1;
        let u = solve_u(v)?;
        return ok(u, v).then_some((u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)));
    }
    let disc = k1 * k1 - 4.0 * k0 * k2;
    if disc < 0.0 {
        return None;
    }
    let sq = disc.sqrt();
    for v in [(-k1 - sq) / (2.0 * k2), (-k1 + sq) / (2.0 * k2)] {
        if let Some(u) = solve_u(v) {
            if ok(u, v) {
                return Some((u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)));
            }
        }
    }
    None
}

/// The field of a `cols`×`rows` control-point warp of the rectangle `r` whose nodes move to
/// `dst` (row-major, top row first).
pub fn mesh_field(r: (f64, f64, f64, f64), cols: usize, rows: usize, dst: &[(f64, f64)]) -> Result<CoordField> {
    let (x, y, w, h) = r;
    let node = |i: usize, j: usize| (x + w * i as f64 / (cols - 1) as f64, y + h * j as f64 / (rows - 1) as f64);
    let mut dxs = vec![0f64; cols * rows];
    let mut dys = vec![0f64; cols * rows];
    for j in 0..rows {
        for i in 0..cols {
            let (nx, ny) = node(i, j);
            dxs[j * cols + i] = dst[j * cols + i].0 - nx;
            dys[j * cols + i] = dst[j * cols + i].1 - ny;
        }
    }
    let disp = |u: f64, v: f64| -> (f64, f64) {
        let tu = u * (cols - 1) as f64;
        let tv = v * (rows - 1) as f64;
        let col_x: Vec<f64> = (0..rows).map(|j| catmull(&dxs[j * cols..(j + 1) * cols], tu)).collect();
        let col_y: Vec<f64> = (0..rows).map(|j| catmull(&dys[j * cols..(j + 1) * cols], tu)).collect();
        (catmull(&col_x, tv), catmull(&col_y, tv))
    };
    let nx = ((w / 8.0).ceil() as usize).clamp(1, 512);
    let ny = ((h / 8.0).ceil() as usize).clamp(1, 512);
    let mut pos = vec![(0f64, 0f64); (nx + 1) * (ny + 1)];
    pos.par_chunks_mut(nx + 1).enumerate().for_each(|(l, row)| {
        let v = l as f64 / ny as f64;
        for (k, o) in row.iter_mut().enumerate() {
            let u = k as f64 / nx as f64;
            let (ddx, ddy) = disp(u, v);
            *o = (x + u * w + ddx, y + v * h + ddy);
        }
    });
    let (bx, by, bw, bh) = field_box(&pos)?;
    let hull_area: f64 = {
        let corners = [pos[0], pos[nx], pos[(nx + 1) * (ny + 1) - 1], pos[ny * (nx + 1)]];
        poly_area(&corners)
    };
    let scale = (hull_area / (w * h).max(1e-9)).sqrt().max(0.0);
    // bucket cells by the destination rows they touch
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); bh as usize];
    let cell_corners = |k: usize, l: usize| {
        let i00 = l * (nx + 1) + k;
        (pos[i00], pos[i00 + 1], pos[i00 + nx + 2], pos[i00 + nx + 1])
    };
    for l in 0..ny {
        for k in 0..nx {
            let (a, b, c, d) = cell_corners(k, l);
            let y0 = a.1.min(b.1).min(c.1).min(d.1);
            let y1 = a.1.max(b.1).max(c.1).max(d.1);
            let r0 = ((y0.floor() as i64) - by as i64).max(0);
            let r1 = ((y1.ceil() as i64) - by as i64).min(bh as i64 - 1);
            for row in r0..=r1 {
                buckets[row as usize].push((l * nx + k) as u32);
            }
        }
    }
    let (cw, ch) = (w / nx as f64, h / ny as f64);
    let mut field = CoordField::new(bx, by, bw, bh, scale);
    field.src.par_chunks_mut(bw as usize).enumerate().for_each(|(py, row)| {
        let cy = by as f64 + py as f64 + 0.5;
        for &cell in &buckets[py] {
            let (k, l) = (cell as usize % nx, cell as usize / nx);
            let (a, b, c, d) = cell_corners(k, l);
            let x0 = a.0.min(b.0).min(c.0).min(d.0);
            let x1 = a.0.max(b.0).max(c.0).max(d.0);
            let c0 = ((x0.floor() as i64) - bx as i64).max(0);
            let c1 = ((x1.ceil() as i64) - bx as i64).min(bw as i64 - 1);
            for px in c0..=c1 {
                let o = &mut row[px as usize];
                if !o[0].is_nan() {
                    continue;
                }
                let cx = bx as f64 + px as f64 + 0.5;
                if let Some((s, t)) = inverse_bilinear((cx, cy), a, b, c, d) {
                    let sx = x + (k as f64 + s) * cw;
                    let sy = y + (l as f64 + t) * ch;
                    *o = [sx as f32, sy as f32];
                }
            }
        }
    });
    Ok(field)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn homography_round_trip() {
        let src = [(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)];
        let dst = [(2.0, 1.0), (12.0, 3.0), (11.0, 14.0), (1.0, 9.0)];
        let h = Homography::from_points(&src, &dst).unwrap();
        for i in 0..4 {
            let (u, v) = h.apply(src[i].0, src[i].1).unwrap();
            assert!((u - dst[i].0).abs() < 1e-9 && (v - dst[i].1).abs() < 1e-9);
        }
        let inv = h.inverse().unwrap();
        let (x, y) = inv.apply(dst[2].0, dst[2].1).unwrap();
        assert!((x - 10.0).abs() < 1e-9 && (y - 10.0).abs() < 1e-9);
    }

    #[test]
    fn inverse_bilinear_unit_square() {
        let (s, t) = inverse_bilinear((0.25, 0.75), (0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)).unwrap();
        assert!((s - 0.25).abs() < 1e-9 && (t - 0.75).abs() < 1e-9);
        assert!(inverse_bilinear((2.0, 0.5), (0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)).is_none());
        let (s, t) = inverse_bilinear((1.5, 1.0), (0.0, 0.0), (2.0, 0.0), (3.0, 2.0), (0.0, 2.0)).unwrap();
        assert!(s > 0.0 && s < 1.0 && (t - 0.5).abs() < 1e-9);
    }

    #[test]
    fn catmull_reproduces_lines() {
        let p = [0.0, 2.0, 4.0, 6.0];
        for t in [0.0, 0.3, 1.5, 2.9, 3.0] {
            assert!((catmull(&p, t) - 2.0 * t).abs() < 1e-9);
        }
    }

    #[test]
    fn identity_mesh_maps_to_itself() {
        let r = (0.0, 0.0, 32.0, 16.0);
        let dst: Vec<(f64, f64)> =
            (0..3).flat_map(|j| (0..3).map(move |i| (16.0 * i as f64, 8.0 * j as f64))).collect();
        let f = mesh_field(r, 3, 3, &dst).unwrap();
        assert_eq!((f.x0, f.y0, f.w, f.h), (0, 0, 32, 16));
        let (sx, sy) = f.get(10, 5).unwrap();
        assert!((sx - 10.5).abs() < 1e-3 && (sy - 5.5).abs() < 1e-3);
    }
}
