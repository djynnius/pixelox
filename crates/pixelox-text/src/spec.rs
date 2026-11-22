//! The text layer spec (`LayerKind::Text { spec }`): everything needed to re-render a type
//! layer, stored as JSON in the document. Reading is lenient (missing or malformed keys take
//! their defaults) because other crates and older documents write parts of it; writing keeps
//! keys this crate does not own.

use crate::fonts::FontStyle;
use crate::warp::Warp;
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

/// Case transform applied when the text is set (`--case`); the content keeps what was typed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Case {
    Upper,
    Lower,
    Title,
}

pub const CASES: &[&str] = &["none", "upper", "lower", "title"];

impl Case {
    pub fn parse(s: &str) -> Option<Option<Case>> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "none" | "as-typed" | "" => None,
            "upper" | "uppercase" | "caps" => Some(Case::Upper),
            "lower" | "lowercase" => Some(Case::Lower),
            "title" | "capitalize" => Some(Case::Title),
            _ => return None,
        })
    }
    pub fn name(self) -> &'static str {
        match self {
            Case::Upper => "upper",
            Case::Lower => "lower",
            Case::Title => "title",
        }
    }
    /// `title` capitalizes the first letter of every word and keeps the rest as typed
    /// (CSS `text-transform: capitalize`).
    pub fn apply(self, s: &str) -> String {
        match self {
            Case::Upper => s.to_uppercase(),
            Case::Lower => s.to_lowercase(),
            Case::Title => {
                let mut out = String::with_capacity(s.len());
                let mut start = true;
                for c in s.chars() {
                    if start && c.is_alphabetic() {
                        out.extend(c.to_uppercase());
                        start = false;
                    } else {
                        out.push(c);
                        if c.is_whitespace() || c == '-' || c == '/' {
                            start = true;
                        } else if c.is_alphanumeric() {
                            start = false;
                        }
                    }
                }
                out
            }
        }
    }
}

/// Size the type to a width or into a box (`--fit-width`, `--fit-box`), measured on the
/// visible letters (ink bounds, stroke included).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fit {
    Width(f64),
    Box(f64, f64),
}

