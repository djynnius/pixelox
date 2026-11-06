//! `select sky`: a classical (no machine learning) sky selection, the counterpart of
//! Photoshop's Select › Sky for landscapes, skylines, and trees against the sky.
//!
//! Pipeline (O(pixels) throughout, deterministic, thread-count invariant):
//!
//! 1. Analyze a copy whose long side is at most [`WORK_LONG`] px, in OKLab: raw colors,
//!    a σ = 1 blurred copy, a texture map (the raw gradient, smoothed), and a
//!    direction-free contrast map. The top band's noise level raises every threshold so
//!    that noise never reads as edges or texture.
//! 2. Seeds: smooth, opaque pixels in the top band that match the band's bright mode (the
//!    reference sky color) or are clearly sky blue. None, or too dark → PX3320.
//! 3. Sky model: a conservative flood from the seeds through smooth continuations gives a
//!    core; each row's model is the median of the core's darker 40% (the clear sky rather
//!    than its clouds), interpolated across rows without core and smoothed vertically.
//! 4. Grow: a flood from the seeds into pixels close to their row's model under an
//!    asymmetric distance (brighter or paler, as clouds, haze, and sun glow are, is cheap;
//!    darker or another hue costs in full), stopped by texture and by hard edges: a step on
//!    the blurred or the raw colors, a step across three pixels (edges spread by
//!    anti-aliasing and downscaling), or a pixel on an edge (so the flood cannot walk along
//!    a nearly level edge). Steps that get brighter and paler together (cloud edges) are
//!    cheap here too; paler at the same lightness (haze, gray facades) is not.
//! 5. Clean-up: annex the sky pixels the barriers stopped short of an edge, up to the
//!    edge's middle; add tightly matching pockets that join the sky without an edge, and
//!    with `--holes` also those that do not (sky seen through branches); fill islands that
//!    do not reach the bottom edge when small (birds, specks) or mostly cloud-colored
//!    (clouds with crisp edges or cut by the top edge, smooth gray storm clouds).
//! 6. Refine: a fast guided filter (coefficients at the analysis size, applied with the
//!    full-size guide) on the image projected onto its sky-vs-ground color axis, so the
//!    soft edge follows mountain ridges and foliage at full resolution.

use crate::select::{apply, sample_source};
use crate::util::{feather_param, len_px, mask_json, mode_param, r3, select_mode};
use pixelox_core::kernels::{Edge, Filter};
use pixelox_core::prelude::*;
use rayon::prelude::*;

/// Long side of the analysis copy; larger images are analyzed downscaled and the guided
/// filter brings the mask back to full size.
const WORK_LONG: u32 = 1024;

// Tuning, in OKLab units (L 0..1; ΔE ≈ 0.02 is just noticeable), per analysis px.
/// Height of the band searched for seeds, as a fraction of the image height.
const TOP_BAND: f64 = 0.03;
/// Darkest reference sky (OKLab L; sRGB #1a1a1a is about 0.2) before giving up.
const MIN_SKY_L: f32 = 0.2;
/// Seeds: largest distance to the reference sky color.
const SEED_DIST: f32 = 0.15;
/// Core flood: largest step and texture (smooth continuations only).
const CORE_STEP: f32 = 0.02;
const CORE_TEXTURE: f32 = 0.02;
/// Highest noise level (median neighbor step) the thresholds adapt to; above it the top
/// band is more likely foliage than a noisy sky.
const MAX_NOISE: f32 = 0.03;
/// Against the sky model, brightening costs this fraction of a darkening (clouds, haze,
/// and sun glow are brighter than the clear sky; mountains, trees, and buildings darker).
const BRIGHT_WEIGHT: f32 = 0.15;
/// Between neighbors, getting brighter (and paler by up to as much) costs this fraction.
const STEP_BRIGHT_WEIGHT: f32 = 0.25;
/// A hard edge spread over three px (anti-aliasing, downscaling) is caught by the step
/// across them, compared with this multiple of the one-pixel limit.
const WIDE_STEP: f32 = 1.25;
/// Desaturating toward white or gray costs this fraction of a hue change.
const PALE_WEIGHT: f32 = 0.4;
/// Most the per-row spread of the sky can widen the tolerance.
const MAX_SPREAD: f32 = 0.05;
/// Edge pixels the barriers left out are annexed up to this many analysis px.
const ANNEX: usize = 3;
/// Regularization of the guided filter (the guide is normalized to sky-ground contrast 1).
const GUIDED_EPS: f32 = 0.01;

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("select sky", Category::Select, "Select the sky (classical, no AI)", sky)
        .about("Select the sky without machine learning: seeds along the top edge, a per-row sky color model, and region growing that crosses soft cloud edges but stops at hard edges (mountains, trees, buildings); a guided filter then snaps the soft edge to the full-size image (images are analyzed at up to 1024 px). Clouds, haze, and sun glow are included; sky enclosed by foreground (seen between branches) only with --holes. Samples the composite unless --layer is given. Fails with PX3320 when the top edge does not look like sky. Besides the selection, the result gives `sky` (coverage and bounds of the sky alone), `horizon` (canvas rows where the sky ends: median y, min, max), `confidence` (0..1), and `advice` when something looks off: below 0.5, check the selection and adjust --tolerance, or use select color --contiguous / select luma.")
        .param(param("tolerance", ParamKind::Int).default("50").range("0..100").help("How far a pixel may stray from the sky's color model, 0..100: raise for hazy or stormy skies, lower when the selection leaks into the ground"))
        .param(param("refine", ParamKind::Length).auto().default("auto").help("Edge refinement radius in px (a guided filter snaps the soft edge to the image); auto scales with the image size, 0 = off"))
        .param(param("holes", ParamKind::Flag).help("Also select sky-colored areas not connected to the top edge (sky between branches); may pick up sky-colored objects"))
        .param(param("layer", ParamKind::Layer).help("Sample this layer instead of the composite"))
        .param(feather_param())
        .param(mode_param())
        .example("pixelox select sky photo.pxo --in-place", "Select the sky")
        .example("pixelox select sky photo.pxo --holes --tolerance 60 --in-place", "Include sky seen through branches, and hazier sky")
        .example("pixelox select sky photo.pxo --refine 0 --feather 3 --mode add --in-place", "Add the sky to the selection with a plain soft edge")
        .example("pixelox select sky photo.jpg -o sky.pxo --dry-run --json", "Plan it and see the coverage, horizon, and confidence")]
}

fn sky(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let mode = select_mode(a)?;
    let feather = len_px(a, "feather", doc)?;
    let refine = if a.is_auto("refine") { None } else { Some(len_px(a, "refine", doc)?) };
    let opts = SkyOptions { tolerance: a.int("tolerance")? as f32, refine, holes: a.flag("holes") };
    let src = sample_source(doc, a.raw("layer"))?;
    let det = detect(&src, &opts, WORK_LONG).map_err(|e| match a.raw("layer") {
        Some(l) => e.with_context("layer", l),
        None => e,
    })?;
    if refine.is_none() {
        // full precision: a replay must rebuild exactly the same radius
        ctx.resolve_param("refine", det.refine_px.to_string());
    }
    let sky_stats = mask_json(&det.mask);
    let mut v = apply(doc, ctx, det.mask, mode, feather);
    if let (Value::Object(out), Value::Object(info)) = (&mut v, det.info) {
        out.extend(info);
    }
    v["sky"] = sky_stats;
    Ok(v)
}

// ---------------------------------------------------------------------------------------
// detection

struct SkyOptions {
    /// 0..100.
    tolerance: f32,
    /// Refinement radius in full-size px; `None` = auto.
    refine: Option<f64>,
    holes: bool,
}

struct Detection {
    /// Soft sky mask, the size of the source.
    mask: Mask,
    refine_px: f64,
    /// Result fields (`sky_color`, `horizon`, `confidence`, ...).
    info: Value,
}

/// Thresholds: from `--tolerance`, raised to clear the image's noise.
struct Limits {
    /// Largest sky distance to the row model.
    tol: f32,
    /// Largest step between neighbors on raw colors (hard edges).
    raw_step: f32,
    /// Largest step between neighbors on blurred colors.
    blur_step: f32,
    /// Largest texture for a pixel that only just matches the model.
    texture: f32,
    /// Largest texture of a seed.
    seed_texture: f32,
    /// Core flood: largest blurred step and texture.
    core_step: f32,
    core_texture: f32,
}

impl Limits {
    /// `noise`: the median raw step between neighbors in a smooth sky.
    fn new(tolerance: f32, noise: f32) -> Limits {
        let tol = 0.03 + 0.0018 * tolerance.clamp(0.0, 100.0);
        let n = noise.min(MAX_NOISE);
        Limits {
            tol,
            raw_step: (0.3 * tol).max(3.0 * n),
            blur_step: (0.3 * tol).max(0.6 * n),
            texture: (0.4 * tol).max(1.5 * n),
            seed_texture: (0.2 * tol + 0.005).max(1.5 * n),
            core_step: CORE_STEP.max(0.6 * n),
            core_texture: CORE_TEXTURE.max(1.5 * n),
        }
    }
}

