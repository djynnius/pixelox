//! The text layer spec (`LayerKind::Text { spec }`): everything needed to re-render a type
//! layer, stored as JSON in the document. Reading is lenient (missing or malformed keys take
//! their defaults) because other crates and older documents write parts of it; writing keeps
//! keys this crate does not own.

use crate::fonts::FontStyle;
use pixelox_core::prelude::*;
use pixelox_core::units::{Length, SizeSpec, ANCHORS};
use pixelox_core::ParamSpec;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Align {
    Left,
    Center,
    Right,
    Justify,
}

pub const ALIGNS: &[&str] = &["left", "center", "right", "justify"];

impl Align {
    pub fn parse(s: &str) -> Option<Align> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "left" | "start" => Align::Left,
            "center" | "centre" | "middle" => Align::Center,
            "right" | "end" => Align::Right,
            "justify" | "justified" => Align::Justify,
            _ => return None,
        })
    }
    pub fn name(self) -> &'static str {
        match self {
            Align::Left => "left",
            Align::Center => "center",
            Align::Right => "right",
            Align::Justify => "justify",
        }
    }
}

/// Which side of a path text sits on (`text on-path --side`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Outside,
    Inside,
}

impl Side {
    pub fn parse(s: &str) -> Option<Side> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "outside" | "out" | "above" => Side::Outside,
            "inside" | "in" | "below" => Side::Inside,
            _ => return None,
        })
    }
    pub fn name(self) -> &'static str {
        match self {
            Side::Outside => "outside",
            Side::Inside => "inside",
        }
    }
}

pub fn anchor_name(a: Anchor) -> &'static str {
    match a {
        Anchor::Tl => "tl",
        Anchor::T => "t",
        Anchor::Tr => "tr",
        Anchor::L => "l",
        Anchor::C => "c",
        Anchor::R => "r",
        Anchor::Bl => "bl",
        Anchor::B => "b",
        Anchor::Br => "br",
    }
}

/// A text layer's parameters (see the crate docs for the JSON form).
#[derive(Clone, Debug)]
pub struct TextSpec {
    pub content: String,
    /// Family as requested (`Inter`, `sans-serif`).
    pub font: String,
    /// SHA-256 of the resolved face's file, preferred when re-resolving.
    pub font_sha256: Option<String>,
    /// The resolved face (`family`, `style`, `weight`, `sha256`, `postscript`), informational.
    pub font_face: Option<Value>,
    /// CSS length string: `48pt`, `64px`, `5%` (of the canvas height).
    pub size: String,
    pub weight: u16,
    pub style: FontStyle,
    /// Variable-font axes, `wght=650,slnt=-5`.
    pub variation: Option<String>,
    pub color: Rgba,
    pub stroke: Option<Rgba>,
    /// Stroke width in canvas px.
    pub stroke_width: f64,
    pub align: Align,
    /// Letter spacing in 1/1000 em.
    pub tracking: f64,
    /// `auto` or a length (`%` of the font size).
    pub leading: String,
    pub kerning: bool,
    /// Anchor point in canvas px.
    pub x: f64,
    pub y: f64,
    pub anchor: Anchor,
    /// Paragraph box (`--box`): width and optional height in px.
    pub box_size: Option<(f64, Option<f64>)>,
    /// Degrees about the anchor point (clockwise).
    pub rotation: f64,
    /// SVG path data the text flows along.
    pub path: Option<String>,
    /// Start along the path: `25%` of its length or px.
    pub path_offset: String,
    pub path_side: Side,
}

impl Default for TextSpec {
    fn default() -> Self {
        TextSpec {
            content: String::new(),
            font: "sans-serif".into(),
            font_sha256: None,
            font_face: None,
            size: "48pt".into(),
            weight: 400,
            style: FontStyle::Normal,
            variation: None,
            color: Rgba::BLACK,
            stroke: None,
            stroke_width: 2.0,
            align: Align::Left,
            tracking: 0.0,
            leading: "auto".into(),
            kerning: true,
            x: 0.0,
            y: 0.0,
            anchor: Anchor::Tl,
            box_size: None,
            rotation: 0.0,
            path: None,
            path_offset: "0".into(),
            path_side: Side::Outside,
        }
    }
}

