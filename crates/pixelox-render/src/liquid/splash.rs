//! `render splash`: a crown splash (a ring wall rising from a pool and breaking into
//! spikes tipped with drops) or a sheet splash (streams thrown along ballistic arcs that
//! merge into a sheet near their origin and break into drops toward their tips).

use super::field::{self, Prim};
use super::{add_layer, liquid_look, r1, Draw, LIQUIDS};
use pixelox_core::prelude::*;
use pixelox_core::units::Length;
use std::f64::consts::{PI, TAU};

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("render splash", Category::Render, "Liquid splash (crown or sheet) on a new layer", splash)
        .about(
            "Renders a liquid splash onto a new layer named 'Splash' above --layer (or the active layer, or on top). \
             --kind crown: a pool at --at whose rim rises into a wall that breaks into spikes tipped with drops (a \
             drop landing, a product dropped in). --kind sheet: streams thrown from --at toward --angle (90 = up), \
             fanned over --spread degrees, falling back under gravity, merged into one sheet near the origin and \
             breaking into drops at their tips (a drink poured or thrown). --size is the crown's width or the \
             sheet's reach. To stand a product in a crown, render --part back below the product and --part front \
             above it with the same --seed, --at, and --size. The liquid is lit like glass: --liquid water (clear, with a dark refracting rim and a \
             bright caustic), milk, juice, cola, coffee, wine, or oil; --color recolors it, --highlight tints the \
             highlights, --opacity fades the body. Neighboring strands and drops merge with soft liquid bridges. \
             --seed draws another splash. The result lists the layer, its bounds, and the counts.",
        )
        .param(param("kind", ParamKind::Choice).choices(&["crown", "sheet"]).default("crown").help("crown (a ring rising from a pool) or sheet (streams thrown from a point)"))
        .param(param("at", ParamKind::Point).default("50%,70%").help("Crown: center of the pool; sheet: where the liquid is thrown from (X,Y, px or %)"))
        .param(param("size", ParamKind::Length).default("60%").help("Crown width or sheet reach in px or % of the shorter canvas side"))
        .param(param("angle", ParamKind::Angle).default("90").help("Sheet: throw direction in degrees (90 = up, 45 = up and right); crown: its axis"))
        .param(param("spread", ParamKind::Float).default("120").range("10..360").help("Sheet: how wide the streams fan out, in degrees"))
        .param(param("part", ParamKind::Choice).choices(&["all", "back", "front"]).default("all").help("Crown: all, back (the pool and far wall: under a product), or front (the near wall: over it)"))
        .param(param("drops", ParamKind::Int).auto().default("auto").range("0..2000").help("Flying drops around the splash (auto: 40)"))
        .param(param("liquid", ParamKind::Choice).choices(LIQUIDS).default("water").help("water, milk, juice, cola, coffee, wine, or oil"))
        .param(param("color", ParamKind::Color).help("Recolor the liquid (keeps its clarity): e.g. '#ff5a1f' for orange soda"))
        .param(param("highlight", ParamKind::Color).default("#ffffff").help("Color of the highlights and reflections"))
        .param(param("opacity", ParamKind::Float).default("100").range("0..100").help("Opacity of the splash in percent"))
        .param(param("layer", ParamKind::Layer).help("Put the splash layer above this layer (default: above the active layer, or on top)"))
        .param(param("name", ParamKind::Str).help("Name for the new layer (default: Splash)"))
        .example("pixelox render splash doc.pxo --in-place --at 50%,88% --size 80%", "A water crown around a product's base")
        .example("pixelox render splash doc.pxo --in-place --kind sheet --at 50%,75% --angle 70 --spread 140 --liquid juice", "Orange juice thrown up and to the right")
        .example("pixelox render splash doc.pxo --in-place --kind sheet --liquid milk --at 30%,80% --angle 60 --size 90% --name Milk", "A milk splash on its own layer")
        .example("pixelox render splash photo.jpg -o splash.jpg --liquid cola --seed 7", "A cola crown on a photo")
        .example("pixelox render splash doc.pxo --in-place --part front --layer Can --at 50%,90% --size 70% --seed 4 --name SplashFront", "The near wall of a crown in front of a can")]
}

