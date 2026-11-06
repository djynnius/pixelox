//! `retouch blemishes`: find small dark or light spots (pimples, marks, whiteheads) at
//! pixel accuracy, and optionally heal them with `retouch heal`'s machinery.
//!
//! Detection, O(pixels) per scale over a bounded number of scales:
//! 1. signal: for dark spots −luma + ½·(r − g) (blemishes are darker and usually redder
//!    than the skin around them), for light spots luma; weighted by the search mask times
//!    the layer's alpha;
//! 2. scale space in octaves (block means at the power of two that brings σ to 1..2 px):
//!    masked (normalized) Gaussian blurs L_k at σ_k = σ_lo·2^((k−1)/3), so nothing outside
//!    the mask leaks into the skin; band-pass D_k = L_k − L_{k+2} (a scale-normalized
//!    difference of Gaussians of ratio 1.59);
//! 3. texture level: per scale, the mean |D_k| over the search area in a window of 8σ
//!    with the central 3σ left out (purely local, and a spot does not raise its own
//!    threshold), so a spot must stand out from the pores around it by a
//!    sensitivity-dependent factor; plus an absolute contrast floor;
//! 4. candidates: strict local maxima over space and scale, round (Hessian eigenvalue
//!    ratio ≤ 5), refined to sub-pixel position and scale;
//! 5. measurement at full resolution: re-centered on the spot, contrast = center mean −
//!    median of a surrounding ring (skin only), radius where the contrast falls to a
//!    quarter, size limits, and the spot (plus a pixel) must lie inside the mask;
//!    overlapping detections keep the strongest. Heal discs are limited to the mask.

use crate::kern;
use crate::retouch::heal_region;
use crate::util::Buf;
use pixelox_core::kernels::Edge;
use pixelox_core::prelude::*;
use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::{BTreeMap, VecDeque};

const CAT: Category = Category::Retouch;
/// Weight of redness (r − g) in the dark-spot signal.
const K_RED: f32 = 0.5;
/// Scale step 2^(1/3).
const STEP: f64 = 1.259_921_049_894_873_2;
/// Most scales searched (wide size ranges get a coarser step).
const MAX_SCALES: usize = 24;
/// Most elongated spot kept: ratio of the Hessian's eigenvalues.
const EDGE_RATIO: f32 = 5.0;
/// Detection threshold at sensitivity 50, in units of the local texture spread.
const TAU_50: f32 = 6.0;
/// Smallest contrast at sensitivity 50, in 8-bit levels of the detection signal.
const MIN_CONTRAST_50: f32 = 4.0;

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("retouch blemishes", CAT, "Find (and heal) blemishes: small dark or light spots", blemishes)
        .target_optional()
        .about("Finds blemishes (pimples, small dark or red marks, and with --kind light whiteheads or dust) at pixel accuracy and lists them strongest first; --heal repairs them all in one pass with the `retouch heal` machinery, --save-mask stores them as a mask (review it, or heal later with `retouch heal --mask`). Run it with --mask on a skin mask: without one the whole layer is searched (the result says so), and eyes, nostrils, lips, and hair can be reported. Detection works on luminance plus redness for dark spots: a masked scale-space band-pass (differences of Gaussians from --min-size to --max-size) whose peaks must exceed the local texture level (the band-passed skin's mean magnitude around the spot, the spot itself left out, so pores stay below it) by a factor set by --sensitivity, be round, and lie inside the mask with a pixel to spare (spots straddling its edge are skipped). Each spot's x,y (canvas px, as `retouch heal --at` takes them), radius (px, out to where its contrast falls to a quarter; --min-size and --max-size limit twice this), contrast (8-bit levels of the detection signal: luminance drop plus half the redness rise for dark spots, luminance rise for light ones), and redness (rise of red over green, levels: pimples are red, moles and freckles are not) are reported. Moles, freckles, and beauty marks are dark spots too, and specular highlights are light spots: check the list (or the saved mask) before healing everything, and heal selected spots with `retouch heal --at X,Y --radius R`. --heal heals a disc of 1.25 × radius + 1 px per spot, limited to the mask so nothing outside it changes. Run it before `retouch frequency-split`, which copies the pixels. Without --heal and --save-mask it only reports and needs no -o; with them it needs -o or --in-place (PX1004). The layer may be any kind for reporting; --heal needs a pixel layer.")
        .param(common::layer())
        .param(param("mask", ParamKind::Mask).help("Where to search, normally a skin mask: a named mask, @sel, @alpha:<layer>, @mask:<layer>, or a grayscale image path (default: the whole layer)"))
        .param(param("invert-mask", ParamKind::Flag).help("Search outside the mask instead of inside it"))
        .param(
            param("min-size", ParamKind::Length)
                .default("auto")
                .auto()
                .help("Smallest spot diameter in px, %, mm, in, or pt (% of the long side; auto = 0.2% of it, at least 3 px)"),
        )
        .param(
            param("max-size", ParamKind::Length)
                .default("auto")
                .auto()
                .help("Largest spot diameter (auto = 2% of the long side, at least 12 px and twice --min-size)"),
        )
        .param(
            param("sensitivity", ParamKind::Float)
                .default("50")
                .range("0..100")
                .help("0..100: higher finds fainter spots (each +25 halves the contrast a spot needs over the skin's texture)"),
        )
        .param(
            param("kind", ParamKind::Choice)
                .choices(&["dark", "light", "both"])
                .default("dark")
                .help("Spots darker than the skin around them (pimples, marks; redness counts), lighter, or both"),
        )
        .param(param("max", ParamKind::Int).default("300").range("1..100000").help("Most spots to report and heal, strongest first"))
        .param(
            param("min-redness", ParamKind::Float)
                .range("-255..255")
                .help("Keep only spots at least this much redder than the skin around them (levels): about 10 keeps pimples and drops moles and freckles"),
        )
        .param(param("heal", ParamKind::Flag).help("Heal every spot found, in one pass; needs -o or --in-place"))
        .param(param("save-mask", ParamKind::Str).help("Store the spots (the discs --heal uses) as a named mask; needs -o or --in-place"))
        .example("pixelox retouch blemishes portrait.pxo --mask skin --json", "List the spots on the skin without changing anything")
        .example("pixelox retouch blemishes portrait.pxo --mask skin --heal --in-place", "Find and heal them in the document")
        .example("pixelox retouch blemishes photo.jpg -o clean.png --mask skin.png --heal --sensitivity 70", "Heal fainter spots too")
        .example("pixelox retouch blemishes portrait.pxo --mask skin --kind both --max-size 1% --save-mask spots --in-place", "Store dark and light spots as a mask for review")
        .example("pixelox retouch blemishes portrait.pxo --mask skin --min-redness 10 --heal --in-place", "Heal red pimples, keep moles and freckles")]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Dark,
    Light,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Dark => "dark",
            Kind::Light => "light",
        }
    }
}

struct Settings {
    /// Spot diameters in px.
    min_d: f64,
    max_d: f64,
    sensitivity: f64,
    /// Peak threshold in units of the local texture spread.
    tau: f32,
    /// Smallest contrast, in signal units (0..1).
    min_contrast: f32,
    kind: String,
    kinds: Vec<Kind>,
    max: usize,
}

fn settings(a: &Args, doc: &Document, ctx: &mut OpCtx) -> Result<Settings> {
    let long = doc.width.max(doc.height) as f64;
    let dpi = doc.dpi();
    let min_d = match a.length("min-size")? {
        Some(l) => l.to_px(long, dpi),
        None => {
            let v = (long * 2.0 / 1000.0).max(3.0);
            ctx.resolve_param("min_size", v.to_string());
            v
        }
    };
    if !(min_d.is_finite() && min_d >= 1.0) {
        return Err(Error::new("PX1013", format!("--min-size must be at least 1 px (got {min_d} px)"))
            .with_context("param", "min-size"));
    }
    let max_d = match a.length("max-size")? {
        Some(l) => l.to_px(long, dpi),
        None => {
            let v = (long * 2.0 / 100.0).max(12.0).max(2.0 * min_d);
            ctx.resolve_param("max_size", v.to_string());
            v
        }
    };
    if !(max_d.is_finite() && max_d > min_d) {
        return Err(Error::new(
            "PX1010",
            format!("--max-size ({max_d} px) must be larger than --min-size ({min_d} px)"),
        )
        .with_hint("pixelox retouch blemishes doc.pxo --mask skin --min-size 4 --max-size 40")
        .with_context("min_size", min_d)
        .with_context("max_size", max_d));
    }
    let sensitivity = a.f64("sensitivity")?;
    let k = (-(sensitivity as f32 - 50.0) / 25.0).exp2();
    let kind = a.choice("kind");
    let kinds = match kind.as_str() {
        "light" => vec![Kind::Light],
        "both" => vec![Kind::Dark, Kind::Light],
        _ => vec![Kind::Dark],
    };
    Ok(Settings {
        min_d,
        max_d,
        sensitivity,
        tau: TAU_50 * k,
        min_contrast: MIN_CONTRAST_50 / 255.0 * k,
        kind,
        kinds,
        max: a.usize("max")?,
    })
}

