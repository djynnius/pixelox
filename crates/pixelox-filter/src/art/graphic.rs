//! Graphic looks: `filter cartoon`, `filter cutout`, `filter halftone`.

use super::*;
use crate::util;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("filter cartoon", CAT, "Cartoon: flat color areas with bold outlines", cartoon)
            .about("Cel-shaded cartoon: an anisotropic Kuwahara filter (two passes) flattens the layer into smooth color areas, lightness is quantized into --levels soft-edged bands (Winnemöller et al. 2006) while hue and chroma are kept (slightly boosted), and bold dark outlines are drawn from a difference of Gaussians of the flattened image. Sizes scale with the layer. Alpha is kept.")
            .param(param("levels", ParamKind::Int).default("5").range("2..32").help("Lightness bands per color (fewer = flatter)"))
            .param(param("stroke", ParamKind::Float).auto().default("auto").range("0.2..200").help("Outline width in px; auto = 2.2 × √(w·h)/1000 (recorded)"))
            .param(param("edges", ParamKind::Float).default("70").range("0..100").help("Outline strength (0 = no outlines)"))
            .param(param("detail", ParamKind::Float).default("40").range("0..100").help("Outline detail: 0 = only major shapes, 100 = small features too"))
            .param(param("smooth", ParamKind::Float).auto().default("auto").range("0.5..500").help("Flattening radius in px; auto = 3.5 × √(w·h)/1000 (recorded)"))
            .params(common::region())
            .example("pixelox filter cartoon portrait.jpg -o cartoon.png", "Cartoon with default bands and outlines")
            .example("pixelox filter cartoon photo.jpg -o flat.png --levels 3 --stroke 3", "Flatter shading, heavier lines")
            .example("pixelox filter cartoon photo.jpg -o soft.png --edges 0 --levels 8", "Cel shading without outlines")
            .example("pixelox filter cartoon doc.pxo --layer Photo --detail 70 --in-place", "More outline detail on one layer"),
        OpSpec::doc("filter cutout", CAT, "Cutout: few flat colors, simplified shapes", cutout)
            .about("Photoshop-style Cutout: the layer is simplified with a median filter, reduced to a --levels color palette (seeded k-means in OKLab), and each region's edge is smoothed by a majority vote over a radius set by --simplicity and --fidelity, as if cut from colored paper. Shapes are computed at a reduced resolution and drawn at full resolution with anti-aliased edges. Sizes scale with the layer. Alpha is kept.")
            .param(param("levels", ParamKind::Int).default("6").range("2..16").help("Number of colors"))
            .param(param("simplicity", ParamKind::Float).default("40").range("0..100").help("How much small detail is removed (larger = simpler shapes)"))
            .param(param("fidelity", ParamKind::Float).default("50").range("0..100").help("How closely edges follow the original (larger = more faithful)"))
            .params(common::region())
            .example("pixelox filter cutout photo.jpg -o cutout.png", "Six-color paper cutout")
            .example("pixelox filter cutout logo-photo.jpg -o poster.png --levels 3 --simplicity 70", "Three bold flat shapes")
            .example("pixelox filter cutout portrait.jpg -o detailed.png --levels 10 --simplicity 10 --fidelity 90", "Many colors, faithful edges")
            .example("pixelox filter cutout doc.pxo --mask sky --in-place", "Cutout only inside a mask"),
        OpSpec::doc("filter halftone", CAT, "Halftone dot screen, mono or CMYK", halftone)
            .about("Prints the layer through a halftone screen: each cell of a rotated grid gets a dot whose area matches the average darkness under it (round dots merge into a checker of holes at high coverage, line screens vary their width, square dots tile). mono uses one screen of --ink on --paper at --angle; cmyk separates the image (gray component replacement) into cyan 15°, magenta 75°, yellow 0°, and black 45° screens (all turned by --angle − 45) printed subtractively. The screen is aligned to the canvas and anti-aliased. Alpha is kept.")
            .param(param("cell", ParamKind::Float).auto().default("auto").range("2..1000").help("Screen cell size in px; auto = 6 × √(w·h)/1000 (recorded)"))
            .param(param("mode", ParamKind::Choice).choices(&["mono", "cmyk"]).default("mono").help("mono (one ink) or cmyk (four rotated screens)"))
            .param(param("shape", ParamKind::Choice).choices(&["round", "line", "square"]).default("round").help("Dot shape: round, line (line screen), or square"))
            .param(param("angle", ParamKind::Angle).default("45").help("Screen angle in degrees counter-clockwise from the right (cmyk: the black screen's angle)"))
            .param(param("ink", ParamKind::Color).default("#111111").help("Ink color for mono"))
            .param(param("paper", ParamKind::Color).default("#ffffff").help("Paper color"))
            .params(common::region())
            .example("pixelox filter halftone photo.jpg -o dots.png", "Black dot screen")
            .example("pixelox filter halftone photo.jpg -o comic.png --mode cmyk --cell 12", "CMYK comic-print look")
            .example("pixelox filter halftone photo.jpg -o lines.png --shape line --angle 0 --cell 8", "Horizontal line screen")
            .example("pixelox filter halftone doc.pxo --ink '#1b3a8a' --paper '#f4ecd8' --in-place", "Blue ink on cream paper"),
    ]
}

