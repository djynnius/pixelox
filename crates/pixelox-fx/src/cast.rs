//! Cast styles: `fx long-shadow`, `fx extrude`, and `fx reflection`; and the rule that
//! content cut off by the canvas edge continues beyond it.
//!
//! Long shadows and extrusions are sweeps of the layer's alpha along a direction: the
//! coverage at p is the strongest alpha found within `length` behind p, `max_t a(p − t·d)`.
//! The sweep runs along lines parallel to d through the buffer, one sample per column with
//! linear interpolation across the rows; a monotonic deque gives each line's sliding maximum
//! in O(width), so a 2,000 px shadow costs the same as a 20 px one. Lines are spaced a quarter
//! pixel apart and each pixel averages the two a quarter pixel either side of its center, so
//! the sides of the sweep are anti-aliased at any angle (45° too), while the pixel's own
//! alpha is the near end, so the shape's edges get no fringe. Extrusions also carry the color
//! of the nearest face pixel along each line, for `--color auto`. A long shadow's fade is one
//! linear gradient along the direction, so gaps between letters do not streak it.
//!
//! A reflection mirrors the styled layer (its face, extrusion, and strokes; not its shadows
//! or glows) about the bottom of its content, offset by a gap, fading out over a length, with
//! a blur that can grow with the distance from the floor line.

use crate::commands::{look, set_style};
use crate::render::{Look, Params};
use pixelox_core::kernels::Edge;
use pixelox_core::prelude::*;
use pixelox_core::units::Length;
use rayon::prelude::*;
use std::collections::VecDeque;

/// Quarter-pixel line offsets (see the module docs).
const PHASES: usize = 4;

/// `fx long-shadow`: a flat shadow dragged along `angle` for `length` px.
#[derive(Clone, Debug)]
pub(crate) struct LongShadow {
    pub color: [f32; 4],
    pub angle: f64,
    pub length: f64,
    /// 0..1: how much of the opacity is gone at the far end.
    pub fade: f64,
}

/// `fx extrude`: the shape's sides receding `depth` px along `angle`.
#[derive(Clone, Debug)]
pub(crate) struct Extrude {
    /// Side color, or None for the face's own colors darkened by `darken`.
    pub color: Option<[f32; 4]>,
    pub darken: f32,
    /// 0..1: extra darkening at the far end of the sides.
    pub shade: f32,
    pub angle: f64,
    pub depth: f64,
    /// Width (px) and color of the stroke around the whole block (face and sides).
    pub outline: f64,
    pub outline_color: [f32; 4],
}

/// `fx reflection`: a mirror image on the floor under the layer.
#[derive(Clone, Debug)]
pub(crate) struct Reflection {
    pub gap: f64,
    /// Reflected height in px.
    pub length: f64,
    /// 0..1: the share of the length over which it fades out (1 = all of it).
    pub fade: f64,
    /// Blur radius (px) at the far end.
    pub blur: f64,
    /// 0..1: how much the blur grows with distance (0 = even, 1 = sharp at the floor line).
    pub distance: f64,
}

#[derive(Clone, Debug)]
pub(crate) enum Cast {
    LongShadow(LongShadow),
    Extrude(Extrude),
    Reflection(Reflection),
}

/// Unit direction in buffer coordinates (y down) for an fx angle (0 = right, 90 = up).
pub(crate) fn dir(angle: f64) -> (f64, f64) {
    let t = angle.to_radians();
    (t.cos(), -t.sin())
}

/// How far a sweep of `len` px along `angle` reaches on each side: left, top, right, bottom.
fn sweep_margins(angle: f64, len: f64) -> [f64; 4] {
    let (dx, dy) = dir(angle);
    [(-dx * len).max(0.0), (-dy * len).max(0.0), (dx * len).max(0.0), (dy * len).max(0.0)]
}

impl Cast {
    /// Reach beyond the content on each side (left, top, right, bottom). Long shadows are
    /// limited by `room`, the distance from the content to the canvas edges (nothing past the
    /// canvas is visible, and a long shadow is usually meant to leave it).
    pub(crate) fn margins(&self, room: [f64; 4]) -> [f64; 4] {
        match self {
            Cast::LongShadow(s) => {
                let m = sweep_margins(s.angle, s.length);
                [0, 1, 2, 3].map(|k| m[k].min(room[k]))
            }
            Cast::Extrude(e) => sweep_margins(e.angle, e.depth).map(|v| v + e.outline + 1.0),
            Cast::Reflection(r) => {
                let spread = 1.5 * r.blur;
                [spread, 0.0, spread, r.gap.max(0.0) + r.length + spread]
            }
        }
    }
}

