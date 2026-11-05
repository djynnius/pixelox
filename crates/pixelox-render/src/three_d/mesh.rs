//! Polygon meshes: primitives, OBJ import, triangulation (ear clipping with holes), and
//! edge adjacency. Faces are polygons (with optional holes) so wireframes show the
//! modelled edges, not triangulation diagonals.

use pixelox_core::color::Rgba;
use pixelox_core::error::{Error, Result};
use std::collections::BTreeMap;
use std::f64::consts::{PI, TAU};

pub type V3 = [f64; 3];

pub fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
pub fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
pub fn mul(a: V3, s: f64) -> V3 {
    [a[0] * s, a[1] * s, a[2] * s]
}
pub fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
pub fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
pub fn len(a: V3) -> f64 {
    dot(a, a).sqrt()
}
pub fn norm(a: V3) -> V3 {
    let l = len(a);
    if l > 0.0 {
        mul(a, 1.0 / l)
    } else {
        [0.0, 0.0, 0.0]
    }
}

/// A polygon face: `loops[0]` is the outer boundary (counter-clockwise seen from outside),
/// further loops are holes.
#[derive(Clone, Debug)]
pub struct Face {
    pub loops: Vec<Vec<u32>>,
    pub color: Rgba,
}

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub verts: Vec<V3>,
    pub faces: Vec<Face>,
}

/// An edge between two vertices and the faces that use it.
#[derive(Clone, Debug)]
pub struct Edge {
    pub a: u32,
    pub b: u32,
    pub faces: Vec<u32>,
}

impl Mesh {
    fn face(&mut self, idx: Vec<u32>) {
        self.faces.push(Face { loops: vec![idx], color: Rgba::WHITE });
    }

    pub fn append(&mut self, other: Mesh) {
        let off = self.verts.len() as u32;
        self.verts.extend(other.verts);
        for mut f in other.faces {
            for l in f.loops.iter_mut() {
                for i in l.iter_mut() {
                    *i += off;
                }
            }
            self.faces.push(f);
        }
    }

    pub fn map(&mut self, f: impl Fn(V3) -> V3) {
        for v in self.verts.iter_mut() {
            *v = f(*v);
        }
    }

    pub fn set_color(&mut self, c: Rgba) {
        for f in self.faces.iter_mut() {
            f.color = c;
        }
    }

    pub fn bounds(&self) -> (V3, V3) {
        let mut lo = [f64::INFINITY; 3];
        let mut hi = [f64::NEG_INFINITY; 3];
        for v in &self.verts {
            for k in 0..3 {
                lo[k] = lo[k].min(v[k]);
                hi[k] = hi[k].max(v[k]);
            }
        }
        (lo, hi)
    }

    /// Newell normal of a face's outer loop (unit length, or zero when degenerate).
    pub fn face_normal(&self, f: &Face) -> V3 {
        let l = &f.loops[0];
        let mut n = [0.0; 3];
        for i in 0..l.len() {
            let a = self.verts[l[i] as usize];
            let b = self.verts[l[(i + 1) % l.len()] as usize];
            n[0] += (a[1] - b[1]) * (a[2] + b[2]);
            n[1] += (a[2] - b[2]) * (a[0] + b[0]);
            n[2] += (a[0] - b[0]) * (a[1] + b[1]);
        }
        norm(n)
    }

    pub fn centroid(&self, f: &Face) -> V3 {
        let l = &f.loops[0];
        let mut c = [0.0; 3];
        for &i in l {
            c = add(c, self.verts[i as usize]);
        }
        mul(c, 1.0 / l.len().max(1) as f64)
    }