/// Full-resolution planes of the layer being searched.
struct Field {
    w: usize,
    h: usize,
    luma: Vec<f32>,
    red: Vec<f32>,
    /// Search weight: mask × alpha, 0 off the canvas.
    wt: Vec<f32>,
}

impl Field {
    fn new(img: &Image, ox: i32, oy: i32, cw: u32, ch: u32, mask: Option<&Mask>) -> Field {
        let (w, h) = (img.width as usize, img.height as usize);
        let mut luma = vec![0f32; w * h];
        let mut red = vec![0f32; w * h];
        let mut wt = vec![0f32; w * h];
        let rows = luma.par_chunks_mut(w.max(1)).zip(red.par_chunks_mut(w.max(1))).zip(wt.par_chunks_mut(w.max(1)));
        rows.enumerate().for_each(|(y, ((lr, rr), wr))| {
            for x in 0..w {
                let p = &img.data[(y * w + x) * 4..(y * w + x) * 4 + 4];
                lr[x] = color::luma(p[0], p[1], p[2]);
                rr[x] = p[0] - p[1];
                let (cx, cy) = (x as i64 + ox as i64, y as i64 + oy as i64);
                let m = match mask {
                    Some(m) => m.at(cx, cy),
                    None if cx >= 0 && cy >= 0 && cx < cw as i64 && cy < ch as i64 => 1.0,
                    None => 0.0,
                };
                wr[x] = m.clamp(0.0, 1.0) * p[3].clamp(0.0, 1.0);
            }
        });
        Field { w, h, luma, red, wt }
    }

    #[inline]
    fn signal(&self, kind: Kind, i: usize) -> f32 {
        match kind {
            Kind::Dark => K_RED * self.red[i] - self.luma[i],
            Kind::Light => self.luma[i],
        }
    }

    #[inline]
    fn inside(&self, i: usize) -> bool {
        self.wt[i] >= 0.5
    }
}

/// A scale-space peak, in full-resolution layer coordinates (pixel centers at +0.5).
struct Cand {
    x: f64,
    y: f64,
    sigma: f64,
    /// Peak over the local texture spread.
    sig: f32,
}

/// A measured spot, in layer coordinates.
#[derive(Clone, Debug)]
struct Spot {
    x: f64,
    y: f64,
    radius: f64,
    /// Signal units (0..1).
    contrast: f32,
    redness: f32,
    kind: Kind,
}

/// Radius of the disc healed for a spot of `radius`.
fn heal_radius(radius: f64) -> f64 {
    1.25 * radius + 1.0
}

fn median(v: &mut [f32]) -> f32 {
    if v.is_empty() {
        return 0.0;
    }
    let m = v.len() / 2;
    v.select_nth_unstable_by(m, |a, b| a.total_cmp(b));
    v[m]
}

/// Scale-space peaks of one kind of spot. Scales run in octaves: each group of scales is
/// computed at the power-of-two block resolution that brings its sigma to 1..2 px, so the
/// total work is a few full-resolution blurs whatever the size range.
fn candidates(field: &Field, kind: Kind, set: &Settings, ctx: &OpCtx) -> Result<Vec<Cand>> {
    let (w, h) = (field.w, field.h);
    if w == 0 || h == 0 {
        return Ok(Vec::new());
    }
    // a disc of diameter d answers best at sigma ≈ d/3.5, a Gaussian spot at ≈ d/4.2
    let lo = set.min_d / 4.6;
    let hi = (set.max_d / 3.2).max(lo * STEP);
    let mut q = STEP;
    let mut n = ((hi / lo).ln() / q.ln()).ceil() as usize + 1;
    if n > MAX_SCALES {
        q = (hi / lo).powf(1.0 / (MAX_SCALES - 1) as f64);
        n = MAX_SCALES;
    }
    // full-resolution sigma of scale k (detection scales are 1..=n)
    let sigma = |k: usize| lo * q.powi(k as i32 - 1);
    let cap = 1usize << (usize::BITS - 1 - w.min(h).leading_zeros());
    let factor = |k: usize| (1usize << (sigma(k).max(1.0).log2().floor() as u32)).min(cap);
    let mut out = Vec::new();
    let mut k0 = 1;
    while k0 <= n {
        let f = factor(k0);
        let mut k1 = k0;
        while k1 < n && factor(k1 + 1) == f {
            k1 += 1;
        }
        out.extend(octave(field, kind, set, ctx, f, k0, k1, &sigma, q)?);
        k0 = k1 + 1;
    }
    Ok(out)
}

/// Peaks at detection scales `k0..=k1`, computed at block factor `f`.
fn octave(
    field: &Field,
    kind: Kind,
    set: &Settings,
    ctx: &OpCtx,
    f: usize,
    k0: usize,
    k1: usize,
    sigma: &dyn Fn(usize) -> f64,
    q: f64,
) -> Result<Vec<Cand>> {
    let (w, h) = (field.w, field.h);
    let (ww, wh) = (w.div_ceil(f), h.div_ceil(f));
    // block means of wt·signal and wt; a block is inside when half its weight is there
    let mut base = Buf::new(ww, wh, 2);
    let mut inside = vec![false; ww * wh];
    let norm = 1.0 / (f * f) as f32;
    base.data.par_chunks_mut(ww * 2).zip(inside.par_chunks_mut(ww)).enumerate().for_each(|(by, (row, ins))| {
        for bx in 0..ww {
            let (mut sw, mut sws, mut cnt) = (0f32, 0f32, 0usize);
            for y in by * f..((by + 1) * f).min(h) {
                for x in bx * f..((bx + 1) * f).min(w) {
                    let i = y * w + x;
                    sw += field.wt[i];
                    sws += field.wt[i] * field.signal(kind, i);
                    cnt += 1;
                }
            }
            row[bx * 2] = sws * norm;
            row[bx * 2 + 1] = sw * norm;
            ins[bx] = cnt > 0 && sw >= 0.5 * cnt as f32;
        }
    });
    if !inside.iter().any(|&b| b) {
        return Ok(Vec::new());
    }
    // masked (normalized) Gaussian of the signal at scale j, in working px
    let level = |j: usize| -> Vec<f32> {
        let mut b = base.clone();
        let s = sigma(j) / f as f64;
        kern::gaussian(&mut b, s, s, Edge::Zero);
        b.data.par_chunks(2).map(|p| if p[1] > 1e-6 { p[0] / p[1] } else { 0.0 }).collect()
    };
    let grid = Grid { w: ww, h: wh, inside: &inside };
    let mut ls: VecDeque<Vec<f32>> = VecDeque::new();
    let mut ds: VecDeque<Vec<f32>> = VecDeque::new();
    let mut out = Vec::new();
    // levels k0−1 ..= k1+3 give D_{k0−1} ..= D_{k1+1} (D_j = L_j − L_{j+2})
    for j in k0 - 1..=k1 + 3 {
        ctx.check_cancel()?;
        ls.push_back(level(j));
        if ls.len() > 3 {
            ls.pop_front();
        }
        if ls.len() == 3 {
            ds.push_back(ls[0].par_iter().zip(ls[2].par_iter()).map(|(a, b)| a - b).collect());
            if ds.len() > 3 {
                ds.pop_front();
            }
        }
        if ds.len() == 3 {
            // ds holds D_{k−1}, D_k, D_{k+1} with k = j − 3
            let k = j - 3;
            out.extend(scan(&grid, [&ds[0], &ds[1], &ds[2]], sigma(k) / f as f64, q, f, set));
        }
    }
    Ok(out)
}

struct Grid<'a> {
    w: usize,
    h: usize,
    inside: &'a [bool],
}