pub(crate) type Lab = [f32; 3];

/// Find the sky in `src` (canvas-sized), analyzing at most `work_long` px on the long side.
fn detect(src: &Image, o: &SkyOptions, work_long: u32) -> Result<Detection> {
    let (fw, fh) = (src.width, src.height);
    if fw == 0 || fh == 0 {
        return Err(no_sky("the image is empty"));
    }
    let long = fw.max(fh);
    let (ww, wh) = if long > work_long.max(1) {
        let s = work_long.max(1) as f64 / long as f64;
        (((fw as f64 * s).round() as u32).max(1), ((fh as f64 * s).round() as u32).max(1))
    } else {
        (fw, fh)
    };
    let scaled = (ww, wh) != (fw, fh);
    let resized;
    let small = if scaled {
        resized = kernels::resize(src, ww, wh, Filter::Bilinear);
        &resized
    } else {
        src
    };
    let (w, h) = (ww as usize, wh as usize);

    // 1. analysis planes and thresholds
    let raw = to_oklab(small);
    let alpha: Vec<f32> = small.data.par_chunks(4).map(|p| p[3]).collect();
    let blurred = to_oklab(&kernels::gaussian_blur(small, 1.0, Edge::Clamp));
    let mut texture = gradient(&raw, w, h);
    kernels::gaussian_planar(&mut texture, w, h, 1, 1.5, 1.5, Edge::Clamp);
    let contrast = contrast_map(&raw, w, h);
    let band = ((h as f64 * TOP_BAND).round() as usize).clamp(1, h);
    let t = Limits::new(o.tolerance, noise_level(&raw, &alpha, w, band));

    // 2. seeds
    let seeds = find_seeds(&blurred, &alpha, &texture, w, band, &t)?;

    // 3. sky model from a conservative core
    let core = flood(&seeds.idx, w, h, |p, q| {
        alpha[q] >= 0.5
            && texture[q] < t.core_texture
            && blurred[q][0] >= seeds.floor
            && dist(blurred[p], blurred[q]) < t.core_step
            && !crosses_edge(&raw, &contrast, p, q, w, t.raw_step, true)
    });
    let models = row_models(&core, &blurred, w, h, seeds.color);
    drop(core);
    let tol_row: Vec<f32> = models.iter().map(|m| t.tol + m.spread.min(MAX_SPREAD)).collect();

    // 4. grow
    let d_blur = sky_dists(&blurred, &models, w);
    let mut sky = flood(&seeds.idx, w, h, |p, q| {
        let tq = tol_row[q / w];
        let d = d_blur[q];
        alpha[q] >= 0.5
            && d <= tq
            && texture[q] < t.texture * (1.0 + 1.5 * (1.0 - d / tq))
            && step_dist(blurred[p], blurred[q]) < t.blur_step
            && !crosses_edge(&raw, &contrast, p, q, w, t.raw_step, true)
    });
    drop(d_blur);

    // 5. clean-up and holes
    let d_raw = sky_dists(&raw, &models, w);
    let px = Pixels {
        raw: &raw,
        contrast: &contrast,
        texture: &texture,
        models: &models,
        d: &d_raw,
        tol_row: &tol_row,
        alpha: &alpha,
        w,
        h,
        edge_limit: t.raw_step,
        smooth_limit: 2.0 * t.texture,
    };
    px.annex(&mut sky);
    let (pockets, holes) = px.tight_regions(&mut sky, o.holes);
    if pockets + holes > 0 {
        px.annex(&mut sky);
    }
    let (specks, clouds) = px.fill_islands(&mut sky);
    let left_out = px.left_out(&sky);

    // 6. refine
    let axis = Axis::measure(&sky, &raw, &alpha, w, h);
    let k = (ww as f64 / fw as f64 + wh as f64 / fh as f64) / 2.0;
    let refine_px = o.refine.unwrap_or(3.0 / k);
    let p: Vec<f32> = sky.iter().map(|&s| if s { 1.0 } else { 0.0 }).collect();
    let guide_w: Vec<f32> = raw.par_iter().map(|&c| axis.guide(c)).collect();
    let (ca, cb) =
        if refine_px > 0.0 { guided_coeffs(&guide_w, &p, w, h, refine_px * k / 2.0) } else { (vec![0.0; w * h], p) };
    let mask = upsample(src, &ca, &cb, w, h, |x, y, px| {
        if refine_px <= 0.0 {
            0.0
        } else if scaled {
            axis.guide(oklab(px))
        } else {
            guide_w[y * w + x]
        }
    });

    let info = describe(&sky, w, h, fh, &seeds, &axis, left_out, refine_px, o, [holes, specks, clouds]);
    Ok(Detection { mask, refine_px, info })
}

fn no_sky(reason: &str) -> Error {
    Error::new("PX3320", format!("no sky found: {reason}")).with_hint(
        "if some sky is visible, magic-wand it: pixelox select color photo.pxo --at 100,20 --contiguous --tolerance 30 --in-place; or select the bright areas: pixelox select luma photo.pxo --range 0.6..1 --in-place",
    )
}

// ---------------------------------------------------------------------------------------
// color

#[inline]
pub(crate) fn oklab(p: &[f32]) -> Lab {
    let (l, a, b) = color::rgb_to_oklab(p[0].clamp(0.0, 1.0), p[1].clamp(0.0, 1.0), p[2].clamp(0.0, 1.0));
    [l, a, b]
}

pub(crate) fn to_oklab(img: &Image) -> Vec<Lab> {
    img.data.par_chunks(4).map(oklab).collect()
}

pub(crate) fn hex(c: Lab) -> String {
    let (r, g, b) = color::oklab_to_rgb(c[0], c[1], c[2]);
    Rgba::new(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), 1.0).to_hex()
}

#[inline]
fn dist(p: Lab, q: Lab) -> f32 {
    ((p[0] - q[0]).powi(2) + (p[1] - q[1]).powi(2) + (p[2] - q[2]).powi(2)).sqrt()
}

/// Step between neighbors. Getting brighter, and paler by up to as much, is how a cloud
/// or haze edge looks against the sky and costs [`STEP_BRIGHT_WEIGHT`]; darker, paler at
/// the same lightness (gray facades, distant haze), or another hue cost in full.
#[inline]
fn step_dist(from: Lab, to: Lab) -> f32 {
    let dl = to[0] - from[0];
    let (da, db) = (to[1] - from[1], to[2] - from[2]);
    let radial = (to[1] * to[1] + to[2] * to[2]).sqrt() - (from[1] * from[1] + from[2] * from[2]).sqrt();
    let hue2 = (da * da + db * db - radial * radial).max(0.0);
    let (l, r) = if dl > 0.0 {
        let pale = (-radial).clamp(0.0, dl);
        (STEP_BRIGHT_WEIGHT * dl, STEP_BRIGHT_WEIGHT * pale + (radial.abs() - pale))
    } else {
        (dl, radial)
    };
    (l * l + r * r + hue2).sqrt()
}

/// The pixel after `b` on the line from `a` through `b` (4-neighbors), if inside.
#[inline]
fn beyond(a: usize, b: usize, w: usize, n: usize) -> Option<usize> {
    let c = (2 * b).checked_sub(a)?;
    let horizontal = a / w == b / w;
    (c < n && (!horizontal || c / w == b / w)).then_some(c)
}

/// Direction-free contrast at each pixel: between its left and right neighbors and between
/// its upper and lower ones (each the cheaper way round, so cloud edges stay cheap),
/// combined. A walk along a nearly level edge changes little from pixel to pixel; this
/// still sees the edge across it.
fn contrast_map(raw: &[Lab], w: usize, h: usize) -> Vec<f32> {
    let two = |a: Lab, b: Lab| step_dist(a, b).min(step_dist(b, a));
    let mut e = vec![0f32; w * h];
    e.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let (y0, y1) = (y.saturating_sub(1), (y + 1).min(h - 1));
        for (x, o) in row.iter_mut().enumerate() {
            let (x0, x1) = (x.saturating_sub(1), (x + 1).min(w - 1));
            let ex = two(raw[y * w + x0], raw[y * w + x1]);
            let ey = two(raw[y0 * w + x], raw[y1 * w + x]);
            *o = (ex * ex + ey * ey).sqrt();
        }
    });
    e
}

/// Whether the move `p → q` meets a hard edge on `raw`: one step over the limit, or an
/// edge spread from the pixel before `p` to the one after `q` (a step over [`WIDE_STEP`]
/// times the limit; anti-aliasing and downscaling spread edges over 2–3 px). `strict`
/// refuses any move with such an edge in its span, even toward the last pixel before it or
/// away from it, and any `q` on an edge (`contrast` over the wide limit), so a flood can
/// neither creep along a jagged edge through mixed pixels nor walk along a level one.
/// Lenient refuses only a move from the near side of the edge to a `q` past its middle.
#[inline]
fn crosses_edge(raw: &[Lab], contrast: &[f32], p: usize, q: usize, w: usize, limit: f32, strict: bool) -> bool {
    if step_dist(raw[p], raw[q]) >= limit || (strict && contrast[q] >= WIDE_STEP * limit) {
        return true;
    }
    let (Some(back), Some(ahead)) = (beyond(q, p, w, raw.len()), beyond(p, q, w, raw.len())) else {
        return false;
    };
    let (b, a) = (raw[back], raw[ahead]);
    step_dist(b, a) >= WIDE_STEP * limit
        && (strict || (dist(b, raw[p]) < dist(raw[p], a) && dist(b, raw[q]) >= dist(raw[q], a)))
}

