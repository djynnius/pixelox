//! The `smoke` kind of `fill noise`: wisps traced along a rising, swirling flow over a
//! soft haze.
//!
//! The flow is the curl of a smooth stream function plus a steady drift along `--angle`
//! (divergence-free, so strands never bunch into blobs). A fine fiber texture is averaged
//! along each pixel's streamline (line integral convolution), which draws long, thin,
//! curling strands; a slow density field leaves clear gaps, and a blurred copy plus a soft
//! haze give the volume. The flow and fibers are sampled from precomputed grids, so the
//! cost is O(pixels × streamline length), computed at half resolution on large canvases.

use pixelox_core::kernels;
use pixelox_core::noise::{self, Basis, Field, Pattern};
use rayon::prelude::*;

#[inline]
fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// A field sampled in px with a cell size (and a period when tiling).
#[derive(Clone, Copy)]
struct Px {
    f: Field,
    cell: (f64, f64),
}

impl Px {
    /// `cell` px cells; with `tile` = canvas size, cells are fitted so the field repeats.
    fn new(f: Field, cell: (f64, f64), tile: Option<(u32, u32)>) -> Px {
        match tile {
            None => Px { f: Field { period: None, ..f }, cell },
            Some((w, h)) => {
                let (nx, ny) = ((w as f64 / cell.0).round().max(1.0), (h as f64 / cell.1).round().max(1.0));
                let per_octave_ok = f.lacunarity.fract() == 0.0;
                Px {
                    f: Field { period: per_octave_ok.then_some((nx as i64, ny as i64)), ..f },
                    cell: (w as f64 / nx, h as f64 / ny),
                }
            }
        }
    }

    #[inline]
    fn at(&self, x: f64, y: f64) -> f64 {
        self.f.at(x / self.cell.0, y / self.cell.1)
    }
}

/// A regular grid of values over a canvas rectangle, sampled bilinearly (clamped).
struct Grid<const C: usize> {
    x0: f64,
    y0: f64,
    step: f64,
    w: usize,
    h: usize,
    data: Vec<[f32; C]>,
}

impl<const C: usize> Grid<C> {
    fn build(x0: f64, y0: f64, step: f64, w: usize, h: usize, f: impl Fn(f64, f64) -> [f32; C] + Sync) -> Grid<C> {
        let mut data = vec![[0f32; C]; w * h];
        data.par_chunks_mut(w.max(1)).enumerate().for_each(|(j, row)| {
            for (i, v) in row.iter_mut().enumerate() {
                *v = f(x0 + i as f64 * step, y0 + j as f64 * step);
            }
        });
        Grid { x0, y0, step, w, h, data }
    }

    #[inline]
    fn sample(&self, x: f64, y: f64) -> [f32; C] {
        let fx = ((x - self.x0) / self.step).clamp(0.0, (self.w - 1) as f64);
        let fy = ((y - self.y0) / self.step).clamp(0.0, (self.h - 1) as f64);
        let (i, j) = ((fx as usize).min(self.w.saturating_sub(2)), (fy as usize).min(self.h.saturating_sub(2)));
        let (tx, ty) = ((fx - i as f64) as f32, (fy - j as f64) as f32);
        let (i1, j1) = ((i + 1).min(self.w - 1), (j + 1).min(self.h - 1));
        let (a, b, c, d) = (
            self.data[j * self.w + i],
            self.data[j * self.w + i1],
            self.data[j1 * self.w + i],
            self.data[j1 * self.w + i1],
        );
        let mut o = [0f32; C];
        for k in 0..C {
            let top = a[k] + (b[k] - a[k]) * tx;
            let bot = c[k] + (d[k] - c[k]) * tx;
            o[k] = top + (bot - top) * ty;
        }
        o
    }
}

/// Smoke over canvas pixels.
pub(crate) struct Smoke {
    psi: Px,
    fibers: Px,
    density: Px,
    haze: Px,
    /// Feature size in px.
    scale: f64,
    /// The drift (rise) vector, in flow units.
    drift: (f64, f64),
    /// Strength of the curl (swirl) relative to the default.
    swirl: f64,
    look: Look,
}

/// The tone curves and mix of the smoke's parts (tuned by eye).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Look {
    /// Fine fibers: smoothstep edges on the averaged fiber texture.
    fiber: (f64, f64),
    /// Coarse volume: smoothstep edges on the averaged volume texture.
    vol: (f64, f64),
    /// Density (gaps): smoothstep edges.
    dens: (f64, f64),
    /// Blur sigmas (x scale) of the soft wisps and of the volume.
    blur: (f64, f64),
    /// Weights of crisp wisps, soft wisps, and volume.
    mix: (f32, f32, f32),
    /// Fiber cell size (x scale).
    fiber_cell: f64,
    /// Volume cell size (x scale).
    vol_cell: f64,
}