/// The texture level around every pixel at one scale: the mean |D| (× 1.2533, the spread
/// of a Gaussian with that mean absolute value) over the search area in a window of 8σ
/// with the central 3σ left out, so a spot does not raise its own threshold. Purely local:
/// flat regions elsewhere in the search area (backgrounds, hair) do not lower it on skin.
fn local_spread(g: &Grid, d: &[f32], sigma: f64) -> Vec<f32> {
    let (ww, wh) = (g.w, g.h);
    let r_in = (3.0 * sigma).ceil() as usize;
    let r_out = ((8.0 * sigma).round() as usize).max(r_in + 4);
    let mut inner = Buf::new(ww, wh, 2);
    inner.data.par_chunks_mut(2).enumerate().for_each(|(i, p)| {
        if g.inside[i] {
            p[0] = d[i].abs();
            p[1] = 1.0;
        }
    });
    let mut outer = inner.clone();
    kern::box_blur(&mut inner, r_in, r_in, Edge::Zero);
    kern::box_blur(&mut outer, r_out, r_out, Edge::Zero);
    let (a_in, a_out) = (((2 * r_in + 1) * (2 * r_in + 1)) as f32, ((2 * r_out + 1) * (2 * r_out + 1)) as f32);
    let min_ring = 0.1 * (a_out - a_in);
    inner
        .data
        .par_chunks(2)
        .zip(outer.data.par_chunks(2))
        .map(|(pi, po)| {
            let (sum, n) = (po[0] * a_out - pi[0] * a_in, po[1] * a_out - pi[1] * a_in);
            let m = if n >= min_ring {
                sum / n
            } else if po[1] > 0.0 {
                po[0] / po[1]
            } else {
                0.0
            };
            1.2533 * m.max(0.0)
        })
        .collect()
}

/// Peaks of `d[1]` (the band-pass at `sigma`, working px) against its neighbors in space
/// and in the scales below and above (`d[0]`, `d[2]`).
fn scan(g: &Grid, d: [&Vec<f32>; 3], sigma: f64, q: f64, f: usize, set: &Settings) -> Vec<Cand> {
    let (ww, wh) = (g.w, g.h);
    let cur = d[1];
    let spread = local_spread(g, cur, sigma);
    let floor = 0.2 * set.min_contrast;
    let ratio = (EDGE_RATIO + 1.0).powi(2) / EDGE_RATIO;
    let hs = ((0.5 * sigma).round() as i64).max(1);
    let reach = 0.7 * 2.1 * sigma;
    let (wi, hi) = (ww as i64, wh as i64);
    let at = |p: &[f32], x: i64, y: i64| p[(y.clamp(0, hi - 1) * wi + x.clamp(0, wi - 1)) as usize];
    let rows: Vec<Vec<Cand>> = (0..wh)
        .into_par_iter()
        .map(|y| {
            let mut found = Vec::new();
            let yi = y as i64;
            'px: for x in 0..ww {
                let i = y * ww + x;
                let v = cur[i];
                if !g.inside[i] || v <= floor {
                    continue;
                }
                let spread = spread[i];
                if v < set.tau * spread {
                    continue;
                }
                let xi = x as i64;
                // strict maximum over 3×3×3 (ties go to the earliest in scale, row, column)
                for (s, plane) in d.iter().enumerate() {
                    for dy in -1i64..=1 {
                        for dx in -1i64..=1 {
                            if s == 1 && dy == 0 && dx == 0 {
                                continue;
                            }
                            let (nx, ny) = (xi + dx, yi + dy);
                            if nx < 0 || ny < 0 || nx >= wi || ny >= hi {
                                continue;
                            }
                            let nv = plane[(ny * wi + nx) as usize];
                            let earlier = s == 0 || (s == 1 && (dy < 0 || (dy == 0 && dx < 0)));
                            if nv > v || (nv == v && earlier) {
                                continue 'px;
                            }
                        }
                    }
                }
                // round: the Hessian is negative definite with a bounded eigenvalue ratio
                let dxx = at(cur, xi + hs, yi) + at(cur, xi - hs, yi) - 2.0 * v;
                let dyy = at(cur, xi, yi + hs) + at(cur, xi, yi - hs) - 2.0 * v;
                let dxy = (at(cur, xi + hs, yi + hs) - at(cur, xi + hs, yi - hs) - at(cur, xi - hs, yi + hs)
                    + at(cur, xi - hs, yi - hs))
                    / 4.0;
                let (tr, det) = (dxx + dyy, dxx * dyy - dxy * dxy);
                if !(tr < 0.0 && det > 0.0 && tr * tr < ratio * det) {
                    continue;
                }
                // the core of the spot is in the search area (checked exactly later)
                let rr = reach.ceil() as i64;
                for yy in yi - rr..=yi + rr {
                    for xx in xi - rr..=xi + rr {
                        let dd = ((xx - xi).pow(2) + (yy - yi).pow(2)) as f64;
                        if dd <= reach * reach
                            && (xx < 0 || yy < 0 || xx >= wi || yy >= hi || !g.inside[(yy * wi + xx) as usize])
                        {
                            continue 'px;
                        }
                    }
                }
                // sub-pixel position and scale from parabolas through the neighbors
                let vertex = |a: f32, b: f32| {
                    let den = a - 2.0 * v + b;
                    if den < 0.0 {
                        (0.5 * (a - b) / den).clamp(-0.5, 0.5) as f64
                    } else {
                        0.0
                    }
                };
                let ox = vertex(at(cur, xi - 1, yi), at(cur, xi + 1, yi));
                let oy = vertex(at(cur, xi, yi - 1), at(cur, xi, yi + 1));
                let os = vertex(d[0][i], d[2][i]);
                found.push(Cand {
                    x: (x as f64 + 0.5 + ox) * f as f64,
                    y: (y as f64 + 0.5 + oy) * f as f64,
                    sigma: sigma * q.powf(os) * f as f64,
                    sig: v / spread,
                });
            }
            found
        })
        .collect();
    rows.into_iter().flatten().collect()
}

/// Median signal and redness of the skin in a ring around (cx, cy): None when too little
/// of the ring is in the search area.
fn surround(field: &Field, kind: Kind, cx: f64, cy: f64, r0: f64) -> Option<(f32, f32)> {
    let (r_in, r_out) = (1.5 * r0 + 1.0, 2.5 * r0 + 2.0);
    let mut s = Vec::new();
    let mut red = Vec::new();
    let y0 = ((cy - r_out).floor() as i64).max(0);
    let y1 = ((cy + r_out).ceil() as i64).min(field.h as i64 - 1);
    let x0 = ((cx - r_out).floor() as i64).max(0);
    let x1 = ((cx + r_out).ceil() as i64).min(field.w as i64 - 1);
    for y in y0..=y1 {
        for x in x0..=x1 {
            let d = ((x as f64 + 0.5 - cx).powi(2) + (y as f64 + 0.5 - cy).powi(2)).sqrt();
            let i = y as usize * field.w + x as usize;
            if d >= r_in && d <= r_out && field.inside(i) {
                s.push(field.signal(kind, i));
                red.push(field.red[i]);
            }
        }
    }
    if s.len() < 8 {
        return None;
    }
    Some((median(&mut s), median(&mut red)))
}