/// A text layer's parameters (see the crate docs for the JSON form).
#[derive(Clone, Debug)]
pub struct TextSpec {
    pub content: String,
    /// Family as requested (`Inter`, `sans-serif`, `@condensed-heavy`).
    pub font: String,
    /// SHA-256 of the resolved face's file, preferred when re-resolving.
    pub font_sha256: Option<String>,
    /// The resolved face (`family`, `style`, `weight`, `sha256`, `postscript`), informational.
    pub font_face: Option<Value>,
    /// CSS length string: `48pt`, `64px`, `5%` (of the canvas height).
    pub size: String,
    /// `None`: automatic (400, or the weight of the class candidate).
    pub weight: Option<u16>,
    pub style: FontStyle,
    /// Width in percent (CSS font-stretch); `None`: normal or the class candidate's width.
    pub stretch: Option<f64>,
    pub case: Option<Case>,
    /// Horizontal / vertical scale of the set text (1 = 100%).
    pub scale_x: f64,
    pub scale_y: f64,
    /// Extra stem weight in 1/1000 em (faux bold, total of both sides).
    pub faux_bold: f64,
    pub fit: Option<Fit>,
    /// `--at`/`--anchor` place the visible letters (ink bounds) instead of the text box.
    pub anchor_ink: bool,
    pub warp: Option<Warp>,
    /// `text on-path --fit shrink`: the size shrinks until the text fits on the path.
    pub path_shrink: bool,
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
            weight: None,
            style: FontStyle::Normal,
            stretch: None,
            case: None,
            scale_x: 1.0,
            scale_y: 1.0,
            faux_bold: 0.0,
            fit: None,
            anchor_ink: false,
            warp: None,
            path_shrink: false,
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
                .and_then(|w| parse_weight(&w).ok()),
            style: text(g("style")).and_then(|s| FontStyle::parse(&s)).unwrap_or(d.style),
            stretch: num(g("stretch")).filter(|s| (50.0..=200.0).contains(s)),
            case: text(g("case")).and_then(|c| Case::parse(&c)).flatten(),
            scale_x: num(g("scale_x")).filter(|s| *s > 0.0).unwrap_or(1.0),
            scale_y: num(g("scale_y")).filter(|s| *s > 0.0).unwrap_or(1.0),
            faux_bold: num(g("faux_bold")).map(|f| f.clamp(0.0, 300.0)).unwrap_or(0.0),
            fit: match (num(g("fit_width")), g("fit_box")) {
                (Some(w), _) if w > 0.0 => Some(Fit::Width(w)),
                (_, Some(Value::Array(a))) if a.len() == 2 => match (num(a.first()), num(a.get(1))) {
                    (Some(w), Some(h)) if w > 0.0 && h > 0.0 => Some(Fit::Box(w, h)),
                    _ => None,
                },
                _ => None,
            },
            anchor_ink: text(g("anchor_to")).as_deref() == Some("ink"),
            warp: g("warp").and_then(Warp::from_json),
            path_shrink: text(g("path_fit")).as_deref() == Some("shrink"),
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
        // keys added in 0.5 are written only when they differ from the default (and
        // removed otherwise), so documents that do not use them stay as they were
        let mut opt = |k: &str, v: Option<Value>| match v {
            Some(v) => {
                m.insert(k.to_string(), v);
            }
            None => {
                m.remove(k);
            }
        };
        opt("stretch", self.stretch.map(|s| json!(s)));
        opt("case", self.case.map(|c| json!(c.name())));
        opt("scale_x", (self.scale_x != 1.0).then(|| json!(self.scale_x)));
        opt("scale_y", (self.scale_y != 1.0).then(|| json!(self.scale_y)));
        opt("faux_bold", (self.faux_bold != 0.0).then(|| json!(self.faux_bold)));
        opt(
            "fit_width",
            match self.fit {
                Some(Fit::Width(w)) => Some(json!(w)),
                _ => None,
            },
        );
        opt(
            "fit_box",
            match self.fit {
                Some(Fit::Box(w, h)) => Some(json!([w, h])),
                _ => None,
            },
        );
        opt("anchor_to", self.anchor_ink.then(|| json!("ink")));
        opt("warp", self.warp.as_ref().map(|w| w.to_json()));
        opt("path_fit", self.path_shrink.then(|| json!("shrink")));
        let mut set = |k: &str, v: Value| {
            m.insert(k.to_string(), v);
        };
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
                "Font family (case-insensitive), a class such as @condensed-heavy or @didone (`pixelox fonts classes`), or a generic name: sans-serif, serif, monospace; a missing family is replaced by the closest installed face of its class (warning PXW4010)",
            ),
            "sans-serif",
        ),
        d(
            param("size", ParamKind::Length)
                .help("Font size: 48pt (points at the document DPI), 64px, a bare number of px, or % of the canvas height"),
            "48pt",
        ),
        param("weight", ParamKind::Str).help(
            "Weight 100..900 or a name (thin, light, normal, medium, semibold, bold, extrabold, black); default 400, or the class's own weight with --font @class",
        ),
        d(
            param("style", ParamKind::Choice)
                .choices(&["normal", "italic", "oblique"])
                .help("Upright or italic; a missing italic face is slanted synthetically"),
            "normal",
        ),
        param("stretch", ParamKind::Str).help(
            "Width: ultra-condensed, extra-condensed, condensed, semi-condensed, normal, semi-expanded, expanded (extended), ... or 50%..200%; picks the face of that width (a weight never changes the width) or sets a variable font's wdth axis",
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
        param("case", ParamKind::Choice)
            .choices(CASES)
            .help("Set the text in upper, lower, or title case (the content keeps what was typed); none sets it as typed"),
        param("scale-x", ParamKind::Length).help(
            "Horizontal scale of the letters and their spacing (Photoshop's character scaling): 80% or 0.8; the text stays live",
        ),
        param("scale-y", ParamKind::Length).help("Vertical scale of the set text: 120% or 1.2; the text stays live"),
        param("faux-bold", ParamKind::Float)
            .range("0..300")
            .help("Thicken every stem by this much, in 1/1000 em (20 is about one weight step; 0 removes it)"),
        param("fit-width", ParamKind::Length).help(
            "Size the type so its visible letters span exactly this width (px, or % of the canvas width); the fit is redone when the text changes; 0 removes it",
        ),
        param("fit-box", ParamKind::Size).help(
            "Size the type as large as fits inside WxH (px or % of the canvas), measured on the visible letters; 0x0 removes it",
        ),
        param("warp", ParamKind::Choice).choices(&WARP_CHOICES).help(
            "Warp the glyph outlines (they stay vector and editable): arc, arc-lower, arc-upper, arch, bulge, shell-lower, shell-upper, flag, wave, fish, rise, fisheye, inflate, squeeze, twist; none removes it",
        ),
        param("bend", ParamKind::Float).range("-100..100").help("Warp strength in % (default 50; negative bends the other way)"),
        param("h-distort", ParamKind::Float)
            .range("-100..100")
            .help("Warp horizontal distortion in %: a perspective that grows the right side (negative: the left)"),
        param("v-distort", ParamKind::Float)
            .range("-100..100")
            .help("Warp vertical distortion in %: a perspective that grows the bottom (negative: the top)"),
        param("orient", ParamKind::Choice)
            .choices(&["horizontal", "vertical"])
            .help("Apply the warp along the horizontal (default) or vertical axis"),
        param("strict-font", ParamKind::Flag).help(
            "Fail when the font is not installed (PX4102, or PX1540 when it can be downloaded) instead of substituting; PIXELOX_STRICT_FONTS=1 does the same",
        ),
    ]
}