    /// Merge vertices at the same position (so faces share edges) and drop degenerate
    /// loops.
    pub fn weld(&mut self) {
        let (lo, hi) = self.bounds();
        let scale = (0..3).map(|k| hi[k] - lo[k]).fold(0.0f64, f64::max).max(1e-12);
        let q = |v: f64| (v / scale * 1e7).round() as i64;
        let mut map: BTreeMap<(i64, i64, i64), u32> = BTreeMap::new();
        let mut remap = Vec::with_capacity(self.verts.len());
        let mut verts = Vec::new();
        for v in &self.verts {
            let key = (q(v[0]), q(v[1]), q(v[2]));
            let id = *map.entry(key).or_insert_with(|| {
                verts.push(*v);
                (verts.len() - 1) as u32
            });
            remap.push(id);
        }
        self.verts = verts;
        for f in self.faces.iter_mut() {
            for l in f.loops.iter_mut() {
                let mut out: Vec<u32> = Vec::with_capacity(l.len());
                for &i in l.iter() {
                    let r = remap[i as usize];
                    if out.last() != Some(&r) {
                        out.push(r);
                    }
                }
                while out.len() > 1 && out.first() == out.last() {
                    out.pop();
                }
                *l = out;
            }
            let mut loops = std::mem::take(&mut f.loops).into_iter();
            if let Some(outer) = loops.next() {
                f.loops.push(outer);
                f.loops.extend(loops.filter(|l| l.len() >= 3));
            }
        }
        self.faces.retain(|f| f.loops[0].len() >= 3);
    }

    /// Reverse a face's winding (and its holes').
    fn flip(f: &mut Face) {
        for l in f.loops.iter_mut() {
            l.reverse();
        }
    }

    /// Orient every face so its normal points away from `center(centroid)`.
    pub fn orient_from(&mut self, center: impl Fn(V3) -> V3) {
        for i in 0..self.faces.len() {
            let n = self.face_normal(&self.faces[i]);
            let c = self.centroid(&self.faces[i]);
            if dot(n, sub(c, center(c))) < 0.0 {
                Self::flip(&mut self.faces[i]);
            }
        }
    }

    /// Edges with their adjacent faces, in a deterministic order.
    pub fn edges(&self) -> Vec<Edge> {
        let mut map: BTreeMap<(u32, u32), Vec<u32>> = BTreeMap::new();
        for (fi, f) in self.faces.iter().enumerate() {
            for l in &f.loops {
                for i in 0..l.len() {
                    let (a, b) = (l[i], l[(i + 1) % l.len()]);
                    if a == b {
                        continue;
                    }
                    let key = (a.min(b), a.max(b));
                    let e = map.entry(key).or_default();
                    if !e.contains(&(fi as u32)) {
                        e.push(fi as u32);
                    }
                }
            }
        }
        map.into_iter().map(|((a, b), faces)| Edge { a, b, faces }).collect()
    }

    /// True when every edge has exactly two faces.
    pub fn is_closed(edges: &[Edge]) -> bool {
        !edges.is_empty() && edges.iter().all(|e| e.faces.len() == 2)
    }

    /// Flip all faces of a closed mesh whose signed volume is negative (inside-out).
    pub fn fix_inside_out(&mut self) {
        if !Self::is_closed(&self.edges()) {
            return;
        }
        let (tris, _) = self.triangulate();
        let mut vol = 0.0;
        for t in &tris {
            let (a, b, c) = (self.verts[t[0] as usize], self.verts[t[1] as usize], self.verts[t[2] as usize]);
            vol += dot(a, cross(b, c));
        }
        if vol < 0.0 {
            for f in self.faces.iter_mut() {
                Self::flip(f);
            }
        }
    }

    /// Triangles (with the winding of their face) and the face of each triangle.
    pub fn triangulate(&self) -> (Vec<[u32; 3]>, Vec<u32>) {
        let mut tris = Vec::new();
        let mut owner = Vec::new();
        for (fi, f) in self.faces.iter().enumerate() {
            let outer = &f.loops[0];
            if f.loops.len() == 1 && outer.len() == 3 {
                tris.push([outer[0], outer[1], outer[2]]);
                owner.push(fi as u32);
                continue;
            }
            let n = self.face_normal(f);
            // project onto the plane most aligned with the normal, keeping orientation
            let k = (0..3).max_by(|&a, &b| n[a].abs().total_cmp(&n[b].abs())).unwrap_or(2);
            let (mut i, mut j) = ((k + 1) % 3, (k + 2) % 3);
            if n[k] < 0.0 {
                std::mem::swap(&mut i, &mut j);
            }
            let mut ids: Vec<u32> = Vec::new();
            let mut pts: Vec<[f64; 2]> = Vec::new();
            let mut loops: Vec<Vec<usize>> = Vec::new();
            for l in &f.loops {
                let mut li = Vec::new();
                for &v in l {
                    let p = self.verts[v as usize];
                    li.push(pts.len());
                    pts.push([p[i], p[j]]);
                    ids.push(v);
                }
                loops.push(li);
            }
            let convex = f.loops.len() == 1 && is_convex(&pts);
            let local = if convex {
                (1..pts.len() - 1).map(|q| [0, q, q + 1]).collect()
            } else {
                triangulate(&pts, &loops[0], &loops[1..])
            };
            for t in local {
                tris.push([ids[t[0]], ids[t[1]], ids[t[2]]]);
                owner.push(fi as u32);
            }
        }
        (tris, owner)
    }
}

