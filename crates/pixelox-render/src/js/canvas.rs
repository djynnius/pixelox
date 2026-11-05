//! A Canvas2D context implemented over tiny-skia. Paths are kept in device space (points
//! are transformed when they are added, as the HTML spec requires) and mapped back through
//! the inverse transform when drawn, so strokes keep their user-space width.

use pixelox_core::color::{parse_color, Rgba};
use std::f32::consts::{FRAC_PI_2, PI, TAU};
use tiny_skia::{
    BlendMode, Color, FillRule, FilterQuality, GradientStop, LineCap, LineJoin, LinearGradient, Mask, Paint,
    PathBuilder, Pattern, Pixmap, Point, RadialGradient, Rect, SpreadMode, Stroke, StrokeDash, Transform,
};

/// A gradient created by `createLinearGradient` / `createRadialGradient`.
#[derive(Clone, Debug)]
pub struct Gradient {
    pub radial: bool,
    pub p: [f32; 6],
    pub stops: Vec<(f32, Rgba)>,
}

#[derive(Clone, Debug)]
enum Style {
    Color(Rgba),
    Gradient(usize),
}

#[derive(Clone)]
struct DrawState {
    ctm: Transform,
    fill: Style,
    stroke: Style,
    line_width: f32,
    cap: LineCap,
    join: LineJoin,
    miter: f32,
    alpha: f32,
    blend: BlendMode,
    dash: Vec<f32>,
    dash_offset: f32,
    clip: Option<Mask>,
}

