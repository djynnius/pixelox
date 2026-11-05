//! Painting: `paint stroke`, a brush engine that stamps dabs along points or an SVG path
//! into a raster layer, a layer mask, the selection, or a named mask.
//!
//! The model is Photoshop's. Dabs fall every `--spacing` % of the (tapered, pressure-
//! scaled) diameter along each stroke. Within one stroke the coverage is
//! `S = 1 − Π(1 − dabᵢ)` with `dabᵢ = tip × flow × pressure`, so low flow builds up where
//! dabs overlap but never past 1; `S × opacity × region` is then mixed into the target.
//! Several strokes are applied one after another, each capped by the opacity on its own.
//!
//! Cost is O(dab area): dabs are binned into fixed 16-row bands of the target, and each
//! band multiplies its dabs in placement order, so the bytes never depend on the thread
//! count (AUTHORING rule 9). Jitter and scatter come from the op's seeded RNG.

use crate::util::{self, Buf};
use kurbo::{BezPath, PathEl};
use pixelox_core::blend::{composite_pixel, BLEND_MODES};
use pixelox_core::kernels::Edge;
use pixelox_core::prelude::*;
use pixelox_core::units::Length;
use rand_core::RngCore;
use rayon::prelude::*;

/// Rows per band of the coverage pass (fixed: the partition must not depend on threads).
const BAND: usize = 16;
/// Smallest distance between dabs, in px.
const MIN_STEP: f64 = 0.25;
/// Brush diameters accepted after resolving `--size`.
const MIN_SIZE: f64 = 0.5;
const MAX_SIZE: f64 = 10_000.0;
/// Dabs one call may place (memory guard: ~64 bytes each).
const MAX_DABS: usize = 2_000_000;

/// Blend modes a brush can use: every layer mode except pass-through (a group mode).
const PAINT_BLENDS: &[&str] = match BLEND_MODES.split_last() {
    Some((_, rest)) => rest,
    None => BLEND_MODES,
};

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("paint stroke", Category::Paint, "Paint brush strokes along points or a path", stroke)
        .about("Drags a brush along each stroke like Photoshop's Brush tool. Geometry (at least one; several add up in this order): --points 'x,y x,y ...' with ';' between strokes and an optional third value x,y,p for pen pressure 0..1 (px or %, negative = from the right/bottom, so start a stroke above or left of the canvas with --d or a points file); --points-file, JSON: an array of [x, y], [x, y, p], or {\"x\", \"y\", \"p\"} points, an object {\"points\": [...]} or {\"d\": \"M...\"}, or an array of point arrays for several strokes (canvas px); --d, SVG path data in canvas px, one stroke per subpath, curves followed. Dabs fall every --spacing % of the diameter; within a stroke they accumulate as 1 − Π(1 − flow × tip) (low --flow builds up with overlap) and the stroke is capped by --opacity; each stroke is capped on its own, then the next is applied. --hardness shapes the round tip (0 = soft falloff from the center, 100 = crisp anti-aliased edge); --tip uses an image instead (its alpha, or 1 − luma when opaque, so dark paints), scaled so its longer side is --size, then squashed by --roundness and rotated by --angle (--follow adds the stroke direction). --taper shrinks the size to 10% at the stroke ends; point pressure scales size and/or flow (--pressure). --size-jitter, --angle-jitter, --scatter, and --count vary the dabs from the seeded RNG, so --seed reproduces a stroke exactly. Targets: --target pixels paints --layer's pixels with --color and --blend (painting beyond the layer grows it, within the canvas; --erase lowers alpha); --target mask paints the layer mask (added revealing all when missing); --target selection paints the selection (empty when none); --into NAME paints a named library mask (created black). On masks the value is the luma of --color (white reveals, black hides) and --erase paints 0. --mask / --invert-mask limit where paint lands. The result reports the dabs placed and the painted bounds (null when nothing landed).")
        .param(common::layer())
        .param(param("points", ParamKind::Str).help("Strokes 'x,y x,y ...' (';' between strokes, x,y,p adds pressure 0..1); px or %, negative = from the far edge"))
        .param(param("points-file", ParamKind::Path).help("JSON file of strokes in canvas px (see the command description)"))
        .param(param("d", ParamKind::Str).help("SVG path data in canvas px; each subpath is one stroke, curves followed"))
        .param(param("size", ParamKind::Length).default("50").help("Brush diameter in px, or % of the shorter canvas side"))
        .param(param("hardness", ParamKind::Float).default("0").range("0..100").help("Round-tip edge in % (0 = soft falloff, 100 = crisp anti-aliased edge)"))
        .param(param("flow", ParamKind::Float).default("100").range("1..100").help("Strength of each dab in %; low flow builds up where dabs overlap"))
        .param(param("opacity", ParamKind::Float).default("100").range("0..100").help("Maximum strength of each stroke in %; build-up never exceeds it"))
        .param(param("spacing", ParamKind::Float).default("10").range("1..1000").help("Distance between dabs in % of the brush diameter"))
        .param(param("angle", ParamKind::Angle).default("0").help("Tip rotation in degrees (counter-clockwise)"))
        .param(param("roundness", ParamKind::Float).default("100").range("1..100").help("Tip roundness in %; lower squashes the tip into an ellipse"))
        .param(param("follow", ParamKind::Flag).help("Rotate the tip along the stroke direction (added to --angle)"))
        .param(param("tip", ParamKind::Path).help("Brush-tip image: its alpha, or 1 - luma when opaque (dark paints); longer side = --size"))
        .param(param("size-jitter", ParamKind::Float).default("0").range("0..100").help("Random size reduction per dab, up to this % of the size"))
        .param(param("angle-jitter", ParamKind::Float).default("0").range("0..360").help("Random tip rotation spread in degrees (360 = any angle)"))
        .param(param("scatter", ParamKind::Float).default("0").range("0..1000").help("Random offset across the stroke, up to ± this % of the size"))
        .param(param("count", ParamKind::Int).default("1").range("1..16").help("Dabs per spacing step, each jittered and scattered on its own"))
        .param(param("taper", ParamKind::Str).help("START[,END]: grow from 10% size over the first START and shrink over the last END (px or % of the stroke)"))
        .param(param("pressure", ParamKind::Choice).choices(&["size", "flow", "both", "none"]).default("size").help("What point pressure scales: size, flow, both, or none"))
        .param(param("color", ParamKind::Color).default("#000000").help("Paint color; on masks its luma is the value (white reveals, black hides)"))
        .param(param("blend", ParamKind::Choice).choices(PAINT_BLENDS).default("normal").help("Blend mode of the paint over the layer's pixels (--target pixels only)"))
        .param(param("erase", ParamKind::Flag).help("Erase instead: lower the layer's alpha, or paint 0 into a mask"))
        .param(param("target", ParamKind::Choice).choices(&["pixels", "mask", "selection"]).default("pixels").help("What to paint: the layer's pixels, its layer mask, or the selection"))
        .param(param("into", ParamKind::Str).help("Paint a named mask in the document's library instead (created black if missing)"))
        .param(common::mask().help("Limit where paint lands: a named mask, @sel, @alpha:<layer>, or a grayscale image path"))
        .param(common::invert_mask())
        .example("pixelox paint stroke doc.pxo --layer Sky --target mask --points '0,58% 35%,62% 70%,57% 100%,61%' --size 420 --flow 8 --color black --in-place", "Fade a sky layer's mask edge with a big soft black brush at low flow")
        .example("pixelox paint stroke doc.pxo --layer Shadow --blend multiply --points '140,860 420,880 700,866' --size 220 --flow 10 --color '#2b211c' --in-place", "Build up a soft contact shadow at 10% flow on a multiply layer")
        .example("pixelox paint stroke doc.pxo --layer Grunge --tip chalk.png --d 'M 80 400 C 320 260 620 560 920 380' --size 90 --scatter 60 --size-jitter 40 --angle-jitter 360 --count 3 --in-place", "Drag a textured tip with scatter and jitter")
        .example("pixelox paint stroke doc.pxo --layer Beams --blend screen --points '22%,0 48%,100%' --size 30% --flow 6 --opacity 70 --taper 60%,0 --color '#fff3d6' --in-place", "Paint a light beam that widens away from its source")
        .example("pixelox paint stroke doc.pxo --target selection --points '210,300 420,310' --size 120 --hardness 80 --color white --in-place", "Add a brushed area to the selection")
        .example("pixelox paint stroke doc.pxo --into hair --points-file strands.json --size 6 --hardness 100 --color white --in-place", "Paint a named mask from a points file with pen pressure")
        .example("pixelox paint stroke photo.jpg -o out.png --points '100,100 300,120' --erase --size 60", "Erase a soft streak to transparency")]
}

// ---------------------------------------------------------------------------------------
// geometry

/// One stroke point in canvas px with pen pressure `p` (0..1).
#[derive(Clone, Copy, Debug, PartialEq)]
struct Pt {
    x: f64,
    y: f64,
    p: f64,
}

