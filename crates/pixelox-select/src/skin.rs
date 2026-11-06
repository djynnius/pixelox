//! `select skin`: the skin mask portrait retouching needs (smoothing and blemish removal
//! stay on skin, never on eyes, brows, lips, hair, teeth, clothing, or background).
//!
//! It is sampled, not a fixed skin-color rule, so it works for every skin tone. Pipeline
//! (O(pixels) throughout, deterministic, thread-count invariant):
//!
//! 1. Analyze a copy whose long side is at most [`WORK_LONG`] px, in OKLab, through a 3x3
//!    per-channel median (noise down, thin brows and lash lines kept; a blur would mix
//!    them into skin-like colors).
//! 2. Samples: for each `--at`, the per-channel median of a small window and its spread
//!    (median absolute deviations). A sample that does not look like skin (near neutral,
//!    near black, or a blue, green, or purple hue) is reported, and left out of the model
//!    when another sample does look like skin.
//! 3. Model, in terms that shading does not change: scaling linear light scales OKLab's
//!    L, a, and b alike, so hue and saturation (C/L) stay put while L moves. The model
//!    spans the samples' hues and log saturations, widened by `--tolerance` (and the
//!    samples' own spread), and a wide but bounded log-lightness range (`--lightness`):
//!    shadowed skin is in, near-black hair and pupils are out. Membership is 1 inside,
//!    0.5 one tolerance outside (hue, saturation, and lightness excess combined in
//!    quadrature), and 0 at one and a half; it is averaged over σ = 1 (decisions, not
//!    colors) and thresholded at 0.5.
//! 4. Regions: skin-colored pixels 4-connected to the samples (`--contiguous on`), or all
//!    of them less islands up to [`ISLAND_RATIO`] speck areas that hold no sample.
//! 5. Clean-up, judged on the unfiltered analysis colors against the skin around each
//!    hole: holes enclosed by skin up to the speck size are filled unless they hold part
//!    of an eye or a mouth (sclera or teeth: bright and nearly neutral; lips: another hue)
//!    or are long (brows, lash lines), so blemishes, moles, and pores are in; holes up to
//!    [`HIGHLIGHT_RATIO`] speck areas are filled when they look like a specular highlight:
//!    compact, brighter than plain skin around them, no dark or off-hue pixels (a pupil,
//!    iris, or lash line keeps an eye out even where the sclera is all that shows).
//! 6. Refine: a guided filter whose guide is a steep membership (fitted at the analysis
//!    size on the median-filtered colors, applied with the full-size pixels' membership)
//!    snaps the soft edge to the image at full resolution; a steep guide keeps colors
//!    that are only a little skin-like (lips, hair) out of the filter's linear model.

use crate::select::{apply, sample_source};
use crate::sky::{
    flood, for_each_component, guided_coeffs, hex, median3, neighbors, oklab, quantile, to_oklab, upsample, Lab,
};
use crate::util::{feather_param, len_px, mask_json, mode_param, r1, r3, select_mode};
use pixelox_core::kernels::{Edge, Filter};
use pixelox_core::prelude::*;
use pixelox_core::units::PointSpec;
use rayon::prelude::*;
use std::f64::consts::PI;

/// Long side of the analysis copy; larger images are analyzed downscaled and the guided
/// filter brings the mask back to full size.
const WORK_LONG: u32 = 1024;
/// Sample window radius: this fraction of the analysis short side, at least [`MIN_WINDOW`].
const WINDOW: f64 = 0.006;
const MIN_WINDOW: usize = 2;
/// Darker than this (OKLab L) is never skin: hue and saturation there are mostly noise.
const MIN_L: f32 = 0.04;
/// Auto clean-up size: holes up to this fraction of the largest skin region are filled
/// (unless they hold sclera, teeth, or lips; see [`Hole::feature`]).
const SPECK_FRACTION: f64 = 0.003;
/// Highlight holes up to this many speck areas (their content is checked, see
/// [`Hole::highlight`]); with `--contiguous off`, islands up to [`ISLAND_RATIO`] are dropped.
const HIGHLIGHT_RATIO: f64 = 8.0;
const ISLAND_RATIO: f64 = 3.0;
/// A highlight hole has at most this fraction of pixels darker than [`DARK_RATIO`] times
/// the skin around it (a pupil or lash line is more), and at most [`OFF_HUE_MAX`] of
/// pixels well outside the hue range (lips, colored objects) ...
const DARK_MAX: f64 = 0.02;
const DARK_RATIO: f32 = 0.8;
const OFF_HUE_MAX: f64 = 0.05;
/// ... and at most this fraction of the skin around it darker than [`DARK_RATIO`] times
/// its median (the sclera beside an iris is bright, but not in plain skin).
const DARK_RING_MAX: f64 = 0.1;
/// Holes this much sclera or teeth (bright, nearly neutral) or lips (off-hue) are part of
/// an eye or a mouth, whatever their size.
const FEATURE_WHITE: f64 = 0.1;
const FEATURE_OFF_HUE: f64 = 0.25;
/// Holes at least this many times longer than wide are brows, lash lines, or lip lines.
const FEATURE_LONG: f64 = 3.0;
/// Soft band of the lightness range, in ln(L) units (membership 0.5 this far outside).
const LIGHT_TOL: f32 = 0.1;
/// Most the samples' own spread can widen the tolerances (hue in degrees, others in ln).
const MAX_SPREAD_HUE: f32 = 8.0;
const MAX_SPREAD_LOG: f32 = 0.25;
/// Plausibility of a sample as skin (advice, and which samples build the model; the
/// selection itself is never ruled by a fixed skin color).
const SKIN_MIN_SAT: f32 = 0.025;
const SKIN_MIN_L: f32 = 0.1;
const SKIN_HUES: (f32, f32) = (10.0, 110.0);
/// A sample window with more spread than this straddles an edge.
const EDGE_HUE: f32 = 12.0;
const EDGE_LIGHT: f32 = 0.15;
/// Used samples whose hues or saturations differ by more than this disagree.
const DISAGREE_HUE: f32 = 15.0;
const DISAGREE_SAT: f32 = 0.7;

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("select skin", Category::Select, "Select skin sampled at points, for retouching", skin)
        .about("Select the skin of the person at --at (one or more points on plain skin: a lit cheek, and the neck or the shadow side when the light is uneven), for retouching that must stay on skin. The skin model is sampled, so it works for every skin tone: in OKLab, light and shadow change lightness but keep hue and saturation (C/L), so the model is tight on hue and saturation (--tolerance) and wide but bounded on lightness (--lightness): shadowed skin is in; lips (redder), eyes and teeth (neutral), dark hair and brows (darker), and backgrounds and clothing (other hues) are out. Holes inside the skin are filled when small and not part of an eye, brow, or mouth (blemishes, moles, pores), or when they look like a specular highlight (compact, brighter, no dark or off-hue pixels); with --contiguous on (default) only skin connected to the samples is kept. A guided filter snaps the soft edge to the image (images are analyzed at up to 1024 px). Samples the composite unless --layer is given. The result gives `skin` (coverage and bounds of the skin alone), `samples` (each point's color, lightness, chroma, hue, and whether it looks like skin), `model` (hue, saturation, and lightness ranges at half membership), clean-up counts, and `advice` when something looks off: a sample that is not skin-like, samples that disagree, skin-colored areas left out, or very little or very much selected.")
        .param(param("at", ParamKind::Point).repeatable().required().help("A point on plain skin X,Y (repeatable): a lit cheek, plus the neck or the shadow side when the light is uneven"))
        .param(param("tolerance", ParamKind::Int).default("50").range("0..100").help("How far hue and saturation may stray from the samples, 0..100: raise when skin is missed, lower when lips, hair, or background leak in"))
        .param(param("lightness", ParamKind::Int).default("50").range("0..100").help("How much darker (shadow) or lighter than the samples skin may be, 0..100: raise for deep shadows, lower when dark hair or brows leak in"))
        .param(param("contiguous", ParamKind::Switch).default("on").help("on: only skin connected to the --at points; off: every skin-colored region (hands, other people) except small specks"))
        .param(param("holes", ParamKind::Length).auto().default("auto").help("Clean-up size: fill holes in the skin up to this diameter in px (blemishes, moles; highlights up to about 2.8x it; never eyes, brows, or mouths) and, with --contiguous off, drop specks; auto scales with the skin area, 0 = off"))
        .param(param("refine", ParamKind::Length).auto().default("auto").help("Edge refinement radius in px (a guided filter snaps the soft edge to the image); auto scales with the image size, 0 = off"))
        .param(param("layer", ParamKind::Layer).help("Sample this layer instead of the composite"))
        .param(feather_param())
        .param(mode_param())
        .example("pixelox select skin portrait.pxo --at 420,610 --in-place", "Select the skin of the person whose cheek is at 420,610")
        .example("pixelox select skin portrait.pxo --at 420,610 --at 560,980 --lightness 70 --in-place", "Sample the lit cheek and the shaded neck, and allow deeper shadows")
        .example("pixelox select skin group.pxo --at 300,400 --contiguous off --in-place", "Every skin-colored region: faces, hands, and arms")
        .example("pixelox select skin portrait.pxo --at 420,610 --tolerance 35 --feather 2 --in-place", "A tighter model when the lips leak in, with a softer edge")
        .example("pixelox select skin portrait.jpg --at 50%,40% -o skin.pxo --dry-run --json", "Plan it and read the coverage, samples, model, and advice")]
}

