//! SVG-profile extrusion: flatten the outlines of an SVG's paths (through usvg, so shapes,
//! transforms, and `<use>` resolve), sort the contours into outers and holes by nesting,
//! and build a closed prism: front/back caps (polygons with holes) plus side quads.

use super::mesh::{signed_area, Face, Mesh};
use crate::svg_options;
use pixelox_core::color::Rgba;
use pixelox_core::error::{Error, Result};
use tiny_skia::{Path, PathSegment, Point};

/// Closed contours (SVG user units, y down) of the visible, filled paths of an SVG (every
/// path when none is filled).
pub fn svg_contours(bytes: &[u8], name: &str) -> Result<Vec<Vec<[f64; 2]>>> {
    let tree = usvg::Tree::from_data(bytes, &svg_options(false))
        .map_err(|e| Error::new("PX1010", format!("{name} is not a valid SVG: {e}")).with_context("path", name))?;
    let mut paths: Vec<(Path, bool)> = Vec::new();
    collect(tree.root(), &mut paths);
    let any_filled = paths.iter().any(|p| p.1);
    let paths: Vec<Path> = paths.into_iter().filter(|p| p.1 || !any_filled).map(|p| p.0).collect();
    if paths.is_empty() {
        return Err(Error::new("PX1010", format!("{name} has no <path> or shape outlines to extrude"))
            .with_hint("export the logo with outlined paths (text converted to outlines)"));
    }
    let mut lo = [f64::INFINITY; 2];
    let mut hi = [f64::NEG_INFINITY; 2];
    for p in &paths {
        let b = p.bounds();
        lo = [lo[0].min(b.left() as f64), lo[1].min(b.top() as f64)];
        hi = [hi[0].max(b.right() as f64), hi[1].max(b.bottom() as f64)];
    }
    let diag = ((hi[0] - lo[0]).powi(2) + (hi[1] - lo[1]).powi(2)).sqrt().max(1e-9);
    let step = diag * 0.01;
    let mut out = Vec::new();
    for p in &paths {
        flatten(p, step, &mut out);
    }
    let min_area = (diag * 1e-5).powi(2);
    let tol = diag * 1e-9;
    let mut cleaned = Vec::new();
    for c in out {
        let mut d: Vec<[f64; 2]> = Vec::with_capacity(c.len());
        for p in c {
            if d.last().is_none_or(|q: &[f64; 2]| (q[0] - p[0]).abs() > tol || (q[1] - p[1]).abs() > tol) {
                d.push(p);
            }
        }
        while d.len() > 1 && {
            let (a, b) = (d[0], d[d.len() - 1]);
            (a[0] - b[0]).abs() <= tol && (a[1] - b[1]).abs() <= tol
        } {
            d.pop();
        }
        let idx: Vec<usize> = (0..d.len()).collect();
        if d.len() >= 3 && signed_area(&d, &idx).abs() > min_area {
            cleaned.push(d);
        }
    }
    if cleaned.is_empty() {
        return Err(Error::new("PX1010", format!("{name}: the outlines enclose no area")));
    }
    Ok(cleaned)
}

fn collect(g: &usvg::Group, out: &mut Vec<(Path, bool)>) {
    for n in g.children() {
        match n {
            usvg::Node::Group(g) => collect(g, out),
            usvg::Node::Path(p) if p.is_visible() => {
                if let Some(t) = p.data().clone().transform(p.abs_transform()) {
                    out.push((t, p.fill().is_some()));
                }
            }
            usvg::Node::Text(t) => collect(t.flattened(), out),
            _ => {}
        }
    }
}

fn flatten(p: &Path, step: f64, out: &mut Vec<Vec<[f64; 2]>>) {
    let mut cur: Vec<[f64; 2]> = Vec::new();
    let mut last = Point::zero();
    let pt = |p: Point| [p.x as f64, p.y as f64];
    let n_for = |len: f64| ((len / step).ceil() as usize).clamp(2, 64);
    let dist = |a: Point, b: Point| (((a.x - b.x) as f64).powi(2) + ((a.y - b.y) as f64).powi(2)).sqrt();
    for seg in p.segments() {
        match seg {
            PathSegment::MoveTo(q) => {
                if cur.len() >= 3 {
                    out.push(std::mem::take(&mut cur));
                }
                cur.clear();
                cur.push(pt(q));
                last = q;
            }
            PathSegment::LineTo(q) => {
                cur.push(pt(q));
                last = q;
            }
            PathSegment::QuadTo(c, q) => {
                let n = n_for(dist(last, c) + dist(c, q));
                for i in 1..=n {
                    let t = i as f64 / n as f64;
                    let u = 1.0 - t;
                    let x = u * u * last.x as f64 + 2.0 * u * t * c.x as f64 + t * t * q.x as f64;
                    let y = u * u * last.y as f64 + 2.0 * u * t * c.y as f64 + t * t * q.y as f64;
                    cur.push([x, y]);
                }
                last = q;
            }
            PathSegment::CubicTo(c1, c2, q) => {
                let n = n_for(dist(last, c1) + dist(c1, c2) + dist(c2, q));
                for i in 1..=n {
                    let t = i as f64 / n as f64;
                    let u = 1.0 - t;
                    let (a, b, c, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
                    let x = a * last.x as f64 + b * c1.x as f64 + c * c2.x as f64 + d * q.x as f64;
                    let y = a * last.y as f64 + b * c1.y as f64 + c * c2.y as f64 + d * q.y as f64;
                    cur.push([x, y]);
                }
                last = q;
            }
            PathSegment::Close => {
                if cur.len() >= 3 {
                    out.push(std::mem::take(&mut cur));
                }
                cur.clear();
            }
        }
    }
    if cur.len() >= 3 {
        out.push(cur);
    }
}

fn point_in(p: [f64; 2], poly: &[[f64; 2]]) -> bool {
    let mut inside = false;
    let n = poly.len();
    let mut j = n - 1;
    for i in 0..n {
        let (a, b) = (poly[i], poly[j]);
        if (a[1] > p[1]) != (b[1] > p[1]) && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0] {
            inside = !inside;
        }
        j = i;
    }
    inside
}