/// `--points`: strokes separated by `;`, points by whitespace, each `x,y` or `x,y,p`.
fn parse_points_arg(s: &str, w: f64, h: f64, dpi: f64) -> Result<Vec<Vec<Pt>>> {
    let bad = |tok: &str| {
        Error::new("PX1011", format!("--points: '{tok}' is not a point (x,y or x,y,pressure)"))
            .with_hint("--points '10,10 200,40 50%,80%' (';' between strokes)")
            .with_context("value", tok)
    };
    let mut out = Vec::new();
    for part in s.split(';') {
        let mut pts = Vec::new();
        for tok in part.split_whitespace() {
            let f: Vec<&str> = tok.split(',').collect();
            if f.len() != 2 && f.len() != 3 {
                return Err(bad(tok));
            }
            let x = Length::parse(f[0]).map_err(|_| bad(tok))?.resolve_pos(w, dpi);
            let y = Length::parse(f[1]).map_err(|_| bad(tok))?.resolve_pos(h, dpi);
            let p = match f.get(2) {
                Some(v) => v.trim().parse::<f64>().ok().filter(|v| v.is_finite()).ok_or_else(|| bad(tok))?,
                None => 1.0,
            };
            pts.push(Pt { x, y, p });
        }
        if !pts.is_empty() {
            out.push(pts);
        }
    }
    if out.is_empty() {
        return Err(bad(s));
    }
    Ok(out)
}

fn file_point(v: &Value) -> Option<Pt> {
    match v {
        Value::Array(a) if a.len() == 2 || a.len() == 3 => Some(Pt {
            x: a[0].as_f64()?,
            y: a[1].as_f64()?,
            p: match a.get(2) {
                Some(p) => p.as_f64()?,
                None => 1.0,
            },
        }),
        Value::Object(o) => Some(Pt {
            x: o.get("x")?.as_f64()?,
            y: o.get("y")?.as_f64()?,
            p: match o.get("p").or_else(|| o.get("pressure")) {
                Some(p) => p.as_f64()?,
                None => 1.0,
            },
        }),
        _ => None,
    }
}

/// Strokes from a points file (the `select path` format, plus optional pressure).
fn file_strokes(v: &Value) -> Result<Vec<Vec<Pt>>> {
    let bad = || {
        Error::new("PX1010", "the points file is not a list of points").with_hint(
            "write [[x, y], ...], [[x, y, pressure], ...], [{\"x\": .., \"y\": .., \"p\": ..}, ...], {\"points\": [...]}, {\"d\": \"M...\"}, or an array of point arrays for several strokes",
        )
    };
    if let Some(d) = v.get("d").and_then(Value::as_str) {
        return path_strokes(d);
    }
    let list = v.get("points").unwrap_or(v).as_array().ok_or_else(bad)?;
    let nested = list.first().is_some_and(|f| f.as_array().is_some_and(|a| a.first().is_some_and(|p| !p.is_number())));
    let groups: Vec<&Vec<Value>> =
        if nested { list.iter().map(|g| g.as_array().ok_or_else(bad)).collect::<Result<_>>()? } else { vec![list] };
    groups
        .into_iter()
        .map(|g| {
            if g.is_empty() {
                return Err(bad());
            }
            g.iter().map(file_point).collect::<Option<Vec<_>>>().ok_or_else(bad)
        })
        .collect()
}

/// One stroke per subpath of SVG path data (curves flattened to 0.1 px).
fn path_strokes(d: &str) -> Result<Vec<Vec<Pt>>> {
    let path = BezPath::from_svg(d).map_err(|e| {
        Error::new("PX1010", format!("invalid SVG path data: {e}"))
            .with_hint("--d 'M 10 10 C 80 0 120 90 200 40'")
            .with_context("d", d)
    })?;
    let pt = |q: kurbo::Point| Pt { x: q.x, y: q.y, p: 1.0 };
    let mut out = Vec::new();
    let mut cur: Vec<Pt> = Vec::new();
    let mut start: Option<kurbo::Point> = None;
    kurbo::flatten(&path, 0.1, |el| match el {
        PathEl::MoveTo(q) => {
            if !cur.is_empty() {
                out.push(std::mem::take(&mut cur));
            }
            cur.push(pt(q));
            start = Some(q);
        }
        PathEl::LineTo(q) => {
            if cur.is_empty() {
                if let Some(s) = start {
                    cur.push(pt(s));
                }
            }
            cur.push(pt(q));
        }
        PathEl::ClosePath => {
            if let (Some(s), false) = (start, cur.is_empty()) {
                cur.push(pt(s));
                out.push(std::mem::take(&mut cur));
            }
        }
        _ => {}
    });
    if !cur.is_empty() {
        out.push(cur);
    }
    if out.is_empty() {
        return Err(Error::new("PX1010", "the SVG path has no points").with_context("d", d));
    }
    Ok(out)
}

/// Every stroke from `--points`, `--points-file`, and `--d`, in that order.
fn gather(a: &Args, doc: &Document, ctx: &mut OpCtx) -> Result<Vec<Vec<Pt>>> {
    if !a.has("points") && !a.has("points-file") && !a.has("d") {
        return Err(Error::new("PX1003", "'paint stroke' needs --points, --points-file, or --d")
            .with_hint("pixelox paint stroke doc.pxo --points '100,100 300,140' --size 40 --in-place"));
    }
    let mut out = Vec::new();
    if let Some(s) = a.str("points") {
        out.extend(parse_points_arg(&s, doc.width as f64, doc.height as f64, doc.dpi())?);
    }
    if let Some(f) = a.path("points-file") {
        let bytes = ctx.read_aux(&f, "points", Some("points-file"))?;
        let v: Value = serde_json::from_slice(&bytes).map_err(|e| {
            Error::new("PX1010", format!("the points file is not valid JSON: {e}"))
                .with_context("path", f.display().to_string())
        })?;
        out.extend(file_strokes(&v)?);
    }
    if let Some(d) = a.str("d") {
        out.extend(path_strokes(&d)?);
    }
    for (i, s) in out.iter().enumerate() {
        for q in s {
            if !q.x.is_finite() || !q.y.is_finite() {
                return Err(Error::new("PX1011", "a stroke point is not a finite number").with_context("stroke", i));
            }
            if !(0.0..=1.0).contains(&q.p) {
                return Err(Error::new("PX1013", format!("pressure {} is outside 0..1", q.p))
                    .with_context("stroke", i)
                    .with_context("point", [q.x, q.y]));
            }
        }
    }
    Ok(out)
}

/// `--taper START[,END]` (one value tapers both ends).
fn parse_taper(s: &str) -> Result<(Length, Length)> {
    let bad = || {
        Error::new("PX1010", format!("--taper: '{s}' is not START[,END]"))
            .with_hint("--taper 40,80 (px) or --taper 20%,30% (of the stroke length)")
    };
    let parts: Vec<&str> = s.split(',').collect();
    let ls: Vec<Length> = parts.iter().map(|p| Length::parse(p).map_err(|_| bad())).collect::<Result<_>>()?;
    if ls.iter().any(|l| l.value < 0.0) {
        return Err(Error::new("PX1013", format!("--taper: '{s}' has a negative length")));
    }
    match ls.as_slice() {
        [b] => Ok((*b, *b)),
        [a, b] => Ok((*a, *b)),
        _ => Err(bad()),
    }
}

// ---------------------------------------------------------------------------------------
// brush

/// Resolved brush settings.
struct Cfg {
    /// Diameter in px.
    base: f64,
    /// Fraction of the diameter between dabs.
    spacing: f64,
    hardness: f64,
    flow: f32,
    /// Degrees, counter-clockwise.
    angle: f64,
    roundness: f64,
    follow: bool,
    size_jitter: f64,
    /// Spread in degrees.
    angle_jitter: f64,
    /// Fraction of the diameter.
    scatter: f64,
    count: usize,
    taper: Option<(Length, Length)>,
    p_size: bool,
    p_flow: bool,
    dpi: f64,
}

fn brush_cfg(a: &Args, doc: &Document) -> Result<Cfg> {
    let dpi = doc.dpi();
    let short = doc.width.min(doc.height) as f64;
    let size = a.length("size")?.map_or(50.0, |l| l.to_px(short, dpi));
    if !(MIN_SIZE..=MAX_SIZE).contains(&size) {
        return Err(Error::new("PX1013", format!("--size resolves to {size:.2} px; brushes are 0.5..10000 px"))
            .with_context("size", size));
    }
    let taper = a.str("taper").map(|s| parse_taper(&s)).transpose()?;
    let pressure = a.choice("pressure");
    Ok(Cfg {
        base: size,
        spacing: a.f64("spacing")? / 100.0,
        hardness: a.f64("hardness")? / 100.0,
        flow: (a.f64("flow")? / 100.0) as f32,
        angle: a.angle("angle")?,
        roundness: a.f64("roundness")? / 100.0,
        follow: a.flag("follow"),
        size_jitter: a.f64("size-jitter")? / 100.0,
        angle_jitter: a.f64("angle-jitter")?,
        scatter: a.f64("scatter")? / 100.0,
        count: a.usize("count")?.max(1),
        taper,
        p_size: matches!(pressure.as_str(), "size" | "both"),
        p_flow: matches!(pressure.as_str(), "flow" | "both"),
        dpi,
    })
}

/// A brush-tip image as a coverage plane with a box-filtered mip chain (level `k` is
/// downsampled by `2^k`), so small dabs of a large tip do not alias.
struct Tip {
    levels: Vec<Buf>,
    w: f32,
    h: f32,
}

impl Tip {
    /// Alpha when the image has transparency, else 1 − luma (dark paints).
    fn from_image(img: &Image) -> Result<Tip> {
        if img.is_empty() {
            return Err(Error::new("PX1010", "the --tip image is empty"));
        }
        let has_alpha = img.data.chunks(4).any(|p| p[3] < 0.999);
        let plane: Vec<f32> = img
            .data
            .chunks(4)
            .map(|p| if has_alpha { p[3] } else { 1.0 - color::luma(p[0], p[1], p[2]) }.clamp(0.0, 1.0))
            .collect();
        if plane.iter().all(|&v| v <= 0.0) {
            return Err(Error::new("PX1010", "the --tip image paints nothing (fully transparent, or opaque white)")
                .with_hint("tips paint with their alpha, or with their dark areas when opaque"));
        }
        let mut levels = vec![Buf { w: img.width as usize, h: img.height as usize, c: 1, data: plane }];
        while let Some(l) = levels.last().filter(|l| l.w.max(l.h) > 1) {
            let next = util::downsample(l, 2);
            levels.push(next);
        }
        Ok(Tip { levels, w: img.width as f32, h: img.height as f32 })
    }

