//! `render lens-flare`: a camera lens flare drawn onto a new layer in screen mode.
//!
//! The flare is light added to the scene. Each lens type is a recipe of elements, sized
//! as fractions of the canvas diagonal: a hot core, a glow and a wide veil around it,
//! starburst rays (a few strong spikes and many faint hairs), an optional chromatic halo
//! ring, an optional horizontal anamorphic streak (movie prime), and a chain of ghosts —
//! reflections of the aperture between the lens elements — along the line from the
//! flare through the image center, with soft edges, bright rims on some, and a slight
//! color split (each channel's ghost a little larger or smaller than the others).
//!
//! Light accumulates in display (sRGB-encoded) values, like the classic filter, so faint
//! tails stay faint, and is compressed with `1 - exp(-E)` so the core rolls off to white
//! instead of clipping. The layer stores it as
//! straight color with alpha = the brightest channel, so `screen` over any backdrop is
//! exactly the screen of the light (and `normal` still reads as a glow). Small
//! variations (ray angles, ghost sizes) come from the op's RNG.

use crate::noise::mix64;
use pixelox_core::prelude::*;
use rand_core::RngCore;
use rayon::prelude::*;
use std::f64::consts::{PI, TAU};

/// The four lens types, as `--lens` spells them.
pub(crate) const LENSES: &[&str] = &["zoom", "35mm", "105mm", "movie"];

pub(crate) fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc(
        "render lens-flare",
        Category::Render,
        "Add a camera lens flare as a new screen layer",
        lens_flare,
    )
    .about(
        "Renders a lens flare (like Filter > Render > Lens Flare) centered at --at onto a new raster layer named \
             'Lens Flare' in screen mode, above --layer (default: above the active layer, or on top), so it stays \
             movable and editable. --lens picks the look: zoom (50-300 mm zoom: warm glow, fine rays, a halo ring, \
             and a chain of colored hexagonal and round ghosts), 35mm (35 mm prime: a small core, a sharp starburst, \
             small round ghosts), 105mm (105 mm prime: a large bright glow with a chromatic halo ring and a few big \
             soft ghosts), or movie (movie prime: a long horizontal blue anamorphic streak and oval ghosts). Ghosts \
             lie on the line from the flare through the image center; every size scales with the canvas. The result \
             lists the layer, its bounds, and each ghost's position and radius.",
    )
    .param(
        param("at", ParamKind::Point)
            .default("30%,30%")
            .help("Flare center X,Y in px or % of the canvas (negative counts from the right/bottom)"),
    )
    .param(
        param("lens", ParamKind::Choice)
            .choices(LENSES)
            .default("zoom")
            .help("Lens type: zoom (50-300 mm), 35mm prime, 105mm prime, or movie prime"),
    )
    .param(param("brightness", ParamKind::Float).range("10..300").default("100").help("Flare brightness in percent"))
    .param(
        param("color", ParamKind::Color).help("Tint the whole flare with this color (default: the lens's own colors)"),
    )
    .param(
        param("layer", ParamKind::Layer)
            .help("Put the flare layer above this layer (default: above the active layer, or on top)"),
    )
    .param(param("name", ParamKind::Str).help("Name for the new layer (default: Lens Flare, made unique)"))
    .example("pixelox render lens-flare photo.jpg -o flare.jpg --at 72%,22%", "Flare in the upper right of a photo")
    .example(
        "pixelox render lens-flare doc.pxo --in-place --at 1200,300 --lens 105mm --brightness 140",
        "A bright telephoto flare on its own layer",
    )
    .example(
        "pixelox render lens-flare doc.pxo --in-place --at 50%,40% --lens movie --color '#9fd4ff'",
        "A blue anamorphic movie flare",
    )
    .example(
        "pixelox render lens-flare doc.pxo --at 20%,15% --lens 35mm --dry-run --json",
        "Plan a flare and see its ghosts",
    )]
}

// ---------------------------------------------------------------------------------------
// the recipe

#[derive(Clone, Copy, Debug, PartialEq)]
enum Lens {
    Zoom,
    Prime35,
    Prime105,
    Movie,
}

impl Lens {
    fn parse(s: &str) -> Result<Lens> {
        Ok(match s.trim() {
            "zoom" => Lens::Zoom,
            "35mm" => Lens::Prime35,
            "105mm" => Lens::Prime105,
            "movie" => Lens::Movie,
            other => {
                return Err(Error::new("PX1010", format!("--lens: '{other}' is not one of: {}", LENSES.join(", ")))
                    .with_context("lens", other))
            }
        })
    }
}