/// The extruded prism: contours in SVG units (y down) are centered, flipped to y up, and
/// extruded along Z from -depth/2 to +depth/2. `depth <= 0` uses 20% of the larger side.
pub fn extrude(contours: Vec<Vec<[f64; 2]>>, depth: Option<f64>) -> (Mesh, f64) {
    let mut lo = [f64::INFINITY; 2];
    let mut hi = [f64::NEG_INFINITY; 2];
    for c in &contours {
        for p in c {
            lo = [lo[0].min(p[0]), lo[1].min(p[1])];
            hi = [hi[0].max(p[0]), hi[1].max(p[1])];
        }
    }
    let (cx, cy) = ((lo[0] + hi[0]) / 2.0, (lo[1] + hi[1]) / 2.0);
    let mut cs: Vec<Vec<[f64; 2]>> =
        contours.into_iter().map(|c| c.into_iter().map(|p| [p[0] - cx, cy - p[1]]).collect()).collect();
    let depth = depth.filter(|d| *d > 0.0).unwrap_or_else(|| 0.2 * (hi[0] - lo[0]).max(hi[1] - lo[1]));

    // nesting depth decides outer (even) vs hole (odd); holes belong to the smallest
    // enclosing outer
    let areas: Vec<f64> = cs.iter().map(|c| signed_area(c, &(0..c.len()).collect::<Vec<_>>()).abs()).collect();
    let n = cs.len();
    let mut level = vec![0usize; n];
    let mut parent: Vec<Option<usize>> = vec![None; n];
    for i in 0..n {
        let probe = cs[i][0];
        let mut best: Option<usize> = None;
        for j in 0..n {
            if i != j && areas[j] > areas[i] && point_in(probe, &cs[j]) {
                level[i] += 1;
                if best.is_none_or(|b| areas[j] < areas[b]) {
                    best = Some(j);
                }
            }
        }
        parent[i] = best;
    }
    for i in 0..n {
        let idx: Vec<usize> = (0..cs[i].len()).collect();
        let a = signed_area(&cs[i], &idx);
        let want_ccw = level[i].is_multiple_of(2);
        if (a > 0.0) != want_ccw {
            cs[i].reverse();
        }
    }
    let mut m = Mesh::default();
    let (zf, zb) = (depth / 2.0, -depth / 2.0);
    let mut front: Vec<Vec<u32>> = Vec::with_capacity(n);
    let mut back: Vec<Vec<u32>> = Vec::with_capacity(n);
    for c in &cs {
        let f0 = m.verts.len() as u32;
        m.verts.extend(c.iter().map(|p| [p[0], p[1], zf]));
        let b0 = m.verts.len() as u32;
        m.verts.extend(c.iter().map(|p| [p[0], p[1], zb]));
        front.push((0..c.len() as u32).map(|k| f0 + k).collect());
        back.push((0..c.len() as u32).map(|k| b0 + k).collect());
    }
    let face = |loops: Vec<Vec<u32>>| Face { loops, color: Rgba::WHITE };
    for i in 0..n {
        if !level[i].is_multiple_of(2) {
            continue;
        }
        let holes: Vec<usize> = (0..n).filter(|&h| !level[h].is_multiple_of(2) && parent[h] == Some(i)).collect();
        let mut fl = vec![front[i].clone()];
        fl.extend(holes.iter().map(|&h| front[h].clone()));
        m.faces.push(face(fl));
        let mut bl = vec![back[i].iter().rev().copied().collect::<Vec<u32>>()];
        bl.extend(holes.iter().map(|&h| back[h].iter().rev().copied().collect::<Vec<u32>>()));
        m.faces.push(face(bl));
    }
    for i in 0..n {
        let k = front[i].len();
        for j in 0..k {
            let j1 = (j + 1) % k;
            m.faces.push(face(vec![vec![front[i][j], back[i][j], back[i][j1], front[i][j1]]]));
        }
    }
    (m, depth)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::three_d::mesh::{dot, Mesh};

    #[test]
    fn square_with_hole_extrudes_closed() {
        let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">
            <path d="M10 10 H90 V90 H10 Z M30 30 V70 H70 V30 Z" fill="black" fill-rule="evenodd"/></svg>"#;
        let cs = svg_contours(svg, "logo.svg").unwrap();
        assert_eq!(cs.len(), 2);
        let (mut m, d) = extrude(cs, Some(10.0));
        assert_eq!(d, 10.0);
        m.weld();
        let edges = m.edges();
        assert!(Mesh::is_closed(&edges));
        // caps: front normal +Z, back -Z
        assert!(dot(m.face_normal(&m.faces[0]), [0.0, 0.0, 1.0]) > 0.99);
        assert!(dot(m.face_normal(&m.faces[1]), [0.0, 0.0, -1.0]) > 0.99);
        let (tris, _) = m.triangulate();
        // 2 caps × 8 triangles + 8 side quads × 2
        assert_eq!(tris.len(), 32);
    }
}