    /// The mip level for a dab of radius `r` px (at most 2 tip px per canvas px).
    fn level_for(&self, r: f64) -> u8 {
        let ratio = self.w.max(self.h) as f64 / (2.0 * r).max(1e-9);
        if ratio < 2.0 {
            return 0;
        }
        (ratio.log2().floor() as usize).min(self.levels.len() - 1) as u8
    }

    /// Coverage at normalized tip coordinates (`-1..1` across each side).
    #[inline]
    fn sample(&self, level: u8, nu: f32, nv: f32) -> f32 {
        if nu.abs() > 1.0 || nv.abs() > 1.0 {
            return 0.0;
        }
        let b = &self.levels[level as usize];
        let f = (1u32 << level) as f32;
        let tx = (nu * 0.5 + 0.5) * self.w / f - 0.5;
        let ty = (nv * 0.5 + 0.5) * self.h / f - 0.5;
        let mut o = [0f32; 1];
        util::sample(b, tx, ty, Edge::Zero, &mut o);
        o[0]
    }
}

/// One stamp. `inv_rx`/`inv_ry` map pixel offsets (in the tip's rotated frame) to
/// normalized tip units; the round profile is `1 − smoothstep(lo, hi, |n|)`.
#[derive(Clone, Copy, Debug)]
struct Dab {
    x: f32,
    y: f32,
    cos: f32,
    sin: f32,
    inv_rx: f32,
    inv_ry: f32,
    lo: f32,
    inv_span: f32,
    hi2: f32,
    /// Strength (flow × pressure × small-dab area correction).
    a: f32,
    level: u8,
    /// Canvas bounding box, end-exclusive, clipped to the canvas.
    bx0: i32,
    by0: i32,
    bx1: i32,
    by1: i32,
}

impl Dab {
    fn rect(&self) -> RectI {
        RectI::new(self.bx0, self.by0, (self.bx1 - self.bx0) as u32, (self.by1 - self.by0) as u32)
    }
}

struct Brush {
    tip: Option<Tip>,
}

impl Brush {
    /// Tip value of `d` at the center of canvas pixel (`x`, `y`).
    #[inline]
    fn value(&self, d: &Dab, x: i32, y: i32) -> f32 {
        let dx = x as f32 + 0.5 - d.x;
        let dy = y as f32 + 0.5 - d.y;
        let nu = (dx * d.cos - dy * d.sin) * d.inv_rx;
        let nv = (dx * d.sin + dy * d.cos) * d.inv_ry;
        match &self.tip {
            None => {
                let q = nu * nu + nv * nv;
                if q >= d.hi2 {
                    return 0.0;
                }
                let t = ((q.sqrt() - d.lo) * d.inv_span).clamp(0.0, 1.0);
                1.0 - t * t * (3.0 - 2.0 * t)
            }
            Some(tip) => tip.sample(d.level, nu, nv),
        }
    }
}

/// A dab of `diameter` px at (`x`, `y`) rotated `angle` degrees counter-clockwise, or
/// None when it is empty or entirely off the canvas.
fn make_dab(
    cfg: &Cfg,
    tip: Option<&Tip>,
    x: f64,
    y: f64,
    diameter: f64,
    angle: f64,
    strength: f32,
    canvas: RectI,
) -> Option<Dab> {
    if diameter.is_nan() || diameter <= 1e-6 || strength <= 0.0 || !x.is_finite() || !y.is_finite() {
        return None;
    }
    // sub-pixel dabs: draw one pixel wide at proportionally less strength
    let (mut a, mut dd) = (strength, diameter);
    if dd < 1.0 {
        a *= (dd * dd) as f32;
        dd = 1.0;
    }
    let r = dd / 2.0;
    // u axis = (cos, -sin) on screen (y down), so positive angles turn counter-clockwise
    let (sin, cos) = angle.to_radians().sin_cos();
    let rho = cfg.roundness;
    let (inv_rx, inv_ry, lo, hi, hw, hh, level) = match tip {
        None => {
            let (rx, ry) = (r, (r * rho).max(0.05));
            // one pixel across the narrow axis, in normalized units: the anti-aliasing ramp
            let aa = (1.0 / ry).min(2.0);
            let soft = 1.0 - cfg.hardness;
            let (lo, hi) = if soft >= aa {
                (1.0 - soft, 1.0)
            } else {
                let c = 1.0 - soft / 2.0;
                (c - aa / 2.0, c + aa / 2.0)
            };
            let hw = hi * ((rx * cos).powi(2) + (ry * sin).powi(2)).sqrt();
            let hh = hi * ((rx * sin).powi(2) + (ry * cos).powi(2)).sqrt();
            (1.0 / rx, 1.0 / ry, lo, hi, hw, hh, 0)
        }
        Some(t) => {
            let m = t.w.max(t.h) as f64;
            let hx = (r * t.w as f64 / m).max(0.05);
            let hy = (r * t.h as f64 / m * rho).max(0.05);
            let hw = hx * cos.abs() + hy * sin.abs();
            let hh = hx * sin.abs() + hy * cos.abs();
            (1.0 / hx, 1.0 / hy, 0.0, 1.0, hw, hh, t.level_for(r))
        }
    };
    let clampi = |v: f64, lo: i32, hi: i32| {
        (v.clamp(i32::MIN as f64, i32::MAX as f64) as i64).clamp(lo as i64, hi as i64) as i32
    };
    let bx0 = clampi((x - hw).floor() - 1.0, canvas.x, canvas.right());
    let bx1 = clampi((x + hw).ceil() + 1.0, canvas.x, canvas.right());
    let by0 = clampi((y - hh).floor() - 1.0, canvas.y, canvas.bottom());
    let by1 = clampi((y + hh).ceil() + 1.0, canvas.y, canvas.bottom());
    if bx1 <= bx0 || by1 <= by0 {
        return None;
    }
    Some(Dab {
        x: x as f32,
        y: y as f32,
        cos: cos as f32,
        sin: sin as f32,
        inv_rx: inv_rx as f32,
        inv_ry: inv_ry as f32,
        lo: lo as f32,
        inv_span: (1.0 / (hi - lo)) as f32,
        hi2: (hi * hi) as f32,
        a,
        level,
        bx0,
        by0,
        bx1,
        by1,
    })
}

/// The dabs of one stroke.
struct Stroke {
    dabs: Vec<Dab>,
    /// Union of the dab boxes (canvas-clipped).
    bbox: RectI,
    /// Path length in px.
    length: f64,
}

/// Uniform 0..1 from the op's RNG.
#[inline]
fn unit(rng: &mut impl RngCore) -> f64 {
    (rng.next_u32() >> 8) as f64 * (1.0 / 16_777_216.0)
}

/// Place one spacing step's `--count` dabs at (`x`, `y`).
fn place(
    st: &mut Stroke,
    cfg: &Cfg,
    tip: Option<&Tip>,
    canvas: RectI,
    rng: &mut impl RngCore,
    budget: &mut usize,
    at: Pt,
    dir: Option<(f64, f64)>,
    taper: f64,
) -> Result<()> {
    let psize = if cfg.p_size { at.p } else { 1.0 };
    let pflow = if cfg.p_flow { at.p } else { 1.0 };
    let eff = cfg.base * taper * psize;
    let follow = match (cfg.follow, dir) {
        (true, Some((dx, dy))) => (-dy).atan2(dx).to_degrees(),
        _ => 0.0,
    };
    let normal = dir.map_or((0.0, 1.0), |(dx, dy)| (-dy, dx));
    for _ in 0..cfg.count {
        // always three draws per dab, so one dynamic never shifts another's sequence
        let (u_size, u_angle, u_scatter) = (unit(rng), unit(rng), unit(rng));
        *budget += 1;
        if *budget > MAX_DABS {
            return Err(Error::new("PX1013", format!("the strokes need more than {MAX_DABS} dabs"))
                .with_hint("raise --spacing, lower --count, or split the strokes across several calls"));
        }
        let d = eff * (1.0 - cfg.size_jitter * u_size);
        let off = cfg.scatter * eff * (2.0 * u_scatter - 1.0);
        let angle = cfg.angle + follow + cfg.angle_jitter * (u_angle - 0.5);
        let (x, y) = (at.x + normal.0 * off, at.y + normal.1 * off);
        if let Some(dab) = make_dab(cfg, tip, x, y, d, angle, cfg.flow * pflow as f32, canvas) {
            st.bbox = st.bbox.union(&dab.rect());
            st.dabs.push(dab);
        }
    }
    Ok(())
}

