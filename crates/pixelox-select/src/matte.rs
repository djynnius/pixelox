//! `mask refine`: Photoshop's Extract rebuilt from published methods. A rough mask becomes
//! a trimap, alpha is solved per pixel in its unknown band, and the band's colors can be
//! decontaminated (background spill removed).
//!
//! Pipeline (O(band) for the solve, O(pixels) for the rest; deterministic, thread-count
//! invariant):
//!
//! 1. Trimap: definite foreground where the rough mask is at least 0.5, definite
//!    background below; unknown within `--band` px of its 0.5 edge (an exact Euclidean
//!    distance transform, Felzenszwalb & Huttenlocher 2012) and wherever the mask is soft,
//!    or exactly the region painted into `--unknown` (Extract's highlighter). Outside the
//!    band the rough mask is kept as it is.
//! 2. Alpha (`--model closed-form`): closed-form matting (Levin, Lischinski & Weiss 2008)
//!    with a confidence-weighted data term (robust matting, Wang & Cohen 2007): known
//!    foreground and background colors are spread into the band by pull-push
//!    interpolation, each band pixel is projected between them, and the projection is
//!    trusted by how well the pixel fits that line. A weak pull toward the rough mask
//!    (stronger where it is confidently inside) keeps color-ambiguous stretches. The
//!    matting Laplacian over 3×3 windows is assembled as a 5×5 stencil for the unknown
//!    pixels only (known neighbors move to the right-hand side) and solved coarse to fine
//!    on the band's bounding box: image, trimap, and data term are halved until the band is
//!    a few px thick, the coarsest level is solved, and each finer level starts from the
//!    coarser solution, with Jacobi-preconditioned conjugate gradients (fixed iteration
//!    caps and tolerances; dot products summed per fixed chunk, then in order).
//! 3. `--force-foreground COLOR` (Extract's Force Foreground) instead projects each band
//!    pixel onto the segment from the local background (pull-push of the known
//!    background) to COLOR.
//! 4. `--decontaminate`: foreground colors by fast multi-level estimation (Germer, Uelwer,
//!    Conrad & Harmeling 2021): `F·α + B·(1−α) ≈ I` with smooth F and B, solved from 1×1 up
//!    to full size with red-black Gauss–Seidel sweeps; F replaces the layer's pixels where
//!    the refined alpha is non-zero in the band.

use crate::select::sample_source;
use crate::util::{len_px, mask_json, r1, r3, resolve_mask_ref, BBOX_THRESHOLD};
use pixelox_core::doc::LayerPath;
use pixelox_core::prelude::*;
use rayon::prelude::*;
use std::borrow::Cow;

type Rgb = [f32; 3];

const BG: u8 = 0;
const FG: u8 = 1;
const UNKNOWN: u8 = 2;

/// Auto band: this fraction of the canvas's long side on each side of the edge.
const AUTO_BAND: f64 = 0.008;
/// Smallest auto band, px.
const MIN_BAND: f64 = 3.0;
/// The rough mask counts as soft (uncertain) strictly between these.
const SOFT_LO: f32 = 0.02;
const SOFT_HI: f32 = 0.98;

/// Solver tuning (colors in sRGB 0..1).
#[derive(Clone, Copy, Debug)]
struct Tuning {
    /// Regularization of the matting Laplacian: smaller keeps finer detail, larger is
    /// smoother and robust to 8-bit quantization in flat areas.
    epsilon: f64,
    /// Weight of the pull toward the rough mask (the Laplacian's diagonal is up to 8).
    prior: f64,
    /// The same where the rough mask is confidently inside (textured foreground such as
    /// hair constrains the Laplacian only weakly; background gaps are caught by the data
    /// term, being background-colored).
    prior_in: f64,
    /// Weight of the sampled-color data term at full confidence.
    data: f64,
    /// Confidence falloff of the sampled-color fit (distance ratio).
    sigma: f32,
}

impl Tuning {
    const DEFAULT: Tuning = Tuning { epsilon: 1e-4, prior: 1e-3, prior_in: 0.03, data: 0.3, sigma: 0.1 };
}

/// Coarsen until the unknown band is at most this many px from known pixels.
const COARSE_REACH: f64 = 6.0;
const MAX_LEVELS: usize = 5;
/// Conjugate gradient caps: (iterations, relative residual) for the coarsest level and the
/// finer ones.
const COARSE_CG: (usize, f64) = (400, 1e-6);
const FINE_CG: (usize, f64) = (100, 1e-5);
/// Unknown pixels per chunk of parallel work (fixed, so sums never depend on threads).
const CHUNK: usize = 4096;
/// Alpha closer than this to 0 or 1 snaps there (half an 8-bit step).
const SNAP: f32 = 0.5 / 255.0;

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("mask refine", Category::Mask, "Refine a mask's edge: hair-level alpha and clean colors", refine)
        .about("Photoshop's Extract, rebuilt from published methods. From a rough mask (--mask: detect subject's, a selection, a traced path) it builds a trimap: definite foreground inside, definite background outside, and an unknown band --band px on each side of the edge plus wherever the mask is soft, or exactly the region painted into --unknown (Extract's highlighter). Alpha in the band is solved per pixel by closed-form matting (Levin et al. 2008: the matting Laplacian over 3x3 windows, solved coarse to fine with preconditioned conjugate gradients) with a data term from the known foreground and background colors (robust matting), so flyaway hair the rough mask missed comes back and background between strands drops out; outside the band the rough mask is kept. --force-foreground COLOR instead takes alpha from how far each band pixel lies from the local background toward COLOR (Extract's Force Foreground). --decontaminate estimates background-free foreground colors in the band (Germer et al. 2021) and writes them into --layer's pixels, removing color fringes. The refined mask replaces --mask (a library mask, @sel, or @mask:<layer>) unless --into names another target; @alpha:<layer> and image files need --into. The result reports the trimap counts, the solver's iterations and residual, how much alpha was added and removed, and what was written where. Fails with PX3320 when the band is empty or leaves no definite foreground or background.")
        .param(param("mask", ParamKind::Mask).required().help("The rough mask: a library name, @sel, @mask:<layer>, @alpha:<layer>, or a grayscale image the size of the canvas"))
        .param(param("layer", ParamKind::Layer).help("Image to pull alpha from, and the raster layer --decontaminate rewrites (default: the layer of an @mask:/@alpha: reference, else the composite; the only layer with --decontaminate)"))
        .param(param("band", ParamKind::Length).auto().default("auto").help("How far on each side of the rough edge alpha is re-solved, px (% of the shorter side); auto = 0.8% of the long side (none with --unknown)"))
        .param(param("unknown", ParamKind::Mask).help("Explicit unknown band (Extract's highlighter), e.g. painted with paint stroke --into; used instead of --band unless --band is also given"))
        .param(param("force-foreground", ParamKind::Color).help("Take band alpha from this foreground color against the local background (wispy hair of one color)"))
        .param(param("decontaminate", ParamKind::Flag).help("Write background-free foreground colors into the layer's pixels in the band (removes color fringes)"))
        .param(param("into", ParamKind::Str).help("Where the refined mask goes: a library name, @sel, or @mask:<layer> (default: replace --mask)"))
        .param(param("model", ParamKind::Choice).choices(&["closed-form", "vitmatte"]).default("closed-form").help("Alpha solver: closed-form (the matting Laplacian, no model) or vitmatte (a model, not in this release)"))
        .example("pixelox mask refine portrait.pxo --mask subject --decontaminate --in-place", "Recover flyaway hair and remove the background fringe")
        .example("pixelox mask refine shot.pxo --mask @sel --band 24 --in-place", "Re-solve 24 px on each side of the selection's edge")
        .example("pixelox mask refine shot.pxo --mask subject --unknown band --force-foreground '#6b4a2f' --into hair --in-place", "Extract-style: a painted band and a forced hair color")
        .example("pixelox mask refine shot.pxo --mask @mask:Photo --decontaminate -o cutout.png", "Refine a layer mask and write the cutout")
        .example("pixelox mask refine shot.pxo --mask subject --dry-run --json", "See the band, solver, and coverage without writing")]
}

// ---------------------------------------------------------------------------------------
// the op

/// Where the refined mask goes.
enum Dest {
    Sel,
    Named(String),
    LayerMask(LayerPath),
}

fn dest_label(doc: &Document, d: &Dest) -> String {
    match d {
        Dest::Sel => "@sel".into(),
        Dest::Named(n) => n.clone(),
        Dest::LayerMask(p) => format!("@mask:{}", doc.layer(p).name),
    }
}

fn parse_dest(doc: &Document, r: &str) -> Result<Dest> {
    let r = r.trim();
    if r == "@sel" {
        return Ok(Dest::Sel);
    }
    if let Some(l) = r.strip_prefix("@mask:") {
        let p = doc.resolve(l)?;
        doc.layer(&p).check_unlocked()?;
        return Ok(Dest::LayerMask(p));
    }
    if r.is_empty() || r.starts_with('@') || std::path::Path::new(r).extension().is_some() {
        return Err(Error::new("PX1010", format!("--into '{r}' is not a place a mask can go"))
            .with_hint("pass a library mask name, @sel, or @mask:<layer>")
            .with_context("into", r));
    }
    Ok(Dest::Named(r.to_string()))
}

/// `--into`, or where `--mask` came from.
fn destination(a: &Args, doc: &Document, mask_ref: &str) -> Result<Dest> {
    if let Some(i) = a.str("into") {
        return parse_dest(doc, &i);
    }
    let m = mask_ref.trim();
    if m == "@sel" || m.starts_with("@mask:") || doc.masks.contains_key(m) {
        return parse_dest(doc, m);
    }
    Err(Error::new("PX1003", format!("'{m}' cannot be rewritten; say where the refined mask goes with --into"))
        .with_hint(format!("pixelox mask refine <doc> --mask {m} --into subject --in-place"))
        .with_context("mask", m))
}

/// The layer named by an `@mask:` / `@alpha:` reference.
fn layer_of(r: &str) -> Option<String> {
    let r = r.trim();
    r.strip_prefix("@mask:").or_else(|| r.strip_prefix("@alpha:")).map(|l| l.to_string())
}

