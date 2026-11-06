//! Drawing media: `filter pencil` (graphite or colored pencil), `filter charcoal`,
//! `filter ink` (pen and ink).

use super::*;
use crate::util;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("filter pencil", CAT, "Pencil sketch: graphite lines and hatching", pencil)
            .about("Turns the layer into a pencil drawing on paper. Outlines come from a directional stroke filter (the dark side of each edge becomes a soft graphite line drawn along its direction, slightly overshooting, after Lu, Xu & Jia 2012); tones are stretched, lifted toward paper white, and rendered with seeded hatching strokes along --angle, crosshatched in the darks; paper grain breaks up the graphite. --color keeps the photo's colors under the strokes (colored pencil). Sizes scale with the layer (unit √(w·h)/1000 px), so a 1 MP and a 24 MP photo look alike. Alpha is kept.")
            .param(param("stroke", ParamKind::Float).auto().default("auto").range("0.2..200").help("Pencil line width in px; auto = 1.1 × √(w·h)/1000 (recorded)"))
            .param(param("detail", ParamKind::Float).default("50").range("0..100").help("Outline detail: 0 = only major edges, 100 = fine texture too"))
            .param(param("edges", ParamKind::Float).default("60").range("0..100").help("Outline strength (0 = no outlines, shading only)"))
            .param(param("hatching", ParamKind::Float).default("50").range("0..100").help("Tonal shading with hatching strokes (0 = outlines only)"))
            .param(param("angle", ParamKind::Angle).default("45").help("Hatching direction in degrees counter-clockwise from the right"))
            .param(param("paper", ParamKind::Color).default("#f7f4ec").help("Paper color"))
            .param(param("color", ParamKind::Flag).help("Colored pencil: keep the photo's colors under the strokes"))
            .params(common::region())
            .example("pixelox filter pencil portrait.jpg -o sketch.png", "Graphite sketch with default hatching")
            .example("pixelox filter pencil photo.jpg -o colored.png --color --hatching 70", "Colored pencil drawing")
            .example("pixelox filter pencil photo.jpg -o lines.png --hatching 0 --edges 80 --detail 70", "Outlines only, more detail")
            .example("pixelox filter pencil doc.pxo --layer Photo --angle 120 --paper '#fffdf5' --in-place", "Left-handed hatching on whiter paper"),
        OpSpec::doc("filter charcoal", CAT, "Charcoal drawing: bold smudged strokes", charcoal)
            .about("Turns the layer into a charcoal drawing: tones are pushed to strong darks and paper whites, mid-tones become streaky strokes along --angle, edges are drawn bold, the whole is smudged along the strokes, and paper tooth leaves white specks in the blacks. --balance shifts the light/dark split; --pigment changes the stick (for example sanguine). Sizes scale with the layer. Alpha is kept.")
            .param(param("stroke", ParamKind::Float).auto().default("auto").range("0.2..400").help("Stroke width in px; auto = 2.5 × √(w·h)/1000 (recorded)"))
            .param(param("detail", ParamKind::Float).default("40").range("0..100").help("Edge detail: 0 = only major edges, 100 = fine texture too"))
            .param(param("balance", ParamKind::Float).default("0").range("-100..100").help("Light/dark balance: + more of the image goes dark, − lighter"))
            .param(param("smudge", ParamKind::Float).default("50").range("0..100").help("How much the charcoal is smeared along the strokes"))
            .param(param("angle", ParamKind::Angle).default("60").help("Stroke direction in degrees counter-clockwise from the right"))
            .param(param("pigment", ParamKind::Color).default("#1d1b19").help("Stick color (#1d1b19 charcoal; #8a3b24 sanguine, #3b2a20 sepia)"))
            .param(param("paper", ParamKind::Color).default("#f1ede3").help("Paper color"))
            .params(common::region())
            .example("pixelox filter charcoal portrait.jpg -o charcoal.png", "Charcoal drawing")
            .example("pixelox filter charcoal photo.jpg -o light.png --balance -40 --smudge 20", "Lighter, crisper drawing")
            .example("pixelox filter charcoal photo.jpg -o sanguine.png --pigment '#8a3b24' --paper '#efe2cc'", "Red chalk on toned paper")
            .example("pixelox filter charcoal doc.pxo --mask subject --in-place", "Charcoal inside a mask only"),
        OpSpec::doc("filter ink", CAT, "Pen and ink: crisp lines, solid blacks", ink)
            .about("Turns the layer into a pen-and-ink drawing: an extended difference of Gaussians (XDoG, Winnemöller 2012) on normalized luminance, thresholded to near-binary lines with solid blacks where the image is darkest. --fill sets how much of the darks become solid ink; --shading adds hand-drawn hatching or stippling in the mid-tones. Sizes scale with the layer. Alpha is kept.")
            .param(param("stroke", ParamKind::Float).auto().default("auto").range("0.2..200").help("Line width in px; auto = 1.0 × √(w·h)/1000 (recorded)"))
            .param(param("detail", ParamKind::Float).default("50").range("0..100").help("Line detail: 0 = only major edges, 100 = fine texture too"))
            .param(param("fill", ParamKind::Float).default("35").range("0..100").help("How much of the dark areas become solid ink"))
            .param(param("shading", ParamKind::Choice).choices(&["none", "hatch", "stipple"]).default("none").help("Mid-tone shading: none, hatch (crosshatched lines), or stipple (dots)"))
            .param(param("angle", ParamKind::Angle).default("45").help("Hatching direction in degrees counter-clockwise from the right"))
            .param(param("ink", ParamKind::Color).default("#141414").help("Ink color"))
            .param(param("paper", ParamKind::Color).default("#fbfaf5").help("Paper color"))
            .params(common::region())
            .example("pixelox filter ink portrait.jpg -o ink.png", "Ink line drawing")
            .example("pixelox filter ink photo.jpg -o hatched.png --shading hatch --fill 20", "Crosshatched pen drawing")
            .example("pixelox filter ink photo.jpg -o dots.png --shading stipple --ink navy", "Stippled drawing in blue ink")
            .example("pixelox filter ink doc.pxo --detail 80 --stroke 2 --in-place", "Finer detail with 2 px lines"),
    ]
}