fn num(v: Option<&Value>) -> Option<f64> {
    v.and_then(|v| v.as_f64().or_else(|| v.as_str().and_then(|s| s.trim().parse().ok())))
        .filter(|v: &f64| v.is_finite())
}

fn text(v: Option<&Value>) -> Option<String> {
    match v {
        Some(Value::String(s)) => Some(s.clone()),
        Some(Value::Number(n)) => Some(n.to_string()),
        _ => None,
    }
}

impl TextSpec {
    /// Read a stored spec, defaulting anything missing or unreadable.
    pub fn from_value(v: &Value) -> TextSpec {
        let d = TextSpec::default();
        let g = |k: &str| v.get(k);
        let size = match g("size") {
            Some(Value::Number(n)) => format!("{n}px"),
            Some(Value::String(s)) if Length::parse(s).is_ok() => normalize_size(s),
            _ => d.size.clone(),
        };
        TextSpec {
            content: text(g("content")).unwrap_or_default(),
            font: text(g("font")).filter(|f| !f.trim().is_empty()).unwrap_or(d.font),
            font_sha256: text(g("font_sha256")).filter(|s| !s.is_empty()),
            font_face: g("font_face").filter(|f| f.is_object()).cloned(),
            size,
            weight: g("weight")
                .and_then(|w| w.as_u64().map(|n| n.to_string()).or_else(|| w.as_str().map(String::from)))
                .and_then(|w| parse_weight(&w).ok())
                .unwrap_or(d.weight),
            style: text(g("style")).and_then(|s| FontStyle::parse(&s)).unwrap_or(d.style),
            variation: text(g("variation")).filter(|s| parse_variations(s).map(|v| !v.is_empty()).unwrap_or(false)),
            color: text(g("color")).and_then(|c| color::parse_color(&c).ok()).unwrap_or(d.color),
            stroke: text(g("stroke")).and_then(|c| color::parse_color(&c).ok()).filter(|c| c.a > 0.0),
            stroke_width: num(g("stroke_width")).map(|w| w.max(0.0)).unwrap_or(d.stroke_width),
            align: text(g("align")).and_then(|a| Align::parse(&a)).unwrap_or(d.align),
            tracking: num(g("tracking")).unwrap_or(0.0),
            leading: text(g("leading")).filter(|l| valid_leading(l)).unwrap_or(d.leading),
            kerning: !matches!(text(g("kerning")).as_deref(), Some("none" | "off" | "false")),
            x: num(g("x")).unwrap_or(0.0),
            y: num(g("y")).unwrap_or(0.0),
            anchor: text(g("anchor")).and_then(|a| Anchor::parse(&a).ok()).unwrap_or(d.anchor),
            box_size: match g("box") {
                Some(Value::Array(a)) if !a.is_empty() => {
                    num(a.first()).filter(|w| *w > 0.0).map(|w| (w, num(a.get(1)).filter(|h| *h > 0.0)))
                }
                _ => None,
            },
            rotation: num(g("rotation")).unwrap_or(0.0),
            path: text(g("path")).filter(|p| !p.trim().is_empty()),
            path_offset: text(g("path_offset")).filter(|o| Length::parse(o).is_ok()).unwrap_or(d.path_offset),
            path_side: text(g("path_side")).and_then(|s| Side::parse(&s)).unwrap_or(d.path_side),
        }
    }

    /// Write the spec, keeping keys of `base` that this crate does not own.
    pub fn to_value(&self, base: Option<&Value>) -> Value {
        let mut m = match base {
            Some(Value::Object(o)) => o.clone(),
            _ => Map::new(),
        };
        let mut set = |k: &str, v: Value| {
            m.insert(k.to_string(), v);
        };
        set("content", json!(self.content));
        set("font", json!(self.font));
        set("font_sha256", json!(self.font_sha256));
        set("font_face", self.font_face.clone().unwrap_or(Value::Null));
        set("size", json!(self.size));
        set("weight", json!(self.weight));
        set("style", json!(self.style.name()));
        set("variation", json!(self.variation));
        set("color", json!(self.color.to_hex()));
        set("stroke", json!(self.stroke.map(|c| c.to_hex())));
        set("stroke_width", json!(self.stroke_width));
        set("align", json!(self.align.name()));
        set("tracking", json!(self.tracking));
        set("leading", json!(self.leading));
        set("kerning", json!(if self.kerning { "metrics" } else { "none" }));
        set("x", json!(self.x));
        set("y", json!(self.y));
        set("anchor", json!(anchor_name(self.anchor)));
        set("box", self.box_size.map(|(w, h)| json!([w, h])).unwrap_or(Value::Null));
        set("rotation", json!(self.rotation));
        set("path", json!(self.path));
        set("path_offset", json!(self.path_offset));
        set("path_side", json!(self.path_side.name()));
        Value::Object(m)
    }