fn refine(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    if a.choice("model") == "vitmatte" {
        return Err(Error::new(
            "PX1520",
            "mask refine --model vitmatte needs the vitmatte-s model, which is not in this release",
        )
        .with_hint("drop --model: the default closed-form solver needs no model")
        .with_context("models", ["vitmatte-s"]));
    }
    let mask_ref = a.str("mask").unwrap_or_default().trim().to_string();
    let rough = resolve_mask_ref(ctx, doc, &mask_ref, "mask")?;
    let dest = destination(a, doc, &mask_ref)?;
    let (w, h) = (doc.width as usize, doc.height as usize);
    let painted = match a.str("unknown") {
        Some(r) => Some(resolve_mask_ref(ctx, doc, &r, "unknown")?),
        None => None,
    };
    let force = a.opt_color("force-foreground")?;
    let band_auto = a.is_auto("band");
    let band = if band_auto {
        if painted.is_some() {
            0.0
        } else {
            auto_band(w, h)
        }
    } else {
        len_px(a, "band", doc)?
    };
    if band_auto {
        // full precision: a replay must rebuild exactly the same band
        ctx.resolve_param("band", band.to_string());
    }

    // the image: --layer, the layer of an @mask:/@alpha: reference or of --into @mask:,
    // else the composite (the only layer when its pixels are rewritten)
    let dest_layer = match &dest {
        Dest::LayerMask(p) => Some(doc.layer(p).id.clone()),
        _ => None,
    };
    let layer_ref = a.str("layer").or_else(|| layer_of(&mask_ref)).or(dest_layer);
    let deco = if a.flag("decontaminate") {
        let p = doc.resolve_or_only(layer_ref.as_deref())?;
        let l = doc.layer(&p);
        l.check_unlocked()?;
        l.raster()?;
        Some(p)
    } else {
        None
    };
    let (src, source_name) = match (&deco, &layer_ref) {
        (Some(p), _) => (sample_source(doc, Some(&doc.layer(p).id))?, json!(doc.layer(p).name)),
        (None, Some(r)) => {
            let name = doc.layer(&doc.resolve(r)?).name.clone();
            (sample_source(doc, Some(r))?, json!(name))
        }
        (None, None) => (compose::composite(doc)?, json!("composite")),
    };
    ctx.set_progress(0.05);

    let tri = trimap(&rough.data, painted.as_ref().map(|m| m.data.as_slice()), band, w, h);
    let counts = Counts::of(&tri);
    let empty = |why: &str, hint: &str| {
        Error::new("PX3320", format!("nothing to refine: {why}"))
            .with_hint(hint.to_string())
            .with_context("mask", mask_ref.as_str())
            .with_context("band", r3(band))
    };
    if counts.unknown == 0 {
        return Err(if painted.is_some() {
            empty("the --unknown band covers no pixels", "paint the band in white first: pixelox paint stroke <doc> --into band --points 'x,y x,y ...' --size 40 --color white --in-place")
        } else {
            empty("the mask has no edge (it is all on or all off)", "make a rough mask first, e.g. pixelox detect subject <doc> --save-mask subject --in-place, or select color --contiguous")
        });
    }
    if counts.bg == 0 {
        return Err(empty(
            "no definite background is left outside the band",
            "narrow the band with a smaller --band, or paint a thinner --unknown",
        ));
    }
    if counts.fg == 0 && force.is_none() {
        return Err(empty(
            "no definite foreground is left inside the band",
            "narrow the band with a smaller --band, or give the foreground color with --force-foreground",
        ));
    }

    let region = unknown_bounds(&tri, w).expect("unknown pixels exist");
    // outside the band the rough mask stands as it is
    let mut alpha = rough.data.clone();
    let solver = match force {
        Some(c) => {
            force_alpha(&src.data, &tri, &rough.data, w, h, region, [c.r, c.g, c.b], &mut alpha);
            json!({"method": "force-foreground"})
        }
        None => {
            let cancel = || ctx.check_cancel();
            let progress = |f: f64| ctx.set_progress(0.1 + 0.7 * f);
            let input = Matting { rgba: &src.data, tri: &tri, rough: &rough.data, w, h, region };
            solve_alpha(&input, &Tuning::DEFAULT, &mut alpha, &cancel, &progress)?.to_json()
        }
    };
    // pixels the source does not cover cannot be foreground
    alpha.par_iter_mut().zip(tri.par_iter()).zip(src.data.par_chunks(4)).for_each(|((v, &t), p)| {
        if t == UNKNOWN {
            *v *= p[3].clamp(0.0, 1.0);
        }
    });
    ctx.set_progress(0.8);

    let refined = Mask { width: w as u32, height: h as u32, data: alpha };
    let change = change_json(&rough.data, &refined.data, w);

    let decontaminated = match &deco {
        Some(p) => {
            let fg = estimate_foreground_in(&src.data, &refined.data, w, h, region);
            let name = doc.layer(p).name.clone();
            let n = write_colors(doc.layer_mut(p), &fg, &refined.data, &tri, w, h)?;
            json!({"layer": name, "pixels": n})
        }
        None => Value::Null,
    };
    ctx.set_progress(0.95);

    let into = dest_label(doc, &dest);
    let stats = mask_json(&refined);
    store(doc, &dest, refined)?;
    Ok(json!({
        "mask": mask_ref,
        "into": into,
        "model": "closed-form",
        "source": source_name,
        "band": band,
        "force_foreground": force.map(|c| c.to_hex()),
        "trimap": {
            "foreground": counts.fg,
            "background": counts.bg,
            "unknown": counts.unknown,
            "bounds": region.to_json(),
        },
        "solver": solver,
        "change": change,
        "before": mask_json(&rough),
        "stats": stats,
        "decontaminated": decontaminated,
    }))
}

fn auto_band(w: usize, h: usize) -> f64 {
    (w.max(h) as f64 * AUTO_BAND).round().max(MIN_BAND)
}

fn store(doc: &mut Document, dest: &Dest, m: Mask) -> Result<()> {
    match dest {
        Dest::Sel => doc.selection = Some(m),
        Dest::Named(n) => doc.put_mask(n, m, true)?,
        Dest::LayerMask(p) => {
            let l = doc.layer_mut(p);
            match &mut l.mask {
                Some(lm) => lm.mask = m,
                None => l.mask = Some(LayerMask::new(m)),
            }
            // the solved pixels no longer match a vector mask's path
            l.vector_mask = None;
        }
    }
    Ok(())
}

/// Alpha added outside / removed inside the rough mask, summed per row in order.
fn change_json(before: &[f32], after: &[f32], w: usize) -> Value {
    let rows: Vec<(f64, f64, usize)> = before
        .par_chunks(w)
        .zip(after.par_chunks(w))
        .map(|(b, a)| {
            let mut r = (0.0, 0.0, 0usize);
            for (&x, &y) in b.iter().zip(a) {
                let d = y as f64 - x as f64;
                if d > 0.0 {
                    r.0 += d;
                } else {
                    r.1 -= d;
                }
                if d.abs() > 0.5 / 255.0 {
                    r.2 += 1;
                }
            }
            r
        })
        .collect();
    let (mut added, mut removed, mut pixels) = (0.0, 0.0, 0usize);
    for r in rows {
        added += r.0;
        removed += r.1;
        pixels += r.2;
    }
    json!({"added": r1(added), "removed": r1(removed), "pixels": pixels})
}

// ---------------------------------------------------------------------------------------
// trimap

struct Counts {
    fg: usize,
    bg: usize,
    unknown: usize,
}

impl Counts {
    fn of(tri: &[u8]) -> Counts {
        let per: Vec<[usize; 3]> = tri
            .par_chunks(CHUNK)
            .map(|c| {
                let mut n = [0usize; 3];
                for &s in c {
                    n[s as usize] += 1;
                }
                n
            })
            .collect();
        let mut n = [0usize; 3];
        for p in per {
            for k in 0..3 {
                n[k] += p[k];
            }
        }
        Counts { bg: n[BG as usize], fg: n[FG as usize], unknown: n[UNKNOWN as usize] }
    }
}

/// Foreground where `rough >= 0.5`, background below; unknown within `band` px of the 0.5
/// edge and where the mask is soft (when `band > 0`), and where `painted >= 0.5`.
fn trimap(rough: &[f32], painted: Option<&[f32]>, band: f64, w: usize, h: usize) -> Vec<u8> {
    let inside = |i: usize| rough[i] >= 0.5;
    let mut tri: Vec<u8> = rough.par_iter().map(|&v| if v >= 0.5 { FG } else { BG }).collect();
    if band > 0.0 {
        // edge pixels on both sides of the 0.5 contour
        let edge: Vec<bool> = (0..w * h)
            .into_par_iter()
            .map(|i| {
                let (x, y) = (i % w, i / w);
                let me = inside(i);
                (x > 0 && inside(i - 1) != me)
                    || (x + 1 < w && inside(i + 1) != me)
                    || (y > 0 && inside(i - w) != me)
                    || (y + 1 < h && inside(i + w) != me)
            })
            .collect();
        let d2 = edt_sq(&edge, w, h);
        let b2 = (band * band) as f32;
        tri.par_iter_mut().zip(d2.par_iter()).zip(rough.par_iter()).for_each(|((t, &d), &m)| {
            if d <= b2 || (m > SOFT_LO && m < SOFT_HI) {
                *t = UNKNOWN;
            }
        });
    }
    if let Some(p) = painted {
        tri.par_iter_mut().zip(p.par_iter()).for_each(|(t, &v)| {
            if v >= 0.5 {
                *t = UNKNOWN;
            }
        });
    }
    tri
}

fn unknown_bounds(tri: &[u8], w: usize) -> Option<RectI> {
    let rows: Vec<Option<(usize, usize)>> = tri
        .par_chunks(w)
        .map(|r| {
            let a = r.iter().position(|&s| s == UNKNOWN)?;
            let b = r.iter().rposition(|&s| s == UNKNOWN)?;
            Some((a, b))
        })
        .collect();
    let (mut x0, mut x1, mut y0, mut y1) = (usize::MAX, 0, usize::MAX, 0);
    for (y, r) in rows.iter().enumerate() {
        if let Some((a, b)) = r {
            x0 = x0.min(*a);
            x1 = x1.max(*b);
            y0 = y0.min(y);
            y1 = y1.max(y);
        }
    }
    (x0 != usize::MAX).then(|| RectI::new(x0 as i32, y0 as i32, (x1 - x0 + 1) as u32, (y1 - y0 + 1) as u32))
}

