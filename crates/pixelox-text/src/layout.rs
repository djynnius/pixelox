//! Shaping and layout: the face set (the chosen face plus the glyph-fallback chain),
//! bidi reordering (unicode-bidi), line breaking (unicode-linebreak), shaping (rustybuzz),
//! alignment, justification, tracking, leading, rotation, and text on a path. The result is
//! glyph outlines in canvas pixels ([`Composed`]), which `render.rs` rasterizes or writes as
//! SVG path data. No hinting: the same outlines on every platform.

use crate::fonts::{self, FaceChoice, FontDb, FontStyle, Request};
use crate::spec::{parse_variations, Align, Side, TextSpec};
use kurbo::{Affine, BezPath, PathEl, Point, Shape};
use pixelox_core::prelude::*;
use pixelox_core::units::Length;
use rustybuzz::ttf_parser::{self, GlyphId, Tag};
use rustybuzz::{Direction, Feature, UnicodeBuffer, Variation};
use std::ops::Range;
use std::sync::Arc;
use unicode_bidi::{BidiClass, Level, ParagraphBidiInfo};
use unicode_linebreak::{linebreaks, BreakOpportunity};

/// Slant of synthetic oblique: tan(12°).
const OBLIQUE_SKEW: f64 = 0.212_556_560_67;
/// Synthetic bold: stroke width as a fraction of the font size.
pub const BOLD_STRENGTH: f64 = 1.0 / 24.0;

/// One face of a layer's face set, with the axis values and synthetic styles it is drawn with.
pub struct Slot {
    pub choice: FaceChoice,
    pub variations: Vec<Variation>,
    pub synthetic_bold: bool,
    pub synthetic_oblique: bool,
}

fn tag4(t: &str) -> Tag {
    let mut b = [b' '; 4];
    for (i, c) in t.bytes().take(4).enumerate() {
        b[i] = c;
    }
    Tag::from_bytes(&b)
}

fn unreadable(c: &FaceChoice) -> Error {
    Error::new("PX4101", format!("cannot parse font face '{}' ({})", c.postscript, c.file.key))
        .with_context("file", &c.file.key)
}

fn make_slot(choice: FaceChoice, weight: u16, style: FontStyle, user: &[(String, f32)], size_px: f64) -> Result<Slot> {
    let face = ttf_parser::Face::parse(&choice.file.data, choice.index).map_err(|_| unreadable(&choice))?;
    let axes: Vec<ttf_parser::VariationAxis> = face.variation_axes().into_iter().collect();
    let axis = |t: &[u8; 4]| axes.iter().find(|a| a.tag == Tag::from_bytes(t)).copied();
    let user_has = |t: &str| user.iter().any(|(u, _)| u.trim_end() == t);
    let mut vars: Vec<Variation> = user.iter().map(|(t, v)| Variation { tag: tag4(t), value: *v }).collect();
    let wght = axis(b"wght");
    if let Some(a) = wght.filter(|_| !user_has("wght")) {
        vars.push(Variation { tag: a.tag, value: (weight as f32).clamp(a.min_value, a.max_value) });
    }
    let italic_axis = axis(b"ital").or(axis(b"slnt"));
    if style != FontStyle::Normal && !user_has("ital") && !user_has("slnt") {
        if let Some(a) = axis(b"ital") {
            vars.push(Variation { tag: a.tag, value: a.max_value.min(1.0) });
        } else if let Some(a) = axis(b"slnt") {
            vars.push(Variation { tag: a.tag, value: (-12.0f32).clamp(a.min_value, a.max_value) });
        }
    }
    if let Some(a) = axis(b"opsz").filter(|_| !user_has("opsz")) {
        vars.push(Variation { tag: a.tag, value: (size_px as f32).clamp(a.min_value, a.max_value) });
    }
    let user_bold = user.iter().any(|(t, v)| t.trim_end() == "wght" && *v >= 600.0);
    let synthetic_bold = weight >= 600 && choice.weight < 600 && wght.is_none() && !user_bold;
    let synthetic_oblique = style != FontStyle::Normal
        && choice.style == FontStyle::Normal
        && italic_axis.is_none()
        && !user_has("ital")
        && !user_has("slnt");
    Ok(Slot { choice, variations: vars, synthetic_bold, synthetic_oblique })
}

fn same_face(a: &FaceChoice, b: &FaceChoice) -> bool {
    a.file.key == b.file.key && a.index == b.index
}

/// The chosen face plus fallback faces loaded on demand for characters it lacks.
struct FaceSet {
    slots: Vec<Slot>,
    db: Arc<FontDb>,
    /// Fallback faces to search, resolved on the first missing character.
    chain: Option<Vec<fontdb::ID>>,
    weight: u16,
    style: FontStyle,
    user: Vec<(String, f32)>,
    size_px: f64,
}