/// Distance of a color from the sky model `m`: brighter (clouds, haze) and paler (toward
/// white or gray along the sky's own hue) are cheap; darker, more saturated, or another hue
/// cost in full.
#[inline]
fn sky_dist(c: Lab, m: Lab) -> f32 {
    let dl = c[0] - m[0];
    let dl = if dl > 0.0 { BRIGHT_WEIGHT * dl } else { dl };
    let cm = (m[1] * m[1] + m[2] * m[2]).sqrt();
    let chroma2 = if cm > 0.02 {
        let (ux, uy) = (m[1] / cm, m[2] / cm);
        let par = c[1] * ux + c[2] * uy;
        let perp = c[1] * uy - c[2] * ux;
        let dpar = if par >= cm { par - cm } else { PALE_WEIGHT * (cm - par.max(0.0)) + (-par).max(0.0) };
        dpar * dpar + perp * perp
    } else {
        (c[1] - m[1]).powi(2) + (c[2] - m[2]).powi(2)
    };
    (dl * dl + chroma2).sqrt()
}

/// Per-channel median (`px` not empty).
pub(crate) fn median3(px: &[Lab]) -> Lab {
    let mut out = [0f32; 3];
    let mut v: Vec<f32> = Vec::with_capacity(px.len());
    for (ch, o) in out.iter_mut().enumerate() {
        v.clear();
        v.extend(px.iter().map(|c| c[ch]));
        let k = v.len() / 2;
        *o = *v.select_nth_unstable_by(k, f32::total_cmp).1;
    }
    out
}

/// The `q`-quantile (0..1) of `v` (not empty); reorders `v`.
pub(crate) fn quantile(v: &mut [f32], q: f64) -> f32 {
    let k = ((v.len() as f64 * q) as usize).min(v.len() - 1);
    *v.select_nth_unstable_by(k, f32::total_cmp).1
}

/// Gradient magnitude (central differences, OKLab ΔE per px).
fn gradient(c: &[Lab], w: usize, h: usize) -> Vec<f32> {
    let mut g = vec![0f32; w * h];
    g.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let (y0, y1) = (y.saturating_sub(1), (y + 1).min(h - 1));
        let dy = (y1 - y0).max(1) as f32;
        for (x, o) in row.iter_mut().enumerate() {
            let (x0, x1) = (x.saturating_sub(1), (x + 1).min(w - 1));
            let dx = (x1 - x0).max(1) as f32;
            let gx = dist(c[y * w + x1], c[y * w + x0]) / dx;
            let gy = dist(c[y1 * w + x], c[y0 * w + x]) / dy;
            *o = (gx * gx + gy * gy).sqrt();
        }
    });
    g
}

// ---------------------------------------------------------------------------------------
// regions

#[inline]
pub(crate) fn neighbors(p: usize, w: usize, h: usize) -> impl Iterator<Item = usize> {
    let (x, y) = (p % w, p / w);
    [(x > 0).then(|| p - 1), (x + 1 < w).then(|| p + 1), (y > 0).then(|| p - w), (y + 1 < h).then(|| p + w)]
        .into_iter()
        .flatten()
}

/// Pixels reachable from `seeds` through 4-neighbor moves `p → q` that `accept`. A pixel is
/// marked only when accepted, so the result is the reachability closure, independent of
/// visiting order.
pub(crate) fn flood(seeds: &[usize], w: usize, h: usize, accept: impl Fn(usize, usize) -> bool) -> Vec<bool> {
    let mut on = vec![false; w * h];
    let mut stack = Vec::with_capacity(seeds.len());
    for &s in seeds {
        if !on[s] {
            on[s] = true;
            stack.push(s);
        }
    }
    while let Some(p) = stack.pop() {
        for q in neighbors(p, w, h) {
            if !on[q] && accept(p, q) {
                on[q] = true;
                stack.push(q);
            }
        }
    }
    on
}

/// Call `f(pixels, touches_border)` for every component of `on` pixels joined by 4-neighbor
/// moves that `link`, in scan order.
pub(crate) fn for_each_component(
    on: &[bool],
    w: usize,
    h: usize,
    link: impl Fn(usize, usize) -> bool,
    mut f: impl FnMut(&[usize], bool),
) {
    let mut seen = vec![false; on.len()];
    let mut stack = Vec::new();
    let mut comp = Vec::new();
    for s in 0..on.len() {
        if !on[s] || seen[s] {
            continue;
        }
        comp.clear();
        seen[s] = true;
        stack.push(s);
        let mut border = false;
        while let Some(p) = stack.pop() {
            comp.push(p);
            let (x, y) = (p % w, p / w);
            border |= x == 0 || y == 0 || x + 1 == w || y + 1 == h;
            for q in neighbors(p, w, h) {
                if on[q] && !seen[q] && link(p, q) {
                    seen[q] = true;
                    stack.push(q);
                }
            }
        }
        f(&comp, border);
    }
}

struct Seeds {
    idx: Vec<usize>,
    /// Reference sky color (the top band's bright mode).
    color: Lab,
    /// Darkest lightness the core may reach.
    floor: f32,
    /// Seeds per top-band pixel.
    top_fraction: f64,
}

fn find_seeds(c: &[Lab], alpha: &[f32], texture: &[f32], w: usize, band: usize, t: &Limits) -> Result<Seeds> {
    let total = band * w;
    let cand: Vec<usize> = (0..total).filter(|&i| alpha[i] >= 0.5 && texture[i] < t.seed_texture).collect();
    let smooth = r3(cand.len() as f64 / total as f64);
    if cand.is_empty() {
        return Err(no_sky("the top edge is textured or transparent, not sky-like")
            .with_context("top_rows", band)
            .with_context("smooth_fraction", smooth));
    }
    // the bright mode of the smooth pixels: sky and clouds rather than dark smooth objects
    let mut ls: Vec<f32> = cand.iter().map(|&i| c[i][0]).collect();
    let l80 = quantile(&mut ls, 0.8);
    let bright: Vec<Lab> = cand.iter().map(|&i| c[i]).filter(|p| p[0] >= l80 - 0.08).collect();
    let c0 = median3(&bright);
    if c0[0] < MIN_SKY_L {
        return Err(no_sky("the top edge is too dark to be sky")
            .with_context("top_rows", band)
            .with_context("top_color", hex(c0)));
    }
    // seeds: near the reference, or clearly sky blue (when clouds set the reference)
    let idx: Vec<usize> = cand
        .into_iter()
        .filter(|&i| {
            let p = c[i];
            dist(p, c0) < SEED_DIST || (p[0] >= MIN_SKY_L && p[2] < -0.03)
        })
        .collect();
    let top_fraction = idx.len() as f64 / total as f64;
    if idx.len() < ((total as f64 * 0.02).ceil() as usize).max(1) {
        return Err(no_sky("too little of the top edge looks like sky")
            .with_context("top_rows", band)
            .with_context("smooth_fraction", smooth)
            .with_context("sky_fraction", r3(top_fraction))
            .with_context("top_color", hex(c0)));
    }
    let mut ls: Vec<f32> = idx.iter().map(|&i| c[i][0]).collect();
    let floor = quantile(&mut ls, 0.1) - 0.25;
    Ok(Seeds { idx, color: c0, floor, top_fraction })
}

/// Noise level: the median raw step between neighbors that a smooth sky would show,
/// estimated from the top band's lower quartile of horizontal steps (foliage or edges in
/// part of the band do not inflate it).
fn noise_level(raw: &[Lab], alpha: &[f32], w: usize, band: usize) -> f32 {
    let mut steps: Vec<f32> = (0..band * w)
        .filter(|&i| i % w + 1 < w && alpha[i] >= 0.5 && alpha[i + 1] >= 0.5)
        .map(|i| dist(raw[i], raw[i + 1]))
        .collect();
    if steps.len() < 8 {
        return 0.0;
    }
    // for noise, the lower quartile of the step length is about 0.6 of its median
    quantile(&mut steps, 0.25) / 0.6
}

#[derive(Clone, Copy, Debug)]
struct RowModel {
    c: Lab,
    /// Median sky distance of the row's core pixels from `c`.
    spread: f32,
}

fn lerp_model(a: RowModel, b: RowModel, f: f32) -> RowModel {
    let l = |x: f32, y: f32| x + (y - x) * f;
    RowModel { c: [l(a.c[0], b.c[0]), l(a.c[1], b.c[1]), l(a.c[2], b.c[2])], spread: l(a.spread, b.spread) }
}