const LOOK: Look = Look {
    fiber: (0.5, 0.74),
    vol: (0.4, 0.8),
    dens: (0.36, 0.74),
    blur: (0.012, 0.02),
    mix: (0.5, 0.4, 0.8),
    fiber_cell: 1.0 / 18.0,
    vol_cell: 0.35,
};

impl Smoke {
    /// `angle`: the rise direction in degrees (90 = up); `stretch`: how straight the
    /// smoke rises (1 = swirly, 3 = nearly straight); `swirl`: curl strength (1 = default).
    pub fn new(
        seed: u64,
        scale: f64,
        angle: f64,
        stretch: f64,
        swirl: f64,
        octaves: u32,
        persistence: f64,
        basis: Basis,
        tile: Option<(u32, u32)>,
    ) -> Smoke {
        let basis = if tile.is_some() { Basis::Perlin } else { basis };
        let f = |k: u64, octaves: u32, persistence: f64| Field {
            pattern: Pattern::Fbm,
            basis,
            octaves,
            persistence,
            lacunarity: 2.0,
            seed: noise::lane(seed, 10 + k),
            period: None,
        };
        let th = angle.to_radians();
        let strength = 0.85 * stretch.max(0.1);
        Smoke {
            psi: Px::new(f(0, 3, 0.5), (1.2 * scale, 1.6 * scale), tile),
            fibers: Px::new(
                f(1, octaves.clamp(1, 6), 0.65f64.max(persistence)),
                (scale * LOOK.fiber_cell, scale * LOOK.fiber_cell),
                tile,
            ),
            density: Px::new(f(2, 3, 0.5), (scale * 1.1, scale * 1.6), tile),
            haze: Px::new(f(3, 4, 0.55), (scale * LOOK.vol_cell, scale * LOOK.vol_cell), tile),
            scale,
            drift: (strength * th.cos(), -strength * th.sin()),
            swirl,
            look: LOOK,
        }
    }