/// `r` grown by `m` px on every side, clipped to the canvas: `(x0, y0, x1, y1)` exclusive.
fn grow(r: RectI, m: usize, w: usize, h: usize) -> (usize, usize, usize, usize) {
    let (x, y, m) = (r.x as i64, r.y as i64, m as i64);
    (
        (x - m).max(0) as usize,
        (y - m).max(0) as usize,
        ((x + r.w as i64 + m) as usize).min(w),
        ((y + r.h as i64 + m) as usize).min(h),
    )
}

fn crop<T: Copy + Default + Send + Sync>(v: &[T], w: usize, b: (usize, usize, usize, usize)) -> Vec<T> {
    let (x0, y0, x1, y1) = b;
    let rw = x1 - x0;
    let mut out = vec![T::default(); rw * (y1 - y0)];
    out.par_chunks_mut(rw).enumerate().for_each(|(y, row)| {
        let s = (y + y0) * w;
        row.copy_from_slice(&v[s + x0..s + x1]);
    });
    out
}

/// The RGB of an RGBA buffer inside `b`.
fn crop_rgb(rgba: &[f32], w: usize, b: (usize, usize, usize, usize)) -> Vec<Rgb> {
    let (x0, y0, x1, y1) = b;
    let rw = x1 - x0;
    let mut out = vec![[0f32; 3]; rw * (y1 - y0)];
    out.par_chunks_mut(rw).enumerate().for_each(|(y, row)| {
        let s = ((y + y0) * w + x0) * 4;
        for (o, p) in row.iter_mut().zip(rgba[s..s + rw * 4].chunks_exact(4)) {
            *o = [p[0], p[1], p[2]];
        }
    });
    out
}

/// Squared Euclidean distance from every pixel to the nearest `feature` pixel (exact;
/// Felzenszwalb & Huttenlocher). Pixels with no feature at all get a huge value.
fn edt_sq(feature: &[bool], w: usize, h: usize) -> Vec<f32> {
    const FAR: f64 = 1e20;
    // columns, as column-major vectors
    let cols: Vec<Vec<f32>> = (0..w)
        .into_par_iter()
        .map(|x| {
            let f: Vec<f64> = (0..h).map(|y| if feature[y * w + x] { 0.0 } else { FAR }).collect();
            dt1d(&f).into_iter().map(|d| d.min(f32::MAX as f64) as f32).collect()
        })
        .collect();
    let mut out = vec![0f32; w * h];
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let f: Vec<f64> = (0..w).map(|x| cols[x][y] as f64).collect();
        for (o, d) in row.iter_mut().zip(dt1d(&f)) {
            *o = d.min(f32::MAX as f64) as f32;
        }
    });
    out
}

/// 1-D squared distance transform of a sampled function (lower envelope of parabolas).
fn dt1d(f: &[f64]) -> Vec<f64> {
    let n = f.len();
    let mut d = vec![0.0; n];
    if n == 0 {
        return d;
    }
    let mut v = vec![0usize; n];
    let mut z = vec![0.0f64; n + 1];
    let mut k = 0usize;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    for q in 1..n {
        let qf = q as f64;
        let mut s;
        loop {
            let p = v[k] as f64;
            s = ((f[q] + qf * qf) - (f[v[k]] + p * p)) / (2.0 * qf - 2.0 * p);
            // z[0] is -inf, so this stops at k = 0
            if s <= z[k] && k > 0 {
                k -= 1;
            } else {
                break;
            }
        }
        k += 1;
        v[k] = q;
        z[k] = s;
        z[k + 1] = f64::INFINITY;
    }
    k = 0;
    for (q, o) in d.iter_mut().enumerate() {
        while z[k + 1] < q as f64 {
            k += 1;
        }
        let p = v[k] as f64;
        *o = (q as f64 - p).powi(2) + f[v[k]];
    }
    d
}

// ---------------------------------------------------------------------------------------
// closed-form matting

/// One level of the coarse-to-fine pyramid. Each unknown pixel carries a data term:
/// `weight·(α − target)²`.
struct Level {
    w: usize,
    h: usize,
    img: Vec<Rgb>,
    tri: Vec<u8>,
    weight: Vec<f32>,
    target: Vec<f32>,
}

impl Level {
    /// Half size: colors and data weights averaged; a pixel stays known only when all its
    /// children agree.
    fn halve(&self) -> Level {
        let (w, h) = (self.w.div_ceil(2), self.h.div_ceil(2));
        let px: Vec<(Rgb, u8, f32, f32)> = (0..w * h)
            .into_par_iter()
            .map(|i| {
                let (x, y) = (i % w, i / w);
                let (mut c, mut wt, mut wa, mut n) = ([0f32; 3], 0f32, 0f32, 0f32);
                let (mut all_fg, mut all_bg) = (true, true);
                for sy in 2 * y..(2 * y + 2).min(self.h) {
                    for sx in 2 * x..(2 * x + 2).min(self.w) {
                        let j = sy * self.w + sx;
                        for (s, v) in c.iter_mut().zip(self.img[j]) {
                            *s += v;
                        }
                        wt += self.weight[j];
                        wa += self.weight[j] * self.target[j];
                        n += 1.0;
                        all_fg &= self.tri[j] == FG;
                        all_bg &= self.tri[j] == BG;
                    }
                }
                let t = if all_fg {
                    FG
                } else if all_bg {
                    BG
                } else {
                    UNKNOWN
                };
                let target = if wt > 0.0 { wa / wt } else { 0.0 };
                ([c[0] / n, c[1] / n, c[2] / n], t, wt / n, target)
            })
            .collect();
        Level {
            w,
            h,
            img: px.iter().map(|p| p.0).collect(),
            tri: px.iter().map(|p| p.1).collect(),
            weight: px.iter().map(|p| p.2).collect(),
            target: px.iter().map(|p| p.3).collect(),
        }
    }
}

/// The data term at full size: a weak pull toward the rough mask, plus sampled colors
/// (Wang & Cohen's robust matting with one sample pair): known foreground and background
/// colors spread into the band by pull-push, each unknown pixel projected between them,
/// trusted by how well it fits that line and how distinct the two colors are.
fn data_term(img: &[Rgb], tri: &[u8], rough: &[f32], w: usize, h: usize, t: &Tuning) -> (Vec<f32>, Vec<f32>) {
    let fw: Vec<f32> = tri.par_iter().map(|&s| if s == FG { 1.0 } else { 0.0 }).collect();
    let bw: Vec<f32> = tri.par_iter().map(|&s| if s == BG { 1.0 } else { 0.0 }).collect();
    let fg = pull_push(img, &fw, w, h);
    let bg = pull_push(img, &bw, w, h);
    (0..w * h)
        .into_par_iter()
        .map(|i| {
            if tri[i] != UNKNOWN {
                return (0.0, 0.0);
            }
            let prior = if rough[i] >= SOFT_HI { t.prior_in } else { t.prior } as f32;
            let (c, f, b) = (img[i], fg[i], bg[i]);
            let d = [f[0] - b[0], f[1] - b[1], f[2] - b[2]];
            let dd = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
            let sep = smoothstep(0.03, 0.15, dd.sqrt());
            if sep <= 0.0 || t.data <= 0.0 {
                return (prior, rough[i]);
            }
            let a = (((c[0] - b[0]) * d[0] + (c[1] - b[1]) * d[1] + (c[2] - b[2]) * d[2]) / dd).clamp(0.0, 1.0);
            let r = [c[0] - b[0] - a * d[0], c[1] - b[1] - a * d[1], c[2] - b[2] - a * d[2]];
            let ratio = (r[0] * r[0] + r[1] * r[1] + r[2] * r[2]).sqrt() / dd.sqrt();
            let conf = (-(ratio / t.sigma).powi(2)).exp() * sep;
            let wd = t.data as f32 * conf;
            let wt = prior + wd;
            (wt, (prior * rough[i] + wd * a) / wt)
        })
        .unzip()
}

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Solver statistics for the result.
#[derive(Debug, Clone, Default)]
struct SolveStats {
    levels: Vec<(usize, usize, f64)>,
    epsilon: f64,
}

impl SolveStats {
    fn to_json(&self) -> Value {
        let last = self.levels.last().copied().unwrap_or((0, 0, 0.0));
        json!({
            "method": "closed-form",
            "epsilon": self.epsilon,
            "levels": self.levels.len(),
            "unknowns": last.0,
            "iterations": last.1,
            "residual": sci(last.2),
            "per_level": self.levels.iter().map(|(n, it, r)| json!({"unknowns": n, "iterations": it, "residual": sci(*r)})).collect::<Vec<_>>(),
        })
    }
}

/// Three significant digits (residuals span many decades).
fn sci(v: f64) -> f64 {
    if v == 0.0 || !v.is_finite() {
        return v;
    }
    format!("{v:.2e}").parse().unwrap_or(v)
}

/// What the alpha solver works on: the canvas-sized source (RGBA), trimap, and rough mask,
/// and the bounds of the unknown band.
struct Matting<'a> {
    rgba: &'a [f32],
    tri: &'a [u8],
    rough: &'a [f32],
    w: usize,
    h: usize,
    region: RectI,
}

