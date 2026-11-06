//! Fire and smoke: a field that rises from a layer's outline.
//!
//! The layer's alpha is resampled into a frame whose "up" is the rise direction (no
//! resampling for the default straight-up rise). Heat (fire) or density (smoke) is
//! injected in a band along the outline, more where the edge faces the rise direction and
//! in noise-placed patches (the roots of the flame tongues), and carried upward one row
//! at a time: each row takes the row below, mixed a little sideways (fire: the tongues
//! round off and taper; smoke: more the higher it gets, so the plume widens) and cooled
//! at a noise-modulated rate (some tongues reach higher than others). A second field
//! tracks the path length since the outline or the layer and cuts the plume off at its
//! height. A last per-pixel pass sways the plume sideways with a smooth noise warp that
//! grows with the height (the turbulence; a warp, unlike per-row drift, never folds the
//! field into hard fronts) and adds detail (fire: licks; smoke: domain-warped billows).
//!
//! All noise is a pure function of the seed and of coordinates relative to the layer's
//! content, the per-pixel passes are independent, and the row sweep is serial, so the
//! result depends neither on the thread count nor on where the layer sits.

use crate::noise::Noise;
use pixelox_core::kernels::{self, Edge};
use pixelox_core::prelude::*;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Plume {
    Fire,
    Smoke,
}

/// The shape of a plume, resolved to pixels.
#[derive(Clone, Debug)]
pub(crate) struct Rise {
    pub plume: Plume,
    /// Maximum plume height in px (path length from the outline).
    pub height: f64,
    /// Fire: intensity (1 = default heat); smoke: density (0..1).
    pub strength: f64,
    /// 0..1.
    pub turbulence: f64,
    /// Rise direction in degrees (0 = right, 90 = up).
    pub angle: f64,
    pub seed: u64,
}

/// A plume over the effect buffer.
pub(crate) struct Field {
    /// Fire: heat (about 0..1.5); smoke: density (0..1).
    pub value: Mask,
    /// Fire: fraction of the height travelled (0 at the outline); smoke: shading (0..1).
    pub aux: Mask,
}

impl Rise {
    /// Width (Gaussian σ) of the band along the outline where the plume starts.
    pub(crate) fn band(&self) -> f64 {
        let k = if self.plume == Plume::Fire { 0.035 } else { 0.05 };
        (self.height * k).clamp(0.75, 64.0)
    }

    /// Sideways sway at the top of the plume, in units of [`Rise::feature`].
    fn slope(&self) -> f64 {
        match self.plume {
            Plume::Fire => 0.1 + 0.45 * self.turbulence,
            Plume::Smoke => 0.15 + 0.6 * self.turbulence,
        }
    }

    /// Width of a flame tongue / smoke billow (px).
    fn feature(&self) -> f64 {
        match self.plume {
            Plume::Fire => (self.height * 0.16).max(2.0),
            Plume::Smoke => (self.height * 0.3).max(3.0),
        }
    }

    /// How far beyond the content the plume can reach (px): along the rise, sideways,
    /// and backward.
    fn reach(&self) -> (f64, f64, f64) {
        let edge = 4.0 * self.band() + 2.0;
        // sideways: the sway plus 3.5 σ of the mixing over the whole height (each row
        // adds a variance of s²/4 for the 5-tap kernel of spacing s)
        let per_row = match self.plume {
            Plume::Fire => 0.25 * self.mix() * self.mix(),
            Plume::Smoke => {
                // s = 0.5 + k t over the height: the mean of s² for t in 0..1
                let k = self.spread();
                0.25 * (0.25 + 0.5 * k + k * k / 3.0)
            }
        };
        let side = self.feature() * self.slope() + 3.5 * (per_row * self.height).sqrt() + edge;
        (self.height + edge, side, edge)
    }

    /// Fire: the spacing of the sideways mixing per row (px).
    fn mix(&self) -> f64 {
        (self.feature() * 0.03).max(0.5)
    }

    /// Smoke: how fast the sideways spreading grows with the height (s = 0.5 + k t).
    fn spread(&self) -> f64 {
        3.0 * (0.5 + self.turbulence)
    }

    /// The reach as margins around the content: left, top, right, bottom (px).
    pub(crate) fn margins(&self) -> [f64; 4] {
        let (up, side, back) = self.reach();
        let th = self.angle.to_radians();
        let u = (th.cos(), -th.sin());
        let q = (-u.1, u.0);
        let mut m = [0.0f64; 4];
        for across in [-side, side] {
            for along in [-back, up] {
                let (x, y) = (across * q.0 + along * u.0, across * q.1 + along * u.1);
                m = [m[0].max(-x), m[1].max(-y), m[2].max(x), m[3].max(y)];
            }
        }
        m
    }
}