/// Paper and pigment colors as RGB.
fn rgb3(c: Rgba) -> [f32; 3] {
    [c.r, c.g, c.b]
}

// ---------------------------------------------------------------------------------------
// pencil

pub(crate) struct Pencil {
    pub stroke: f64,
    pub detail: f32,
    pub edges: f32,
    pub hatching: f32,
    pub angle: f64,
    pub paper: [f32; 3],
    pub colored: bool,
}

fn pencil(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (lx, ly) = util::layer_offset(a, doc)?;
    let mut p = Pencil {
        stroke: 0.0,
        detail: pct(a, "detail")?,
        edges: pct(a, "edges")?,
        hatching: pct(a, "hatching")?,
        angle: a.angle("angle")?,
        paper: rgb3(a.color("paper")?),
        colored: a.flag("color"),
    };
    let v = apply_to_raster(a, doc, ctx, |img, ctx| {
        if img.is_empty() {
            return Ok(img.clone());
        }
        p.stroke = size_param(a, ctx, "stroke", 1.1 * unit(img.width as usize, img.height as usize))?;
        let tex = Tex::new(ctx, lx, ly);
        Ok(with_alpha(img, &pencil_render(&solid_rgb(img), &p, tex)))
    })?;
    Ok(sized(v, "stroke", p.stroke))
}

/// Add the resolved size to the result (not for smart filters, which run later).
fn sized(v: Value, key: &str, px: f64) -> Value {
    if px > 0.0 {
        util::merge(v, json!({ key: px }))
    } else {
        v
    }
}

/// Edge-tangent angle (radians, image coordinates) from a smoothed structure tensor of
/// the gradient `(gx, gy)`.
fn tangent_angles(gx: &Buf, gy: &Buf, sigma: f64) -> Vec<f32> {
    let mut st = Buf::new(gx.w, gx.h, 3);
    st.data.par_chunks_mut(3).zip(gx.data.par_iter().zip(gy.data.par_iter())).for_each(|(o, (&x, &y))| {
        o[0] = x * x;
        o[1] = x * y;
        o[2] = y * y;
    });
    gauss_in(&mut st, sigma);
    st.data.par_chunks(3).map(|s| 0.5 * (2.0 * s[1]).atan2(s[0] - s[2]) + std::f32::consts::FRAC_PI_2).collect()
}