fn covers(slot: &Slot, c: char) -> bool {
    ttf_parser::Face::parse(&slot.choice.file.data, slot.choice.index)
        .map(|f| f.glyph_index(c).is_some())
        .unwrap_or(false)
}

impl FaceSet {
    fn face_for(&mut self, c: char) -> Result<Option<usize>> {
        if let Some(i) = (1..self.slots.len()).find(|&i| covers(&self.slots[i], c)) {
            return Ok(Some(i));
        }
        let chain = self.chain.get_or_insert_with(|| fonts::chain_ids(&self.db, self.weight, self.style)).clone();
        for id in chain {
            if !fonts::face_covers(&self.db, id, c) {
                continue;
            }
            let ch = fonts::choice_by_id(&self.db, id)?;
            if self.slots.iter().any(|s| same_face(&s.choice, &ch)) {
                continue;
            }
            self.slots.push(make_slot(ch, self.weight, self.style, &self.user, self.size_px)?);
            return Ok(Some(self.slots.len() - 1));
        }
        Ok(None)
    }
}

/// Characters that never trigger a font switch (they stay with the surrounding run).
fn is_ignorable(c: char) -> bool {
    c.is_whitespace()
        || c.is_control()
        || matches!(c,
            '\u{00AD}' | '\u{034F}' | '\u{180B}'..='\u{180E}' | '\u{200B}'..='\u{200F}' | '\u{202A}'..='\u{202E}'
            | '\u{2060}'..='\u{206F}' | '\u{FE00}'..='\u{FE0F}' | '\u{FEFF}' | '\u{E0000}'..='\u{E0FFF}')
}

fn is_mark(c: char) -> bool {
    unicode_bidi::bidi_class(c) == BidiClass::NSM
}

/// The face of every byte of `text` (fallback faces for characters the chosen face lacks).
fn assign_faces(text: &str, set: &mut FaceSet, missing: &mut Vec<char>) -> Result<Vec<usize>> {
    let pfile = set.slots[0].choice.file.clone();
    let primary = ttf_parser::Face::parse(&pfile.data, set.slots[0].choice.index)
        .map_err(|_| unreadable(&set.slots[0].choice))?;
    let mut out = vec![0usize; text.len()];
    let mut prev = 0usize;
    for (i, c) in text.char_indices() {
        let f = if is_ignorable(c) || is_mark(c) {
            prev
        } else if primary.glyph_index(c).is_some() {
            0
        } else {
            match set.face_for(c)? {
                Some(f) => f,
                None => {
                    if !missing.contains(&c) {
                        missing.push(c);
                    }
                    0
                }
            }
        };
        out[i..i + c.len_utf8()].fill(f);
        prev = f;
    }
    Ok(out)
}

/// A shaped glyph, in px (`dx`/`dy` are offsets from the pen; `dy` points up).
#[derive(Clone, Copy, Debug)]
struct G {
    slot: usize,
    id: u16,
    cluster: usize,
    adv: f64,
    dx: f64,
    dy: f64,
    space: bool,
}

struct Shaper<'a> {
    hb: &'a [rustybuzz::Face<'a>],
    slots: &'a [Slot],
    features: &'a [Feature],
    size: f64,
    bold_extra: f64,
}

impl Shaper<'_> {
    fn shape(&self, text: &str, r: Range<usize>, slot: usize, rtl: bool) -> Vec<G> {
        if r.is_empty() {
            return Vec::new();
        }
        let face = &self.hb[slot];
        let scale = self.size / face.units_per_em().max(1) as f64;
        let mut buf = UnicodeBuffer::new();
        for (i, c) in text[r.clone()].char_indices() {
            buf.add(c, (r.start + i) as u32);
        }
        buf.guess_segment_properties();
        buf.set_direction(if rtl { Direction::RightToLeft } else { Direction::LeftToRight });
        let out = rustybuzz::shape(face, self.features, buf);
        let extra = if self.slots[slot].synthetic_bold { self.bold_extra } else { 0.0 };
        out.glyph_infos()
            .iter()
            .zip(out.glyph_positions())
            .map(|(i, p)| {
                let cluster = i.cluster as usize;
                G {
                    slot,
                    id: i.glyph_id as u16,
                    cluster,
                    adv: p.x_advance as f64 * scale + if p.x_advance != 0 { extra } else { 0.0 },
                    dx: p.x_offset as f64 * scale,
                    dy: p.y_offset as f64 * scale,
                    space: text.get(cluster..).and_then(|s| s.chars().next()).is_some_and(char::is_whitespace),
                }
            })
            .collect()
    }
}