// ---------------------------------------------------------------------------------------
// cartoon

pub(crate) struct Cartoon {
    pub levels: usize,
    pub stroke: f64,
    pub edges: f32,
    pub detail: f32,
    pub smooth: f64,
}

fn cartoon(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let mut p = Cartoon {
        levels: a.usize("levels")?.max(2),
        stroke: 0.0,
        edges: pct(a, "edges")?,
        detail: pct(a, "detail")?,
        smooth: 0.0,
    };
    let v = apply_to_raster(a, doc, ctx, |img, ctx| {
        if img.is_empty() {
            return Ok(img.clone());
        }
        let u = unit(img.width as usize, img.height as usize);
        p.stroke = size_param(a, ctx, "stroke", 2.2 * u)?;
        p.smooth = size_param(a, ctx, "smooth", 3.5 * u)?;
        Ok(with_alpha(img, &cartoon_render(&solid_rgb(img), &p)))
    })?;
    Ok(if p.stroke > 0.0 { util::merge(v, json!({"stroke": p.stroke, "smooth": p.smooth})) } else { v })
}

pub(crate) fn cartoon_render(rgb: &Buf, p: &Cartoon) -> Buf {
    let (w, h) = (rgb.w, rgb.h);
    let (flat, f) = abstraction(rgb, p.smooth, 10.0, 2);
    let flat = if f > 1 { util::upsample(&flat, w, h, f) } else { flat };
    let mut lab = to_oklab(&flat);
    // lightness bands between the 1st and 99th percentiles
    let lp = Buf { w, h, c: 1, data: lab.data.par_chunks(3).map(|p| p[0]).collect() };
    let (lo, hi) = (quantile(&lp, 0.01), quantile(&lp, 0.99));
    let span = (hi - lo).max(0.05);
    let n = p.levels as f32;
    let dq = 1.0 / n;
    let phi = 20.0 * n;
    lab.data.par_chunks_mut(3).for_each(|px| {
        let t = ((px[0] - lo) / span).clamp(0.0, 1.0);
        let near = ((t * n).floor().min(n - 1.0) + 0.5) * dq;
        let qv = near + 0.5 * dq * (phi * (t - near)).tanh();
        px[0] = lo + qv * span;
        px[1] *= 1.15;
        px[2] *= 1.15;
    });
    let col = from_oklab(&lab);
    if p.edges <= 0.0 {
        return col;
    }
    // outlines: dark side of a difference of Gaussians of the flattened lightness
    let sigma = (0.6 * p.stroke).max(0.4);
    let g1 = gauss(&lp, sigma);
    let g2 = gauss(&lp, 2.0 * sigma);
    let t = 0.004 + 0.016 * (1.0 - p.detail);
    let mut out = col;
    out.data.par_chunks_mut(3).enumerate().for_each(|(i, o)| {
        let d = g2.data[i] - g1.data[i];
        let line = util::smoothstep(t, 2.5 * t, d) * p.edges.sqrt();
        for ch in 0..3 {
            let dark = o[ch] * 0.12;
            o[ch] += (dark - o[ch]) * line;
        }
    });
    out
}