/// Lu-style stroke map: gradient magnitude `mag` split by edge direction into eight
/// orientations, each drawn as lines of length `len` along its orientation, and summed.
/// A straight edge of contrast c gives about c along its length, extended by `len`/2 past
/// its ends; texture with no coherent direction averages away.
pub(crate) fn directional_strokes(mag: &Buf, theta: &[f32], len: f64) -> Buf {
    let (w, h) = (mag.w, mag.h);
    let mut acc = Buf::new(w, h, 1);
    const N: usize = 8;
    let step = std::f32::consts::PI / N as f32;
    for i in 0..N {
        let ang = i as f32 * step;
        let mut ci = Buf::new(w, h, 1);
        ci.data.par_iter_mut().zip(mag.data.par_iter().zip(theta.par_iter())).for_each(|(o, (&m, &t))| {
            let mut d = (t - ang).rem_euclid(std::f32::consts::PI);
            if d > std::f32::consts::FRAC_PI_2 {
                d = std::f32::consts::PI - d;
            }
            let wt = (1.0 - d / step).max(0.0);
            *o = m * wt;
        });
        // image angle (y down) → kernel angle (counter-clockwise, y up)
        let lines = kern::motion_blur(&ci, -(ang as f64).to_degrees(), len, Edge::Clamp);
        acc.data.par_iter_mut().zip(lines.data.par_iter()).for_each(|(a, &b)| *a += b);
    }
    acc
}

/// Scale-normalized gradient magnitude of `l` smoothed at `sigma`: a step edge of
/// contrast c peaks at about c whatever the scale. Also returns the gradient.
pub(crate) fn edge_strength(l: &Buf, sigma: f64) -> (Buf, Buf, Buf) {
    let ls = gauss(l, sigma);
    let (gx, gy) = gradient(&ls);
    let norm = (sigma.max(0.6) * (2.0 * std::f64::consts::PI).sqrt()) as f32;
    let mut m = Buf::new(l.w, l.h, 1);
    m.data.par_iter_mut().zip(gx.data.par_iter().zip(gy.data.par_iter())).for_each(|(o, (&x, &y))| {
        *o = (x * x + y * y).sqrt() * norm;
    });
    (m, gx, gy)
}

/// Dark-side line response: a difference of Gaussians (σ, 2.5σ) that is positive where a
/// pixel is darker than its surroundings, normalized so a step edge of contrast c peaks at
/// about c (1.5σ inside the dark side) and a thin dark line gives one centered response.
pub(crate) fn dark_lines(l: &Buf, sigma: f64) -> Buf {
    let g1 = gauss(l, sigma);
    let g2 = gauss(l, 2.5 * sigma);
    let mut m = g1;
    m.data.par_iter_mut().zip(g2.data.par_iter()).for_each(|(a, &b)| *a = ((b - *a) / 0.208).max(0.0));
    m
}

/// Pencil tone: luma stretched between its 0.5th and 99.5th percentiles, then lifted
/// (drawings are lighter than photos) so black maps to dense graphite and white to paper.
pub(crate) fn pencil_tone(l: &Buf) -> Buf {
    let (lo, hi) = tone_range(l, 0.005);
    let span = hi - lo;
    let mut out = l.clone();
    out.data.par_iter_mut().for_each(|v| {
        let n = ((*v - lo) / span).clamp(0.0, 1.0);
        *v = ((0.12 + 0.88 * n.powf(0.5)) * 1.08).min(1.0);
    });
    out
}

