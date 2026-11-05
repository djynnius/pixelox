//! Cameras, projection, a z-buffer rasterizer, and the three styles: wireframe (every
//! modelled edge), hidden-line (front-facing creases, silhouettes, and boundaries, cut
//! where the z-buffer hides them), and shaded (flat Lambert + ambient).

use super::mesh::{cross, dot, len, mul, norm, sub, Edge, Mesh, V3};
use pixelox_core::color::{linear_to_srgb, srgb_to_linear, Rgba};
use pixelox_core::image::Image;

/// Edges whose faces meet at more than this angle are drawn as creases.
const CREASE_COS: f64 = 0.866; // 30°

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Style {
    Wireframe,
    HiddenLine,
    Shaded,
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub persp: bool,
    /// Direction from the target toward the eye (unit).
    pub dir: V3,
    pub up: V3,
    /// Explicit eye position (perspective scene cameras).
    pub eye: Option<V3>,
    pub target: Option<V3>,
    pub fov_deg: f64,
}

impl Camera {
    pub fn preset(name: &str) -> Camera {
        let e = |az: f64, el: f64| -> V3 {
            let (az, el) = (az.to_radians(), el.to_radians());
            [el.cos() * az.sin(), el.sin(), el.cos() * az.cos()]
        };
        let (dir, up) = match name {
            "front" => ([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]),
            "top" => ([0.0, 1.0, 0.0], [0.0, 0.0, -1.0]),
            "dimetric" => (e(45.0, 30.0), [0.0, 1.0, 0.0]),
            "persp" => (e(35.0, 25.0), [0.0, 1.0, 0.0]),
            _ => (norm([1.0, 1.0, 1.0]), [0.0, 1.0, 0.0]),
        };
        Camera { persp: name == "persp", dir, up, eye: None, target: None, fov_deg: 40.0 }
    }
}

/// Lighting: a directional light (toward the light, world space) plus ambient.
#[derive(Clone, Debug)]
pub struct Light {
    /// `None`: a headlight from the camera's upper left.
    pub dir: Option<V3>,
    pub ambient: f64,
    pub intensity: f64,
}

impl Default for Light {
    fn default() -> Self {
        Light { dir: None, ambient: 0.3, intensity: 0.75 }
    }
}

/// A camera fitted to a mesh and an output size.
pub struct View {
    eye: V3,
    right: V3,
    up: V3,
    fwd: V3,
    persp: bool,
    near: f64,
    // 2D fit
    s: f64,
    cx: f64,
    cy: f64,
    w: f64,
    h: f64,
}

impl View {
    pub fn fit(mesh: &Mesh, cam: &Camera, w: u32, h: u32, margin: f64) -> View {
        let (lo, hi) = mesh.bounds();
        let center = mul([lo[0] + hi[0], lo[1] + hi[1], lo[2] + hi[2]], 0.5);
        let target = cam.target.unwrap_or(center);
        let radius = mesh.verts.iter().map(|v| len(sub(*v, target))).fold(1e-9, f64::max);
        let half = (cam.fov_deg.clamp(1.0, 170.0) / 2.0).to_radians();
        let eye = match cam.eye {
            Some(e) => e,
            None if cam.persp => sub(target, mul(cam.dir, -(radius / half.sin() * 1.05))),
            None => sub(target, mul(cam.dir, -(radius * 4.0))),
        };
        let fwd = norm(sub(target, eye));
        let mut up = cam.up;
        if len(cross(fwd, up)) < 1e-6 {
            up = if fwd[1].abs() > 0.9 { [0.0, 0.0, -fwd[1].signum()] } else { [0.0, 1.0, 0.0] };
        }
        let right = norm(cross(fwd, up));
        let up = cross(right, fwd);
        let mut v = View {
            eye,
            right,
            up,
            fwd,
            persp: cam.persp,
            near: radius * 1e-3,
            s: 1.0,
            cx: 0.0,
            cy: 0.0,
            w: w as f64,
            h: h as f64,
        };
        let mut lo2 = [f64::INFINITY; 2];
        let mut hi2 = [f64::NEG_INFINITY; 2];
        for p in &mesh.verts {
            if let Some((x, y, _)) = v.project_raw(*p) {
                lo2 = [lo2[0].min(x), lo2[1].min(y)];
                hi2 = [hi2[0].max(x), hi2[1].max(y)];
            }
        }
        let bw = (hi2[0] - lo2[0]).max(1e-12);
        let bh = (hi2[1] - lo2[1]).max(1e-12);
        v.s = ((w as f64 - 2.0 * margin).max(1.0) / bw).min((h as f64 - 2.0 * margin).max(1.0) / bh);
        v.cx = (lo2[0] + hi2[0]) / 2.0;
        v.cy = (lo2[1] + hi2[1]) / 2.0;
        v
    }