    /// Font size in canvas px (`pt` through the document DPI, `%` of the canvas height).
    pub fn size_px(&self, doc: &Document) -> f64 {
        Length::parse(&self.size).map(|l| l.to_px(doc.height as f64, doc.dpi())).unwrap_or(48.0).clamp(0.0, 20_000.0)
    }

    /// Line distance in px, or `None` for the font's own line height.
    pub fn leading_px(&self, size_px: f64, dpi: f64) -> Option<f64> {
        if self.leading.trim() == "auto" {
            return None;
        }
        Length::parse(&self.leading).ok().map(|l| l.to_px(size_px, dpi))
    }
}

fn valid_leading(s: &str) -> bool {
    s.trim() == "auto" || Length::parse(s).is_ok()
}

/// A bare number means px: `64` → `64px`.
pub fn normalize_size(s: &str) -> String {
    let t = s.trim();
    if t.parse::<f64>().is_ok() {
        format!("{t}px")
    } else {
        t.to_string()
    }
}

/// `100..900`, or a CSS name.
pub fn parse_weight(s: &str) -> Result<u16> {
    let t = s.trim().to_ascii_lowercase().replace(['-', '_', ' '], "");
    let named = match t.as_str() {
        "thin" | "hairline" => Some(100),
        "extralight" | "ultralight" => Some(200),
        "light" => Some(300),
        "normal" | "regular" | "book" => Some(400),
        "medium" => Some(500),
        "semibold" | "demibold" => Some(600),
        "bold" => Some(700),
        "extrabold" | "ultrabold" => Some(800),
        "black" | "heavy" => Some(900),
        _ => None,
    };
    if let Some(w) = named {
        return Ok(w);
    }
    match t.parse::<f64>() {
        Ok(w) if (1.0..=1000.0).contains(&w) => Ok(w.round() as u16),
        Ok(w) => Err(Error::new("PX1013", format!("--weight {w} is outside 1..1000")).with_context("weight", s)),
        Err(_) => Err(Error::usage(format!("--weight: '{s}' is not a weight (100..900, normal, bold, ...)"))
            .with_hint(
                "use a number like 700 or a name: thin, light, normal, medium, semibold, bold, extrabold, black",
            )),
    }
}

/// `wght=650,slnt=-5` → [("wght", 650), ("slnt", -5)].
pub fn parse_variations(s: &str) -> Result<Vec<(String, f32)>> {
    let mut out = Vec::new();
    for part in s.split([',', ' ']).map(str::trim).filter(|p| !p.is_empty()) {
        let bad = || {
            Error::usage(format!("--variation: '{part}' is not an axis setting (tag=value, e.g. wght=650)"))
                .with_context("variation", s)
        };
        let (tag, val) = part.split_once(['=', ':']).ok_or_else(bad)?;
        let tag = tag.trim().trim_matches(['\'', '"']);
        if tag.is_empty() || tag.len() > 4 || !tag.is_ascii() {
            return Err(bad());
        }
        let v: f32 = val.trim().parse().ok().filter(|v: &f32| v.is_finite()).ok_or_else(bad)?;
        out.push((format!("{tag:<4}"), v));
    }
    Ok(out)
}

/// Canonical text of a variation list.
pub fn format_variations(v: &[(String, f32)]) -> String {
    v.iter().map(|(t, x)| format!("{}={}", t.trim_end(), x)).collect::<Vec<_>>().join(",")
}