pub(crate) fn pencil_render(rgb: &Buf, p: &Pencil, tex: Tex) -> Buf {
    let (w, h) = (rgb.w, rgb.h);
    let u = p.stroke;
    let l = luma(rgb);
    // outlines
    let se = (u * (1.0 - 0.6 * p.detail as f64)).max(0.35);
    let (_, gx, gy) = edge_strength(&l, se);
    let theta = tangent_angles(&gx, &gy, 2.0 * u);
    let lines = directional_strokes(&dark_lines(&l, se), &theta, 9.0 * u);
    let k_edge = 8.0 * p.edges;
    let floor = 0.03 + 0.05 * (1.0 - p.detail);
    // tone and hatching
    let tone = pencil_tone(&gauss(&l, 1.5 * u));
    let f1 = stroke_field(w, h, tex.salt(10), p.angle, 26.0 * u, 1.2 * u, 1.7);
    let f2 = stroke_field(w, h, tex.salt(11), p.angle - 75.0, 26.0 * u, 1.2 * u, 1.7);
    let grain = smooth_noise(w, h, tex.salt(20), 0.7 * u);
    let hatch = 1.6 * p.hatching;
    let mut gray = Buf::new(w, h, 1);
    gray.data.par_iter_mut().enumerate().for_each(|(i, o)| {
        let z = grain.data[i].clamp(-2.5, 2.5);
        let tooth = (1.0 + 0.15 * z).max(0.3);
        let s = (lines.data[i] - floor).max(0.0);
        let dl = (1.0 - (-k_edge * s).exp()) * (0.85 + 0.15 * tooth).min(1.0);
        let j = tone.data[i].clamp(0.02, 1.0);
        let cross = util::smoothstep(0.45, 0.75, 1.0 - j);
        let f = f1.data[i] * (1.0 - 0.5 * cross) + f2.data[i] * 0.5 * cross;
        let t = j.powf(f * tooth * hatch);
        *o = (1.0 - dl) * t;
    });
    let gmin = 0.1f32;
    let mut out = Buf::new(w, h, 3);
    if !p.colored {
        out.data.par_chunks_mut(3).zip(gray.data.par_iter()).for_each(|(o, &g)| {
            let v = gmin + (1.0 - gmin) * g;
            for c in 0..3 {
                o[c] = p.paper[c] * v;
            }
        });
        return out;
    }
    // colored pencil: lightness from the drawing, hue and chroma from the photo where
    // there is pigment
    let base = to_oklab(&gauss(rgb, 0.6 * u));
    out.data.par_chunks_mut(3).zip(gray.data.par_iter()).zip(base.data.par_chunks(3)).for_each(|((o, &g), b)| {
        let v = gmin + (1.0 - gmin) * g;
        let lv = color::srgb_to_linear(v).cbrt();
        let k = 1.3 * (0.3 + 0.7 * (1.0 - v).powf(0.7));
        let (r, gg, bb) = color::oklab_to_rgb(lv, b[1] * k, b[2] * k);
        let rgbv = [r, gg, bb];
        for c in 0..3 {
            o[c] = p.paper[c] * rgbv[c].clamp(0.0, 1.0);
        }
    });
    out
}

// ---------------------------------------------------------------------------------------
// charcoal

pub(crate) struct Charcoal {
    pub stroke: f64,
    pub detail: f32,
    pub balance: f32,
    pub smudge: f32,
    pub angle: f64,
    pub pigment: [f32; 3],
    pub paper: [f32; 3],
}

fn charcoal(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (lx, ly) = util::layer_offset(a, doc)?;
    let mut p = Charcoal {
        stroke: 0.0,
        detail: pct(a, "detail")?,
        balance: pct(a, "balance")?,
        smudge: pct(a, "smudge")?,
        angle: a.angle("angle")?,
        pigment: rgb3(a.color("pigment")?),
        paper: rgb3(a.color("paper")?),
    };
    let v = apply_to_raster(a, doc, ctx, |img, ctx| {
        if img.is_empty() {
            return Ok(img.clone());
        }
        p.stroke = size_param(a, ctx, "stroke", 2.5 * unit(img.width as usize, img.height as usize))?;
        let tex = Tex::new(ctx, lx, ly);
        Ok(with_alpha(img, &charcoal_render(&solid_rgb(img), &p, tex)))
    })?;
    Ok(sized(v, "stroke", p.stroke))
}