/// A ghost of the recipe: position `t` along the flare → center line (0 = the flare,
/// 1 = the center, 2 = the mirror point), radius as a fraction of the diagonal,
/// aperture sides (0 = round), color, intensity, and rim emphasis (0 = filled disc,
/// 1 = a ring).
type GhostDef = (f64, f64, u32, [f64; 3], f64, f64);

struct Recipe {
    core: (f64, f64, [f64; 3]),
    glow: (f64, f64, [f64; 3]),
    veil: (f64, f64),
    /// radius, width, intensity, color split.
    halo: Option<(f64, f64, f64, f64)>,
    /// count, length, intensity, width.
    spikes: (usize, f64, f64, f64),
    /// count, length, intensity.
    hairs: (usize, f64, f64),
    /// length, width, intensity, color (horizontal, anamorphic).
    streak: Option<(f64, f64, f64, [f64; 3])>,
    ghosts: &'static [GhostDef],
    /// Vertical stretch of the ghosts (anamorphic ovals).
    aspect: f64,
    /// Aperture rotation in degrees.
    blades: f64,
}

const WARM: [f64; 3] = [1.0, 0.74, 0.44];

fn recipe(lens: Lens) -> Recipe {
    match lens {
        Lens::Zoom => Recipe {
            core: (0.0045, 9.0, [1.0, 0.97, 0.9]),
            glow: (0.04, 0.8, WARM),
            veil: (0.12, 0.1),
            halo: Some((0.15, 0.006, 0.14, 0.03)),
            spikes: (12, 0.09, 0.7, 0.0009),
            hairs: (64, 0.09, 0.3),
            streak: None,
            ghosts: &[
                (0.27, 0.011, 6, [0.55, 1.0, 0.35], 0.5, 0.25),
                (0.44, 0.022, 6, [1.0, 0.55, 0.2], 0.3, 0.45),
                (0.6, 0.007, 0, [0.75, 0.45, 1.0], 0.6, 0.0),
                (0.82, 0.04, 6, [0.3, 0.5, 1.0], 0.17, 0.6),
                (1.24, 0.015, 0, [0.4, 1.0, 0.6], 0.38, 0.2),
                (1.45, 0.055, 6, [0.95, 0.4, 0.9], 0.15, 0.8),
                (1.68, 0.01, 0, [1.0, 0.7, 0.3], 0.55, 0.0),
                (1.95, 0.075, 0, [0.5, 0.8, 1.0], 0.12, 1.0),
                (2.25, 0.026, 6, [0.4, 0.95, 1.0], 0.26, 0.4),
            ],
            aspect: 1.0,
            blades: 12.0,
        },
        Lens::Prime35 => Recipe {
            core: (0.0035, 10.0, [1.0, 0.98, 0.94]),
            glow: (0.026, 0.75, [1.0, 0.88, 0.72]),
            veil: (0.08, 0.08),
            halo: None,
            spikes: (16, 0.11, 1.0, 0.0008),
            hairs: (96, 0.12, 0.35),
            streak: None,
            ghosts: &[
                (0.34, 0.008, 0, [0.5, 1.0, 0.5], 0.55, 0.2),
                (0.55, 0.014, 0, [1.0, 0.6, 0.3], 0.38, 0.3),
                (0.74, 0.005, 0, [0.6, 0.6, 1.0], 0.7, 0.0),
                (1.3, 0.02, 0, [0.4, 0.7, 1.0], 0.26, 0.5),
                (1.62, 0.012, 0, [1.0, 0.5, 0.8], 0.36, 0.2),
                (2.1, 0.035, 0, [0.6, 1.0, 0.7], 0.16, 0.9),
            ],
            aspect: 1.0,
            blades: 0.0,
        },
        Lens::Prime105 => Recipe {
            core: (0.0075, 8.0, [1.0, 0.95, 0.85]),
            glow: (0.06, 1.1, [1.0, 0.78, 0.5]),
            veil: (0.16, 0.16),
            halo: Some((0.085, 0.011, 0.35, 0.05)),
            spikes: (8, 0.07, 0.5, 0.0012),
            hairs: (24, 0.06, 0.2),
            streak: None,
            ghosts: &[
                (0.5, 0.03, 0, [1.0, 0.7, 0.35], 0.24, 0.3),
                (1.3, 0.05, 0, [0.45, 0.6, 1.0], 0.17, 0.5),
                (1.7, 0.016, 0, [0.6, 1.0, 0.5], 0.36, 0.1),
                (2.2, 0.09, 0, [1.0, 0.6, 0.8], 0.12, 1.0),
            ],
            aspect: 1.0,
            blades: 0.0,
        },
        Lens::Movie => Recipe {
            core: (0.004, 9.0, [0.92, 0.96, 1.0]),
            glow: (0.03, 0.8, [0.6, 0.78, 1.0]),
            veil: (0.1, 0.1),
            halo: None,
            spikes: (6, 0.05, 0.35, 0.0009),
            hairs: (16, 0.05, 0.18),
            streak: Some((0.5, 0.0022, 1.3, [0.42, 0.68, 1.0])),
            ghosts: &[
                (0.4, 0.009, 0, [0.4, 0.7, 1.0], 0.5, 0.3),
                (0.65, 0.016, 0, [0.5, 1.0, 0.9], 0.3, 0.5),
                (1.2, 0.006, 0, [0.6, 0.6, 1.0], 0.6, 0.0),
                (1.5, 0.025, 0, [0.3, 0.6, 1.0], 0.2, 0.7),
                (1.9, 0.012, 0, [0.8, 0.6, 1.0], 0.36, 0.2),
                (2.3, 0.04, 0, [0.4, 0.8, 1.0], 0.15, 0.9),
            ],
            aspect: 1.7,
            blades: 0.0,
        },
    }
}