// ---------------------------------------------------------------------------------------
// commands

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("fx long-shadow", Category::Fx, "Add or replace a long (flat) shadow", long_shadow_op)
            .about(
                "A flat shadow dragged from the layer's shape along --angle (0 = right, 90 = up, 315 = down-right) for \
                 --length, as in flat design: every point of the shape casts a streak, so the shadow keeps the shape's \
                 outline at its near end and runs off the canvas at the default length. --fade fades it toward the far \
                 end. It is live (re-renders when the layer changes or moves), works on text, shape, pixel, and placed \
                 layers, is never cut at the layer's bounds, and stops at the canvas edge. With an fx extrude on the \
                 same layer, the shadow is cast by the whole block.",
            )
            .param(common::layer())
            .param(param("angle", ParamKind::Angle).default("315").help("Direction the shadow falls in degrees (0 = right, 90 = up, 315 = down-right)"))
            .param(param("length", ParamKind::Length).default("100%").help("Shadow length: px, or % of the canvas diagonal (100% always reaches the edge)"))
            .param(param("color", ParamKind::Color).default("#000000").help("Shadow color; its alpha multiplies --opacity"))
            .param(param("fade", ParamKind::Float).range("0..100").default("0").help("Percent of the opacity lost by the far end (100 = fades to nothing)"))
            .params(look("multiply", "35", "Blend mode of the shadow"))
            .example("pixelox fx long-shadow d.pxo --in-place --layer Title", "A 45-degree long shadow off the canvas")
            .example("pixelox fx long-shadow d.pxo --in-place --layer Icon --angle 300 --length 40% --fade 100 --opacity 50", "A shorter shadow that fades out")
            .example("pixelox fx long-shadow d.pxo --in-place --layer Title --color '#0d6b62' --blend normal --opacity 100", "A solid shadow in a darker shade of the ground"),
        OpSpec::doc("fx extrude", Category::Fx, "Add or replace a 3D extrusion (block sides)", extrude_op)
            .about(
                "Gives the layer depth: its shape is swept back along --angle for --depth, drawn under the layer as the \
                 sides of a solid block (retro block lettering, 3D badges). --color auto takes each side's color from \
                 the face pixel it recedes from, darkened by --darken (so gradients and multi-colored text extrude \
                 correctly); --shade darkens the sides further toward the back. --outline strokes the whole block \
                 (face and sides) from outside, closing the far end. Live like the other styles, on any layer kind, \
                 never cut at the layer's bounds; fx long-shadow and fx drop-shadow on the same layer are cast by the \
                 block.",
            )
            .param(common::layer())
            .param(param("angle", ParamKind::Angle).default("315").help("Direction the sides recede in degrees (0 = right, 90 = up, 315 = down-right)"))
            .param(param("depth", ParamKind::Length).default("8%").help("Depth of the block: px, or % of the layer's height"))
            .param(param("color", ParamKind::Color).auto().default("auto").help("Side color, or auto: the face's colors darkened by --darken"))
            .param(param("darken", ParamKind::Float).range("0..100").default("35").help("How much darker than the face the sides are with --color auto, in percent"))
            .param(param("shade", ParamKind::Float).range("0..100").default("0").help("Extra darkening toward the back of the block, in percent"))
            .param(param("outline", ParamKind::Float).range("0..1000").default("0").help("Width in px of a stroke around the whole block (0 = none)"))
            .param(param("outline-color", ParamKind::Color).default("#000000").help("Color of --outline"))
            .params(look("normal", "100", "Blend mode of the sides"))
            .example("pixelox fx extrude d.pxo --in-place --layer Title", "Solid sides in a darker shade of the letters")
            .example("pixelox fx extrude d.pxo --in-place --layer Title --depth 28 --angle 300 --color '#b3261e' --outline 4 --outline-color '#3b0d0a'", "Red block lettering with a dark outline")
            .example("pixelox fx extrude d.pxo --in-place --layer Badge --depth 12% --shade 40", "Sides that darken toward the back"),
        OpSpec::doc("fx reflection", Category::Fx, "Add or replace a floor reflection", reflection_op)
            .about(
                "A mirror image of the layer on a glossy floor: the layer (with its extrusion and strokes, not its \
                 shadows or glows) is flipped about its bottom edge, --gap px below it, and fades out over --length \
                 (--fade sets how much of that length the fade takes). --opacity is the opacity where it starts. --blur \
                 softens it, growing with the distance from the floor line by --distance percent (100 = sharp where the \
                 object touches the floor, as on real glossy surfaces). Live; trim a cutout with transform trim first \
                 so the reflection touches it.",
            )
            .param(common::layer())
            .param(param("gap", ParamKind::Float).range("-10000..10000").default("0").help("Space in px between the layer's bottom edge and the reflection"))
            .param(param("length", ParamKind::Length).default("40%").help("How far the reflection reaches: px, or % of the layer's height"))
            .param(param("fade", ParamKind::Float).range("0..100").default("100").help("Share of the length over which it fades out, in percent (0 = a hard end)"))
            .param(param("blur", ParamKind::Float).range("0..1000").default("0").help("Blur radius in px at the far end (Gaussian sigma = radius / 2)"))
            .param(param("distance", ParamKind::Float).range("0..100").default("100").help("How much the blur grows with distance from the floor line, in percent (0 = even)"))
            .params(look("normal", "35", "Blend mode of the reflection"))
            .example("pixelox fx reflection ad.pxo --in-place --layer Product", "A faded mirror image under a product")
            .example("pixelox fx reflection ad.pxo --in-place --layer Product --length 30% --opacity 45 --blur 12", "Shorter, softer, blurring with distance")
            .example("pixelox fx reflection d.pxo --in-place --layer Title --gap 6 --fade 60 --opacity 25", "A reflection of a title, a little below it"),
    ]
}