// ---------------------------------------------------------------------------------------
// cutout

pub(crate) struct Cutout {
    pub levels: usize,
    pub simplicity: f32,
    pub fidelity: f32,
}

fn cutout(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let p =
        Cutout { levels: a.usize("levels")?.max(2), simplicity: pct(a, "simplicity")?, fidelity: pct(a, "fidelity")? };
    let mut palette = Vec::new();
    let v = apply_to_raster(a, doc, ctx, |img, ctx| {
        if img.is_empty() {
            return Ok(img.clone());
        }
        let tex = Tex::new(ctx, 0, 0);
        let (out, pal) = cutout_render(&solid_rgb(img), &p, tex);
        palette = pal;
        Ok(with_alpha(img, &out))
    })?;
    let hex: Vec<String> = palette.iter().map(|c| Rgba::new(c[0], c[1], c[2], 1.0).to_hex()).collect();
    Ok(util::merge(v, json!({"palette": hex})))
}

/// CIELab scaled to about 0..1 (L*/100, a*/100, b*/100): lightness is linear near black,
/// so shadow noise does not look like distinct colors.
pub(crate) fn to_cielab(rgb: &Buf) -> Buf {
    let mut out = rgb.clone();
    out.data.par_chunks_mut(3).for_each(|p| {
        let (l, a, b) = color::rgb_to_lab(p[0].clamp(0.0, 1.0), p[1].clamp(0.0, 1.0), p[2].clamp(0.0, 1.0));
        p[0] = l / 100.0;
        p[1] = a / 100.0;
        p[2] = b / 100.0;
    });
    out
}

/// Seeded k-means++ palette of `lab` (from [`to_cielab`]). Colors are binned on a 32³ grid
/// first and each occupied bin weighs √count, so large flat areas (a black backdrop) do not
/// take several near-identical entries; near-duplicate centers are re-seeded at the worst
/// represented color. Returns the centers sorted darkest first.
pub(crate) fn kmeans(lab: &Buf, k: usize, tex: Tex) -> Vec<[f32; 3]> {
    const G: usize = 32;
    let bin = |p: &[f32]| {
        let l = ((p[0].clamp(0.0, 1.0) * G as f32) as usize).min(G - 1);
        let a = (((p[1] + 0.8) / 1.6).clamp(0.0, 1.0) * G as f32) as usize;
        let b = (((p[2] + 0.8) / 1.6).clamp(0.0, 1.0) * G as f32) as usize;
        (l * G + a.min(G - 1)) * G + b.min(G - 1)
    };
    // per-bin sums, accumulated per chunk and merged in chunk order
    let parts: Vec<Vec<[f64; 4]>> = lab
        .data
        .par_chunks(3 * 65_536)
        .map(|ch| {
            let mut acc = vec![[0f64; 4]; G * G * G];
            for p in ch.chunks(3) {
                let e = &mut acc[bin(p)];
                e[0] += p[0] as f64;
                e[1] += p[1] as f64;
                e[2] += p[2] as f64;
                e[3] += 1.0;
            }
            acc
        })
        .collect();
    let mut bins = vec![[0f64; 4]; G * G * G];
    for part in parts {
        for (b, p) in bins.iter_mut().zip(part) {
            for c in 0..4 {
                b[c] += p[c];
            }
        }
    }
    let samples: Vec<([f32; 3], f64)> = bins
        .iter()
        .filter(|b| b[3] > 0.0)
        .map(|b| ([(b[0] / b[3]) as f32, (b[1] / b[3]) as f32, (b[2] / b[3]) as f32], b[3].sqrt()))
        .collect();
    if samples.is_empty() {
        return vec![[0.5, 0.0, 0.0]];
    }
    let d2 = |a: &[f32; 3], b: &[f32; 3]| {
        let (x, y, z) = (a[0] - b[0], a[1] - b[1], a[2] - b[2]);
        x * x + y * y + z * z
    };
    let k = k.min(samples.len()).max(1);
    // first center: the heaviest bin
    let first = samples.iter().enumerate().fold(0, |bi, (i, s)| if s.1 > samples[bi].1 { i } else { bi });
    let mut centers = vec![samples[first].0];
    let mut best: Vec<f32> = samples.iter().map(|s| d2(&s.0, &centers[0])).collect();
    for j in 1..k {
        let total: f64 = best.iter().zip(&samples).map(|(&v, s)| v as f64 * s.1).sum();
        if total <= 0.0 {
            break;
        }
        let r = unit_f(hash2(tex.key, j as i64, 7)) as f64 * total;
        let mut acc = 0.0;
        let mut pick = samples.len() - 1;
        for (i, (&v, s)) in best.iter().zip(&samples).enumerate() {
            acc += v as f64 * s.1;
            if acc >= r {
                pick = i;
                break;
            }
        }
        let c = samples[pick].0;
        centers.push(c);
        for (b, s) in best.iter_mut().zip(&samples) {
            *b = b.min(d2(&s.0, &c));
        }
    }
    let lloyd = |centers: &mut Vec<[f32; 3]>| {
        for _ in 0..20 {
            let mut sums = vec![[0f64; 4]; centers.len()];
            for (s, wt) in &samples {
                let j = nearest(centers, s);
                for c in 0..3 {
                    sums[j][c] += s[c] as f64 * wt;
                }
                sums[j][3] += wt;
            }
            for (c, s) in centers.iter_mut().zip(&sums) {
                if s[3] > 0.0 {
                    *c = [(s[0] / s[3]) as f32, (s[1] / s[3]) as f32, (s[2] / s[3]) as f32];
                }
            }
        }
    };
    lloyd(&mut centers);
    // near-duplicate centers: move one to the worst-represented color and refine
    for _ in 0..k {
        let mut pair = None;
        let mut close = 0.06f32 * 0.06;
        for i in 0..centers.len() {
            for j in i + 1..centers.len() {
                let d = d2(&centers[i], &centers[j]);
                if d < close {
                    close = d;
                    pair = Some(j);
                }
            }
        }
        let Some(j) = pair else { break };
        let worst = samples
            .iter()
            .map(|(s, wt)| d2(s, &centers[nearest(&centers, s)]) as f64 * wt)
            .enumerate()
            .fold((0, -1.0), |b, (i, v)| if v > b.1 { (i, v) } else { b });
        if worst.1 <= 0.0 {
            break;
        }
        centers[j] = samples[worst.0].0;
        lloyd(&mut centers);
    }
    centers.sort_by(|a, b| a[0].total_cmp(&b[0]));
    centers
}