pub(crate) fn charcoal_render(rgb: &Buf, p: &Charcoal, tex: Tex) -> Buf {
    let (w, h) = (rgb.w, rgb.h);
    let u = p.stroke;
    let l = luma(rgb);
    let ls = gauss(&l, 0.5 * u);
    let (lo, hi) = tone_range(&ls, 0.02);
    let span = hi - lo;
    // bold lines on the dark side of edges
    let se = (u * (0.7 - 0.4 * p.detail as f64)).max(0.35);
    let (_, gx, gy) = edge_strength(&l, se);
    let theta = tangent_angles(&gx, &gy, 1.5 * u);
    let lines = directional_strokes(&dark_lines(&l, se), &theta, 5.0 * u);
    let floor = 0.03 + 0.06 * (1.0 - p.detail);
    // broad strokes carrying the tone, slightly blurred so darks stay dense
    // two stroke families 25° apart, so the marks are less mechanical
    let mut field = stroke_field(w, h, tex.salt(30), p.angle, 12.0 * u, 1.6 * u, 2.2);
    let other = stroke_field(w, h, tex.salt(33), p.angle + 25.0, 9.0 * u, 1.6 * u, 2.2);
    field.data.par_iter_mut().zip(other.data.par_iter()).for_each(|(a, &b)| *a = 0.6 * *a + 0.4 * b);
    gauss_in(&mut field, 0.35 * u);
    let thr = 0.45 + 0.25 * p.balance;
    let mut cov = Buf::new(w, h, 1);
    cov.data.par_iter_mut().enumerate().for_each(|(i, o)| {
        let n = ((ls.data[i] - lo) / span).clamp(0.0, 1.0).powf(1.1);
        let j = 0.03 + 0.97 * util::smoothstep(thr - 0.5, thr + 0.5, n);
        // gaps between strokes still take some charcoal, so darks stay dense
        let t = j.powf(0.45 + 0.55 * field.data[i]);
        let dl = 1.0 - (-5.0 * (lines.data[i] - floor).max(0.0)).exp();
        *o = 1.0 - t * (1.0 - dl);
    });
    // smudge along the strokes and a little around, keeping some crisp marks on top
    let sm = p.smudge;
    if sm > 0.0 {
        let mut b = kern::motion_blur(&cov, p.angle, 3.5 * u * sm as f64, Edge::Clamp);
        gauss_in(&mut b, 0.6 * u * sm as f64);
        let k = 0.25 + 0.6 * sm;
        cov.data.par_iter_mut().zip(b.data.par_iter()).for_each(|(c, &s)| *c += (s - *c) * k);
    }
    // paper tooth: a few white specks where the paper's valleys stay clean
    let grain = smooth_noise(w, h, tex.salt(31), 0.3 * u);
    let fine = smooth_noise(w, h, tex.salt(32), 0.8 * u);
    let mut out = Buf::new(w, h, 3);
    out.data.par_chunks_mut(3).enumerate().for_each(|(i, o)| {
        let speck = util::smoothstep(1.3, 2.4, grain.data[i]);
        let c = cov.data[i] * (1.0 - 0.25 * speck) * (1.0 - 0.06 * fine.data[i].clamp(-2.0, 2.0));
        let c = c.clamp(0.0, 1.0);
        for ch in 0..3 {
            o[ch] = p.paper[ch] + (p.pigment[ch] - p.paper[ch]) * c;
        }
    });
    out
}

// ---------------------------------------------------------------------------------------
// ink

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Shading {
    None,
    Hatch,
    Stipple,
}

pub(crate) struct Ink {
    pub stroke: f64,
    pub detail: f32,
    pub fill: f32,
    pub shading: Shading,
    pub angle: f64,
    pub ink: [f32; 3],
    pub paper: [f32; 3],
}

fn ink(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (lx, ly) = util::layer_offset(a, doc)?;
    let mut p = Ink {
        stroke: 0.0,
        detail: pct(a, "detail")?,
        fill: pct(a, "fill")?,
        shading: match a.choice("shading").as_str() {
            "hatch" => Shading::Hatch,
            "stipple" => Shading::Stipple,
            _ => Shading::None,
        },
        angle: a.angle("angle")?,
        ink: rgb3(a.color("ink")?),
        paper: rgb3(a.color("paper")?),
    };
    let mut inked = 0.0;
    let v = apply_to_raster(a, doc, ctx, |img, ctx| {
        if img.is_empty() {
            return Ok(img.clone());
        }
        p.stroke = size_param(a, ctx, "stroke", unit(img.width as usize, img.height as usize))?;
        let tex = Tex::new(ctx, lx, ly);
        let cov = ink_coverage(&solid_rgb(img), &p, tex);
        inked = mean_std(&cov.data, cov.w.max(1)).0;
        let mut out = Buf::new(cov.w, cov.h, 3);
        out.data.par_chunks_mut(3).zip(cov.data.par_iter()).for_each(|(o, &c)| {
            for ch in 0..3 {
                o[ch] = p.paper[ch] + (p.ink[ch] - p.paper[ch]) * c;
            }
        });
        Ok(with_alpha(img, &out))
    })?;
    let v = sized(v, "stroke", p.stroke);
    Ok(util::merge(v, json!({"ink_coverage": (inked * 1000.0).round() / 1000.0})))
}