/// Measure a candidate at full resolution: None when it is not a spot within the limits.
fn measure(field: &Field, kind: Kind, set: &Settings, c: &Cand) -> Option<Spot> {
    let r0 = (2.1 * c.sigma).max(0.75);
    let (w, h) = (field.w as i64, field.h as i64);
    let pixels = |cx: f64, cy: f64, r: f64| {
        let y0 = ((cy - r).floor() as i64).max(0);
        let y1 = ((cy + r).ceil() as i64).min(h - 1);
        let x0 = ((cx - r).floor() as i64).max(0);
        let x1 = ((cx + r).ceil() as i64).min(w - 1);
        (y0..=y1).flat_map(move |y| (x0..=x1).map(move |x| (x, y))).filter_map(move |(x, y)| {
            let d = ((x as f64 + 0.5 - cx).powi(2) + (y as f64 + 0.5 - cy).powi(2)).sqrt();
            (d <= r).then_some((y as usize * field.w + x as usize, x, y, d))
        })
    };
    // re-center on the spot: centroid of the signal above the surround
    let (b, _) = surround(field, kind, c.x, c.y, r0)?;
    let (mut sx, mut sy, mut sw) = (0f64, 0f64, 0f64);
    for (i, x, y, _) in pixels(c.x, c.y, r0) {
        let e = (field.signal(kind, i) - b).max(0.0) as f64;
        if field.inside(i) && e > 0.0 {
            sx += e * (x as f64 + 0.5);
            sy += e * (y as f64 + 0.5);
            sw += e;
        }
    }
    let (cx, cy) = if sw > 0.0 && ((sx / sw - c.x).powi(2) + (sy / sw - c.y).powi(2)).sqrt() <= 0.5 * r0 {
        (sx / sw, sy / sw)
    } else {
        (c.x, c.y)
    };
    let (b, red_b) = surround(field, kind, cx, cy, r0)?;
    // center level and radial profile
    let rc = (0.4 * r0).max(0.75);
    let nb = (2.5 * r0 + 2.0).ceil() as usize + 1;
    let mut bins = vec![(0f64, 0usize); nb];
    let (mut cs, mut cred, mut cn) = (0f64, 0f64, 0usize);
    for (i, _, _, d) in pixels(cx, cy, nb as f64) {
        if !field.inside(i) {
            continue;
        }
        let s = field.signal(kind, i) as f64;
        if d <= rc {
            cs += s;
            cred += field.red[i] as f64;
            cn += 1;
        }
        let k = d as usize;
        if k < nb {
            bins[k].0 += s;
            bins[k].1 += 1;
        }
    }
    if cn == 0 {
        return None;
    }
    let contrast = (cs / cn as f64) as f32 - b;
    if contrast < set.min_contrast {
        return None;
    }
    let quarter = 0.25 * contrast as f64;
    let mut prev: Option<(f64, f64)> = None;
    let mut radius = None;
    for (k, &(sum, cnt)) in bins.iter().enumerate() {
        if cnt == 0 {
            continue;
        }
        let (rho, p) = (k as f64 + 0.5, sum / cnt as f64 - b as f64);
        if p <= quarter {
            radius = Some(match prev {
                Some((r1, p1)) if p1 > p => r1 + (rho - r1) * (p1 - quarter) / (p1 - p),
                _ => rho,
            });
            break;
        }
        prev = Some((rho, p));
    }
    let radius = radius?.max(0.5);
    let diameter = 2.0 * radius;
    if diameter < set.min_d || diameter > set.max_d {
        return None;
    }
    // the spot (plus a pixel) lies inside the search area and the layer
    let hr = radius + 1.5;
    if cx - hr < 0.0 || cy - hr < 0.0 || cx + hr > w as f64 || cy + hr > h as f64 {
        return None;
    }
    if pixels(cx, cy, hr).any(|(i, ..)| !field.inside(i)) {
        return None;
    }
    Some(Spot { x: cx, y: cy, radius, contrast, redness: (cred / cn as f64) as f32 - red_b, kind })
}

/// Strongest first; a spot whose center lies within a stronger one's radius (or that
/// holds a stronger one's center) is the same blemish and is dropped.
fn rank(mut spots: Vec<Spot>, max_d: f64) -> Vec<Spot> {
    spots.sort_by(|a, b| {
        b.contrast.total_cmp(&a.contrast).then(a.y.total_cmp(&b.y)).then(a.x.total_cmp(&b.x)).then(a.kind.cmp(&b.kind))
    });
    let cell = (max_d / 2.0).max(1.0);
    let key = |s: &Spot| ((s.x / cell).floor() as i64, (s.y / cell).floor() as i64);
    let mut grid: BTreeMap<(i64, i64), Vec<usize>> = BTreeMap::new();
    let mut kept: Vec<Spot> = Vec::new();
    for s in spots {
        let (kx, ky) = key(&s);
        let clash = (ky - 1..=ky + 1).flat_map(|y| (kx - 1..=kx + 1).map(move |x| (x, y))).any(|c| {
            grid.get(&c).is_some_and(|ids| {
                ids.iter().any(|&j| {
                    let o = &kept[j];
                    ((o.x - s.x).powi(2) + (o.y - s.y).powi(2)).sqrt() < o.radius.max(s.radius)
                })
            })
        });
        if !clash {
            grid.entry((kx, ky)).or_default().push(kept.len());
            kept.push(s);
        }
    }
    kept
}

/// The heal discs of `spots` as a canvas-sized soft mask (layer offset `ox`, `oy`), limited
/// to the search area so nothing outside it changes.
fn disc_mask(cw: u32, ch: u32, spots: &[Spot], ox: i32, oy: i32, search: Option<&Mask>) -> Mask {
    let mut m = Mask::new(cw, ch);
    for s in spots {
        let (cx, cy) = (s.x + ox as f64, s.y + oy as f64);
        let r = heal_radius(s.radius);
        let y0 = ((cy - r - 1.0).floor() as i64).max(0);
        let y1 = ((cy + r + 1.0).ceil() as i64).min(ch as i64 - 1);
        let x0 = ((cx - r - 1.0).floor() as i64).max(0);
        let x1 = ((cx + r + 1.0).ceil() as i64).min(cw as i64 - 1);
        for y in y0..=y1 {
            for x in x0..=x1 {
                let d = ((x as f64 + 0.5 - cx).powi(2) + (y as f64 + 0.5 - cy).powi(2)).sqrt();
                let v = (r + 0.5 - d).clamp(0.0, 1.0) as f32 * search.map(|s| s.get(x as u32, y as u32)).unwrap_or(1.0);
                if v > m.get(x as u32, y as u32) {
                    m.set(x as u32, y as u32, v);
                }
            }
        }
    }
    m
}

fn round1(v: f64) -> f64 {
    (v * 10.0).round() / 10.0
}

fn blemishes(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let heal = a.flag("heal");
    let save = a.str("save-mask").filter(|s| !s.trim().is_empty());
    if let Some(n) = &save {
        if n.starts_with('@') {
            return Err(Error::new("PX1010", format!("--save-mask '{n}': mask names cannot start with @")));
        }
    }
    let has_target = ctx.globals.output.is_some() || ctx.globals.in_place;
    if (heal || save.is_some()) && !has_target && !ctx.dry_run() && !ctx.replaying {
        return Err(Error::new(
            "PX1004",
            "'retouch blemishes' changes the document with --heal or --save-mask; say where to write it",
        )
        .with_hint("add -o out.pxo (or --in-place on a .pxo), or drop --heal and --save-mask to only list the spots"));
    }
    let path = doc.resolve_or_only(a.raw("layer"))?;
    let set = settings(a, doc, ctx)?;
    let search = ctx.mask_arg(a, doc)?;
    let layer = doc.layer(&path);
    let name = layer.name.clone();
    if !layer.visible {
        ctx.warn("PXW2001", format!("layer '{name}' is hidden"));
    }
    if heal {
        layer.check_unlocked()?;
        layer.raster().map_err(|e| e.with_hint("rasterize it first, or run without --heal to only list the spots"))?;
    }
    let (img, ox, oy): (Cow<Image>, i32, i32) = match &layer.content {
        LayerKind::Raster { image } => (Cow::Borrowed(image), layer.x, layer.y),
        _ => match compose::render_layer(doc, layer)? {
            Some(r) => (Cow::Owned(r.image), r.x, r.y),
            None => (Cow::Owned(Image::new(0, 0)), 0, 0),
        },
    };
    let field = Field::new(&img, ox, oy, doc.width, doc.height, search.as_ref());
    drop(img);
    let cap = set.max.saturating_mul(8).max(1000);
    let mut spots = Vec::new();
    for &kind in &set.kinds {
        let mut cands = candidates(&field, kind, &set, ctx)?;
        cands.sort_by(|a, b| b.sig.total_cmp(&a.sig).then(a.y.total_cmp(&b.y)).then(a.x.total_cmp(&b.x)));
        cands.truncate(cap);
        spots.extend(cands.par_iter().filter_map(|c| measure(&field, kind, &set, c)).collect::<Vec<_>>());
    }
    let mut spots = rank(spots, set.max_d);
    if let Some(r) = a.opt_f64("min-redness")? {
        // levels in the result; redness is stored in 0..1 units
        spots.retain(|s| s.redness as f64 * 255.0 >= r);
    }
    let found = spots.len();
    spots.truncate(set.max);

    let discs = (heal || save.is_some()).then(|| disc_mask(doc.width, doc.height, &spots, ox, oy, search.as_ref()));
    if let (Some(n), Some(m)) = (&save, &discs) {
        doc.put_mask(n, m.clone(), true)?;
    }
    let (regions, changed) = match &discs {
        Some(m) if heal && !spots.is_empty() => {
            let r = heal_region(doc, &path, m)?;
            (r.regions.len(), r.pixels_changed)
        }
        _ => (0, 0),
    };
    let list: Vec<Value> = spots
        .iter()
        .map(|s| {
            json!({
                "x": round1(s.x + ox as f64),
                "y": round1(s.y + oy as f64),
                "radius": round1(s.radius),
                "contrast": round1(s.contrast as f64 * 255.0),
                "kind": s.kind.name(),
                "redness": round1(s.redness as f64 * 255.0),
            })
        })
        .collect();
    let mut v = json!({
        "layer": name,
        "spots": list,
        "count": spots.len(),
        "found": found,
        "truncated": found > spots.len(),
        "settings": {
            "min_size": set.min_d,
            "max_size": set.max_d,
            "sensitivity": set.sensitivity,
            "kind": set.kind,
            "max": set.max,
            "min_redness": a.opt_f64("min-redness")?,
        },
        "searched": if search.is_some() { "mask" } else { "layer" },
        "healed": heal,
        "healed_regions": regions,
        "pixels_changed": changed,
        "saved_mask": save,
    });
    if search.is_none() {
        let msg = "no --mask: the whole layer was searched, so eyes, nostrils, lips, and hair may be listed; pass a skin mask (select skin)";
        ctx.warn("PXW2005", msg);
        v["note"] = json!(msg);
    }
    Ok(v)
}