#[inline]
fn nearest(centers: &[[f32; 3]], s: &[f32; 3]) -> usize {
    let mut bi = 0;
    let mut bd = f32::MAX;
    for (j, c) in centers.iter().enumerate() {
        let (x, y, z) = (s[0] - c[0], s[1] - c[1], s[2] - c[2]);
        let d = x * x + y * y + z * z;
        if d < bd {
            bd = d;
            bi = j;
        }
    }
    bi
}

pub(crate) fn cutout_render(rgb: &Buf, p: &Cutout, tex: Tex) -> (Buf, Vec<[f32; 3]>) {
    let (w, h) = (rgb.w, rgb.h);
    let u = unit(w, h);
    // working copy of at most about 1.5 MP
    let f = ((w as f64 * h as f64 / 1.5e6).sqrt().ceil() as usize).max(1);
    let small = if f > 1 { util::downsample(rgb, f) } else { rgb.clone() };
    let (sw, sh) = (small.w, small.h);
    let uf = u / f as f64;
    // simplify: median
    let rm = ((0.5 + 7.0 * p.simplicity as f64) * uf).round() as usize;
    let mut img = Image::new(sw as u32, sh as u32);
    img.data.par_chunks_mut(4).zip(small.data.par_chunks(3)).for_each(|(o, s)| {
        o[..3].copy_from_slice(s);
        o[3] = 1.0;
    });
    let med = if rm > 0 { kern::median_image(&img, rm) } else { img };
    let mut mrgb = Buf::new(sw, sh, 3);
    mrgb.data.par_chunks_mut(3).zip(med.data.par_chunks(4)).for_each(|(o, s)| o.copy_from_slice(&s[..3]));
    let lab = to_cielab(&mrgb);
    let k = p.levels;
    let centers = kmeans(&lab, k, tex);
    let k = centers.len();
    // one-hot label maps, smoothed: a soft majority vote that rounds the shapes
    let labels: Vec<u8> = lab.data.par_chunks(3).map(|s| nearest(&centers, &[s[0], s[1], s[2]]) as u8).collect();
    // palette: mean color of each label's pixels (rows summed in order)
    let parts: Vec<Vec<[f64; 4]>> = mrgb
        .data
        .par_chunks(3 * sw)
        .zip(labels.par_chunks(sw))
        .map(|(row, lr)| {
            let mut acc = vec![[0f64; 4]; k];
            for (px, &l) in row.chunks(3).zip(lr) {
                let e = &mut acc[l as usize];
                for c in 0..3 {
                    e[c] += px[c] as f64;
                }
                e[3] += 1.0;
            }
            acc
        })
        .collect();
    let mut sums = vec![[0f64; 4]; k];
    for part in parts {
        for (s, q) in sums.iter_mut().zip(part) {
            for c in 0..4 {
                s[c] += q[c];
            }
        }
    }
    let mut maps = Buf::new(sw, sh, k);
    maps.data.par_chunks_mut(k).zip(labels.par_iter()).for_each(|(m, &l)| m[l as usize] = 1.0);
    let sigma = ((0.4 + 4.0 * (1.0 - p.fidelity as f64) * (0.3 + p.simplicity as f64)) * uf).max(0.5);
    gauss_in(&mut maps, sigma);
    let pal: Vec<[f32; 3]> = sums
        .iter()
        .zip(&centers)
        .map(|(s, c)| {
            if s[3] > 0.0 {
                [(s[0] / s[3]) as f32, (s[1] / s[3]) as f32, (s[2] / s[3]) as f32]
            } else {
                // a center no pixel chose (rare): gray of its lightness
                [c[0]; 3]
            }
        })
        .collect();
    // draw at full resolution: the winning label, blended with the runner-up over about
    // one pixel where the vote is close
    let ff = f as f32;
    // (v1 − v2) changes by about 0.8/σ per working px across a boundary: blend over ~1 px
    let aa = (0.5 * 0.8 / (sigma as f32 * ff)).min(1.0);
    let mut out = Buf::new(w, h, 3);
    out.data.par_chunks_mut(w * 3).enumerate().for_each(|(y, row)| {
        let mut v = vec![0f32; k];
        let sy = (y as f32 + 0.5) / ff - 0.5;
        for x in 0..w {
            let sx = (x as f32 + 0.5) / ff - 0.5;
            util::sample(&maps, sx, sy, Edge::Clamp, &mut v);
            let (mut b1, mut b2) = (0usize, usize::MAX);
            for j in 1..k {
                if v[j] > v[b1] {
                    b2 = b1;
                    b1 = j;
                } else if b2 == usize::MAX || v[j] > v[b2] {
                    b2 = j;
                }
            }
            let o = &mut row[x * 3..x * 3 + 3];
            o.copy_from_slice(&pal[b1]);
            if b2 != usize::MAX {
                let t = 0.5 * (1.0 - ((v[b1] - v[b2]) / aa).min(1.0));
                if t > 0.0 {
                    for c in 0..3 {
                        o[c] += (pal[b2][c] - o[c]) * t;
                    }
                }
            }
        }
    });
    (out, pal)
}