fn long_shadow_op(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("long-shadow", a, doc, ctx)
}
fn extrude_op(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("extrude", a, doc, ctx)
}
fn reflection_op(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    set_style("reflection", a, doc, ctx)
}

/// A length param: px, or % of `reference`.
fn length(p: &Params, key: &str, d: &str, reference: f64, doc: &Document) -> Result<f64> {
    let v = p.raw(key).unwrap_or_else(|| d.to_string());
    let l = Length::parse(&v).map_err(|_| p.bad(key, &v, "a length (px or %)"))?;
    Ok(l.to_px(reference, doc.dpi()))
}

/// Interpret a long-shadow, extrude, or reflection spec. `size` is the content's visible
/// width and height.
pub(crate) fn parse(p: &Params, doc: &Document, size: (u32, u32)) -> Result<(Look, Cast)> {
    let pct = |key: &str, d: f64| -> Result<f64> { Ok(p.num(key, d)?.clamp(0.0, 100.0) / 100.0) };
    Ok(match p.kind() {
        "long-shadow" => {
            let diag = (doc.width as f64).hypot(doc.height as f64);
            let len = length(p, "length", "100%", diag, doc)?.clamp(0.0, 4.0 * diag);
            (
                p.look("blend", BlendMode::Multiply, 35.0)?,
                Cast::LongShadow(LongShadow {
                    color: p.color("color", Rgba::BLACK)?,
                    angle: p.angle("angle", 315.0)?,
                    length: len,
                    fade: pct("fade", 0.0)?,
                }),
            )
        }
        "extrude" => {
            let color = match p.raw("color").map(|v| v.trim().to_ascii_lowercase()) {
                None => None,
                Some(v) if v == "auto" || v.is_empty() => None,
                Some(_) => Some(p.color("color", Rgba::BLACK)?),
            };
            (
                p.look("blend", BlendMode::Normal, 100.0)?,
                Cast::Extrude(Extrude {
                    color,
                    darken: pct("darken", 35.0)? as f32,
                    shade: pct("shade", 0.0)? as f32,
                    angle: p.angle("angle", 315.0)?,
                    depth: length(p, "depth", "8%", size.1 as f64, doc)?.clamp(0.0, 4096.0),
                    outline: p.num("outline", 0.0)?.clamp(0.0, 1000.0),
                    outline_color: p.color("outline_color", Rgba::BLACK)?,
                }),
            )
        }
        _ => (
            p.look("blend", BlendMode::Normal, 35.0)?,
            Cast::Reflection(Reflection {
                gap: p.num("gap", 0.0)?.clamp(-10000.0, 10000.0),
                length: length(p, "length", "40%", size.1 as f64, doc)?.clamp(0.0, 4096.0),
                fade: pct("fade", 100.0)?,
                blur: p.num("blur", 0.0)?.clamp(0.0, 1000.0),
                distance: pct("distance", 100.0)?,
            }),
        ),
    })
}

// ---------------------------------------------------------------------------------------
// plane helpers

fn transpose(p: &[f32], w: usize, h: usize, c: usize) -> Vec<f32> {
    let mut out = vec![0.0; p.len()];
    for y in 0..h {
        for x in 0..w {
            for k in 0..c {
                out[(x * h + y) * c + k] = p[(y * w + x) * c + k];
            }
        }
    }
    out
}

fn flip_h(p: &[f32], w: usize, h: usize, c: usize) -> Vec<f32> {
    let mut out = vec![0.0; p.len()];
    for y in 0..h {
        for x in 0..w {
            for k in 0..c {
                out[(y * w + (w - 1 - x)) * c + k] = p[(y * w + x) * c + k];
            }
        }
    }
    out
}