/// Byte ranges of a line in display order, each with its direction (RTL runs are drawn
/// right to left by the shaper).
fn line_runs(info: &ParagraphBidiInfo, line: Range<usize>) -> Vec<(Range<usize>, bool)> {
    if line.is_empty() {
        return Vec::new();
    }
    let (levels, runs) = info.visual_runs(line);
    runs.into_iter().filter(|r| !r.is_empty()).map(|r| (r.clone(), levels[r.start].is_rtl())).collect()
}

/// Display-order runs of a single-line string (`(text, rtl)` pairs), for inspection and tests.
pub fn visual_runs(text: &str) -> Vec<(String, bool)> {
    let info = ParagraphBidiInfo::new(text, None);
    line_runs(&info, 0..text.len()).into_iter().map(|(r, rtl)| (text[r].to_string(), rtl)).collect()
}

/// Characters in display order, left to right (RTL runs reversed), without shaping.
pub fn visual_order(text: &str) -> String {
    visual_runs(text).into_iter().map(|(s, rtl)| if rtl { s.chars().rev().collect::<String>() } else { s }).collect()
}

/// Logical runs of equal bidi level and face.
fn logical_runs(text: &str, levels: &[Level], face_at: &[usize]) -> Vec<(Range<usize>, usize, bool)> {
    let mut out: Vec<(Range<usize>, usize, bool)> = Vec::new();
    let mut start = 0;
    let mut key: Option<(Level, usize)> = None;
    for (i, _) in text.char_indices() {
        let k = (levels[i], face_at[i]);
        if let Some(prev) = key.filter(|p| *p != k) {
            out.push((start..i, prev.1, prev.0.is_rtl()));
            start = i;
        }
        key = Some(k);
    }
    if let Some(prev) = key {
        out.push((start..text.len(), prev.1, prev.0.is_rtl()));
    }
    out
}

/// Cluster advances in logical order (for line breaking).
struct Measure {
    pre_adv: Vec<f64>,
    pre_cl: Vec<usize>,
    bounds: Vec<usize>,
    track: f64,
}

impl Measure {
    fn new(sh: &Shaper, text: &str, levels: &[Level], face_at: &[usize], track: f64) -> Measure {
        let n = text.len();
        let mut adv = vec![0.0; n];
        let mut is_cl = vec![false; n];
        for (r, slot, rtl) in logical_runs(text, levels, face_at) {
            for g in sh.shape(text, r, slot, rtl) {
                if g.cluster < n {
                    adv[g.cluster] += g.adv;
                    is_cl[g.cluster] = true;
                }
            }
        }
        let mut pre_adv = vec![0.0; n + 1];
        let mut pre_cl = vec![0usize; n + 1];
        for i in 0..n {
            pre_adv[i + 1] = pre_adv[i] + adv[i];
            pre_cl[i + 1] = pre_cl[i] + usize::from(is_cl[i]);
        }
        let mut bounds: Vec<usize> = (0..n).filter(|&i| is_cl[i]).collect();
        bounds.push(n);
        Measure { pre_adv, pre_cl, bounds, track }
    }

    fn width(&self, a: usize, b: usize) -> f64 {
        if b <= a {
            return 0.0;
        }
        let clusters = self.pre_cl[b] - self.pre_cl[a];
        self.pre_adv[b] - self.pre_adv[a] + self.track * clusters.saturating_sub(1) as f64
    }

    /// The last cluster boundary in (a, b] that keeps the width within `max` (at least one
    /// cluster per line).
    fn cut(&self, a: usize, b: usize, max: f64) -> usize {
        let mut best = None;
        for &c in self.bounds.iter().filter(|&&c| c > a && c <= b) {
            if self.width(a, c) <= max + 1e-6 {
                best = Some(c);
            } else {
                return best.unwrap_or(c);
            }
        }
        best.unwrap_or(b)
    }
}

fn trim_end(text: &str, a: usize, mut b: usize) -> usize {
    while b > a {
        match text[..b].chars().next_back() {
            Some(c) if c.is_whitespace() => b -= c.len_utf8(),
            _ => break,
        }
    }
    b
}