/// Ink coverage 0..1 per pixel.
pub(crate) fn ink_coverage(rgb: &Buf, p: &Ink, tex: Tex) -> Buf {
    let (w, h) = (rgb.w, rgb.h);
    let u = p.stroke;
    // flatten texture (fabric, skin, noise) but keep edges: fewer specks in the ink
    let (flat, f) = abstraction(rgb, 2.5 * u, 8.0, 1);
    let flat = if f > 1 { util::upsample(&flat, w, h, f) } else { flat };
    let mut l = luma(&flat);
    let (lo, hi) = tone_range(&l, 0.01);
    let span = hi - lo;
    l.data.par_iter_mut().for_each(|v| *v = ((*v - lo) / span).clamp(0.0, 1.0));
    let sigma = (u * (1.1 - 0.6 * p.detail as f64)).max(0.35);
    let g1 = gauss(&l, sigma);
    let g2 = gauss(&l, 1.6 * sigma);
    let sharp = 22.0f32;
    let eps = 0.05 + 0.4 * p.fill;
    let phi = 45.0f32;
    let mut cov = Buf::new(w, h, 1);
    cov.data.par_iter_mut().enumerate().for_each(|(i, o)| {
        let s = g1.data[i] + sharp * (g1.data[i] - g2.data[i]);
        *o = if s >= eps { 0.0 } else { -(phi * (s - eps)).tanh() };
    });
    if p.shading == Shading::None {
        return cov;
    }
    // mid-tone darkness for shading, from the smoothed tone
    let tone = gauss(&l, 1.2 * u);
    let sp = (3.6 * u).max(2.0);
    let wob = smooth_noise(w, h, tex.salt(40), 14.0 * u);
    let (ca, sa) = ((p.angle.to_radians().cos()) as f32, (-(p.angle.to_radians()).sin()) as f32);
    let a2 = (p.angle + 90.0).to_radians();
    let (cb, sb) = (a2.cos() as f32, (-a2.sin()) as f32);
    let spf = sp as f32;
    let (ox, oy) = (tex.ox as f32, tex.oy as f32);
    let t_dash = tex.salt(41);
    let t_dot = tex.salt(42);
    let range = 0.45f32;
    cov.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let i = y * w + x;
            let d = 1.0 - util::smoothstep(eps, eps + range, tone.data[i]);
            if d <= 0.02 {
                continue;
            }
            let (px, py) = (x as f32 + ox, y as f32 + oy);
            let shade = match p.shading {
                Shading::Hatch => {
                    let wv = 0.18 * wob.data[i];
                    let mut c = hatch_line(px, py, ca, sa, spf, wv, d, t_dash, 0.12);
                    if d > 0.5 {
                        c = c.max(hatch_line(px, py, cb, sb, spf, wv, (d - 0.5) * 2.0, t_dash.salt(7), 0.12));
                    }
                    c
                }
                _ => stipple(px, py, spf * 0.8, d, (u as f32 * 0.55).max(0.6), t_dot),
            };
            row[x] = row[x].max(shade);
        }
    });
    cov
}

/// Coverage of a hatching line family along direction (ca, sa) with spacing `sp` px at
/// darkness `d`: lines get wider with darkness and break into dashes of random length.
fn hatch_line(px: f32, py: f32, ca: f32, sa: f32, sp: f32, wobble: f32, d: f32, t: Tex, gap: f32) -> f32 {
    // across-line coordinate (in line spacings) and along-line position (px)
    let across = (-sa * px + ca * py) / sp + wobble;
    let along = ca * px + sa * py;
    let li = across.floor();
    let frac = across - li - 0.5;
    let half = (0.12 + 0.3 * d.clamp(0.0, 1.0)) * sp;
    let dist = frac.abs() * sp;
    let line = (half - dist + 0.5).clamp(0.0, 1.0);
    if line <= 0.0 {
        return 0.0;
    }
    // dashes: segments of ~6 spacings with random offsets per line; a few are skipped
    let seg = 6.0 * sp;
    let off = unit_f(hash2(t.key, li as i64, 0)) * seg;
    let si = ((along + off) / seg).floor();
    let keep = unit_f(hash2(t.key, li as i64, si as i64 + 1));
    if keep < gap + 0.25 * (1.0 - d) {
        return 0.0;
    }
    line
}