/// `--warp` choices: `none` and the presets.
pub const WARP_CHOICES: [&str; 16] = [
    "none",
    "arc",
    "arc-lower",
    "arc-upper",
    "arch",
    "bulge",
    "shell-lower",
    "shell-upper",
    "flag",
    "wave",
    "fish",
    "rise",
    "fisheye",
    "inflate",
    "squeeze",
    "twist",
];

/// Placement flags (`--at`, `--anchor`, `--box`, `--anchor-to`).
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
        param("anchor-to", ParamKind::Choice).choices(&["box", "ink"]).help(
            "box: --at/--anchor place the text box (default); ink: they place the visible letters, so --at 80,90 --anchor tl puts the top-left of the letters exactly there",
        ),
    ]
}

/// What a set of style flags changed.
#[derive(Default)]
pub struct Changes {
    /// Family, weight, style, stretch, or variation changed: the face must be resolved again.
    pub face: bool,
    /// `--font` was given.
    pub font_given: bool,
    pub any: bool,
}

/// A scale factor: `80%` → 0.8, `0.8` → 0.8.
fn parse_scale(name: &str, s: &str) -> Result<f64> {
    let l = Length::parse(s)?;
    let v = match l.unit {
        pixelox_core::units::Unit::Pct => l.value / 100.0,
        pixelox_core::units::Unit::Px => l.value,
        _ => {
            return Err(Error::usage(format!("--{name}: '{s}' is not a scale"))
                .with_hint(format!("use a percent (--{name} 80%) or a factor (--{name} 0.8)")))
        }
    };
    if !(0.01..=20.0).contains(&v) {
        return Err(Error::new("PX1013", format!("--{name} {s} is outside 1%..2000%")).with_context(name, s));
    }
    Ok(v)
}