/// Bring planes into the frame where the sweep runs along +x with slope |s| ≤ 1, and back.
struct Frame {
    swap: bool,
    flip: bool,
    w: usize,
    h: usize,
}

impl Frame {
    fn new(d: (f64, f64), w: usize, h: usize) -> (Frame, f64) {
        let swap = d.1.abs() > d.0.abs();
        let (dx, dy) = if swap { (d.1, d.0) } else { d };
        let flip = dx < 0.0;
        let s = if dx.abs() < 1e-12 { 0.0 } else { dy / dx.abs() };
        let (fw, fh) = if swap { (h, w) } else { (w, h) };
        (Frame { swap, flip, w: fw, h: fh }, s)
    }
    fn forward(&self, p: &[f32], c: usize) -> Vec<f32> {
        // original size is (h, w) when swapped
        let mut v = if self.swap { transpose(p, self.h, self.w, c) } else { p.to_vec() };
        if self.flip {
            v = flip_h(&v, self.w, self.h, c);
        }
        v
    }
    fn back(&self, p: &[f32], c: usize) -> Vec<f32> {
        let mut v = if self.flip { flip_h(p, self.w, self.h, c) } else { p.to_vec() };
        if self.swap {
            v = transpose(&v, self.w, self.h, c);
        }
        v
    }
}

/// What a sweep found at one output pixel.
#[derive(Clone, Copy, Default)]
struct Hit {
    cov: f32,
    /// Steps back to the nearest face pixel (alpha ≥ 0.5), or -1 when none.
    dist: f32,
    /// That pixel's color (straight RGB).
    rgb: [f32; 3],
}

/// One line of a sweep: the coverage at each column, and the nearest face pixel behind it.
type LineHits = Vec<(f32, Option<(f32, [f32; 3])>)>;

/// Sweep `alpha` (w×h, in the frame: direction (1, s), |s| ≤ 1) back over `steps` columns
/// (fractional: the last column counts by its fraction). Each output pixel averages two
/// lines a quarter pixel either side of its center, which anti-aliases the sides of the
/// sweep even at 45°, and keeps its own alpha as the near end (so the shape's own edges get
/// no fringe). With `rgb` (premultiplied, 3 channels), also report the nearest face pixel's
/// color along the lines.
fn sweep_frame(alpha: &[f32], rgb: Option<&[f32]>, w: usize, h: usize, s: f64, steps: f64) -> Vec<Hit> {
    let n_steps = steps.floor().max(0.0) as usize;
    let frac = (steps - n_steps as f64) as f32;
    let k = PHASES as i64;
    // lines are indexed in quarter pixels: the line through output pixel (x, y) is
    // Q = K·y + c(x), with c(x) = round(-K·s·x); it samples y' = Q / K + s·x' at column x'
    let c: Vec<i64> = (0..w).map(|x| (-s * x as f64 * k as f64).round() as i64).collect();
    let c_min = c.iter().copied().min().unwrap_or(0);
    let c_max = c.iter().copied().max().unwrap_or(0);
    let (l_min, l_max) = (c_min - 1, k * (h as i64 - 1) + c_max + 1);
    let sample = |plane: &[f32], ch: usize, ci: usize, x: usize, y: f64| -> f32 {
        let y0 = y.floor();
        let f = (y - y0) as f32;
        let y0 = y0 as i64;
        let at = |yy: i64| if yy < 0 || yy >= h as i64 { 0.0 } else { plane[(yy as usize * w + x) * ch + ci] };
        at(y0) * (1.0 - f) + at(y0 + 1) * f
    };
    let line = |l: i64| -> LineHits {
        let lam = l as f64 / k as f64;
        let ys = |x: usize| lam + s * x as f64;
        let b: Vec<f32> = (0..w).map(|x| sample(alpha, 1, 0, x, ys(x))).collect();
        let mut res = Vec::with_capacity(w);
        // sliding maxima of b over j in [x - n, x - 1] and [x - n - 1, x - 1]
        let mut near: VecDeque<usize> = VecDeque::new();
        let mut far: VecDeque<usize> = VecDeque::new();
        let mut last_face: Option<usize> = None;
        for x in 0..w {
            for (dq, reach) in [(&mut near, n_steps), (&mut far, n_steps + 1)] {
                while dq.front().is_some_and(|&j| j + reach < x) {
                    dq.pop_front();
                }
            }
            let m0 = near.front().map(|&j| b[j]).unwrap_or(0.0);
            let m1 = far.front().map(|&j| b[j]).unwrap_or(0.0);
            let cov = (m0 + (m1 - m0) * frac).clamp(0.0, 1.0);
            let hit = match (rgb, last_face) {
                (Some(rgb), Some(f)) if x - f <= n_steps => {
                    let a = b[f].max(1e-6);
                    let y = ys(f);
                    Some(((x - f) as f32, [0, 1, 2].map(|ci| (sample(rgb, 3, ci, f, y) / a).clamp(0.0, 1.0))))
                }
                _ => None,
            };
            res.push((cov, hit));
            for dq in [&mut near, &mut far] {
                while dq.back().is_some_and(|&j| b[j] <= b[x]) {
                    dq.pop_back();
                }
                dq.push_back(x);
            }
            if b[x] >= 0.5 {
                last_face = Some(x);
            }
        }
        res
    };
    let mut cov = vec![0f32; w * h];
    let mut col = vec![[0f32; 3]; w * h];
    let mut col_w = vec![0f32; w * h];
    let mut dist = vec![f32::INFINITY; w * h];
    // lines in chunks, so the results waiting to be scattered stay small
    let mut start = l_min;
    while start <= l_max {
        let end = (start + 255).min(l_max);
        let lines: Vec<(i64, LineHits)> = (start..=end).into_par_iter().map(|l| (l, line(l))).collect();
        for (l, res) in lines {
            for (x, (cv, hit)) in res.into_iter().enumerate() {
                // this line is a quarter pixel below or above the center line Q = l ± 1
                for q in [l - 1, l + 1] {
                    let off = q - c[x];
                    if off.rem_euclid(k) != 0 {
                        continue;
                    }
                    let y = off.div_euclid(k);
                    if y < 0 || y >= h as i64 {
                        continue;
                    }
                    let i = y as usize * w + x;
                    cov[i] += 0.5 * cv;
                    if let Some((d, rgb)) = hit {
                        dist[i] = dist[i].min(d);
                        col_w[i] += 0.5;
                        for (a, v) in col[i].iter_mut().zip(rgb) {
                            *a += 0.5 * v;
                        }
                    }
                }
            }
        }
        start = end + 1;
    }
    (0..w * h)
        .map(|i| Hit {
            cov: cov[i].max(alpha[i]).min(1.0),
            dist: if dist[i].is_finite() { dist[i] } else { -1.0 },
            rgb: if col_w[i] > 0.0 { col[i].map(|v| v / col_w[i]) } else { [0.0; 3] },
        })
        .collect()
}