/// Style flags shared by `text add`, `text edit`, and `text style`. With `defaults`, each
/// flag carries the default of a new layer; without, only given flags change the layer.
pub fn style_params(defaults: bool) -> Vec<ParamSpec> {
    let d = |p: ParamSpec, v: &'static str| if defaults { p.default(v) } else { p };
    vec![
        d(
            param("font", ParamKind::Str).help(
                "Font family (case-insensitive) or a generic name: sans-serif, serif, monospace; see `pixelox fonts list`",
            ),
            "sans-serif",
        ),
        d(
            param("size", ParamKind::Length)
                .help("Font size: 48pt (points at the document DPI), 64px, a bare number of px, or % of the canvas height"),
            "48pt",
        ),
        d(
            param("weight", ParamKind::Str)
                .help("Weight 100..900 or a name (thin, light, normal, medium, semibold, bold, extrabold, black)"),
            "400",
        ),
        d(
            param("style", ParamKind::Choice)
                .choices(&["normal", "italic", "oblique"])
                .help("Upright or italic; a missing italic face is slanted synthetically"),
            "normal",
        ),
        param("variation", ParamKind::Str)
            .help("Variable-font axis values, e.g. 'wght=650,slnt=-5' (none clears them)"),
        d(param("color", ParamKind::Color).help("Fill color of the glyphs"), "#000000"),
        param("stroke", ParamKind::Color).help("Outline color, centered on the glyph edges (transparent removes it)"),
        d(param("stroke-width", ParamKind::Length).help("Outline width: px, pt, or % of the font size"), "2"),
        d(
            param("align", ParamKind::Choice)
                .choices(ALIGNS)
                .help("Line alignment inside the text box; justify stretches all but each paragraph's last line"),
            "left",
        ),
        d(
            param("tracking", ParamKind::Float)
                .range("-1000..10000")
                .help("Letter spacing in 1/1000 em (60 = 0.06 em); nonzero tracking turns ligatures off"),
            "0",
        ),
        d(
            param("leading", ParamKind::Length)
                .auto()
                .help("Line spacing: auto (the font's line height), px or pt, or % of the font size (120%)"),
            "auto",
        ),
        d(
            param("kerning", ParamKind::Choice)
                .choices(&["metrics", "none"])
                .help("metrics applies the font's kerning; none turns it off"),
            "metrics",
        ),
    ]
}

/// Placement flags (`--at`, `--anchor`, `--box`).
pub fn place_params(defaults: bool) -> Vec<ParamSpec> {
    let d = |p: ParamSpec, v: &'static str| if defaults { p.default(v) } else { p };
    vec![
        d(
            param("at", ParamKind::Point).help(
                "Anchor point on the canvas: X,Y in px, %, pt, mm, or in; negative values count from the far edge",
            ),
            "0,0",
        ),
        d(
            param("anchor", ParamKind::Choice)
                .choices(ANCHORS)
                .help("Which point of the text box sits at --at: tl t tr l c r bl b br"),
            "tl",
        ),
        param("box", ParamKind::Size).help(
            "Paragraph box WxH (px or % of the canvas): lines wrap at its width; the height is reported if exceeded",
        ),
    ]
}

/// What a set of style flags changed.
#[derive(Default)]
pub struct Changes {
    /// Family, weight, style, or variation changed: the face must be resolved again.
    pub face: bool,
    /// `--font` was given (a missing family is an error rather than a fallback).
    pub font_given: bool,
    pub any: bool,
}