/// Per-row sky model from the core: the median of each row's darker 40% (clear sky rather
/// than clouds), interpolated across rows without enough core, then smoothed vertically.
fn row_models(core: &[bool], col: &[Lab], w: usize, h: usize, fallback: Lab) -> Vec<RowModel> {
    let min_n = (w / 50).max(2);
    let rows: Vec<Option<RowModel>> = (0..h)
        .into_par_iter()
        .map(|y| {
            let px: Vec<Lab> = (y * w..(y + 1) * w).filter(|&i| core[i]).map(|i| col[i]).collect();
            if px.len() < min_n {
                return None;
            }
            let mut ls: Vec<f32> = px.iter().map(|p| p[0]).collect();
            let lq = quantile(&mut ls, 0.4);
            let darker: Vec<Lab> = px.iter().copied().filter(|p| p[0] <= lq).collect();
            let c = median3(&darker);
            let mut d: Vec<f32> = px.iter().map(|&p| sky_dist(p, c)).collect();
            Some(RowModel { c, spread: quantile(&mut d, 0.5) })
        })
        .collect();
    let valid: Vec<usize> = (0..h).filter(|&y| rows[y].is_some()).collect();
    if valid.is_empty() {
        return vec![RowModel { c: fallback, spread: 0.0 }; h];
    }
    let at = |y: usize| rows[y].expect("valid row");
    let mut filled = Vec::with_capacity(h);
    let mut j = 0;
    for y in 0..h {
        while j < valid.len() && valid[j] < y {
            j += 1;
        }
        filled.push(if j < valid.len() && valid[j] == y {
            at(y)
        } else if j == 0 {
            at(valid[0])
        } else if j == valid.len() {
            at(valid[j - 1])
        } else {
            let (y0, y1) = (valid[j - 1], valid[j]);
            lerp_model(at(y0), at(y1), (y - y0) as f32 / (y1 - y0) as f32)
        });
    }
    let r = (h / 64).max(1);
    (0..h)
        .map(|y| {
            let (lo, hi) = (y.saturating_sub(r), (y + r).min(h - 1));
            let n = (hi - lo + 1) as f32;
            let mut m = RowModel { c: [0.0; 3], spread: 0.0 };
            for f in &filled[lo..=hi] {
                for ch in 0..3 {
                    m.c[ch] += f.c[ch] / n;
                }
                m.spread += f.spread / n;
            }
            m
        })
        .collect()
}

fn sky_dists(c: &[Lab], models: &[RowModel], w: usize) -> Vec<f32> {
    let mut d = vec![0f32; c.len()];
    d.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let m = models[y].c;
        for (o, &p) in row.iter_mut().zip(&c[y * w..(y + 1) * w]) {
            *o = sky_dist(p, m);
        }
    });
    d
}

/// What the clean-up steps read: raw colors, their sky distances, and thresholds.
struct Pixels<'a> {
    raw: &'a [Lab],
    contrast: &'a [f32],
    texture: &'a [f32],
    models: &'a [RowModel],
    /// Sky distance of the raw colors.
    d: &'a [f32],
    tol_row: &'a [f32],
    alpha: &'a [f32],
    w: usize,
    h: usize,
    /// Raw step limit of [`crosses_edge`].
    edge_limit: f32,
    /// Texture of a smooth (cloud) area.
    smooth_limit: f32,
}

impl Pixels<'_> {
    /// A move from sky pixel `p` to `q` that meets no hard edge (see [`crosses_edge`]).
    #[inline]
    fn joins(&self, sky: &[bool], p: usize, q: usize, strict: bool) -> bool {
        sky[p] && !crosses_edge(self.raw, self.contrast, p, q, self.w, self.edge_limit, strict)
    }

    /// Grow the sky into model-matching neighbors: up to [`ANNEX`] px with the strict edge
    /// test (the blurred barriers stop a pixel or two short of a hard edge), then one more
    /// px up to the middle of the edge.
    fn annex(&self, sky: &mut [bool]) {
        let (w, h) = (self.w, self.h);
        for round in 0..=ANNEX {
            let strict = round < ANNEX;
            let cur: &[bool] = sky;
            let add: Vec<usize> = (0..w * h)
                .into_par_iter()
                .filter(|&q| {
                    !cur[q]
                        && self.alpha[q] >= 0.5
                        && self.d[q] <= self.tol_row[q / w]
                        && neighbors(q, w, h).any(|p| self.joins(cur, p, q, strict))
                })
                .collect();
            for q in add {
                sky[q] = true;
            }
        }
    }

    /// Matches the sky model closely: within 0.4·tol of it (no discount for paler colors:
    /// gray facades and haze are pale), or clearly brighter (cloud) and within 0.6·tol by
    /// the sky distance.
    fn tight(&self, i: usize) -> bool {
        let (c, m) = (self.raw[i], self.models[i / self.w].c);
        let tol = self.tol_row[i / self.w];
        self.alpha[i] >= 0.5 && (dist(c, m) <= 0.4 * tol || (c[0] >= m[0] + 0.1 && self.d[i] <= 0.6 * tol))
    }

    /// Components of tightly matching pixels outside the sky, split at hard edges: those
    /// joining the sky without an edge are pockets the barriers cut off (always added); the
    /// others are sky seen through the foreground, added with `holes` when not specks, the
    /// sky's own color on average (not haze or a white facade), and not reaching the bottom
    /// edge (water reflecting the sky). Returns the counts.
    fn tight_regions(&self, sky: &mut [bool], holes: bool) -> (usize, usize) {
        let (w, h) = (self.w, self.h);
        let min_px = (w * h / 100_000).max(3);
        let cand: Vec<bool> = (0..w * h).map(|i| !sky[i] && self.tight(i)).collect();
        let mut add = Vec::new();
        let (mut pockets, mut regions) = (0, 0);
        let cur: &[bool] = sky;
        let link = |p: usize, q: usize| !crosses_edge(self.raw, self.contrast, p, q, w, self.edge_limit, false);
        for_each_component(&cand, w, h, link, |comp, _| {
            let touches = comp.iter().any(|&q| neighbors(q, w, h).any(|p| self.joins(cur, p, q, false)));
            if touches {
                pockets += 1;
            } else if holes && comp.len() >= min_px && !comp.iter().any(|&i| i / w == h - 1) && {
                let off: f64 =
                    comp.iter().map(|&i| (dist(self.raw[i], self.models[i / w].c) / self.tol_row[i / w]) as f64).sum();
                off / comp.len() as f64 <= 0.3
            } {
                regions += 1;
            } else {
                return;
            }
            add.extend_from_slice(comp);
        });
        for i in add {
            sky[i] = true;
        }
        (pockets, regions)
    }

    /// Cloud-colored: within tolerance of the sky model, or a smooth gray that is not
    /// near-black (storm clouds are darker than the sky they float in).
    fn cloudy(&self, i: usize) -> bool {
        let c = self.raw[i];
        let m = self.models[i / self.w].c;
        let gray = (c[1] * c[1] + c[2] * c[2]).sqrt() <= (0.5 * (m[1] * m[1] + m[2] * m[2]).sqrt()).max(0.04);
        self.alpha[i] >= 0.5
            && (self.d[i] <= self.tol_row[i / self.w]
                || (gray && c[0] >= MIN_SKY_L + 0.1 && self.texture[i] < self.smooth_limit))
    }

    /// Sky-colored area left out of the sky, per selected sky pixel: sky behind foreground
    /// (`--holes` adds it) or between dark storm clouds (a higher tolerance reaches it).
    fn left_out(&self, sky: &[bool]) -> f64 {
        // integer counts: the result does not depend on how the work is split
        let on = sky.par_iter().filter(|&&s| s).count();
        let off = (0..sky.len()).into_par_iter().filter(|&i| !sky[i] && self.tight(i)).count();
        off as f64 / on.max(1) as f64
    }

    /// Fill non-sky islands that do not reach the bottom edge (the ground does): small ones
    /// (birds, specks) and mostly cloud-colored ones (clouds the edge barriers cut off).
    /// Returns (specks, clouds).
    fn fill_islands(&self, sky: &mut [bool]) -> (usize, usize) {
        let (w, h) = (self.w, self.h);
        let speck = (w * h / 2000).max(4);
        let off: Vec<bool> = sky.iter().map(|&s| !s).collect();
        let mut add = Vec::new();
        let (mut specks, mut clouds) = (0, 0);
        for_each_component(
            &off,
            w,
            h,
            |_, _| true,
            |comp, _| {
                if comp.iter().any(|&i| i / w == h - 1) {
                    return;
                }
                if comp.len() <= speck {
                    specks += 1;
                } else {
                    let cloudy = comp.iter().filter(|&&i| self.cloudy(i)).count();
                    if (cloudy as f64) < 0.6 * comp.len() as f64 {
                        return;
                    }
                    clouds += 1;
                }
                add.extend_from_slice(comp);
            },
        );
        for i in add {
            sky[i] = true;
        }
        (specks, clouds)
    }
}

// ---------------------------------------------------------------------------------------
// refinement