/// A crown splash centered at `c` with width `w` (y grows down).
fn crown(c: (f64, f64), w: f64, axis: f64, drops: usize, d: &mut Draw) -> (Vec<(Prim, bool)>, usize) {
    // each primitive with whether it is on the near (front) side of the crown
    let mut p: Vec<(Prim, bool)> = Vec::new();
    let (rx, ry, height) = (0.33 * w, 0.33 * w * 0.3, 0.42 * w);
    // the axis tilt: rotate everything about the base center
    let tilt = (axis - 90.0).to_radians();
    let rot = |q: (f64, f64)| {
        let (dx, dy) = (q.0 - c.0, q.1 - c.1);
        let (s, co) = (-tilt).sin_cos();
        (c.0 + dx * co - dy * s, c.1 + dx * s + dy * co)
    };
    // the pool: a flat, filled ellipse with an uneven edge, wider than the crown
    for (ring, count) in [(0.0, 1), (0.4, 9), (0.8, 16), (1.15, 22), (1.45, 26)] {
        for k in 0..count {
            let th = TAU * (k as f64 + d.range(-0.3, 0.3)) / count as f64;
            let q = (c.0 + rx * ring * th.cos(), c.1 + ry * ring * 0.95 * th.sin() + 0.02 * w);
            let r = ry * if ring > 1.3 { d.range(0.28, 0.5) } else { d.range(0.55, 0.7) };
            // the outer ring's near half laps over a product standing in the pool
            p.push((Prim::sphere(q, r), ring > 1.0 && th.sin() > 0.25));
        }
    }
    // the wall: a solid band around the ellipse, built from rings of smooth chains stacked
    // up to a wavy top (each ring flows smoothly; stacked rings merge into one sheet)
    let phase = d.range(0.0, TAU);
    let band_top = |th: f64| height * (0.28 + 0.07 * (3.0 * th + phase).sin());
    let segs = 48;
    let rings = 4;
    for j in 0..rings {
        let f = j as f64 / (rings - 1) as f64;
        let r = w * (0.026 - 0.008 * f);
        let point = |th: f64| {
            let base = (c.0 + rx * th.cos(), c.1 + ry * th.sin());
            let up = band_top(th) * f * 0.92;
            (base.0 + (base.0 - c.0) * 0.12 * f, base.1 - up)
        };
        // the near half and the far half are separate chains (for --part)
        for (front, k0, k1) in [(true, 0, segs / 2), (false, segs / 2, segs)] {
            let g = 2 + 2 * j as u32 + front as u32;
            for k in k0..k1 {
                let (t0, t1) = (TAU * k as f64 / segs as f64, TAU * (k + 1) as f64 / segs as f64);
                p.push((Prim::capsule(point(t0), point(t1), r, r).in_group(g), front));
            }
        }
    }
    // spikes rising from the band, each ending in a bulb, many flinging a drop
    let spikes = 22;
    let mut flung = 0;
    for k in 0..spikes {
        let th = TAU * (k as f64 + d.range(-0.3, 0.3)) / spikes as f64;
        let base = (c.0 + rx * th.cos(), c.1 + ry * th.sin());
        let hb = band_top(th);
        let root = (base.0 + (base.0 - c.0) * 0.1, base.1 - hb * 0.85);
        // the far (upper) side looks taller from slightly above
        let len = height * d.range(0.25, 0.7) * (1.0 - 0.35 * th.sin());
        let out = d.range(0.25, 0.5);
        let tip = (root.0 + (base.0 - c.0) * out * len / height * 1.4, root.1 - len);
        let mid = ((root.0 + tip.0) / 2.0 + (base.0 - c.0) * 0.04, (root.1 + tip.1) / 2.0);
        let (r0, r1, r2) = (0.02 * w, 0.011 * w, 0.006 * w);
        let g = 10 + k as u32;
        let front = th.sin() > 0.0;
        p.push((Prim::capsule(root, mid, r0, r1).in_group(g), front));
        p.push((Prim::capsule(mid, tip, r1, r2).in_group(g), front));
        let bulb = w * d.range(0.011, 0.017);
        let (ux, uy) = {
            let (vx, vy) = (tip.0 - mid.0, tip.1 - mid.1);
            let m = vx.hypot(vy).max(1e-9);
            (vx / m, vy / m)
        };
        p.push((Prim::sphere((tip.0 + ux * bulb * 0.5, tip.1 + uy * bulb * 0.5), bulb), front));
        if d.u() < 0.65 {
            let gap = w * d.range(0.02, 0.06) + bulb * 2.0;
            let r = bulb * d.range(0.6, 1.0);
            p.push((Prim::sphere((tip.0 + ux * gap, tip.1 + uy * gap), r), front));
            flung += 1;
        }
    }
    // spray: drops thrown up and outward, smaller the farther they fly
    for k in 0..drops {
        let phi = d.range(-PI + 0.15, -0.15);
        let f = d.u();
        let dist = height * (0.75 + 1.1 * f);
        let q = (c.0 + phi.cos() * dist * 1.3, c.1 - 0.2 * height + phi.sin() * dist * 0.85);
        let r = w * (0.004 + 0.011 * (1.0 - f) * d.u());
        let len = r * d.range(0.0, 2.5);
        let (ux, uy) = (phi.cos(), phi.sin());
        p.push((Prim::capsule((q.0 - ux * len, q.1 - uy * len), q, r * 0.6, r), k % 3 == 0));
    }
    let p = p.into_iter().map(|(q, f)| (Prim { a: rot(q.a), b: rot(q.b), ..q }, f)).collect();
    (p, flung)
}