fn orient2(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn is_convex(p: &[[f64; 2]]) -> bool {
    let n = p.len();
    if n < 3 {
        return false;
    }
    let mut sign = 0.0;
    for i in 0..n {
        let o = orient2(p[i], p[(i + 1) % n], p[(i + 2) % n]);
        if o.abs() < 1e-15 {
            continue;
        }
        if sign == 0.0 {
            sign = o.signum();
        } else if o.signum() != sign {
            return false;
        }
    }
    sign > 0.0
}

pub fn signed_area(p: &[[f64; 2]], idx: &[usize]) -> f64 {
    let mut a = 0.0;
    for i in 0..idx.len() {
        let (u, v) = (p[idx[i]], p[idx[(i + 1) % idx.len()]]);
        a += u[0] * v[1] - v[0] * u[1];
    }
    a / 2.0
}

fn in_triangle(p: [f64; 2], a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> bool {
    orient2(a, b, p) >= 0.0 && orient2(b, c, p) >= 0.0 && orient2(c, a, p) >= 0.0
}

/// Ear-clipping triangulation of a polygon with holes (indices into `pts`). The outer
/// loop is made counter-clockwise and holes clockwise; holes are bridged to the outer loop
/// (Eberly's visible-vertex method). Returns counter-clockwise triangles.
pub fn triangulate(pts: &[[f64; 2]], outer: &[usize], holes: &[Vec<usize>]) -> Vec<[usize; 3]> {
    let mut poly: Vec<usize> = outer.to_vec();
    if signed_area(pts, &poly) < 0.0 {
        poly.reverse();
    }
    let mut hs: Vec<Vec<usize>> = holes
        .iter()
        .filter(|h| h.len() >= 3)
        .map(|h| {
            let mut h = h.clone();
            if signed_area(pts, &h) > 0.0 {
                h.reverse();
            }
            h
        })
        .collect();
    let maxx = |h: &Vec<usize>| h.iter().map(|&i| pts[i][0]).fold(f64::NEG_INFINITY, f64::max);
    hs.sort_by(|a, b| maxx(b).total_cmp(&maxx(a)));
    for h in hs {
        bridge(pts, &mut poly, &h);
    }
    ear_clip(pts, poly)
}

fn bridge(pts: &[[f64; 2]], poly: &mut Vec<usize>, hole: &[usize]) {
    // hole vertex with the largest x
    let mi = (0..hole.len()).max_by(|&a, &b| pts[hole[a]][0].total_cmp(&pts[hole[b]][0])).unwrap_or(0);
    let m = pts[hole[mi]];
    // nearest edge hit by the ray m + t·(1, 0)
    let n = poly.len();
    let mut best: Option<(f64, usize)> = None; // (x of hit, index in poly of the visible endpoint)
    for i in 0..n {
        let (a, b) = (pts[poly[i]], pts[poly[(i + 1) % n]]);
        if (a[1] > m[1]) == (b[1] > m[1]) && a[1] != m[1] && b[1] != m[1] {
            continue;
        }
        if a[1] == b[1] {
            if a[1] == m[1] {
                for (k, p) in [(i, a), ((i + 1) % n, b)] {
                    if p[0] >= m[0] && best.is_none_or(|(x, _)| p[0] < x) {
                        best = Some((p[0], k));
                    }
                }
            }
            continue;
        }
        let t = (m[1] - a[1]) / (b[1] - a[1]);
        if !(0.0..=1.0).contains(&t) {
            continue;
        }
        let x = a[0] + t * (b[0] - a[0]);
        if x < m[0] {
            continue;
        }
        let k = if a[0] > b[0] { i } else { (i + 1) % n };
        if best.is_none_or(|(bx, _)| x < bx) {
            best = Some((x, k));
        }
    }
    let Some((hx, mut pk)) = best else {
        // no visible edge (degenerate input): attach to the nearest vertex
        let k = (0..n)
            .min_by(|&a, &b| {
                let da = (pts[poly[a]][0] - m[0]).powi(2) + (pts[poly[a]][1] - m[1]).powi(2);
                let db = (pts[poly[b]][0] - m[0]).powi(2) + (pts[poly[b]][1] - m[1]).powi(2);
                da.total_cmp(&db)
            })
            .unwrap_or(0);
        splice(poly, k, hole, mi);
        return;
    };
    let p = pts[poly[pk]];
    if p[1] != m[1] || p[0] != hx {
        // reflex vertices inside triangle (m, hit, p) may block the view: take the one
        // with the smallest angle to the ray
        let i_pt = [hx, m[1]];
        let (ta, tb, tc) = if orient2(m, i_pt, p) >= 0.0 { (m, i_pt, p) } else { (m, p, i_pt) };
        let mut best_ang = f64::INFINITY;
        for k in 0..n {
            let q = pts[poly[k]];
            if poly[k] == poly[pk] {
                continue;
            }
            let prev = pts[poly[(k + n - 1) % n]];
            let next = pts[poly[(k + 1) % n]];
            let reflex = orient2(prev, q, next) <= 0.0;
            if reflex && in_triangle(q, ta, tb, tc) {
                let ang = ((q[1] - m[1]).abs()).atan2(q[0] - m[0]);
                if ang < best_ang {
                    best_ang = ang;
                    pk = k;
                }
            }
        }
    }
    splice(poly, pk, hole, mi);
}

fn splice(poly: &mut Vec<usize>, pk: usize, hole: &[usize], mi: usize) {
    let mut out = Vec::with_capacity(poly.len() + hole.len() + 2);
    out.extend_from_slice(&poly[..=pk]);
    for k in 0..=hole.len() {
        out.push(hole[(mi + k) % hole.len()]);
    }
    out.extend_from_slice(&poly[pk..]);
    *poly = out;
}

fn ear_clip(pts: &[[f64; 2]], mut v: Vec<usize>) -> Vec<[usize; 3]> {
    let mut tris = Vec::with_capacity(v.len().saturating_sub(2));
    let scale = v.iter().map(|&i| pts[i][0].abs().max(pts[i][1].abs())).fold(1e-12, f64::max);
    let eps = scale * scale * 1e-14;
    let mut i = 0usize;
    let mut misses = 0usize;
    while v.len() > 3 {
        let n = v.len();
        i %= n;
        let (ia, ib, ic) = (v[(i + n - 1) % n], v[i], v[(i + 1) % n]);
        let (a, b, c) = (pts[ia], pts[ib], pts[ic]);
        let o = orient2(a, b, c);
        if o.abs() <= eps {
            // collinear or repeated vertex: drop it without a triangle
            v.remove(i);
            misses = 0;
            continue;
        }
        let mut ear = o > 0.0;
        if ear {
            for &k in &v {
                if k == ia || k == ib || k == ic {
                    continue;
                }
                let p = pts[k];
                if p == a || p == b || p == c {
                    continue;
                }
                if in_triangle(p, a, b, c) {
                    ear = false;
                    break;
                }
            }
        }
        if ear || misses > n {
            if o > 0.0 || misses > 2 * n {
                tris.push([ia, ib, ic]);
            }
            v.remove(i);
            misses = 0;
        } else {
            i += 1;
            misses += 1;
        }
    }
    if v.len() == 3 && orient2(pts[v[0]], pts[v[1]], pts[v[2]]).abs() > eps {
        tris.push([v[0], v[1], v[2]]);
    }
    tris
}

// ---------------------------------------------------------------------------------------
// primitives

/// Default `--dims` of a primitive.
pub fn default_dims(kind: &str) -> V3 {
    match kind {
        "torus" => [1.0, 0.3, 1.0],
        _ => [1.0, 1.0, 1.0],
    }
}

fn ring(y: f64, r: f64, n: usize) -> Vec<V3> {
    (0..n)
        .map(|j| {
            let t = TAU * j as f64 / n as f64;
            [r * t.cos(), y, -r * t.sin()]
        })
        .collect()
}

/// A primitive centered at the origin (Y up). `dims` are full extents: box X,Y,Z;
/// sphere diameters X,Y,Z; cylinder/cone base X×Z and height Y; torus outer diameter
/// X (and Z) with tube diameter Y; plane X×Z.
pub fn primitive(kind: &str, dims: V3, segments: usize) -> Result<Mesh> {
    let seg = segments.max(3);
    let mut m = Mesh::default();
    match kind {
        "box" => {
            for &x in &[-0.5, 0.5] {
                for &y in &[-0.5, 0.5] {
                    for &z in &[-0.5, 0.5] {
                        m.verts.push([x, y, z]);
                    }
                }
            }
            // index = 4x + 2y + z
            for f in [[0, 1, 3, 2], [4, 6, 7, 5], [0, 4, 5, 1], [2, 3, 7, 6], [0, 2, 6, 4], [1, 5, 7, 3]] {
                m.face(f.to_vec());
            }
            m.orient_from(|_| [0.0; 3]);
        }
        "sphere" => {
            let lat = (seg / 2).max(2);
            m.verts.push([0.0, 0.5, 0.0]);
            for i in 1..lat {
                let phi = PI * i as f64 / lat as f64;
                m.verts.extend(ring(0.5 * phi.cos(), 0.5 * phi.sin(), seg));
            }
            m.verts.push([0.0, -0.5, 0.0]);
            let south = (m.verts.len() - 1) as u32;
            let at = |i: usize, j: usize| (1 + (i - 1) * seg + j % seg) as u32;
            for j in 0..seg {
                m.face(vec![0, at(1, j), at(1, j + 1)]);
                for i in 1..lat - 1 {
                    m.face(vec![at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1)]);
                }
                m.face(vec![at(lat - 1, j), south, at(lat - 1, j + 1)]);
            }
            m.orient_from(|_| [0.0; 3]);
        }
        "cylinder" | "cone" => {
            m.verts.extend(ring(-0.5, 0.5, seg));
            if kind == "cylinder" {
                m.verts.extend(ring(0.5, 0.5, seg));
                for j in 0..seg {
                    let (a, b) = (j as u32, ((j + 1) % seg) as u32);
                    m.face(vec![a, b, b + seg as u32, a + seg as u32]);
                }
                m.face((seg as u32..2 * seg as u32).collect());
            } else {
                m.verts.push([0.0, 0.5, 0.0]);
                let apex = seg as u32;
                for j in 0..seg {
                    m.face(vec![j as u32, ((j + 1) % seg) as u32, apex]);
                }
            }
            m.face((0..seg as u32).collect());
            m.orient_from(|_| [0.0; 3]);
        }
        "torus" => {
            let (rx, rz, rt) = (dims[0] / 2.0, dims[2] / 2.0, dims[1] / 2.0);
            if rt <= 0.0 || rt >= rx.min(rz) {
                return Err(Error::new(
                    "PX1010",
                    "torus: the tube diameter (Y) must be positive and smaller than the outer diameter (X, Z)",
                )
                .with_hint("pixelox render 3d --primitive torus --dims 2,0.5,2 -o torus.svg"));
            }
            let (ax, az) = (rx - rt, rz - rt);
            let minor = (seg / 2).max(3);
            for i in 0..seg {
                let t = TAU * i as f64 / seg as f64;
                let (c, s) = (t.cos(), t.sin());
                for j in 0..minor {
                    let p = TAU * j as f64 / minor as f64;
                    m.verts.push([(ax + rt * p.cos()) * c, rt * p.sin(), -(az + rt * p.cos()) * s]);
                }
            }
            let at = |i: usize, j: usize| ((i % seg) * minor + j % minor) as u32;
            for i in 0..seg {
                for j in 0..minor {
                    m.face(vec![at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1)]);
                }
            }
            m.orient_from(|c| {
                let t = (-c[2] / az).atan2(c[0] / ax);
                [ax * t.cos(), 0.0, -az * t.sin()]
            });
            return Ok(m);
        }
        "plane" => {
            m.verts.extend([[-0.5, 0.0, -0.5], [-0.5, 0.0, 0.5], [0.5, 0.0, 0.5], [0.5, 0.0, -0.5]]);
            m.face(vec![0, 1, 2, 3]);
            m.orient_from(|c| sub(c, [0.0, 1.0, 0.0]));
            let (x, z) = (dims[0], dims[2]);
            m.map(|v| [v[0] * x, v[1], v[2] * z]);
            return Ok(m);
        }
        other => {
            return Err(Error::new("PX1010", format!("unknown primitive '{other}'"))
                .with_hint("use box, sphere, cylinder, cone, torus, or plane"));
        }
    }
    m.map(|v| [v[0] * dims[0], v[1] * dims[1], v[2] * dims[2]]);
    Ok(m)
}