/// Solve the unknown band and write it into `alpha` (canvas-sized; other pixels are left).
fn solve_alpha(
    m: &Matting,
    tuning: &Tuning,
    alpha: &mut [f32],
    cancel: &dyn Fn() -> Result<()>,
    progress: &dyn Fn(f64),
) -> Result<SolveStats> {
    let Matting { rgba, tri, rough, w, h, region } = *m;
    let b = grow(region, 2, w, h);
    let (rw, rh) = (b.2 - b.0, b.3 - b.1);
    let (img, ctri) = (crop_rgb(rgba, w, b), crop(tri, w, b));
    let (weight, target) = data_term(&img, &ctri, &crop(rough, w, b), rw, rh, tuning);
    let mut levels = vec![Level { w: rw, h: rh, img, tri: ctri, weight, target }];
    let reach = max_reach(&levels[0].tri, rw, rh);
    while levels.len() < MAX_LEVELS + 1 {
        let l = levels.len() - 1;
        let scale = (1usize << l) as f64;
        if reach / scale <= COARSE_REACH || levels[l].w < 16 || levels[l].h < 16 {
            break;
        }
        let next = levels[l].halve();
        levels.push(next);
    }
    let mut stats = SolveStats { levels: vec![], epsilon: tuning.epsilon };
    let mut field: Option<(Vec<f32>, usize, usize)> = None;
    let n_levels = levels.len();
    for (step, lv) in levels.iter().rev().enumerate() {
        let init: Vec<f32> = match &field {
            None => lv.target.clone(),
            Some((coarse, cw, ch)) => prolong(coarse, *cw, *ch, lv.w, lv.h),
        };
        let caps = if step == 0 { COARSE_CG } else { FINE_CG };
        let (a, n, it, res) = solve_level(lv, &init, caps, tuning.epsilon, cancel)?;
        stats.levels.push((n, it, res));
        field = Some((a, lv.w, lv.h));
        progress((step + 1) as f64 / n_levels as f64);
    }
    let (sol, _, _) = field.expect("one level at least");
    // the band back onto the canvas
    alpha.par_chunks_mut(w).zip(tri.par_chunks(w)).enumerate().for_each(|(y, (row, trow))| {
        if y < b.1 || y >= b.3 {
            return;
        }
        let src = &sol[(y - b.1) * rw..(y - b.1 + 1) * rw];
        for x in b.0..b.2 {
            if trow[x] == UNKNOWN {
                row[x] = src[x - b.0];
            }
        }
    });
    Ok(stats)
}

/// Largest distance from an unknown pixel to the nearest known one.
fn max_reach(tri: &[u8], w: usize, h: usize) -> f64 {
    let known: Vec<bool> = tri.par_iter().map(|&t| t != UNKNOWN).collect();
    let d = edt_sq(&known, w, h);
    let m = d
        .par_chunks(w)
        .zip(tri.par_chunks(w))
        .map(|(d, t)| d.iter().zip(t).filter(|(_, &t)| t == UNKNOWN).map(|(&d, _)| d).fold(0f32, f32::max))
        .collect::<Vec<f32>>()
        .into_iter()
        .fold(0f32, f32::max);
    (m as f64).sqrt()
}

/// Bilinear upsampling of a coarse field to `w`×`h` (pixel centers aligned).
fn prolong(c: &[f32], cw: usize, ch: usize, w: usize, h: usize) -> Vec<f32> {
    let tap = |x: usize, n: usize| {
        let f = ((x as f64 + 0.5) / 2.0 - 0.5).clamp(0.0, (n - 1) as f64);
        let i0 = f.floor() as usize;
        (i0, (i0 + 1).min(n - 1), (f - i0 as f64) as f32)
    };
    let mut out = vec![0f32; w * h];
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let (y0, y1, ty) = tap(y, ch);
        for (x, o) in row.iter_mut().enumerate() {
            let (x0, x1, tx) = tap(x, cw);
            let top = c[y0 * cw + x0] + (c[y0 * cw + x1] - c[y0 * cw + x0]) * tx;
            let bot = c[y1 * cw + x0] + (c[y1 * cw + x1] - c[y1 * cw + x0]) * tx;
            *o = top + (bot - top) * ty;
        }
    });
    out
}

/// The matting Laplacian restricted to a level's unknown pixels: a 5×5 stencil per
/// unknown, indexed through a padded grid (known and outside pixels map to a zero slot).
struct System {
    n: usize,
    grid: Vec<u32>,
    pos: Vec<u32>,
    offs: [isize; 25],
    coef: Vec<[f32; 25]>,
    inv_diag: Vec<f64>,
    rhs: Vec<f64>,
}

/// Window statistics: mean and `(Σ + ε/9·I)⁻¹` (symmetric, 6 entries).
#[inline]
fn window(img: &[Rgb], w: usize, kx: usize, ky: usize, epsilon: f64) -> ([f64; 3], [f64; 6]) {
    let mut s = [0f64; 3];
    let mut q = [0f64; 6];
    for y in ky - 1..=ky + 1 {
        for x in kx - 1..=kx + 1 {
            let c = img[y * w + x];
            let (r, g, b) = (c[0] as f64, c[1] as f64, c[2] as f64);
            s[0] += r;
            s[1] += g;
            s[2] += b;
            q[0] += r * r;
            q[1] += r * g;
            q[2] += r * b;
            q[3] += g * g;
            q[4] += g * b;
            q[5] += b * b;
        }
    }
    let mu = [s[0] / 9.0, s[1] / 9.0, s[2] / 9.0];
    let e = epsilon / 9.0;
    let a = q[0] / 9.0 - mu[0] * mu[0] + e;
    let b = q[1] / 9.0 - mu[0] * mu[1];
    let c = q[2] / 9.0 - mu[0] * mu[2];
    let d = q[3] / 9.0 - mu[1] * mu[1] + e;
    let f = q[4] / 9.0 - mu[1] * mu[2];
    let g = q[5] / 9.0 - mu[2] * mu[2] + e;
    // adjugate of [[a b c] [b d f] [c f g]]
    let i00 = d * g - f * f;
    let i01 = c * f - b * g;
    let i02 = b * f - c * d;
    let i11 = a * g - c * c;
    let i12 = b * c - a * f;
    let i22 = a * d - b * b;
    let det = a * i00 + b * i01 + c * i02;
    let inv = 1.0 / det;
    (mu, [i00 * inv, i01 * inv, i02 * inv, i11 * inv, i12 * inv, i22 * inv])
}

impl System {
    fn build(lv: &Level, init: &[f32], epsilon: f64) -> (System, Vec<f64>) {
        let (w, h) = (lv.w, lv.h);
        let pw = w + 4;
        let unknown: Vec<usize> = (0..w * h).filter(|&i| lv.tri[i] == UNKNOWN).collect();
        let n = unknown.len();
        let mut grid = vec![n as u32; pw * (h + 4)];
        let mut pos = Vec::with_capacity(n);
        for (k, &i) in unknown.iter().enumerate() {
            let p = (i / w + 2) * pw + i % w + 2;
            grid[p] = k as u32;
            pos.push(p as u32);
        }
        let mut offs = [0isize; 25];
        for (t, o) in offs.iter_mut().enumerate() {
            *o = (t as isize / 5 - 2) * pw as isize + (t as isize % 5 - 2);
        }
        let known_alpha = |j: usize| if lv.tri[j] == FG { 1.0 } else { 0.0 };
        let rows: Vec<([f32; 25], f64, f64)> = unknown
            .par_iter()
            .map(|&i| {
                let (x, y) = (i % w, i / w);
                let mut acc = [0f64; 25];
                let ci = lv.img[i];
                for ky in y.saturating_sub(1)..=(y + 1).min(h - 1) {
                    if ky < 1 || ky + 1 >= h {
                        continue;
                    }
                    for kx in x.saturating_sub(1)..=(x + 1).min(w - 1) {
                        if kx < 1 || kx + 1 >= w {
                            continue;
                        }
                        let (mu, m) = window(&lv.img, w, kx, ky, epsilon);
                        let di = [ci[0] as f64 - mu[0], ci[1] as f64 - mu[1], ci[2] as f64 - mu[2]];
                        let v = [
                            m[0] * di[0] + m[1] * di[1] + m[2] * di[2],
                            m[1] * di[0] + m[3] * di[1] + m[4] * di[2],
                            m[2] * di[0] + m[4] * di[1] + m[5] * di[2],
                        ];
                        for jy in ky - 1..=ky + 1 {
                            for jx in kx - 1..=kx + 1 {
                                let cj = lv.img[jy * w + jx];
                                let t = v[0] * (cj[0] as f64 - mu[0])
                                    + v[1] * (cj[1] as f64 - mu[1])
                                    + v[2] * (cj[2] as f64 - mu[2]);
                                let slot = (jy + 2 - y) * 5 + (jx + 2 - x);
                                acc[slot] -= (1.0 + t) / 9.0;
                            }
                        }
                        acc[12] += 1.0;
                    }
                }
                let wt = lv.weight[i] as f64;
                acc[12] += wt;
                let mut rhs = wt * lv.target[i] as f64;
                let mut coef = [0f32; 25];
                for t in 0..25 {
                    if acc[t] == 0.0 {
                        continue;
                    }
                    let (jx, jy) = ((x + t % 5) as isize - 2, (y + t / 5) as isize - 2);
                    let j = jy as usize * w + jx as usize;
                    if lv.tri[j] == UNKNOWN {
                        coef[t] = acc[t] as f32;
                    } else {
                        rhs -= acc[t] * known_alpha(j);
                    }
                }
                (coef, 1.0 / acc[12].max(1e-12), rhs)
            })
            .collect();
        let mut coef = Vec::with_capacity(n);
        let mut inv_diag = Vec::with_capacity(n);
        let mut rhs = Vec::with_capacity(n);
        for (c, d, r) in rows {
            coef.push(c);
            inv_diag.push(d);
            rhs.push(r);
        }
        let mut x0: Vec<f64> = unknown.iter().map(|&i| init[i].clamp(0.0, 1.0) as f64).collect();
        x0.push(0.0);
        (System { n, grid, pos, offs, coef, inv_diag, rhs }, x0)
    }

    /// `y = A·x` (`x` has the zero slot at index `n`).
    fn apply(&self, x: &[f64], y: &mut [f64]) {
        y[..self.n].par_chunks_mut(CHUNK).enumerate().for_each(|(c, ys)| {
            for (k, yi) in ys.iter_mut().enumerate() {
                let i = c * CHUNK + k;
                let p = self.pos[i] as isize;
                let co = &self.coef[i];
                let mut s = 0.0;
                for t in 0..25 {
                    s += co[t] as f64 * x[self.grid[(p + self.offs[t]) as usize] as usize];
                }
                *yi = s;
            }
        });
    }
}

/// Dot product summed per fixed chunk, then in order.
fn dot(a: &[f64], b: &[f64]) -> f64 {
    let parts: Vec<f64> =
        a.par_chunks(CHUNK).zip(b.par_chunks(CHUNK)).map(|(x, y)| x.iter().zip(y).map(|(u, v)| u * v).sum()).collect();
    parts.iter().sum()
}