// ---------------------------------------------------------------------------------------
// the plan: the recipe placed on this canvas, with the RNG's small variations

struct Ghost {
    x: f64,
    y: f64,
    r: f64,
    sides: u32,
    color: [f64; 3],
    i: f64,
    rim: f64,
}

struct Ray {
    dir: (f64, f64),
    len: f64,
    width: f64,
    i: f64,
}

struct Plan {
    at: (f64, f64),
    core: (f64, f64, [f64; 3]),
    glow: (f64, f64, [f64; 3]),
    veil: (f64, f64),
    halo: Option<(f64, f64, f64, f64)>,
    rays: Vec<Ray>,
    streak: Option<(f64, f64, f64, [f64; 3])>,
    ghosts: Vec<Ghost>,
    aspect: f64,
    blades: f64,
}

/// A uniform draw in `lo..hi`.
fn uniform(rng: &mut impl RngCore, lo: f64, hi: f64) -> f64 {
    lo + (hi - lo) * (rng.next_u64() >> 11) as f64 / (1u64 << 53) as f64
}

fn plan(lens: Lens, at: (f64, f64), size: (u32, u32), rng: &mut impl RngCore) -> Plan {
    let rc = recipe(lens);
    let s = (size.0 as f64).hypot(size.1 as f64).max(1.0);
    let center = (size.0 as f64 / 2.0, size.1 as f64 / 2.0);
    let axis = (center.0 - at.0, center.1 - at.1);
    let px = |f: f64, min: f64| (f * s).max(min);

    let mut rays = Vec::new();
    let (n, len, i, width) = rc.spikes;
    let turn = uniform(rng, 0.0, TAU);
    for k in 0..n {
        let th = turn + k as f64 * TAU / n as f64 + uniform(rng, -0.05, 0.05);
        rays.push(Ray {
            dir: (th.cos(), th.sin()),
            len: px(len * uniform(rng, 0.6, 1.25), 2.0),
            width: px(width, 0.6),
            i: i * uniform(rng, 0.6, 1.2),
        });
    }
    let (n, len, i) = rc.hairs;
    for _ in 0..n {
        let th = uniform(rng, 0.0, TAU);
        rays.push(Ray {
            dir: (th.cos(), th.sin()),
            len: px(len * uniform(rng, 0.25, 1.0), 1.5),
            width: px(0.0005, 0.5),
            i: i * uniform(rng, 0.3, 1.0),
        });
    }
    let ghosts = rc
        .ghosts
        .iter()
        .map(|&(t, r, sides, color, i, rim)| Ghost {
            x: at.0 + t * axis.0,
            y: at.1 + t * axis.1,
            r: px(r * uniform(rng, 0.9, 1.1), 1.0),
            sides,
            color,
            i: i * uniform(rng, 0.85, 1.15),
            rim,
        })
        .collect();
    Plan {
        at,
        core: (px(rc.core.0, 1.0), rc.core.1, rc.core.2),
        glow: (px(rc.glow.0, 2.0), rc.glow.1, rc.glow.2),
        veil: (px(rc.veil.0, 4.0), rc.veil.1),
        halo: rc.halo.map(|(r, w, i, split)| (px(r, 3.0), px(w, 0.8), i, split)),
        rays,
        streak: rc.streak.map(|(l, w, i, c)| (px(l, 4.0), px(w, 0.7), i, c)),
        ghosts,
        aspect: rc.aspect,
        blades: (rc.blades + uniform(rng, -4.0, 4.0)).to_radians(),
    }
}