/// The sky-vs-ground color axis along the boundary: the guide for the guided filter is the
/// image projected on it, scaled so the two sides are about 1 apart.
struct Axis {
    origin: Lab,
    dir: Lab,
    gain: f32,
    /// OKLab distance between the mean colors just inside and just outside the sky.
    contrast: f32,
    /// Whether the sky has an edge inside the canvas.
    boundary: bool,
}

impl Axis {
    fn measure(sky: &[bool], raw: &[Lab], alpha: &[f32], w: usize, h: usize) -> Axis {
        // bands 1..3 px inside and outside the edge (pure colors, not the mixed edge pixels);
        // 0..2 px when the sky or the ground is too thin for that
        let m =
            Mask { width: w as u32, height: h as u32, data: sky.iter().map(|&s| if s { 1.0 } else { 0.0 }).collect() };
        let (in1, in2, in3) = (kernels::erode(&m, 1.0), kernels::erode(&m, 2.0), kernels::erode(&m, 3.0));
        let (out1, out2, out3) = (kernels::dilate(&m, 1.0), kernels::dilate(&m, 2.0), kernels::dilate(&m, 3.0));
        let on = |v: &Mask, i: usize| v.data[i] > 0.5;
        // per-row partial sums, added in row order (thread-count invariant)
        let rows: Vec<[f64; 16]> = (0..h)
            .into_par_iter()
            .map(|y| {
                // [pure in (3), n, pure out (3), n, near in (3), n, near out (3), n]
                let mut acc = [0f64; 16];
                let mut add = |slot: usize, c: Lab| {
                    (0..3).for_each(|k| acc[slot + k] += c[k] as f64);
                    acc[slot + 3] += 1.0;
                };
                for i in y * w..(y + 1) * w {
                    if alpha[i] < 0.5 {
                        continue;
                    }
                    if sky[i] {
                        if on(&in1, i) && !on(&in3, i) {
                            add(0, raw[i]);
                        }
                        if !on(&in2, i) {
                            add(8, raw[i]);
                        }
                    } else {
                        if !on(&out1, i) && on(&out3, i) {
                            add(4, raw[i]);
                        }
                        if on(&out2, i) {
                            add(12, raw[i]);
                        }
                    }
                }
                acc
            })
            .collect();
        let mut acc = [0f64; 16];
        for r in rows {
            for (a, v) in acc.iter_mut().zip(r) {
                *a += v;
            }
        }
        let pure = acc[3] > 0.0 && acc[7] > 0.0;
        let (si, ni, so, no) = if pure {
            ([acc[0], acc[1], acc[2]], acc[3], [acc[4], acc[5], acc[6]], acc[7])
        } else {
            ([acc[8], acc[9], acc[10]], acc[11], [acc[12], acc[13], acc[14]], acc[15])
        };
        if ni == 0.0 || no == 0.0 {
            return Axis { origin: [0.0; 3], dir: [1.0, 0.0, 0.0], gain: 1.0 / 0.3, contrast: 0.0, boundary: false };
        }
        let mi = si.map(|v| (v / ni) as f32);
        let mo = so.map(|v| (v / no) as f32);
        let contrast = dist(mi, mo);
        if contrast < 0.02 {
            return Axis { origin: mo, dir: [1.0, 0.0, 0.0], gain: 1.0 / 0.3, contrast, boundary: true };
        }
        let dir = [(mi[0] - mo[0]) / contrast, (mi[1] - mo[1]) / contrast, (mi[2] - mo[2]) / contrast];
        Axis { origin: mo, dir, gain: 1.0 / contrast.max(0.05), contrast, boundary: true }
    }

    #[inline]
    fn guide(&self, c: Lab) -> f32 {
        ((c[0] - self.origin[0]) * self.dir[0]
            + (c[1] - self.origin[1]) * self.dir[1]
            + (c[2] - self.origin[2]) * self.dir[2])
            * self.gain
    }
}

/// Guided filter coefficients (He et al.) with Gaussian windows of `sigma`, already
/// averaged: the output is `a·I + b` for the guide `I` at any resolution.
pub(crate) fn guided_coeffs(i: &[f32], p: &[f32], w: usize, h: usize, sigma: f64) -> (Vec<f32>, Vec<f32>) {
    let blur = |mut v: Vec<f32>| {
        kernels::gaussian_planar(&mut v, w, h, 1, sigma, sigma, Edge::Clamp);
        v
    };
    let mi = blur(i.to_vec());
    let mp = blur(p.to_vec());
    let mii = blur(i.par_iter().map(|v| v * v).collect());
    let mip = blur(i.par_iter().zip(p).map(|(a, b)| a * b).collect());
    let (a, b): (Vec<f32>, Vec<f32>) = (0..w * h)
        .into_par_iter()
        .map(|k| {
            let var = (mii[k] - mi[k] * mi[k]).max(0.0);
            let cov = mip[k] - mi[k] * mp[k];
            let a = cov / (var + GUIDED_EPS);
            (a, mp[k] - a * mi[k])
        })
        .unzip();
    (blur(a), blur(b))
}

/// The full-size mask `a·I + b`, with `a`, `b` bilinearly upsampled from the analysis size
/// and `I = guide(x, y, pixel)` at full size; times the source alpha.
pub(crate) fn upsample(
    src: &Image,
    ca: &[f32],
    cb: &[f32],
    w: usize,
    h: usize,
    guide: impl Fn(usize, usize, &[f32]) -> f32 + Sync,
) -> Mask {
    let (fw, fh) = (src.width as usize, src.height as usize);
    let taps = |n: usize, full: usize| -> Vec<(usize, usize, f32)> {
        (0..full)
            .map(|x| {
                let f = ((x as f64 + 0.5) * n as f64 / full as f64 - 0.5).clamp(0.0, (n - 1) as f64);
                let i0 = f.floor() as usize;
                (i0, (i0 + 1).min(n - 1), (f - i0 as f64) as f32)
            })
            .collect()
    };
    let (xs, ys) = (taps(w, fw), taps(h, fh));
    let mut out = Mask::new(src.width, src.height);
    out.data.par_chunks_mut(fw).enumerate().for_each(|(y, row)| {
        let (y0, y1, ty) = ys[y];
        let lerp2 = |v: &[f32], x0: usize, x1: usize, tx: f32| {
            let top = v[y0 * w + x0] + (v[y0 * w + x1] - v[y0 * w + x0]) * tx;
            let bot = v[y1 * w + x0] + (v[y1 * w + x1] - v[y1 * w + x0]) * tx;
            top + (bot - top) * ty
        };
        for (x, o) in row.iter_mut().enumerate() {
            let (x0, x1, tx) = xs[x];
            let px = &src.data[(y * fw + x) * 4..(y * fw + x) * 4 + 4];
            let a = lerp2(ca, x0, x1, tx);
            let b = lerp2(cb, x0, x1, tx);
            let v = (a * guide(x, y, px) + b).clamp(0.0, 1.0) * px[3].clamp(0.0, 1.0);
            // snap the filter's float dust so untouched areas stay exactly 0 or 1
            *o = if v < 0.004 {
                0.0
            } else if v > 0.996 {
                1.0
            } else {
                v
            };
        }
    });
    out
}

// ---------------------------------------------------------------------------------------
// result

fn r2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

