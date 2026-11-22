//! `render droplets`: condensation on a cold product. Beads of many sizes (most small,
//! some merged), a few running drops with trails, and a fine mist, drawn on a new layer
//! clipped to the product. Each bead is a little lens: it shows the product behind it
//! turned upside down and magnified, darker at its upper rim, with a sharp highlight and
//! a bright caustic at its foot.

use super::field::{self, Prim};
use super::{add_layer, r1, Draw};
use pixelox_core::prelude::*;
use rayon::prelude::*;

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("render droplets", Category::Render, "Condensation droplets clipped to a layer", droplets)
        .about(
            "Covers --layer (a can, bottle, or glass) with condensation on a new layer named 'Droplets' directly \
             above it and clipped to it, so beads appear only on the product. Beads come in many sizes (most small, \
             up to --size), never overlap, and --density sets how much of the surface they cover; --trickles adds \
             drops running down with a trail; --mist adds a fine frosting of tiny beads. With --refract on (the \
             default) each bead shows the product behind it upside down and magnified, like real water; the \
             refraction is computed once, so re-run the command after changing the product. --color tints the \
             water. Without --layer the whole canvas is covered (on top, not clipped); --above/--below/--into \
             place the layer elsewhere (then it is not clipped). --seed draws another arrangement.",
        )
        .param(param("layer", ParamKind::Layer).help("The product layer to bead (default: the whole canvas)"))
        .param(
            param("density", ParamKind::Float)
                .default("45")
                .range("0..100")
                .help("How much of the surface the beads cover, in percent of the maximum"),
        )
        .param(
            param("size", ParamKind::Length)
                .auto()
                .default("auto")
                .help("Largest bead radius in px or % of the product width (auto: 2.4%)"),
        )
        .param(
            param("trickles", ParamKind::Int)
                .auto()
                .default("auto")
                .range("0..200")
                .help("Drops running down with a trail (auto: 3)"),
        )
        .param(
            param("mist", ParamKind::Float)
                .default("30")
                .range("0..100")
                .help("Fine mist of tiny beads (frosting), in percent"),
        )
        .param(
            param("refract", ParamKind::Switch)
                .default("on")
                .help("on: beads show the product behind them (lens); off: clear tinted beads"),
        )
        .param(param("color", ParamKind::Color).default("#ffffff").help("Tint of the water (white = clear)"))
        .param(param("name", ParamKind::Str).help("Name for the new layer (default: Droplets)"))
        .params(common::placement())
        .example("pixelox render droplets doc.pxo --in-place --layer Can", "Bead a can with condensation")
        .example(
            "pixelox render droplets doc.pxo --in-place --layer Bottle --density 70 --trickles 6 --mist 60",
            "A frosty, dripping bottle",
        )
        .example(
            "pixelox render droplets doc.pxo --in-place --layer Glass --size 1.5% --refract off --seed 4",
            "Small clear beads on a glass",
        )]
}

/// A bead: center and radius.
#[derive(Clone, Copy)]
struct Bead {
    x: f64,
    y: f64,
    r: f64,
}