/// Solve one level; returns its full alpha field, the unknown count, iterations, and the
/// final relative residual.
fn solve_level(
    lv: &Level,
    init: &[f32],
    caps: (usize, f64),
    epsilon: f64,
    cancel: &dyn Fn() -> Result<()>,
) -> Result<(Vec<f32>, usize, usize, f64)> {
    let (sys, mut x) = System::build(lv, init, epsilon);
    let n = sys.n;
    let (iterations, residual) = if n == 0 { (0, 0.0) } else { pcg(&sys, &mut x, caps, cancel)? };
    let mut out: Vec<f32> = lv.tri.iter().map(|&t| if t == FG { 1.0 } else { 0.0 }).collect();
    let mut k = 0;
    for (i, t) in lv.tri.iter().enumerate() {
        if *t == UNKNOWN {
            let v = x[k].clamp(0.0, 1.0) as f32;
            out[i] = if v < SNAP {
                0.0
            } else if v > 1.0 - SNAP {
                1.0
            } else {
                v
            };
            k += 1;
        }
    }
    Ok((out, n, iterations, residual))
}

/// Jacobi-preconditioned conjugate gradients from `x`; returns (iterations, ‖r‖/‖b‖).
fn pcg(sys: &System, x: &mut [f64], caps: (usize, f64), cancel: &dyn Fn() -> Result<()>) -> Result<(usize, f64)> {
    let n = sys.n;
    let (max_it, tol) = caps;
    let bnorm = dot(&sys.rhs, &sys.rhs).sqrt().max(1e-30);
    let mut q = vec![0f64; n];
    sys.apply(x, &mut q);
    let mut r: Vec<f64> = sys.rhs.par_iter().zip(q.par_iter()).map(|(b, q)| b - q).collect();
    let mut z: Vec<f64> = r.par_iter().zip(sys.inv_diag.par_iter()).map(|(r, d)| r * d).collect();
    let mut p = z.clone();
    p.push(0.0);
    let mut rz = dot(&r, &z);
    let mut res = dot(&r, &r).sqrt() / bnorm;
    let mut it = 0;
    while it < max_it && res > tol {
        if it % 32 == 0 {
            cancel()?;
        }
        sys.apply(&p, &mut q);
        let pq = dot(&p[..n], &q);
        if pq <= 0.0 || !pq.is_finite() {
            break;
        }
        let step = rz / pq;
        x[..n].par_iter_mut().zip(p[..n].par_iter()).for_each(|(x, p)| *x += step * p);
        r.par_iter_mut().zip(q.par_iter()).for_each(|(r, q)| *r -= step * q);
        it += 1;
        res = dot(&r, &r).sqrt() / bnorm;
        z.par_iter_mut().zip(r.par_iter().zip(sys.inv_diag.par_iter())).for_each(|(z, (r, d))| *z = r * d);
        let rz_new = dot(&r, &z);
        let beta = rz_new / rz;
        rz = rz_new;
        p[..n].par_iter_mut().zip(z.par_iter()).for_each(|(p, z)| *p = z + beta * *p);
    }
    Ok((it, res))
}

// ---------------------------------------------------------------------------------------
// force foreground

/// Band alpha from projecting each band pixel onto the segment from its local background
/// to `fc`, written into `alpha`. Where the two colors nearly coincide, the rough mask is
/// kept.
#[allow(clippy::too_many_arguments)]
fn force_alpha(rgba: &[f32], tri: &[u8], rough: &[f32], w: usize, h: usize, region: RectI, fc: Rgb, alpha: &mut [f32]) {
    let margin = (region.w.max(region.h) as usize / 8).clamp(8, 64);
    let b = grow(region, margin, w, h);
    let (rw, rh) = (b.2 - b.0, b.3 - b.1);
    let img = crop_rgb(rgba, w, b);
    let t = crop(tri, w, b);
    let weight: Vec<f32> = t.iter().map(|&s| if s == BG { 1.0 } else { 0.0 }).collect();
    let bgc = pull_push(&img, &weight, rw, rh);
    alpha.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        if y < b.1 || y >= b.3 {
            return;
        }
        for (x, out) in row.iter_mut().enumerate().take(b.2).skip(b.0) {
            let i = y * w + x;
            if tri[i] != UNKNOWN {
                continue;
            }
            let k = (y - b.1) * rw + (x - b.0);
            let (c, bc) = (img[k], bgc[k]);
            let d = [fc[0] - bc[0], fc[1] - bc[1], fc[2] - bc[2]];
            let dd = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
            let v = if dd < 1e-3 {
                rough[i]
            } else {
                ((c[0] - bc[0]) * d[0] + (c[1] - bc[1]) * d[1] + (c[2] - bc[2]) * d[2]) / dd
            };
            let v = v.clamp(0.0, 1.0);
            *out = if v < SNAP {
                0.0
            } else if v > 1.0 - SNAP {
                1.0
            } else {
                v
            };
        }
    });
}

/// Pull-push interpolation: colors where `weight > 0`, filled smoothly elsewhere.
fn pull_push(img: &[Rgb], weight: &[f32], w: usize, h: usize) -> Vec<Rgb> {
    let mut out = img.to_vec();
    if w <= 1 && h <= 1 {
        return out;
    }
    // pull: weighted averages up the pyramid
    let mut levels = vec![pull(img, weight, w, h)];
    while let Some(l) = levels.last().filter(|l| l.2 > 1 || l.3 > 1) {
        let next = pull(&l.0, &l.1, l.2, l.3);
        levels.push(next);
    }
    // push: fill each level's gaps from the one above
    for l in (0..levels.len() - 1).rev() {
        let (lo, hi) = levels.split_at_mut(l + 1);
        let (c, wt, lw, _) = &mut lo[l];
        push(c, wt, *lw, &hi[0].0, hi[0].2, hi[0].3);
    }
    push(&mut out, weight, w, &levels[0].0, levels[0].2, levels[0].3);
    out
}

type PyrLevel = (Vec<Rgb>, Vec<f32>, usize, usize);

fn pull(c: &[Rgb], wt: &[f32], lw: usize, lh: usize) -> PyrLevel {
    let (nw, nh) = (lw.div_ceil(2), lh.div_ceil(2));
    let next: Vec<(Rgb, f32)> = (0..nw * nh)
        .into_par_iter()
        .map(|i| {
            let (x, y) = (i % nw, i / nw);
            let (mut s, mut ws) = ([0f32; 3], 0f32);
            for sy in 2 * y..(2 * y + 2).min(lh) {
                for sx in 2 * x..(2 * x + 2).min(lw) {
                    let j = sy * lw + sx;
                    for (a, v) in s.iter_mut().zip(c[j]) {
                        *a += wt[j] * v;
                    }
                    ws += wt[j];
                }
            }
            if ws > 0.0 {
                ([s[0] / ws, s[1] / ws, s[2] / ws], ws.min(1.0))
            } else {
                ([0.0; 3], 0.0)
            }
        })
        .collect();
    (next.iter().map(|p| p.0).collect(), next.iter().map(|p| p.1).collect(), nw, nh)
}

/// `c = wt·c + (1 − wt)·bilinear(above)`.
fn push(c: &mut [Rgb], wt: &[f32], lw: usize, above: &[Rgb], cw: usize, ch: usize) {
    c.par_chunks_mut(lw).zip(wt.par_chunks(lw)).enumerate().for_each(|(y, (row, wrow))| {
        let fy = ((y as f64 + 0.5) / 2.0 - 0.5).clamp(0.0, (ch - 1) as f64);
        let (y0, ty) = (fy.floor() as usize, (fy - fy.floor()) as f32);
        let y1 = (y0 + 1).min(ch - 1);
        for (x, px) in row.iter_mut().enumerate() {
            let fx = ((x as f64 + 0.5) / 2.0 - 0.5).clamp(0.0, (cw - 1) as f64);
            let (x0, tx) = (fx.floor() as usize, (fx - fx.floor()) as f32);
            let x1 = (x0 + 1).min(cw - 1);
            let wv = wrow[x];
            for k in 0..3 {
                let top = above[y0 * cw + x0][k] + (above[y0 * cw + x1][k] - above[y0 * cw + x0][k]) * tx;
                let bot = above[y1 * cw + x0][k] + (above[y1 * cw + x1][k] - above[y1 * cw + x0][k]) * tx;
                px[k] = wv * px[k] + (1.0 - wv) * (top + (bot - top) * ty);
            }
        }
    });
}

// ---------------------------------------------------------------------------------------
// foreground estimation (decontamination)

/// Foreground colors for the band's bounding box (grown for context): `(colors, bounds)`.
fn estimate_foreground_in(
    rgba: &[f32],
    alpha: &[f32],
    w: usize,
    h: usize,
    region: RectI,
) -> (Vec<Rgb>, (usize, usize, usize, usize)) {
    let margin = (region.w.max(region.h) as usize / 8).clamp(8, 64);
    let b = grow(region, margin, w, h);
    let (rw, rh) = (b.2 - b.0, b.3 - b.1);
    let (f, _) = estimate_fb(&crop_rgb(rgba, w, b), &crop(alpha, w, b), rw, rh);
    (f, b)
}