#[inline]
fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Bilinear sample of a mask at continuous pixel coordinates (centers on integers),
/// zero outside.
fn sample(m: &Mask, x: f64, y: f64) -> f32 {
    let (x0, y0) = (x.floor(), y.floor());
    let (tx, ty) = ((x - x0) as f32, (y - y0) as f32);
    let (ix, iy) = (x0 as i64, y0 as i64);
    let get = |x: i64, y: i64| m.at(x, y);
    let a = get(ix, iy) * (1.0 - tx) + get(ix + 1, iy) * tx;
    let b = get(ix, iy + 1) * (1.0 - tx) + get(ix + 1, iy + 1) * tx;
    a * (1.0 - ty) + b * ty
}

/// Linear sample of one row at a fractional x; `out` beyond the ends.
#[inline]
fn row_at(row: &[f32], x: f64, out: f32) -> f32 {
    let x0 = x.floor();
    let t = (x - x0) as f32;
    let i = x0 as i64;
    let n = row.len() as i64;
    let get = |i: i64| if i < 0 || i >= n { out } else { row[i as usize] };
    get(i) * (1.0 - t) + get(i + 1) * t
}

/// The smallest value of `row` over the pixels within `r` of `x` (and at least the two
/// around it).
#[inline]
fn row_min(row: &[f32], x: f64, r: f64, out: f32) -> f32 {
    let lo = (x - r).floor() as i64;
    let hi = (x + r).ceil() as i64;
    let n = row.len() as i64;
    (lo..=hi).filter(|&i| i >= 0 && i < n).map(|i| row[i as usize]).fold(out, f32::min)
}

/// The plume of `r` rising from the coverage `a` (the effect buffer's alpha). `org` is the
/// buffer's top-left corner in coordinates relative to the layer's content, which anchors
/// the noise so the plume moves with the layer.
pub(crate) fn field(a: &Mask, r: &Rise, org: (f64, f64)) -> Field {
    let (w, h) = (a.width, a.height);
    let straight = (r.angle - 90.0).rem_euclid(360.0) < 1e-9;
    let (up, side) = if straight {
        ((0.0, -1.0), (1.0, 0.0))
    } else {
        let th = r.angle.to_radians();
        let up = (th.cos(), -th.sin());
        (up, (-up.1, up.0))
    };
    let dot = |p: (f64, f64), q: (f64, f64)| p.0 * q.0 + p.1 * q.1;
    let c = (w as f64 / 2.0, h as f64 / 2.0);
    let (fw, fh) = if straight {
        (w, h)
    } else {
        (
            (side.0.abs() * w as f64 + side.1.abs() * h as f64).ceil() as u32 + 2,
            (up.0.abs() * w as f64 + up.1.abs() * h as f64).ceil() as u32 + 2,
        )
    };
    let fc = (fw as f64 / 2.0, fh as f64 / 2.0);
    // noise coordinates of frame pixel (i, j): n0 + (i + 0.5, j + 0.5)
    let anchor = (c.0 + org.0, c.1 + org.1);
    let n0 = (dot(side, anchor) - fc.0, -dot(up, anchor) - fc.1);
    if straight {
        let (value, aux) = rise(a, r, n0);
        return Field { value, aux };
    }
    let fa = Mask::from_fn(fw, fh, |i, j| {
        let (fx, fy) = (i as f64 + 0.5 - fc.0, j as f64 + 0.5 - fc.1);
        let (bx, by) = (c.0 + fx * side.0 - fy * up.0, c.1 + fx * side.1 - fy * up.1);
        sample(a, bx - 0.5, by - 0.5)
    });
    let (value, aux) = rise(&fa, r, n0);
    drop(fa);
    let back = |m: &Mask| {
        Mask::from_fn(w, h, |x, y| {
            let p = (x as f64 + 0.5 - c.0, y as f64 + 0.5 - c.1);
            sample(m, dot(side, p) + fc.0 - 0.5, -dot(up, p) + fc.1 - 0.5)
        })
    };
    Field { value: back(&value), aux: back(&aux) }
}