fn skin(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let mode = select_mode(a)?;
    let feather = len_px(a, "feather", doc)?;
    let refine = if a.is_auto("refine") { None } else { Some(len_px(a, "refine", doc)?) };
    let holes = if a.is_auto("holes") { None } else { Some(len_px(a, "holes", doc)?) };
    let opts = SkinOptions {
        points: sample_points(a, doc)?,
        tolerance: a.int("tolerance")? as f32,
        lightness: a.int("lightness")? as f32,
        contiguous: a.switch("contiguous", true),
        holes,
        refine,
    };
    let src = sample_source(doc, a.raw("layer"))?;
    let det = detect(&src, &opts, WORK_LONG).map_err(|e| match a.raw("layer") {
        Some(l) => e.with_context("layer", l),
        None => e,
    })?;
    // full precision: a replay must rebuild exactly the same mask
    if refine.is_none() {
        ctx.resolve_param("refine", det.refine_px.to_string());
    }
    if holes.is_none() {
        ctx.resolve_param("holes", det.holes_px.to_string());
    }
    let skin_stats = mask_json(&det.mask);
    let mut v = apply(doc, ctx, det.mask, mode, feather);
    if let (Value::Object(out), Value::Object(info)) = (&mut v, det.info) {
        out.extend(info);
    }
    v["skin"] = skin_stats;
    Ok(v)
}

/// The `--at` points as canvas pixels (PX1011 outside the canvas).
fn sample_points(a: &Args, doc: &Document) -> Result<Vec<[u32; 2]>> {
    let raw = a.all("at");
    if raw.is_empty() {
        return Err(Error::new("PX1003", "'select skin' needs at least one --at X,Y on skin")
            .with_hint("--at 420,610 (a point on a lit cheek)")
            .with_context("param", "at"));
    }
    raw.iter()
        .map(|r| {
            let p = PointSpec::parse(r).map_err(|mut e| {
                e.message = format!("--at: {}", e.message);
                e
            })?;
            let (x, y) = p.resolve(doc.width as f64, doc.height as f64, doc.dpi());
            let (ix, iy) = (x.floor(), y.floor());
            if ix < 0.0 || iy < 0.0 || ix >= doc.width as f64 || iy >= doc.height as f64 {
                return Err(Error::new(
                    "PX1011",
                    format!("--at {x},{y} is outside the {}x{} canvas", doc.width, doc.height),
                )
                .with_context("at", [x, y]));
            }
            Ok([ix as u32, iy as u32])
        })
        .collect()
}

// ---------------------------------------------------------------------------------------
// detection

struct SkinOptions {
    /// Sample points in canvas px.
    points: Vec<[u32; 2]>,
    /// 0..100.
    tolerance: f32,
    /// 0..100.
    lightness: f32,
    contiguous: bool,
    /// Clean-up diameter in full-size px; `None` = auto.
    holes: Option<f64>,
    /// Refinement radius in full-size px; `None` = auto.
    refine: Option<f64>,
}

struct Detection {
    /// Soft skin mask, the size of the source.
    mask: Mask,
    refine_px: f64,
    holes_px: f64,
    /// Result fields (`samples`, `model`, clean-up counts, `advice`, ...).
    info: Value,
}