/// Apply the style flags present in `a` to `spec`.
pub fn apply_style(a: &Args, doc: &Document, spec: &mut TextSpec) -> Result<Changes> {
    let mut ch = Changes::default();
    if let Some(f) = a.str("font") {
        let f = f.trim().to_string();
        if f.is_empty() {
            return Err(Error::usage("--font must not be empty").with_hint("pass a family, e.g. --font sans-serif"));
        }
        spec.font = f;
        ch.face = true;
        ch.font_given = true;
    }
    if let Some(s) = a.raw("size") {
        let l = Length::parse(s)?;
        if l.value <= 0.0 {
            return Err(Error::new("PX1013", format!("--size {s} must be greater than 0")).with_context("size", s));
        }
        spec.size = normalize_size(s);
        ch.any = true;
    }
    if let Some(w) = a.raw("weight") {
        spec.weight = parse_weight(w)?;
        ch.face = true;
    }
    if let Some(s) = a.raw("style") {
        spec.style = FontStyle::parse(s)
            .ok_or_else(|| Error::usage(format!("--style: '{s}' is not normal, italic, or oblique")))?;
        ch.face = true;
    }
    if let Some(v) = a.raw("variation") {
        let t = v.trim();
        spec.variation = if t.is_empty() || t == "none" {
            None
        } else {
            let parsed = parse_variations(t)?;
            (!parsed.is_empty()).then(|| format_variations(&parsed))
        };
        ch.face = true;
    }
    if a.has("color") {
        spec.color = a.color("color")?;
        ch.any = true;
    }
    if a.has("stroke") {
        spec.stroke = Some(a.color("stroke")?).filter(|c| c.a > 0.0);
        ch.any = true;
    }
    if a.has("stroke-width") {
        let size = spec.size_px(doc);
        let w = a.px("stroke-width", size, doc.dpi())?;
        if w < 0.0 {
            return Err(Error::new("PX1013", "--stroke-width must not be negative"));
        }
        spec.stroke_width = w;
        ch.any = true;
    }
    if let Some(al) = a.raw("align") {
        spec.align = Align::parse(al)
            .ok_or_else(|| Error::usage(format!("--align: '{al}' is not one of left, center, right, justify")))?;
        ch.any = true;
    }
    if a.has("tracking") {
        spec.tracking = a.f64("tracking")?;
        ch.any = true;
    }
    if let Some(l) = a.raw("leading") {
        if !valid_leading(l) {
            return Err(Error::usage(format!("--leading: '{l}' is not auto or a length")));
        }
        if l.trim() != "auto" && Length::parse(l)?.value <= 0.0 {
            return Err(Error::new("PX1013", "--leading must be greater than 0"));
        }
        spec.leading = normalize_size(l);
        ch.any = true;
    }
    if let Some(k) = a.raw("kerning") {
        spec.kerning = k.trim() != "none";
        ch.any = true;
    }
    ch.any |= ch.face;
    Ok(ch)
}

/// Apply `--at`, `--anchor`, and `--box` when present.
pub fn apply_place(a: &Args, doc: &Document, spec: &mut TextSpec) -> Result<bool> {
    let (w, h, dpi) = (doc.width as f64, doc.height as f64, doc.dpi());
    let mut any = false;
    if let Some(p) = a.point("at")? {
        let (x, y) = p.resolve(w, h, dpi);
        spec.x = x;
        spec.y = y;
        any = true;
    }
    if let Some(an) = a.raw("anchor") {
        spec.anchor = Anchor::parse(an)?;
        any = true;
    }
    if let Some(raw) = a.raw("box") {
        let sz = SizeSpec::parse(raw)?;
        let bw = sz.w.map(|l| l.to_px(w, dpi)).filter(|v| *v > 0.0).ok_or_else(|| {
            Error::new("PX1011", format!("--box '{raw}' needs a positive width (WxH or Wx)")).with_context("box", raw)
        })?;
        let bh = sz.h.map(|l| l.to_px(h, dpi)).filter(|v| *v > 0.0);
        spec.box_size = Some((bw, bh));
        any = true;
    }
    Ok(any)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_keeps_foreign_keys() {
        let base = json!({"content": "Hi", "size": 64, "slot_note": "keep me", "anchor": "c", "box": [300, null]});
        let s = TextSpec::from_value(&base);
        assert_eq!(s.size, "64px");
        assert_eq!(s.anchor, Anchor::C);
        assert_eq!(s.box_size, Some((300.0, None)));
        let v = s.to_value(Some(&base));
        assert_eq!(v["slot_note"], "keep me");
        assert_eq!(v["content"], "Hi");
        assert_eq!(v["rotation"], 0.0);
        assert!(v["path"].is_null());
        let again = TextSpec::from_value(&v);
        assert_eq!(again.size, "64px");
    }

    #[test]
    fn weights_and_variations() {
        assert_eq!(parse_weight("bold").unwrap(), 700);
        assert_eq!(parse_weight("Semi-Bold").unwrap(), 600);
        assert_eq!(parse_weight("650").unwrap(), 650);
        assert!(parse_weight("heavyish").is_err());
        assert_eq!(parse_weight("2000").unwrap_err().code, "PX1013");
        let v = parse_variations("wght=650, slnt=-5").unwrap();
        assert_eq!(v, vec![("wght".to_string(), 650.0), ("slnt".to_string(), -5.0)]);
        assert_eq!(format_variations(&v), "wght=650,slnt=-5");
        assert!(parse_variations("wght").is_err());
        assert!(parse_variations("toolong=1").is_err());
    }
}