fn smoothstep(e0: f64, e1: f64, x: f64) -> f64 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// The result fields; `filled` = [hole regions, specks, cloud islands].
#[allow(clippy::too_many_arguments)]
fn describe(
    sky: &[bool],
    w: usize,
    h: usize,
    full_h: u32,
    seeds: &Seeds,
    axis: &Axis,
    left_out: f64,
    refine_px: f64,
    o: &SkyOptions,
    filled: [usize; 3],
) -> Value {
    let [hole_regions, specks, clouds] = filled;
    let to_canvas = |rows: usize| (rows as f64 * full_h as f64 / h as f64).round();
    // skyline: per column, how far the sky reaches down from the top edge
    let mut depth: Vec<usize> =
        (0..w).map(|x| (0..h).take_while(|&y| sky[y * w + x]).count()).filter(|&d| d > 0).collect();
    let sky_columns = depth.len() as f64 / w as f64;
    let horizon = if depth.is_empty() {
        Value::Null
    } else {
        depth.sort_unstable();
        json!({
            "y": to_canvas(depth[depth.len() / 2]),
            "min": to_canvas(depth[0]),
            "max": to_canvas(depth[depth.len() - 1]),
            "columns": r3(sky_columns * 100.0),
        })
    };
    let area = sky.iter().filter(|&&s| s).count() as f64 / (w * h) as f64;
    let bottom = sky[(h - 1) * w..].iter().filter(|&&s| s).count() as f64 / w as f64;
    let top_score = (seeds.top_fraction / 0.5).min(1.0);
    let mut confidence = if axis.boundary {
        smoothstep(0.03, 0.15, axis.contrast as f64)
            * (0.6 + 0.4 * top_score)
            * (area / 0.03).min(1.0)
            * (1.0 - left_out).clamp(0.2, 1.0)
    } else {
        0.5 * top_score
    };
    let advice = if !axis.boundary {
        Some("the whole canvas matched the sky model; lower --tolerance if the image shows ground")
    } else if bottom > 0.25 && h > 1 {
        confidence *= 0.5;
        Some("the sky reaches the bottom edge and may have leaked into water or haze; try a lower --tolerance")
    } else if left_out > 0.4 {
        Some("much sky-colored area was left out: add sky seen through foreground with --holes, or reach sky between dark storm clouds with a higher --tolerance")
    } else if confidence < 0.5 {
        Some("the sky's edge is faint or the ground has sky-like colors; check the selection, adjust --tolerance, or pick the sky with select color --contiguous")
    } else {
        None
    };
    let mut v = json!({
        "sky_color": hex(seeds.color),
        "horizon": horizon,
        "confidence": r3(confidence),
        "edge_contrast": r3(axis.contrast as f64),
        "top_sky_fraction": r3(seeds.top_fraction),
        "sky_colored_left_out": r3(left_out),
        "specks_filled": specks,
        "cloud_islands": clouds,
        "refine": r2(refine_px),
        "tolerance": o.tolerance,
        "analysis_size": [w, h],
    });
    if o.holes {
        v["hole_regions"] = json!(hole_regions);
    }
    if let Some(a) = advice {
        v["advice"] = json!(a);
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing;

    const W: u32 = 160;
    const H: u32 = 120;

    /// Deterministic value noise in -0.5..0.5.
    fn noise(x: u32, y: u32, seed: u32) -> f32 {
        let mut v = x.wrapping_mul(0x9E37_79B1) ^ y.wrapping_mul(0x85EB_CA77) ^ seed.wrapping_mul(0xC2B2_AE3D);
        v ^= v >> 15;
        v = v.wrapping_mul(0x2C1B_3C6D);
        v ^= v >> 12;
        v = v.wrapping_mul(0x297A_2D39);
        v ^= v >> 15;
        (v & 0xFF_FFFF) as f32 / 16_777_216.0 - 0.5
    }

    fn jitter(c: [f32; 3], x: u32, y: u32, amount: f32) -> [f32; 3] {
        [c[0] + amount * noise(x, y, 1), c[1] + amount * noise(x, y, 2), c[2] + amount * noise(x, y, 3)]
    }

    fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
        [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
    }

    fn px(c: [f32; 3]) -> [f32; 4] {
        [c[0].clamp(0.0, 1.0), c[1].clamp(0.0, 1.0), c[2].clamp(0.0, 1.0), 1.0]
    }

    /// Deep blue at the top to pale blue at the bottom, with a little sensor noise.
    fn blue_sky(x: u32, y: u32) -> [f32; 3] {
        let t = (y as f32 + 0.5) / H as f32;
        jitter([0.18 + 0.44 * t, 0.42 + 0.36 * t, 0.85 + 0.10 * t], x, y, 0.012)
    }

    fn tri(u: f32) -> f32 {
        (u - u.floor() - 0.5).abs() * 2.0
    }

    /// A jagged mountain ridge: rows 48..66 of 120.
    fn ridge(x: f32) -> f32 {
        let t = x / W as f32;
        H as f32 * (0.40 + 0.10 * tri(t * 4.0) + 0.05 * tri(t * 11.0 + 0.3))
    }

    /// Sky coverage of a pixel above the ridge (4×4 supersampled).
    fn above_ridge(x: u32, y: u32) -> f32 {
        let mut n = 0;
        for j in 0..4 {
            for i in 0..4 {
                let (sx, sy) = (x as f32 + (i as f32 + 0.5) / 4.0, y as f32 + (j as f32 + 0.5) / 4.0);
                if sy < ridge(sx) {
                    n += 1;
                }
            }
        }
        n as f32 / 16.0
    }

    /// Textured dark rock above textured green ground.
    fn ground(x: u32, y: u32) -> [f32; 3] {
        if y as f32 > H as f32 * 0.8 {
            jitter([0.22, 0.36, 0.14], x, y, 0.12)
        } else {
            jitter([0.24, 0.25, 0.28], x, y, 0.12)
        }
    }

    /// Mountains under a sky drawn by `sky`; returns the image and the true sky coverage.
    fn scene(sky: impl Fn(u32, u32) -> [f32; 3] + Sync) -> (Image, Mask) {
        let gt = Mask::from_fn(W, H, above_ridge);
        let img = Image::from_fn(W, H, |x, y| px(mix(ground(x, y), sky(x, y), gt.get(x, y))));
        (img, gt)
    }

    fn landscape() -> (Image, Mask) {
        scene(blue_sky)
    }

    /// Soft IoU: Σ min / Σ max.
    fn iou(a: &Mask, b: &Mask) -> f64 {
        let (mut i, mut u) = (0f64, 0f64);
        for (&x, &y) in a.data.iter().zip(&b.data) {
            i += x.min(y) as f64;
            u += x.max(y) as f64;
        }
        i / u
    }

    fn spec(name: &str) -> OpSpec {
        testing::find(&crate::ops(), name)
    }

    fn run(pairs: &[(&str, &str)], doc: &mut Document) -> Result<Value> {
        testing::run(&spec("select sky"), pairs, doc)
    }

    fn sel(doc: &Document) -> &Mask {
        doc.selection.as_ref().expect("a selection")
    }

    fn mean_where(m: &Mask, keep: impl Fn(u32, u32) -> bool) -> f64 {
        let (mut s, mut n) = (0f64, 0usize);
        for y in 0..m.height {
            for x in 0..m.width {
                if keep(x, y) {
                    s += m.get(x, y) as f64;
                    n += 1;
                }
            }
        }
        assert!(n > 0);
        s / n as f64
    }

    fn fractional(m: &Mask) -> usize {
        m.data.iter().filter(|&&v| v > 0.02 && v < 0.98).count()
    }

    #[test]
    fn mountains_match_the_ground_truth() {
        let (img, gt) = landscape();
        let mut d = testing::doc_from(img);
        let v = run(&[], &mut d).unwrap();
        let score = iou(sel(&d), &gt);
        assert!(score > 0.95, "IoU {score}");
        // the result tells an agent where the sky is and how sure the op is
        let y = v["horizon"]["y"].as_f64().unwrap();
        assert!((48.0..=66.0).contains(&y), "horizon {y}");
        assert!(v["horizon"]["min"].as_f64().unwrap() >= 46.0 && v["horizon"]["max"].as_f64().unwrap() <= 68.0);
        assert!(v["confidence"].as_f64().unwrap() > 0.7, "{v}");
        assert!(v["sky_color"].as_str().unwrap().starts_with('#'));
        assert_eq!(v["analysis_size"], json!([W, H]));
        assert_eq!(v["refine"], json!(3.0));
        assert!(v.get("advice").is_none(), "{v}");
        let cov = v["sky"]["coverage"].as_f64().unwrap();
        let truth = gt.data.iter().map(|&g| g as f64).sum::<f64>() / gt.data.len() as f64 * 100.0;
        assert!((cov - truth).abs() < 2.0, "coverage {cov} vs {truth}");
        assert_eq!(v["selection"], v["sky"], "replace mode: the selection is the sky");
    }

    #[test]
    fn soft_clouds_are_included() {
        let cloud = |x: u32, y: u32| {
            let (dx, dy) = (x as f32 - 88.0, y as f32 - 24.0);
            (1.6 * (-(dx * dx / (2.0 * 16.0 * 16.0) + dy * dy / (2.0 * 6.0 * 6.0))).exp()).min(1.0)
        };
        let (img, gt) = scene(|x, y| mix(blue_sky(x, y), jitter([0.96, 0.96, 0.97], x, y, 0.012), cloud(x, y)));
        let mut d = testing::doc_from(img);
        run(&[], &mut d).unwrap();
        let core = mean_where(sel(&d), |x, y| cloud(x, y) > 0.6);
        assert!(core > 0.97, "cloud core selected {core}");
        assert!(iou(sel(&d), &gt) > 0.95);
    }

    #[test]
    fn sunset_gradient_and_black_silhouette() {
        let sunset = |x: u32, y: u32| {
            let t = (y as f32 + 0.5) / H as f32 / 0.55;
            jitter(mix([0.16, 0.20, 0.48], [0.98, 0.62, 0.30], t.min(1.0)), x, y, 0.012)
        };
        let gt = Mask::from_fn(W, H, above_ridge);
        let img =
            Image::from_fn(W, H, |x, y| px(mix(jitter([0.05, 0.04, 0.06], x, y, 0.04), sunset(x, y), gt.get(x, y))));
        let mut d = testing::doc_from(img);
        run(&[], &mut d).unwrap();
        let score = iou(sel(&d), &gt);
        assert!(score > 0.95, "IoU {score}");
    }

    /// A dark tree crown standing on the ground with a sky window inside it.
    fn tree() -> Image {
        let (cx, cy, r, hole) = (80.0f32, 60.0f32, 26.0f32, 7.0f32);
        Image::from_fn(W, H, |x, y| {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            let d = ((fx - cx).powi(2) + (fy - cy).powi(2)).sqrt();
            let c = if fy > H as f32 * 0.82 {
                jitter([0.22, 0.36, 0.14], x, y, 0.12)
            } else if d < hole {
                blue_sky(x, y)
            } else if d < r {
                jitter([0.10, 0.22, 0.08], x, y, 0.10)
            } else if (fx - cx).abs() < 3.0 && fy > cy {
                jitter([0.25, 0.17, 0.10], x, y, 0.08)
            } else {
                blue_sky(x, y)
            };
            px(c)
        })
    }

    #[test]
    fn enclosed_sky_needs_holes() {
        let inside = |x: u32, y: u32| ((x as f32 + 0.5 - 80.0).powi(2) + (y as f32 + 0.5 - 60.0).powi(2)).sqrt() < 5.0;
        let crown = |x: u32, y: u32| {
            let d = ((x as f32 + 0.5 - 80.0).powi(2) + (y as f32 + 0.5 - 60.0).powi(2)).sqrt();
            (11.0..23.0).contains(&d)
        };
        let mut d = testing::doc_from(tree());
        let v = run(&[], &mut d).unwrap();
        assert!(mean_where(sel(&d), inside) < 0.02, "window excluded by default");
        assert!(mean_where(sel(&d), crown) < 0.02, "crown excluded");
        assert!(mean_where(sel(&d), |x, y| y < 30 && !(70..90).contains(&x)) > 0.98, "open sky selected");
        assert!(v.get("hole_regions").is_none());
        let missed = v["sky_colored_left_out"].as_f64().unwrap();
        assert!(missed > 0.0);
        let v = run(&[("holes", "true")], &mut d).unwrap();
        assert!(v["sky_colored_left_out"].as_f64().unwrap() < missed);
        assert!(mean_where(sel(&d), inside) > 0.95, "window included with --holes");
        assert!(mean_where(sel(&d), crown) < 0.02, "crown still excluded");
        assert_eq!(v["hole_regions"], json!(1));
    }

    #[test]
    fn no_sky_fails_with_a_hint() {
        // dark, busy texture all the way to the top
        let busy = Image::from_fn(W, H, |x, y| {
            let v = 0.15 + 0.5 * (noise(x / 2, y / 2, 7) + 0.5);
            px([v, v * 0.9, v * 0.8])
        });
        let e = run(&[], &mut testing::doc_from(busy)).unwrap_err();
        assert_eq!(e.code, "PX3320");
        assert!(e.hint.as_deref().unwrap_or("").contains("select color"), "{:?}", e.hint);
        // smooth but nearly black (a night or studio backdrop)
        let dark = testing::solid(W, H, Rgba::new(0.04, 0.05, 0.08, 1.0));
        let e = run(&[], &mut testing::doc_from(dark)).unwrap_err();
        assert_eq!(e.code, "PX3320");
        assert!(e.message.contains("too dark"), "{}", e.message);
        // transparent
        let e = run(&[], &mut testing::doc_from(Image::new(16, 16))).unwrap_err();
        assert_eq!(e.code, "PX3320");
    }

    #[test]
    fn modes_combine_with_the_selection() {
        let (img, _) = landscape();
        let rock = |m: &Mask| m.get(80, 84);
        let mut d = testing::doc_from(img);
        // add: a strip of ground plus the sky
        testing::run(&spec("select rect"), &[("rect", "0,100,160x20")], &mut d).unwrap();
        run(&[("mode", "add")], &mut d).unwrap();
        assert_eq!(sel(&d).get(10, 110), 1.0);
        assert_eq!(sel(&d).get(10, 10), 1.0);
        assert_eq!(rock(sel(&d)), 0.0);
        // subtract: everything but the sky
        testing::run(&spec("select all"), &[], &mut d).unwrap();
        run(&[("mode", "subtract")], &mut d).unwrap();
        assert_eq!(sel(&d).get(10, 10), 0.0);
        assert_eq!(rock(sel(&d)), 1.0);
        assert_eq!(sel(&d).get(10, 110), 1.0);
        // intersect: the sky in the left half
        testing::run(&spec("select rect"), &[("rect", "0,0,80x120")], &mut d).unwrap();
        run(&[("mode", "intersect")], &mut d).unwrap();
        assert_eq!(sel(&d).get(10, 10), 1.0);
        assert_eq!(sel(&d).get(150, 10), 0.0);
        assert_eq!(sel(&d).get(10, 110), 0.0);
    }

    #[test]
    fn refine_and_feather_soften_the_edge() {
        let (img, gt) = landscape();
        let mut hard = testing::doc_from(img);
        let mut soft = hard.clone();
        let mut feathered = hard.clone();
        let v = run(&[("refine", "0")], &mut hard).unwrap();
        assert_eq!(v["refine"], json!(0.0));
        assert!(sel(&hard).data.iter().all(|&v| v == 0.0 || v == 1.0), "refine 0 is a hard mask");
        assert!(iou(sel(&hard), &gt) > 0.95);
        run(&[], &mut soft).unwrap();
        let edge = fractional(sel(&soft));
        assert!(edge > 60, "the refined edge is soft: {edge} fractional px");
        run(&[("feather", "6")], &mut feathered).unwrap();
        assert!(fractional(sel(&feathered)) > 2 * edge);
        let (a, b) = (sel(&soft).data.iter().sum::<f32>(), sel(&feathered).data.iter().sum::<f32>());
        assert!((a - b).abs() / a < 0.03, "feathering keeps the area: {a} vs {b}");
    }

    #[test]
    fn deterministic_across_runs_and_thread_counts() {
        let img = tree();
        let pairs = [("holes", "true"), ("feather", "2")];
        let on = |threads: usize| {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
            pool.install(|| {
                let mut d = testing::doc_from(img.clone());
                let v = run(&pairs, &mut d).unwrap();
                (d.selection.unwrap(), v)
            })
        };
        let (m1, v1) = on(1);
        let (m4, v4) = on(4);
        assert_eq!(m1, m4);
        assert_eq!(v1, v4);
        assert_eq!(on(3).0, m1);
    }

    #[test]
    fn samples_one_layer() {
        let (img, gt) = landscape();
        let cover = Image::from_fn(W, H, |x, y| {
            let v = 0.2 + 0.4 * (noise(x / 2, y / 2, 9) + 0.5);
            px([v, v, v])
        });
        let mut d = testing::doc_layers(W, H, vec![("Sky", img), ("Cover", cover)]);
        assert_eq!(run(&[], &mut d).unwrap_err().code, "PX3320", "the composite shows no sky");
        let e = run(&[("layer", "Cover")], &mut d).unwrap_err();
        assert_eq!(e.context.get("layer"), Some(&json!("Cover")));
        run(&[("layer", "Sky")], &mut d).unwrap();
        assert!(iou(sel(&d), &gt) > 0.95);
    }

    #[test]
    fn recorded_auto_refine_replays_exactly() {
        // the auto radius is recorded as text for replay; it must rebuild the same mask
        let (img, _) = landscape();
        let o = SkyOptions { tolerance: 50.0, refine: None, holes: false };
        let auto = detect(&img, &o, 70).unwrap();
        let recorded: f64 = auto.refine_px.to_string().parse().unwrap();
        let again = detect(&img, &SkyOptions { refine: Some(recorded), ..o }, 70).unwrap();
        assert!(auto.mask.data == again.mask.data, "replayed refine {recorded} changed the mask");
    }

    #[test]
    fn large_images_are_analyzed_downscaled() {
        let (img, gt) = landscape();
        let o = SkyOptions { tolerance: 50.0, refine: None, holes: false };
        let det = detect(&img, &o, 64).unwrap();
        assert_eq!(det.info["analysis_size"], json!([64, 48]));
        assert_eq!((det.mask.width, det.mask.height), (W, H));
        assert!((det.refine_px - 7.5).abs() < 1e-9, "auto refine is 3 analysis px: {}", det.refine_px);
        let score = iou(&det.mask, &gt);
        assert!(score > 0.93, "IoU {score}");
        // refine 0: the upsampled analysis mask
        let det = detect(&img, &SkyOptions { refine: Some(0.0), ..o }, 64).unwrap();
        assert!(iou(&det.mask, &gt) > 0.9);
    }

    #[test]
    fn tiny_and_uniform_images() {
        let blue = Rgba::new(0.45, 0.65, 0.95, 1.0);
        let mut d = testing::doc_from(testing::solid(1, 1, blue));
        let v = run(&[], &mut d).unwrap();
        assert_eq!(sel(&d).data, vec![1.0]);
        assert!(v["advice"].as_str().unwrap().contains("whole canvas"));
        let mut d = testing::doc_from(testing::solid(3, 2, blue));
        run(&[("holes", "true"), ("refine", "5")], &mut d).unwrap();
        assert!(sel(&d).data.iter().all(|&v| v == 1.0));
    }

    #[test]
    fn gray_and_white_buildings_are_not_sky() {
        // facades as light as the sky (pale is cheap for clouds) and brighter (white)
        let roof = |x: f32| -> (f32, usize) {
            let k = (x / (W as f32 / 6.0)) as usize;
            (H as f32 * [0.55, 0.35, 0.62, 0.45, 0.30, 0.50][k.min(5)], k.min(5))
        };
        let gt = Mask::from_fn(W, H, |x, y| if (y as f32) < roof(x as f32 + 0.5).0 { 1.0 } else { 0.0 });
        let img = Image::from_fn(W, H, |x, y| {
            let facade = [[0.25, 0.26, 0.30], [0.62, 0.60, 0.58], [0.92, 0.92, 0.90]][roof(x as f32 + 0.5).1 % 3];
            let c = if (x % 6 < 3) && (y % 8 < 4) { [0.95, 0.85, 0.5] } else { facade };
            px(mix(jitter(c, x, y, 0.04), blue_sky(x, y), gt.get(x, y)))
        });
        let mut d = testing::doc_from(img);
        run(&[], &mut d).unwrap();
        let score = iou(sel(&d), &gt);
        assert!(score > 0.95, "IoU {score}");
        for k in 0..6 {
            let x = (k as f32 + 0.5) * W as f32 / 6.0;
            let below = mean_where(sel(&d), |px, py| (px as f32 - x).abs() < 6.0 && py as f32 > roof(x).0 + 3.0);
            assert!(below < 0.03, "building {k} selected: {below}");
        }
    }

    #[test]
    fn crisp_clouds_at_the_top_and_floating() {
        // hard-edged clouds: one fills most of the top band (it sets the reference color,
        // the blue must still seed), one floats mid-sky (an island the edges cut off)
        let ellipse = |x: u32, y: u32, cx: f32, cy: f32, rx: f32, ry: f32| {
            let (dx, dy) = ((x as f32 + 0.5 - cx) / rx, (y as f32 + 0.5 - cy) / ry);
            dx * dx + dy * dy < 1.0
        };
        let top = |x: u32, y: u32| ellipse(x, y, 55.0, 2.0, 50.0, 18.0);
        let floating = |x: u32, y: u32| ellipse(x, y, 120.0, 30.0, 18.0, 7.0);
        let (img, gt) =
            scene(
                |x, y| {
                    if top(x, y) || floating(x, y) {
                        jitter([0.96, 0.96, 0.97], x, y, 0.012)
                    } else {
                        blue_sky(x, y)
                    }
                },
            );
        let mut d = testing::doc_from(img);
        let v = run(&[], &mut d).unwrap();
        assert!(mean_where(sel(&d), top) > 0.97, "top cloud selected");
        assert!(mean_where(sel(&d), floating) > 0.97, "floating cloud selected");
        assert!(iou(sel(&d), &gt) > 0.95);
        assert!(v["cloud_islands"].as_u64().unwrap() >= 1, "{v}");
    }

    /// Smooth value noise in 0..1 (cell size 1).
    fn value_noise(x: f32, y: f32, seed: u32) -> f32 {
        let at = |i: f32, j: f32| noise(i as u32, j as u32, seed) + 0.5;
        let (x0, y0) = (x.floor(), y.floor());
        let (tx, ty) = (x - x0, y - y0);
        let (tx, ty) = (tx * tx * (3.0 - 2.0 * tx), ty * ty * (3.0 - 2.0 * ty));
        let top = at(x0, y0) + (at(x0 + 1.0, y0) - at(x0, y0)) * tx;
        let bottom = at(x0, y0 + 1.0) + (at(x0 + 1.0, y0 + 1.0) - at(x0, y0 + 1.0)) * tx;
        top + (bottom - top) * ty
    }

    fn fbm(x: f32, y: f32, seed: u32) -> f32 {
        (0..4)
            .map(|o| 0.5f32.powi(o as i32 + 1) * value_noise(x * (1 << o) as f32, y * (1 << o) as f32, seed + o))
            .sum()
    }

    /// A scene `w`×`h` whose sky is above `edge(x)` (3×3 supersampled), for the downscaled
    /// analysis path (edges then spread over 2–3 analysis px).
    fn big_scene(
        w: u32,
        h: u32,
        edge: impl Fn(f32) -> f32 + Sync,
        sky: impl Fn(u32, u32) -> [f32; 3] + Sync,
        ground: impl Fn(u32, u32) -> [f32; 3] + Sync,
    ) -> (Image, Mask) {
        let gt = Mask::from_fn(w, h, |x, y| {
            let n = (0..9)
                .filter(|k| {
                    (y as f32 + (k / 3) as f32 / 3.0 + 1.0 / 6.0) < edge(x as f32 + (k % 3) as f32 / 3.0 + 1.0 / 6.0)
                })
                .count();
            n as f32 / 9.0
        });
        let img = Image::from_fn(w, h, |x, y| px(mix(ground(x, y), sky(x, y), gt.get(x, y))));
        (img, gt)
    }

    fn analyze(img: &Image, tolerance: f32) -> Detection {
        detect(img, &SkyOptions { tolerance, refine: None, holes: false }, 300).unwrap()
    }

    #[test]
    fn hazy_ridge_with_level_stretches_is_ground() {
        // a distant bluish ridge nearly the sky's color (ΔE ≈ 0.065, paler at the same
        // lightness) with nearly level stretches, downscaled: each pixel step across the
        // edge is small and a flood could walk along it
        let (w, h) = (480u32, 300u32);
        let (fw, fh) = (w as f32, h as f32);
        let far = move |x: f32| fh * (0.45 + 0.08 * tri(x / fw * 3.0 + 0.2) + 0.03 * tri(x / fw * 9.0));
        let near = move |x: f32| fh * (0.62 + 0.10 * tri(x / fw * 2.0 + 0.6));
        let sky = move |x: u32, y: u32| {
            let t = y as f32 / fh;
            jitter([0.18 + 0.50 * t, 0.42 + 0.42 * t, 0.85 + 0.12 * t], x, y, 0.015)
        };
        let ground = move |x: u32, y: u32| {
            if (y as f32 + 0.5) > near(x as f32 + 0.5) {
                jitter([0.20, 0.24, 0.20], x, y, 0.12)
            } else {
                jitter([0.50, 0.60, 0.74], x, y, 0.03)
            }
        };
        let (img, gt) = big_scene(w, h, far, sky, ground);
        let det = analyze(&img, 50.0);
        let score = iou(&det.mask, &gt);
        assert!(score > 0.97, "IoU {score}");
        // honest about the faint edge
        assert!(det.info["confidence"].as_f64().unwrap() < 0.5, "{}", det.info);
        assert!(det.info["advice"].is_string());
        // the highest tolerance accepts the haze as sky
        assert!(iou(&analyze(&img, 100.0).mask, &gt) < 0.9);
    }

    #[test]
    fn noisy_sky_with_soft_clouds() {
        // heavy sensor noise and soft, mottled clouds, downscaled: the noise must not read
        // as edges, and cloud edges (brighter and paler) must stay cheap to cross
        let (w, h) = (480u32, 300u32);
        let (fw, fh) = (w as f32, h as f32);
        let ridge = move |x: f32| fh * (0.45 + 0.12 * tri(x / fw * 4.0) + 0.05 * tri(x / fw * 11.0 + 0.3));
        let sky = move |x: u32, y: u32| {
            let cloud = ((fbm(x as f32 / 25.0, y as f32 / 12.0, 41) - 0.45) * 4.0).clamp(0.0, 1.0);
            let t = y as f32 / fh;
            jitter(mix([0.18 + 0.50 * t, 0.42 + 0.42 * t, 0.85 + 0.12 * t], [0.95, 0.95, 0.96], cloud), x, y, 0.08)
        };
        let (img, gt) = big_scene(w, h, ridge, sky, |x, y| jitter([0.30, 0.27, 0.24], x, y, 0.15));
        let score = iou(&analyze(&img, 50.0).mask, &gt);
        assert!(score > 0.97, "IoU {score}");
    }

    #[test]
    fn noise_is_not_texture_even_at_tolerance_zero() {
        // high-ISO noise (about ±0.04 per channel): thresholds rise to clear it
        let (img, gt) = scene(|x, y| {
            let t = (y as f32 + 0.5) / H as f32;
            jitter([0.18 + 0.44 * t, 0.42 + 0.36 * t, 0.85 + 0.10 * t], x, y, 0.08)
        });
        let mut d = testing::doc_from(img);
        let v = run(&[("tolerance", "0")], &mut d).unwrap();
        let score = iou(sel(&d), &gt);
        assert!(score > 0.95, "IoU {score} {v}");
    }

    #[test]
    fn tolerance_zero_is_stricter() {
        // a hazy band just above the ridge is sky-ish: tolerance decides
        let (img, _) = landscape();
        let mut a = testing::doc_from(img);
        let mut b = a.clone();
        run(&[("tolerance", "0")], &mut a).unwrap();
        run(&[("tolerance", "100")], &mut b).unwrap();
        let (sa, sb) = (sel(&a).data.iter().sum::<f32>(), sel(&b).data.iter().sum::<f32>());
        assert!(sa <= sb + 1e-3, "{sa} vs {sb}");
        assert!(sa > 0.0);
        assert_eq!(run(&[("tolerance", "101")], &mut a).unwrap_err().code, "PX1013");
    }
}