#[inline]
fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl Plan {
    /// Linear light at pixel center (x, y).
    fn light(&self, x: f64, y: f64) -> [f64; 3] {
        let mut e = [0.0f64; 3];
        let add = |e: &mut [f64; 3], c: [f64; 3], k: f64| {
            for ch in 0..3 {
                e[ch] += c[ch] * k;
            }
        };
        let (dx, dy) = (x - self.at.0, y - self.at.1);
        let r = dx.hypot(dy);
        let (cr, ci, cc) = self.core;
        add(&mut e, cc, ci * (-(r / cr) * (r / cr)).exp());
        let (gr, gi, gc) = self.glow;
        add(&mut e, gc, gi * (-r / gr).exp());
        let (vr, vi) = self.veil;
        let q = 1.0 + (r / vr) * (r / vr);
        add(&mut e, gc, vi / (q * q));
        if let Some((hr, hw, hi, split)) = self.halo {
            if (r - hr).abs() < hr * split * 2.0 + 4.0 * hw {
                for (ch, k) in [1.0, 0.0, -1.0].iter().enumerate() {
                    let d = (r - hr * (1.0 + split * k)) / hw;
                    e[ch] += hi * (-d * d).exp();
                }
            }
        }
        for ray in &self.rays {
            let along = dx * ray.dir.0 + dy * ray.dir.1;
            if along <= 0.0 {
                continue;
            }
            let perp = (dx * ray.dir.1 - dy * ray.dir.0).abs();
            if perp > 4.0 * ray.width {
                continue;
            }
            let t = along / ray.len;
            let k = ray.i * (-(perp / ray.width) * (perp / ray.width)).exp() * (-0.5 * t - 0.5 * t * t).exp();
            add(&mut e, cc, k);
        }
        if let Some((len, width, i, c)) = self.streak {
            let k = i * (-(dy / width) * (dy / width)).exp() * (-dx.abs() / len).exp()
                + i * 0.18 * (-(dy / (width * 6.0)) * (dy / (width * 6.0))).exp() * (-dx.abs() / (len * 0.6)).exp();
            add(&mut e, c, k);
        }
        for g in &self.ghosts {
            let (gx, gy) = (x - g.x, (y - g.y) / self.aspect);
            let reach = g.r * 1.35;
            if gx.abs() > reach || gy.abs() > reach {
                continue;
            }
            let d = gx.hypot(gy);
            // distance in units of the aperture's radius in this direction
            let mut rho = d / g.r;
            if g.sides >= 3 {
                let seg = TAU / g.sides as f64;
                let local = (gy.atan2(gx) - self.blades).rem_euclid(seg) - seg / 2.0;
                rho *= local.cos() / (PI / g.sides as f64).cos();
            }
            let soft = if g.sides >= 3 { 0.07 } else { 0.12 };
            for (ch, k) in [-1.0, 0.0, 1.0].iter().enumerate() {
                // red a little larger, blue a little smaller: a slight color fringe
                let p = rho * (1.0 + 0.035 * k);
                let inside = 1.0 - smoothstep(1.0 - soft, 1.0 + soft, p);
                if inside <= 0.0 {
                    continue;
                }
                let body = (1.0 - g.rim) * (0.7 + 0.3 * (1.0 - p * p).max(0.0));
                let rim = g.rim * (0.15 + 0.85 * p.min(1.0).powi(6));
                e[ch] += g.i * g.color[ch] * inside * (body + rim);
            }
        }
        e
    }
}