/// Where flames lick over the content's own edges (for `--front`): tongue-shaped patches,
/// stretched along the rise direction, that cover about half of the outline and reach a
/// little way inside it (0..1, already × the content's alpha).
pub(crate) fn licks(a: &Mask, r: &Rise, org: (f64, f64)) -> Mask {
    let (w, h) = (a.width as usize, a.height as usize);
    let lam = r.feature();
    let mut g = a.clone();
    let s = r.band() * 3.0;
    kernels::gaussian_planar(&mut g.data, w, h, 1, s, s, Edge::Zero);
    let th = r.angle.to_radians();
    let up = (th.cos(), -th.sin());
    let side = (-up.1, up.0);
    let n = Noise::new(r.seed).lane(6);
    Mask::from_fn(a.width, a.height, |x, y| {
        let i = y as usize * w + x as usize;
        let av = a.data[i];
        // 1 at the outline, 0 deep inside
        let edge = ((1.0 - g.data[i]) * 2.0).clamp(0.0, 1.0);
        if av <= 0.0 || edge <= 0.0 {
            return 0.0;
        }
        let p = (x as f64 + 0.5 + org.0, y as f64 + 0.5 + org.1);
        let (u, v) = (side.0 * p.0 + side.1 * p.1, -(up.0 * p.0 + up.1 * p.1));
        let t = (0.5 + 0.5 * n.fbm(u / (lam * 0.3), v / (lam * 0.9), 2)) as f32;
        av * smoothstep(0.0, 0.1, t - 0.45 - (1.0 - edge) * 0.5)
    })
}