/// Walk a stroke and place its dabs every `spacing × diameter` along the path.
fn plan_stroke(
    pts: &[Pt],
    cfg: &Cfg,
    tip: Option<&Tip>,
    canvas: RectI,
    rng: &mut impl RngCore,
    budget: &mut usize,
) -> Result<Stroke> {
    let n = pts.len();
    let mut cum = vec![0.0f64; n];
    for i in 1..n {
        cum[i] = cum[i - 1] + (pts[i].x - pts[i - 1].x).hypot(pts[i].y - pts[i - 1].y);
    }
    let len = cum[n - 1];
    let mut st = Stroke { dabs: Vec::new(), bbox: RectI::default(), length: len };
    if len <= 1e-9 {
        place(&mut st, cfg, tip, canvas, rng, budget, pts[0], None, 1.0)?;
        return Ok(st);
    }
    // unit direction per segment; zero-length segments borrow a neighbor's
    let mut dirs: Vec<Option<(f64, f64)>> = (0..n - 1)
        .map(|i| {
            let (dx, dy) = (pts[i + 1].x - pts[i].x, pts[i + 1].y - pts[i].y);
            let l = dx.hypot(dy);
            (l > 1e-12).then(|| (dx / l, dy / l))
        })
        .collect();
    let mut last = None;
    for d in dirs.iter_mut() {
        if d.is_some() {
            last = *d;
        } else {
            *d = last;
        }
    }
    let mut next = None;
    for d in dirs.iter_mut().rev() {
        if d.is_some() {
            next = *d;
        } else {
            *d = next;
        }
    }
    let (t0, t1) = cfg.taper.map_or((0.0, 0.0), |(s, e)| (s.to_px(len, cfg.dpi), e.to_px(len, cfg.dpi)));
    let taper = |s: f64| -> f64 {
        let ramp = |t: f64| 0.1 + 0.9 * t.clamp(0.0, 1.0);
        let mut f = 1.0f64;
        if t0 > 0.0 {
            f = f.min(ramp(s / t0));
        }
        if t1 > 0.0 {
            f = f.min(ramp((len - s) / t1));
        }
        f
    };
    let (mut s, mut seg) = (0.0f64, 0usize);
    loop {
        while seg + 2 < n && cum[seg + 1] < s {
            seg += 1;
        }
        let l = cum[seg + 1] - cum[seg];
        let t = if l > 0.0 { ((s - cum[seg]) / l).clamp(0.0, 1.0) } else { 0.0 };
        let (a, b) = (pts[seg], pts[seg + 1]);
        let at = Pt { x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t, p: a.p + (b.p - a.p) * t };
        let f = taper(s);
        place(&mut st, cfg, tip, canvas, rng, budget, at, dirs[seg], f)?;
        let psize = if cfg.p_size { at.p } else { 1.0 };
        s += (cfg.spacing * cfg.base * f * psize).max(MIN_STEP);
        if s > len + 1e-6 {
            break;
        }
    }
    Ok(st)
}

// ---------------------------------------------------------------------------------------
// rasterizing strokes

fn union_opt(a: Option<RectI>, b: Option<RectI>) -> Option<RectI> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.union(&b)),
        (a, b) => a.or(b),
    }
}

/// Rasterize one stroke into a target plane of `ch` floats per pixel, `width` pixels per
/// row, whose pixel (0, 0) sits at canvas (`ox`, `oy`); `clip` (canvas coordinates) must
/// lie inside the plane. `apply(pixel, coverage, x, y)` mixes the stroke's coverage into
/// one pixel and says whether it painted. Returns the painted bounds.
fn paint_plane<F>(
    data: &mut [f32],
    width: u32,
    ch: usize,
    (ox, oy): (i32, i32),
    clip: RectI,
    s: &Stroke,
    brush: &Brush,
    apply: F,
) -> Option<RectI>
where
    F: Fn(&mut [f32], f32, i32, i32) -> bool + Sync,
{
    let area = s.bbox.intersect(&clip);
    let stride = width as usize * ch;
    if area.is_empty() || stride == 0 || s.dabs.is_empty() {
        return None;
    }
    let height = data.len() / stride;
    let ty0 = (area.y - oy) as usize;
    let ty1 = (area.bottom() - oy) as usize;
    let (kb0, kb1) = (ty0 / BAND, ty1.div_ceil(BAND));
    let mut lists: Vec<Vec<u32>> = vec![Vec::new(); kb1 - kb0];
    for (i, d) in s.dabs.iter().enumerate() {
        let (y0, y1) = (d.by0.max(area.y), d.by1.min(area.bottom()));
        if y1 <= y0 || d.bx1.min(area.right()) <= d.bx0.max(area.x) {
            continue;
        }
        for k in (y0 - oy) as usize / BAND..=(y1 - 1 - oy) as usize / BAND {
            lists[k - kb0].push(i as u32);
        }
    }
    let (start, end) = (kb0 * BAND * stride, (kb1 * BAND).min(height) * stride);
    let parts: Vec<Option<RectI>> = data[start..end]
        .par_chunks_mut(BAND * stride)
        .zip(lists.par_iter())
        .enumerate()
        .map(|(j, (rows, list))| {
            if list.is_empty() {
                return None;
            }
            let band_ty0 = (kb0 + j) * BAND;
            let cy0 = (oy + band_ty0 as i32).max(area.y);
            let cy1 = (oy + (band_ty0 + rows.len() / stride) as i32).min(area.bottom());
            let mut cx0 = i32::MAX;
            let mut cx1 = i32::MIN;
            for &i in list {
                let d = &s.dabs[i as usize];
                cx0 = cx0.min(d.bx0.max(area.x));
                cx1 = cx1.max(d.bx1.min(area.right()));
            }
            if cy1 <= cy0 || cx1 <= cx0 {
                return None;
            }
            let bw = (cx1 - cx0) as usize;
            // transmittance Π(1 − a·tip), dabs in placement order
            let mut t = vec![1f32; (cy1 - cy0) as usize * bw];
            for &i in list {
                let d = &s.dabs[i as usize];
                let (x0, x1) = (d.bx0.max(cx0), d.bx1.min(cx1));
                for y in d.by0.max(cy0)..d.by1.min(cy1) {
                    let row = &mut t[(y - cy0) as usize * bw..][..bw];
                    for x in x0..x1 {
                        let v = brush.value(d, x, y);
                        if v > 0.0 {
                            row[(x - cx0) as usize] *= 1.0 - d.a * v;
                        }
                    }
                }
            }
            let mut bb: Option<(i32, i32, i32, i32)> = None;
            for y in cy0..cy1 {
                let trow = &t[(y - cy0) as usize * bw..][..bw];
                let ty = (y - oy) as usize - band_ty0;
                let prow = &mut rows[ty * stride..(ty + 1) * stride];
                for x in cx0..cx1 {
                    let cov = 1.0 - trow[(x - cx0) as usize];
                    if cov <= 0.0 {
                        continue;
                    }
                    let px = (x - ox) as usize * ch;
                    if apply(&mut prow[px..px + ch], cov.min(1.0), x, y) {
                        bb = Some(match bb {
                            None => (x, y, x, y),
                            Some((a, b, c, _)) => (a.min(x), b, c.max(x), y),
                        });
                    }
                }
            }
            bb.map(|(x0, y0, x1, y1)| RectI::new(x0, y0, (x1 - x0 + 1) as u32, (y1 - y0 + 1) as u32))
        })
        .collect();
    parts.into_iter().fold(None, union_opt)
}

/// Paint `value` into a mask plane (canvas coordinates) with `strength` × coverage.
fn paint_mask(
    m: &mut Mask,
    strokes: &[Stroke],
    brush: &Brush,
    canvas: RectI,
    value: f32,
    strength: f32,
    region: Option<&Mask>,
) -> Option<RectI> {
    let clip = RectI::new(0, 0, m.width, m.height).intersect(&canvas);
    let mut bounds = None;
    for s in strokes {
        let b = paint_plane(&mut m.data, m.width, 1, (0, 0), clip, s, brush, |p, cov, x, y| {
            let mut k = cov * strength;
            if let Some(r) = region {
                k *= r.at(x as i64, y as i64);
            }
            if k <= 0.0 {
                return false;
            }
            p[0] += (value - p[0]) * k.min(1.0);
            true
        });
        bounds = union_opt(bounds, b);
    }
    bounds
}

/// Re-frame a raster layer to canvas rectangle `to` (new area transparent).
fn reframe(layer: &mut Layer, to: RectI) -> Result<()> {
    let (lx, ly) = (layer.x, layer.y);
    let img = layer.raster_mut()?;
    let mut out = Image::new(to.w, to.h);
    let ov = RectI::new(lx, ly, img.width, img.height).intersect(&to);
    if !ov.is_empty() {
        let n = ov.w as usize * 4;
        for y in ov.y..ov.bottom() {
            let s = ((y - ly) as usize * img.width as usize + (ov.x - lx) as usize) * 4;
            let d = ((y - to.y) as usize * to.w as usize + (ov.x - to.x) as usize) * 4;
            out.data[d..d + n].copy_from_slice(&img.data[s..s + n]);
        }
    }
    *img = out;
    layer.x = to.x;
    layer.y = to.y;
    Ok(())
}