// ---------------------------------------------------------------------------------------
// halftone

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Shape {
    Round,
    Line,
    Square,
}

pub(crate) struct Halftone {
    pub cell: f64,
    pub cmyk: bool,
    pub shape: Shape,
    pub angle: f64,
    pub ink: [f32; 3],
    pub paper: [f32; 3],
}

fn halftone(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (lx, ly) = util::layer_offset(a, doc)?;
    let (ink, paper) = (a.color("ink")?, a.color("paper")?);
    let mut p = Halftone {
        cell: 0.0,
        cmyk: a.choice("mode") == "cmyk",
        shape: match a.choice("shape").as_str() {
            "line" => Shape::Line,
            "square" => Shape::Square,
            _ => Shape::Round,
        },
        angle: a.angle("angle")?,
        ink: [ink.r, ink.g, ink.b],
        paper: [paper.r, paper.g, paper.b],
    };
    let v = apply_to_raster(a, doc, ctx, |img, ctx| {
        if img.is_empty() {
            return Ok(img.clone());
        }
        p.cell = size_param(a, ctx, "cell", 6.0 * unit(img.width as usize, img.height as usize))?.max(2.0);
        Ok(with_alpha(img, &halftone_render(&solid_rgb(img), &p, lx as f64, ly as f64)))
    })?;
    Ok(if p.cell > 0.0 { util::merge(v, json!({"cell": p.cell})) } else { v })
}