/// The plume in the rise frame (up = toward row 0).
fn rise(fa: &Mask, r: &Rise, n0: (f64, f64)) -> (Mask, Mask) {
    let (w, h) = (fa.width as usize, fa.height as usize);
    if w == 0 || h == 0 || r.height <= 0.0 || r.strength <= 0.0 {
        return (Mask::new(fa.width, fa.height), Mask::new(fa.width, fa.height));
    }
    let fire = r.plume == Plume::Fire;
    let band = r.band();
    let lam = r.feature();
    let slope = r.slope();
    let turb = r.turbulence;
    let noise = Noise::new(r.seed);
    let (n_drift, n_cool, n_root, n_detail, n_warp) =
        (noise.lane(1), noise.lane(2), noise.lane(3), noise.lane(4), noise.lane(5));

    // the outline band and how much each part of it faces the rise direction
    let mut g = fa.clone();
    kernels::gaussian_planar(&mut g.data, w, h, 1, band, band, Edge::Zero);
    // heat injected per row: the integral of 4g(1-g) across a straight edge is ~2.26σ
    let gain = (r.strength * if fire { 1.35 } else { 1.0 } / (2.26 * band)) as f32;
    let mut inj = vec![0f32; w * h];
    inj.par_chunks_mut(w).enumerate().for_each(|(j, row)| {
        let gy = |i: usize, jj: i64| g.at(i as i64, jj);
        for (i, v) in row.iter_mut().enumerate() {
            let gc = g.data[j * w + i];
            let ring = (4.0 * gc * (1.0 - gc)).clamp(0.0, 1.0);
            if ring < 1e-3 {
                continue;
            }
            let dy = (gy(i, j as i64 + 1) - gy(i, j as i64 - 1)) * 0.5;
            let dx = (g.at(i as i64 + 1, j as i64) - g.at(i as i64 - 1, j as i64)) * 0.5;
            // n·up for the outward normal n = -∇g/|∇g| (up = -y): ∂g/∂y / |∇g|
            let facing = dy / (dx.hypot(dy) + 1e-6);
            let weight = if fire { (0.3 + 0.7 * facing).clamp(0.0, 1.0) } else { (0.2 + 0.8 * facing).clamp(0.0, 1.0) };
            let (nx, ny) = (n0.0 + i as f64 + 0.5, n0.1 + j as f64 + 0.5);
            // patches along the outline feed the tongues (fire) or puffs (smoke)
            let root = n_root.fbm(nx / (lam * 0.7), ny / (lam * 0.7), 2) as f32;
            let patch = if fire { (0.45 + 1.6 * root).clamp(0.0, 1.8) } else { (0.75 + 0.6 * root).clamp(0.2, 1.4) };
            *v = ring * weight * patch * gain;
        }
    });

    drop(g);
    // carry the plume up row by row; sideways mixing rounds the tongues so they taper
    // (fire) or spreads the plume wider the higher it gets (smoke)
    let cap = (r.strength * if fire { 1.6 } else { 1.2 }) as f32;
    let ell0 = if fire { r.height / 1.8 } else { r.height / 1.4 };
    let reset = 0.02 * gain * band as f32; // injection that counts as "at the source"
    let none = f32::MAX / 4.0;
    let (mix, spread) = (r.mix(), r.spread());
    // the cooling per row: a noise-modulated rate, so some tongues reach higher than
    // others (computed for every pixel in parallel; the sweep below is cheap and serial)
    let mut cool = vec![0f32; w * h];
    cool.par_chunks_mut(w).enumerate().for_each(|(j, row)| {
        for (i, c) in row.iter_mut().enumerate() {
            let (nx, ny) = (n0.0 + i as f64 + 0.5, n0.1 + j as f64 + 0.5);
            let keep = 0.5 + 0.5 * n_cool.fbm(nx / lam, ny / (lam * if fire { 1.2 } else { 2.5 }), 2);
            let ell = if fire { ell0 * (0.25 + 1.1 * keep) } else { ell0 * (0.5 + 0.8 * keep) };
            *c = (-1.0 / ell.max(0.5)).exp() as f32;
        }
    });
    let mut val = vec![0f32; w * h];
    let mut age = vec![none; w * h];
    for j in (0..h).rev() {
        let (vh, vt) = val.split_at_mut((j + 1) * w);
        let (ah, at) = age.split_at_mut((j + 1) * w);
        let (vrow, arow) = (&mut vh[j * w..], &mut ah[j * w..]);
        let below: Option<(&[f32], &[f32])> = if j + 1 < h { Some((&vt[..w], &at[..w])) } else { None };
        let (inj_row, cool_row) = (&inj[j * w..(j + 1) * w], &cool[j * w..(j + 1) * w]);
        let inside = &fa.data[j * w..(j + 1) * w];
        for i in 0..w {
            let x = i as f64;
            let (carried, travelled) = match below {
                None => (0.0, none),
                Some((bv, ba)) => {
                    let s = if fire { mix } else { 0.5 + spread * (ba[i] as f64 / r.height).clamp(0.0, 1.0) };
                    // the youngest plume that mixes into this pixel
                    let a0 = row_min(ba, x, s, none);
                    let v = 0.0625 * (row_at(bv, x - s, 0.0) + row_at(bv, x + s, 0.0))
                        + 0.25 * (row_at(bv, x - s * 0.5, 0.0) + row_at(bv, x + s * 0.5, 0.0))
                        + 0.375 * bv[i];
                    (v, a0)
                }
            };
            let added = inj_row[i];
            vrow[i] = (added + carried * cool_row[i]).min(cap);
            // the age counts from the outline or the layer itself (behind which the plume
            // is hidden), so it is continuous across the outline
            arow[i] = if added >= reset || inside[i] > 0.5 { 0.0 } else { (travelled + 1.0).min(none) };
        }
    }
    drop(inj);
    drop(cool);
    // the age jumps where the plume passes a new stretch of outline; everything shaped
    // by it uses a smoothed copy (no seams), and the true age only caps the very top
    let height = r.height as f32;
    let mut smooth: Vec<f32> = age.par_iter().map(|&a| (a / height).min(1.25)).collect();
    let soft = (lam * 0.3).max(1.0);
    kernels::gaussian_planar(&mut smooth, w, h, 1, soft, soft, Edge::Clamp);

    // sway (a smooth sideways warp that grows with the height), detail, and the height cap
    let sway = lam * slope;
    let mut out = vec![0f32; w * h];
    let mut aux = vec![0f32; w * h];
    out.par_chunks_mut(w).zip(aux.par_chunks_mut(w)).enumerate().for_each(|(j, (orow, xrow))| {
        let row = j * w..(j + 1) * w;
        let (vrow, arow, srow) = (&val[row.clone()], &age[row.clone()], &smooth[row]);
        for i in 0..w {
            let (nx, ny) = (n0.0 + i as f64 + 0.5, n0.1 + j as f64 + 0.5);
            let t0 = srow[i].min(1.0) as f64;
            let d = sway * (0.15 + 0.85 * t0) * n_drift.fbm(nx / lam, ny / (lam * 1.3), 2);
            let sx = i as f64 + d;
            let v = row_at(vrow, sx, 0.0);
            if v <= 0.0 {
                continue;
            }
            let t = row_at(srow, sx, 1.25);
            let top = 1.0 - smoothstep(0.9, 1.0, row_min(arow, sx, 0.0, none) / height);
            let window = (1.0 - smoothstep(if fire { 0.6 } else { 0.45 }, 1.0, t)) * top;
            if window <= 0.0 {
                continue;
            }
            if fire {
                // fine licks, stretched along the rise
                let dd = n_detail.fbm(nx / (lam * 0.35), ny / (lam * 0.8), 3) as f32;
                let amp = 0.35 + 0.5 * turb as f32;
                orow[i] = (v * window * (1.0 + amp * dd)).max(0.0);
                xrow[i] = t.min(1.0);
            } else {
                // billows: domain-warped noise that grows with the height
                let s = lam * (0.45 + 0.6 * t.min(1.0) as f64);
                let wx = nx + s * 0.8 * n_warp.fbm(nx / s, ny / s, 2);
                let wy = ny + s * 0.8 * n_warp.lane(9).fbm(nx / s, ny / s, 2);
                let b = (0.5 + 0.5 * n_detail.fbm(wx / s, wy / s, 4)) as f32;
                let puff = smoothstep(0.3, 0.8, b);
                orow[i] = (v * window * (0.1 + 1.0 * puff)).clamp(0.0, 1.0);
                xrow[i] = b;
            }
        }
    });
    (Mask { width: fa.width, height: fa.height, data: out }, Mask { width: fa.width, height: fa.height, data: aux })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn disc() -> Mask {
        Mask::from_fn(80, 80, |x, y| if (x as f32 - 40.0).hypot(y as f32 - 50.0) < 10.0 { 1.0 } else { 0.0 })
    }

    fn rise(plume: Plume, angle: f64) -> Rise {
        Rise { plume, height: 24.0, strength: 1.0, turbulence: 0.5, angle, seed: 3 }
    }

    /// Sum of the field above / below / left / right of the disc's center.
    fn quadrants(m: &Mask) -> [f64; 4] {
        let mut q = [0.0; 4];
        for y in 0..m.height {
            for x in 0..m.width {
                let (dx, dy) = (x as f64 - 40.0, y as f64 - 50.0);
                let v = m.get(x, y) as f64;
                if dy.abs() >= dx.abs() {
                    q[if dy < 0.0 { 0 } else { 1 }] += v;
                } else {
                    q[if dx < 0.0 { 2 } else { 3 }] += v;
                }
            }
        }
        q
    }

    #[test]
    fn plumes_rise_along_the_angle() {
        for plume in [Plume::Fire, Plume::Smoke] {
            let f = field(&disc(), &rise(plume, 90.0), (0.0, 0.0));
            let [up, down, ..] = quadrants(&f.value);
            assert!(up > 4.0 * down.max(1e-3), "{plume:?}: up {up} down {down}");
            let f = field(&disc(), &rise(plume, 0.0), (0.0, 0.0));
            let [_, _, left, right] = quadrants(&f.value);
            assert!(right > 4.0 * left.max(1e-3), "{plume:?}: right {right} left {left}");
        }
    }

    #[test]
    fn the_margins_hold_the_whole_plume() {
        // with the renderer's margins around a disc, nothing reaches the buffer's border
        for plume in [Plume::Fire, Plume::Smoke] {
            for (height, turbulence, angle) in
                [(6.0, 1.0, 90.0), (30.0, 0.0, 0.0), (30.0, 1.0, 225.0), (90.0, 0.5, 120.0)]
            {
                let r = Rise {
                    plume,
                    height,
                    strength: if plume == Plume::Fire { 2.0 } else { 1.0 },
                    turbulence,
                    angle,
                    seed: 5,
                };
                let [l, t, rr, b] = r.margins().map(|v| v.ceil() as u32 + 2);
                let (w, h) = (20 + l + rr, 20 + t + b);
                let a = Mask::from_fn(w, h, |x, y| {
                    let (dx, dy) = (x as f32 + 0.5 - (l + 10) as f32, y as f32 + 0.5 - (t + 10) as f32);
                    if dx.hypot(dy) < 10.0 {
                        1.0
                    } else {
                        0.0
                    }
                });
                let f = field(&a, &r, (0.0, 0.0));
                let total: f32 = f.value.data.iter().sum();
                assert!(total > 1.0, "{plume:?} {height} {turbulence} {angle}: empty");
                for y in 0..h {
                    for x in 0..w {
                        if x == 0 || y == 0 || x == w - 1 || y == h - 1 {
                            assert!(
                                f.value.get(x, y) < 1e-3,
                                "{plume:?} {height} {turbulence} {angle}: clipped at {x},{y}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_field_moves_with_its_anchor() {
        // shifting the buffer by whole pixels while moving the anchor the other way is
        // the same plume, shifted
        let a = disc();
        let shifted = Mask::from_fn(80, 80, |x, y| a.at(x as i64 - 3, y as i64 + 2));
        let r = rise(Plume::Fire, 90.0);
        let f1 = field(&a, &r, (10.0, 20.0));
        let f2 = field(&shifted, &r, (7.0, 22.0));
        for y in 10..70 {
            for x in 10..70 {
                assert_eq!(f1.value.get(x, y), f2.value.get(x + 3, y - 2), "{x},{y}");
            }
        }
    }
}