impl Default for DrawState {
    fn default() -> Self {
        DrawState {
            ctm: Transform::identity(),
            fill: Style::Color(Rgba::BLACK),
            stroke: Style::Color(Rgba::BLACK),
            line_width: 1.0,
            cap: LineCap::Butt,
            join: LineJoin::Miter,
            miter: 10.0,
            alpha: 1.0,
            blend: BlendMode::SourceOver,
            dash: Vec::new(),
            dash_offset: 0.0,
            clip: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Seg {
    Move(f32, f32),
    Line(f32, f32),
    Quad(f32, f32, f32, f32),
    Cubic(f32, f32, f32, f32, f32, f32),
    Close,
}

/// Opcodes shared with `prelude.js` (`OP`).
pub mod op {
    pub const FILL_RECT: u32 = 1;
    pub const STROKE_RECT: u32 = 2;
    pub const CLEAR_RECT: u32 = 3;
    pub const BEGIN_PATH: u32 = 4;
    pub const CLOSE_PATH: u32 = 5;
    pub const MOVE_TO: u32 = 6;
    pub const LINE_TO: u32 = 7;
    pub const BEZIER_TO: u32 = 8;
    pub const QUAD_TO: u32 = 9;
    pub const ARC: u32 = 10;
    pub const ARC_TO: u32 = 11;
    pub const ELLIPSE: u32 = 12;
    pub const RECT: u32 = 13;
    pub const FILL: u32 = 14;
    pub const STROKE: u32 = 15;
    pub const CLIP: u32 = 16;
    pub const SAVE: u32 = 17;
    pub const RESTORE: u32 = 18;
    pub const TRANSLATE: u32 = 19;
    pub const ROTATE: u32 = 20;
    pub const SCALE: u32 = 21;
    pub const TRANSFORM: u32 = 22;
    pub const SET_TRANSFORM: u32 = 23;
    pub const RESET_TRANSFORM: u32 = 24;
    pub const LINE_WIDTH: u32 = 25;
    pub const GLOBAL_ALPHA: u32 = 26;
    pub const MITER_LIMIT: u32 = 27;
    pub const DASH_OFFSET: u32 = 28;
    pub const ROUND_RECT: u32 = 29;
}

/// One `px.canvas()` context.
pub struct Canvas {
    pub pixmap: Pixmap,
    st: DrawState,
    stack: Vec<DrawState>,
    path: Vec<Seg>,
    /// Current point and subpath start, in device space.
    cur: Option<(f32, f32)>,
    start: (f32, f32),
}

fn finite(a: &[f64]) -> bool {
    a.iter().all(|v| v.is_finite())
}

impl Canvas {
    pub fn new(w: u32, h: u32) -> Option<Canvas> {
        Some(Canvas {
            pixmap: Pixmap::new(w, h)?,
            st: DrawState::default(),
            stack: Vec::new(),
            path: Vec::new(),
            cur: None,
            start: (0.0, 0.0),
        })
    }

    pub fn transform(&self) -> [f64; 6] {
        let t = self.st.ctm;
        [t.sx as f64, t.ky as f64, t.kx as f64, t.sy as f64, t.tx as f64, t.ty as f64]
    }

    fn dev(&self, x: f32, y: f32) -> (f32, f32) {
        let mut p = Point::from_xy(x, y);
        self.st.ctm.map_point(&mut p);
        (p.x, p.y)
    }

    fn ensure(&mut self, x: f32, y: f32) {
        if self.cur.is_none() {
            self.move_to(x, y);
        }
    }

    fn move_to(&mut self, x: f32, y: f32) {
        let d = self.dev(x, y);
        self.path.push(Seg::Move(d.0, d.1));
        self.cur = Some(d);
        self.start = d;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        if self.cur.is_none() {
            return self.move_to(x, y);
        }
        let d = self.dev(x, y);
        self.path.push(Seg::Line(d.0, d.1));
        self.cur = Some(d);
    }

    fn cubic_to(&mut self, c: [f32; 6]) {
        self.ensure(c[0], c[1]);
        let a = self.dev(c[0], c[1]);
        let b = self.dev(c[2], c[3]);
        let e = self.dev(c[4], c[5]);
        self.path.push(Seg::Cubic(a.0, a.1, b.0, b.1, e.0, e.1));
        self.cur = Some(e);
    }

    fn quad_to(&mut self, c: [f32; 4]) {
        self.ensure(c[0], c[1]);
        let a = self.dev(c[0], c[1]);
        let e = self.dev(c[2], c[3]);
        self.path.push(Seg::Quad(a.0, a.1, e.0, e.1));
        self.cur = Some(e);
    }

    fn close(&mut self) {
        if self.cur.is_some() {
            self.path.push(Seg::Close);
            self.cur = Some(self.start);
        }
    }

    /// Elliptical arc in user space from angle `a0` sweeping `sweep` radians.
    fn arc_sweep(&mut self, cx: f32, cy: f32, rx: f32, ry: f32, rot: f32, a0: f32, sweep: f32) {
        let (sr, cr) = rot.sin_cos();
        let map = |ux: f32, uy: f32| -> (f32, f32) {
            let x = ux * rx;
            let y = uy * ry;
            (cx + x * cr - y * sr, cy + x * sr + y * cr)
        };
        let start = map(a0.cos(), a0.sin());
        if self.cur.is_some() {
            self.line_to(start.0, start.1);
        } else {
            self.move_to(start.0, start.1);
        }
        if sweep == 0.0 || (rx == 0.0 && ry == 0.0) {
            return;
        }
        let n = (sweep.abs() / FRAC_PI_2).ceil().max(1.0) as usize;
        let d = sweep / n as f32;
        let k = 4.0 / 3.0 * (d / 4.0).tan();
        let mut t = a0;
        for _ in 0..n {
            let (s0, c0) = t.sin_cos();
            let (s1, c1) = (t + d).sin_cos();
            let p1 = map(c0 - k * s0, s0 + k * c0);
            let p2 = map(c1 + k * s1, s1 - k * c1);
            let p3 = map(c1, s1);
            self.cubic_to([p1.0, p1.1, p2.0, p2.1, p3.0, p3.1]);
            t += d;
        }
    }

    fn arc(&mut self, a: &[f64]) {
        let (cx, cy, rx, ry, rot, a0, a1, ccw) =
            (a[0] as f32, a[1] as f32, a[2] as f32, a[3] as f32, a[4] as f32, a[5] as f32, a[6] as f32, a[7] != 0.0);
        let sweep = if !ccw {
            if a1 - a0 >= TAU {
                TAU
            } else {
                (a1 - a0).rem_euclid(TAU)
            }
        } else if a0 - a1 >= TAU {
            -TAU
        } else {
            -((a0 - a1).rem_euclid(TAU))
        };
        self.arc_sweep(cx, cy, rx, ry, rot, a0, sweep);
    }

    fn arc_to(&mut self, a: &[f64]) {
        let (x1, y1, x2, y2, r) = (a[0] as f32, a[1] as f32, a[2] as f32, a[3] as f32, a[4] as f32);
        let Some(cur) = self.cur else {
            return self.move_to(x1, y1);
        };
        let Some(inv) = self.st.ctm.invert() else { return };
        let mut p0 = Point::from_xy(cur.0, cur.1);
        inv.map_point(&mut p0);
        let (v1x, v1y) = (p0.x - x1, p0.y - y1);
        let (v2x, v2y) = (x2 - x1, y2 - y1);
        let l1 = (v1x * v1x + v1y * v1y).sqrt();
        let l2 = (v2x * v2x + v2y * v2y).sqrt();
        let cross = v1x * v2y - v1y * v2x;
        if l1 < 1e-6 || l2 < 1e-6 || r == 0.0 || cross.abs() < 1e-6 * l1 * l2 {
            return self.line_to(x1, y1);
        }
        let (u1x, u1y, u2x, u2y) = (v1x / l1, v1y / l1, v2x / l2, v2y / l2);
        let angle = (u1x * u2x + u1y * u2y).clamp(-1.0, 1.0).acos();
        let dist = r / (angle / 2.0).tan();
        let t1 = (x1 + u1x * dist, y1 + u1y * dist);
        let t2 = (x1 + u2x * dist, y1 + u2y * dist);
        let (bx, by) = (u1x + u2x, u1y + u2y);
        let bl = (bx * bx + by * by).sqrt();
        let cd = r / (angle / 2.0).sin();
        let c = (x1 + bx / bl * cd, y1 + by / bl * cd);
        let s = (t1.1 - c.1).atan2(t1.0 - c.0);
        let e = (t2.1 - c.1).atan2(t2.0 - c.0);
        let mut sweep = e - s;
        while sweep > PI {
            sweep -= TAU;
        }
        while sweep < -PI {
            sweep += TAU;
        }
        self.line_to(t1.0, t1.1);
        self.arc_sweep(c.0, c.1, r, r, 0.0, s, sweep);
    }

    fn rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.move_to(x, y);
        self.line_to(x + w, y);
        self.line_to(x + w, y + h);
        self.line_to(x, y + h);
        self.close();
        self.move_to(x, y);
    }

    fn round_rect(&mut self, a: &[f64]) {
        let (mut x, mut y, mut w, mut h) = (a[0] as f32, a[1] as f32, a[2] as f32, a[3] as f32);
        if w < 0.0 {
            x += w;
            w = -w;
        }
        if h < 0.0 {
            y += h;
            h = -h;
        }
        let mut r = [a[4] as f32, a[5] as f32, a[6] as f32, a[7] as f32].map(|v| v.max(0.0));
        let scale = [w / (r[0] + r[1]), h / (r[1] + r[2]), w / (r[2] + r[3]), h / (r[3] + r[0])]
            .into_iter()
            .filter(|s| s.is_finite())
            .fold(1.0f32, f32::min);
        for v in r.iter_mut() {
            *v *= scale;
        }
        let [tl, tr, br, bl] = r;
        self.move_to(x + tl, y);
        self.line_to(x + w - tr, y);
        if tr > 0.0 {
            self.arc_sweep(x + w - tr, y + tr, tr, tr, 0.0, -FRAC_PI_2, FRAC_PI_2);
        }
        self.line_to(x + w, y + h - br);
        if br > 0.0 {
            self.arc_sweep(x + w - br, y + h - br, br, br, 0.0, 0.0, FRAC_PI_2);
        }
        self.line_to(x + bl, y + h);
        if bl > 0.0 {
            self.arc_sweep(x + bl, y + h - bl, bl, bl, 0.0, FRAC_PI_2, FRAC_PI_2);
        }
        self.line_to(x, y + tl);
        if tl > 0.0 {
            self.arc_sweep(x + tl, y + tl, tl, tl, 0.0, PI, FRAC_PI_2);
        }
        self.close();
        self.move_to(x, y);
    }

    /// The current path in device space.
    fn device_path(&self) -> Option<tiny_skia::Path> {
        let mut pb = PathBuilder::new();
        for s in &self.path {
            match *s {
                Seg::Move(x, y) => pb.move_to(x, y),
                Seg::Line(x, y) => pb.line_to(x, y),
                Seg::Quad(a, b, x, y) => pb.quad_to(a, b, x, y),
                Seg::Cubic(a, b, c, d, x, y) => pb.cubic_to(a, b, c, d, x, y),
                Seg::Close => pb.close(),
            }
        }
        pb.finish()
    }

    /// The current path mapped back into user space (drawn with the CTM).
    fn user_path(&self) -> Option<tiny_skia::Path> {
        let inv = self.st.ctm.invert()?;
        self.device_path()?.transform(inv)
    }

    fn paint<'a>(&self, style: &Style, grads: &[Gradient]) -> Option<Paint<'a>> {
        let mut p = Paint { anti_alias: true, blend_mode: self.st.blend, ..Default::default() };
        let alpha = self.st.alpha;
        match style {
            Style::Color(c) => {
                p.set_color(Color::from_rgba(c.r, c.g, c.b, (c.a * alpha).clamp(0.0, 1.0))?);
            }
            Style::Gradient(i) => {
                let g = grads.get(*i)?;
                if g.stops.is_empty() {
                    return None;
                }
                let mut stops = g.stops.clone();
                let [x0, y0, r0, x1, y1, r1] = g.p;
                if g.radial && r0 > 0.0 && r1 > r0 {
                    // tiny-skia's two-point conical gradient starts at radius 0: remap stops
                    let k = r0 / r1;
                    for s in stops.iter_mut() {
                        s.0 = k + s.0 * (1.0 - k);
                    }
                    stops.insert(0, (0.0, stops[0].1));
                }
                let gs: Vec<GradientStop> = stops
                    .iter()
                    .filter_map(|(o, c)| {
                        Color::from_rgba(c.r, c.g, c.b, (c.a * alpha).clamp(0.0, 1.0))
                            .map(|col| GradientStop::new(*o, col))
                    })
                    .collect();
                let shader = if g.radial {
                    RadialGradient::new(
                        Point::from_xy(x0, y0),
                        Point::from_xy(x1, y1),
                        r1,
                        gs,
                        SpreadMode::Pad,
                        Transform::identity(),
                    )?
                } else {
                    LinearGradient::new(
                        Point::from_xy(x0, y0),
                        Point::from_xy(x1, y1),
                        gs,
                        SpreadMode::Pad,
                        Transform::identity(),
                    )?
                };
                p.shader = shader;
            }
        }
        Some(p)
    }

    fn stroke_spec(&self) -> Stroke {
        let dash =
            if self.st.dash.len() >= 2 { StrokeDash::new(self.st.dash.clone(), self.st.dash_offset) } else { None };
        Stroke {
            width: self.st.line_width,
            miter_limit: self.st.miter,
            line_cap: self.st.cap,
            line_join: self.st.join,
            dash,
        }
    }

    fn fill_current(&mut self, rule: FillRule, grads: &[Gradient]) {
        let Some(path) = self.user_path() else { return };
        let Some(paint) = self.paint(&self.st.fill, grads) else { return };
        let ctm = self.st.ctm;
        self.pixmap.fill_path(&path, &paint, rule, ctm, self.st.clip.as_ref());
    }

    fn stroke_current(&mut self, grads: &[Gradient]) {
        let Some(path) = self.user_path() else { return };
        let Some(paint) = self.paint(&self.st.stroke, grads) else { return };
        let stroke = self.stroke_spec();
        let ctm = self.st.ctm;
        self.pixmap.stroke_path(&path, &paint, &stroke, ctm, self.st.clip.as_ref());
    }

    fn clip_current(&mut self, rule: FillRule) {
        let (w, h) = (self.pixmap.width(), self.pixmap.height());
        let path = self.device_path();
        let mut mask = match self.st.clip.take() {
            Some(m) => m,
            None => {
                let Some(mut m) = Mask::new(w, h) else { return };
                if let Some(p) = &path {
                    m.fill_path(p, rule, true, Transform::identity());
                }
                self.st.clip = Some(m);
                return;
            }
        };
        match &path {
            Some(p) => mask.intersect_path(p, rule, true, Transform::identity()),
            None => mask.clear(),
        }
        self.st.clip = Some(mask);
    }

    fn norm_rect(a: &[f64]) -> Option<Rect> {
        let (mut x, mut y, mut w, mut h) = (a[0] as f32, a[1] as f32, a[2] as f32, a[3] as f32);
        if w < 0.0 {
            x += w;
            w = -w;
        }
        if h < 0.0 {
            y += h;
            h = -h;
        }
        Rect::from_xywh(x, y, w, h)
    }

    fn fill_rect(&mut self, a: &[f64], grads: &[Gradient]) {
        let Some(r) = Self::norm_rect(a) else { return };
        let Some(paint) = self.paint(&self.st.fill, grads) else { return };
        let ctm = self.st.ctm;
        self.pixmap.fill_rect(r, &paint, ctm, self.st.clip.as_ref());
    }

    fn stroke_rect(&mut self, a: &[f64], grads: &[Gradient]) {
        let (x, y, w, h) = (a[0] as f32, a[1] as f32, a[2] as f32, a[3] as f32);
        let mut pb = PathBuilder::new();
        pb.move_to(x, y);
        pb.line_to(x + w, y);
        pb.line_to(x + w, y + h);
        pb.line_to(x, y + h);
        pb.close();
        let Some(path) = pb.finish() else { return };
        let Some(paint) = self.paint(&self.st.stroke, grads) else { return };
        let stroke = self.stroke_spec();
        let ctm = self.st.ctm;
        self.pixmap.stroke_path(&path, &paint, &stroke, ctm, self.st.clip.as_ref());
    }

    fn clear_rect(&mut self, a: &[f64]) {
        let Some(r) = Self::norm_rect(a) else { return };
        let mut paint = Paint { blend_mode: BlendMode::Clear, anti_alias: true, ..Default::default() };
        paint.set_color(Color::BLACK);
        let ctm = self.st.ctm;
        self.pixmap.fill_rect(r, &paint, ctm, self.st.clip.as_ref());
    }

    /// Run one numeric opcode. Returns false for an unknown opcode.
    pub fn op(&mut self, code: u32, a: &[f64], grads: &[Gradient]) -> bool {
        let need = |n: usize| a.len() >= n && finite(&a[..n]);
        let f = |i: usize| a[i] as f32;
        match code {
            op::FILL_RECT if need(4) => self.fill_rect(a, grads),
            op::STROKE_RECT if need(4) => self.stroke_rect(a, grads),
            op::CLEAR_RECT if need(4) => self.clear_rect(a),
            op::BEGIN_PATH => {
                self.path.clear();
                self.cur = None;
            }
            op::CLOSE_PATH => self.close(),
            op::MOVE_TO if need(2) => self.move_to(f(0), f(1)),
            op::LINE_TO if need(2) => self.line_to(f(0), f(1)),
            op::BEZIER_TO if need(6) => self.cubic_to([f(0), f(1), f(2), f(3), f(4), f(5)]),
            op::QUAD_TO if need(4) => self.quad_to([f(0), f(1), f(2), f(3)]),
            op::ARC if need(6) => self.arc(&[a[0], a[1], a[2], a[2], 0.0, a[3], a[4], a[5]]),
            op::ARC_TO if need(5) => self.arc_to(a),
            op::ELLIPSE if need(8) => self.arc(a),
            op::RECT if need(4) => self.rect(f(0), f(1), f(2), f(3)),
            op::ROUND_RECT if need(8) => self.round_rect(a),
            op::FILL => {
                let rule = if a.first() == Some(&1.0) { FillRule::EvenOdd } else { FillRule::Winding };
                self.fill_current(rule, grads);
            }
            op::STROKE => self.stroke_current(grads),
            op::CLIP => {
                let rule = if a.first() == Some(&1.0) { FillRule::EvenOdd } else { FillRule::Winding };
                self.clip_current(rule);
            }
            op::SAVE => self.stack.push(self.st.clone()),
            op::RESTORE => {
                if let Some(s) = self.stack.pop() {
                    self.st = s;
                }
            }
            op::TRANSLATE if need(2) => self.st.ctm = self.st.ctm.pre_translate(f(0), f(1)),
            op::ROTATE if need(1) => self.st.ctm = self.st.ctm.pre_concat(Transform::from_rotate(f(0).to_degrees())),
            op::SCALE if need(2) => self.st.ctm = self.st.ctm.pre_scale(f(0), f(1)),
            op::TRANSFORM if need(6) => {
                self.st.ctm = self.st.ctm.pre_concat(Transform::from_row(f(0), f(1), f(2), f(3), f(4), f(5)))
            }
            op::SET_TRANSFORM if need(6) => self.st.ctm = Transform::from_row(f(0), f(1), f(2), f(3), f(4), f(5)),
            op::RESET_TRANSFORM => self.st.ctm = Transform::identity(),
            op::LINE_WIDTH if need(1) && a[0] > 0.0 => self.st.line_width = f(0),
            op::GLOBAL_ALPHA if need(1) && (0.0..=1.0).contains(&a[0]) => self.st.alpha = f(0),
            op::MITER_LIMIT if need(1) && a[0] > 0.0 => self.st.miter = f(0),
            op::DASH_OFFSET if need(1) => self.st.dash_offset = f(0),
            1..=29 => {} // non-finite or missing arguments: ignored, as in browsers
            _ => return false,
        }
        true
    }

    /// `fillStyle` (which = 0) or `strokeStyle` (1) from a CSS color; false when invalid.
    pub fn set_color(&mut self, which: u32, css: &str) -> bool {
        let Ok(c) = parse_color(css) else { return false };
        if which == 0 {
            self.st.fill = Style::Color(c);
        } else {
            self.st.stroke = Style::Color(c);
        }
        true
    }

    pub fn set_gradient(&mut self, which: u32, gid: usize) {
        if which == 0 {
            self.st.fill = Style::Gradient(gid);
        } else {
            self.st.stroke = Style::Gradient(gid);
        }
    }

    /// `lineCap` (1), `lineJoin` (2), `globalCompositeOperation` (3).
    pub fn set_str(&mut self, prop: u32, v: &str) -> bool {
        match prop {
            1 => {
                self.st.cap = match v {
                    "butt" => LineCap::Butt,
                    "round" => LineCap::Round,
                    "square" => LineCap::Square,
                    _ => return false,
                }
            }
            2 => {
                self.st.join = match v {
                    "miter" => LineJoin::Miter,
                    "round" => LineJoin::Round,
                    "bevel" => LineJoin::Bevel,
                    _ => return false,
                }
            }
            3 => {
                self.st.blend = match blend_mode(v) {
                    Some(b) => b,
                    None => return false,
                }
            }
            _ => return false,
        }
        true
    }

    pub fn set_dash(&mut self, dash: &[f64]) {
        if dash.iter().all(|v| v.is_finite() && *v >= 0.0) {
            let mut d: Vec<f32> = dash.iter().map(|v| *v as f32).collect();
            if d.len() % 2 == 1 {
                d.extend(d.clone());
            }
            self.st.dash = d;
        }
    }

    /// `drawImage(src, ...)` with 2, 4, or 8 numeric arguments.
    pub fn draw_canvas(&mut self, src: &Pixmap, a: &[f64]) {
        if !finite(a) {
            return;
        }
        let (sw0, sh0) = (src.width() as f32, src.height() as f32);
        let f = |i: usize| a[i] as f32;
        let (sx, sy, sw, sh, dx, dy, dw, dh) = match a.len() {
            2 => (0.0, 0.0, sw0, sh0, f(0), f(1), sw0, sh0),
            4 => (0.0, 0.0, sw0, sh0, f(0), f(1), f(2), f(3)),
            8 => (f(0), f(1), f(2), f(3), f(4), f(5), f(6), f(7)),
            _ => return,
        };
        if sw == 0.0 || sh == 0.0 {
            return;
        }
        let Some(r) = Self::norm_rect(&[dx as f64, dy as f64, dw as f64, dh as f64]) else { return };
        let ts = Transform::from_translate(-sx, -sy).post_scale(dw / sw, dh / sh).post_translate(dx, dy);
        let paint = Paint {
            shader: Pattern::new(src.as_ref(), SpreadMode::Pad, FilterQuality::Bilinear, self.st.alpha, ts),
            blend_mode: self.st.blend,
            anti_alias: true,
            ..Default::default()
        };
        let ctm = self.st.ctm;
        self.pixmap.fill_rect(r, &paint, ctm, self.st.clip.as_ref());
    }

    /// Straight-alpha RGBA8 of a rectangle (pixels outside the canvas are transparent).
    pub fn get_image(&self, x: i32, y: i32, w: u32, h: u32) -> Vec<u8> {
        let mut out = vec![0u8; w as usize * h as usize * 4];
        let (cw, ch) = (self.pixmap.width() as i32, self.pixmap.height() as i32);
        let px = self.pixmap.pixels();
        for j in 0..h as i32 {
            let sy = y + j;
            if sy < 0 || sy >= ch {
                continue;
            }
            for i in 0..w as i32 {
                let sx = x + i;
                if sx < 0 || sx >= cw {
                    continue;
                }
                let c = px[(sy * cw + sx) as usize].demultiply();
                let o = ((j * w as i32 + i) * 4) as usize;
                out[o..o + 4].copy_from_slice(&[c.red(), c.green(), c.blue(), c.alpha()]);
            }
        }
        out
    }

    /// Write straight-alpha RGBA8 at (dx, dy), ignoring transform, alpha, and clip.
    pub fn put_image(&mut self, data: &[u8], w: u32, h: u32, dx: i32, dy: i32) {
        let (cw, ch) = (self.pixmap.width() as i32, self.pixmap.height() as i32);
        let buf = self.pixmap.data_mut();
        for j in 0..h as i32 {
            let ty = dy + j;
            if ty < 0 || ty >= ch {
                continue;
            }
            for i in 0..w as i32 {
                let tx = dx + i;
                if tx < 0 || tx >= cw {
                    continue;
                }
                let s = ((j * w as i32 + i) * 4) as usize;
                if s + 4 > data.len() {
                    return;
                }
                let a = data[s + 3] as u32;
                let pm = |v: u8| ((v as u32 * a + 127) / 255) as u8;
                let o = ((ty * cw + tx) * 4) as usize;
                buf[o] = pm(data[s]);
                buf[o + 1] = pm(data[s + 1]);
                buf[o + 2] = pm(data[s + 2]);
                buf[o + 3] = a as u8;
            }
        }
    }
}

fn blend_mode(v: &str) -> Option<BlendMode> {
    Some(match v {
        "source-over" => BlendMode::SourceOver,
        "source-in" => BlendMode::SourceIn,
        "source-out" => BlendMode::SourceOut,
        "source-atop" => BlendMode::SourceAtop,
        "destination-over" => BlendMode::DestinationOver,
        "destination-in" => BlendMode::DestinationIn,
        "destination-out" => BlendMode::DestinationOut,
        "destination-atop" => BlendMode::DestinationAtop,
        "lighter" => BlendMode::Plus,
        "copy" => BlendMode::Source,
        "xor" => BlendMode::Xor,
        "multiply" => BlendMode::Multiply,
        "screen" => BlendMode::Screen,
        "overlay" => BlendMode::Overlay,
        "darken" => BlendMode::Darken,
        "lighten" => BlendMode::Lighten,
        "color-dodge" => BlendMode::ColorDodge,
        "color-burn" => BlendMode::ColorBurn,
        "hard-light" => BlendMode::HardLight,
        "soft-light" => BlendMode::SoftLight,
        "difference" => BlendMode::Difference,
        "exclusion" => BlendMode::Exclusion,
        "hue" => BlendMode::Hue,
        "saturation" => BlendMode::Saturation,
        "color" => BlendMode::Color,
        "luminosity" => BlendMode::Luminosity,
        _ => return None,
    })
}

/// A gradient from `createLinearGradient` (radial = false: x0, y0, _, x1, y1, _) or
/// `createRadialGradient` (x0, y0, r0, x1, y1, r1).
pub fn new_gradient(radial: bool, a: &[f64]) -> Gradient {
    let mut p = [0f32; 6];
    for (i, v) in a.iter().take(6).enumerate() {
        p[i] = if v.is_finite() { *v as f32 } else { 0.0 };
    }
    Gradient { radial, p, stops: Vec::new() }
}

impl Gradient {
    pub fn add_stop(&mut self, offset: f64, css: &str) -> bool {
        let Ok(c) = parse_color(css) else { return false };
        let o = offset.clamp(0.0, 1.0) as f32;
        // keep stops sorted; equal offsets keep insertion order (hard stops)
        let at = self.stops.iter().position(|s| s.0 > o).unwrap_or(self.stops.len());
        self.stops.insert(at, (o, c));
        true
    }
}

/// tiny-skia pixmap (premultiplied RGBA8) → pixelox image (straight f32).
pub fn pixmap_to_image(p: &Pixmap) -> pixelox_core::Image {
    let mut bytes = Vec::with_capacity(p.data().len());
    for c in p.pixels() {
        let d = c.demultiply();
        bytes.extend_from_slice(&[d.red(), d.green(), d.blue(), d.alpha()]);
    }
    pixelox_core::Image::from_rgba8(p.width(), p.height(), &bytes)
}

/// pixelox image (straight f32) → tiny-skia pixmap (premultiplied RGBA8).
pub fn image_to_pixmap(img: &pixelox_core::Image) -> Option<Pixmap> {
    let mut p = Pixmap::new(img.width, img.height)?;
    let src = img.to_rgba8();
    for (dst, s) in p.pixels_mut().iter_mut().zip(src.chunks_exact(4)) {
        *dst = tiny_skia::ColorU8::from_rgba(s[0], s[1], s[2], s[3]).premultiply();
    }
    Some(p)
}
