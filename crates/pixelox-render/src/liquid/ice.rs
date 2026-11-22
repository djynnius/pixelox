//! `render ice`: ice cubes, ray-marched. Each cube is a slightly irregular rounded box in a
//! random orientation (a signed distance field), seen by an orthographic camera. A ray
//! that hits a cube refracts in (index 1.31), crosses it, and refracts out onto the design
//! behind, so the cube shows the design bent and flipped; along the way it gathers a
//! frosty, cracked white core. The surface reflects a soft studio environment (Fresnel)
//! and catches specular highlights on its rounded edges. Cubes are drawn back to front and
//! later ones refract the earlier ones.

use super::field::smooth;
use super::{add_layer, r1, Draw};
use pixelox_core::noise::{self, Field};
use pixelox_core::prelude::*;
use pixelox_core::units::Length;
use rand_core::RngCore;
use rayon::prelude::*;

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("render ice", Category::Render, "Ice cubes that refract the design, on a new layer", ice)
        .about(
            "Renders --count ice cubes around --at (scattered within --spread) onto a new layer named 'Ice' on top \
             of the stack (directly above --layer, or where --above/--below/--into say). Each cube is a rounded, \
             slightly irregular block in a random tumble, ray-traced: it refracts the design behind it (bent and \
             flipped, as real ice does), has a frosty, cracked core (--frost), a --tint, and reflections and \
             highlights on its rounded edges. The refraction is of the design as it is when the command runs: add \
             the ice after the background and the product. --seed draws another arrangement; add `fx drop-shadow` \
             to the layer for cubes resting on a surface.",
        )
        .param(param("count", ParamKind::Int).default("3").range("1..40").help("Number of cubes"))
        .param(param("at", ParamKind::Point).default("50%,70%").help("Center of the group of cubes (X,Y)"))
        .param(
            param("size", ParamKind::Length)
                .default("16%")
                .help("Cube size (edge length) in px or % of the shorter canvas side"),
        )
        .param(
            param("spread", ParamKind::Length)
                .default("22%")
                .help("How far the cubes scatter from --at, in px or % of the shorter side"),
        )
        .param(
            param("frost", ParamKind::Float)
                .default("55")
                .range("0..100")
                .help("Cloudy, cracked core: 0 = clear, 100 = white"),
        )
        .param(param("tint", ParamKind::Color).default("#e4f4ff").help("Color of the ice"))
        .param(
            param("layer", ParamKind::Layer).help(
                "Put the ice layer directly above this layer (the same as --above; default: on top of the stack)",
            ),
        )
        .param(param("name", ParamKind::Str).help("Name for the new layer (default: Ice)"))
        .params(common::placement())
        .example(
            "pixelox render ice doc.pxo --in-place --at 50%,80% --count 4",
            "Four ice cubes at the foot of a drink",
        )
        .example(
            "pixelox render ice doc.pxo --in-place --count 1 --size 30% --at 70%,30% --frost 30 --seed 5",
            "One large, clear cube",
        )
        .example(
            "pixelox render ice doc.pxo --in-place --count 8 --size 9% --spread 40% --tint '#fff4e0' --name IceFar",
            "A scatter of small warm-tinted cubes",
        )]
}

type V3 = [f64; 3];

fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn mul(a: V3, k: f64) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn norm(a: V3) -> V3 {
    let m = dot(a, a).sqrt().max(1e-12);
    mul(a, 1.0 / m)
}

/// Refract `i` through a surface with normal `n` (facing against `i`), ratio `eta`;
/// `None` on total internal reflection.
fn refract(i: V3, n: V3, eta: f64) -> Option<V3> {
    let c = -dot(n, i);
    let k = 1.0 - eta * eta * (1.0 - c * c);
    (k >= 0.0).then(|| add(mul(i, eta), mul(n, eta * c - k.sqrt())))
}

fn reflect(i: V3, n: V3) -> V3 {
    sub(i, mul(n, 2.0 * dot(i, n)))
}

/// A cube: center, rotation (rows: local axes in world), half extents, corner radius.
struct Cube {
    c: V3,
    rot: [V3; 3],
    b: V3,
    r: f64,
}