/// Find the skin in `src` (canvas-sized), analyzing at most `work_long` px on the long side.
fn detect(src: &Image, o: &SkinOptions, work_long: u32) -> Result<Detection> {
    let (fw, fh) = (src.width, src.height);
    if fw == 0 || fh == 0 {
        return Err(Error::new("PX3320", "the image is empty"));
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
    // analysis px per full-size px
    let k = (ww as f64 / fw as f64 + wh as f64 / fh as f64) / 2.0;

    // 1. analysis planes: a 3x3 median keeps thin features (brows, lash lines) that a blur
    // would mix into skin-like colors
    let alpha: Vec<f32> = small.data.par_chunks(4).map(|p| p[3]).collect();
    let raw = to_oklab(small);
    let smooth = median_planes(&raw, w, h);

    // 2. samples
    let r = ((w.min(h) as f64 * WINDOW).round() as usize).max(MIN_WINDOW);
    let samples: Vec<Sample> = o
        .points
        .iter()
        .map(|&at| {
            let ax = ((at[0] as f64 + 0.5) * k).floor().clamp(0.0, (w - 1) as f64) as usize;
            let ay = ((at[1] as f64 + 0.5) * k).floor().clamp(0.0, (h - 1) as f64) as usize;
            Sample::measure(&smooth, &alpha, w, h, ax, ay, r, at)
        })
        .collect::<Result<_>>()?;
    let any_plausible = samples.iter().any(|s| s.issue.is_none());
    let used: Vec<bool> = samples.iter().map(|s| !any_plausible || s.issue.is_none()).collect();

    // 3. model and membership
    let model = Model::fit(samples.iter().zip(&used).filter(|(_, &u)| u).map(|(s, _)| s), o);
    // membership averaged over σ = 1 (decisions, not colors, are averaged): noise-robust
    let mut soft: Vec<f32> =
        smooth.par_iter().zip(alpha.par_iter()).map(|(&c, &a)| if a >= 0.5 { model.member(c) } else { 0.0 }).collect();
    kernels::gaussian_planar(&mut soft, w, h, 1, 1.0, 1.0, Edge::Clamp);
    let colored: Vec<bool> = soft.par_iter().map(|&m| m >= 0.5).collect();
    drop(soft);

    // 4. regions
    let mut seeds: Vec<usize> = samples
        .iter()
        .zip(&used)
        .filter(|(_, &u)| u)
        .flat_map(|(s, _)| s.window.iter().copied())
        .filter(|&i| colored[i])
        .collect();
    seeds.sort_unstable();
    seeds.dedup();
    let mut skin = if o.contiguous { flood(&seeds, w, h, |_, q| colored[q]) } else { colored.clone() };
    let mut largest = 0usize;
    for_each_component(&skin, w, h, |_, _| true, |comp, _| largest = largest.max(comp.len()));
    let holes_px = o.holes.unwrap_or_else(|| (4.0 * SPECK_FRACTION * largest as f64 / PI).sqrt() / k);
    // analysis px areas, always derived from the (recorded) diameter so replays match
    let speck_area = PI / 4.0 * (holes_px * k).powi(2);
    let highlight_area = HIGHLIGHT_RATIO * speck_area;
    let islands = if o.contiguous || holes_px <= 0.0 {
        0
    } else {
        drop_islands(&mut skin, &seeds, w, h, ISLAND_RATIO * speck_area)
    };
    // integer counts: the result does not depend on how the work is split
    let on = skin.par_iter().filter(|&&s| s).count();
    let left = (0..w * h).into_par_iter().filter(|&i| colored[i] && !skin[i]).count();
    let left_out = left as f64 / on.max(1) as f64;

    // 5. clean-up
    let (blemishes, highlights) =
        if holes_px > 0.0 { fill_holes(&mut skin, &raw, &model, w, h, speck_area, highlight_area) } else { (0, 0) };

    // 6. refine
    // the guide: a steep membership, of the median-filtered colors at the analysis size
    // (noise-robust, edges kept) and of the full-size pixels when the analysis is scaled
    let refine_px = o.refine.unwrap_or(3.0 / k);
    let p: Vec<f32> = skin.iter().map(|&s| if s { 1.0 } else { 0.0 }).collect();
    let mask = if refine_px > 0.0 {
        let guide_w: Vec<f32> = smooth.par_iter().map(|&c| model.edge(c)).collect();
        let (ca, cb) = guided_coeffs(&guide_w, &p, w, h, refine_px * k / 2.0);
        upsample(src, &ca, &cb, w, h, |x, y, px| if scaled { model.edge(oklab(px)) } else { guide_w[y * w + x] })
    } else {
        upsample(src, &vec![0.0; w * h], &p, w, h, |_, _, _| 0.0)
    };

    let found = Found {
        samples: &samples,
        used: &used,
        model: &model,
        seeds: seeds.len(),
        left_out,
        counts: [blemishes, highlights, islands],
        coverage: mask_cover(&mask),
    };
    let info = found.describe(o, holes_px, refine_px, [w, h]);
    Ok(Detection { mask, refine_px, holes_px, info })
}

// ---------------------------------------------------------------------------------------
// samples and the model

/// Hue in degrees, -180..180.
#[inline]
fn hue_deg(c: Lab) -> f32 {
    c[2].atan2(c[1]).to_degrees()
}

/// `d` wrapped into -180..180.
#[inline]
fn wrap(d: f32) -> f32 {
    (d + 180.0).rem_euclid(360.0) - 180.0
}

#[inline]
fn chroma(c: Lab) -> f32 {
    (c[1] * c[1] + c[2] * c[2]).sqrt()
}

/// ln of saturation C/L (bounded away from 0 and from noise at near-black).
#[inline]
fn ln_sat(c: Lab) -> f32 {
    (chroma(c) / c[0].max(MIN_L)).max(1e-4).ln()
}

#[inline]
fn ln_light(c: Lab) -> f32 {
    c[0].max(MIN_L).ln()
}

/// How far `v` is outside `lo..=hi` (0 inside).
#[inline]
fn excess(v: f32, (lo, hi): (f32, f32)) -> f32 {
    if v < lo {
        lo - v
    } else if v > hi {
        v - hi
    } else {
        0.0
    }
}

struct Sample {
    /// Canvas px.
    at: [u32; 2],
    /// Median color of the window (median-filtered analysis colors).
    color: Lab,
    /// Hue in degrees, -180..180.
    hue: f32,
    /// Median absolute deviation in the window: hue (degrees), ln saturation, ln lightness.
    spread: [f32; 3],
    /// Opaque window pixels (analysis indices).
    window: Vec<usize>,
    /// Why the sample does not look like skin.
    issue: Option<String>,
    /// The window is not uniform (an edge between skin and something else).
    edge: bool,
}

impl Sample {
    #[allow(clippy::too_many_arguments)]
    fn measure(
        c: &[Lab],
        alpha: &[f32],
        w: usize,
        h: usize,
        x: usize,
        y: usize,
        r: usize,
        at: [u32; 2],
    ) -> Result<Sample> {
        let (x0, x1, y0, y1) = (x.saturating_sub(r), (x + r).min(w - 1), y.saturating_sub(r), (y + r).min(h - 1));
        let window: Vec<usize> =
            (y0..=y1).flat_map(|yy| (x0..=x1).map(move |xx| yy * w + xx)).filter(|&i| alpha[i] >= 0.5).collect();
        if window.is_empty() {
            return Err(Error::new("PX3320", format!("--at {},{} is on a transparent area", at[0], at[1]))
                .with_hint("put --at on visible skin, or sample the layer that shows it with --layer")
                .with_context("at", at));
        }
        let px: Vec<Lab> = window.iter().map(|&i| c[i]).collect();
        let color = median3(&px);
        let hue = hue_deg(color);
        let mad = |f: &dyn Fn(Lab) -> f32| {
            let mut d: Vec<f32> = px.iter().map(|&p| f(p).abs()).collect();
            quantile(&mut d, 0.5)
        };
        let spread = [
            mad(&|p| wrap(hue_deg(p) - hue)),
            mad(&|p| ln_sat(p) - ln_sat(color)),
            mad(&|p| ln_light(p) - ln_light(color)),
        ];
        let sat = chroma(color) / color[0].max(MIN_L);
        let hue360 = hue.rem_euclid(360.0);
        let issue = if color[0] < SKIN_MIN_L {
            Some("nearly black (hair, a pupil, or deep shadow)".to_string())
        } else if sat < SKIN_MIN_SAT {
            Some("nearly neutral (white, gray, or black, like eyes, teeth, or a backdrop)".to_string())
        } else if !(SKIN_HUES.0..=SKIN_HUES.1).contains(&hue360) {
            Some(format!("not a skin hue ({hue360:.0}°: blue, green, or purple)"))
        } else {
            None
        };
        let edge = issue.is_none() && (spread[0] > EDGE_HUE || spread[2] > EDGE_LIGHT);
        Ok(Sample { at, color, hue, spread, window, issue, edge })
    }

    fn json(&self, used: bool) -> Value {
        let mut v = json!({
            "at": self.at,
            "color": hex(self.color),
            "lightness": r3(self.color[0] as f64),
            "chroma": r3(chroma(self.color) as f64),
            "hue": r1(self.hue.rem_euclid(360.0) as f64),
            "skin_like": self.issue.is_none(),
            "used": used,
        });
        if let Some(i) = &self.issue {
            v["issue"] = json!(i);
        } else if self.edge {
            v["issue"] = json!("on an edge (the area around it is not uniform)");
        }
        v
    }
}

/// The skin model: ranges of hue (offsets from `h0`), ln saturation, and ln lightness,
/// with the half-membership distance beyond each.
struct Model {
    h0: f32,
    hue: (f32, f32),
    sat: (f32, f32),
    light: (f32, f32),
    hue_tol: f32,
    sat_tol: f32,
    light_tol: f32,
}

impl Model {
    fn fit<'a>(used: impl Iterator<Item = &'a Sample> + Clone, o: &SkinOptions) -> Model {
        // circular mean of the hues (summed in sample order)
        let (sx, sy) = used.clone().fold((0f32, 0f32), |(x, y), s| {
            let r = s.hue.to_radians();
            (x + r.cos(), y + r.sin())
        });
        let h0 = if sx.abs() + sy.abs() > 1e-6 {
            sy.atan2(sx).to_degrees()
        } else {
            used.clone().next().map_or(0.0, |s| s.hue)
        };
        let range = |f: &dyn Fn(&Sample) -> f32| {
            used.clone().map(f).fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), v| (lo.min(v), hi.max(v)))
        };
        let spread = |k: usize| used.clone().map(|s| s.spread[k]).fold(0f32, f32::max);
        let hue = range(&|s| wrap(s.hue - h0));
        let sat = range(&|s| ln_sat(s.color));
        let light = range(&|s| ln_light(s.color));
        let (t, l) = (o.tolerance.clamp(0.0, 100.0), o.lightness.clamp(0.0, 100.0));
        Model {
            h0,
            hue,
            sat,
            light: (light.0 - (0.1 + 0.008 * l), light.1 + (0.1 + 0.006 * l)),
            hue_tol: 4.0 + 0.2 * t + (1.5 * spread(0)).min(MAX_SPREAD_HUE),
            sat_tol: 0.15 + 0.006 * t + (1.5 * spread(1)).min(MAX_SPREAD_LOG),
            light_tol: LIGHT_TOL + (1.5 * spread(2)).min(MAX_SPREAD_LOG),
        }
    }

    /// Hue excess in tolerances (0 for colors too neutral to have a hue).
    #[inline]
    fn hue_excess(&self, c: Lab) -> f32 {
        if chroma(c) < 1e-5 {
            return 0.0;
        }
        excess(wrap(hue_deg(c) - self.h0), self.hue) / self.hue_tol
    }

    /// Skin membership 0..1.
    #[inline]
    fn member(&self, c: Lab) -> f32 {
        if c[0] <= MIN_L {
            return 0.0;
        }
        let es = excess(ln_sat(c), self.sat) / self.sat_tol;
        let el = excess(c[0].ln(), self.light) / self.light_tol;
        let eh = self.hue_excess(c);
        let d2 = es * es + el * el + eh * eh;
        if d2 <= 0.25 {
            1.0
        } else if d2 >= 2.25 {
            0.0
        } else {
            let t = d2.sqrt() - 0.5;
            1.0 - t * t * (3.0 - 2.0 * t)
        }
    }

    /// Steep membership for the refinement guide: 1 up to 0.8 tolerances, 0 from 1.2, so
    /// colors that are only a little skin-like (lips, mixed hair) do not pass through the
    /// guided filter's linear model.
    #[inline]
    fn edge(&self, c: Lab) -> f32 {
        if c[0] <= MIN_L {
            return 0.0;
        }
        let es = excess(ln_sat(c), self.sat) / self.sat_tol;
        let el = excess(c[0].ln(), self.light) / self.light_tol;
        let eh = self.hue_excess(c);
        let d2 = es * es + el * el + eh * eh;
        if d2 <= 0.64 {
            1.0
        } else if d2 >= 1.44 {
            0.0
        } else {
            let t = (d2.sqrt() - 0.8) / 0.4;
            1.0 - t * t * (3.0 - 2.0 * t)
        }
    }

    /// Clearly another hue (lips, colored objects): saturated enough to have one, and
    /// more than two tolerances outside the range.
    #[inline]
    fn off_hue(&self, c: Lab) -> bool {
        ln_sat(c) >= self.sat.0 - std::f32::consts::LN_2 && self.hue_excess(c) > 2.0
    }

    /// `{hue, saturation, lightness}` ranges at half membership (degrees, C/L, OKLab L).
    fn json(&self) -> Value {
        let deg = |d: f32| r1((self.h0 + d).rem_euclid(360.0) as f64);
        json!({
            "hue": [deg(self.hue.0 - self.hue_tol), deg(self.hue.1 + self.hue_tol)],
            "saturation": [r3((self.sat.0 - self.sat_tol).exp() as f64), r3((self.sat.1 + self.sat_tol).exp() as f64)],
            "lightness": [
                r3((self.light.0 - self.light_tol).exp() as f64),
                r3((self.light.1 + self.light_tol).exp().min(1.0) as f64),
            ],
        })
    }
}

