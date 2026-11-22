//! Liquid as a 2.5-D metaball surface: tapered capsules and spheres add up to a smooth
//! field whose iso-line is the outline, so neighboring strands merge with the soft
//! bridges real liquid makes. A height map grows from the outline like a tube or a
//! dome (scaled by the local strand radius), and its slopes give the normals, so merged
//! sheets keep gentle undulations. The shader lights it like a glossy liquid: a tinted,
//! thickness-dependent body, a darker refracting rim, a Fresnel edge, a bright caustic
//! opposite the light, and specular highlights.

use pixelox_core::prelude::*;
use rayon::prelude::*;

/// Iso level of the field: the outline.
pub(crate) const T: f64 = 0.3;

/// Normalized radius at which one isolated primitive reaches the iso level.
fn s_iso() -> f64 {
    (1.0 - T.cbrt()).sqrt()
}

/// A tapered capsule from `a` (radius `ra`) to `b` (radius `rb`); a sphere when a = b.
/// Radii are the visible radii (where the field crosses the outline on its own).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Prim {
    pub a: (f64, f64),
    pub b: (f64, f64),
    pub ra: f64,
    pub rb: f64,
    /// Primitives of one nonzero group (consecutive in the list) combine by their
    /// maximum, so a chain of segments has smooth joints; groups and ungrouped (0)
    /// primitives add up, so separate bodies merge with liquid bridges.
    pub group: u32,
}

impl Prim {
    pub fn sphere(c: (f64, f64), r: f64) -> Prim {
        Prim { a: c, b: c, ra: r, rb: r, group: 0 }
    }

    pub fn capsule(a: (f64, f64), b: (f64, f64), ra: f64, rb: f64) -> Prim {
        Prim { a, b, ra, rb, group: 0 }
    }

    pub fn in_group(self, group: u32) -> Prim {
        Prim { group, ..self }
    }

    /// Influence radius (field support) at the larger end.
    fn reach(&self) -> f64 {
        self.ra.max(self.rb) / s_iso()
    }