fn droplets(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (cw, ch) = (doc.width, doc.height);
    // the target: its rendered pixels (for the region and the refraction)
    let placed_by_flag = pixelox_core::stack::from_args(a, doc)?.is_some();
    let (target, tname) = match a.raw("layer") {
        Some(r) => {
            let p = doc.resolve(r)?;
            let l = doc.layer(&p).clone();
            let rendered = compose::render_layer(doc, &l)?.ok_or_else(|| {
                Error::new("PX3343", format!("layer '{}' draws nothing to put droplets on", l.name))
                    .with_hint("pick the product layer (pixelox layer list <doc> shows the layers)")
                    .with_context("layer", &l.name)
            })?;
            (Some(rendered.to_canvas(cw, ch)), Some(l.name))
        }
        None => (None, None),
    };
    let region = match &target {
        Some(t) => t.alpha_bbox(0.5).ok_or_else(|| {
            Error::new("PX3343", "the product layer has no solid pixels to put droplets on")
                .with_hint("pick a layer with opaque pixels")
        })?,
        None => RectI::new(0, 0, cw, ch),
    };
    let size = match a.length("size")? {
        Some(l) => l.to_px(region.w as f64, doc.dpi()),
        None => {
            ctx.resolve_param("size", "2.4%");
            region.w as f64 * 0.024
        }
    }
    .max(1.0);
    let density = a.f64("density")? / 100.0;
    let trickles = match a.opt_int("trickles")? {
        Some(n) => n as usize,
        None => {
            ctx.resolve_param("trickles", 3);
            3
        }
    };
    let mist = a.f64("mist")? / 100.0;
    let refract = a.switch("refract", true) && target.is_some();
    let tint = a.color("color")?;
    let inside = |x: f64, y: f64| match &target {
        Some(t) => {
            let (xi, yi) = (x as i64, y as i64);
            xi >= 0 && yi >= 0 && xi < cw as i64 && yi < ch as i64 && t.get(xi as u32, yi as u32)[3] > 0.5
        }
        None => true,
    };
    let mut d = Draw(ctx.rng());
    // beads by dart throwing on a grid: no overlaps, most small
    let cell = (size * 2.0).max(2.0);
    let (gw, gh) = (((region.w as f64) / cell).ceil() as usize + 1, ((region.h as f64) / cell).ceil() as usize + 1);
    let mut grid: Vec<Vec<usize>> = vec![Vec::new(); gw * gh];
    let mut beads: Vec<Bead> = Vec::new();
    let mean_area = std::f64::consts::PI * (size * 0.35).powi(2);
    let target_n = ((region.w as f64 * region.h as f64) * 0.42 * density / mean_area) as usize;
    let attempts = target_n * 6 + 10;
    let fits = |b: &Bead, beads: &[Bead], grid: &[Vec<usize>]| {
        let (gx, gy) = (((b.x - region.x as f64) / cell) as i64, ((b.y - region.y as f64) / cell) as i64);
        for j in gy - 1..=gy + 1 {
            for i in gx - 1..=gx + 1 {
                if i < 0 || j < 0 || i >= gw as i64 || j >= gh as i64 {
                    continue;
                }
                for &k in &grid[j as usize * gw + i as usize] {
                    let o = &beads[k];
                    if (o.x - b.x).hypot(o.y - b.y) < o.r + b.r + 1.0 {
                        return false;
                    }
                }
            }
        }
        true
    };
    for _ in 0..attempts {
        if beads.len() >= target_n {
            break;
        }
        let u = d.u();
        let b = Bead {
            x: region.x as f64 + d.u() * region.w as f64,
            y: region.y as f64 + d.u() * region.h as f64,
            r: size * (0.14 + 0.86 * u * u * u),
        };
        if !inside(b.x, b.y) || !fits(&b, &beads, &grid) {
            continue;
        }
        let (gx, gy) = (((b.x - region.x as f64) / cell) as usize, ((b.y - region.y as f64) / cell) as usize);
        grid[gy.min(gh - 1) * gw + gx.min(gw - 1)].push(beads.len());
        beads.push(b);
    }
    let mut prims: Vec<Prim> = beads
        .iter()
        .map(|b| {
            // gravity: a little taller than wide, heavier at the foot; beads touching each
            // other stay separate (one group: their maximum, no liquid bridges)
            Prim::capsule((b.x, b.y - 0.2 * b.r), (b.x, b.y + 0.12 * b.r), 0.82 * b.r, b.r).in_group(u32::MAX)
        })
        .collect();
    // running drops: a trail down to a heavy drop
    for t_i in 0..trickles {
        let x = region.x as f64 + d.range(0.08, 0.92) * region.w as f64;
        let y0 = region.y as f64 + d.range(0.05, 0.6) * region.h as f64;
        let len = region.h as f64 * d.range(0.12, 0.35);
        let r_drop = size * d.range(1.0, 1.5);
        let steps = 10;
        let mut prev = (x, y0);
        let phase = d.range(0.0, std::f64::consts::TAU);
        for k in 1..=steps {
            let t = k as f64 / steps as f64;
            let q = (x + (t * 7.0 + phase).sin() * size * 0.35, y0 + t * len);
            if !inside(q.0, q.1) {
                break;
            }
            prims.push(
                Prim::capsule(prev, q, size * 0.22 * (0.6 + 0.4 * t), size * 0.25 * (0.6 + 0.4 * t))
                    .in_group(1 + t_i as u32),
            );
            prev = q;
        }
        if inside(prev.0, prev.1) {
            prims.push(Prim::capsule(
                (prev.0, prev.1 - r_drop * 0.4),
                (prev.0, prev.1 + r_drop * 0.2),
                r_drop * 0.75,
                r_drop,
            ));
        }
    }
    // mist: tiny beads
    let n_mist = ((region.w as f64 * region.h as f64) * mist * 0.012 / (size * 0.12 + 0.6).powi(2)) as usize;
    let mut mists = 0;
    for _ in 0..n_mist {
        let (x, y) = (region.x as f64 + d.u() * region.w as f64, region.y as f64 + d.u() * region.h as f64);
        if inside(x, y) {
            prims.push(Prim::sphere((x, y), (size * 0.06 + 0.45) * d.range(0.7, 1.6)));
            mists += 1;
        }
    }
    let clip = RectI::new(0, 0, cw, ch);
    let rendered = match field::field(&prims, clip) {
        Some(f) => shade(&f, target.as_ref().filter(|_| refract), [tint.r, tint.g, tint.b]),
        None => Rendered { image: Image::new(1, 1), x: 0, y: 0 },
    };
    // directly above the product: clip to it (a placement flag puts it elsewhere)
    let path = if target.is_some() && placed_by_flag {
        let r = rendered;
        let mut b = Args::new();
        for k in ["above", "below", "into", "name"] {
            if let Some(v) = a.raw(k) {
                b.set(k, v);
            }
        }
        add_layer(&b, doc, r, "Droplets")?
    } else {
        add_layer(a, doc, rendered, "Droplets")?
    };
    if target.is_some() && !placed_by_flag {
        doc.layer_mut(&path).clip = true;
    }
    let l = doc.layer(&path);
    let bounds = compose::layer_bounds(doc, l)?.map(|b| b.to_json()).unwrap_or(Value::Null);
    Ok(json!({
        "layer": l.name,
        "id": l.id,
        "on": tname,
        "clipped": l.clip,
        "beads": beads.len(),
        "trickles": trickles,
        "mist": mists,
        "size_px": r1(size),
        "refract": refract,
        "bounds": bounds,
    }))
}