// ---------------------------------------------------------------------------------------
// OBJ

/// Parse Wavefront OBJ geometry (`v` and `f`; texture/normal indices and materials are
/// ignored; negative indices are relative).
pub fn parse_obj(text: &str, name: &str) -> Result<Mesh> {
    let mut m = Mesh::default();
    for (ln, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        let mut it = line.split_whitespace();
        let bad = |what: &str| Error::new("PX1010", format!("{name}:{}: {what}", ln + 1)).with_context("line", ln + 1);
        match it.next() {
            Some("v") => {
                let c: Vec<f64> = it
                    .take(3)
                    .map(|s| s.parse::<f64>())
                    .collect::<std::result::Result<_, _>>()
                    .map_err(|_| bad("bad vertex"))?;
                if c.len() < 3 || c.iter().any(|v| !v.is_finite()) {
                    return Err(bad("a vertex needs three numbers"));
                }
                m.verts.push([c[0], c[1], c[2]]);
            }
            Some("f") => {
                let mut idx = Vec::new();
                for tok in it {
                    let s = tok.split('/').next().unwrap_or("");
                    let i: i64 = s.parse().map_err(|_| bad("bad face index"))?;
                    let n = m.verts.len() as i64;
                    let r = if i < 0 { n + i } else { i - 1 };
                    if r < 0 || r >= n {
                        return Err(bad(&format!("face index {i} is out of range (1..{n})")));
                    }
                    idx.push(r as u32);
                }
                if idx.len() >= 3 {
                    m.face(idx);
                }
            }
            _ => {}
        }
    }
    if m.faces.is_empty() {
        return Err(
            Error::new("PX1010", format!("{name}: no faces found")).with_hint("the OBJ file needs `v` and `f` records")
        );
    }
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn closed_outward(m: &Mesh) -> bool {
        let edges = m.edges();
        Mesh::is_closed(&edges) && m.faces.iter().all(|f| dot(m.face_normal(f), m.centroid(f)) > -1e-9)
    }

    #[test]
    fn primitives_are_closed_and_outward() {
        for k in ["box", "sphere", "cylinder", "cone"] {
            let mut m = primitive(k, default_dims(k), 16).unwrap();
            m.weld();
            assert!(closed_outward(&m), "{k}");
        }
        let mut t = primitive("torus", default_dims("torus"), 16).unwrap();
        t.weld();
        assert!(Mesh::is_closed(&t.edges()));
        let b = primitive("box", [1.0; 3], 32).unwrap();
        assert_eq!(b.edges().len(), 12);
        assert_eq!(b.triangulate().0.len(), 12);
    }

    #[test]
    fn ear_clipping_with_a_hole() {
        // 10x10 square with a 4x4 hole: area 84
        let pts =
            vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0], [3.0, 3.0], [7.0, 3.0], [7.0, 7.0], [3.0, 7.0]];
        let tris = triangulate(&pts, &[0, 1, 2, 3], &[vec![4, 5, 6, 7]]);
        let area: f64 = tris.iter().map(|t| orient2(pts[t[0]], pts[t[1]], pts[t[2]]) / 2.0).sum();
        assert!((area - 84.0).abs() < 1e-9, "area {area}");
        assert!(tris.iter().all(|t| orient2(pts[t[0]], pts[t[1]], pts[t[2]]) > 0.0));
    }

    #[test]
    fn obj_faces_and_relative_indices() {
        let m = parse_obj("v 0 0 0\nv 1 0 0\nv 1 1 0\nv 0 1 0\nf 1/1/1 2/2/2 3/3/3 4\nf -4 -2 -1\n", "q.obj").unwrap();
        assert_eq!(m.faces.len(), 2);
        assert_eq!(m.faces[1].loops[0], vec![0, 2, 3]);
        assert_eq!(parse_obj("v 0 0 0\nf 1 2 3\n", "bad.obj").unwrap_err().code, "PX1010");
    }
}