// ---------------------------------------------------------------------------------------
// clean-up

/// With `--contiguous off`: drop skin components up to `max_area` px that hold no sample
/// (specks, irises, skin-colored bits of the background). Returns how many.
fn drop_islands(skin: &mut [bool], seeds: &[usize], w: usize, h: usize, max_area: f64) -> usize {
    let mut seed = vec![false; w * h];
    for &s in seeds {
        seed[s] = true;
    }
    let mut off = Vec::new();
    let mut n = 0;
    for_each_component(
        skin,
        w,
        h,
        |_, _| true,
        |comp, _| {
            if comp.len() as f64 <= max_area && !comp.iter().any(|&i| seed[i]) {
                n += 1;
                off.extend_from_slice(comp);
            }
        },
    );
    for i in off {
        skin[i] = false;
    }
    n
}

/// Fill holes (non-skin components that do not reach the canvas edge): up to `speck` px
/// unless they hold part of an eye or a mouth (blemishes, moles, pores, nostrils), up to
/// `highlight` px when they look like a specular highlight. Returns (specks, highlights).
fn fill_holes(
    skin: &mut [bool],
    c: &[Lab],
    model: &Model,
    w: usize,
    h: usize,
    speck: f64,
    highlight: f64,
) -> (usize, usize) {
    let off: Vec<bool> = skin.iter().map(|&s| !s).collect();
    let mut mark = vec![false; w * h];
    let mut add = Vec::new();
    let (mut specks, mut highlights) = (0, 0);
    let cur: &[bool] = skin;
    for_each_component(
        &off,
        w,
        h,
        |_, _| true,
        |comp, border| {
            let n = comp.len() as f64;
            if border || n > highlight {
                return;
            }
            let hole = Hole::measure(comp, cur, c, model, w, h, &mut mark);
            if n <= speck && !hole.feature() {
                specks += 1;
            } else if hole.highlight() {
                highlights += 1;
            } else {
                return;
            }
            add.extend_from_slice(comp);
        },
    );
    for i in add {
        skin[i] = true;
    }
    (specks, highlights)
}

/// What a hole holds, as fractions of its pixels, against the skin around it.
struct Hole {
    /// Darker than [`DARK_RATIO`] times the surrounding skin (pupil, iris, lashes, nostril).
    dark: f64,
    /// Lighter than the surrounding skin and nearly neutral (sclera, teeth).
    white: f64,
    /// Well outside the hue range (lips, colored objects).
    off_hue: f64,
    /// Mean lightness over the surrounding skin's median.
    brightness: f64,
    /// Fraction of the surrounding skin darker than [`DARK_RATIO`] times its median: an
    /// iris or a shadow next to the hole, where a highlight sits in plain skin.
    dark_ring: f64,
    /// Length over width (from the second moments): brows, lash lines, and lip lines are
    /// long, blemishes and highlights compact.
    elongation: f64,
}

impl Hole {
    fn measure(comp: &[usize], skin: &[bool], c: &[Lab], model: &Model, w: usize, h: usize, mark: &mut [bool]) -> Hole {
        let mut ring = Vec::new();
        let mut touched = Vec::new();
        for &p in comp {
            for q in neighbors(p, w, h) {
                if skin[q] && !mark[q] {
                    mark[q] = true;
                    touched.push(q);
                    ring.push(c[q][0]);
                }
            }
        }
        for q in touched {
            mark[q] = false;
        }
        if ring.is_empty() {
            return Hole { dark: 1.0, white: 0.0, off_hue: 0.0, brightness: 0.0, dark_ring: 1.0, elongation: 1.0 };
        }
        let around = quantile(&mut ring, 0.5);
        let dark_ring = ring.iter().filter(|&&l| l < DARK_RATIO * around).count() as f64 / ring.len() as f64;
        let n = comp.len() as f64;
        let frac = |f: &dyn Fn(Lab) -> bool| comp.iter().filter(|&&i| f(c[i])).count() as f64 / n;
        let neutral = model.sat.0 - std::f32::consts::LN_2;
        Hole {
            dark: frac(&|p| p[0] < DARK_RATIO * around),
            white: frac(&|p| p[0] > around && ln_sat(p) < neutral),
            off_hue: frac(&|p| model.off_hue(p)),
            brightness: comp.iter().map(|&i| c[i][0] as f64).sum::<f64>() / n / around.max(MIN_L) as f64,
            dark_ring,
            elongation: elongation(comp, w),
        }
    }

    /// Part of an eye (sclera), a mouth (teeth, lips), or a brow or lash line (long): never
    /// filled.
    fn feature(&self) -> bool {
        self.white >= FEATURE_WHITE || self.off_hue >= FEATURE_OFF_HUE || self.elongation >= FEATURE_LONG
    }

    /// A specular highlight: brighter on average than the plain skin around it, with almost
    /// no pixels darker than that skin and almost none of another hue.
    fn highlight(&self) -> bool {
        self.dark <= DARK_MAX
            && self.off_hue <= OFF_HUE_MAX
            && self.brightness >= 1.02
            && self.dark_ring <= DARK_RING_MAX
            && self.elongation < FEATURE_LONG
    }
}

/// Length over width of a pixel set: the square root of its coordinate covariance's
/// eigenvalue ratio (an ellipse's axis ratio).
fn elongation(comp: &[usize], w: usize) -> f64 {
    let n = comp.len() as f64;
    let (mut sx, mut sy) = (0f64, 0f64);
    for &i in comp {
        sx += (i % w) as f64;
        sy += (i / w) as f64;
    }
    let (mx, my) = (sx / n, sy / n);
    let (mut xx, mut yy, mut xy) = (0f64, 0f64, 0f64);
    for &i in comp {
        let (dx, dy) = ((i % w) as f64 - mx, (i / w) as f64 - my);
        xx += dx * dx;
        yy += dy * dy;
        xy += dx * dy;
    }
    // a pixel is a unit square: add its own variance so tiny sets are not degenerate
    let (xx, yy, xy) = (xx / n + 1.0 / 12.0, yy / n + 1.0 / 12.0, xy / n);
    let (mean, diff) = ((xx + yy) / 2.0, (((xx - yy) / 2.0).powi(2) + xy * xy).sqrt());
    ((mean + diff) / (mean - diff).max(1e-9)).sqrt()
}