/// Sweep a mask along `angle` for `len` px; with `colors`, also the nearest face color and
/// distance per pixel.
fn sweep(a: &Mask, colors: Option<&Image>, angle: f64, len: f64) -> (Mask, Option<(Image, Vec<f32>)>) {
    let (w, h) = (a.width as usize, a.height as usize);
    let d = dir(angle);
    let (frame, s) = Frame::new(d, w, h);
    let step = (1.0 + s * s).sqrt();
    let steps = (len / step).max(0.0);
    let alpha = frame.forward(&a.data, 1);
    let rgb: Option<Vec<f32>> = colors.map(|img| {
        let pm: Vec<f32> = img.data.chunks(4).flat_map(|p| [p[0] * p[3], p[1] * p[3], p[2] * p[3]]).collect();
        frame.forward(&pm, 3)
    });
    let hits = sweep_frame(&alpha, rgb.as_deref(), frame.w, frame.h, s, steps);
    let cov: Vec<f32> = hits.iter().map(|h| h.cov).collect();
    let mask = Mask { width: a.width, height: a.height, data: frame.back(&cov, 1) };
    let extra = colors.map(|_| {
        let px: Vec<f32> = hits.iter().flat_map(|h| [h.rgb[0], h.rgb[1], h.rgb[2], 1.0]).collect();
        let dist: Vec<f32> = hits.iter().map(|h| h.dist * step as f32).collect();
        let img = Image { width: a.width, height: a.height, data: frame.back(&px, 4) };
        (img, frame.back(&dist, 1))
    });
    (mask, extra)
}

/// Long-shadow coverage of `a` (beneath the content).
///
/// The fade is one gradient along the shadow (full at the trailing edge of the casting
/// shape, `1 − fade` at the farthest point the shadow reaches), so gaps between letters
/// do not streak it.
pub(crate) fn long_shadow(a: &Mask, s: &LongShadow) -> Mask {
    let mut cov = sweep(a, None, s.angle, s.length).0;
    if s.fade > 0.0 && s.length > 0.0 {
        let (dx, dy) = dir(s.angle);
        let w = a.width as usize;
        let proj = |x: usize, y: usize| x as f64 * dx + y as f64 * dy;
        let tail = a
            .data
            .iter()
            .enumerate()
            .filter(|(_, v)| **v > 0.0)
            .map(|(i, _)| proj(i % w, i / w))
            .fold(f64::NEG_INFINITY, f64::max);
        if tail.is_finite() {
            cov.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
                for (x, v) in row.iter_mut().enumerate() {
                    let t = ((proj(x, y) - tail) / s.length).clamp(0.0, 1.0);
                    *v *= (1.0 - s.fade * t) as f32;
                }
            });
        }
    }
    cov
}