/// Fast multi-level foreground (and background) estimation (Germer et al. 2021).
fn estimate_fb(img: &[Rgb], alpha: &[f32], w: usize, h: usize) -> (Vec<Rgb>, Vec<Rgb>) {
    const REG: f32 = 1e-5;
    const GRAD: f32 = 1.0;
    const SMALL: usize = 32;
    const SMALL_ITERS: usize = 10;
    const BIG_ITERS: usize = 2;
    // starting colors: the mean of the clearly foreground / background pixels
    let mean_where = |keep: &(dyn Fn(f32) -> bool + Sync)| -> Rgb {
        let rows: Vec<([f64; 3], f64)> = img
            .par_chunks(w)
            .zip(alpha.par_chunks(w))
            .map(|(r, a)| {
                let mut s = ([0f64; 3], 0f64);
                for (c, &v) in r.iter().zip(a) {
                    if keep(v) {
                        for (acc, &ch) in s.0.iter_mut().zip(c) {
                            *acc += ch as f64;
                        }
                        s.1 += 1.0;
                    }
                }
                s
            })
            .collect();
        let (mut s, mut n) = ([0f64; 3], 0f64);
        for r in rows {
            for (acc, v) in s.iter_mut().zip(r.0) {
                *acc += v;
            }
            n += r.1;
        }
        if n > 0.0 {
            [(s[0] / n) as f32, (s[1] / n) as f32, (s[2] / n) as f32]
        } else {
            [0.5; 3]
        }
    };
    let mut fp = vec![mean_where(&|a| a > 0.9)];
    let mut bp = vec![mean_where(&|a| a < 0.1)];
    let (mut pw, mut ph) = (1usize, 1usize);
    let n_levels = ((w.max(h) as f64).log2().ceil() as usize).max(1);
    let mut scratch: Vec<(Rgb, Rgb)> = Vec::new();
    for level in 0..=n_levels {
        let t = level as f64 / n_levels as f64;
        let lw = ((w as f64).powf(t).round() as usize).clamp(1, w);
        let lh = ((h as f64).powf(t).round() as usize).clamp(1, h);
        let nearest = |x: usize, n_dst: usize, n_src: usize| ((x * n_src) / n_dst).min(n_src - 1);
        let full = lw == w && lh == h;
        let (im, al): (Cow<[Rgb]>, Cow<[f32]>) = if full {
            (Cow::Borrowed(img), Cow::Borrowed(alpha))
        } else {
            let idx = |i: usize| nearest(i / lw, lh, h) * w + nearest(i % lw, lw, w);
            (
                Cow::Owned((0..lw * lh).into_par_iter().map(|i| img[idx(i)]).collect()),
                Cow::Owned((0..lw * lh).into_par_iter().map(|i| alpha[idx(i)]).collect()),
            )
        };
        let up = |v: &[Rgb]| -> Vec<Rgb> {
            (0..lw * lh).into_par_iter().map(|i| v[nearest(i / lw, lh, ph) * pw + nearest(i % lw, lw, pw)]).collect()
        };
        let mut f = up(&fp);
        let mut bg = up(&bp);
        drop((fp, bp));
        let iters = if lw <= SMALL && lh <= SMALL { SMALL_ITERS } else { BIG_ITERS };
        scratch.resize(lh * lw.div_ceil(2), ([0.0; 3], [0.0; 3]));
        for _ in 0..iters {
            for parity in 0..2 {
                fb_sweep(&im, &al, &mut f, &mut bg, lw, lh, parity, &mut scratch, REG, GRAD);
            }
        }
        fp = f;
        bp = bg;
        pw = lw;
        ph = lh;
    }
    (fp, bp)
}

/// One red-black half sweep: the pixels with `(x + y) % 2 == parity` solve their 2×2
/// system per channel from their 4 neighbors (which all have the other parity), computed
/// into `scratch` and then written back, so the result never depends on the schedule.
#[allow(clippy::too_many_arguments)]
fn fb_sweep(
    im: &[Rgb],
    al: &[f32],
    f: &mut [Rgb],
    b: &mut [Rgb],
    w: usize,
    h: usize,
    parity: usize,
    scratch: &mut [(Rgb, Rgb)],
    reg: f32,
    grad: f32,
) {
    let hw = w.div_ceil(2);
    let (fr, br) = (&*f, &*b);
    scratch.par_chunks_mut(hw).enumerate().for_each(|(y, out)| {
        let x0 = (parity + y) % 2;
        for (o, x) in out.iter_mut().zip((x0..w).step_by(2)) {
            let i = y * w + x;
            let a0 = al[i];
            let a1 = 1.0 - a0;
            let (mut a00, a01, mut a11) = (a0 * a0, a0 * a1, a1 * a1);
            let c = im[i];
            let mut b0 = [a0 * c[0], a0 * c[1], a0 * c[2]];
            let mut b1 = [a1 * c[0], a1 * c[1], a1 * c[2]];
            let mut visit = |j: usize| {
                let da = reg + grad * (a0 - al[j]).abs();
                a00 += da;
                a11 += da;
                for k in 0..3 {
                    b0[k] += da * fr[j][k];
                    b1[k] += da * br[j][k];
                }
            };
            // neighbors clamped at the border (the pixel itself), as in the reference
            visit(if x > 0 { i - 1 } else { i });
            visit(if x + 1 < w { i + 1 } else { i });
            visit(if y > 0 { i - w } else { i });
            visit(if y + 1 < h { i + w } else { i });
            let inv = 1.0 / (a00 * a11 - a01 * a01);
            for k in 0..3 {
                o.0[k] = (inv * (a11 * b0[k] - a01 * b1[k])).clamp(0.0, 1.0);
                o.1[k] = (inv * (a00 * b1[k] - a01 * b0[k])).clamp(0.0, 1.0);
            }
        }
    });
    f.par_chunks_mut(w).zip(b.par_chunks_mut(w)).zip(scratch.par_chunks(hw)).enumerate().for_each(
        |(y, ((rf, rb), s))| {
            let x0 = (parity + y) % 2;
            for (v, x) in s.iter().zip((x0..w).step_by(2)) {
                rf[x] = v.0;
                rb[x] = v.1;
            }
        },
    );
}