/// Per-channel 3x3 median of color planes (edges clamped).
fn median_planes(c: &[Lab], w: usize, h: usize) -> Vec<Lab> {
    let mut out = vec![[0f32; 3]; w * h];
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let ys = [y.saturating_sub(1), y, (y + 1).min(h - 1)];
        for (x, o) in row.iter_mut().enumerate() {
            let xs = [x.saturating_sub(1), x, (x + 1).min(w - 1)];
            for (ch, v) in o.iter_mut().enumerate() {
                let mut win = [0f32; 9];
                for (j, &yy) in ys.iter().enumerate() {
                    for (i, &xx) in xs.iter().enumerate() {
                        win[j * 3 + i] = c[yy * w + xx][ch];
                    }
                }
                *v = *win.select_nth_unstable_by(4, f32::total_cmp).1;
            }
        }
    });
    out
}

/// Coverage of a mask in percent, summed row by row in order.
fn mask_cover(m: &Mask) -> f64 {
    crate::util::mask_stats(m).coverage_pct
}

// ---------------------------------------------------------------------------------------
// result

fn r2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

/// What the detection found, for the result.
struct Found<'a> {
    samples: &'a [Sample],
    used: &'a [bool],
    model: &'a Model,
    /// Seed pixels (sample windows that matched the model).
    seeds: usize,
    /// Skin-colored area left out (not connected, or dropped islands), per selected px.
    left_out: f64,
    /// [specks filled, highlights filled, islands removed].
    counts: [usize; 3],
    /// Final skin coverage in percent of the canvas.
    coverage: f64,
}