/// Coverage of seeded stipple dots on a jittered grid of cell `cell` px: a cell's dot is
/// drawn when its random rank is below the darkness `d`.
fn stipple(px: f32, py: f32, cell: f32, d: f32, radius: f32, t: Tex) -> f32 {
    let (gx, gy) = ((px / cell).floor() as i64, (py / cell).floor() as i64);
    let mut best = 0f32;
    for j in -1..=1 {
        for i in -1..=1 {
            let (cx, cy) = (gx + i, gy + j);
            let hsh = hash2(t.key, cx, cy);
            if unit_f(mix(hsh ^ 3)) >= d {
                continue;
            }
            let dx = (cx as f32 + 0.15 + 0.7 * unit_f(hsh)) * cell - px;
            let dy = (cy as f32 + 0.15 + 0.7 * unit_f(mix(hsh))) * cell - py;
            let dist = (dx * dx + dy * dy).sqrt();
            best = best.max((radius - dist + 0.5).clamp(0.0, 1.0));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::super::fixture::*;
    use super::*;

    /// Light background with a mid-gray square in the middle.
    fn square(w: u32, h: u32) -> Image {
        Image::from_fn(w, h, |x, y| {
            let inside = x >= w / 4 && x < 3 * w / 4 && y >= h / 4 && y < 3 * h / 4;
            let v = if inside { 0.45 } else { 0.9 };
            [v, v, v, 1.0]
        })
    }

    #[test]
    fn pencil_is_mostly_paper_with_dark_lines_at_edges() {
        let src = square(120, 80);
        let (out, v) = run("filter pencil", &[("stroke", "1.5"), ("hatching", "30")], &src);
        assert_eq!(v["stroke"], json!(1.5));
        let l = lum(&out);
        let mean: f32 = l.iter().sum::<f32>() / l.len() as f32;
        assert!(mean > 0.7, "mostly light: {mean}");
        // the square's border (dark side, just inside) is darker than both flat areas
        let edge = mean_rect(&out, 32, 30, 34, 50);
        let flat_out = mean_rect(&out, 4, 30, 14, 50);
        let flat_in = mean_rect(&out, 50, 30, 70, 50);
        assert!(edge < flat_out - 0.15 && edge < flat_in - 0.05, "edge {edge}, outside {flat_out}, inside {flat_in}");
        // darker regions get more graphite than lighter ones
        assert!(flat_in < flat_out, "tone: {flat_in} vs {flat_out}");
    }

    #[test]
    fn pencil_without_hatching_leaves_flat_areas_as_paper() {
        let gray = Image::filled(60, 40, Rgba::new(0.4, 0.4, 0.4, 1.0));
        let (out, _) = run("filter pencil", &[("hatching", "0"), ("paper", "#ffffff")], &gray);
        assert!(out.data.chunks(4).all(|p| p[0] > 0.999 && p[1] > 0.999), "no strokes on a flat field");
    }

    #[test]
    fn pencil_hatching_runs_along_the_angle() {
        let gray = Image::filled(120, 120, Rgba::new(0.45, 0.45, 0.45, 1.0));
        for (angle, along, across) in [("0", (4i32, 0i32), (0i32, 4i32)), ("90", (0, 4), (4, 0))] {
            let (out, _) = run("filter pencil", &[("angle", angle), ("stroke", "1.5")], &gray);
            let l = lum(&out);
            let diff = |d: (i32, i32)| {
                let mut s = 0.0;
                for y in 20..100 {
                    for x in 20..100 {
                        let a = l[y * 120 + x];
                        let b = l[(y as i32 + d.1) as usize * 120 + (x as i32 + d.0) as usize];
                        s += (a - b).abs();
                    }
                }
                s
            };
            assert!(diff(along) < 0.8 * diff(across), "angle {angle}: along {} across {}", diff(along), diff(across));
        }
    }

    #[test]
    fn colored_pencil_keeps_hue() {
        let src = Image::from_fn(60, 40, |x, _| if x < 30 { [0.85, 0.15, 0.1, 1.0] } else { [0.95, 0.95, 0.95, 1.0] });
        let (gray, _) = run("filter pencil", &[("paper", "#ffffff")], &src);
        let (col, _) = run("filter pencil", &[("paper", "#ffffff"), ("color", "true")], &src);
        let avg = |img: &Image, c: usize| {
            (5..25).flat_map(|x| (10..30).map(move |y| (x, y))).map(|(x, y)| img.get(x, y)[c]).sum::<f32>() / 400.0
        };
        assert!((avg(&gray, 0) - avg(&gray, 1)).abs() < 0.02, "graphite is gray");
        assert!(avg(&col, 0) > avg(&col, 1) + 0.15, "red stays red: {} {}", avg(&col, 0), avg(&col, 1));
    }

    #[test]
    fn charcoal_is_contrasty_smudged_and_takes_the_pigment() {
        let src = scene(120, 90);
        let (out, _) = run("filter charcoal", &[], &src);
        let l = lum(&out);
        let dark = l.iter().filter(|&&v| v < 0.35).count() as f32 / l.len() as f32;
        let light = l.iter().filter(|&&v| v > 0.75).count() as f32 / l.len() as f32;
        assert!(dark > 0.05 && light > 0.25, "dark {dark} light {light}");
        // the dark disc is solid charcoal, the sky is paper
        assert!(mean_rect(&out, 80, 36, 88, 44) < 0.3 && mean_rect(&out, 90, 5, 115, 15) > 0.7);
        let (crisp, _) = run("filter charcoal", &[("smudge", "0")], &src);
        let (soft, _) = run("filter charcoal", &[("smudge", "100")], &src);
        assert!(roughness(&soft, 10, 10, 110, 80) < roughness(&crisp, 10, 10, 110, 80));
        let (red, _) = run("filter charcoal", &[("pigment", "#8a3b24")], &src);
        let p = red.get(84, 40);
        assert!(p[0] > p[2] + 0.1, "sanguine darks are reddish: {p:?}");
    }

    #[test]
    fn ink_is_near_binary_with_lines_and_fills() {
        let src = scene(120, 90);
        let (out, v) = run("filter ink", &[("stroke", "1.2")], &src);
        let l = lum(&out);
        let (ink_l, paper_l) = (color::luma(0.078, 0.078, 0.078), color::luma(0.984, 0.98, 0.96));
        let binary = l.iter().filter(|&&v| (v - ink_l).abs() < 0.12 || (v - paper_l).abs() < 0.12).count();
        assert!(binary as f32 / l.len() as f32 > 0.9, "near-binary: {}", binary as f32 / l.len() as f32);
        // the dark disc is filled, the sky stays paper, the square is outlined
        assert!(mean_rect(&out, 82, 38, 86, 42) < 0.2);
        assert!(mean_rect(&out, 90, 5, 115, 15) > 0.9);
        assert!(v["ink_coverage"].as_f64().unwrap() > 0.02);
        let cov = |params: &[(&str, &str)]| run("filter ink", params, &src).1["ink_coverage"].as_f64().unwrap();
        assert!(cov(&[("fill", "100")]) > cov(&[("fill", "0")]), "fill adds solid ink");
    }

    #[test]
    fn ink_shading_adds_marks_in_the_midtones() {
        let src = Image::from_fn(120, 60, |x, _| {
            let v = 0.25 + 0.7 * x as f32 / 119.0;
            [v, v, v, 1.0]
        });
        let cov = |shading: &str| {
            let (out, _) = run("filter ink", &[("shading", shading), ("fill", "10"), ("stroke", "1.5")], &src);
            1.0 - mean_rect(&out, 20, 10, 70, 50)
        };
        let (none, hatch, stipple) = (cov("none"), cov("hatch"), cov("stipple"));
        assert!(hatch > none + 0.05 && stipple > none + 0.03, "none {none} hatch {hatch} stipple {stipple}");
    }
}