/// Per-pixel hash in 0..1 for dissolve.
#[inline]
fn hash01(seed: u64, x: i32, y: i32) -> f32 {
    let mut z = seed ^ (((x as u32 as u64) << 32) | y as u32 as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    (z >> 40) as f32 / 16_777_216.0
}

// ---------------------------------------------------------------------------------------
// the command

enum Target {
    Pixels,
    LayerMask,
    Selection,
    Named(String),
}

fn resolve_target(a: &Args) -> Result<Target> {
    let t = a.choice("target");
    let target = match (a.str("into"), t.as_str()) {
        (Some(_), "mask" | "selection") => {
            return Err(Error::new("PX1005", format!("give --into or --target {t}, not both")).with_hint(
                "--into paints a named mask; --target mask paints the layer mask; --target selection the selection",
            ))
        }
        (Some(n), _) => {
            if n.trim().is_empty() || n.starts_with('@') {
                return Err(Error::new("PX1010", format!("--into: '{n}' is not a mask name"))
                    .with_hint("--into takes a library mask name; use --target selection to paint the selection"));
            }
            Target::Named(n)
        }
        (None, "mask") => Target::LayerMask,
        (None, "selection") => Target::Selection,
        _ => Target::Pixels,
    };
    if matches!(target, Target::Selection | Target::Named(_)) && a.has("layer") {
        return Err(Error::new("PX1005", "--layer applies to --target pixels and --target mask only")
            .with_hint("drop --layer when painting the selection or a named mask"));
    }
    Ok(target)
}

fn stroke(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let target = resolve_target(a)?;
    let erase = a.flag("erase");
    let blend = BlendMode::parse(&a.choice("blend"))?;
    if blend != BlendMode::Normal {
        if erase {
            return Err(Error::new("PX1005", "--blend does not apply to --erase"));
        }
        if !matches!(target, Target::Pixels) {
            return Err(Error::new("PX1005", "--blend applies to --target pixels only")
                .with_hint("masks and the selection take the luma of --color"));
        }
    }
    let cfg = brush_cfg(a, doc)?;
    let color = a.color("color")?.to_array();
    let opacity = (a.f64("opacity")? / 100.0) as f32;
    let tip = match a.path("tip") {
        Some(p) => {
            let (img, _) = ctx.decode_aux(&p, "tip", Some("tip"))?;
            Some(Tip::from_image(&img).map_err(|e| e.with_context("tip", p.display().to_string()))?)
        }
        None => None,
    };
    let strokes = gather(a, doc, ctx)?;
    let region = ctx.mask_arg(a, doc)?;
    let canvas = RectI::new(0, 0, doc.width, doc.height);
    let mut rng = ctx.rng();
    let mut budget = 0usize;
    let planned: Vec<Stroke> = strokes
        .iter()
        .map(|s| plan_stroke(s, &cfg, tip.as_ref(), canvas, &mut rng, &mut budget))
        .collect::<Result<_>>()?;
    let dissolve_seed = ctx.rng_lane(1).next_u64();
    let brush = Brush { tip };
    let mut reach = planned.iter().fold(RectI::default(), |r, s| r.union(&s.bbox));
    if let Some(m) = &region {
        reach = m.bbox(0.0).map_or(RectI::default(), |b| reach.intersect(&b));
    }
    let base = json!({
        "strokes": planned.len(),
        "dabs": planned.iter().map(|s| s.dabs.len()).sum::<usize>(),
        "length": (planned.iter().map(|s| s.length).sum::<f64>() * 100.0).round() / 100.0,
        "size": (cfg.base * 100.0).round() / 100.0,
        "erase": erase,
    });
    let region = region.as_ref();
    let mask_value = if erase { 0.0 } else { color::luma(color[0], color[1], color[2]) };
    let mask_strength = opacity * if erase { 1.0 } else { color[3] };
    let (bounds, extra) = match target {
        Target::Pixels => {
            let path = target_layer(a, doc)?;
            let layer = doc.layer(&path);
            let name = layer.name.clone();
            if !layer.visible {
                ctx.warn("PXW2001", format!("layer '{name}' is hidden"));
            }
            if !matches!(layer.content, LayerKind::Raster { .. }) {
                let kind = layer.kind_name();
                return Err(Error::new(
                    "PX2303",
                    format!("layer '{name}' is a {kind} layer; painting pixels needs a raster layer"),
                )
                .with_hint(format!(
                    "pixelox layer rasterize <doc> --layer '{name}' --in-place, or paint its layer mask with --target mask"
                ))
                .with_context("layer", &name)
                .with_context("kind", kind));
            }
            let layer = doc.layer_mut(&path);
            let old = {
                let img = layer.raster()?;
                RectI::new(layer.x, layer.y, img.width, img.height)
            };
            let grow_to = if erase || reach.is_empty() { old } else { old.union(&reach) };
            if grow_to != old {
                reframe(layer, grow_to)?;
            }
            let (ox, oy) = (layer.x, layer.y);
            let img = layer.raster_mut()?;
            let clip = RectI::new(ox, oy, img.width, img.height).intersect(&canvas);
            let (r, g, b, ca) = (color[0], color[1], color[2], color[3]);
            let mut bounds = None;
            for (i, s) in planned.iter().enumerate() {
                let seed = dissolve_seed ^ (i as u64).wrapping_mul(0xD1B5_4A32_D192_ED03);
                let painted = paint_plane(&mut img.data, img.width, 4, (ox, oy), clip, s, &brush, |p, cov, x, y| {
                    let mut k = cov * opacity;
                    if let Some(m) = region {
                        k *= m.at(x as i64, y as i64);
                    }
                    if k <= 0.0 {
                        return false;
                    }
                    if erase {
                        if p[3] <= 0.0 {
                            return false;
                        }
                        p[3] *= 1.0 - k.min(1.0);
                        if p[3] <= 1e-6 {
                            p.fill(0.0);
                        }
                        return true;
                    }
                    let mut alpha = (k * ca).min(1.0);
                    if blend == BlendMode::Dissolve {
                        if hash01(seed, x, y) >= alpha {
                            return false;
                        }
                        alpha = 1.0;
                    }
                    composite_pixel(blend, p, [r, g, b, alpha]);
                    true
                });
                bounds = union_opt(bounds, painted);
            }
            // keep only the growth that received paint
            let mut grown = false;
            if grow_to != old {
                let keep = bounds.map_or(old, |b| old.union(&b.intersect(&grow_to)));
                if keep != grow_to {
                    reframe(layer, keep)?;
                }
                grown = keep != old;
            }
            let img = layer.raster()?;
            let lb = RectI::new(layer.x, layer.y, img.width, img.height);
            (bounds, json!({"target": "pixels", "layer": name, "layer_bounds": lb.to_json(), "grown": grown}))
        }
        Target::LayerMask => {
            let path = target_layer(a, doc)?;
            let (cw, chh) = (doc.width, doc.height);
            let layer = doc.layer_mut(&path);
            let name = layer.name.clone();
            if !layer.visible {
                ctx.warn("PXW2001", format!("layer '{name}' is hidden"));
            }
            let created = layer.mask.is_none();
            let lm = layer.mask.get_or_insert_with(|| LayerMask::new(Mask::filled(cw, chh, 1.0)));
            let bounds = paint_mask(&mut lm.mask, &planned, &brush, canvas, mask_value, mask_strength, region);
            // edited pixels no longer match a vector mask's path
            let dropped = bounds.is_some() && layer.vector_mask.take().is_some();
            (bounds, json!({"target": "mask", "layer": name, "created_mask": created, "vector_mask_dropped": dropped}))
        }
        Target::Selection => {
            let existed = doc.selection.is_some();
            let mut m = doc.selection.take().unwrap_or_else(|| Mask::new(doc.width, doc.height));
            let bounds = paint_mask(&mut m, &planned, &brush, canvas, mask_value, mask_strength, region);
            if existed || bounds.is_some() {
                doc.selection = Some(m);
            }
            (bounds, json!({"target": "selection", "created": !existed && bounds.is_some()}))
        }
        Target::Named(name) => {
            let existed = doc.masks.contains_key(&name);
            let mut m = doc.masks.remove(&name).unwrap_or_else(|| Mask::new(doc.width, doc.height));
            let bounds = paint_mask(&mut m, &planned, &brush, canvas, mask_value, mask_strength, region);
            doc.put_mask(&name, m, true)?;
            (bounds, json!({"target": "named-mask", "mask": name, "created_mask": !existed}))
        }
    };
    if bounds.is_none() {
        ctx.warn("PXW2004", "the strokes painted nothing: they are off the canvas, or --mask hides them");
    }
    let v = util::merge(base, extra);
    Ok(util::merge(v, json!({"bounds": bounds.map_or(Value::Null, |b| b.to_json())})))
}

// ---------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_io;
    use pixelox_core::ctx::{Globals, NoIo, NoProgress};
    use pixelox_core::testing::{self, find, max_diff, raster, run};
    use pixelox_core::Runner;

    fn op() -> OpSpec {
        find(&ops(), "paint stroke")
    }

    fn white(w: u32, h: u32) -> Document {
        testing::doc_from(testing::solid(w, h, Rgba::WHITE))
    }

    fn px(doc: &Document, x: u32, y: u32) -> [f32; 4] {
        raster(doc, "Background").get(x, y)
    }

    fn code(r: Result<Value>) -> String {
        r.map(|_| ()).unwrap_err().code
    }

    fn run_seeded(pairs: &[(&str, &str)], doc: &mut Document, seed: u64) -> Value {
        let spec = op();
        let a = testing::args(&spec, pairs).unwrap();
        let globals = Globals { seed, ..Globals::default() };
        let mut ctx = OpCtx::new(spec.name, vec![], globals, &NoIo, &NoProgress);
        match spec.runner {
            Runner::Doc(f) => f(&a, doc, &mut ctx).unwrap(),
            _ => unreachable!(),
        }
    }

    #[test]
    fn soft_and_hard_profiles() {
        // hard: solid inside, a half-covered pixel exactly on the rim, nothing outside
        let mut d = white(100, 100);
        let v = run(&op(), &[("points", "50.5,50.5"), ("size", "40"), ("hardness", "100")], &mut d).unwrap();
        assert_eq!(v["dabs"], 1);
        assert_eq!(px(&d, 50, 50)[0], 0.0);
        assert_eq!(px(&d, 65, 50)[0], 0.0);
        assert!((px(&d, 70, 50)[0] - 0.5).abs() < 0.02, "rim pixel {}", px(&d, 70, 50)[0]);
        assert_eq!(px(&d, 72, 50)[0], 1.0);
        assert_eq!(v["bounds"], json!([30, 30, 41, 41]));
        // soft: full at the center, half at half the radius, lighter than hard inside
        let mut s = white(100, 100);
        run(&op(), &[("points", "50.5,50.5"), ("size", "40")], &mut s).unwrap();
        assert!(px(&s, 50, 50)[0] < 1e-6);
        assert!((px(&s, 60, 50)[0] - 0.5).abs() < 1e-3, "half radius {}", px(&s, 60, 50)[0]);
        assert!((px(&s, 55, 50)[0] - 0.15625).abs() < 1e-3);
        assert!(px(&s, 68, 50)[0] > 0.9 && px(&s, 68, 50)[0] < 1.0);
        assert_eq!(px(&s, 71, 50)[0], 1.0);
        // middle hardness: solid core out to the hardness fraction
        let mut m = white(100, 100);
        run(&op(), &[("points", "50.5,50.5"), ("size", "40"), ("hardness", "50")], &mut m).unwrap();
        assert!(px(&m, 59, 50)[0] < 1e-6 && px(&m, 65, 50)[0] > 0.1 && px(&m, 65, 50)[0] < 0.9);
    }

    #[test]
    fn flow_builds_up_and_opacity_caps_each_stroke() {
        let line = [("points", "10,20 110,20"), ("size", "20"), ("hardness", "100"), ("color", "black")];
        // one dab at 10% flow
        let mut d = white(120, 40);
        run(&op(), &[("points", "60,20"), ("size", "20"), ("hardness", "100"), ("flow", "10")], &mut d).unwrap();
        assert!((px(&d, 60, 20)[0] - 0.9).abs() < 1e-5);
        // dense low-flow stroke builds up to the opacity and never past it
        let mut d = white(120, 40);
        let mut p = line.to_vec();
        p.extend([("flow", "10"), ("opacity", "50"), ("spacing", "1")]);
        run(&op(), &p, &mut d).unwrap();
        assert!((px(&d, 60, 20)[0] - 0.5).abs() < 0.005, "{}", px(&d, 60, 20)[0]);
        let darkest = raster(&d, "Background").data.chunks(4).map(|q| q[0]).fold(1.0, f32::min);
        assert!(darkest >= 0.5 - 1e-6, "build-up exceeded opacity: {darkest}");
        // flow 10 builds up less with wider spacing
        let mut d2 = white(120, 40);
        let mut p2 = line.to_vec();
        p2.extend([("flow", "10"), ("spacing", "25")]);
        run(&op(), &p2, &mut d2).unwrap();
        let c = px(&d2, 60, 20)[0];
        assert!(c > 0.5 && c < 0.75, "4 overlapping dabs at 10%: {c}");
        // two strokes in one call are capped one by one: 50% then 50% of the rest
        let mut d = white(120, 40);
        let mut p = line.to_vec();
        p[0] = ("points", "10,20 110,20; 10,20 110,20");
        p.extend([("opacity", "50")]);
        let v = run(&op(), &p, &mut d).unwrap();
        assert_eq!(v["strokes"], 2);
        assert!((px(&d, 60, 20)[0] - 0.25).abs() < 1e-4, "{}", px(&d, 60, 20)[0]);
        // opacity 0 paints nothing
        let mut d = white(120, 40);
        let v = run(&op(), &[("points", "60,20"), ("opacity", "0")], &mut d).unwrap();
        assert_eq!(v["bounds"], Value::Null);
        assert_eq!(max_diff(raster(&d, "Background"), &testing::solid(120, 40, Rgba::WHITE)), 0.0);
    }

    #[test]
    fn spacing_sets_the_dab_count() {
        let dabs = |extra: &[(&str, &str)]| {
            let mut d = white(120, 100);
            let mut p = vec![("points", "10,50 110,50"), ("size", "20")];
            p.extend_from_slice(extra);
            run(&op(), &p, &mut d).unwrap()["dabs"].as_u64().unwrap()
        };
        assert_eq!(dabs(&[]), 51, "every 2 px over 100 px");
        assert_eq!(dabs(&[("spacing", "100")]), 6);
        assert_eq!(dabs(&[("count", "3")]), 153);
        assert_eq!(dabs(&[("spacing", "50"), ("size", "40")]), 6);
        // a click is one dab (or --count dabs)
        let mut d = white(50, 50);
        assert_eq!(run(&op(), &[("points", "25,25")], &mut d).unwrap()["dabs"], 1);
        assert_eq!(run(&op(), &[("points", "25,25"), ("count", "4")], &mut d).unwrap()["dabs"], 4);
        // strokes off the canvas place nothing
        let mut d = white(50, 50);
        let v = run(&op(), &[("d", "M -300 20 L -100 20"), ("size", "20")], &mut d).unwrap();
        assert_eq!((v["dabs"].as_u64(), &v["bounds"]), (Some(0), &Value::Null));
        assert_eq!(v["length"], 200.0);
    }

    #[test]
    fn mask_target_adds_and_paints_the_layer_mask() {
        let red = testing::solid(60, 60, Rgba::new(1.0, 0.0, 0.0, 1.0));
        let mut d = testing::doc_from(red.clone());
        let base = [("target", "mask"), ("points", "30.5,30.5"), ("size", "20"), ("hardness", "100")];
        let v = run(&op(), &base, &mut d).unwrap();
        assert_eq!((v["target"].as_str(), v["created_mask"].as_bool()), (Some("mask"), Some(true)));
        let m = &d.layers[0].mask.as_ref().unwrap().mask;
        assert_eq!((m.get(30, 30), m.get(5, 5)), (0.0, 1.0), "black hides, the rest stays revealed");
        assert_eq!(max_diff(raster(&d, "Background"), &red), 0.0, "pixels untouched");
        // white reveals again; a second call reuses the mask
        let mut p = base.to_vec();
        p.push(("color", "white"));
        let v = run(&op(), &p, &mut d).unwrap();
        assert_eq!(v["created_mask"], false);
        assert_eq!(d.layers[0].mask.as_ref().unwrap().mask.get(30, 30), 1.0);
        // erase paints 0 whatever the color; gray paints its luma
        let mut p = base.to_vec();
        p.extend([("erase", "true"), ("color", "white")]);
        run(&op(), &p, &mut d).unwrap();
        assert_eq!(d.layers[0].mask.as_ref().unwrap().mask.get(30, 30), 0.0);
        let mut p = base.to_vec();
        p.push(("color", "#808080"));
        run(&op(), &p, &mut d).unwrap();
        let g = d.layers[0].mask.as_ref().unwrap().mask.get(30, 30);
        assert!((g - color::luma(0.502, 0.502, 0.502)).abs() < 0.01, "{g}");
        // works on non-raster layers, which cannot take pixel paint; edits drop a vector mask
        let mut d = white(60, 60);
        let shape = d.new_layer("Box", LayerKind::Shape { spec: json!({"d": "M 0 0 H 30 V 30 Z", "fill": "#00f"}) });
        d.insert_above(None, shape);
        d.layers[1].vector_mask = Some("M 0 0 H 60 V 60 H 0 Z".into());
        d.layers[1].mask = Some(LayerMask::new(Mask::filled(60, 60, 1.0)));
        let v = run(&op(), &[("layer", "Box"), ("target", "mask"), ("points", "10,10")], &mut d).unwrap();
        assert_eq!(v["vector_mask_dropped"], true);
        assert!(d.layers[1].vector_mask.is_none());
        let e = run(&op(), &[("layer", "Box"), ("points", "10,10")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX2303");
        assert!(e.hint.unwrap().contains("--target mask"));
    }

    #[test]
    fn selection_and_named_mask_targets() {
        let mut d = white(60, 60);
        let p =
            [("target", "selection"), ("points", "30.5,30.5"), ("size", "20"), ("hardness", "100"), ("color", "white")];
        let v = run(&op(), &p, &mut d).unwrap();
        assert_eq!(v["created"], true);
        let sel = d.selection.as_ref().unwrap();
        assert_eq!((sel.get(30, 30), sel.get(5, 5)), (1.0, 0.0));
        // painting off-canvas into a missing selection leaves no selection behind
        let mut d2 = white(60, 60);
        run(&op(), &[("target", "selection"), ("d", "M -90 10 L -80 10"), ("color", "white")], &mut d2).unwrap();
        assert!(d2.selection.is_none());
        // --into creates a black named mask, then keeps painting it
        let mut d = white(60, 60);
        let v = run(
            &op(),
            &[("into", "hair"), ("points", "10.5,10.5"), ("size", "8"), ("hardness", "100"), ("color", "white")],
            &mut d,
        )
        .unwrap();
        assert_eq!(
            (v["target"].as_str(), v["mask"].as_str(), v["created_mask"].as_bool()),
            (Some("named-mask"), Some("hair"), Some(true))
        );
        let v = run(
            &op(),
            &[("into", "hair"), ("points", "40.5,40.5"), ("size", "8"), ("hardness", "100"), ("color", "white")],
            &mut d,
        )
        .unwrap();
        assert_eq!(v["created_mask"], false);
        let m = &d.masks["hair"];
        assert_eq!((m.get(10, 10), m.get(40, 40), m.get(25, 25)), (1.0, 1.0, 0.0));
        assert!(d.selection.is_none());
        // conflicts and bad names
        assert_eq!(code(run(&op(), &[("into", "x"), ("target", "mask"), ("points", "1,1")], &mut d)), "PX1005");
        assert_eq!(code(run(&op(), &[("into", "@sel"), ("points", "1,1")], &mut d)), "PX1010");
        assert_eq!(
            code(run(&op(), &[("target", "selection"), ("layer", "Background"), ("points", "1,1")], &mut d)),
            "PX1005"
        );
        assert_eq!(code(run(&op(), &[("target", "mask"), ("blend", "multiply"), ("points", "1,1")], &mut d)), "PX1005");
        assert_eq!(code(run(&op(), &[("erase", "true"), ("blend", "screen"), ("points", "1,1")], &mut d)), "PX1005");
    }

    #[test]
    fn erase_lowers_alpha_without_growing() {
        let mut d = white(60, 60);
        run(&op(), &[("points", "30.5,30.5"), ("size", "20"), ("hardness", "100"), ("erase", "true")], &mut d).unwrap();
        assert_eq!(px(&d, 30, 30), [0.0; 4]);
        assert_eq!(px(&d, 5, 5), [1.0; 4]);
        let mut d = white(60, 60);
        run(
            &op(),
            &[("points", "30.5,30.5"), ("size", "20"), ("hardness", "100"), ("erase", "true"), ("flow", "50")],
            &mut d,
        )
        .unwrap();
        assert_eq!(px(&d, 30, 30), [1.0, 1.0, 1.0, 0.5]);
        // erasing next to a small layer does not grow it
        let mut d = testing::doc_layers(60, 60, vec![("Small", testing::solid(10, 10, Rgba::WHITE))]);
        let v = run(&op(), &[("points", "8,8 50,50"), ("size", "10"), ("erase", "true")], &mut d).unwrap();
        assert_eq!(v["grown"], false);
        assert_eq!((raster(&d, "Small").width, raster(&d, "Small").height), (10, 10));
        assert!(raster(&d, "Small").get(8, 8)[3] < 0.5);
    }

    #[test]
    fn tip_images_sample_alpha_or_inverse_luma() {
        // transparency present: alpha is the coverage
        let mut img = Image::filled(8, 4, Rgba::new(1.0, 1.0, 1.0, 1.0));
        img.set(0, 0, [1.0, 1.0, 1.0, 0.0]);
        let t = Tip::from_image(&img).unwrap();
        assert_eq!(t.levels[0].data[0], 0.0);
        assert_eq!(t.levels[0].data[1], 1.0);
        // opaque: dark paints (1 - luma)
        let img = Image::from_fn(4, 4, |x, _| if x < 2 { [0.0, 0.0, 0.0, 1.0] } else { [1.0, 1.0, 1.0, 1.0] });
        let t = Tip::from_image(&img).unwrap();
        assert_eq!(&t.levels[0].data[..4], &[1.0, 1.0, 0.0, 0.0]);
        assert!((t.sample(0, -0.5, 0.0) - 1.0).abs() < 1e-6 && t.sample(0, 0.5, 0.0) < 1e-6);
        assert_eq!(t.sample(0, 1.2, 0.0), 0.0);
        assert_eq!(t.levels.len(), 3, "4x4 -> 2x2 -> 1x1");
        assert_eq!(t.level_for(2.0), 0);
        let big = Tip::from_image(&testing::solid(64, 64, Rgba::BLACK)).unwrap();
        assert_eq!(big.level_for(4.0), 3, "64 px tip on an 8 px dab: 8x smaller");
        assert!((big.sample(3, 0.0, 0.0) - 1.0).abs() < 1e-6);
        // tips that paint nothing are refused
        assert_eq!(Tip::from_image(&testing::solid(4, 4, Rgba::WHITE)).err().unwrap().code, "PX1010");
        assert_eq!(Tip::from_image(&Image::new(4, 4)).err().unwrap().code, "PX1010");
    }

    #[test]
    fn tip_files_shape_the_dab_and_rotate() {
        let td = testing::TempDir::new("paint-tip");
        let bar = Image::from_fn(20, 4, |_, _| [0.0, 0.0, 0.0, 1.0]);
        let f = td.path("bar.pxt");
        std::fs::write(&f, test_io::encode(&bar)).unwrap();
        let tip = f.to_str().unwrap();
        let mut d = white(100, 100);
        let v = test_io::run_io(&op(), &[("tip", tip), ("points", "50,50"), ("size", "40")], &mut d).unwrap();
        assert_eq!(v["dabs"], 1);
        // the 20x4 bar becomes 40x8: wide along x, thin along y
        assert!(px(&d, 65, 50)[0] < 0.01 && px(&d, 34, 50)[0] < 0.01);
        assert_eq!(px(&d, 50, 60)[0], 1.0);
        let mut d = white(100, 100);
        test_io::run_io(&op(), &[("tip", tip), ("points", "50,50"), ("size", "40"), ("angle", "90")], &mut d).unwrap();
        assert!(px(&d, 50, 65)[0] < 0.01);
        assert_eq!(px(&d, 65, 50)[0], 1.0);
        // --follow turns the bar along a vertical stroke
        let mut d = white(100, 100);
        test_io::run_io(&op(), &[("tip", tip), ("points", "50,20 50,80"), ("size", "40"), ("follow", "true")], &mut d)
            .unwrap();
        assert_eq!(px(&d, 65, 50)[0], 1.0);
        assert!(px(&d, 50, 50)[0] < 0.01);
    }

    #[test]
    fn roundness_angle_and_follow_on_the_round_tip() {
        let p = [("points", "50.5,50.5"), ("size", "40"), ("hardness", "100"), ("roundness", "20")];
        let mut d = white(100, 100);
        run(&op(), &p, &mut d).unwrap();
        assert_eq!((px(&d, 66, 50)[0], px(&d, 50, 56)[0]), (0.0, 1.0), "40 x 8 ellipse");
        let mut q = p.to_vec();
        q.push(("angle", "90"));
        let mut d = white(100, 100);
        run(&op(), &q, &mut d).unwrap();
        assert_eq!((px(&d, 66, 50)[0], px(&d, 50, 66)[0]), (1.0, 0.0));
        // --follow: a thin tip along a vertical stroke makes a thin vertical line
        let mut d = white(100, 100);
        run(
            &op(),
            &[
                ("points", "50.5,20 50.5,80"),
                ("size", "40"),
                ("hardness", "100"),
                ("roundness", "10"),
                ("follow", "true"),
            ],
            &mut d,
        )
        .unwrap();
        assert_eq!((px(&d, 50, 50)[0], px(&d, 56, 50)[0]), (0.0, 1.0));
    }

    #[test]
    fn same_seed_same_strokes_new_seed_new_jitter() {
        let p = [
            ("points", "10,40 150,60"),
            ("size", "16"),
            ("size-jitter", "60"),
            ("angle-jitter", "360"),
            ("scatter", "150"),
            ("count", "3"),
            ("roundness", "40"),
            ("flow", "40"),
        ];
        let paint = |seed: u64| {
            let mut d = white(160, 100);
            let v = run_seeded(&p, &mut d, seed);
            (raster(&d, "Background").clone(), v)
        };
        let (a, va) = paint(7);
        let (b, vb) = paint(7);
        let (c, _) = paint(8);
        assert_eq!(a.data, b.data);
        assert_eq!(va, vb);
        assert!(max_diff(&a, &c) > 0.1, "a different seed should move the dabs");
        // without dynamics the seed does not matter
        let still = |seed: u64| {
            let mut d = white(160, 100);
            run_seeded(&[("points", "10,40 150,60"), ("size", "16")], &mut d, seed);
            raster(&d, "Background").clone()
        };
        assert_eq!(still(1).data, still(2).data);
        // dissolve speckles deterministically
        let speck = |seed: u64| {
            let mut d = white(160, 100);
            run_seeded(
                &[("points", "10,40 150,60"), ("size", "30"), ("blend", "dissolve"), ("opacity", "50")],
                &mut d,
                seed,
            );
            raster(&d, "Background").clone()
        };
        let s = speck(3);
        assert_eq!(s.data, speck(3).data);
        assert!(s.data.chunks(4).any(|q| q[0] == 0.0) && s.data.chunks(4).filter(|q| q[0] == 1.0).count() > 10_000);
    }

    #[test]
    fn region_masks_limit_paint() {
        let mut d = white(40, 20);
        d.put_mask("left", testing::half_mask(40, 20), false).unwrap();
        let p = [("points", "0,10 40,10"), ("size", "30"), ("hardness", "100"), ("mask", "left")];
        let v = run(&op(), &p, &mut d).unwrap();
        assert_eq!((px(&d, 5, 10)[0], px(&d, 30, 10)[0]), (0.0, 1.0));
        assert_eq!(v["bounds"], json!([0, 0, 20, 20]));
        let mut d = white(40, 20);
        d.put_mask("left", testing::half_mask(40, 20), false).unwrap();
        let mut q = p.to_vec();
        q.push(("invert-mask", "true"));
        run(&op(), &q, &mut d).unwrap();
        assert_eq!((px(&d, 5, 10)[0], px(&d, 30, 10)[0]), (1.0, 0.0));
        // also on mask targets
        let mut d = white(40, 20);
        d.put_mask("left", testing::half_mask(40, 20), false).unwrap();
        let mut q = p.to_vec();
        q.extend([("target", "selection"), ("color", "white")]);
        run(&op(), &q, &mut d).unwrap();
        let s = d.selection.as_ref().unwrap();
        assert_eq!((s.get(5, 10), s.get(30, 10)), (1.0, 0.0));
    }

    #[test]
    fn locked_and_wrong_inputs() {
        let mut d = white(30, 30);
        d.layers[0].locked = true;
        assert_eq!(code(run(&op(), &[("points", "10,10")], &mut d)), "PX2304");
        assert_eq!(code(run(&op(), &[("points", "10,10"), ("target", "mask")], &mut d)), "PX2304");
        let mut d = white(30, 30);
        assert_eq!(code(run(&op(), &[], &mut d)), "PX1003");
        assert_eq!(code(run(&op(), &[("points", "10;20")], &mut d)), "PX1011");
        assert_eq!(code(run(&op(), &[("points", "10,10,1.5")], &mut d)), "PX1013");
        assert_eq!(code(run(&op(), &[("d", "garbage")], &mut d)), "PX1010");
        assert_eq!(code(run(&op(), &[("points", "1,1"), ("size", "0")], &mut d)), "PX1013");
        assert_eq!(code(run(&op(), &[("points", "1,1"), ("taper", "a,b")], &mut d)), "PX1010");
        assert_eq!(code(run(&op(), &[("points", "1,1"), ("taper", "-3")], &mut d)), "PX1013");
        assert_eq!(code(run(&op(), &[("points", "1,1"), ("flow", "0")], &mut d)), "PX1013");
        // the tip is read through the IO layer (none in plain test runs)
        assert!(run(&op(), &[("points", "1,1"), ("tip", "missing.png")], &mut d).is_err());
    }

    #[test]
    fn painting_beyond_a_layer_grows_it_within_the_canvas() {
        let mut d =
            testing::doc_layers(100, 100, vec![("Paint", testing::solid(10, 10, Rgba::new(0.0, 0.0, 1.0, 1.0)))]);
        let v = run(&op(), &[("points", "60.5,60.5"), ("size", "20"), ("hardness", "100")], &mut d).unwrap();
        assert_eq!(v["grown"], true);
        assert_eq!(v["layer_bounds"], json!([0, 0, 71, 71]), "grown to the painted pixels");
        let l = raster(&d, "Paint");
        assert_eq!(l.get(60, 60), [0.0, 0.0, 0.0, 1.0]);
        assert_eq!(l.get(5, 5), [0.0, 0.0, 1.0, 1.0], "old pixels stay put");
        assert_eq!(l.get(30, 5)[3], 0.0);
        // up and left of a placed layer, clamped to the canvas
        let mut d =
            testing::doc_layers(100, 100, vec![("Paint", testing::solid(10, 10, Rgba::new(0.0, 0.0, 1.0, 1.0)))]);
        d.layers[0].x = 40;
        d.layers[0].y = 40;
        let v = run(&op(), &[("points", "2,2"), ("size", "30"), ("hardness", "100")], &mut d).unwrap();
        assert_eq!(v["layer_bounds"], json!([0, 0, 50, 50]));
        assert_eq!((d.layers[0].x, d.layers[0].y), (0, 0));
        assert_eq!(raster(&d, "Paint").get(45, 45), [0.0, 0.0, 1.0, 1.0]);
        assert_eq!(raster(&d, "Paint").get(1, 1)[3], 1.0);
        // nothing painted: no growth
        let mut d = testing::doc_layers(100, 100, vec![("Paint", testing::solid(10, 10, Rgba::WHITE))]);
        let v = run(&op(), &[("points", "60,60"), ("opacity", "0")], &mut d).unwrap();
        assert_eq!((v["grown"].as_bool(), &v["layer_bounds"]), (Some(false), &json!([0, 0, 10, 10])));
        // an empty layer grows to exactly the stroke
        let mut d = testing::doc_layers(50, 50, vec![("Empty", Image::new(0, 0))]);
        let v = run(&op(), &[("points", "25.5,25.5"), ("size", "10"), ("hardness", "100")], &mut d).unwrap();
        assert_eq!(v["layer_bounds"], v["bounds"]);
    }

    #[test]
    fn blend_modes_and_color_alpha() {
        let gray = testing::solid(40, 40, Rgba::new(0.5, 0.5, 0.5, 1.0));
        let mut d = testing::doc_from(gray);
        run(
            &op(),
            &[
                ("points", "20.5,20.5"),
                ("size", "10"),
                ("hardness", "100"),
                ("blend", "multiply"),
                ("color", "#ff0000"),
            ],
            &mut d,
        )
        .unwrap();
        let p = px(&d, 20, 20);
        assert!((p[0] - 0.5).abs() < 1e-5 && p[1] < 1e-5, "{p:?}");
        // a translucent color halves the paint
        let mut d = white(40, 40);
        run(&op(), &[("points", "20.5,20.5"), ("size", "10"), ("hardness", "100"), ("color", "#00000080")], &mut d)
            .unwrap();
        assert!((px(&d, 20, 20)[0] - 0.498).abs() < 0.01);
        // painting onto transparency builds the color, not gray fringes
        let mut d = testing::doc_from(Image::new(40, 40));
        run(&op(), &[("points", "20.5,20.5"), ("size", "20"), ("color", "#ff0000")], &mut d).unwrap();
        let e = px(&d, 27, 20);
        assert!(e[3] > 0.0 && e[3] < 1.0 && e[0] == 1.0, "{e:?}");
        assert!(!PAINT_BLENDS.contains(&"pass-through") && PAINT_BLENDS.len() == BLEND_MODES.len() - 1);
    }

    #[test]
    fn points_files_and_svg_paths() {
        let td = testing::TempDir::new("paint-points");
        let f = td.path("p.json");
        let fs = f.to_str().unwrap();
        let strokes = |json: &str, doc: &mut Document| -> Result<Value> {
            std::fs::write(&f, json).unwrap();
            run(&op(), &[("points-file", fs), ("size", "6")], doc)
        };
        let mut d = white(100, 100);
        assert_eq!(strokes("[[10,10],[50,10,0.5]]", &mut d).unwrap()["strokes"], 1);
        assert_eq!(strokes(r#"{"points": [{"x":10,"y":10,"p":1},{"x":50,"y":50}]}"#, &mut d).unwrap()["strokes"], 1);
        assert_eq!(strokes("[[[10,10],[20,20]], [[30,30],[40,40,0.2]], [[60,60]]]", &mut d).unwrap()["strokes"], 3);
        assert_eq!(strokes(r#"{"points": [[[10,10],[20,20]], [[30,30],[40,40]]]}"#, &mut d).unwrap()["strokes"], 2);
        let v = strokes(r#"{"d": "M 10 10 L 50 10 M 10 30 L 50 30"}"#, &mut d).unwrap();
        assert_eq!((v["strokes"].as_u64(), v["length"].as_f64()), (Some(2), Some(80.0)));
        assert_eq!(code(strokes(r#"{"nope": 1}"#, &mut d)), "PX1010");
        assert_eq!(code(strokes("[]", &mut d)), "PX1010");
        assert_eq!(code(strokes("[[1,2],[3]]", &mut d)), "PX1010");
        assert_eq!(code(strokes("[[1,2,5]]", &mut d)), "PX1013");
        assert_eq!(code(strokes("not json", &mut d)), "PX1010");
        // SVG paths: subpaths are strokes, curves are followed, Z closes the loop
        let v = run(&op(), &[("d", "M 10 50 C 30 0 70 100 90 50"), ("size", "4")], &mut d).unwrap();
        assert_eq!(v["strokes"], 1);
        assert!(v["length"].as_f64().unwrap() > 85.0);
        let v = run(&op(), &[("d", "M 10 10 H 40 V 40 H 10 Z M 60 60 L 90 90"), ("size", "4")], &mut d).unwrap();
        assert_eq!((v["strokes"].as_u64(), v["length"].as_f64()), (Some(2), Some(162.43)));
        // --points: units, ';' strokes, and every source together
        let mut d = white(100, 100);
        let v = run(&op(), &[("points", "10%,10% 50%,10%; -10,-10"), ("d", "M 0 50 L 10 50"), ("size", "4")], &mut d)
            .unwrap();
        assert_eq!((v["strokes"].as_u64(), v["length"].as_f64()), (Some(3), Some(50.0)));
        assert!(px(&d, 89, 89)[0] < 0.5, "negative = from the far edge");
    }

    #[test]
    fn pressure_and_taper_shape_the_stroke() {
        let hard = [("size", "20"), ("hardness", "100")];
        let paint = |extra: &[(&str, &str)]| {
            let mut d = white(120, 100);
            let mut p = hard.to_vec();
            p.extend_from_slice(extra);
            run(&op(), &p, &mut d).unwrap();
            d
        };
        // pressure 1 -> 0.1 along the stroke thins it (size), or fades it (flow)
        let d = paint(&[("points", "10,50,1 110,50,0.1")]);
        assert_eq!((px(&d, 20, 57)[0], px(&d, 100, 57)[0]), (0.0, 1.0));
        let d = paint(&[("points", "10,50,1 110,50,0.1"), ("pressure", "none")]);
        assert_eq!((px(&d, 20, 57)[0], px(&d, 100, 57)[0]), (0.0, 0.0));
        let d = paint(&[("points", "10,50,1 110,50,0.1"), ("pressure", "flow"), ("spacing", "100")]);
        assert_eq!(px(&d, 10, 57)[0], 0.0);
        assert!(px(&d, 110, 57)[0] > 0.8, "{}", px(&d, 110, 57)[0]);
        // taper: thin ends, full body
        let d = paint(&[("points", "10,50 110,50"), ("taper", "30,30")]);
        assert_eq!((px(&d, 12, 57)[0], px(&d, 60, 57)[0], px(&d, 108, 57)[0]), (1.0, 0.0, 1.0));
        let d = paint(&[("points", "10,50 110,50"), ("taper", "30%,0")]);
        assert_eq!((px(&d, 12, 57)[0], px(&d, 108, 57)[0]), (1.0, 0.0));
    }

    #[test]
    fn size_accepts_percent_of_the_shorter_side() {
        let mut d = white(200, 100);
        let v = run(&op(), &[("points", "100,50"), ("size", "20%")], &mut d).unwrap();
        assert_eq!(v["size"], 20.0);
    }
}