/// Greedy line breaking at Unicode break opportunities; words wider than the box break
/// between clusters. Returns `(start, end, ends_paragraph)` byte ranges.
fn wrap(text: &str, m: &Measure, max_w: Option<f64>) -> Vec<(usize, usize, bool)> {
    let mut lines = Vec::new();
    let mut start = 0usize;
    let mut last_ok: Option<usize> = None;
    for (pos, opp) in linebreaks(text) {
        if let Some(mw) = max_w {
            loop {
                let end = trim_end(text, start, pos);
                if end <= start || m.width(start, end) <= mw + 1e-6 {
                    break;
                }
                if let Some(lb) = last_ok.filter(|&lb| lb > start) {
                    lines.push((start, lb, false));
                    start = lb;
                    last_ok = None;
                    continue;
                }
                let cut = m.cut(start, end, mw);
                if cut >= end {
                    break;
                }
                lines.push((start, cut, false));
                start = cut;
            }
        }
        if opp == BreakOpportunity::Mandatory {
            lines.push((start, pos, true));
            start = pos;
            last_ok = None;
        } else {
            last_ok = Some(pos);
        }
    }
    if lines.is_empty() {
        lines.push((0, text.len(), true));
    }
    lines
}

fn line_glyphs(sh: &Shaper, info: &ParagraphBidiInfo, text: &str, r: Range<usize>, face_at: &[usize]) -> Vec<G> {
    let mut out = Vec::new();
    for (run, rtl) in line_runs(info, r) {
        let mut subs: Vec<(Range<usize>, usize)> = Vec::new();
        for (i, _) in text[run.clone()].char_indices() {
            let i = run.start + i;
            match subs.last_mut() {
                Some((sr, f)) if *f == face_at[i] => sr.end = i,
                _ => subs.push((i..i, face_at[i])),
            }
        }
        // close the ranges at the next sub-run's start / the run end
        let n = subs.len();
        for k in 0..n {
            subs[k].0.end = if k + 1 < n { subs[k + 1].0.start } else { run.end };
        }
        if rtl {
            subs.reverse();
        }
        for (sr, f) in subs {
            out.extend(sh.shape(text, sr, f, rtl));
        }
    }
    out
}

/// Add tracking between clusters (never between a base and its marks).
fn apply_tracking(gl: &mut [G], track: f64) {
    if track == 0.0 || gl.len() < 2 {
        return;
    }
    for i in 0..gl.len() - 1 {
        if gl[i + 1].cluster != gl[i].cluster {
            gl[i].adv += track;
        }
    }
}

/// A flattened path for text on a path, with cumulative arc lengths.
pub struct Polyline {
    pts: Vec<Point>,
    cum: Vec<f64>,
    pub closed: bool,
}

fn quad_at(p0: Point, p1: Point, p2: Point, t: f64) -> Point {
    let u = 1.0 - t;
    Point::new(u * u * p0.x + 2.0 * u * t * p1.x + t * t * p2.x, u * u * p0.y + 2.0 * u * t * p1.y + t * t * p2.y)
}