impl Cube {
    fn to_local(&self, p: V3) -> V3 {
        let d = sub(p, self.c);
        [dot(self.rot[0], d), dot(self.rot[1], d), dot(self.rot[2], d)]
    }
    fn dir_local(&self, d: V3) -> V3 {
        [dot(self.rot[0], d), dot(self.rot[1], d), dot(self.rot[2], d)]
    }
    fn dir_world(&self, d: V3) -> V3 {
        add(add(mul(self.rot[0], d[0]), mul(self.rot[1], d[1])), mul(self.rot[2], d[2]))
    }
    /// Signed distance in local coordinates.
    fn sdf(&self, p: V3) -> f64 {
        let q =
            [p[0].abs() - (self.b[0] - self.r), p[1].abs() - (self.b[1] - self.r), p[2].abs() - (self.b[2] - self.r)];
        let o = [q[0].max(0.0), q[1].max(0.0), q[2].max(0.0)];
        dot(o, o).sqrt() + q[0].max(q[1]).max(q[2]).min(0.0) - self.r
    }
    fn normal(&self, p: V3) -> V3 {
        let e = self.r * 0.05 + 1e-4;
        let d = |dx: f64, dy: f64, dz: f64| self.sdf([p[0] + dx, p[1] + dy, p[2] + dz]);
        norm([d(e, 0.0, 0.0) - d(-e, 0.0, 0.0), d(0.0, e, 0.0) - d(0.0, -e, 0.0), d(0.0, 0.0, e) - d(0.0, 0.0, -e)])
    }
    fn radius(&self) -> f64 {
        dot(self.b, self.b).sqrt()
    }
    /// March from `o` along `d` (local) to the surface; outside → in when `inside` is false.
    fn march(&self, o: V3, d: V3, inside: bool, max_t: f64) -> Option<V3> {
        let mut t = 0.0;
        let eps = self.r * 0.01 + 1e-3;
        for _ in 0..96 {
            let p = add(o, mul(d, t));
            let s = if inside { -self.sdf(p) } else { self.sdf(p) };
            if s < eps {
                return Some(p);
            }
            t += s.max(eps * 0.5);
            if t > max_t {
                return None;
            }
        }
        None
    }
}

/// A random rotation (uniform unit quaternion).
fn rotation(d: &mut Draw) -> [V3; 3] {
    let (u1, u2, u3) = (d.u(), d.u() * std::f64::consts::TAU, d.u() * std::f64::consts::TAU);
    let (a, b) = ((1.0 - u1).sqrt(), u1.sqrt());
    let (x, y, z, w) = (a * u2.sin(), a * u2.cos(), b * u3.sin(), b * u3.cos());
    [
        [1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y + z * w), 2.0 * (x * z - y * w)],
        [2.0 * (x * y - z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z + x * w)],
        [2.0 * (x * z + y * w), 2.0 * (y * z - x * w), 1.0 - 2.0 * (x * x + y * y)],
    ]
}

/// The studio around the cubes: bright above, dark below, two soft boxes.
fn env(r: V3) -> [f64; 3] {
    let sky = smooth(-0.35, 0.85, -r[1] as f32) as f64;
    let mut c = [0.16 + 0.8 * sky, 0.19 + 0.79 * sky, 0.24 + 0.76 * sky];
    for (dir, k) in [([-0.45, -0.75, 0.48], 0.9), ([0.7, -0.35, 0.62], 0.45)] {
        let d = sub(r, norm(dir));
        let g = k * (-dot(d, d) * 9.0).exp();
        c = [c[0] + g, c[1] + g, c[2] + g];
    }
    c
}