/// Radius (in cell units) of a round dot whose clipped area covers `a` of its cell.
fn round_radius_lut() -> Vec<f32> {
    let area = |r: f64| {
        if r <= 0.5 {
            std::f64::consts::PI * r * r
        } else {
            let seg = r * r * (0.5 / r).acos() - 0.5 * (r * r - 0.25).max(0.0).sqrt();
            std::f64::consts::PI * r * r - 4.0 * seg
        }
    };
    let rmax = std::f64::consts::FRAC_1_SQRT_2;
    let n = 256;
    let mut lut = vec![0f32; n + 1];
    let mut r = 0.0;
    for (i, v) in lut.iter_mut().enumerate() {
        let target = i as f64 / n as f64;
        while r < rmax && area(r) < target {
            r += 1e-4;
        }
        *v = r.min(rmax) as f32;
    }
    lut[n] = (rmax + 0.02) as f32;
    lut
}

/// One screen: coverage 0..1 per pixel of `tone` (0..1 darkness, already cell-averaged)
/// at `angle` degrees, cell `cell` px, layer offset (ox, oy).
fn screen(tone: &Buf, cell: f64, angle: f64, shape: Shape, ox: f64, oy: f64, lut: &[f32]) -> Buf {
    let (w, h) = (tone.w, tone.h);
    let th = angle.to_radians();
    let (c, s) = (th.cos(), -th.sin());
    let mut out = Buf::new(w, h, 1);
    let cf = cell;
    let n = (lut.len() - 1) as f32;
    out.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let mut t = [0f32];
        for x in 0..w {
            let (px, py) = (x as f64 + 0.5 + ox, y as f64 + 0.5 + oy);
            // screen coordinates (cells)
            let su = (px * c + py * s) / cf;
            let sv = (-px * s + py * c) / cf;
            let (iu, iv) = (su.floor(), sv.floor());
            let mut best = 0f32;
            match shape {
                Shape::Line => {
                    for dv in -1..=1 {
                        let cv = iv + dv as f64 + 0.5;
                        // tone at the nearest point of this line
                        let (cx, cy) = ((su * c - cv * s) * cf - ox - 0.5, (su * s + cv * c) * cf - oy - 0.5);
                        crate::util::sample(tone, cx as f32, cy as f32, Edge::Clamp, &mut t);
                        let half = t[0].clamp(0.0, 1.0) * 0.5 * cf as f32;
                        let dist = ((sv - cv).abs() * cf) as f32;
                        best = best.max((half - dist + 0.5).clamp(0.0, 1.0));
                    }
                }
                _ => {
                    for dv in -1..=1 {
                        for du in -1..=1 {
                            let (cu, cv) = (iu + du as f64 + 0.5, iv + dv as f64 + 0.5);
                            let (cx, cy) = ((cu * c - cv * s) * cf - ox - 0.5, (cu * s + cv * c) * cf - oy - 0.5);
                            crate::util::sample(tone, cx as f32, cy as f32, Edge::Clamp, &mut t);
                            let a = t[0].clamp(0.0, 1.0);
                            if a <= 0.0 {
                                continue;
                            }
                            let (du_px, dv_px) = (((su - cu) * cf) as f32, ((sv - cv) * cf) as f32);
                            let cov = if shape == Shape::Round {
                                let fi = a * n;
                                let i0 = (fi as usize).min(lut.len() - 2);
                                let r = (lut[i0] + (lut[i0 + 1] - lut[i0]) * (fi - i0 as f32)) * cf as f32;
                                let dist = (du_px * du_px + dv_px * dv_px).sqrt();
                                r - dist + 0.5
                            } else {
                                let half = a.sqrt() * 0.5 * cf as f32;
                                half - du_px.abs().max(dv_px.abs()) + 0.5
                            };
                            best = best.max(cov.clamp(0.0, 1.0));
                        }
                    }
                }
            }
            row[x] = best;
        }
    });
    out
}