#[inline]
fn smooth(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Shade beads: lenses over `behind` (the product on the canvas), or clear tinted water.
fn shade(f: &field::Field, behind: Option<&Image>, tint: [f32; 3]) -> Rendered {
    let (w, h) = (f.w, f.h);
    let l = {
        let v = [-0.45f32, -0.7, 0.55];
        let m = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        [v[0] / m, v[1] / m, v[2] / m]
    };
    let hv = {
        let v = [l[0], l[1], l[2] + 1.0];
        let m = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        [v[0] / m, v[1] / m, v[2] / m]
    };
    let mut img = Image::new(w as u32, h as u32);
    img.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let Some(sf) = f.surface(x, y) else { continue };
            let (cx, cy) = (f.x0 as f64 + x as f64 + 0.5, f.y0 as f64 + y as f64 + 0.5);
            let s = sf.s;
            // the bead's local radius from how deep the pixel sits (exact for a lone bead)
            let r_est = (sf.inset.max(0.0) / (1.0 - s.min(0.9))).clamp(0.5, 200.0);
            let mut px = [0f32; 4];
            let mut put = |c: [f32; 3], a: f32| {
                if a <= 0.0 {
                    return;
                }
                let ao = a + px[3] * (1.0 - a);
                for i in 0..3 {
                    px[i] = (c[i] * a + px[i] * px[3] * (1.0 - a)) / ao.max(1e-6);
                }
                px[3] = ao;
            };
            match behind {
                Some(b) => {
                    // an inverted, magnified view of the surface behind
                    let off = 1.7 * s * r_est;
                    let sx = cx - sf.dir.0 as f64 * off as f64;
                    let sy = cy - sf.dir.1 as f64 * off as f64;
                    let c = b.sample_bilinear(sx - 0.5, sy - 0.5);
                    let base = if c[3] > 0.05 { [c[0], c[1], c[2]] } else { [0.5; 3] };
                    let k = 0.82 - 0.25 * smooth(0.5, 1.0, s);
                    put([base[0] * k * tint[0], base[1] * k * tint[1], base[2] * k * tint[2]], 1.0);
                    // the upper rim refracts the dark: a crescent shadow
                    let up = smooth(-0.2, 0.9, -sf.dir.1);
                    put([0.0; 3], smooth(0.6, 1.0, s) * (0.25 + 0.4 * up));
                }
                None => {
                    let a_body = 0.12 + 0.6 * smooth(0.55, 1.0, s);
                    put([tint[0] * 0.55, tint[1] * 0.6, tint[2] * 0.66], a_body);
                }
            }
            // caustic at the foot, away from the light
            let away = -(sf.dir.0 * l[0] + sf.dir.1 * l[1]) / (l[0] * l[0] + l[1] * l[1]).sqrt();
            put([1.0, 1.0, 1.0], 0.5 * smooth(0.25, 0.95, away) * smooth(0.45, 0.85, s) * (1.0 - smooth(0.9, 1.0, s)));
            // the highlight
            let n = sf.n;
            let spec = (n[0] * hv[0] + n[1] * hv[1] + n[2] * hv[2]).max(0.0).powf(45.0);
            put([1.0, 1.0, 1.0], (spec * 1.2).min(1.0));
            px[3] *= sf.cov;
            row[x * 4..x * 4 + 4].copy_from_slice(&px);
        }
    });
    Rendered { image: img, x: f.x0, y: f.y0 }
}