fn cubic_at(p0: Point, p1: Point, p2: Point, p3: Point, t: f64) -> Point {
    let u = 1.0 - t;
    let (a, b, c, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
    Point::new(a * p0.x + b * p1.x + c * p2.x + d * p3.x, a * p0.y + b * p1.y + c * p2.y + d * p3.y)
}

fn dist(a: Point, b: Point) -> f64 {
    ((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)).sqrt()
}

fn steps(poly_len: f64) -> usize {
    ((poly_len / 0.5).ceil() as usize).clamp(1, 4000)
}

pub fn parse_path(d: &str) -> Result<BezPath> {
    BezPath::from_svg(d).map_err(|e| {
        Error::usage(format!("invalid SVG path data: {e}"))
            .with_hint("give path data such as 'M 100,300 C 200,100 400,100 500,300'")
            .with_context("d", d)
    })
}

impl Polyline {
    /// Flatten the first subpath of SVG path data.
    pub fn from_svg(d: &str) -> Result<Polyline> {
        let path = parse_path(d)?;
        let mut pts: Vec<Point> = Vec::new();
        let mut cur = Point::ZERO;
        let mut start = None;
        let mut closed = false;
        for el in path.elements() {
            match *el {
                PathEl::MoveTo(p) => {
                    if !pts.is_empty() {
                        break;
                    }
                    pts.push(p);
                    cur = p;
                    start = Some(p);
                }
                PathEl::LineTo(p) => {
                    pts.push(p);
                    cur = p;
                }
                PathEl::QuadTo(p1, p2) => {
                    let n = steps(dist(cur, p1) + dist(p1, p2));
                    pts.extend((1..=n).map(|i| quad_at(cur, p1, p2, i as f64 / n as f64)));
                    cur = p2;
                }
                PathEl::CurveTo(p1, p2, p3) => {
                    let n = steps(dist(cur, p1) + dist(p1, p2) + dist(p2, p3));
                    pts.extend((1..=n).map(|i| cubic_at(cur, p1, p2, p3, i as f64 / n as f64)));
                    cur = p3;
                }
                PathEl::ClosePath => {
                    if let Some(s) = start {
                        if dist(cur, s) > 1e-9 {
                            pts.push(s);
                        }
                    }
                    closed = true;
                    break;
                }
            }
        }
        pts.dedup_by(|a, b| dist(*a, *b) < 1e-9);
        if !closed && pts.len() > 2 && dist(pts[0], pts[pts.len() - 1]) < 1.0 {
            closed = true;
        }
        let mut cum = vec![0.0; pts.len()];
        for i in 1..pts.len() {
            cum[i] = cum[i - 1] + dist(pts[i - 1], pts[i]);
        }
        if pts.len() < 2 || cum[cum.len() - 1] <= 1e-9 {
            return Err(Error::usage("the path has no length; text cannot follow it").with_context("d", d));
        }
        Ok(Polyline { pts, cum, closed })
    }

    pub fn length(&self) -> f64 {
        self.cum[self.cum.len() - 1]
    }

    /// Shoelace area; positive when the path runs clockwise on screen (y down).
    fn signed_area(&self) -> f64 {
        let n = self.pts.len();
        (0..n)
            .map(|i| {
                let (a, b) = (self.pts[i], self.pts[(i + 1) % n]);
                a.x * b.y - b.x * a.y
            })
            .sum::<f64>()
            / 2.0
    }

    fn reversed(&self) -> Polyline {
        let pts: Vec<Point> = self.pts.iter().rev().copied().collect();
        let total = self.length();
        let cum = self.cum.iter().rev().map(|c| total - c).collect();
        Polyline { pts, cum, closed: self.closed }
    }

    /// Orient so glyphs stand on the requested side: outside of a closed path's interior
    /// (or the left of an open path's direction, i.e. above a left-to-right path).
    fn oriented(self, side: Side) -> Polyline {
        let flip =
            if self.closed { (self.signed_area() >= 0.0) != (side == Side::Outside) } else { side == Side::Inside };
        if flip {
            self.reversed()
        } else {
            self
        }
    }

    /// Point at arc length `s` (wraps on closed paths, extends tangentially on open ones).
    pub fn at(&self, s: f64) -> Point {
        let total = self.length();
        let n = self.pts.len();
        let s = if self.closed { s.rem_euclid(total) } else { s };
        if s <= 0.0 {
            let d = self.dir(0);
            return self.pts[0] + d * s;
        }
        if s >= total {
            let d = self.dir(n - 2);
            return self.pts[n - 1] + d * (s - total);
        }
        let i = match self.cum.binary_search_by(|c| c.partial_cmp(&s).unwrap_or(std::cmp::Ordering::Less)) {
            Ok(i) => return self.pts[i],
            Err(i) => i - 1,
        };
        let seg = self.cum[i + 1] - self.cum[i];
        let t = if seg > 0.0 { (s - self.cum[i]) / seg } else { 0.0 };
        self.pts[i].lerp(self.pts[i + 1], t)
    }

    fn dir(&self, i: usize) -> kurbo::Vec2 {
        let v = self.pts[i + 1] - self.pts[i];
        let l = v.hypot2().sqrt();
        if l > 0.0 {
            v / l
        } else {
            kurbo::Vec2::new(1.0, 0.0)
        }
    }
}

/// Laid-out text: glyph outlines in canvas pixels plus what an op reports.
pub struct Composed {
    /// Outlines of glyphs drawn with their face as is.
    pub regular: BezPath,
    /// Outlines of glyphs that get synthetic bold (stroked by `bold_width` and filled).
    pub bold: BezPath,
    pub bold_width: f64,
    pub color: Rgba,
    pub stroke: Option<(Rgba, f64)>,
    /// The (unrotated) text box `[x, y, w, h]` on the canvas.
    pub layout_box: [f64; 4],
    pub lines: usize,
    pub overflow: bool,
    pub size_px: f64,
    pub primary: FaceChoice,
    /// The requested family was found (false: drawn with the fallback).
    pub found: bool,
    pub synthetic: Vec<&'static str>,
    pub fallbacks: Vec<FaceChoice>,
    pub missing: Vec<char>,
    pub glyphs: usize,
    pub path_length: Option<f64>,
}

impl Composed {
    pub fn is_empty(&self) -> bool {
        self.regular.elements().is_empty() && self.bold.elements().is_empty()
    }

    /// Bounds of the ink (outlines grown by the stroke and synthetic bold), `[x0, y0, x1, y1]`.
    pub fn ink(&self) -> Option<[f64; 4]> {
        let mut r: Option<kurbo::Rect> = None;
        for (p, grow) in [(&self.regular, 0.0), (&self.bold, self.bold_width / 2.0)] {
            if p.elements().is_empty() {
                continue;
            }
            let b = p.bounding_box().inflate(grow, grow);
            r = Some(r.map_or(b, |a| a.union(b)));
        }
        let mut r = r?;
        if let Some((_, w)) = self.stroke {
            r = r.inflate(w / 2.0 + self.bold_width / 2.0, w / 2.0 + self.bold_width / 2.0);
        }
        Some([r.x0, r.y0, r.x1, r.y1])
    }

    /// Rounded-out ink bounds as `[x, y, w, h]`.
    pub fn ink_rect(&self) -> Option<RectI> {
        self.ink().map(|[x0, y0, x1, y1]| {
            let (x0, y0) = (x0.floor(), y0.floor());
            let (x1, y1) = (x1.ceil(), y1.ceil());
            RectI::new(x0 as i32, y0 as i32, (x1 - x0).max(0.0) as u32, (y1 - y0).max(0.0) as u32)
        })
    }
}

struct Sink<'a> {
    p: &'a mut BezPath,
    m: Affine,
}