    /// Camera-plane coordinates (y down) and "closeness" (larger = nearer), which is
    /// affine in screen space for planar triangles in both projections.
    fn project_raw(&self, p: V3) -> Option<(f64, f64, f64)> {
        let d = sub(p, self.eye);
        let (x, y, z) = (dot(d, self.right), dot(d, self.up), dot(d, self.fwd));
        if self.persp {
            if z <= self.near {
                return None;
            }
            Some((x / z, -y / z, 1.0 / z))
        } else {
            Some((x, -y, -z))
        }
    }

    /// Output pixel coordinates and closeness.
    pub fn project(&self, p: V3) -> Option<[f64; 3]> {
        let (x, y, c) = self.project_raw(p)?;
        Some([(x - self.cx) * self.s + self.w / 2.0, (y - self.cy) * self.s + self.h / 2.0, c])
    }

    /// Does a face with normal `n` through point `p` face the camera?
    pub fn front_facing(&self, n: V3, p: V3) -> bool {
        if self.persp {
            dot(n, sub(self.eye, p)) > 0.0
        } else {
            dot(n, mul(self.fwd, -1.0)) > 0.0
        }
    }

    fn headlight(&self) -> V3 {
        norm([
            -0.45 * self.right[0] + 0.75 * self.up[0] - 0.55 * self.fwd[0],
            -0.45 * self.right[1] + 0.75 * self.up[1] - 0.55 * self.fwd[1],
            -0.45 * self.right[2] + 0.75 * self.up[2] - 0.55 * self.fwd[2],
        ])
    }
}

/// A z-buffer (closeness, larger = nearer) with the face id of each pixel.
pub struct ZBuf {
    pub w: usize,
    pub h: usize,
    pub z: Vec<f32>,
    pub id: Vec<u32>,
}

impl ZBuf {
    fn new(w: usize, h: usize) -> ZBuf {
        ZBuf { w, h, z: vec![f32::NEG_INFINITY; w * h], id: vec![u32::MAX; w * h] }
    }

    fn tri(&mut self, p: [[f64; 3]; 3], face: u32) {
        let area = (p[1][0] - p[0][0]) * (p[2][1] - p[0][1]) - (p[1][1] - p[0][1]) * (p[2][0] - p[0][0]);
        if area.abs() < 1e-12 {
            return;
        }
        let minx = p.iter().map(|q| q[0]).fold(f64::INFINITY, f64::min).floor().max(0.0) as usize;
        let miny = p.iter().map(|q| q[1]).fold(f64::INFINITY, f64::min).floor().max(0.0) as usize;
        let maxx = (p.iter().map(|q| q[0]).fold(f64::NEG_INFINITY, f64::max).ceil() as i64).min(self.w as i64 - 1);
        let maxy = (p.iter().map(|q| q[1]).fold(f64::NEG_INFINITY, f64::max).ceil() as i64).min(self.h as i64 - 1);
        if maxx < 0 || maxy < 0 {
            return;
        }
        let inv = 1.0 / area;
        for y in miny..=maxy as usize {
            let py = y as f64 + 0.5;
            for x in minx..=maxx as usize {
                let px = x as f64 + 0.5;
                let w0 = ((p[1][0] - px) * (p[2][1] - py) - (p[1][1] - py) * (p[2][0] - px)) * inv;
                let w1 = ((p[2][0] - px) * (p[0][1] - py) - (p[2][1] - py) * (p[0][0] - px)) * inv;
                let w2 = 1.0 - w0 - w1;
                if w0 < -1e-9 || w1 < -1e-9 || w2 < -1e-9 {
                    continue;
                }
                let z = (w0 * p[0][2] + w1 * p[1][2] + w2 * p[2][2]) as f32;
                let i = y * self.w + x;
                if z > self.z[i] {
                    self.z[i] = z;
                    self.id[i] = face;
                }
            }
        }
    }
}