fn ice(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (w, h) = (doc.width as f64, doc.height as f64);
    let short = w.min(h).max(1.0);
    let pct = |v: f64| Length { value: v, unit: units::Unit::Pct, plus: None };
    let at = a.point("at")?.map(|p| p.resolve(w, h, doc.dpi())).unwrap_or((w / 2.0, h * 0.7));
    let size = a.length("size")?.unwrap_or(pct(16.0)).to_px(short, doc.dpi()).max(4.0);
    let spread = a.length("spread")?.unwrap_or(pct(22.0)).to_px(short, doc.dpi()).max(0.0);
    let count = a.usize("count")?.max(1);
    let frost = a.f64("frost")? / 100.0;
    let tint = a.color("tint")?;
    let tint = [tint.r as f64, tint.g as f64, tint.b as f64];
    super::placement(a, doc)?;
    // the design the cubes refract (straight alpha over transparency)
    let mut backdrop = compose::composite(doc)?;
    let mut d = Draw(ctx.rng());
    let seed = d.0.next_u64();
    let crack = Field { octaves: 3, ..Field::new(noise::lane(seed, 1)) };
    let cloud = Field { octaves: 4, ..Field::new(noise::lane(seed, 2)) };
    let mut cubes: Vec<Cube> = (0..count)
        .map(|_| {
            let s = size * d.range(0.78, 1.2) / 2.0;
            let ang = d.range(0.0, std::f64::consts::TAU);
            let rad = spread * d.u().sqrt();
            Cube {
                c: [at.0 + ang.cos() * rad, at.1 + ang.sin() * rad * 0.75, s * 3.0 + d.range(0.0, size)],
                rot: rotation(&mut d),
                b: [s * d.range(0.92, 1.0), s * d.range(0.86, 0.98), s * d.range(0.8, 0.95)],
                r: s * d.range(0.2, 0.3),
            }
        })
        .collect();
    // back to front: lower z first
    cubes.sort_by(|p, q| p.c[2].total_cmp(&q.c[2]));
    let mut layer = Image::new(doc.width, doc.height);
    let light = norm([-0.45, -0.75, 0.48]);
    let view = [0.0, 0.0, 1.0];
    for cube in &cubes {
        let rr = cube.radius() * 1.02;
        let x0 = ((cube.c[0] - rr).floor().max(0.0)) as u32;
        let y0 = ((cube.c[1] - rr).floor().max(0.0)) as u32;
        let x1 = ((cube.c[0] + rr).ceil().min(w)) as u32;
        let y1 = ((cube.c[1] + rr).ceil().min(h)) as u32;
        if x1 <= x0 || y1 <= y0 {
            continue;
        }
        let bw = (x1 - x0) as usize;
        let src = &backdrop;
        let sample = |x: f64, y: f64| -> [f64; 4] {
            let p = src.sample_bilinear(x - 0.5, y - 0.5);
            [p[0] as f64, p[1] as f64, p[2] as f64, p[3] as f64]
        };
        let plane_z = cube.c[2] - cube.radius() * 1.6;
        let rows: Vec<Vec<[f32; 4]>> = (y0..y1)
            .into_par_iter()
            .map(|py| {
                let mut row = vec![[0f32; 4]; bw];
                for (ix, out) in row.iter_mut().enumerate() {
                    let px = x0 as f64 + ix as f64;
                    let mut acc = [0f64; 4];
                    // 2x2 supersampling
                    for (sx, sy) in [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
                        let (wx, wy) = (px + sx, py as f64 + sy);
                        let o = [wx, wy, cube.c[2] + rr * 2.0];
                        let dir = [0.0, 0.0, -1.0];
                        let (ol, dl) = (cube.to_local(o), cube.dir_local(dir));
                        let Some(hit) = cube.march(ol, dl, false, rr * 4.0) else { continue };
                        let n = cube.normal(hit);
                        let nw = cube.dir_world(n);
                        // reflection
                        let fres = 0.04 + 0.96 * (1.0 - dot(nw, view).abs()).powi(5);
                        let refl = env(reflect(dir, nw));
                        // refraction in, across, and out
                        let d1 = refract(dl, n, 1.0 / 1.31).unwrap_or(dl);
                        let inside_start = add(hit, mul(d1, cube.r * 0.05));
                        let (exit, d2) = match cube.march(inside_start, d1, true, rr * 4.0) {
                            Some(e) => {
                                let ne = cube.normal(e);
                                let out =
                                    refract(d1, mul(ne, -1.0), 1.31).unwrap_or_else(|| reflect(d1, mul(ne, -1.0)));
                                (e, out)
                            }
                            None => (inside_start, d1),
                        };
                        let len = dot(sub(exit, hit), sub(exit, hit)).sqrt() / (cube.radius() * 2.0);
                        // where the exiting ray meets the design behind
                        let (ew, dw) = (add(cube.c, cube.dir_world(exit)), cube.dir_world(d2));
                        let t = if dw[2] < -1e-3 { (plane_z - ew[2]) / dw[2] } else { 0.0 };
                        let bg = sample(ew[0] + dw[0] * t, ew[1] + dw[1] * t);
                        // the frosty core: cracks and clouds sampled along the path inside
                        let mut dens = 0.0;
                        for k in 0..5 {
                            let q = add(hit, mul(sub(exit, hit), (k as f64 + 0.5) / 5.0));
                            let u = mul(q, 1.0 / (cube.radius() * 0.9));
                            let core = 1.0 - (dot(u, u).sqrt() / 1.1).min(1.0);
                            let c1 = crack.at(u[0] * 1.7 + u[2], u[1] * 1.7 - u[2]);
                            let veins = (1.0 - ((c1 - 0.5).abs() / 0.05).min(1.0)).powi(2);
                            let c2 = cloud.at(u[0] * 2.3 - u[2] * 0.7, u[1] * 2.3 + u[2] * 0.4);
                            dens += 0.2 * (core * core * 1.6 + veins * 0.7 + (c2 - 0.35).max(0.0) * 1.2);
                        }
                        let fog = (1.0 - (-dens * frost * 2.4).exp()).min(0.95);
                        // the body: refracted design, tinted with depth, fogged white
                        let absorb = 1.0 - (-len * 0.9).exp();
                        let mut col = [0f64; 3];
                        let lit = 0.75 + 0.35 * dot(nw, light).max(0.0);
                        for i in 0..3 {
                            let behind = bg[i] * bg[3] + (0.82 * tint[i]) * (1.0 - bg[3]);
                            let body = behind * (1.0 - absorb * 0.55) + tint[i] * absorb * 0.55 * lit;
                            let foggy = body * (1.0 - fog) + tint[i] * (0.86 + 0.14 * lit) * fog;
                            col[i] = foggy * (1.0 - fres) + refl[i] * fres;
                        }
                        // highlights on the rounded edges
                        let hv = norm(add(light, view));
                        let spec = dot(nw, hv).max(0.0).powf(90.0) * 1.3 + dot(nw, hv).max(0.0).powf(12.0) * 0.08;
                        for i in 0..3 {
                            acc[i] += (col[i] + spec).min(1.0);
                        }
                        acc[3] += 1.0;
                    }
                    if acc[3] > 0.0 {
                        let a = acc[3] / 4.0;
                        *out = [(acc[0] / acc[3]) as f32, (acc[1] / acc[3]) as f32, (acc[2] / acc[3]) as f32, a as f32];
                    }
                }
                row
            })
            .collect();
        for (j, row) in rows.iter().enumerate() {
            for (i, p) in row.iter().enumerate() {
                if p[3] <= 0.0 {
                    continue;
                }
                let (x, y) = (x0 + i as u32, y0 + j as u32);
                for img in [&mut layer, &mut backdrop] {
                    let mut dst = img.get(x, y);
                    let k = p[3];
                    let ao = k + dst[3] * (1.0 - k);
                    for c in 0..3 {
                        dst[c] = (p[c] * k + dst[c] * dst[3] * (1.0 - k)) / ao.max(1e-6);
                    }
                    dst[3] = ao;
                    img.set(x, y, dst);
                }
            }
        }
    }
    let path = add_layer(a, doc, Rendered { image: layer, x: 0, y: 0 }, "Ice")?;
    let l = doc.layer(&path);
    let bounds = compose::layer_bounds(doc, l)?.map(|b| b.to_json()).unwrap_or(Value::Null);
    Ok(json!({
        "layer": l.name,
        "id": l.id,
        "cubes": cubes.iter().map(|c| json!({"x": r1(c.c[0]), "y": r1(c.c[1]), "size": r1(c.b[0] * 2.0)})).collect::<Vec<_>>(),
        "bounds": bounds,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing;

    fn scene() -> Document {
        let bg = Image::from_fn(240, 300, |x, y| {
            let t = y as f32 / 300.0;
            if (90..150).contains(&x) && (40..260).contains(&y) {
                [0.9, 0.15, 0.1, 1.0]
            } else {
                [0.02 + 0.1 * t, 0.12 + 0.2 * t, 0.35 + 0.3 * t, 1.0]
            }
        });
        testing::doc_from(bg)
    }

    #[test]
    fn cubes_refract_and_are_seeded() {
        let spec = testing::find(&ops(), "render ice");
        let mut a = scene();
        let v = testing::run(&spec, &[("count", "2"), ("at", "50%,60%"), ("size", "30%")], &mut a).unwrap();
        assert_eq!(v["cubes"].as_array().unwrap().len(), 2);
        let mut b = scene();
        testing::run(&spec, &[("count", "2"), ("at", "50%,60%"), ("size", "30%")], &mut b).unwrap();
        assert_eq!(testing::raster(&a, "Ice"), testing::raster(&b, "Ice"));
        let ice = testing::raster(&a, "Ice");
        assert!(ice.alpha_bbox(0.5).is_some());
    }

    /// Look development: `LAB=dir cargo test -p pixelox-render ice_look -- --ignored`.
    #[test]
    #[ignore]
    fn ice_look() {
        let dir = std::env::var("LAB").unwrap();
        let spec = testing::find(&ops(), "render ice");
        for (i, frost) in ["20", "55", "85"].iter().enumerate() {
            let mut d = scene();
            let p = [("count", "3"), ("at", "50%,62%"), ("size", "34%"), ("spread", "25%"), ("frost", *frost)];
            testing::run(&spec, &p, &mut d).unwrap();
            let img = compose::composite(&d).unwrap();
            let mut out = format!("P6 {} {} 255 ", img.width, img.height).into_bytes();
            for p in img.data.chunks(4) {
                out.extend([(p[0] * 255.0) as u8, (p[1] * 255.0) as u8, (p[2] * 255.0) as u8]);
            }
            std::fs::write(format!("{dir}/ice{i}.ppm"), out).unwrap();
        }
    }
}