/// Write the estimated foreground colors into the layer's pixels where the refined alpha is
/// non-zero in the band; returns how many pixels changed.
fn write_colors(
    layer: &mut Layer,
    fg: &(Vec<Rgb>, (usize, usize, usize, usize)),
    alpha: &[f32],
    tri: &[u8],
    w: usize,
    h: usize,
) -> Result<usize> {
    let (f, b) = fg;
    let rw = b.2 - b.0;
    let (lx, ly) = (layer.x as i64, layer.y as i64);
    let img = layer.raster_mut()?;
    let iw = img.width as usize;
    let counts: Vec<usize> = img
        .data
        .par_chunks_mut(iw * 4)
        .enumerate()
        .map(|(y, row)| {
            let cy = y as i64 + ly;
            if cy < b.1 as i64 || cy >= b.3 as i64 || cy >= h as i64 {
                return 0;
            }
            let cy = cy as usize;
            let mut n = 0;
            for x in 0..iw {
                let cx = x as i64 + lx;
                if cx < b.0 as i64 || cx >= b.2 as i64 || cx >= w as i64 {
                    continue;
                }
                let cx = cx as usize;
                let i = cy * w + cx;
                if tri[i] != UNKNOWN || alpha[i] < BBOX_THRESHOLD {
                    continue;
                }
                let c = f[(cy - b.1) * rw + (cx - b.0)];
                let px = &mut row[x * 4..x * 4 + 3];
                if px[0] != c[0] || px[1] != c[1] || px[2] != c[2] {
                    px.copy_from_slice(&c);
                    n += 1;
                }
            }
            n
        })
        .collect();
    Ok(counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::ctx::{Globals, NoIo, NoProgress};
    use pixelox_core::op::Runner;
    use pixelox_core::testing;

    /// Deterministic noise in -0.5..0.5.
    fn noise(x: u32, y: u32, seed: u32) -> f32 {
        let mut v = x.wrapping_mul(374761393) ^ y.wrapping_mul(668265263) ^ seed.wrapping_mul(2246822519);
        v = (v ^ (v >> 13)).wrapping_mul(1274126177);
        v ^= v >> 16;
        (v & 0xffff) as f32 / 65535.0 - 0.5
    }

    fn smooth(e0: f32, e1: f32, x: f32) -> f32 {
        let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// Distance from (px, py) to the segment a-b.
    fn seg_dist(px: f32, py: f32, a: (f32, f32), b: (f32, f32)) -> f32 {
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let t = (((px - a.0) * dx + (py - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
        ((px - a.0 - t * dx).powi(2) + (py - a.1 - t * dy).powi(2)).sqrt()
    }

    struct Scene {
        img: Image,
        /// Ground truth.
        alpha: Vec<f32>,
        fg: Vec<Rgb>,
        rough: Mask,
    }

    const S: u32 = 96;
    const CX: f32 = 46.0;
    const CY: f32 = 54.0;
    const R: f32 = 24.0;

    fn bg_color(x: u32, y: u32) -> Rgb {
        let t = y as f32 / S as f32;
        [0.06 + 0.04 * t, 0.12 + 0.05 * t, 0.50 + 0.12 * (x as f32 / S as f32)]
    }

    fn fg_color(x: u32, y: u32) -> Rgb {
        let v = 0.06 * noise(x / 3, y / 3, 7);
        [0.50 + v, 0.33 + 0.8 * v, 0.18 + 0.5 * v]
    }

    /// A soft-edged brown head with nine thin hair strands (all in the upper half) over a
    /// strong blue with sensor noise, and a coarse rough mask: an 8 px blocky disc a bit
    /// too small, blurred, missing every strand.
    fn scene() -> Scene {
        let strands: Vec<((f32, f32), (f32, f32))> = (0..9)
            .map(|k| {
                let ang = -2.6 + k as f32 * 0.3;
                let (c, s) = (ang.cos(), ang.sin());
                (
                    (CX + c * (R - 4.0), CY + s * (R - 4.0)),
                    (CX + c * (R + 13.0) + 3.0 * s, CY + s * (R + 13.0) - 3.0 * c),
                )
            })
            .collect();
        let alpha: Vec<f32> = (0..S * S)
            .map(|i| {
                let (x, y) = ((i % S) as f32 + 0.5, (i / S) as f32 + 0.5);
                let d = ((x - CX).powi(2) + (y - CY).powi(2)).sqrt();
                let head = 1.0 - smooth(R - 1.5, R + 1.5, d);
                let hair = strands
                    .iter()
                    .map(|&(a, b)| 0.85 * (1.0 - smooth(0.2, 1.0, seg_dist(x, y, a, b))))
                    .fold(0f32, f32::max);
                1.0 - (1.0 - head) * (1.0 - hair)
            })
            .collect();
        let fg: Vec<Rgb> = (0..S * S).map(|i| fg_color(i % S, i / S)).collect();
        let img = Image::from_fn(S, S, |x, y| {
            let i = (y * S + x) as usize;
            let (a, f, b) = (alpha[i], fg[i], bg_color(x, y));
            let n = 0.008 * noise(x, y, 1);
            let c = |k: usize| (a * f[k] + (1.0 - a) * b[k] + n).clamp(0.0, 1.0);
            [c(0), c(1), c(2), 1.0]
        });
        let blocky = Mask::from_fn(S, S, |x, y| {
            let (bx, by) = ((x / 8 * 8) as f32 + 4.0, (y / 8 * 8) as f32 + 4.0);
            if ((bx - CX).powi(2) + (by - CY).powi(2)).sqrt() < R - 2.0 {
                1.0
            } else {
                0.0
            }
        });
        let rough = kernels::blur_mask(&blocky, 2.5);
        Scene { img, alpha, fg, rough }
    }

    fn sad(a: &[f32], b: &[f32]) -> f64 {
        a.iter().zip(b).map(|(x, y)| (x - y).abs() as f64).sum()
    }

    fn mse(a: &[f32], b: &[f32]) -> f64 {
        a.iter().zip(b).map(|(x, y)| ((x - y) as f64).powi(2)).sum::<f64>() / a.len() as f64
    }

    /// Mean of `v` over the pixels where `keep` holds.
    fn mean_where(v: &[f32], keep: impl Fn(usize) -> bool) -> f64 {
        let (s, n) = (0..v.len()).filter(|&i| keep(i)).fold((0f64, 0usize), |(s, n), i| (s + v[i] as f64, n + 1));
        assert!(n > 0);
        s / n as f64
    }

    /// Strand pixels (beyond the head) the rough mask misses entirely.
    fn missed_strand(sc: &Scene, i: usize) -> bool {
        let (x, y) = ((i as u32 % S) as f32 + 0.5, (i as u32 / S) as f32 + 0.5);
        let beyond = ((x - CX).powi(2) + (y - CY).powi(2)).sqrt() > R + 2.0;
        beyond && sc.alpha[i] > 0.6 && sc.rough.data[i] < 0.02
    }

    fn spec() -> OpSpec {
        testing::find(&ops(), "mask refine")
    }

    fn run(pairs: &[(&str, &str)], doc: &mut Document) -> Result<Value> {
        testing::run(&spec(), pairs, doc)
    }

    /// Run and also return the values resolved at run time (what replay records).
    fn run_resolved(pairs: &[(&str, &str)], doc: &mut Document) -> Result<(Value, Map<String, Value>)> {
        let s = spec();
        let a = testing::args(&s, pairs)?;
        let mut ctx = OpCtx::new(s.name, vec![], Globals::default(), &NoIo, &NoProgress);
        let v = match s.runner {
            Runner::Doc(f) => f(&a, doc, &mut ctx)?,
            _ => unreachable!("mask refine is a doc op"),
        };
        Ok((v, ctx.resolved.clone()))
    }

    fn scene_doc(sc: &Scene) -> Document {
        let mut d = testing::doc_from(sc.img.clone());
        d.masks.insert("subject".into(), sc.rough.clone());
        d
    }

    #[test]
    fn recovers_strands_and_the_true_edge() {
        let sc = scene();
        let mut d = scene_doc(&sc);
        let v = run(&[("mask", "subject"), ("band", "16")], &mut d).unwrap();
        let m = &d.masks["subject"].data;
        let (sad0, sad1) = (sad(&sc.rough.data, &sc.alpha), sad(m, &sc.alpha));
        let (mse0, mse1) = (mse(&sc.rough.data, &sc.alpha), mse(m, &sc.alpha));
        assert!(sad1 < 0.15 * sad0, "SAD {sad0:.1} -> {sad1:.1}");
        assert!(mse1 < 0.05 * mse0, "MSE {mse0:.5} -> {mse1:.5}");
        // the strands come back, the head is solid, and the background stays clear
        let strands = mean_where(m, |i| missed_strand(&sc, i));
        assert!(strands > 0.6, "missed strands now at {strands:.3}");
        assert!(mean_where(m, |i| sc.alpha[i] == 1.0) > 0.97);
        assert!(mean_where(m, |i| sc.alpha[i] == 0.0) < 0.01);
        assert_eq!(v["into"], json!("subject"));
        assert_eq!(v["band"], json!(16.0));
        assert_eq!(v["source"], json!("composite"));
        assert!(v["trimap"]["unknown"].as_u64().unwrap() > 1000);
        assert!(v["solver"]["iterations"].as_u64().unwrap() > 0);
        assert!(v["solver"]["levels"].as_u64().unwrap() >= 2);
        assert!(v["change"]["added"].as_f64().unwrap() > 100.0);
        assert_eq!(v["decontaminated"], Value::Null);
        assert_eq!(v["stats"], mask_json(&d.masks["subject"]));
    }

    #[test]
    fn decontaminate_removes_the_background_tint() {
        let sc = scene();
        let mut d = scene_doc(&sc);
        let v = run(&[("mask", "subject"), ("band", "16"), ("decontaminate", "true")], &mut d).unwrap();
        let out = testing::raster(&d, "Background");
        let mixed = |i: usize| sc.alpha[i] > 0.15 && sc.alpha[i] < 0.85;
        let px = |img: &Image, i: usize| [img.data[i * 4], img.data[i * 4 + 1], img.data[i * 4 + 2]];
        let dist = |c: Rgb, f: Rgb| ((c[0] - f[0]).powi(2) + (c[1] - f[1]).powi(2) + (c[2] - f[2]).powi(2)).sqrt();
        let blue = |c: Rgb| c[2] - (c[0] + c[1]) / 2.0;
        let n = S as usize * S as usize;
        let err = |img: &Image| mean_where(&(0..n).map(|i| dist(px(img, i), sc.fg[i])).collect::<Vec<_>>(), mixed);
        let tint =
            |img: &Image| mean_where(&(0..n).map(|i| blue(px(img, i)) - blue(sc.fg[i])).collect::<Vec<_>>(), mixed);
        let (e0, e1) = (err(&sc.img), err(out));
        let (t0, t1) = (tint(&sc.img), tint(out));
        assert!(e1 < 0.25 * e0, "color error to the true foreground {e0:.4} -> {e1:.4}");
        assert!(t0 > 0.15 && t1.abs() < 0.03, "blue tint {t0:.4} -> {t1:.4}");
        // only band pixels change
        let tri = trimap(&sc.rough.data, None, 16.0, S as usize, S as usize);
        assert!(tri.iter().filter(|&&t| t != UNKNOWN).count() > n / 2);
        for i in (0..n).filter(|&i| tri[i] != UNKNOWN) {
            assert_eq!(px(out, i), px(&sc.img, i), "pixel {i} is outside the band");
        }
        let changed = (0..n).filter(|&i| px(out, i) != px(&sc.img, i)).count();
        assert_eq!(v["decontaminated"], json!({"layer": "Background", "pixels": changed}));
        assert!(changed > 300);
        assert_eq!(out.data.chunks(4).map(|p| p[3]).fold(1f32, f32::min), 1.0, "alpha is untouched");
    }

    #[test]
    fn force_foreground_takes_alpha_from_the_color() {
        let sc = scene();
        let mut d = scene_doc(&sc);
        let v = run(&[("mask", "subject"), ("band", "16"), ("force-foreground", "#80542e")], &mut d).unwrap();
        let m = &d.masks["subject"].data;
        let (sad0, sad1) = (sad(&sc.rough.data, &sc.alpha), sad(m, &sc.alpha));
        assert!(sad1 < 0.25 * sad0, "SAD {sad0:.1} -> {sad1:.1}");
        assert!(mean_where(m, |i| missed_strand(&sc, i)) > 0.6);
        assert_eq!(v["solver"]["method"], json!("force-foreground"));
        assert_eq!(v["force_foreground"], json!("#80542e"));
        assert!(mean_where(m, |i| sc.alpha[i] == 0.0) < 0.02, "the background stays clear");
        assert!(mean_where(m, |i| sc.alpha[i] == 1.0) > 0.95, "the head stays solid");
    }

    #[test]
    fn painted_unknown_band_is_respected() {
        let sc = scene();
        // Extract's highlighter over the lower half of the edge only (no strands there)
        let ring = |lower: bool| {
            Mask::from_fn(S, S, move |x, y| {
                let d = ((x as f32 + 0.5 - CX).powi(2) + (y as f32 + 0.5 - CY).powi(2)).sqrt();
                if (d - R).abs() < 16.0 && ((y as f32) >= CY) == lower {
                    1.0
                } else {
                    0.0
                }
            })
        };
        for (lower, strands_back) in [(true, false), (false, true)] {
            let mut d = scene_doc(&sc);
            let band = ring(lower);
            d.masks.insert("band".into(), band.clone());
            let (v, resolved) = run_resolved(&[("mask", "subject"), ("unknown", "band")], &mut d).unwrap();
            assert_eq!(resolved.get("band"), Some(&json!("0")), "no automatic band with --unknown");
            assert_eq!(
                v["trimap"]["unknown"].as_u64().unwrap() as f64,
                band.data.iter().map(|&v| v as f64).sum::<f64>()
            );
            let m = &d.masks["subject"].data;
            for (i, (&a, &r)) in m.iter().zip(&sc.rough.data).enumerate() {
                if band.data[i] < 0.5 {
                    assert_eq!(a, r, "outside the painted band the rough mask stands (pixel {i})");
                }
            }
            let strands = mean_where(m, |i| missed_strand(&sc, i));
            let ok = if strands_back { strands > 0.5 } else { strands < 0.02 };
            assert!(ok, "painted lower half {lower}: missed strands at {strands:.3}");
        }
        // with --band too, the two unite
        let mut d = scene_doc(&sc);
        d.masks.insert("band".into(), ring(true));
        let painted =
            run(&[("mask", "subject"), ("unknown", "band")], &mut d).unwrap()["trimap"]["unknown"].as_u64().unwrap();
        let mut d = scene_doc(&sc);
        d.masks.insert("band".into(), ring(true));
        let both = run(&[("mask", "subject"), ("unknown", "band"), ("band", "6")], &mut d).unwrap()["trimap"]
            ["unknown"]
            .as_u64()
            .unwrap();
        assert!(both > painted);
    }

    #[test]
    fn writes_where_told() {
        let sc = scene();
        let base = || {
            let mut d = scene_doc(&sc);
            d.selection = Some(sc.rough.clone());
            d
        };
        // @sel replaces the selection
        let mut d = base();
        let v = run(&[("mask", "@sel"), ("band", "16")], &mut d).unwrap();
        assert_eq!(v["into"], json!("@sel"));
        assert_ne!(d.selection.as_ref().unwrap(), &sc.rough);
        assert_eq!(d.masks["subject"], sc.rough, "the library is untouched");
        // --into a new library mask leaves the input alone
        let mut d = base();
        run(&[("mask", "subject"), ("band", "16"), ("into", "hair")], &mut d).unwrap();
        assert_eq!(d.masks["subject"], sc.rough);
        assert!(sad(&d.masks["hair"].data, &sc.alpha) < 0.15 * sad(&sc.rough.data, &sc.alpha));
        // @mask:LAYER rewrites that layer mask and drops a stale vector path
        let mut d = base();
        {
            let l = &mut d.layers[0];
            l.mask = Some(LayerMask { density: 80.0, ..LayerMask::new(sc.rough.clone()) });
            l.vector_mask = Some("M 0 0 H 10 V 10 Z".into());
        }
        let v = run(&[("mask", "@mask:Background"), ("band", "16")], &mut d).unwrap();
        assert_eq!(v["into"], json!("@mask:Background"));
        assert_eq!(v["source"], json!("Background"));
        let lm = d.layers[0].mask.as_ref().unwrap();
        assert_ne!(lm.mask, sc.rough);
        assert_eq!(lm.density, 80.0, "layer mask properties are kept");
        assert!(d.layers[0].vector_mask.is_none());
        // --into @mask:LAYER attaches a mask where there was none
        let mut d = base();
        run(&[("mask", "subject"), ("band", "16"), ("into", "@mask:Background")], &mut d).unwrap();
        assert!(d.layers[0].mask.is_some());
        // read-only references need --into; bad targets are refused before any work
        let mut d = base();
        assert_eq!(run(&[("mask", "@alpha:Background")], &mut d).unwrap_err().code, "PX1003");
        for bad in ["@nope", "hair.png", ""] {
            assert_eq!(
                run(&[("mask", "subject"), ("into", bad)], &mut d).unwrap_err().code,
                "PX1010",
                "--into '{bad}'"
            );
        }
        assert_eq!(run(&[("mask", "missing")], &mut d).unwrap_err().code, "PX2311");
        let mut d = scene_doc(&sc);
        assert_eq!(run(&[("mask", "@sel")], &mut d).unwrap_err().code, "PX2312");
    }

    #[test]
    fn vitmatte_is_not_in_this_release() {
        let mut d = scene_doc(&scene());
        let e = run(&[("mask", "subject"), ("model", "vitmatte")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX1520");
        assert_eq!(e.exit_code, 12);
        assert_eq!(d.masks["subject"], scene().rough, "nothing changed");
    }

    #[test]
    fn deterministic_across_thread_counts() {
        let sc = scene();
        let on = |threads: usize| {
            let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
            pool.install(|| {
                let mut d = scene_doc(&sc);
                let v = run(&[("mask", "subject"), ("band", "16"), ("decontaminate", "true")], &mut d).unwrap();
                (d.masks["subject"].clone(), testing::raster(&d, "Background").clone(), v)
            })
        };
        let one = on(1);
        for t in [3, 4] {
            let other = on(t);
            assert!(one.0 == other.0, "mask differs with {t} threads");
            assert!(one.1.data == other.1.data, "colors differ with {t} threads");
            assert_eq!(one.2, other.2);
        }
    }

    #[test]
    fn auto_band_is_recorded_and_replays_exactly() {
        assert_eq!(auto_band(3000, 4000), 32.0);
        assert_eq!(auto_band(96, 96), MIN_BAND);
        // a canvas whose auto band is not a whole number before rounding
        let img = Image::from_fn(437, 120, |x, y| {
            let inside = (x as f32 - 200.0).powi(2) + (y as f32 - 60.0).powi(2) < 40.0f32.powi(2);
            let c = if inside { [0.6, 0.4, 0.2] } else { [0.1, 0.2, 0.7] };
            [c[0] + 0.01 * noise(x, y, 3), c[1], c[2], 1.0]
        });
        let rough = kernels::blur_mask(
            &Mask::from_fn(437, 120, |x, y| {
                if (x as f32 - 202.0).powi(2) + (y as f32 - 60.0).powi(2) < 37.0f32.powi(2) {
                    1.0
                } else {
                    0.0
                }
            }),
            2.0,
        );
        let mut d = testing::doc_from(img.clone());
        d.masks.insert("m".into(), rough.clone());
        let (v, resolved) = run_resolved(&[("mask", "m")], &mut d).unwrap();
        let recorded = resolved["band"].as_str().unwrap().to_string();
        assert_eq!(recorded, auto_band(437, 120).to_string());
        assert_eq!(v["band"], json!(auto_band(437, 120)));
        let mut again = testing::doc_from(img);
        again.masks.insert("m".into(), rough);
        let (_, none) = run_resolved(&[("mask", "m"), ("band", &recorded)], &mut again).unwrap();
        assert!(none.is_empty(), "an explicit band records nothing");
        assert!(d.masks["m"] == again.masks["m"], "replaying band {recorded} changed the mask");
    }

    #[test]
    fn refuses_what_it_cannot_refine() {
        let sc = scene();
        let mut d = scene_doc(&sc);
        for (name, m) in [("on", Mask::filled(S, S, 1.0)), ("off", Mask::new(S, S))] {
            d.masks.insert(name.into(), m);
            assert_eq!(run(&[("mask", name)], &mut d).unwrap_err().code, "PX3320", "{name}");
        }
        // a band wider than the subject leaves no definite foreground
        let e = run(&[("mask", "subject"), ("band", "40")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX3320");
        assert!(e.message.contains("foreground"), "{}", e.message);
        // an empty painted band
        d.masks.insert("blank".into(), Mask::new(S, S));
        assert_eq!(run(&[("mask", "subject"), ("unknown", "blank")], &mut d).unwrap_err().code, "PX3320");
        // a 1x1 canvas has no edge
        let mut tiny = testing::doc_from(Image::filled(1, 1, Rgba::WHITE));
        tiny.selection = Some(Mask::filled(1, 1, 1.0));
        assert_eq!(run(&[("mask", "@sel")], &mut tiny).unwrap_err().code, "PX3320");
        // decontaminating needs one unlocked raster layer
        let mut two = testing::doc_layers(S, S, vec![("Photo", sc.img.clone()), ("Top", sc.img.clone())]);
        two.masks.insert("subject".into(), sc.rough.clone());
        assert_eq!(run(&[("mask", "subject"), ("decontaminate", "true")], &mut two).unwrap_err().code, "PX2302");
        run(&[("mask", "subject"), ("band", "16"), ("into", "x")], &mut two).unwrap();
        two.layers[0].locked = true;
        let e = run(&[("mask", "subject"), ("decontaminate", "true"), ("layer", "Photo")], &mut two).unwrap_err();
        assert_eq!(e.code, "PX2304");
    }

    #[test]
    fn transparent_source_pixels_cannot_be_foreground() {
        let sc = scene();
        let mut img = sc.img.clone();
        for (i, p) in img.data.chunks_mut(4).enumerate() {
            if i as u32 % S > 60 {
                p[3] = 0.0;
            }
        }
        let mut d = testing::doc_from(img);
        d.masks.insert("subject".into(), sc.rough.clone());
        run(&[("mask", "subject"), ("band", "16")], &mut d).unwrap();
        let m = &d.masks["subject"].data;
        for (i, (&a, &r)) in m.iter().zip(&sc.rough.data).enumerate() {
            if i as u32 % S > 60 {
                assert!(a == 0.0 || a == r, "pixel {i}: {a} (rough {r})");
            }
        }
    }

    #[test]
    fn edt_matches_brute_force() {
        let (w, h) = (23, 17);
        let feat: Vec<bool> = (0..w * h).map(|i| (i * 7919) % 31 == 0).collect();
        let d = edt_sq(&feat, w, h);
        for y in 0..h {
            for x in 0..w {
                let mut best = f64::MAX;
                for (j, _) in feat.iter().enumerate().filter(|(_, &f)| f) {
                    let (fx, fy) = ((j % w) as f64, (j / w) as f64);
                    best = best.min((fx - x as f64).powi(2) + (fy - y as f64).powi(2));
                }
                assert_eq!(d[y * w + x] as f64, best, "at {x},{y}");
            }
        }
    }

    #[test]
    fn foreground_estimate_inverts_compositing() {
        // F·α + B·(1−α) = I with flat F and B: the estimate recovers both
        let (w, h) = (40, 30);
        let (f, b) = ([0.8f32, 0.5, 0.2], [0.1f32, 0.2, 0.9]);
        let alpha: Vec<f32> = (0..w * h).map(|i| (i % w) as f32 / (w - 1) as f32).collect();
        let img: Vec<Rgb> = alpha.iter().map(|&a| [0, 1, 2].map(|k| a * f[k] + (1.0 - a) * b[k])).collect();
        let (fe, be) = estimate_fb(&img, &alpha, w, h);
        for i in 0..w * h {
            for k in 0..3 {
                if alpha[i] > 0.2 {
                    assert!((fe[i][k] - f[k]).abs() < 0.02, "F at {i}: {:?}", fe[i]);
                }
                if alpha[i] < 0.8 {
                    assert!((be[i][k] - b[k]).abs() < 0.02, "B at {i}: {:?}", be[i]);
                }
            }
        }
    }

    #[test]
    fn pull_push_fills_gaps_and_keeps_samples() {
        let (w, h) = (17, 9);
        let img: Vec<Rgb> = (0..w * h).map(|i| if i % w < 8 { [1.0, 0.0, 0.0] } else { [0.0, 0.0, 1.0] }).collect();
        let weight: Vec<f32> = (0..w * h).map(|i| if i % w == 0 || i % w == w - 1 { 1.0 } else { 0.0 }).collect();
        let out = pull_push(&img, &weight, w, h);
        for y in 0..h {
            assert_eq!(out[y * w], [1.0, 0.0, 0.0]);
            assert_eq!(out[y * w + w - 1], [0.0, 0.0, 1.0]);
            let mid = out[y * w + w / 2];
            assert!(mid[0] > 0.1 && mid[2] > 0.1, "the middle mixes both: {mid:?}");
        }
    }

    #[test]
    fn sci_keeps_three_digits() {
        assert_eq!(sci(0.000123456), 0.000123);
        assert_eq!(sci(0.0), 0.0);
    }
}