impl Found<'_> {
    fn advice(&self, o: &SkinOptions) -> Vec<String> {
        let mut out = Vec::new();
        for (i, (s, &u)) in self.samples.iter().zip(self.used).enumerate() {
            let name = format!("sample {} at {},{}", i + 1, s.at[0], s.at[1]);
            match (&s.issue, u) {
                (Some(why), false) => {
                    out.push(format!("{name} does not look like skin ({why}) and was left out of the model; move it onto skin"))
                }
                (Some(why), true) => out.push(format!("{name} does not look like skin ({why}); move --at onto plain skin")),
                (None, _) if s.edge => out.push(format!(
                    "{name} is on an edge (the area around it is not uniform), which widens the model; move it onto plain skin"
                )),
                _ => {}
            }
        }
        let m = self.model;
        if m.hue.1 - m.hue.0 > DISAGREE_HUE {
            out.push(format!(
                "the samples' hues differ by {:.0}° (is one on lips, hair, or under colored light?): the model spans them all, so check the --at points",
                m.hue.1 - m.hue.0
            ));
        }
        if m.sat.1 - m.sat.0 > DISAGREE_SAT {
            out.push(format!(
                "the samples' saturations differ {:.1}x (is one on lips, a highlight, or a shadow?): check the --at points",
                (m.sat.1 - m.sat.0).exp()
            ));
        }
        if self.seeds == 0 {
            out.push("no pixels around the samples match the model; move --at onto plain skin".to_string());
        } else if self.coverage < 0.2 {
            out.push("very little skin was selected; check the --at points or raise --tolerance".to_string());
        } else if self.coverage > 60.0 {
            out.push(format!(
                "the skin covers {:.0}% of the canvas; if it leaked into a skin-colored background or clothing, lower --tolerance or --lightness",
                self.coverage
            ));
        }
        if o.contiguous && self.left_out > 0.25 {
            out.push(format!(
                "skin-colored area {:.0}% the size of the selection is not connected to the samples (hands, another person, or skin cut off by hair or glasses); if it is skin, add --at points on it or use --contiguous off",
                self.left_out * 100.0
            ));
        }
        out
    }

    fn describe(&self, o: &SkinOptions, holes_px: f64, refine_px: f64, size: [usize; 2]) -> Value {
        let [specks, highlights, islands] = self.counts;
        let samples: Vec<Value> = self.samples.iter().zip(self.used).map(|(s, &u)| s.json(u)).collect();
        let mut v = json!({
            "samples": samples,
            "model": self.model.json(),
            "contiguous": o.contiguous,
            "skin_colored_left_out": r3(self.left_out),
            "holes_filled": specks,
            "highlights_filled": highlights,
            "islands_removed": islands,
            "holes": r2(holes_px),
            "refine": r2(refine_px),
            "analysis_size": size,
        });
        let advice = self.advice(o);
        if !advice.is_empty() {
            v["advice"] = json!(advice.join("; "));
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing;

    const W: u32 = 160;
    const H: u32 = 200;
    /// A point on the lit cheek, clear of the blemishes.
    const CHEEK: &str = "58,96";
    /// A point on the neck (in the chin's shadow).
    const NECK: &str = "70,160";

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

    fn lin(c: [f32; 3]) -> [f32; 3] {
        c.map(color::srgb_to_linear)
    }

    fn scale(c: [f32; 3], k: [f32; 3]) -> [f32; 3] {
        [c[0] * k[0], c[1] * k[1], c[2] * k[2]]
    }

    fn times(c: [f32; 3], k: f32) -> [f32; 3] {
        c.map(|v| v * k)
    }

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Part {
        Background,
        Cloth,
        Hair,
        Skin,
        Brow,
        Eye,
        Lips,
        Teeth,
        Hand,
    }

    const PARTS: [Part; 9] = [
        Part::Background,
        Part::Cloth,
        Part::Hair,
        Part::Skin,
        Part::Brow,
        Part::Eye,
        Part::Lips,
        Part::Teeth,
        Part::Hand,
    ];

    fn ell(x: f32, y: f32, cx: f32, cy: f32, rx: f32, ry: f32) -> f32 {
        ((x - cx) / rx).powi(2) + ((y - cy) / ry).powi(2)
    }

    fn smooth(e0: f32, e1: f32, v: f32) -> f32 {
        let t = ((v - e0) / (e1 - e0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    const BLEMISHES: [(f32, f32); 3] = [(50.0, 106.0), (98.0, 97.0), (104.0, 105.0)];
    const BLEMISH_R: f32 = 2.0;
    const HIGHLIGHT: (f32, f32) = (80.0, 101.0);

    #[derive(Clone, Copy)]
    struct Look {
        /// Lit skin color (sRGB).
        skin: [f32; 3],
        /// Black hair and very dark irises (else dark brown).
        dark_hair: bool,
        /// Light on the shadow side, linear (the lit side gets 1).
        shadow: f32,
    }

    /// A portrait: an oval face lit from the left, a neck in the chin's shadow, hair with a
    /// hairline, brows, eyes (sclera, iris, pupil), red lips around white teeth, three small
    /// dark blemishes, a specular highlight on the nose, a separate hand, a shirt, and a
    /// blue-gray backdrop. Returns (part, linear color) at a sub-pixel position.
    fn portrait_at(x: f32, y: f32, look: Look) -> (Part, [f32; 3]) {
        let skin = lin(look.skin);
        let hair = lin(if look.dark_hair { [0.086, 0.071, 0.063] } else { [0.23, 0.165, 0.125] });
        let iris = lin(if look.dark_hair { [0.10, 0.067, 0.047] } else { [0.227, 0.141, 0.094] });
        let face = ell(x, y, 80.0, 92.0, 42.0, 56.0);
        let light = 1.0 - (1.0 - look.shadow) * smooth(-15.0, 25.0, x - 80.0);
        if ell(x, y, 140.0, 172.0, 12.0, 17.0) < 1.0 {
            return (Part::Hand, times(skin, 0.85));
        }
        let hairline = 50.0 + 0.012 * (x - 80.0).powi(2);
        if (face < 1.0 && y < hairline) || (face >= 1.0 && ell(x, y, 80.0, 84.0, 50.0, 62.0) < 1.0 && y < 125.0) {
            let streak = 0.8 + 0.4 * (noise((x * 0.5) as u32, (y * 0.1) as u32, 5) + 0.5);
            return (Part::Hair, times(hair, streak));
        }
        if face < 1.0 {
            let shade = light * (1.0 - 0.25 * face * face);
            for cx in [64.0, 96.0] {
                if ell(x, y, cx, 71.0, 10.0, 2.2) < 1.0 {
                    return (Part::Brow, times(hair, shade));
                }
                if ell(x, y, cx, 82.0, 8.0, 3.5) < 1.0 {
                    let d = ((x - cx).powi(2) + (y - 82.0).powi(2)).sqrt();
                    let c = if d < 1.3 {
                        lin([0.03, 0.03, 0.03])
                    } else if d < 3.0 {
                        iris
                    } else {
                        lin([0.90, 0.88, 0.855])
                    };
                    return (Part::Eye, times(c, shade));
                }
            }
            if ell(x, y, 80.0, 118.0, 14.0, 5.5) < 1.0 {
                if ell(x, y, 80.0, 118.5, 9.0, 2.2) < 1.0 {
                    return (Part::Teeth, times(lin([0.925, 0.89, 0.84]), shade * 0.9));
                }
                return (Part::Lips, times(scale(skin, [0.8, 0.36, 0.42]), shade));
            }
            let mut c = times(skin, shade);
            if BLEMISHES.iter().any(|&(bx, by)| ell(x, y, bx, by, BLEMISH_R, BLEMISH_R) < 1.0) {
                c = scale(c, [0.5, 0.38, 0.38]);
            }
            let (hx, hy) = HIGHLIGHT;
            let g = 0.6 * (-((x - hx).powi(2) + (y - hy).powi(2)) / (2.0 * 2.2 * 2.2)).exp();
            return (Part::Skin, c.map(|v| v + g));
        }
        if (62.0..98.0).contains(&x) && y > 120.0 && y < 184.0 {
            return (Part::Skin, times(skin, 0.7 * light));
        }
        if y > 168.0 + 0.8 * (18.0 - (x - 80.0).abs()).max(0.0) {
            return (Part::Cloth, lin([0.10, 0.26, 0.30]));
        }
        let t = y / H as f32;
        (Part::Background, lin([0.36 + 0.08 * t, 0.44 + 0.08 * t, 0.56 + 0.06 * t]))
    }

    struct Portrait {
        img: Image,
        /// Coverage per part, indexed like [`PARTS`].
        parts: Vec<Mask>,
    }

    impl Portrait {
        fn of(look: Look) -> Portrait {
            // 2x2 supersampled, averaged in linear light, a little sensor noise in sRGB
            let n = (W * H) as usize;
            let mut cover = vec![vec![0f32; n]; PARTS.len()];
            let mut img = Image::new(W, H);
            for y in 0..H {
                for x in 0..W {
                    let mut acc = [0f32; 3];
                    for (sx, sy) in [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
                        let (part, c) = portrait_at(x as f32 + sx, y as f32 + sy, look);
                        cover[PARTS.iter().position(|&p| p == part).unwrap()][(y * W + x) as usize] += 0.25;
                        (0..3).for_each(|k| acc[k] += c[k] / 4.0);
                    }
                    let px: Vec<f32> = (0..3)
                        .map(|k| (color::linear_to_srgb(acc[k]) + 0.02 * noise(x, y, k as u32 + 1)).clamp(0.0, 1.0))
                        .collect();
                    img.set(x, y, [px[0], px[1], px[2], 1.0]);
                }
            }
            let parts = cover.into_iter().map(|data| Mask { width: W, height: H, data }).collect();
            Portrait { img, parts }
        }

        fn part(&self, p: Part) -> &Mask {
            &self.parts[PARTS.iter().position(|&q| q == p).unwrap()]
        }

        fn doc(&self) -> Document {
            testing::doc_from(self.img.clone())
        }
    }

    const TONES: [(&str, [f32; 3]); 7] = [
        ("very light", [0.965, 0.843, 0.784]),
        ("light", [0.91, 0.725, 0.63]),
        ("medium", [0.776, 0.533, 0.388]),
        ("tan", [0.66, 0.45, 0.31]),
        ("brown", [0.48, 0.29, 0.196]),
        ("dark", [0.29, 0.172, 0.122]),
        ("very dark", [0.18, 0.11, 0.078]),
    ];
    const MEDIUM: usize = 2;
    const VERY_DARK: usize = 6;

    fn look(tone: usize) -> Look {
        Look { skin: TONES[tone].1, dark_hair: tone >= 4, shadow: 0.3 }
    }

    fn portrait(tone: usize) -> Portrait {
        Portrait::of(look(tone))
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

    /// Mean of `m` over the pixels fully inside `region` (all four subsamples).
    fn mean_in(m: &Mask, region: &Mask) -> f64 {
        let (mut s, mut n) = (0f64, 0usize);
        for (&v, &r) in m.data.iter().zip(&region.data) {
            if r >= 1.0 {
                s += v as f64;
                n += 1;
            }
        }
        assert!(n > 0, "empty region");
        s / n as f64
    }

    /// The pixels fully inside `region` whose 8 neighbors are too (a 1 px erosion): the
    /// interior, where a correct selection is exactly 0 or 1 whatever its edge softness.
    fn core(region: &Mask) -> Mask {
        let (w, h) = (region.width as i64, region.height as i64);
        Mask::from_fn(region.width, region.height, |x, y| {
            let all = (-1..=1).all(|dy| {
                (-1..=1).all(|dx| {
                    let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                    nx >= 0 && ny >= 0 && nx < w && ny < h && region.get(nx as u32, ny as u32) >= 1.0
                })
            });
            if all {
                1.0
            } else {
                0.0
            }
        })
    }

    fn mean_near(m: &Mask, (cx, cy): (f32, f32), r: f32) -> f64 {
        let (mut s, mut n) = (0f64, 0usize);
        for y in 0..m.height {
            for x in 0..m.width {
                if (x as f32 + 0.5 - cx).powi(2) + (y as f32 + 0.5 - cy).powi(2) <= r * r {
                    s += m.get(x, y) as f64;
                    n += 1;
                }
            }
        }
        s / n as f64
    }

    fn spec() -> OpSpec {
        testing::find(&crate::ops(), "select skin")
    }

    fn run(pairs: &[(&str, &str)], doc: &mut Document) -> Result<Value> {
        testing::run(&spec(), pairs, doc)
    }

    fn sel(doc: &Document) -> &Mask {
        doc.selection.as_ref().expect("a selection")
    }

    /// The shadow side of the skin (right of x = 100).
    fn shadow_side(p: &Portrait) -> Mask {
        let skin = p.part(Part::Skin);
        Mask::from_fn(W, H, |x, y| if x > 100 { skin.get(x, y) } else { 0.0 })
    }

    /// Interior selection limits for what must stay out: the small features (soft edges
    /// reach one or two px into them) and the large areas.
    const EXCLUDED: [(Part, f64); 8] = [
        (Part::Eye, 0.05),
        (Part::Brow, 0.06),
        (Part::Lips, 0.1),
        (Part::Teeth, 0.02),
        (Part::Hair, 0.01),
        (Part::Background, 0.005),
        (Part::Cloth, 0.005),
        (Part::Hand, 0.005),
    ];

    fn check_tone(tone: usize) {
        let (name, _) = TONES[tone];
        let p = portrait(tone);
        let mut d = p.doc();
        let v = run(&[("at", CHEEK)], &mut d).unwrap();
        let s = sel(&d);
        let report = |what: &str| format!("{name}: {what}; result {v}");
        let score = iou(s, p.part(Part::Skin));
        assert!(score >= 0.9, "{}", report(&format!("IoU {score:.3}")));
        let shadow = mean_in(s, &shadow_side(&p));
        assert!(shadow > 0.97, "{}", report(&format!("shadow side {shadow:.3}")));
        for b in BLEMISHES {
            let m = mean_near(s, b, BLEMISH_R);
            assert!(m > 0.9, "{}", report(&format!("blemish at {b:?}: {m:.3}")));
        }
        let m = mean_near(s, HIGHLIGHT, 2.5);
        assert!(m > 0.9, "{}", report(&format!("highlight: {m:.3}")));
        for (part, most) in EXCLUDED {
            let m = mean_in(s, &core(p.part(part)));
            assert!(m <= most, "{}", report(&format!("{part:?} selected {m:.3}")));
        }
        assert!(v.get("advice").is_none(), "{}", report("advice"));
        let sample = &v["samples"][0];
        assert_eq!(sample["at"], json!([58, 96]));
        assert_eq!((sample["skin_like"].clone(), sample["used"].clone()), (json!(true), json!(true)));
        // the model's half-membership ranges contain the sample
        let hue = sample["hue"].as_f64().unwrap();
        let [lo, hi] = [0, 1].map(|i| v["model"]["hue"][i].as_f64().unwrap());
        assert!(lo < hue && hue < hi, "{}", report("hue range"));
        let l = sample["lightness"].as_f64().unwrap();
        let [llo, lhi] = [0, 1].map(|i| v["model"]["lightness"][i].as_f64().unwrap());
        assert!(llo < 0.65 * l && lhi > l, "{}", report("lightness range"));
        assert_eq!(v["selection"], v["skin"], "replace mode: the selection is the skin");
    }

    #[test]
    fn very_light_and_light_skin() {
        check_tone(0);
        check_tone(1);
    }

    #[test]
    fn medium_and_tan_skin() {
        check_tone(2);
        check_tone(3);
    }

    #[test]
    fn brown_and_dark_skin() {
        check_tone(4);
        check_tone(5);
    }

    #[test]
    fn very_dark_skin() {
        check_tone(VERY_DARK);
    }

    #[test]
    fn result_reports_samples_model_and_clean_up() {
        let p = portrait(MEDIUM);
        let mut d = p.doc();
        let v = run(&[("at", CHEEK), ("at", NECK)], &mut d).unwrap();
        assert!(iou(sel(&d), p.part(Part::Skin)) >= 0.9, "{v}");
        assert_eq!(v["samples"].as_array().unwrap().len(), 2);
        assert_eq!(v["samples"][1]["at"], json!([70, 160]));
        assert!(v["samples"][0]["color"].as_str().unwrap().starts_with('#'));
        // the neck is darker than the cheek: the model's lightness range spans both
        let (cheek, neck) =
            (v["samples"][0]["lightness"].as_f64().unwrap(), v["samples"][1]["lightness"].as_f64().unwrap());
        assert!(neck < cheek, "{v}");
        assert!(v["model"]["lightness"][0].as_f64().unwrap() < 0.65 * neck, "{v}");
        assert_eq!(v["islands_removed"], json!(0));
        assert!(v["holes"].as_f64().unwrap() > 0.0 && v["refine"].as_f64().unwrap() == 3.0, "{v}");
        assert_eq!(v["analysis_size"], json!([W, H]));
        assert_eq!(v["contiguous"], json!(true));
        // the hand is skin-colored but not connected
        let left = v["skin_colored_left_out"].as_f64().unwrap();
        assert!((0.03..0.2).contains(&left), "{v}");
        let cov = v["skin"]["coverage"].as_f64().unwrap();
        let truth = p.part(Part::Skin).data.iter().map(|&g| g as f64).sum::<f64>() / (W * H) as f64 * 100.0;
        assert!((cov - truth).abs() < 1.5, "coverage {cov} vs {truth}");
        // on very light skin the blemishes are holes the clean-up filled, and the nose's
        // highlight a highlight hole
        let mut d = portrait(0).doc();
        let v = run(&[("at", CHEEK)], &mut d).unwrap();
        assert!(v["holes_filled"].as_u64().unwrap() >= 3, "{v}");
        assert!(v["highlights_filled"].as_u64().unwrap() >= 1, "{v}");
    }

    #[test]
    fn holes_zero_leaves_blemishes_and_highlights_out() {
        let p = portrait(VERY_DARK);
        let mut d = p.doc();
        let v = run(&[("at", CHEEK), ("holes", "0")], &mut d).unwrap();
        assert_eq!((v["holes_filled"].clone(), v["highlights_filled"].clone()), (json!(0), json!(0)));
        assert!(mean_near(sel(&d), HIGHLIGHT, 1.5) < 0.2, "{v}");
        let blemish = BLEMISHES.iter().map(|&b| mean_near(sel(&d), b, 1.0)).fold(1.0, f64::min);
        assert!(blemish < 0.5, "a blemish center stays out: {blemish}");
    }

    #[test]
    fn contiguous_off_adds_separate_skin() {
        for tone in [MEDIUM, VERY_DARK] {
            let p = portrait(tone);
            let mut d = p.doc();
            run(&[("at", CHEEK)], &mut d).unwrap();
            let hand = core(p.part(Part::Hand));
            assert!(mean_in(sel(&d), &hand) < 0.005, "contiguous: the hand is not selected");
            let v = run(&[("at", CHEEK), ("contiguous", "off")], &mut d).unwrap();
            let s = sel(&d);
            assert!(mean_in(s, &hand) > 0.97, "{v}");
            assert_eq!(v["contiguous"], json!(false));
            // the face is still right, and skin-colored irises (very dark skin) are dropped
            // as islands rather than selected
            assert!(iou(s, p.part(Part::Skin)) > 0.85, "{v}");
            for (part, most) in EXCLUDED.iter().filter(|(p, _)| *p != Part::Hand) {
                let m = mean_in(s, &core(p.part(*part)));
                assert!(m <= *most, "{} --contiguous off: {part:?} selected {m:.3}; {v}", TONES[tone].0);
            }
        }
    }

    #[test]
    fn deep_shadow_needs_lightness_or_a_second_sample() {
        // the shadow side gets 1/12 of the light (3.6 stops); black hair (dark brown hair
        // has deeply shadowed skin's hue and saturation: a sample there admits it too)
        let p = Portrait::of(Look { shadow: 1.0 / 12.0, dark_hair: true, ..look(MEDIUM) });
        let deep = core(&Mask::from_fn(W, H, |x, y| if x > 108 { p.part(Part::Skin).get(x, y) } else { 0.0 }));
        let mut d = p.doc();
        let v = run(&[("at", CHEEK)], &mut d).unwrap();
        let by_default = mean_in(sel(&d), &deep);
        assert!(by_default < 0.5, "default lightness stops short of a 3.6-stop shadow: {by_default}; {v}");
        run(&[("at", CHEEK), ("lightness", "100")], &mut d).unwrap();
        assert!(mean_in(sel(&d), &deep) > 0.97);
        run(&[("at", CHEEK), ("at", "112,96")], &mut d).unwrap();
        assert!(mean_in(sel(&d), &deep) > 0.97);
        for (part, most) in EXCLUDED {
            let m = mean_in(sel(&d), &core(p.part(part)));
            assert!(m <= most, "{part:?} selected {m:.3}");
        }
    }

    #[test]
    fn non_skin_samples_get_advice() {
        let p = portrait(MEDIUM);
        // a stray point on the backdrop: reported and left out of the model
        let mut d = p.doc();
        let v = run(&[("at", CHEEK), ("at", "10,10")], &mut d).unwrap();
        let bad = &v["samples"][1];
        assert_eq!((bad["skin_like"].clone(), bad["used"].clone()), (json!(false), json!(false)));
        assert!(bad["issue"].as_str().unwrap().contains("hue"), "{v}");
        let advice = v["advice"].as_str().unwrap();
        assert!(advice.contains("sample 2 at 10,10") && advice.contains("left out"), "{advice}");
        assert!(iou(sel(&d), p.part(Part::Skin)) >= 0.9, "the stray sample does not spoil the selection");
        // only a point on a white wall: used (nothing better), with advice
        let wall =
            Image::from_fn(40, 40, |x, _| if x < 20 { [0.93, 0.93, 0.92, 1.0] } else { [0.78, 0.53, 0.39, 1.0] });
        let mut w = testing::doc_from(wall);
        let v = run(&[("at", "5,5")], &mut w).unwrap();
        assert_eq!(v["samples"][0]["used"], json!(true), "{v}");
        let advice = v["advice"].as_str().expect("advice");
        assert!(advice.contains("sample 1") && advice.contains("neutral"), "{advice}");
        // a cheek and a lip sample disagree on hue
        let v = run(&[("at", CHEEK), ("at", "80,114")], &mut d).unwrap();
        assert!(v["advice"].as_str().unwrap_or("").contains("hues differ"), "{v}");
    }

    #[test]
    fn modes_combine_with_the_selection() {
        let p = portrait(MEDIUM);
        let mut d = p.doc();
        let (cheek, bg) = ((58, 96), (10, 10));
        // add: a backdrop corner plus the skin
        testing::run(&testing::find(&crate::ops(), "select rect"), &[("rect", "0,0,20x20")], &mut d).unwrap();
        run(&[("at", CHEEK), ("mode", "add")], &mut d).unwrap();
        assert_eq!(sel(&d).get(bg.0, bg.1), 1.0);
        assert_eq!(sel(&d).get(cheek.0, cheek.1), 1.0);
        assert_eq!(sel(&d).get(100, 30), 0.0);
        // subtract: everything but the skin
        testing::run(&testing::find(&crate::ops(), "select all"), &[], &mut d).unwrap();
        run(&[("at", CHEEK), ("mode", "subtract")], &mut d).unwrap();
        assert_eq!(sel(&d).get(cheek.0, cheek.1), 0.0);
        assert_eq!(sel(&d).get(bg.0, bg.1), 1.0);
        assert!(mean_in(sel(&d), &core(p.part(Part::Eye))) > 0.95, "the eyes stay selected");
        // intersect: the skin in the left half
        testing::run(&testing::find(&crate::ops(), "select rect"), &[("rect", "0,0,80x200")], &mut d).unwrap();
        run(&[("at", CHEEK), ("mode", "intersect")], &mut d).unwrap();
        assert_eq!(sel(&d).get(cheek.0, cheek.1), 1.0);
        assert_eq!(sel(&d).get(100, 100), 0.0);
        assert_eq!(sel(&d).get(bg.0, bg.1), 0.0);
    }

    #[test]
    fn feather_softens_and_keeps_the_area() {
        let p = portrait(MEDIUM);
        let mut a = p.doc();
        let mut b = a.clone();
        run(&[("at", CHEEK)], &mut a).unwrap();
        run(&[("at", CHEEK), ("feather", "4")], &mut b).unwrap();
        let soft = |m: &Mask| m.data.iter().filter(|&&v| v > 0.02 && v < 0.98).count();
        assert!(soft(sel(&b)) > 2 * soft(sel(&a)));
        let (x, y) = (sel(&a).data.iter().sum::<f32>(), sel(&b).data.iter().sum::<f32>());
        assert!((x - y).abs() / x < 0.03, "feathering keeps the area: {x} vs {y}");
        // refine 0: a hard mask
        run(&[("at", CHEEK), ("refine", "0")], &mut a).unwrap();
        assert!(sel(&a).data.iter().all(|&v| v == 0.0 || v == 1.0));
        assert!(iou(sel(&a), p.part(Part::Skin)) >= 0.9);
    }

    #[test]
    fn deterministic_across_runs_and_thread_counts() {
        let img = portrait(VERY_DARK).img;
        let pairs = [("at", CHEEK), ("at", NECK), ("contiguous", "off"), ("feather", "2")];
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

    fn options(points: Vec<[u32; 2]>) -> SkinOptions {
        SkinOptions { points, tolerance: 50.0, lightness: 50.0, contiguous: true, holes: None, refine: None }
    }

    #[test]
    fn recorded_auto_values_replay_exactly() {
        // the auto clean-up size and refine radius are recorded as text for replay; they
        // must rebuild the same mask (the analysis is scaled, so neither is a round number)
        let img = portrait(MEDIUM).img;
        let o = options(vec![[58, 96]]);
        let auto = detect(&img, &o, 130).unwrap();
        let holes: f64 = auto.holes_px.to_string().parse().unwrap();
        let refine: f64 = auto.refine_px.to_string().parse().unwrap();
        let again = detect(&img, &SkinOptions { holes: Some(holes), refine: Some(refine), ..o }, 130).unwrap();
        assert!(auto.mask.data == again.mask.data, "replayed holes {holes} / refine {refine} changed the mask");
        assert_eq!(auto.info, again.info);
    }

    #[test]
    fn large_images_are_analyzed_downscaled() {
        let p = portrait(MEDIUM);
        let det = detect(&p.img, &options(vec![[58, 96]]), 100).unwrap();
        assert_eq!(det.info["analysis_size"], json!([80, 100]));
        assert_eq!((det.mask.width, det.mask.height), (W, H));
        assert!((det.refine_px - 6.0).abs() < 1e-9, "auto refine is 3 analysis px: {}", det.refine_px);
        let score = iou(&det.mask, p.part(Part::Skin));
        assert!(score > 0.9, "IoU {score}");
        // the analysis is 2x coarser: small features get twice the edge
        for (part, most) in [(Part::Hair, 0.02), (Part::Background, 0.01), (Part::Teeth, 0.1), (Part::Eye, 0.1)] {
            let m = mean_in(&det.mask, &core(p.part(part)));
            assert!(m <= most, "{part:?} selected {m:.3}");
        }
    }

    #[test]
    fn samples_one_layer() {
        let p = portrait(MEDIUM);
        let cover = testing::solid(W, H, Rgba::new(0.2, 0.5, 0.3, 1.0));
        let mut d = testing::doc_layers(W, H, vec![("Photo", p.img.clone()), ("Cover", cover)]);
        let v = run(&[("at", CHEEK)], &mut d).unwrap();
        assert!(v["advice"].as_str().unwrap().contains("does not look like skin"), "the composite is green: {v}");
        run(&[("at", CHEEK), ("layer", "Photo")], &mut d).unwrap();
        assert!(iou(sel(&d), p.part(Part::Skin)) >= 0.9);
    }

    #[test]
    fn errors_name_the_problem() {
        let mut d = portrait(MEDIUM).doc();
        let e = run(&[], &mut d).unwrap_err();
        assert_eq!(e.code, "PX1003");
        assert!(e.message.contains("--at"), "{}", e.message);
        let e = run(&[("at", "400,10")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX1011");
        assert_eq!(run(&[("at", CHEEK), ("tolerance", "101")], &mut d).unwrap_err().code, "PX1013");
        assert_eq!(run(&[("at", CHEEK), ("lightness", "-1")], &mut d).unwrap_err().code, "PX1013");
        assert_eq!(run(&[("at", CHEEK), ("contiguous", "maybe")], &mut d).unwrap_err().code, "PX1010");
        assert_eq!(run(&[("at", CHEEK), ("holes", "-2")], &mut d).unwrap_err().code, "PX1013");
        // a sample on a transparent area
        let mut img = Image::new(20, 20);
        img.set(15, 15, [0.8, 0.55, 0.4, 1.0]);
        let e = run(&[("at", "2,2")], &mut testing::doc_from(img)).unwrap_err();
        assert_eq!(e.code, "PX3320");
        assert!(e.hint.is_some());
    }

    #[test]
    fn tiny_and_uniform_images() {
        let tan = Rgba::new(0.78, 0.53, 0.39, 1.0);
        let mut d = testing::doc_from(testing::solid(1, 1, tan));
        let v = run(&[("at", "0,0")], &mut d).unwrap();
        assert_eq!(sel(&d).data, vec![1.0]);
        assert!(v["advice"].as_str().unwrap().contains("covers 100%"), "{v}");
        let mut d = testing::doc_from(testing::solid(3, 2, tan));
        run(&[("at", "2,1"), ("contiguous", "off"), ("refine", "5"), ("holes", "4")], &mut d).unwrap();
        assert!(sel(&d).data.iter().all(|&v| v == 1.0));
        // percent coordinates resolve on the canvas
        let mut d = testing::doc_from(testing::solid(10, 10, tan));
        let v = run(&[("at", "50%,50%")], &mut d).unwrap();
        assert_eq!(v["samples"][0]["at"], json!([5, 5]));
    }
}