/// The flare as a straight-alpha layer image over the whole canvas, dithered by half an
/// 8-bit step (seeded) so the long, dark glow gradients do not band.
fn draw(p: &Plan, w: u32, h: u32, gain: f64, tint: [f64; 3], dither: u64) -> Image {
    let mut img = Image::new(w, h);
    let row_len = w as usize * 4;
    img.data.par_chunks_mut(row_len.max(1)).enumerate().for_each(|(y, row)| {
        for x in 0..w as usize {
            let e = p.light(x as f64 + 0.5, y as f64 + 0.5);
            let mut c = [0, 1, 2].map(|ch| (1.0 - (-e[ch] * gain * tint[ch]).exp()) as f32);
            let n = mix64(dither ^ ((y as u64) << 32 | x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let d = ((n >> 40) as f32 / (1u64 << 24) as f32 - 0.5) / 255.0;
            for v in c.iter_mut() {
                if *v > 0.0 {
                    *v = (*v + d).clamp(0.0, 1.0);
                }
            }
            let a = c[0].max(c[1]).max(c[2]);
            // below half an 8-bit step: nothing (keeps the layer's bounds tight)
            if a < 0.5 / 255.0 {
                continue;
            }
            let px = &mut row[x * 4..x * 4 + 4];
            px.copy_from_slice(&[c[0] / a, c[1] / a, c[2] / a, a]);
        }
    });
    img
}

fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

fn lens_flare(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (w, h) = (doc.width, doc.height);
    let at =
        a.point("at")?.map(|p| p.resolve(w as f64, h as f64, doc.dpi())).unwrap_or((w as f64 * 0.3, h as f64 * 0.3));
    let lens_name = a.choice("lens");
    let lens = Lens::parse(&lens_name)?;
    let brightness = a.f64("brightness")?;
    let tint = match a.opt_color("color")? {
        None => [1.0; 3],
        Some(c) => {
            // tint by hue and saturation, keeping the flare's brightness
            let v = [c.r as f64, c.g as f64, c.b as f64];
            let m = v[0].max(v[1]).max(v[2]);
            if m <= 1e-6 {
                return Err(Error::new("PX1010", "--color: a black tint would remove the flare")
                    .with_hint("give a light color such as '#9fd4ff', or leave --color out"));
            }
            v.map(|x| x / m)
        }
    };
    let above = match a.raw("layer") {
        Some(r) => Some(doc.resolve(r)?),
        None => doc.active.clone().and_then(|id| doc.find_by_id(&id)),
    };
    let name = match a.str("name") {
        Some(n) => {
            doc.check_name_free(&n, None)?;
            n
        }
        None => "Lens Flare".to_string(),
    };

    let mut rng = ctx.rng();
    let p = plan(lens, at, (w, h), &mut rng);
    let full = draw(&p, w, h, brightness / 100.0, tint, rng.next_u64());
    let (image, x, y) = match full.alpha_bbox(0.0) {
        Some(r) if (r.w, r.h) != (w, h) => (full.crop(r), r.x, r.y),
        Some(_) => (full, 0, 0),
        None => (Image::new(w.max(1), h.max(1)), 0, 0),
    };
    let mut layer = doc.new_layer(&name, LayerKind::Raster { image });
    layer.x = x;
    layer.y = y;
    layer.blend = BlendMode::Screen;
    let (id, lname) = (layer.id.clone(), layer.name.clone());
    let path = doc.insert_above(above.as_deref(), layer);
    let bounds = compose::layer_bounds(doc, doc.layer(&path))?.map(|b| b.to_json()).unwrap_or(Value::Null);
    let ghosts: Vec<Value> = p
        .ghosts
        .iter()
        .map(|g| {
            json!({
                "x": round1(g.x),
                "y": round1(g.y),
                "radius": round1(g.r),
                "shape": match g.sides {
                    6 => "hexagon".to_string(),
                    n if n >= 3 => format!("{n}-gon"),
                    _ if p.aspect != 1.0 => "oval".to_string(),
                    _ => "circle".to_string(),
                },
            })
        })
        .collect();
    Ok(json!({
        "layer": lname,
        "id": id,
        "above": above.map(|p| doc.layer(&p).name.clone()),
        "blend": "screen",
        "lens": lens_name,
        "at": [round1(at.0), round1(at.1)],
        "axis_through": [round1(w as f64 / 2.0), round1(h as f64 / 2.0)],
        "brightness": brightness,
        "streak": p.streak.is_some(),
        "ghosts": ghosts,
        "bounds": bounds,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lens_names_parse() {
        for l in LENSES {
            assert!(Lens::parse(l).is_ok());
        }
        assert_eq!(Lens::parse("fisheye").unwrap_err().code, "PX1010");
    }

    #[test]
    fn hexagon_ghosts_have_flat_sides() {
        // a 6-gon ghost reaches farther toward a corner than across a flat side
        let p = Plan {
            at: (-1000.0, -1000.0),
            core: (1.0, 0.0, [0.0; 3]),
            glow: (1.0, 0.0, [0.0; 3]),
            veil: (1.0, 0.0),
            halo: None,
            rays: vec![],
            streak: None,
            ghosts: vec![Ghost { x: 50.0, y: 50.0, r: 20.0, sides: 6, color: [1.0; 3], i: 1.0, rim: 0.0 }],
            aspect: 1.0,
            blades: 0.0,
        };
        // corners at 0°, 60°, ...; flat sides face 30°, 90°, ...
        let corner = p.light(50.0 + 19.0, 50.0)[1];
        let flat = p.light(50.0, 50.0 + 19.0)[1];
        assert!(corner > 0.3 && flat < corner * 0.5, "corner {corner} flat {flat}");
    }
}