/// An extrusion's sides: coverage (face included) and the side colors as an image whose
/// alpha is that coverage.
pub(crate) fn extrude(a: &Mask, face: &Image, e: &Extrude) -> (Mask, Image) {
    let (cov, extra) = sweep(a, Some(face), e.angle, e.depth);
    let (near, dist) = extra.expect("colors requested");
    let depth = e.depth.max(1e-6) as f32;
    let mut img = Image::new(a.width, a.height);
    img.data.par_chunks_mut(4).enumerate().for_each(|(i, px)| {
        let c = cov.data[i];
        if c <= 0.0 {
            return;
        }
        let shade = if dist[i] >= 0.0 { 1.0 - e.shade * (dist[i] / depth).clamp(0.0, 1.0) } else { 1.0 - e.shade };
        let base = match e.color {
            Some(col) => [col[0], col[1], col[2]],
            None => {
                let n = &near.data[i * 4..i * 4 + 3];
                [n[0] * (1.0 - e.darken), n[1] * (1.0 - e.darken), n[2] * (1.0 - e.darken)]
            }
        };
        px[0] = base[0] * shade;
        px[1] = base[1] * shade;
        px[2] = base[2] * shade;
        px[3] = c * e.color.map(|col| col[3]).unwrap_or(1.0);
    });
    (cov, img)
}

/// The reflection of `body` (buffer-sized) under the content box `cbox` = [x, y, w, h]:
/// an image of the same size holding only the mirror image.
pub(crate) fn reflection(body: &Image, cbox: [f64; 4], r: &Reflection) -> Image {
    let (w, h) = (body.width as usize, body.height as usize);
    let bottom = (cbox[1] + cbox[3]).round() as i64; // first row below the content
    let floor = bottom + r.gap.round() as i64;
    let len = r.length.max(0.0);
    let rows = len.ceil() as usize;
    let mut out = Image::new(body.width, body.height);
    if rows == 0 || w == 0 {
        return out;
    }
    // the mirrored strip, unfaded: strip row i shows content row bottom - 1 - i
    let pad = (1.5 * r.blur).ceil() as usize;
    let sh = rows + 2 * pad;
    let mut strip = Image::new(body.width, sh as u32);
    for i in 0..rows {
        let src = bottom - 1 - i as i64;
        if src < 0 || src >= h as i64 {
            continue;
        }
        let s0 = src as usize * w * 4;
        let d0 = (i + pad) * w * 4;
        strip.data[d0..d0 + w * 4].copy_from_slice(&body.data[s0..s0 + w * 4]);
    }
    // blur levels (radius = 2σ, as in the other styles); rows mix the two nearest levels
    let levels: Vec<Image> = if r.blur > 0.0 {
        (0..=4).map(|l| kernels::gaussian_blur(&strip, r.blur * l as f64 / 4.0 / 2.0, Edge::Zero)).collect()
    } else {
        vec![strip]
    };
    let fade = r.fade.clamp(0.0, 1.0);
    for y in 0..(rows + pad) {
        let oy = floor + y as i64;
        if oy < 0 || oy >= h as i64 {
            continue;
        }
        let t = (y as f64 + 0.5) / len;
        // opacity: full until the fade starts, then linear to 0 at the far end
        let k = if t >= 1.0 {
            0.0
        } else if fade <= 0.0 || t <= 1.0 - fade {
            1.0
        } else {
            (1.0 - t) / fade
        };
        if k <= 0.0 {
            continue;
        }
        let blur_t = (1.0 - r.distance) + r.distance * t.min(1.0);
        let lv = blur_t * (levels.len() - 1) as f64;
        let (l0, f) = (lv.floor() as usize, (lv - lv.floor()) as f32);
        let l1 = (l0 + 1).min(levels.len() - 1);
        let sy = y + pad;
        for x in 0..w {
            let i = (sy * w + x) * 4;
            let (a0, a1) = (&levels[l0].data[i..i + 4], &levels[l1].data[i..i + 4]);
            let al = a0[3] * (1.0 - f) + a1[3] * f;
            if al <= 0.0 {
                continue;
            }
            let o = (oy as usize * w + x) * 4;
            for c in 0..3 {
                out.data[o + c] = (a0[c] * a0[3] * (1.0 - f) + a1[c] * a1[3] * f) / al;
            }
            out.data[o + 3] = al * k as f32;
        }
    }
    // the blur spreads above the floor line too (rows -pad..0 of the strip)
    if pad > 0 {
        for y in 0..pad {
            let oy = floor - (pad - y) as i64;
            if oy < 0 || oy >= h as i64 || oy < bottom {
                continue;
            }
            let lv = (1.0 - r.distance) * (levels.len() - 1) as f64;
            let l0 = lv.round() as usize;
            for x in 0..w {
                let i = (y * w + x) * 4;
                let p = &levels[l0].data[i..i + 4];
                if p[3] > 0.0 {
                    let o = (oy as usize * w + x) * 4;
                    out.data[o..o + 4].copy_from_slice(p);
                }
            }
        }
    }
    out
}