pub(crate) fn halftone_render(rgb: &Buf, p: &Halftone, ox: f64, oy: f64) -> Buf {
    let (w, h) = (rgb.w, rgb.h);
    let lut = round_radius_lut();
    let sigma = 0.3 * p.cell;
    let mut out = Buf::new(w, h, 3);
    if !p.cmyk {
        let mut tone = luma(rgb);
        tone.data.par_iter_mut().for_each(|v| *v = 1.0 - v.clamp(0.0, 1.0));
        gauss_in(&mut tone, sigma);
        let cov = screen(&tone, p.cell, p.angle, p.shape, ox, oy, &lut);
        out.data.par_chunks_mut(3).zip(cov.data.par_iter()).for_each(|(o, &c)| {
            for ch in 0..3 {
                o[ch] = p.paper[ch] + (p.ink[ch] - p.paper[ch]) * c;
            }
        });
        return out;
    }
    // CMYK separation with gray component replacement
    let mut sep = Buf::new(w, h, 4);
    sep.data.par_chunks_mut(4).zip(rgb.data.par_chunks(3)).for_each(|(o, c)| {
        let (r, g, b) = (c[0].clamp(0.0, 1.0), c[1].clamp(0.0, 1.0), c[2].clamp(0.0, 1.0));
        let k = 1.0 - r.max(g).max(b);
        let inv = if k < 0.999 { 1.0 / (1.0 - k) } else { 0.0 };
        o[0] = (1.0 - r - k) * inv;
        o[1] = (1.0 - g - k) * inv;
        o[2] = (1.0 - b - k) * inv;
        o[3] = k;
    });
    gauss_in(&mut sep, sigma);
    const INKS: [[f32; 3]; 4] = [[0.0, 0.62, 0.89], [0.90, 0.0, 0.50], [1.0, 0.93, 0.0], [0.12, 0.11, 0.11]];
    const ANGLES: [f64; 4] = [15.0, 75.0, 0.0, 45.0];
    out.data.par_chunks_mut(3).for_each(|o| o.copy_from_slice(&p.paper));
    for ch in 0..4 {
        let plane = Buf { w, h, c: 1, data: sep.data.par_chunks(4).map(|v| v[ch]).collect() };
        let cov = screen(&plane, p.cell, ANGLES[ch] + p.angle - 45.0, p.shape, ox, oy, &lut);
        out.data.par_chunks_mut(3).zip(cov.data.par_iter()).for_each(|(o, &c)| {
            for k in 0..3 {
                o[k] *= 1.0 - c * (1.0 - INKS[ch][k]);
            }
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::super::fixture::*;
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn cartoon_has_few_bands_and_dark_outlines() {
        let ramp = Image::from_fn(160, 40, |x, _| {
            let v = 0.1 + 0.8 * x as f32 / 159.0;
            [v, v * 0.8, v * 0.6, 1.0]
        });
        let (out, _) = run("filter cartoon", &[("levels", "4"), ("edges", "0")], &ramp);
        // lightness collects in a few bands
        let mut hist = [0usize; 50];
        for v in lum(&out) {
            hist[((v * 50.0) as usize).min(49)] += 1;
        }
        let mut counts = hist.to_vec();
        counts.sort_unstable_by(|a, b| b.cmp(a));
        let top: usize = counts[..8].iter().sum();
        assert!(top as f32 > 0.8 * (160 * 40) as f32, "banded: {hist:?}");
        // a step gets a dark outline on its dark side
        let step = Image::from_fn(80, 40, |x, _| if x < 40 { [0.55, 0.5, 0.45, 1.0] } else { [0.9, 0.9, 0.85, 1.0] });
        let (out, _) = run("filter cartoon", &[("stroke", "2")], &step);
        let line = (30..40).map(|x| mean_rect(&out, x, 10, x + 1, 30)).fold(1.0f32, f32::min);
        assert!(line < 0.25 && mean_rect(&out, 5, 10, 20, 30) > 0.4, "outline {line}");
    }

    #[test]
    fn cutout_uses_few_flat_colors() {
        let src = scene(120, 90);
        let (out, v) = run("filter cutout", &[("levels", "4"), ("simplicity", "60")], &src);
        let pal = v["palette"].as_array().unwrap();
        assert_eq!(pal.len(), 4);
        let key = |p: &[f32]| {
            ((p[0] * 255.0).round() as u32) << 16 | ((p[1] * 255.0).round() as u32) << 8 | (p[2] * 255.0).round() as u32
        };
        let mut counts = std::collections::BTreeMap::new();
        for p in out.data.chunks(4) {
            *counts.entry(key(p)).or_insert(0usize) += 1;
        }
        let mut c: Vec<usize> = counts.values().copied().collect();
        c.sort_unstable_by(|a, b| b.cmp(a));
        let top: usize = c.iter().take(4).sum();
        assert!(top as f32 > 0.93 * (120 * 90) as f32, "flat colors (edges anti-aliased): {} of {}", top, 120 * 90);
    }

    #[test]
    fn cutout_simplicity_removes_specks() {
        let src = Image::from_fn(120, 90, |x, y| {
            let speck = x % 20 == 10 && y % 20 == 10;
            let v = if speck {
                0.0
            } else if x < 60 {
                0.8
            } else {
                0.3
            };
            [v, v, v, 1.0]
        });
        let (out, _) = run("filter cutout", &[("levels", "3"), ("simplicity", "80")], &src);
        assert!(lum(&out)[10 * 120 + 30] > 0.5, "speck removed");
        let distinct: BTreeSet<u32> = out.data.chunks(4).map(|p| (p[0] * 255.0) as u32).collect();
        assert!(distinct.len() <= 6, "{distinct:?}");
    }

    #[test]
    fn halftone_dots_repeat_at_the_cell_size() {
        let gray = Image::filled(96, 96, Rgba::new(0.5, 0.5, 0.5, 1.0));
        let (out, v) = run("filter halftone", &[("cell", "8"), ("angle", "0"), ("shape", "square")], &gray);
        assert_eq!(v["cell"], json!(8.0));
        let l = lum(&out);
        let ink: f32 = l.iter().map(|&v| 1.0 - v).sum::<f32>() / l.len() as f32;
        assert!(
            (ink - 0.5 * (1.0 - color::luma(0.067, 0.067, 0.067))).abs() < 0.05,
            "coverage matches the tone: {ink}"
        );
        for y in 8..80 {
            for x in 8..80 {
                assert!((l[y * 96 + x] - l[y * 96 + x + 8]).abs() < 1e-4, "period 8 at {x},{y}");
                assert!((l[y * 96 + x] - l[(y + 8) * 96 + x]).abs() < 1e-4);
            }
        }
        // round dots: cell centers inked, corners paper (at 30% darkness)
        let light = Image::filled(96, 96, Rgba::new(0.7, 0.7, 0.7, 1.0));
        let (out, _) = run("filter halftone", &[("cell", "12"), ("angle", "0")], &light);
        let l = lum(&out);
        assert!(l[42 * 96 + 42] < 0.2 && l[36 * 96 + 36] > 0.95, "{} {}", l[42 * 96 + 42], l[36 * 96 + 36]);
        // line screen at 0°: rows are uniform along x
        let (out, _) = run("filter halftone", &[("cell", "8"), ("angle", "0"), ("shape", "line")], &gray);
        let l = lum(&out);
        assert!((0..96).all(|x| (l[20 * 96 + x] - l[20 * 96]).abs() < 1e-4));
    }

    #[test]
    fn halftone_cmyk_separates_colors() {
        let red = Image::filled(48, 48, Rgba::new(1.0, 0.0, 0.0, 1.0));
        let (out, _) = run("filter halftone", &[("mode", "cmyk"), ("cell", "6")], &red);
        let p = out.get(24, 24);
        assert!(p[0] > 0.8 && p[1] < 0.1 && p[2] < 0.1, "magenta + yellow print red: {p:?}");
        let white = Image::filled(16, 16, Rgba::WHITE);
        let (out, _) = run("filter halftone", &[("mode", "cmyk")], &white);
        assert!(out.data.iter().all(|&v| v == 1.0), "no ink on white");
    }
}