    /// The field contribution at p and the visible radius there.
    #[inline]
    fn at(&self, p: (f64, f64)) -> (f64, f64) {
        let (dx, dy) = (self.b.0 - self.a.0, self.b.1 - self.a.1);
        let l2 = dx * dx + dy * dy;
        let t = if l2 > 1e-12 { (((p.0 - self.a.0) * dx + (p.1 - self.a.1) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
        let (qx, qy) = (self.a.0 + dx * t - p.0, self.a.1 + dy * t - p.1);
        let rv = self.ra + (self.rb - self.ra) * t;
        let r = rv / s_iso();
        let s2 = (qx * qx + qy * qy) / (r * r).max(1e-12);
        if s2 >= 1.0 {
            (0.0, rv)
        } else {
            let k = 1.0 - s2;
            (k * k * k, rv)
        }
    }
}

/// The summed field over a box of the canvas, and the height map derived from it.
pub(crate) struct Field {
    pub x0: i32,
    pub y0: i32,
    pub w: usize,
    pub h: usize,
    pub v: Vec<f32>,
    /// Height above the canvas in px (a dome or tube profile).
    pub height: Vec<f32>,
}

/// Evaluate the field of `prims` over their bounding box (clipped to `clip`, a canvas
/// rectangle with a margin), binning primitives into 32 px tiles.
pub(crate) fn field(prims: &[Prim], clip: RectI) -> Option<Field> {
    if prims.is_empty() {
        return None;
    }
    let (mut x0, mut y0, mut x1, mut y1) = (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for p in prims {
        let r = p.reach();
        x0 = x0.min(p.a.0.min(p.b.0) - r);
        y0 = y0.min(p.a.1.min(p.b.1) - r);
        x1 = x1.max(p.a.0.max(p.b.0) + r);
        y1 = y1.max(p.a.1.max(p.b.1) + r);
    }
    let bx0 = (x0.floor() as i32 - 2).max(clip.x);
    let by0 = (y0.floor() as i32 - 2).max(clip.y);
    let bx1 = (x1.ceil() as i32 + 2).min(clip.x + clip.w as i32);
    let by1 = (y1.ceil() as i32 + 2).min(clip.y + clip.h as i32);
    if bx1 <= bx0 || by1 <= by0 {
        return None;
    }
    let (w, h) = ((bx1 - bx0) as usize, (by1 - by0) as usize);
    const TILE: usize = 32;
    let (tw, th) = (w.div_ceil(TILE), h.div_ceil(TILE));
    let mut bins: Vec<Vec<u32>> = vec![Vec::new(); tw * th];
    for (i, p) in prims.iter().enumerate() {
        let r = p.reach();
        let tx0 = ((p.a.0.min(p.b.0) - r - bx0 as f64).floor().max(0.0) as usize / TILE).min(tw - 1);
        let ty0 = ((p.a.1.min(p.b.1) - r - by0 as f64).floor().max(0.0) as usize / TILE).min(th - 1);
        let tx1 = ((p.a.0.max(p.b.0) + r - bx0 as f64).ceil().max(0.0) as usize / TILE).min(tw - 1);
        let ty1 = ((p.a.1.max(p.b.1) + r - by0 as f64).ceil().max(0.0) as usize / TILE).min(th - 1);
        for ty in ty0..=ty1 {
            for tx in tx0..=tx1 {
                bins[ty * tw + tx].push(i as u32);
            }
        }
    }
    let mut v = vec![0f32; w * h];
    let mut height = vec![0f32; w * h];
    v.par_chunks_mut(w).zip(height.par_chunks_mut(w)).enumerate().for_each(|(y, (row, hrow))| {
        let cy = by0 as f64 + y as f64 + 0.5;
        for x in 0..w {
            let cx = bx0 as f64 + x as f64 + 0.5;
            let bin = &bins[(y / TILE) * tw + x / TILE];
            let (mut s, mut sr) = (0.0, 0.0);
            // the running maximum of the current group
            let (mut g, mut gf, mut gr) = (0u32, 0.0f64, 0.0f64);
            for &i in bin {
                let p = &prims[i as usize];
                let (f, r) = p.at((cx, cy));
                if p.group != 0 && p.group == g {
                    if f > gf {
                        (gf, gr) = (f, r);
                    }
                    continue;
                }
                s += gf;
                sr += gf * gr;
                if p.group != 0 {
                    (g, gf, gr) = (p.group, f, r);
                } else {
                    (g, gf, gr) = (0, 0.0, 0.0);
                    s += f;
                    sr += f * r;
                }
            }
            s += gf;
            sr += gf * gr;
            row[x] = s as f32;
            if s > T {
                // a dome about as high as the local strand radius, vertical at the outline
                let r = sr / s;
                hrow[x] = (r * ((s - T).min(1.0 - T) / (1.0 - T)).sqrt()) as f32;
            }
        }
    });
    Some(Field { x0: bx0, y0: by0, w, h, v, height })
}

/// The surface at one field pixel.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Surf {
    /// Anti-aliased coverage 0..1.
    pub cov: f32,
    /// Tilt of the surface: 0 facing the viewer, 1 edge-on (at the outline).
    pub s: f32,
    /// Unit direction the surface leans toward (in the image plane: away from the core).
    pub dir: (f32, f32),
    /// The normal (x, y toward the edge, z toward the viewer).
    pub n: [f32; 3],
    /// Distance to the outline in px.
    pub inset: f32,
}

impl Field {
    /// The surface at field pixel (x, y); `None` outside the liquid.
    pub(crate) fn surface(&self, x: usize, y: usize) -> Option<Surf> {
        let (w, h) = (self.w as i64, self.h as i64);
        let at = |m: &[f32], x: i64, y: i64| m[(y.clamp(0, h - 1) as usize) * self.w + x.clamp(0, w - 1) as usize];
        let v = self.v[y * self.w + x];
        let (xi, yi) = (x as i64, y as i64);
        let gx = (at(&self.v, xi + 1, yi) - at(&self.v, xi - 1, yi)) * 0.5;
        let gy = (at(&self.v, xi, yi + 1) - at(&self.v, xi, yi - 1)) * 0.5;
        let g = (gx * gx + gy * gy).sqrt();
        let inset = (v - T as f32) / g.max(1e-4);
        let cov = (inset + 0.5).clamp(0.0, 1.0);
        if cov <= 0.0 {
            return None;
        }
        let hx = (at(&self.height, xi + 1, yi) - at(&self.height, xi - 1, yi)) * 0.5;
        let hy = (at(&self.height, xi, yi + 1) - at(&self.height, xi, yi - 1)) * 0.5;
        let m = (hx * hx + hy * hy + 1.0).sqrt();
        let (mut nx, mut ny, mut nz) = (-hx / m, -hy / m, 1.0 / m);
        // in the outermost pixel the height map is cut off: lean along the field gradient
        if inset < 1.0 && g > 1e-6 {
            let k = (1.0 - inset.max(0.0)).clamp(0.0, 1.0);
            let (ex, ey) = (-gx / g, -gy / g);
            nx = nx * (1.0 - k) + ex * 0.97 * k;
            ny = ny * (1.0 - k) + ey * 0.97 * k;
            nz = nz * (1.0 - k) + 0.24 * k;
            let mm = (nx * nx + ny * ny + nz * nz).sqrt();
            (nx, ny, nz) = (nx / mm, ny / mm, nz / mm);
        }
        let s = (nx * nx + ny * ny).sqrt().min(1.0);
        let dir = if s > 1e-6 { (nx / s, ny / s) } else { (0.0, 0.0) };
        Some(Surf { cov, s, dir, n: [nx, ny, nz], inset })
    }
}

/// How a liquid looks.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Look {
    /// Body color (straight sRGB).
    pub color: [f32; 3],
    /// Color of the refracting rim and the thickest parts.
    pub deep: [f32; 3],
    /// Highlight color.
    pub shine: [f32; 3],
    /// Opacity of the body where it faces the viewer and at the rim, 0..1.
    pub alpha_center: f32,
    pub alpha_rim: f32,
    /// 0 = a clear liquid (caustic and rim dominate), 1 = opaque and diffusely lit.
    pub opaque: f32,
    /// Overall opacity multiplier (the --opacity flag).
    pub opacity: f32,
}

#[inline]
pub(crate) fn smooth(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Source-over `c` at alpha `a` onto a straight-alpha pixel.
#[inline]
pub(crate) fn over(dst: &mut [f32; 4], c: [f32; 3], a: f32) {
    if a <= 0.0 {
        return;
    }
    let ao = a + dst[3] * (1.0 - a);
    if ao <= 1e-6 {
        return;
    }
    for i in 0..3 {
        dst[i] = (c[i] * a + dst[i] * dst[3] * (1.0 - a)) / ao;
    }
    dst[3] = ao;
}

pub(crate) fn norm3(v: [f32; 3]) -> [f32; 3] {
    let m = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / m, v[1] / m, v[2] / m]
}

#[inline]
pub(crate) fn dot(a: [f32; 3], b: [f32; 3]) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Shade a field into a straight-alpha image (placed at the field's origin).
pub(crate) fn shade(f: &Field, look: &Look) -> Rendered {
    let w = f.w;
    // the key light: upper left, toward the viewer; a weaker rim light from the right
    let l = norm3([-0.5, -0.65, 0.58]);
    let hv = norm3([l[0], l[1], l[2] + 1.0]);
    let l2 = norm3([0.75, -0.2, 0.62]);
    let hv2 = norm3([l2[0], l2[1], l2[2] + 1.0]);
    let lxy = (l[0] * l[0] + l[1] * l[1]).sqrt();
    let mut img = Image::new(w as u32, f.h as u32);
    img.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let Some(sf) = f.surface(x, y) else { continue };
            let (n, s) = (sf.n, sf.s);
            let ndl = dot(n, l).max(0.0);
            let rim = smooth(0.5, 0.97, s);
            let fres = 0.04 + 0.96 * (1.0 - n[2]).max(0.0).powi(5);
            let mut px = [0f32; 4];
            // body: clear liquids are see-through where they face the viewer, dense at the rim
            let a_body = (look.alpha_center + (look.alpha_rim - look.alpha_center) * rim) * (1.0 - look.opaque)
                + look.opaque * (0.95 + 0.05 * rim);
            // diffuse: 1 on a flat face, darker turned away from the light, a little brighter toward it
            let diffuse = ((ndl - l[2]) / l[2]).clamp(-1.0, 0.25);
            let lit = 1.0 + look.opaque * 0.55 * diffuse + (1.0 - look.opaque) * 0.15 * diffuse;
            let mut body = [0f32; 3];
            for (i, b) in body.iter_mut().enumerate() {
                let c = look.color[i] * lit;
                *b = (c + (look.deep[i] - c) * rim * (0.8 - 0.45 * look.opaque)).clamp(0.0, 1.0);
            }
            over(&mut px, body, a_body);
            // caustic: light focused through the body onto the side away from the light
            let away = -(sf.dir.0 * l[0] + sf.dir.1 * l[1]) / lxy;
            let caustic = smooth(0.15, 0.9, away)
                * smooth(0.25, 0.75, s)
                * (1.0 - smooth(0.86, 0.99, s))
                * (1.0 - look.opaque * 0.75);
            let ccol = [
                (look.color[0] * 0.35 + look.shine[0] * 0.65).min(1.0),
                (look.color[1] * 0.35 + look.shine[1] * 0.65).min(1.0),
                (look.color[2] * 0.35 + look.shine[2] * 0.65).min(1.0),
            ];
            over(&mut px, ccol, caustic * 0.6);
            // Fresnel: grazing edges reflect a bright sky above
            let sky = smooth(-0.6, 0.5, -n[1]);
            over(&mut px, look.shine, fres * 0.65 * sky);
            // specular highlights: a sharp key, a softer rim light, and a broad sheen
            let spec = dot(n, hv).max(0.0).powf(60.0);
            let spec2 = dot(n, hv2).max(0.0).powf(120.0) * 0.55;
            let sheen = dot(n, hv).max(0.0).powf(10.0) * 0.16;
            over(&mut px, look.shine, (spec + spec2 + sheen).min(1.0));
            px[3] *= sf.cov * look.opacity;
            row[x * 4..x * 4 + 4].copy_from_slice(&px);
        }
    });
    Rendered { image: img, x: f.x0, y: f.y0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_sphere_reaches_the_outline_at_its_radius() {
        let p = Prim::sphere((50.0, 50.0), 20.0);
        assert!((p.at((70.0, 50.0)).0 - T).abs() < 1e-9);
        assert!(p.at((60.0, 50.0)).0 > T && p.at((75.0, 50.0)).0 < T);
        let f = field(&[p], RectI::new(0, 0, 100, 100)).unwrap();
        let i = ((50 - f.y0) as usize) * f.w + (50 - f.x0) as usize;
        assert!(f.v[i] > 0.99);
        // a dome about as high as the radius, facing the viewer at the top
        assert!(f.height[i] > 15.0 && f.height[i] < 25.0, "{}", f.height[i]);
        let top = f.surface((50 - f.x0) as usize, (50 - f.y0) as usize).unwrap();
        assert!(top.n[2] > 0.95);
        let edge = f.surface((69 - f.x0) as usize, (50 - f.y0) as usize).unwrap();
        assert!(edge.dir.0 > 0.9 && edge.s > 0.6, "{edge:?}");
    }

    #[test]
    fn chains_have_smooth_joints() {
        // two collinear segments of one group: no bulge at the joint
        let ps = [
            Prim::capsule((20.0, 50.0), (50.0, 50.0), 8.0, 8.0).in_group(1),
            Prim::capsule((50.0, 50.0), (80.0, 50.0), 8.0, 8.0).in_group(1),
        ];
        let f = field(&ps, RectI::new(0, 0, 100, 100)).unwrap();
        let at = |x: i32, y: i32| f.v[((y - f.y0) as usize) * f.w + (x - f.x0) as usize];
        assert!((at(50, 56) - at(35, 56)).abs() < 1e-3);
    }

    #[test]
    fn neighbors_merge_into_one_body() {
        // two spheres a little more than their radii apart are bridged
        let ps = [Prim::sphere((40.0, 50.0), 12.0), Prim::sphere((66.0, 50.0), 12.0)];
        let f = field(&ps, RectI::new(0, 0, 120, 100)).unwrap();
        let mid = f.v[((50 - f.y0) as usize) * f.w + (53 - f.x0) as usize] as f64;
        assert!(mid > T, "{mid}");
    }
}