/// A projected scene: screen positions, triangles, edges, and face data.
pub struct Scene<'a> {
    pub mesh: &'a Mesh,
    pub view: View,
    pub screen: Vec<Option<[f64; 3]>>,
    pub tris: Vec<[u32; 3]>,
    pub tri_face: Vec<u32>,
    pub edges: Vec<Edge>,
    pub normals: Vec<V3>,
    pub front: Vec<bool>,
    pub closed: bool,
}

impl<'a> Scene<'a> {
    pub fn new(mesh: &'a Mesh, cam: &Camera, w: u32, h: u32, margin: f64) -> Scene<'a> {
        let view = View::fit(mesh, cam, w, h, margin);
        let screen: Vec<Option<[f64; 3]>> = mesh.verts.iter().map(|v| view.project(*v)).collect();
        let (tris, tri_face) = mesh.triangulate();
        let edges = mesh.edges();
        let normals: Vec<V3> = mesh.faces.iter().map(|f| mesh.face_normal(f)).collect();
        let front: Vec<bool> =
            mesh.faces.iter().zip(&normals).map(|(f, n)| view.front_facing(*n, mesh.centroid(f))).collect();
        let closed = Mesh::is_closed(&edges);
        Scene { mesh, view, screen, tris, tri_face, edges, normals, front, closed }
    }

    fn zbuffer(&self, scale: f64) -> ZBuf {
        let mut zb = ZBuf::new((self.view.w * scale).ceil() as usize, (self.view.h * scale).ceil() as usize);
        for (t, &f) in self.tris.iter().zip(&self.tri_face) {
            let (Some(a), Some(b), Some(c)) =
                (self.screen[t[0] as usize], self.screen[t[1] as usize], self.screen[t[2] as usize])
            else {
                continue;
            };
            let s = |p: [f64; 3]| [p[0] * scale, p[1] * scale, p[2]];
            zb.tri([s(a), s(b), s(c)], f);
        }
        zb
    }

    /// Every modelled edge as a 2D segment.
    pub fn wireframe(&self) -> Vec<[f64; 4]> {
        self.edges
            .iter()
            .filter_map(|e| {
                let (a, b) = (self.screen[e.a as usize]?, self.screen[e.b as usize]?);
                Some([a[0], a[1], b[0], b[1]])
            })
            .collect()
    }

    fn feature(&self, e: &Edge, all: bool) -> bool {
        if all {
            return e.faces.iter().any(|f| self.front[*f as usize]) || !self.closed;
        }
        match e.faces.as_slice() {
            [f] => self.front[*f as usize] || !self.closed,
            [f, g] => {
                let (ff, fg) = (self.front[*f as usize], self.front[*g as usize]);
                if ff != fg {
                    return true; // silhouette
                }
                ff && dot(self.normals[*f as usize], self.normals[*g as usize]) < CREASE_COS
            }
            _ => e.faces.iter().any(|f| self.front[*f as usize]),
        }
    }

    /// Visible parts of feature edges (creases, silhouettes, boundaries), or of every
    /// edge touching a front face when `all`.
    pub fn hidden_line(&self, all: bool) -> Vec<[f64; 4]> {
        let scale = if self.view.w.max(self.view.h) <= 2048.0 { 2.0 } else { 1.0 };
        let zb = self.zbuffer(scale);
        let (mut zlo, mut zhi) = (f64::INFINITY, f64::NEG_INFINITY);
        for p in self.screen.iter().flatten() {
            zlo = zlo.min(p[2]);
            zhi = zhi.max(p[2]);
        }
        let eps = ((zhi - zlo) * 2e-3).max(1e-12);
        let mut out = Vec::new();
        for e in &self.edges {
            if !self.feature(e, all) {
                continue;
            }
            let (Some(a), Some(b)) = (self.screen[e.a as usize], self.screen[e.b as usize]) else { continue };
            let (ax, ay, bx, by) = (a[0] * scale, a[1] * scale, b[0] * scale, b[1] * scale);
            let l = ((bx - ax).powi(2) + (by - ay).powi(2)).sqrt();
            let n = ((l / 0.5).ceil() as usize).max(1) + 1;
            let vis: Vec<bool> = (0..n)
                .map(|k| {
                    let t = k as f64 / (n - 1) as f64;
                    let (x, y) = (ax + (bx - ax) * t, ay + (by - ay) * t);
                    let c = a[2] + (b[2] - a[2]) * t;
                    // visible when any pixel of the 3×3 neighbourhood is empty, belongs to
                    // the edge's own faces, or is not nearer: robust at grazing angles and
                    // silhouettes, overrunning occluders by at most one z-buffer pixel
                    let (xi, yi) = (x.floor() as i64, y.floor() as i64);
                    (-1..=1).any(|dy| {
                        (-1..=1).any(|dx| {
                            let (px, py) = (xi + dx, yi + dy);
                            if px < 0 || py < 0 || px >= zb.w as i64 || py >= zb.h as i64 {
                                return true;
                            }
                            let i = py as usize * zb.w + px as usize;
                            zb.id[i] == u32::MAX || e.faces.contains(&zb.id[i]) || (zb.z[i] as f64) <= c + eps
                        })
                    })
                })
                .collect();
            for (t0, t1) in runs(&vis) {
                let p = |t: f64| [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t];
                let (p0, p1) = (p(t0), p(t1));
                out.push([p0[0], p0[1], p1[0], p1[1]]);
            }
        }
        out
    }

    /// Flat-shaded raster (straight alpha) with 2×2 supersampling.
    pub fn shaded(&self, light: &Light) -> Image {
        let ss = 2usize;
        let zb = self.zbuffer(ss as f64);
        let l = light.dir.map(norm).unwrap_or_else(|| self.view.headlight());
        let colors: Vec<[f32; 3]> = self
            .mesh
            .faces
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let mut n = self.normals[i];
                if !self.front[i] {
                    n = mul(n, -1.0);
                }
                let k = (light.ambient + light.intensity * dot(n, l).max(0.0)) as f32;
                let c = f.color;
                [srgb_to_linear(c.r) * k, srgb_to_linear(c.g) * k, srgb_to_linear(c.b) * k]
            })
            .collect();
        let (w, h) = (self.view.w as u32, self.view.h as u32);
        Image::from_fn(w, h, |x, y| {
            let mut acc = [0f32; 3];
            let mut cov = 0f32;
            for sy in 0..ss {
                for sx in 0..ss {
                    let (px, py) = (x as usize * ss + sx, y as usize * ss + sy);
                    if px >= zb.w || py >= zb.h {
                        continue;
                    }
                    let id = zb.id[py * zb.w + px];
                    if id != u32::MAX {
                        let c = colors[id as usize];
                        acc = [acc[0] + c[0], acc[1] + c[1], acc[2] + c[2]];
                        cov += 1.0;
                    }
                }
            }
            if cov == 0.0 {
                return [0.0; 4];
            }
            let a = cov / (ss * ss) as f32;
            [
                linear_to_srgb((acc[0] / cov).min(1.0)),
                linear_to_srgb((acc[1] / cov).min(1.0)),
                linear_to_srgb((acc[2] / cov).min(1.0)),
                a,
            ]
        })
    }

    /// Faces back to front for painter's-algorithm SVG (back faces culled on closed meshes).
    pub fn painter_faces(&self, light: &Light) -> Vec<(Vec<Vec<[f64; 2]>>, Rgba)> {
        let l = light.dir.map(norm).unwrap_or_else(|| self.view.headlight());
        let mut order: Vec<(f64, usize)> = Vec::new();
        for (i, f) in self.mesh.faces.iter().enumerate() {
            if self.closed && !self.front[i] {
                continue;
            }
            let mut zsum = 0.0;
            let mut ok = true;
            for &v in &f.loops[0] {
                match self.screen[v as usize] {
                    Some(p) => zsum += p[2],
                    None => ok = false,
                }
            }
            if ok {
                order.push((zsum / f.loops[0].len() as f64, i));
            }
        }
        order.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        order
            .into_iter()
            .map(|(_, i)| {
                let f = &self.mesh.faces[i];
                let mut n = self.normals[i];
                if !self.front[i] {
                    n = mul(n, -1.0);
                }
                let k = (light.ambient + light.intensity * dot(n, l).max(0.0)) as f32;
                let c = f.color;
                let shade = |v: f32| linear_to_srgb((srgb_to_linear(v) * k).min(1.0));
                let loops = f
                    .loops
                    .iter()
                    .map(|lp| lp.iter().filter_map(|&v| self.screen[v as usize].map(|p| [p[0], p[1]])).collect())
                    .collect();
                (loops, Rgba::new(shade(c.r), shade(c.g), shade(c.b), c.a))
            })
            .collect()
    }
}