/// Apply the style flags present in `a` to `spec`.
pub fn apply_style(a: &Args, doc: &Document, spec: &mut TextSpec) -> Result<Changes> {
    let mut ch = Changes::default();
    let (cw, chh, dpi) = (doc.width as f64, doc.height as f64, doc.dpi());
    if let Some(f) = a.str("font") {
        let f = f.trim().to_string();
        if f.is_empty() {
            return Err(Error::usage("--font must not be empty").with_hint("pass a family, e.g. --font sans-serif"));
        }
        if crate::classes::is_class_ref(&f) && crate::classes::find(&f).is_none() {
            return Err(crate::fonts::unknown_class(&f));
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
        // an explicit size replaces a fit, unless a fit is given in the same call
        spec.fit = None;
        ch.any = true;
    }
    if let Some(w) = a.raw("weight") {
        spec.weight = Some(parse_weight(w)?);
        ch.face = true;
    }
    if let Some(s) = a.raw("style") {
        spec.style = FontStyle::parse(s)
            .ok_or_else(|| Error::usage(format!("--style: '{s}' is not normal, italic, or oblique")))?;
        ch.face = true;
    }
    if let Some(s) = a.raw("stretch") {
        let p = crate::fonts::parse_stretch(s)? as f64;
        spec.stretch = Some(p);
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
    if let Some(c) = a.raw("case") {
        spec.case =
            Case::parse(c).ok_or_else(|| Error::usage(format!("--case: '{c}' is not upper, lower, title, or none")))?;
        ch.any = true;
    }
    if let Some(s) = a.raw("scale-x") {
        spec.scale_x = parse_scale("scale-x", s)?;
        ch.any = true;
    }
    if let Some(s) = a.raw("scale-y") {
        spec.scale_y = parse_scale("scale-y", s)?;
        ch.any = true;
    }
    if a.has("faux-bold") {
        spec.faux_bold = a.f64("faux-bold")?.clamp(0.0, 300.0);
        ch.any = true;
    }
    match (a.raw("fit-width"), a.raw("fit-box")) {
        (Some(_), Some(_)) => {
            return Err(Error::new("PX1005", "give either --fit-width or --fit-box, not both")
                .with_hint("--fit-width 920 sizes to a width; --fit-box 920x300 sizes into a box"))
        }
        (Some(fw), None) => {
            let w = Length::parse(fw)?.to_px(cw, dpi);
            if w < 0.0 {
                return Err(Error::new("PX1013", "--fit-width must not be negative"));
            }
            spec.fit = (w > 0.0).then_some(Fit::Width(w));
            ch.any = true;
        }
        (None, Some(fb)) => {
            let sz = SizeSpec::parse(fb)?;
            let w = sz.w.map(|l| l.to_px(cw, dpi)).unwrap_or(0.0);
            let h = sz.h.map(|l| l.to_px(chh, dpi)).unwrap_or(0.0);
            spec.fit = match (w > 0.0, h > 0.0) {
                (true, true) => Some(Fit::Box(w, h)),
                (true, false) => Some(Fit::Width(w)),
                (false, true) => Some(Fit::Box(f64::MAX / 4.0, h)),
                _ => None,
            };
            ch.any = true;
        }
        (None, None) => {}
    }
    let warp_keys = ["bend", "h-distort", "v-distort", "orient"];
    if let Some(p) = a.raw("warp") {
        match crate::warp::parse_preset(p)? {
            None => spec.warp = None,
            Some(preset) => {
                let base = spec.warp.clone();
                spec.warp = Some(Warp {
                    preset,
                    bend: base.as_ref().map(|w| w.bend).unwrap_or(50.0),
                    h_distort: base.as_ref().map(|w| w.h_distort).unwrap_or(0.0),
                    v_distort: base.as_ref().map(|w| w.v_distort).unwrap_or(0.0),
                    vertical: base.as_ref().map(|w| w.vertical).unwrap_or(false),
                });
            }
        }
        ch.any = true;
    }
    if warp_keys.iter().any(|k| a.has(k)) {
        let w = spec.warp.as_mut().ok_or_else(|| {
            Error::new("PX1003", "the layer has no warp to adjust: give --warp <preset> as well")
                .with_hint("e.g. --warp arc --bend 40")
        })?;
        if a.has("bend") {
            w.bend = a.f64("bend")?;
        }
        if a.has("h-distort") {
            w.h_distort = a.f64("h-distort")?;
        }
        if a.has("v-distort") {
            w.v_distort = a.f64("v-distort")?;
        }
        if let Some(o) = a.raw("orient") {
            w.vertical = o.trim() == "vertical";
        }
        ch.any = true;
    }
    ch.any |= ch.face;
    Ok(ch)
}

/// Apply `--at`, `--anchor`, `--box`, and `--anchor-to` when present.
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
    if let Some(t) = a.raw("anchor-to") {
        spec.anchor_ink = t.trim() == "ink";
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
        // 0.5 keys are absent unless set
        for k in ["stretch", "case", "scale_x", "faux_bold", "fit_width", "warp", "anchor_to", "path_fit"] {
            assert!(v.get(k).is_none(), "{k}");
        }
        assert_eq!(again.weight, None);
    }

    #[test]
    fn new_keys_roundtrip_and_reset() {
        let mut s = TextSpec {
            content: "Hi".into(),
            weight: Some(700),
            stretch: Some(75.0),
            case: Some(Case::Upper),
            scale_x: 0.8,
            faux_bold: 20.0,
            fit: Some(Fit::Box(900.0, 300.0)),
            anchor_ink: true,
            warp: Some(Warp {
                preset: crate::warp::Preset::Arc,
                bend: 40.0,
                h_distort: 0.0,
                v_distort: 10.0,
                vertical: false,
            }),
            path_shrink: true,
            ..TextSpec::default()
        };
        let v = s.to_value(None);
        let back = TextSpec::from_value(&v);
        assert_eq!(back.weight, Some(700));
        assert_eq!(back.stretch, Some(75.0));
        assert_eq!(back.case, Some(Case::Upper));
        assert_eq!(back.scale_x, 0.8);
        assert_eq!(back.faux_bold, 20.0);
        assert_eq!(back.fit, Some(Fit::Box(900.0, 300.0)));
        assert!(back.anchor_ink && back.path_shrink);
        assert_eq!(back.warp, s.warp);
        // resetting removes the keys from the stored spec, even with the old spec as base
        s.case = None;
        s.warp = None;
        s.fit = None;
        s.scale_x = 1.0;
        let v2 = s.to_value(Some(&v));
        assert!(v2.get("case").is_none() && v2.get("warp").is_none() && v2.get("fit_box").is_none());
        assert!(v2.get("scale_x").is_none());
    }

    #[test]
    fn cases_transform() {
        assert_eq!(Case::Upper.apply("straße"), "STRASSE");
        assert_eq!(Case::Lower.apply("HeLLo"), "hello");
        assert_eq!(Case::Title.apply("night of worship-and praise"), "Night Of Worship-And Praise");
        assert_eq!(Case::Title.apply("iPhone 15 pro"), "IPhone 15 Pro");
        assert_eq!(Case::parse("none"), Some(None));
        assert_eq!(Case::parse("caps"), Some(Some(Case::Upper)));
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