impl ttf_parser::OutlineBuilder for Sink<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.p.move_to(self.m * Point::new(x as f64, y as f64));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.p.line_to(self.m * Point::new(x as f64, y as f64));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.p.quad_to(self.m * Point::new(x1 as f64, y1 as f64), self.m * Point::new(x as f64, y as f64));
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.p.curve_to(
            self.m * Point::new(x1 as f64, y1 as f64),
            self.m * Point::new(x2 as f64, y2 as f64),
            self.m * Point::new(x as f64, y as f64),
        );
    }
    fn close(&mut self) {
        self.p.close_path();
    }
}

/// Rotation by `deg` (clockwise on screen) about `c`, with platform-independent trig.
fn rotation_about(deg: f64, c: Point) -> Affine {
    if deg.rem_euclid(360.0) == 0.0 {
        return Affine::IDENTITY;
    }
    let r = deg.to_radians();
    let (s, co) = (libm::sin(r), libm::cos(r));
    Affine::translate(c.to_vec2()) * Affine::new([co, s, -s, co, 0.0, 0.0]) * Affine::translate(-c.to_vec2())
}

/// Lay out a text spec on `doc`'s canvas. With `strict`, a missing `font` family fails with
/// PX4102; otherwise it falls back to the embedded face (`found` = false).
pub fn compose(spec: &TextSpec, doc: &Document, strict: bool) -> Result<Composed> {
    let size = spec.size_px(doc);
    let req =
        Request { family: &spec.font, weight: spec.weight, style: spec.style, sha256: spec.font_sha256.as_deref() };
    let (primary, found) = if strict { (fonts::resolve_strict(&req)?, true) } else { fonts::resolve_lenient(&req)? };
    let user: Vec<(String, f32)> = spec.variation.as_deref().and_then(|v| parse_variations(v).ok()).unwrap_or_default();
    let db = fonts::db();
    let first = make_slot(primary.clone(), spec.weight, spec.style, &user, size)?;
    let mut set =
        FaceSet { slots: vec![first], chain: None, db, weight: spec.weight, style: spec.style, user, size_px: size };
    let on_path = spec.path.is_some();
    // fonts rarely have a tab glyph: tabs become four spaces
    let content = spec.content.replace('\t', "    ");
    let paragraphs: Vec<String> = if on_path {
        vec![content.replace("\r\n", " ").replace(['\r', '\n'], " ")]
    } else {
        content.split('\n').map(|p| p.trim_end_matches('\r').to_string()).collect()
    };
    let mut missing = Vec::new();
    let face_maps: Vec<Vec<usize>> =
        paragraphs.iter().map(|p| assign_faces(p, &mut set, &mut missing)).collect::<Result<_>>()?;

    let hb: Vec<rustybuzz::Face> = set
        .slots
        .iter()
        .map(|s| {
            let mut f = rustybuzz::Face::from_slice(&s.choice.file.data, s.choice.index)
                .ok_or_else(|| unreadable(&s.choice))?;
            f.set_variations(&s.variations);
            Ok(f)
        })
        .collect::<Result<_>>()?;
    let mut features = Vec::new();
    if !spec.kerning {
        features.push(Feature::new(Tag::from_bytes(b"kern"), 0, ..));
    }
    if spec.tracking != 0.0 {
        for t in [b"liga", b"clig", b"dlig"] {
            features.push(Feature::new(Tag::from_bytes(t), 0, ..));
        }
    }
    let bold_width = size * BOLD_STRENGTH;
    let sh = Shaper { hb: &hb, slots: &set.slots, features: &features, size, bold_extra: bold_width };
    let track = spec.tracking / 1000.0 * size;

    // vertical metrics of the chosen face
    let f0 = &hb[0];
    let s0 = size / f0.units_per_em().max(1) as f64;
    let (mut asc, mut desc) = (f0.ascender() as f64 * s0, f0.descender() as f64 * s0);
    let gap = (f0.line_gap() as f64 * s0).max(0.0);
    if asc - desc <= 0.0 {
        asc = 0.8 * size;
        desc = -0.2 * size;
    }
    let lh = spec.leading_px(size, doc.dpi()).unwrap_or(asc - desc + gap);

    // shape every line in display order
    struct Line {
        glyphs: Vec<G>,
        width: f64,
        para_end: bool,
    }
    let max_w = if on_path { None } else { spec.box_size.map(|b| b.0) };
    let mut lines: Vec<Line> = Vec::new();
    for (text, face_at) in paragraphs.iter().zip(&face_maps) {
        if text.is_empty() {
            lines.push(Line { glyphs: Vec::new(), width: 0.0, para_end: true });
            continue;
        }
        let info = ParagraphBidiInfo::new(text, None);
        let ranges = if on_path {
            vec![(0, text.len(), true)]
        } else {
            let m = Measure::new(&sh, text, &info.levels, face_at, track);
            wrap(text, &m, max_w)
        };
        for (a, b, para_end) in ranges {
            let end = if on_path { b } else { trim_end(text, a, b) };
            let mut gl = line_glyphs(&sh, &info, text, a..end, face_at);
            apply_tracking(&mut gl, track);
            let width = gl.iter().map(|g| g.adv).sum();
            lines.push(Line { glyphs: gl, width, para_end });
        }
    }

    let mut regular = BezPath::new();
    let mut bold = BezPath::new();
    let mut glyph_count = 0usize;
    let mut outline = |g: &G, frame: Affine| {
        let slot = &set.slots[g.slot];
        let s = size / hb[g.slot].units_per_em().max(1) as f64;
        let k = if slot.synthetic_oblique { OBLIQUE_SKEW } else { 0.0 };
        let local = Affine::new([s, 0.0, s * k, -s, g.dx, -g.dy]);
        let target = if slot.synthetic_bold { &mut bold } else { &mut regular };
        let before = target.elements().len();
        let mut sink = Sink { p: &mut *target, m: frame * local };
        hb[g.slot].outline_glyph(GlyphId(g.id), &mut sink);
        if target.elements().len() > before {
            glyph_count += 1;
        }
    };

    let content_w = lines.iter().map(|l| l.width).fold(0.0, f64::max);
    let mut overflow = false;
    let mut path_length = None;
    let layout_box;
    if let Some(d) = &spec.path {
        let poly = Polyline::from_svg(d)?.oriented(spec.path_side);
        let total = poly.length();
        path_length = Some(total);
        let off = Length::parse(&spec.path_offset).map(|l| l.to_px(total, doc.dpi())).unwrap_or(0.0);
        let line = lines.first();
        let w = line.map(|l| l.width).unwrap_or(0.0);
        let mut pen = match spec.align {
            Align::Left | Align::Justify => off,
            Align::Center => off - w / 2.0,
            Align::Right => off - w,
        };
        overflow = w > total + 0.5;
        for g in line.map(|l| l.glyphs.as_slice()).unwrap_or(&[]) {
            let mid = pen + g.adv / 2.0;
            let h = (g.adv / 2.0).max(0.5);
            let (p0, p1) = (poly.at(mid - h), poly.at(mid + h));
            let v = p1 - p0;
            let l = v.hypot2().sqrt();
            let t = if l > 1e-12 { v / l } else { kurbo::Vec2::new(1.0, 0.0) };
            let origin = poly.at(mid) - t * (g.adv / 2.0);
            let frame = Affine::new([t.x, t.y, -t.y, t.x, origin.x, origin.y]);
            outline(g, frame);
            pen += g.adv;
        }
        layout_box = [0.0, 0.0, 0.0, 0.0];
    } else {
        let box_w = max_w.unwrap_or(content_w);
        let n = lines.len();
        let text_h = if n > 0 { (n - 1) as f64 * lh + (asc - desc) } else { 0.0 };
        let box_h = spec.box_size.and_then(|b| b.1).unwrap_or(text_h);
        if let Some((bw, bh)) = spec.box_size {
            overflow = lines.iter().any(|l| l.width > bw + 0.5) || bh.is_some_and(|bh| text_h > bh + 0.5);
        }
        let (bx, by) = spec.anchor.place(spec.x, spec.y, box_w, box_h);
        let rot = rotation_about(spec.rotation, Point::new(spec.x, spec.y));
        for (i, line) in lines.iter().enumerate() {
            let baseline = asc + i as f64 * lh;
            let x0 = match spec.align {
                Align::Left | Align::Justify => 0.0,
                Align::Center => (box_w - line.width) / 2.0,
                Align::Right => box_w - line.width,
            };
            let mut extra = 0.0;
            if spec.align == Align::Justify && !line.para_end && box_w > line.width {
                let spaces = line.glyphs.iter().filter(|g| g.space).count();
                if spaces > 0 {
                    extra = (box_w - line.width) / spaces as f64;
                }
            }
            let mut pen = x0;
            for g in &line.glyphs {
                let frame = rot * Affine::translate((bx + pen, by + baseline));
                outline(g, frame);
                pen += g.adv + if g.space { extra } else { 0.0 };
            }
        }
        layout_box = [bx, by, box_w, box_h];
    }

    let mut used: Vec<usize> = face_maps.iter().flatten().copied().filter(|&f| f > 0).collect();
    used.sort_unstable();
    used.dedup();
    let fallbacks = used.into_iter().map(|i| set.slots[i].choice.clone()).collect();
    let mut synthetic = Vec::new();
    if set.slots[0].synthetic_bold {
        synthetic.push("bold");
    }
    if set.slots[0].synthetic_oblique {
        synthetic.push(if spec.style == FontStyle::Italic { "italic" } else { "oblique" });
    }
    let has_bold = set.slots.iter().any(|s| s.synthetic_bold);
    Ok(Composed {
        regular,
        bold,
        bold_width: if has_bold { bold_width } else { 0.0 },
        color: spec.color,
        stroke: spec.stroke.filter(|_| spec.stroke_width > 0.0).map(|c| (c, spec.stroke_width)),
        layout_box,
        lines: lines.len(),
        overflow,
        size_px: size,
        primary,
        found,
        synthetic,
        fallbacks,
        missing,
        glyphs: glyph_count,
        path_length,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bidi_reorders_rtl_runs() {
        // pure LTR is untouched
        assert_eq!(visual_order("abc def"), "abc def");
        // Hebrew inside an English sentence is reversed in place
        assert_eq!(visual_order("abc \u{5D0}\u{5D1}\u{5D2} def"), "abc \u{5D2}\u{5D1}\u{5D0} def");
        // an RTL paragraph puts the later English run on the left
        assert_eq!(visual_order("\u{5D0}\u{5D1}\u{5D2} def"), "def \u{5D2}\u{5D1}\u{5D0}");
        // numbers keep their own order inside Arabic
        let ar = "\u{633}\u{644}\u{627}\u{645} 123";
        assert_eq!(visual_order(ar), "123 \u{645}\u{627}\u{644}\u{633}");
        let runs = visual_runs(ar);
        assert_eq!(runs.len(), 2);
        assert_eq!(runs[0], ("123".to_string(), false));
        assert!(runs[1].1);
    }

    #[test]
    fn polyline_measures_and_orients() {
        let p = Polyline::from_svg("M 0 0 L 100 0 L 100 100 L 0 100 Z").unwrap();
        assert!(p.closed);
        assert!((p.length() - 400.0).abs() < 1e-9);
        assert!(p.signed_area() > 0.0); // clockwise on screen
        assert_eq!(p.at(150.0), Point::new(100.0, 50.0));
        assert_eq!(p.at(450.0), Point::new(50.0, 0.0)); // wraps
        let inside = Polyline::from_svg("M 0 0 L 100 0 L 100 100 L 0 100 Z").unwrap().oriented(Side::Inside);
        assert!(inside.signed_area() < 0.0);
        let open = Polyline::from_svg("M 0 0 L 10 0").unwrap();
        assert!(!open.closed);
        assert_eq!(open.at(15.0), Point::new(15.0, 0.0)); // extends past the end
        assert!(Polyline::from_svg("M 5 5").is_err());
        assert!(Polyline::from_svg("bogus").is_err());
    }

    #[test]
    fn wrap_breaks_at_opportunities() {
        // fake measure: every byte is a 10 px cluster
        let text = "aaa bbb ccc";
        let n = text.len();
        let m = Measure {
            pre_adv: (0..=n).map(|i| i as f64 * 10.0).collect(),
            pre_cl: (0..=n).collect(),
            bounds: (1..=n).collect(),
            track: 0.0,
        };
        let lines = wrap(text, &m, Some(65.0));
        let words: Vec<&str> = lines.iter().map(|&(a, b, _)| text[a..trim_end(text, a, b)].trim()).collect();
        assert_eq!(words, ["aaa", "bbb", "ccc"]);
        assert!(lines.last().unwrap().2);
        // a word longer than the box breaks between clusters
        let lines = wrap("aaaaaaaaaa", &Measure { bounds: (1..=10).collect(), ..m }, Some(35.0));
        assert_eq!(lines.len(), 4);
    }
}