/// The content's alpha over the effect buffer, continued past the canvas edge where the
/// content was cut by it: a side of the content image that lies on or beyond the canvas edge
/// is extended by repeating its edge pixels, so inner effects (inner glow, inner shadow,
/// satin, bevel, inside and center strokes) do not draw a false edge along the canvas, and
/// offset effects (drop shadows) do not stop short of it.
///
/// `alpha` is the buffer (top-left at `origin` on the canvas); `content` is the content
/// image's rect on the canvas.
pub(crate) fn continued(alpha: &Mask, origin: (i32, i32), content: RectI, image: &Image, doc: &Document) -> Mask {
    let (cw, ch) = (doc.width as i64, doc.height as i64);
    let (cx0, cy0) = (content.x as i64, content.y as i64);
    let (cx1, cy1) = (cx0 + content.w as i64, cy0 + content.h as i64);
    let cut = [cx0 <= 0, cy0 <= 0, cx1 >= cw, cy1 >= ch];
    if !cut.iter().any(|c| *c) || content.w == 0 || content.h == 0 {
        return alpha.clone();
    }
    let mut out = alpha.clone();
    let w = alpha.width as usize;
    out.data.par_chunks_mut(w).enumerate().for_each(|(by, row)| {
        let y = origin.1 as i64 + by as i64;
        let sy = if y < cy0 && cut[1] {
            Some(cy0)
        } else if y >= cy1 && cut[3] {
            Some(cy1 - 1)
        } else if (cy0..cy1).contains(&y) {
            Some(y)
        } else {
            None
        };
        for (bx, v) in row.iter_mut().enumerate() {
            let x = origin.0 as i64 + bx as i64;
            let inside = (cx0..cx1).contains(&x) && (cy0..cy1).contains(&y);
            if inside {
                continue;
            }
            let sx = if x < cx0 && cut[0] {
                Some(cx0)
            } else if x >= cx1 && cut[2] {
                Some(cx1 - 1)
            } else if (cx0..cx1).contains(&x) {
                Some(x)
            } else {
                None
            };
            if let (Some(sx), Some(sy)) = (sx, sy) {
                *v = image.get((sx - cx0) as u32, (sy - cy0) as u32)[3];
            }
        }
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(w: u32, h: u32, r: RectI) -> Mask {
        Mask::from_fn(w, h, |x, y| {
            let (x, y) = (x as i32, y as i32);
            if x >= r.x && y >= r.y && x < r.x + r.w as i32 && y < r.y + r.h as i32 {
                1.0
            } else {
                0.0
            }
        })
    }

    #[test]
    fn horizontal_sweep_reaches_its_length() {
        let a = square(100, 20, RectI::new(10, 5, 10, 10));
        let s = long_shadow(&a, &LongShadow { color: [0.0; 4], angle: 0.0, length: 30.0, fade: 0.0 });
        assert!(s.get(15, 10) > 0.99, "under the square");
        assert!(s.get(45, 10) > 0.99, "29 px beyond the square's right edge... within reach");
        assert!(s.get(52, 10) < 0.01, "beyond the length");
        assert!(s.get(5, 10) < 0.01, "nothing behind it");
        assert!(s.get(30, 3) < 0.01 && s.get(30, 16) < 0.01, "no spread sideways");
    }

    #[test]
    fn every_direction_matches_a_brute_force_sweep() {
        let a = square(60, 60, RectI::new(22, 24, 12, 9));
        for angle in [0.0, 30.0, 45.0, 90.0, 135.0, 200.0, 270.0, 315.0, 350.0] {
            let len = 18.0;
            let s = long_shadow(&a, &LongShadow { color: [0.0; 4], angle, length: len, fade: 0.0 });
            let (dx, dy) = dir(angle);
            // bilinear alpha with pixel centers on integer coordinates
            let bilinear = |px: f64, py: f64| {
                let (x0, y0) = (px.floor(), py.floor());
                let (fx, fy) = ((px - x0) as f32, (py - y0) as f32);
                let (x0, y0) = (x0 as i64, y0 as i64);
                let row = |y: i64| a.at(x0, y) * (1.0 - fx) + a.at(x0 + 1, y) * fx;
                row(y0) * (1.0 - fy) + row(y0 + 1) * fy
            };
            let (mut worst, mut total) = (0f32, 0f32);
            for y in 0..60u32 {
                for x in 0..60u32 {
                    // brute force: fine steps back along the direction
                    let mut m = 0f32;
                    let mut t = 0.0;
                    while t <= len + 1e-9 {
                        m = m.max(bilinear(x as f64 - dx * t, y as f64 - dy * t));
                        t += 0.05;
                    }
                    let d = (m - s.get(x, y)).abs();
                    worst = worst.max(d);
                    total += d;
                }
            }
            let mean = total / 3600.0;
            assert!(worst < 0.4 && mean < 0.01, "angle {angle}: worst {worst}, mean {mean}");
        }
    }

    #[test]
    fn fade_runs_out_at_the_far_end() {
        let a = square(200, 10, RectI::new(0, 0, 10, 10));
        let s = long_shadow(&a, &LongShadow { color: [0.0; 4], angle: 0.0, length: 100.0, fade: 1.0 });
        assert!((s.get(9, 5) - 1.0).abs() < 1e-3);
        let mid = s.get(59, 5);
        assert!((mid - 0.5).abs() < 0.03, "half way: {mid}");
        assert!(s.get(108, 5) < 0.02);
    }

    #[test]
    fn auto_extrusion_takes_the_face_color() {
        let a = square(80, 80, RectI::new(20, 20, 20, 20));
        let face = Image::from_fn(80, 80, |x, y| {
            let inside = (20..40).contains(&x) && (20..40).contains(&y);
            if inside {
                [1.0, 0.5, 0.0, 1.0]
            } else {
                [0.0; 4]
            }
        });
        let e = Extrude {
            color: None,
            darken: 0.5,
            shade: 0.0,
            angle: 315.0,
            depth: 10.0,
            outline: 0.0,
            outline_color: [0.0; 4],
        };
        let (cov, img) = extrude(&a, &face, &e);
        assert!(cov.get(45, 45) > 0.9, "the side reaches down-right");
        assert!(cov.get(15, 15) < 0.01, "not up-left");
        let p = img.get(45, 45);
        assert!((p[0] - 0.5).abs() < 0.05 && (p[1] - 0.25).abs() < 0.05, "darkened face color: {p:?}");
    }

    #[test]
    fn reflection_mirrors_below_the_content() {
        // a 10-px tall bar: red top half, blue bottom half
        let body = Image::from_fn(20, 40, |_, y| match y {
            0..=4 => [1.0, 0.0, 0.0, 1.0],
            5..=9 => [0.0, 0.0, 1.0, 1.0],
            _ => [0.0; 4],
        });
        let r = Reflection { gap: 0.0, length: 10.0, fade: 0.0, blur: 0.0, distance: 1.0 };
        let out = reflection(&body, [0.0, 0.0, 20.0, 10.0], &r);
        assert_eq!(out.get(5, 10), [0.0, 0.0, 1.0, 1.0], "blue (the bottom) touches the floor line");
        assert_eq!(out.get(5, 19), [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(out.get(5, 20)[3], 0.0);
        let faded = reflection(&body, [0.0, 0.0, 20.0, 10.0], &Reflection { fade: 1.0, ..r.clone() });
        assert!(faded.get(5, 10)[3] > 0.9 && faded.get(5, 19)[3] < 0.1);
        let gap = reflection(&body, [0.0, 0.0, 20.0, 10.0], &Reflection { gap: 4.0, ..r });
        assert_eq!(gap.get(5, 12)[3], 0.0);
        assert_eq!(gap.get(5, 14), [0.0, 0.0, 1.0, 1.0]);
    }

    #[test]
    fn cut_content_continues_past_the_canvas() {
        let doc = Document::new(50, 50);
        // a content image touching the left canvas edge, opaque in rows 10..20
        let img = Image::from_fn(10, 30, |_, y| if (10..20).contains(&y) { [0.0, 0.0, 0.0, 1.0] } else { [0.0; 4] });
        let content = RectI::new(0, 0, 10, 30);
        // a buffer starting 5 px left of the canvas
        let buf = Mask::from_fn(20, 30, |x, y| if (5..15).contains(&x) { img.get(x - 5, y)[3] } else { 0.0 });
        let c = continued(&buf, (-5, 0), content, &img, &doc);
        assert_eq!(c.get(0, 15), 1.0, "continued to the left");
        assert_eq!(c.get(0, 5), 0.0);
        // not cut on the right: the right side stays empty
        assert_eq!(c.get(18, 15), 0.0);
        // content not touching the canvas edge is left alone
        let inner = RectI::new(5, 5, 10, 30);
        let c2 = continued(&buf, (-5, 0), inner, &img, &doc);
        assert_eq!(c2.data, buf.data);
    }
}