/// A sheet splash thrown from `o` toward `angle` with reach `l` (y grows down).
fn sheet(o: (f64, f64), l: f64, angle: f64, spread: f64, drops: usize, d: &mut Draw) -> (Vec<Prim>, usize) {
    let mut p = vec![Prim::sphere(o, 0.05 * l)];
    let m = 22;
    let g = 1.1 * l;
    let mut ends = Vec::new();
    for i in 0..m {
        let phi = (angle + (i as f64 / (m - 1) as f64 - 0.5) * spread + d.range(-4.0, 4.0)).to_radians();
        let speed = l * d.range(0.7, 1.08);
        let v = (phi.cos() * speed, -phi.sin() * speed);
        let t_end = d.range(0.65, 1.0);
        let t_break = t_end * d.range(0.62, 0.82);
        let r0 = 0.04 * l * d.range(0.7, 1.05);
        let r_end = 0.005 * l;
        let wob = d.range(0.0, TAU);
        let at = |t: f64| (o.0 + v.0 * t, o.1 + v.1 * t + 0.5 * g * t * t);
        let radius = |t: f64| {
            r_end + (r0 - r_end) * (1.0 - t / t_end).max(0.0).powf(1.4) * (1.0 + 0.12 * (t * 23.0 + wob).sin())
        };
        let segs = 16;
        let mut t0 = 0.0;
        for j in 1..=segs {
            let t1 = t_break * j as f64 / segs as f64;
            p.push(Prim::capsule(at(t0), at(t1), radius(t0), radius(t1)).in_group(100 + i as u32));
            t0 = t1;
        }
        // the tip breaks into a string of shrinking drops
        let mut t = t_break;
        let mut r = radius(t_break) * 1.15;
        while t < t_end && r > 0.6 {
            t += (r * 2.6 + l * d.range(0.005, 0.02)) / speed;
            if t > t_end {
                break;
            }
            p.push(Prim::sphere(at(t), r));
            r *= d.range(0.6, 0.85);
        }
        ends.push((at(t.min(t_end)), phi, speed));
    }
    for _ in 0..drops {
        let (e, phi, speed) = ends[(d.u() * ends.len() as f64) as usize % ends.len()];
        let f = d.u();
        let dist = speed * d.range(0.05, 0.35);
        let a = phi + d.range(-0.5, 0.5);
        let q = (e.0 + a.cos() * dist, e.1 - a.sin() * dist + f * 0.1 * l);
        let r = l * (0.003 + 0.01 * d.u() * (1.0 - 0.6 * f));
        p.push(Prim::capsule((q.0 - a.cos() * r * 1.5, q.1 + a.sin() * r * 1.5), q, r * 0.6, r));
    }
    (p, m)
}

fn splash(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (w, h) = (doc.width as f64, doc.height as f64);
    let short = w.min(h).max(1.0);
    let at = a.point("at")?.map(|p| p.resolve(w, h, doc.dpi())).unwrap_or((w / 2.0, h * 0.7));
    let size =
        a.length("size")?.unwrap_or(Length { value: 60.0, unit: units::Unit::Pct }).to_px(short, doc.dpi()).max(4.0);
    let kind = a.choice("kind");
    let angle = a.angle("angle")?;
    let spread = a.f64("spread")?;
    let drops = match a.opt_int("drops")? {
        Some(n) => n as usize,
        None => {
            ctx.resolve_param("drops", 40);
            40
        }
    };
    let liquid = a.choice("liquid");
    let look = liquid_look(&liquid, a.opt_color("color")?, a.color("highlight")?, (a.f64("opacity")? / 100.0) as f32);
    let mut d = Draw(ctx.rng());
    let part = a.choice("part");
    let (prims, n) = if kind == "sheet" {
        if part != "all" {
            return Err(
                Error::new("PX1005", "--part applies to --kind crown").with_hint("drop --part, or use --kind crown")
            );
        }
        sheet(at, size, angle, spread, drops, &mut d)
    } else {
        let (all, n) = crown(at, size, angle, drops, &mut d);
        let keep = |front: bool| match part.as_str() {
            "back" => !front,
            "front" => front,
            _ => true,
        };
        (all.into_iter().filter(|(_, f)| keep(*f)).map(|(p, _)| p).collect(), n)
    };
    let clip = RectI::new(-(w as i32), -(h as i32), doc.width * 3, doc.height * 3);
    let rendered = match field::field(&prims, clip) {
        Some(f) => field::shade(&f, &look),
        None => Rendered { image: Image::new(1, 1), x: 0, y: 0 },
    };
    // keep what lies on the canvas
    let on = RectI::new(0, 0, doc.width, doc.height);
    let rendered = Rendered { image: rendered.to_canvas(doc.width, doc.height).crop(on), x: 0, y: 0 };
    let path = add_layer(a, doc, rendered, "Splash")?;
    let l = doc.layer(&path);
    let bounds = compose::layer_bounds(doc, l)?.map(|b| b.to_json()).unwrap_or(Value::Null);
    let mut v = json!({
        "layer": l.name,
        "id": l.id,
        "kind": kind,
        "liquid": liquid,
        "part": part,
        "at": [r1(at.0), r1(at.1)],
        "size_px": r1(size),
        "drops": drops,
        "bounds": bounds,
    });
    v[if kind == "sheet" { "streams" } else { "drops_on_spikes" }] = json!(n);
    Ok(v)
}