    /// Smoke values (about 0..1) of the `w`×`h` canvas pixels whose top-left is (x0, y0).
    pub fn plane(&self, x0: i64, y0: i64, w: usize, h: usize) -> Vec<f32> {
        if w == 0 || h == 0 {
            return Vec::new();
        }
        let s = self.scale;
        let lk = self.look;
        let (wisp_sigma, haze_sigma) = (lk.blur.0 * s, lk.blur.1 * s);
        let mb = (3.0 * haze_sigma).ceil() as i64;
        let reach = 0.22 * s;
        let res = if s > 260.0 { 2usize } else { 1 };
        // the extended region (output + blur margin), in px
        let (ex0, ey0) = (x0 - mb, y0 - mb);
        let (ew, eh) = (w + 2 * mb as usize, h + 2 * mb as usize);
        let (ww, wh) = (ew.div_ceil(res), eh.div_ceil(res));
        let rf = res as f64;
        // flow directions over the extended region plus the streamline reach
        let vstep = (s / 48.0).max(2.0);
        let pad = reach + 2.0 * vstep;
        let (gx0, gy0) = (ex0 as f64 - pad, ey0 as f64 - pad);
        let (gw, gh) = (
            ((ew as f64 + 2.0 * pad) / vstep).ceil() as usize + 2,
            ((eh as f64 + 2.0 * pad) / vstep).ceil() as usize + 2,
        );
        let e = 0.5 * vstep;
        let (psi, drift, swirl) = (self.psi, self.drift, self.swirl);
        let flow = Grid::<2>::build(gx0, gy0, vstep, gw, gh, |x, y| {
            let dpu = (psi.at(x + e, y) - psi.at(x - e, y)) / (2.0 * e / psi.cell.0);
            let dpv = (psi.at(x, y + e) - psi.at(x, y - e)) / (2.0 * e / psi.cell.1);
            let (vx, vy) = (swirl * dpv + drift.0, -swirl * dpu + drift.1);
            let m = (vx * vx + vy * vy).sqrt().max(1e-9);
            [(vx / m) as f32, (vy / m) as f32]
        });
        // fiber texture at the working resolution
        let (fibers, haze) = (self.fibers, self.haze);
        let (tw, th) =
            (((ew as f64 + 2.0 * pad) / rf).ceil() as usize + 2, ((eh as f64 + 2.0 * pad) / rf).ceil() as usize + 2);
        let tex = Grid::<2>::build(gx0, gy0, rf, tw, th, |x, y| [fibers.at(x, y) as f32, haze.at(x, y) as f32]);
        // line integral convolution along the flow, both ways: fine fibers and coarse volume
        let h_step = 1.5 * rf;
        let steps = ((reach / h_step).round() as usize).clamp(4, 64);
        let weights: Vec<f32> =
            (1..=steps).map(|k| 0.5 + 0.5 * (std::f32::consts::PI * k as f32 / (steps + 1) as f32).cos()).collect();
        let wsum = 1.0 + 2.0 * weights.iter().sum::<f32>();
        let dens = self.density;
        let mut wisps = vec![0f32; ww * wh];
        let mut hz = vec![0f32; ww * wh];
        wisps.par_chunks_mut(ww).zip(hz.par_chunks_mut(ww)).enumerate().for_each(|(j, (wrow, hrow))| {
            let py = ey0 as f64 + (j as f64 + 0.5) * rf;
            for i in 0..ww {
                let px = ex0 as f64 + (i as f64 + 0.5) * rf;
                let mut acc = tex.sample(px, py);
                for dir in [1.0f64, -1.0] {
                    let (mut x, mut y) = (px, py);
                    for wt in &weights {
                        let v = flow.sample(x, y);
                        x += dir * v[0] as f64 * h_step;
                        y += dir * v[1] as f64 * h_step;
                        let t = tex.sample(x, y);
                        acc[0] += wt * t[0];
                        acc[1] += wt * t[1];
                    }
                }
                let (fine, coarse) = ((acc[0] / wsum) as f64, (acc[1] / wsum) as f64);
                let d = smoothstep(lk.dens.0, lk.dens.1, dens.at(px, py));
                wrow[i] = (smoothstep(lk.fiber.0, lk.fiber.1, fine) * d) as f32;
                hrow[i] = (smoothstep(lk.vol.0, lk.vol.1, coarse) * (0.3 + 0.7 * d)) as f32;
            }
        });
        // back to full resolution
        let up = |src: Vec<f32>| -> Vec<f32> {
            if res == 1 {
                return src;
            }
            let g = Grid::<1> {
                x0: 0.5 * rf - 0.5,
                y0: 0.5 * rf - 0.5,
                step: rf,
                w: ww,
                h: wh,
                data: src.into_iter().map(|v| [v]).collect(),
            };
            let mut out = vec![0f32; ew * eh];
            out.par_chunks_mut(ew).enumerate().for_each(|(j, row)| {
                for (i, o) in row.iter_mut().enumerate() {
                    *o = g.sample(i as f64, j as f64)[0];
                }
            });
            out
        };
        let wisps = up(wisps);
        let hz = up(hz);
        let mut soft = wisps.clone();
        kernels::gaussian_planar(&mut soft, ew, eh, 1, wisp_sigma, wisp_sigma, kernels::Edge::Clamp);
        let mut hz_soft = hz;
        kernels::gaussian_planar(&mut hz_soft, ew, eh, 1, haze_sigma, haze_sigma, kernels::Edge::Clamp);
        let mut out = vec![0f32; w * h];
        let m = mb as usize;
        out.par_chunks_mut(w).enumerate().for_each(|(j, row)| {
            for (i, o) in row.iter_mut().enumerate() {
                let k = (j + m) * ew + i + m;
                *o = (lk.mix.0 * wisps[k] + lk.mix.1 * soft[k] + lk.mix.2 * hz_soft[k]).min(1.0);
            }
        });
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_has_wisps_and_gaps_and_is_deterministic() {
        let sm = Smoke::new(3, 120.0, 90.0, 1.8, 1.0, 5, 0.5, Basis::Perlin, None);
        let p = sm.plane(10, 20, 160, 200);
        assert_eq!(p.len(), 160 * 200);
        assert!(p.iter().all(|v| v.is_finite() && (0.0..=1.0).contains(v)));
        let dark = p.iter().filter(|v| **v < 0.05).count() as f64 / p.len() as f64;
        let lit = p.iter().filter(|v| **v > 0.3).count() as f64 / p.len() as f64;
        assert!(dark > 0.1 && lit > 0.02, "dark {dark} lit {lit}");
        assert_eq!(p, sm.plane(10, 20, 160, 200));
        // a sub-rectangle matches the same pixels of a larger render
        let q = sm.plane(30, 40, 40, 40);
        for y in 0..40 {
            for x in 0..40 {
                assert!((q[y * 40 + x] - p[(y + 20) * 160 + x + 20]).abs() < 0.02);
            }
        }
    }
}