/// Visible runs of a sample row as parameter intervals, with one-sample gaps closed and
/// specks dropped.
fn runs(vis: &[bool]) -> Vec<(f64, f64)> {
    let n = vis.len();
    if n == 0 {
        return vec![];
    }
    let mut v = vis.to_vec();
    // close single-sample gaps between visible samples
    for k in 1..n.saturating_sub(1) {
        if !v[k] && vis[k - 1] && vis[k + 1] {
            v[k] = true;
        }
    }
    let t = |k: usize| k as f64 / (n - 1).max(1) as f64;
    let mut out = Vec::new();
    let mut k = 0;
    while k < n {
        if !v[k] {
            k += 1;
            continue;
        }
        let s = k;
        while k < n && v[k] {
            k += 1;
        }
        let e = k - 1;
        let t0 = if s == 0 { 0.0 } else { (t(s - 1) + t(s)) / 2.0 };
        let t1 = if e == n - 1 { 1.0 } else { (t(e) + t(e + 1)) / 2.0 };
        if e > s || n <= 3 {
            out.push((t0, t1));
        }
    }
    out
}

/// Chain segments that share endpoints into polylines.
pub fn polylines(segs: &[[f64; 4]]) -> Vec<Vec<[f64; 2]>> {
    let mut out: Vec<Vec<[f64; 2]>> = Vec::new();
    for s in segs {
        let (a, b) = ([s[0], s[1]], [s[2], s[3]]);
        if let Some(last) = out.last_mut() {
            let end = *last.last().unwrap();
            if (end[0] - a[0]).abs() < 1e-6 && (end[1] - a[1]).abs() < 1e-6 {
                last.push(b);
                continue;
            }
        }
        out.push(vec![a, b]);
    }
    out
}

pub fn fmt_num(v: f64) -> String {
    let r = (v * 100.0).round() / 100.0;
    let s = format!("{r:.2}");
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" {
        "0".into()
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_cover_and_split() {
        assert_eq!(runs(&[true, true, true]), vec![(0.0, 1.0)]);
        let r = runs(&[true, true, false, false, false, true, true]);
        assert_eq!(r.len(), 2);
        assert!(r[0].1 < r[1].0);
        assert_eq!(runs(&[false, false, false]), vec![]);
        // a one-sample gap is closed
        assert_eq!(runs(&[true, true, false, true, true]), vec![(0.0, 1.0)]);
        assert_eq!(fmt_num(1.001), "1");
        assert_eq!(fmt_num(-0.001), "0");
        assert_eq!(fmt_num(12.5), "12.5");
    }
}