// ---------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::ctx::{Globals, ImageInfo, NoIo, NoProgress, OpenOptions};
    use pixelox_core::envelope::Warning;
    use pixelox_core::testing::{self, find, raster};
    use pixelox_core::{ImageIo, Runner};
    use std::path::Path;
    use std::sync::Mutex;

    fn op() -> OpSpec {
        find(&crate::ops(), "retouch blemishes")
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Mark {
        /// A pimple: redder and darker.
        Red,
        /// A brown mark: darker.
        Brown,
        /// A whitehead: lighter.
        White,
    }

    type Marks = Vec<(f32, f32, f32, Mark)>;

    /// Light to very dark skin tones (8-bit sRGB).
    const TONES: [[f32; 3]; 4] =
        [[232.0, 190.0, 165.0], [190.0, 135.0, 100.0], [120.0, 78.0, 55.0], [72.0, 48.0, 36.0]];

    fn hash(x: i64, y: i64, seed: u32) -> f32 {
        let mut z = (x as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F);
        z ^= seed as u64;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        (z >> 40) as f32 / (1u64 << 24) as f32
    }

    /// Synthetic skin: a warm base lit from the left, fine noise, sparse pores (dark dots of
    /// about 1 px), and marks (x, y, radius out to a quarter of their strength, kind);
    /// quantized to 8 bits like a decoded photo.
    fn skin(w: u32, h: u32, tone: [f32; 3], seed: u32, marks: &[(f32, f32, f32, Mark)]) -> Image {
        Image::from_fn(w, h, |x, y| {
            let light = 1.06 - 0.22 * x as f32 / w as f32 - 0.08 * y as f32 / h as f32;
            let mut c = [tone[0] * light, tone[1] * light, tone[2] * light];
            // pores: one candidate per 5×5 cell, 60% present
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            let mut pore = 0f32;
            let (cx, cy) = (x as i64 / 5, y as i64 / 5);
            for gy in cy - 1..=cy + 1 {
                for gx in cx - 1..=cx + 1 {
                    if hash(gx, gy, seed ^ 0x55) > 0.6 {
                        continue;
                    }
                    let px = gx as f32 * 5.0 + 5.0 * hash(gx, gy, seed ^ 0x11);
                    let py = gy as f32 * 5.0 + 5.0 * hash(gx, gy, seed ^ 0x22);
                    let depth = 0.04 + 0.05 * hash(gx, gy, seed ^ 0x33);
                    let d2 = (fx - px).powi(2) + (fy - py).powi(2);
                    pore = pore.max(depth * (-d2 / (2.0 * 0.6 * 0.6)).exp());
                }
            }
            let n = hash(x as i64, y as i64, seed) - 0.5;
            for (k, v) in c.iter_mut().enumerate() {
                *v = *v * (1.0 - pore) * (1.0 + 0.05 * n) + 4.0 * n * (1.0 - 0.1 * k as f32);
            }
            for &(mx, my, r, kind) in marks {
                let s = r / 1.665;
                let d2 = (fx - mx).powi(2) + (fy - my).powi(2);
                if d2 > 16.0 * s * s {
                    continue;
                }
                let g = (-d2 / (2.0 * s * s)).exp();
                match kind {
                    Mark::Red => {
                        c[0] *= 1.0 - 0.04 * g;
                        c[1] *= 1.0 - 0.32 * g;
                        c[2] *= 1.0 - 0.3 * g;
                    }
                    Mark::Brown => c.iter_mut().for_each(|v| *v *= 1.0 - 0.3 * g),
                    Mark::White => c.iter_mut().for_each(|v| *v += (255.0 - *v) * 0.35 * g),
                }
            }
            let q8 = |v: f32| ((v / 255.0).clamp(0.0, 1.0) * 255.0).round() / 255.0;
            [q8(c[0]), q8(c[1]), q8(c[2]), 1.0]
        })
    }

    /// 16 marks of radius 3..6.5 on a grid in the left two thirds of a 320×240 image (the
    /// right third, x > 215, is pores only), cycling through `kinds`.
    fn grid_marks(kinds: &[Mark]) -> Marks {
        let radii = [3.0, 4.0, 5.0, 6.5];
        let mut v = Vec::new();
        for (i, (gy, gx)) in (0..4).flat_map(|gy| (0..4).map(move |gx| (gy, gx))).enumerate() {
            let x = 30.0 + gx as f32 * 45.0 + 3.0 * hash(gx, gy, 9);
            let y = 30.0 + gy as f32 * 55.0 + 3.0 * hash(gx, gy, 10);
            v.push((x, y, radii[i % 4], kinds[(i + i / 4) % kinds.len()]));
        }
        v
    }

    /// Run with or without a destination (`-o`), returning the context too.
    fn run_to(pairs: &[(&str, &str)], doc: &mut Document, output: bool) -> Result<(Value, OpCtx<'static>)> {
        let spec = op();
        let a = testing::args(&spec, pairs)?;
        let globals = Globals { output: output.then(|| "out.pxo".to_string()), ..Default::default() };
        let mut ctx = OpCtx::new(spec.name, vec![], globals, &NoIo, &NoProgress);
        let Runner::Doc(f) = spec.runner else { panic!() };
        let v = f(&a, doc, &mut ctx)?;
        Ok((v, ctx))
    }

    fn report(pairs: &[(&str, &str)], img: &Image) -> Value {
        let mut doc = testing::doc_from(img.clone());
        run_to(pairs, &mut doc, false).unwrap().0
    }

    /// (x, y, radius, contrast, kind) of each reported spot.
    fn spots(v: &Value) -> Vec<(f64, f64, f64, f64, String)> {
        v["spots"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| {
                let f = |k: &str| s[k].as_f64().unwrap();
                (f("x"), f("y"), f("radius"), f("contrast"), s["kind"].as_str().unwrap().to_string())
            })
            .collect()
    }

    /// The mark a spot found, if any (center within max(3, r) px).
    fn matched<'a>(marks: &'a Marks, s: &(f64, f64, f64, f64, String)) -> Option<&'a (f32, f32, f32, Mark)> {
        marks.iter().find(|m| ((m.0 as f64 - s.0).powi(2) + (m.1 as f64 - s.1).powi(2)).sqrt() < (m.2 as f64).max(3.0))
    }

    fn recall(marks: &Marks, found: &[(f64, f64, f64, f64, String)], kinds: &[Mark]) -> f64 {
        let want: Vec<_> = marks.iter().filter(|m| kinds.contains(&m.3)).collect();
        let hit = want
            .iter()
            .filter(|m| {
                found
                    .iter()
                    .any(|s| ((m.0 as f64 - s.0).powi(2) + (m.1 as f64 - s.1).powi(2)).sqrt() < (m.2 as f64).max(3.0))
            })
            .count();
        hit as f64 / want.len() as f64
    }

    #[test]
    fn finds_dark_and_red_spots_on_every_tone_but_not_pores() {
        let marks = grid_marks(&[Mark::Red, Mark::Brown]);
        for (t, tone) in TONES.iter().enumerate() {
            let img = skin(320, 240, *tone, 7 + t as u32, &marks);
            let v = report(&[("max-size", "20")], &img);
            let found = spots(&v);
            let r = recall(&marks, &found, &[Mark::Red, Mark::Brown]);
            assert!(r >= 0.9, "tone {t}: recall {r}, found {found:?}");
            for s in &found {
                let m = matched(&marks, s).unwrap_or_else(|| panic!("tone {t}: detection on pores {s:?}"));
                assert!(s.0 < 215.0, "tone {t}: detection in the pore-only area {s:?}");
                assert_eq!(s.4, "dark");
                assert!((s.2 - m.2 as f64).abs() <= 0.35 * m.2 as f64 + 0.5, "tone {t}: radius {} for {}", s.2, m.2);
                assert!(s.3 > 0.0);
            }
            // strongest first
            assert!(found.windows(2).all(|p| p[0].3 >= p[1].3), "tone {t}: not sorted by contrast");
            assert_eq!(v["searched"], "layer");
            assert!(v["note"].is_string());
        }
    }

    #[test]
    fn kind_selects_dark_or_light_spots() {
        let marks = grid_marks(&[Mark::Red, Mark::White]);
        for (t, tone) in TONES.iter().enumerate().step_by(2) {
            let img = skin(320, 240, *tone, 3 + t as u32, &marks);
            let light = spots(&report(&[("max-size", "20"), ("kind", "light")], &img));
            assert!(recall(&marks, &light, &[Mark::White]) >= 0.9, "tone {t}: {light:?}");
            for s in &light {
                assert_eq!(matched(&marks, s).map(|m| m.3), Some(Mark::White), "tone {t}: {s:?}");
                assert_eq!(s.4, "light");
            }
            let dark = spots(&report(&[("max-size", "20")], &img));
            assert!(recall(&marks, &dark, &[Mark::Red]) >= 0.9);
            assert!(dark.iter().all(|s| matched(&marks, s).map(|m| m.3) == Some(Mark::Red)), "tone {t}: {dark:?}");
            let both = spots(&report(&[("max-size", "20"), ("kind", "both")], &img));
            assert!(recall(&marks, &both, &[Mark::Red, Mark::White]) >= 0.9);
            assert!(both.iter().all(|s| matched(&marks, s).is_some()));
        }
    }

    #[test]
    fn masks_exclude_spots_outside_and_on_their_edge() {
        let (w, h) = (320u32, 240u32);
        let inside: Marks =
            vec![(40.0, 40.0, 4.0, Mark::Red), (90.0, 120.0, 5.0, Mark::Brown), (50.0, 200.0, 3.5, Mark::Red)];
        let outside: Marks = vec![(250.0, 60.0, 5.0, Mark::Red), (220.0, 180.0, 4.0, Mark::Brown)];
        // centered on the mask edge (x = 160), and just inside it with the spot crossing it
        let straddling: Marks = vec![(160.0, 60.0, 5.0, Mark::Red), (157.0, 150.0, 6.0, Mark::Brown)];
        let all: Marks = inside.iter().chain(&outside).chain(&straddling).copied().collect();
        let mut img = skin(w, h, TONES[1], 11, &all);
        // a dark feature (an eye, say) just outside the mask
        for y in 0..h {
            for x in 0..w {
                let d = ((x as f32 - 172.0).powi(2) + ((y as f32 - 205.0) / 0.6).powi(2)).sqrt();
                if d < 14.0 {
                    img.set(x, y, [0.2, 0.13, 0.1, 1.0]);
                }
            }
        }
        let mut doc = testing::doc_from(img.clone());
        doc.put_mask("skin", testing::half_mask(w, h), false).unwrap();
        let (v, _) = run_to(&[("mask", "skin"), ("max-size", "20")], &mut doc, false).unwrap();
        let found = spots(&v);
        assert_eq!(v["searched"], "mask");
        assert!(v.get("note").is_none());
        assert_eq!(recall(&inside, &found, &[Mark::Red, Mark::Brown]), 1.0, "{found:?}");
        assert_eq!(found.len(), inside.len(), "only the spots inside: {found:?}");
        // inverted: the right half, without the straddling spots or the feature
        let (v, _) = run_to(&[("mask", "skin"), ("invert-mask", "true"), ("max-size", "20")], &mut doc, false).unwrap();
        let found = spots(&v);
        assert_eq!(recall(&outside, &found, &[Mark::Red, Mark::Brown]), 1.0, "{found:?}");
        assert_eq!(found.len(), outside.len(), "{found:?}");
        // an empty mask finds nothing and says so
        doc.put_mask("none", Mask::new(w, h), false).unwrap();
        let spec = op();
        let (v, warnings) = testing::run_with_warnings(&spec, &[("mask", "none")], &mut doc).unwrap();
        assert_eq!(v["count"], 0);
        assert_eq!(warnings, vec!["PXW2002".to_string()]);
    }

    #[test]
    fn size_limits_select_spots() {
        let marks: Marks = vec![
            (40.0, 40.0, 2.5, Mark::Brown),
            (120.0, 40.0, 2.5, Mark::Red),
            (60.0, 150.0, 7.0, Mark::Brown),
            (160.0, 160.0, 7.0, Mark::Red),
        ];
        let img = skin(240, 200, TONES[1], 5, &marks);
        let (small, big) = (&marks[..2], &marks[2..]);
        let all = spots(&report(&[("min-size", "3"), ("max-size", "20")], &img));
        assert_eq!(all.len(), 4, "{all:?}");
        let only_big = spots(&report(&[("min-size", "9"), ("max-size", "20")], &img));
        assert_eq!(recall(&big.to_vec(), &only_big, &[Mark::Red, Mark::Brown]), 1.0);
        assert_eq!(only_big.len(), 2, "{only_big:?}");
        let only_small = spots(&report(&[("min-size", "3"), ("max-size", "9")], &img));
        assert_eq!(recall(&small.to_vec(), &only_small, &[Mark::Red, Mark::Brown]), 1.0);
        assert_eq!(only_small.len(), 2, "{only_small:?}");
        // % sizes are relative to the long side: 5% of 240 = 12 px
        let v = report(&[("min-size", "5%"), ("max-size", "20")], &img);
        assert_eq!(v["settings"]["min_size"], 12.0);
        assert_eq!(spots(&v).len(), 2);
        // --max keeps the strongest
        let v = report(&[("min-size", "3"), ("max-size", "20"), ("max", "1")], &img);
        assert_eq!((v["count"].clone(), v["found"].clone(), v["truncated"].clone()), (json!(1), json!(4), json!(true)));
        assert_eq!(spots(&v)[0].3, all[0].3);
    }

    /// Mean of a channel over a disc, and the spread of fine detail (pixel minus a 1.5 px
    /// blur) over a disc.
    fn disc_stats(img: &Image, fine: &Image, cx: f32, cy: f32, r: f32) -> (f64, f64) {
        let (mut sum, mut n, mut f2) = (0f64, 0f64, 0f64);
        for y in 0..img.height {
            for x in 0..img.width {
                if ((x as f32 + 0.5 - cx).powi(2) + (y as f32 + 0.5 - cy).powi(2)).sqrt() <= r {
                    sum += img.get(x, y)[1] as f64;
                    f2 += (fine.get(x, y)[1] as f64).powi(2);
                    n += 1.0;
                }
            }
        }
        (sum / n, (f2 / n).sqrt())
    }

    fn fine_detail(img: &Image) -> Image {
        let b = kern::gaussian_image(img, 1.5);
        Image { width: img.width, height: img.height, data: img.data.iter().zip(&b.data).map(|(a, b)| a - b).collect() }
    }

    #[test]
    fn heal_removes_spots_and_keeps_pore_texture() {
        let marks = grid_marks(&[Mark::Red, Mark::Brown]);
        let base = skin(320, 240, TONES[1], 21, &[]);
        let img = skin(320, 240, TONES[1], 21, &marks);
        let mut doc = testing::doc_from(img.clone());
        let (v, _) = run_to(&[("max-size", "20"), ("heal", "true")], &mut doc, true).unwrap();
        assert_eq!(v["healed"], true);
        let n = v["count"].as_u64().unwrap() as usize;
        assert!(n >= 15, "{v}");
        assert_eq!(v["healed_regions"].as_u64().unwrap() as usize, n, "each spot is its own region");
        assert!(v["pixels_changed"].as_u64().unwrap() > 0);
        let out = raster(&doc, "Background").clone();
        // nothing is found any more
        let again = spots(&report(&[("max-size", "20")], &out));
        assert!(again.is_empty(), "still found {again:?}");
        // each spot's tone is back to the skin's, and its texture is skin texture, not a blur
        let (fb, fo) = (fine_detail(&base), fine_detail(&out));
        let fi = fine_detail(&img);
        for m in &marks {
            let (mb, tb) = disc_stats(&base, &fb, m.0, m.1, m.2);
            let (mi, _) = disc_stats(&img, &fi, m.0, m.1, m.2);
            let (mo, to) = disc_stats(&out, &fo, m.0, m.1, m.2);
            assert!(
                (mo - mb).abs() < 0.25 * (mi - mb).abs() + 1.0 / 255.0,
                "{m:?}: green {mo} vs skin {mb} (spotted {mi})"
            );
            assert!(to > 0.7 * tb && to < 1.4 * tb, "{m:?}: texture {to} vs {tb}");
        }
        // pore-only areas are untouched
        for y in 0..240 {
            for x in 230..320 {
                assert_eq!(out.get(x, y), img.get(x, y));
            }
        }
        // the whole image's fine-detail spread stays close to the clean skin's
        let spread =
            |f: &Image| (f.data.chunks(4).map(|p| (p[1] as f64).powi(2)).sum::<f64>() / f.pixels() as f64).sqrt();
        let (sb, so) = (spread(&fb), spread(&fo));
        assert!((so - sb).abs() < 0.1 * sb, "texture spread {so} vs {sb}");
    }

    #[test]
    fn saved_mask_heals_like_heal() {
        let marks = grid_marks(&[Mark::Red, Mark::Brown]);
        let img = skin(320, 240, TONES[2], 2, &marks);
        let mut healed = testing::doc_from(img.clone());
        run_to(&[("max-size", "20"), ("heal", "true")], &mut healed, true).unwrap();
        let mut doc = testing::doc_from(img.clone());
        let (v, _) = run_to(&[("max-size", "20"), ("save-mask", "spots")], &mut doc, true).unwrap();
        assert_eq!(v["saved_mask"], "spots");
        assert_eq!(v["healed"], false);
        assert_eq!(raster(&doc, "Background"), &img, "--save-mask alone changes no pixels");
        let m = doc.named_mask("spots").unwrap();
        for s in spots(&v) {
            assert_eq!(m.get(s.0 as u32, s.1 as u32), 1.0, "spot center masked");
        }
        let heal = find(&crate::ops(), "retouch heal");
        testing::run(&heal, &[("mask", "spots")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background"), raster(&healed, "Background"), "retouch heal --mask spots == --heal");
    }

    #[test]
    fn reports_without_a_destination_and_heals_only_with_one() {
        let marks = grid_marks(&[Mark::Red]);
        let img = skin(200, 160, TONES[0], 4, &marks);
        let mut doc = testing::doc_from(img.clone());
        let (v, ctx) = run_to(&[], &mut doc, false).unwrap();
        assert!(v["count"].as_u64().unwrap() > 0);
        assert_eq!(v["healed"], false);
        assert_eq!(raster(&doc, "Background"), &img, "report-only changes nothing");
        // resolved defaults are recorded at full precision for replay
        assert_eq!(ctx.resolved["min_size"], "3");
        assert_eq!(ctx.resolved["max_size"], "12");
        for p in [&[("heal", "true")][..], &[("save-mask", "spots")][..]] {
            let e = run_to(p, &mut doc, false).err().unwrap();
            assert_eq!(e.code, "PX1004", "{p:?}");
            assert!(e.hint.as_deref().unwrap_or("").contains("-o"));
        }
        assert_eq!(run_to(&[("save-mask", "@sel")], &mut doc, true).err().unwrap().code, "PX1010");
        // replays and dry runs need no destination
        let spec = op();
        let Runner::Doc(f) = spec.runner else { panic!() };
        let a = testing::args(&spec, &[("heal", "true")]).unwrap();
        let mut ctx = OpCtx::new(spec.name, vec![], Globals::default(), &NoIo, &NoProgress);
        ctx.replaying = true;
        f(&a, &mut doc.clone(), &mut ctx).unwrap();
        let mut ctx =
            OpCtx::new(spec.name, vec![], Globals { dry_run: true, ..Default::default() }, &NoIo, &NoProgress);
        f(&a, &mut doc.clone(), &mut ctx).unwrap();
    }

    /// An IO layer that opens every path as one fixed document and keeps what is saved.
    struct DocIo {
        doc: Document,
        saved: Mutex<Option<Document>>,
    }

    impl ImageIo for DocIo {
        fn open(&self, _: &Path, _: &OpenOptions) -> Result<(Document, Vec<Warning>)> {
            Ok((self.doc.clone(), vec![]))
        }
        fn decode(&self, _: &[u8], _: Option<&str>, _: &OpenOptions) -> Result<(Image, ImageInfo)> {
            Err(Error::unsupported("test io"))
        }
        fn probe(&self, _: &Path) -> Result<ImageInfo> {
            Err(Error::unsupported("test io"))
        }
        fn encode(&self, _: &Image, _: Option<&Document>, _: &Path, _: &EncodeOptions) -> Result<Vec<Warning>> {
            Err(Error::unsupported("test io"))
        }
        fn save_pxo(&self, doc: &Document, path: &Path) -> Result<()> {
            std::fs::write(path, b"pxo")?;
            *self.saved.lock().unwrap() = Some(doc.clone());
            Ok(())
        }
        fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>) {
            (vec!["png"], vec!["pxo"])
        }
    }

    #[test]
    fn the_executor_runs_it_without_an_output() {
        let td = testing::TempDir::new("blemishes");
        let input = td.path("portrait.png");
        std::fs::write(&input, b"not decoded").unwrap();
        let marks = grid_marks(&[Mark::Red]);
        let io = DocIo { doc: testing::doc_from(skin(200, 160, TONES[1], 8, &marks)), saved: Mutex::new(None) };
        let exec = |name: &str, pairs: &[(&str, &str)], output: Option<String>| {
            let spec = find(&crate::ops(), name);
            let a = testing::args(&spec, pairs).unwrap();
            let globals = Globals { output, ..Default::default() };
            let mut ctx = OpCtx::new(spec.name, vec![input.clone()], globals, &io, &NoProgress);
            pixelox_core::exec::run_spec(&spec, &a, &mut ctx)
        };
        let v = exec("retouch blemishes", &[], None).unwrap();
        assert!(v["count"].as_u64().unwrap() > 0);
        assert_eq!(v["op"], "retouch.blemishes");
        assert_eq!(v["params"]["min_size"], "3", "resolved defaults reach the recipe params");
        assert!(io.saved.lock().unwrap().is_none(), "nothing written");
        assert_eq!(exec("retouch blemishes", &[("heal", "true")], None).err().unwrap().code, "PX1004");
        // other mutating commands still need a destination
        assert_eq!(exec("retouch frequency-split", &[], None).err().unwrap().code, "PX1004");
        // with one, the healed document is written
        let out = td.path("clean.pxo").display().to_string();
        let v = exec("retouch blemishes", &[("heal", "true")], Some(out)).unwrap();
        assert!(v["healed_regions"].as_u64().unwrap() > 0);
        let saved = io.saved.lock().unwrap().clone().expect("saved");
        assert_ne!(raster(&saved, "Background"), raster(&io.doc, "Background"));
        assert_eq!(saved.recipe.last().unwrap().params["max_size"], "12");
    }

    #[test]
    fn results_do_not_depend_on_thread_count_or_replay() {
        let marks = grid_marks(&[Mark::Red, Mark::Brown, Mark::White]);
        let img = skin(320, 240, TONES[1], 13, &marks);
        let go = |threads: usize, pairs: &[(&str, &str)]| {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
            let mut doc = testing::doc_from(img.clone());
            let (v, ctx) = pool.install(|| run_to(pairs, &mut doc, true)).unwrap();
            (v, raster(&doc, "Background").clone(), ctx.resolved.clone())
        };
        let p = [("kind", "both"), ("heal", "true"), ("sensitivity", "60")];
        let (v1, i1, r1) = go(1, &p);
        let (v3, i3, _) = go(3, &p);
        assert_eq!(v1["spots"], v3["spots"]);
        assert!(
            i1.data.iter().zip(&i3.data).all(|(a, b)| a.to_bits() == b.to_bits()),
            "pixels differ across thread counts"
        );
        // a replay passes the recorded sizes explicitly and rebuilds the same result
        let (min, max) = (r1["min_size"].as_str().unwrap().to_string(), r1["max_size"].as_str().unwrap().to_string());
        let (v2, i2, _) = go(2, &[p[0], p[1], p[2], ("min-size", min.as_str()), ("max-size", max.as_str())]);
        assert_eq!(v1["spots"], v2["spots"]);
        assert_eq!(i1, i2);
    }

    #[test]
    fn edge_cases() {
        let spec = op();
        for img in
            [testing::solid(1, 1, Rgba::new(0.8, 0.2, 0.2, 1.0)), Image::new(4, 4), testing::solid(5, 300, Rgba::WHITE)]
        {
            let mut doc = testing::doc_from(img.clone());
            let (v, _) = run_to(&[("heal", "true")], &mut doc, true).unwrap();
            assert_eq!(v["count"], 0);
            assert_eq!(raster(&doc, "Background"), &img);
        }
        let img = skin(120, 100, TONES[1], 1, &[(60.0, 50.0, 5.0, Mark::Red)]);
        // invalid sizes
        let mut doc = testing::doc_from(img.clone());
        assert_eq!(
            testing::run(&spec, &[("min-size", "20"), ("max-size", "10")], &mut doc).err().unwrap().code,
            "PX1010"
        );
        assert_eq!(testing::run(&spec, &[("min-size", "0.5")], &mut doc).err().unwrap().code, "PX1013");
        // locked layers can be inspected but not healed
        doc.layers[0].locked = true;
        assert_eq!(testing::run(&spec, &[], &mut doc).unwrap()["count"], 1);
        assert_eq!(run_to(&[("heal", "true")], &mut doc, true).err().unwrap().code, "PX2304");
        // a layer offset on the canvas reports canvas coordinates
        let mut doc = Document::new(200, 150);
        let mut l = doc.new_layer("Photo", LayerKind::Raster { image: img.clone() });
        (l.x, l.y) = (50, 30);
        doc.insert_above(None, l);
        let found = spots(&testing::run(&spec, &[], &mut doc).unwrap());
        assert_eq!(found.len(), 1);
        assert!((found[0].0 - 110.0).abs() < 1.0 && (found[0].1 - 80.0).abs() < 1.0, "{found:?}");
        // non-pixel layers are searched but cannot be healed
        let mut doc = testing::doc_from(img.clone());
        let g = doc.new_layer("G", LayerKind::Group { children: vec![] });
        let p = doc.insert_above(None, g);
        let bg = doc.remove(&[0]);
        doc.layer_mut(&[p[0] - 1]).children_mut().unwrap().push(bg);
        assert_eq!(testing::run(&spec, &[("layer", "G")], &mut doc).unwrap()["count"], 1);
        assert_eq!(run_to(&[("layer", "G"), ("heal", "true")], &mut doc, true).err().unwrap().code, "PX2303");
    }

    /// The eval portrait of `skills/pixelox-retouch/evals/face.js` rebuilt in code at
    /// `scale`, with a skin mask like `select skin` gives (the face minus the features and
    /// a margin around them). Returns the image, the mask, the visible pimples, and the
    /// beauty mark.
    fn face(tone: [f32; 3], scale: f32) -> (Image, Mask, Marks, (f32, f32)) {
        let k = scale;
        let (w, h) = ((1200.0 * k) as u32, (1500.0 * k) as u32);
        let in_face = |x: f32, y: f32| {
            ((x - 600.0 * k) / (330.0 * k)).powi(2) + ((y - 720.0 * k) / (450.0 * k)).powi(2) <= 1.0
                || ((x - 600.0 * k).abs() < 150.0 * k && y > 1050.0 * k)
        };
        let ell = |x: f32, y: f32, cx: f32, cy: f32, rx: f32, ry: f32, rot: f32| {
            let (s, c) = rot.sin_cos();
            let (dx, dy) = (x - cx * k, y - cy * k);
            ((dx * c + dy * s) / (rx * k)).powi(2) + ((-dx * s + dy * c) / (ry * k)).powi(2) <= 1.0
        };
        let brow = |x, y| ell(x, y, 480.0, 560.0, 90.0, 18.0, -0.1) || ell(x, y, 720.0, 560.0, 90.0, 18.0, 0.1);
        let eye = |x, y| ell(x, y, 480.0, 640.0, 60.0, 26.0, 0.0) || ell(x, y, 720.0, 640.0, 60.0, 26.0, 0.0);
        let lips = |x, y| ell(x, y, 600.0, 1010.0, 110.0, 38.0, 0.0);
        let pimples =
            [(470.0, 820.0, 9.0), (720.0, 760.0, 7.0), (560.0, 960.0, 8.0), (690.0, 900.0, 6.0), (415.0, 720.0, 7.0)];
        let img = Image::from_fn(w, h, |x, y| {
            let (fx, fy) = (x as f32, y as f32);
            let mut c = if in_face(fx, fy) {
                let light =
                    1.08 - 0.35 * ((fx / k - 520.0) / 500.0).max(0.0) - 0.1 * ((fy / k - 900.0) / 600.0).max(0.0);
                let pore = (hash(x as i64, y as i64, 5) - 0.5) * 14.0;
                [0, 1, 2].map(|i| (tone[i] * light + pore * (1.0 - 0.15 * i as f32)).clamp(0.0, 255.0).round())
            } else if ell(fx, fy, 600.0, 620.0, 420.0, 560.0, 0.0) {
                [42.0, 29.0, 23.0]
            } else {
                [93.0, 107.0, 120.0]
            };
            for (px, py, r) in pimples {
                let d = ((fx - px * k).powi(2) + (fy - py * k).powi(2)).sqrt();
                let a = 0.75 * (1.0 - d / (1.6 * r * k)).max(0.0);
                for (i, pc) in [150.0, 40.0, 40.0].iter().enumerate() {
                    c[i] = c[i] * (1.0 - a) + pc * a;
                }
            }
            if ((fx - 760.0 * k).powi(2) + (fy - 1000.0 * k).powi(2)).sqrt() <= 6.0 * k {
                c = [0, 1, 2].map(|i| c[i] * 0.05 + [45.0, 25.0, 20.0][i] * 0.95);
            }
            if brow(fx, fy) {
                c = [42.0, 29.0, 23.0];
            }
            if eye(fx, fy) {
                c = [242.0, 239.0, 233.0];
            }
            for ex in [480.0, 720.0] {
                if ((fx - ex * k).powi(2) + (fy - 640.0 * k).powi(2)).sqrt() <= 22.0 * k {
                    c = [59.0, 42.0, 32.0];
                }
            }
            if lips(fx, fy) {
                c = [163.0, 64.0, 74.0];
            }
            [c[0].round() / 255.0, c[1].round() / 255.0, c[2].round() / 255.0, 1.0]
        });
        let margin = 4.0 * k;
        let mask = Mask::from_fn(w, h, |x, y| {
            let (fx, fy) = (x as f32, y as f32);
            let near = [(-margin, 0.0), (margin, 0.0), (0.0, -margin), (0.0, margin), (0.0, 0.0)]
                .iter()
                .any(|(dx, dy)| brow(fx + dx, fy + dy) || eye(fx + dx, fy + dy) || lips(fx + dx, fy + dy));
            if in_face(fx, fy) && !near {
                1.0
            } else {
                0.0
            }
        });
        let visible = pimples.iter().map(|&(x, y, r)| (x * k, y * k, 1.2 * r * k, Mark::Red)).collect();
        (img, mask, visible, (760.0 * k, 1000.0 * k))
    }

    #[test]
    fn finds_the_eval_portraits_pimples_on_the_skin_mask() {
        for tone in [TONES[0], TONES[2]] {
            let (img, mask, pimples, mole) = face(tone, 0.5);
            let mut doc = testing::doc_from(img);
            doc.put_mask("skin", mask, false).unwrap();
            let (v, _) = run_to(&[("mask", "skin")], &mut doc, false).unwrap();
            let found = spots(&v);
            let mut known = pimples.clone();
            known.push((mole.0, mole.1, 3.0, Mark::Brown));
            for s in &found {
                assert!(matched(&known, s).is_some(), "{tone:?}: unexpected spot {s:?}");
            }
            assert!(recall(&pimples, &found, &[Mark::Red]) >= 0.75, "{tone:?}: {found:?}");
            assert!(recall(&known, &found, &[Mark::Brown]) == 1.0, "{tone:?}: the beauty mark is a dark spot too");
            // pimples are red, the mole is not
            for s in v["spots"].as_array().unwrap() {
                let near_mole = (s["x"].as_f64().unwrap() - mole.0 as f64).abs() < 3.0;
                assert_eq!(s["redness"].as_f64().unwrap() > 10.0, !near_mole, "{s}");
            }
            // --min-redness keeps the pimples and drops the mole
            let (v, _) = run_to(&[("mask", "skin"), ("min-redness", "10")], &mut doc, false).unwrap();
            let kept = v["spots"].as_array().unwrap();
            assert!(!kept.is_empty());
            assert!(kept.iter().all(|s| (s["x"].as_f64().unwrap() - mole.0 as f64).abs() >= 3.0), "{tone:?}: {kept:?}");
            assert_eq!(v["settings"]["min_redness"], json!(10.0));
        }
    }
}
