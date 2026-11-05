//! Color values and color math: parsing (`#RRGGBB[AA]`, `rgb()`, `hsl()`, CSS names),
//! sRGB ⇄ linear, HSL, CIE Lab with ΔE00, OKLab, and luminance.
//!
//! Pixel values throughout pixelox are `f32` in 0..1, sRGB-encoded (the document working
//! space), straight (not premultiplied) alpha.

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

/// An RGBA color, sRGB-encoded, straight alpha, components 0..1.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub const TRANSPARENT: Rgba = Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    pub const BLACK: Rgba = Rgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const WHITE: Rgba = Rgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Rgba {
        Rgba { r, g, b, a }
    }

    pub fn from_u8(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
    }

    pub fn to_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    pub fn from_array(a: [f32; 4]) -> Rgba {
        Rgba::new(a[0], a[1], a[2], a[3])
    }

    pub fn to_u8(self) -> [u8; 4] {
        [to_u8(self.r), to_u8(self.g), to_u8(self.b), to_u8(self.a)]
    }

    /// `#rrggbb` or `#rrggbbaa` when not opaque.
    pub fn to_hex(self) -> String {
        let [r, g, b, a] = self.to_u8();
        if a == 255 {
            format!("#{r:02x}{g:02x}{b:02x}")
        } else {
            format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
        }
    }

    pub fn with_alpha(mut self, a: f32) -> Rgba {
        self.a = a;
        self
    }

    /// Parse any accepted color syntax.
    pub fn parse(s: &str) -> Result<Rgba> {
        parse_color(s)
    }
}

pub fn to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
}

pub fn to_u16(v: f32) -> u16 {
    (v.clamp(0.0, 1.0) * 65535.0 + 0.5) as u16
}

fn bad(s: &str) -> Error {
    Error::new("PX1012", format!("'{s}' is not a color")).with_context("value", s)
}

/// Parse `#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`, `rgb(r,g,b)`, `rgba(r,g,b,a)`,
/// `hsl(h,s%,l%)`, `hsla(...)`, a CSS color name, or `transparent`.
pub fn parse_color(s: &str) -> Result<Rgba> {
    let t = s.trim();
    let lower = t.to_ascii_lowercase();
    if let Some(hex) = lower.strip_prefix('#') {
        let digits: Vec<u8> =
            hex.chars().map(|c| c.to_digit(16).map(|d| d as u8)).collect::<Option<Vec<u8>>>().ok_or_else(|| bad(s))?;
        return match digits.len() {
            3 => Ok(Rgba::from_u8(digits[0] * 17, digits[1] * 17, digits[2] * 17, 255)),
            4 => Ok(Rgba::from_u8(digits[0] * 17, digits[1] * 17, digits[2] * 17, digits[3] * 17)),
            6 => Ok(Rgba::from_u8(
                digits[0] * 16 + digits[1],
                digits[2] * 16 + digits[3],
                digits[4] * 16 + digits[5],
                255,
            )),
            8 => Ok(Rgba::from_u8(
                digits[0] * 16 + digits[1],
                digits[2] * 16 + digits[3],
                digits[4] * 16 + digits[5],
                digits[6] * 16 + digits[7],
            )),
            _ => Err(bad(s)),
        };
    }
    if let Some(inner) = func_args(&lower, &["rgba", "rgb"]) {
        let parts = split_args(inner);
        if parts.len() != 3 && parts.len() != 4 {
            return Err(bad(s));
        }
        let ch = |p: &str| -> Result<f32> {
            if let Some(pc) = p.strip_suffix('%') {
                pc.trim().parse::<f32>().map(|v| v / 100.0).map_err(|_| bad(s))
            } else {
                p.trim().parse::<f32>().map(|v| v / 255.0).map_err(|_| bad(s))
            }
        };
        let a = if parts.len() == 4 { alpha(parts[3]).ok_or_else(|| bad(s))? } else { 1.0 };
        return Ok(Rgba::new(ch(parts[0])?, ch(parts[1])?, ch(parts[2])?, a));
    }
    if let Some(inner) = func_args(&lower, &["hsla", "hsl"]) {
        let parts = split_args(inner);
        if parts.len() != 3 && parts.len() != 4 {
            return Err(bad(s));
        }
        let h: f32 = parts[0].trim().trim_end_matches("deg").parse().map_err(|_| bad(s))?;
        let pct = |p: &str| -> Result<f32> {
            let p = p.trim();
            let v: f32 = p.trim_end_matches('%').parse().map_err(|_| bad(s))?;
            Ok(if p.ends_with('%') || v > 1.0 { v / 100.0 } else { v })
        };
        let (r, g, b) = hsl_to_rgb(h, pct(parts[1])?, pct(parts[2])?);
        let a = if parts.len() == 4 { alpha(parts[3]).ok_or_else(|| bad(s))? } else { 1.0 };
        return Ok(Rgba::new(r, g, b, a));
    }
    if lower == "transparent" || lower == "none" {
        return Ok(Rgba::TRANSPARENT);
    }
    named_color(&lower).map(|(r, g, b)| Rgba::from_u8(r, g, b, 255)).ok_or_else(|| bad(s))
}

fn alpha(p: &str) -> Option<f32> {
    let p = p.trim();
    if let Some(pc) = p.strip_suffix('%') {
        pc.trim().parse::<f32>().ok().map(|v| v / 100.0)
    } else {
        p.parse::<f32>().ok()
    }
}

fn func_args<'a>(s: &'a str, names: &[&str]) -> Option<&'a str> {
    for n in names {
        if let Some(rest) = s.strip_prefix(n) {
            let rest = rest.trim_start();
            if let Some(inner) = rest.strip_prefix('(').and_then(|r| r.strip_suffix(')')) {
                return Some(inner);
            }
        }
    }
    None
}

fn split_args(s: &str) -> Vec<&str> {
    if s.contains(',') {
        s.split(',').map(|p| p.trim()).collect()
    } else {
        // CSS level 4: `rgb(10 20 30 / 50%)`
        s.split(|c: char| c.is_whitespace() || c == '/').filter(|p| !p.is_empty()).collect()
    }
}

/// HSL (h degrees, s and l 0..1) → RGB 0..1.
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let h = h.rem_euclid(360.0) / 360.0;
    if s <= 0.0 {
        return (l, l, l);
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let f = |mut t: f32| {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    };
    (f(h + 1.0 / 3.0), f(h), f(h - 1.0 / 3.0))
}

/// RGB 0..1 → HSL (h degrees 0..360, s and l 0..1).
pub fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < 1e-7 {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if max == r {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if max == g {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    (h * 60.0, s, l)
}

/// RGB 0..1 → HSV (h degrees, s and v 0..1).
pub fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let h = if d < 1e-7 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / d).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    let s = if max <= 0.0 { 0.0 } else { d / max };
    (h, s, max)
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let hp = h.rem_euclid(360.0) / 60.0;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let (r, g, b) = match hp as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = v - c;
    (r + m, g + m, b + m)
}

/// sRGB transfer function: encoded → linear.
#[inline]
pub fn srgb_to_linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

/// sRGB transfer function: linear → encoded.
#[inline]
pub fn linear_to_srgb(v: f32) -> f32 {
    if v <= 0.003_130_8 {
        v * 12.92
    } else {
        1.055 * v.max(0.0).powf(1.0 / 2.4) - 0.055
    }
}

/// Rec. 709 luma of encoded RGB (the Photoshop-style "luminosity" used for selections).
#[inline]
pub fn luma(r: f32, g: f32, b: f32) -> f32 {
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

/// Luminosity used by the non-separable blend modes (PDF/W3C spec weights).
#[inline]
pub fn lum(r: f32, g: f32, b: f32) -> f32 {
    0.3 * r + 0.59 * g + 0.11 * b
}

/// Encoded sRGB → CIE L*a*b* (D65).
pub fn rgb_to_lab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let (r, g, b) = (srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b));
    let x = (0.412_456_4 * r + 0.357_576_1 * g + 0.180_437_5 * b) / 0.950_47;
    let y = 0.212_672_9 * r + 0.715_152_2 * g + 0.072_175 * b;
    let z = (0.019_333_9 * r + 0.119_192 * g + 0.950_304_1 * b) / 1.088_83;
    let f = |t: f32| if t > 0.008_856 { t.cbrt() } else { 7.787 * t + 16.0 / 116.0 };
    let (fx, fy, fz) = (f(x), f(y), f(z));
    (116.0 * fy - 16.0, 500.0 * (fx - fy), 200.0 * (fy - fz))
}

/// CIEDE2000 color difference between two Lab colors.
pub fn delta_e00(l1: f32, a1: f32, b1: f32, l2: f32, a2: f32, b2: f32) -> f32 {
    let (l1, a1, b1, l2, a2, b2) = (l1 as f64, a1 as f64, b1 as f64, l2 as f64, a2 as f64, b2 as f64);
    let c1 = (a1 * a1 + b1 * b1).sqrt();
    let c2 = (a2 * a2 + b2 * b2).sqrt();
    let cb = (c1 + c2) / 2.0;
    let g = 0.5 * (1.0 - (cb.powi(7) / (cb.powi(7) + 25f64.powi(7))).sqrt());
    let a1p = (1.0 + g) * a1;
    let a2p = (1.0 + g) * a2;
    let c1p = (a1p * a1p + b1 * b1).sqrt();
    let c2p = (a2p * a2p + b2 * b2).sqrt();
    let h = |b: f64, a: f64| {
        if a == 0.0 && b == 0.0 {
            0.0
        } else {
            let d = b.atan2(a).to_degrees();
            if d < 0.0 {
                d + 360.0
            } else {
                d
            }
        }
    };
    let h1p = h(b1, a1p);
    let h2p = h(b2, a2p);
    let dlp = l2 - l1;
    let dcp = c2p - c1p;
    let dhp = if c1p * c2p == 0.0 {
        0.0
    } else if (h2p - h1p).abs() <= 180.0 {
        h2p - h1p
    } else if h2p - h1p > 180.0 {
        h2p - h1p - 360.0
    } else {
        h2p - h1p + 360.0
    };
    let dhp_big = 2.0 * (c1p * c2p).sqrt() * (dhp.to_radians() / 2.0).sin();
    let lbp = (l1 + l2) / 2.0;
    let cbp = (c1p + c2p) / 2.0;
    let hbp = if c1p * c2p == 0.0 {
        h1p + h2p
    } else if (h1p - h2p).abs() <= 180.0 {
        (h1p + h2p) / 2.0
    } else if h1p + h2p < 360.0 {
        (h1p + h2p + 360.0) / 2.0
    } else {
        (h1p + h2p - 360.0) / 2.0
    };
    let t = 1.0 - 0.17 * (hbp - 30.0).to_radians().cos()
        + 0.24 * (2.0 * hbp).to_radians().cos()
        + 0.32 * (3.0 * hbp + 6.0).to_radians().cos()
        - 0.20 * (4.0 * hbp - 63.0).to_radians().cos();
    let dtheta = 30.0 * (-((hbp - 275.0) / 25.0).powi(2)).exp();
    let rc = 2.0 * (cbp.powi(7) / (cbp.powi(7) + 25f64.powi(7))).sqrt();
    let sl = 1.0 + (0.015 * (lbp - 50.0).powi(2)) / (20.0 + (lbp - 50.0).powi(2)).sqrt();
    let sc = 1.0 + 0.045 * cbp;
    let sh = 1.0 + 0.015 * cbp * t;
    let rt = -(2.0 * dtheta).to_radians().sin() * rc;
    let v = (dlp / sl).powi(2) + (dcp / sc).powi(2) + (dhp_big / sh).powi(2) + rt * (dcp / sc) * (dhp_big / sh);
    v.max(0.0).sqrt() as f32
}

/// Encoded sRGB → OKLab.
pub fn rgb_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let (r, g, b) = (srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b));
    let l = 0.412_221_47 * r + 0.536_332_55 * g + 0.051_445_995 * b;
    let m = 0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b;
    let s = 0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b;
    let (l, m, s) = (l.cbrt(), m.cbrt(), s.cbrt());
    (
        0.210_454_26 * l + 0.793_617_8 * m - 0.004_072_047 * s,
        1.977_998_5 * l - 2.428_592_2 * m + 0.450_593_7 * s,
        0.025_904_037 * l + 0.782_771_77 * m - 0.808_675_77 * s,
    )
}

/// OKLab → encoded sRGB (unclamped).
pub fn oklab_to_rgb(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let l_ = l + 0.396_337_78 * a + 0.215_803_76 * b;
    let m_ = l - 0.105_561_346 * a - 0.063_854_17 * b;
    let s_ = l - 0.089_484_18 * a - 1.291_485_5 * b;
    let (l3, m3, s3) = (l_ * l_ * l_, m_ * m_ * m_, s_ * s_ * s_);
    let r = 4.076_741_7 * l3 - 3.307_711_6 * m3 + 0.230_969_94 * s3;
    let g = -1.268_438 * l3 + 2.609_757_4 * m3 - 0.341_319_38 * s3;
    let b = -0.004_196_086_3 * l3 - 0.703_418_6 * m3 + 1.707_614_7 * s3;
    (linear_to_srgb(r), linear_to_srgb(g), linear_to_srgb(b))
}

/// Interpolation space for gradients (`--interp perceptual|linear|oklab`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Interp {
    /// Straight interpolation of encoded sRGB values (Photoshop's default).
    Perceptual,
    /// Interpolation in linear light.
    Linear,
    /// Interpolation in OKLab.
    Oklab,
}

impl Interp {
    pub fn parse(s: &str) -> Result<Interp> {
        match s {
            "perceptual" | "srgb" => Ok(Interp::Perceptual),
            "linear" => Ok(Interp::Linear),
            "oklab" => Ok(Interp::Oklab),
            _ => Err(Error::usage(format!("--interp must be perceptual, linear, or oklab, got '{s}'"))),
        }
    }
}

/// Mix two colors at `t` in the given space (alpha mixes linearly).
pub fn mix(a: Rgba, b: Rgba, t: f32, interp: Interp) -> Rgba {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: f32, y: f32| x + (y - x) * t;
    let alpha = lerp(a.a, b.a);
    match interp {
        Interp::Perceptual => Rgba::new(lerp(a.r, b.r), lerp(a.g, b.g), lerp(a.b, b.b), alpha),
        Interp::Linear => Rgba::new(
            linear_to_srgb(lerp(srgb_to_linear(a.r), srgb_to_linear(b.r))),
            linear_to_srgb(lerp(srgb_to_linear(a.g), srgb_to_linear(b.g))),
            linear_to_srgb(lerp(srgb_to_linear(a.b), srgb_to_linear(b.b))),
            alpha,
        ),
        Interp::Oklab => {
            let (l1, a1, b1) = rgb_to_oklab(a.r, a.g, a.b);
            let (l2, a2, b2) = rgb_to_oklab(b.r, b.g, b.b);
            let (r, g, bb) = oklab_to_rgb(lerp(l1, l2), lerp(a1, a2), lerp(b1, b2));
            Rgba::new(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), bb.clamp(0.0, 1.0), alpha)
        }
    }
}

/// A gradient stop.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    pub color: Rgba,
    /// Position 0..1.
    pub pos: f32,
}

/// Parse `#000@0,#ff8800@0.5,#fff@1` (positions 0..1 or percentages). Commas inside
/// `rgb(...)`/`hsl(...)` are respected. Stops without a position are spread evenly.
pub fn parse_stops(s: &str) -> Result<Vec<Stop>> {
    let mut parts = Vec::new();
    let mut depth = 0;
    let mut cur = String::new();
    for c in s.chars() {
        match c {
            '(' => {
                depth += 1;
                cur.push(c);
            }
            ')' => {
                depth -= 1;
                cur.push(c);
            }
            ',' if depth == 0 => parts.push(std::mem::take(&mut cur)),
            _ => cur.push(c),
        }
    }
    if !cur.trim().is_empty() {
        parts.push(cur);
    }
    if parts.len() < 2 {
        return Err(Error::new("PX1010", format!("gradient stops need at least two colors, got '{s}'"))
            .with_hint("write stops like '#000@0,#fff@1'"));
    }
    let n = parts.len();
    let mut stops = Vec::new();
    for (i, p) in parts.iter().enumerate() {
        let p = p.trim();
        let (c, pos) = match p.rsplit_once('@') {
            Some((c, pos)) => {
                let pos = pos.trim();
                let v = if let Some(pc) = pos.strip_suffix('%') {
                    pc.parse::<f32>().map(|v| v / 100.0)
                } else {
                    pos.parse::<f32>()
                }
                .map_err(|_| Error::new("PX1010", format!("bad stop position '{pos}' in '{s}'")))?;
                (c, v)
            }
            None => (p, i as f32 / (n - 1) as f32),
        };
        stops.push(Stop { color: parse_color(c)?, pos: pos.clamp(0.0, 1.0) });
    }
    stops.sort_by(|a, b| a.pos.partial_cmp(&b.pos).unwrap_or(std::cmp::Ordering::Equal));
    Ok(stops)
}

/// Sample a stop list at `t` (0..1).
pub fn sample_stops(stops: &[Stop], t: f32, interp: Interp) -> Rgba {
    if stops.is_empty() {
        return Rgba::TRANSPARENT;
    }
    if t <= stops[0].pos {
        return stops[0].color;
    }
    for w in stops.windows(2) {
        if t <= w[1].pos {
            let span = (w[1].pos - w[0].pos).max(1e-6);
            return mix(w[0].color, w[1].color, (t - w[0].pos) / span, interp);
        }
    }
    stops[stops.len() - 1].color
}

/// The 148 CSS named colors.
pub fn named_color(name: &str) -> Option<(u8, u8, u8)> {
    let v: u32 = match name {
        "aliceblue" => 0xf0f8ff,
        "antiquewhite" => 0xfaebd7,
        "aqua" | "cyan" => 0x00ffff,
        "aquamarine" => 0x7fffd4,
        "azure" => 0xf0ffff,
        "beige" => 0xf5f5dc,
        "bisque" => 0xffe4c4,
        "black" => 0x000000,
        "blanchedalmond" => 0xffebcd,
        "blue" => 0x0000ff,
        "blueviolet" => 0x8a2be2,
        "brown" => 0xa52a2a,
        "burlywood" => 0xdeb887,
        "cadetblue" => 0x5f9ea0,
        "chartreuse" => 0x7fff00,
        "chocolate" => 0xd2691e,
        "coral" => 0xff7f50,
        "cornflowerblue" => 0x6495ed,
        "cornsilk" => 0xfff8dc,
        "crimson" => 0xdc143c,
        "darkblue" => 0x00008b,
        "darkcyan" => 0x008b8b,
        "darkgoldenrod" => 0xb8860b,
        "darkgray" | "darkgrey" => 0xa9a9a9,
        "darkgreen" => 0x006400,
        "darkkhaki" => 0xbdb76b,
        "darkmagenta" => 0x8b008b,
        "darkolivegreen" => 0x556b2f,
        "darkorange" => 0xff8c00,
        "darkorchid" => 0x9932cc,
        "darkred" => 0x8b0000,
        "darksalmon" => 0xe9967a,
        "darkseagreen" => 0x8fbc8f,
        "darkslateblue" => 0x483d8b,
        "darkslategray" | "darkslategrey" => 0x2f4f4f,
        "darkturquoise" => 0x00ced1,
        "darkviolet" => 0x9400d3,
        "deeppink" => 0xff1493,
        "deepskyblue" => 0x00bfff,
        "dimgray" | "dimgrey" => 0x696969,
        "dodgerblue" => 0x1e90ff,
        "firebrick" => 0xb22222,
        "floralwhite" => 0xfffaf0,
        "forestgreen" => 0x228b22,
        "fuchsia" | "magenta" => 0xff00ff,
        "gainsboro" => 0xdcdcdc,
        "ghostwhite" => 0xf8f8ff,
        "gold" => 0xffd700,
        "goldenrod" => 0xdaa520,
        "gray" | "grey" => 0x808080,
        "green" => 0x008000,
        "greenyellow" => 0xadff2f,
        "honeydew" => 0xf0fff0,
        "hotpink" => 0xff69b4,
        "indianred" => 0xcd5c5c,
        "indigo" => 0x4b0082,
        "ivory" => 0xfffff0,
        "khaki" => 0xf0e68c,
        "lavender" => 0xe6e6fa,
        "lavenderblush" => 0xfff0f5,
        "lawngreen" => 0x7cfc00,
        "lemonchiffon" => 0xfffacd,
        "lightblue" => 0xadd8e6,
        "lightcoral" => 0xf08080,
        "lightcyan" => 0xe0ffff,
        "lightgoldenrodyellow" => 0xfafad2,
        "lightgray" | "lightgrey" => 0xd3d3d3,
        "lightgreen" => 0x90ee90,
        "lightpink" => 0xffb6c1,
        "lightsalmon" => 0xffa07a,
        "lightseagreen" => 0x20b2aa,
        "lightskyblue" => 0x87cefa,
        "lightslategray" | "lightslategrey" => 0x778899,
        "lightsteelblue" => 0xb0c4de,
        "lightyellow" => 0xffffe0,
        "lime" => 0x00ff00,
        "limegreen" => 0x32cd32,
        "linen" => 0xfaf0e6,
        "maroon" => 0x800000,
        "mediumaquamarine" => 0x66cdaa,
        "mediumblue" => 0x0000cd,
        "mediumorchid" => 0xba55d3,
        "mediumpurple" => 0x9370db,
        "mediumseagreen" => 0x3cb371,
        "mediumslateblue" => 0x7b68ee,
        "mediumspringgreen" => 0x00fa9a,
        "mediumturquoise" => 0x48d1cc,
        "mediumvioletred" => 0xc71585,
        "midnightblue" => 0x191970,
        "mintcream" => 0xf5fffa,
        "mistyrose" => 0xffe4e1,
        "moccasin" => 0xffe4b5,
        "navajowhite" => 0xffdead,
        "navy" => 0x000080,
        "oldlace" => 0xfdf5e6,
        "olive" => 0x808000,
        "olivedrab" => 0x6b8e23,
        "orange" => 0xffa500,
        "orangered" => 0xff4500,
        "orchid" => 0xda70d6,
        "palegoldenrod" => 0xeee8aa,
        "palegreen" => 0x98fb98,
        "paleturquoise" => 0xafeeee,
        "palevioletred" => 0xdb7093,
        "papayawhip" => 0xffefd5,
        "peachpuff" => 0xffdab9,
        "peru" => 0xcd853f,
        "pink" => 0xffc0cb,
        "plum" => 0xdda0dd,
        "powderblue" => 0xb0e0e6,
        "purple" => 0x800080,
        "rebeccapurple" => 0x663399,
        "red" => 0xff0000,
        "rosybrown" => 0xbc8f8f,
        "royalblue" => 0x4169e1,
        "saddlebrown" => 0x8b4513,
        "salmon" => 0xfa8072,
        "sandybrown" => 0xf4a460,
        "seagreen" => 0x2e8b57,
        "seashell" => 0xfff5ee,
        "sienna" => 0xa0522d,
        "silver" => 0xc0c0c0,
        "skyblue" => 0x87ceeb,
        "slateblue" => 0x6a5acd,
        "slategray" | "slategrey" => 0x708090,
        "snow" => 0xfffafa,
        "springgreen" => 0x00ff7f,
        "steelblue" => 0x4682b4,
        "tan" => 0xd2b48c,
        "teal" => 0x008080,
        "thistle" => 0xd8bfd8,
        "tomato" => 0xff6347,
        "turquoise" => 0x40e0d0,
        "violet" => 0xee82ee,
        "wheat" => 0xf5deb3,
        "white" => 0xffffff,
        "whitesmoke" => 0xf5f5f5,
        "yellow" => 0xffff00,
        "yellowgreen" => 0x9acd32,
        _ => return None,
    };
    Some(((v >> 16) as u8, (v >> 8) as u8, v as u8))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_every_syntax() {
        assert_eq!(parse_color("#0ea5e9").unwrap().to_hex(), "#0ea5e9");
        assert_eq!(parse_color("#00000066").unwrap().to_u8()[3], 0x66);
        assert_eq!(parse_color("#fff").unwrap(), Rgba::WHITE);
        assert_eq!(parse_color("rgb(255, 0, 0)").unwrap().to_hex(), "#ff0000");
        assert_eq!(parse_color("rgba(0,0,255,0.5)").unwrap().to_u8(), [0, 0, 255, 128]);
        assert_eq!(parse_color("hsl(120, 100%, 50%)").unwrap().to_hex(), "#00ff00");
        assert_eq!(parse_color("RebeccaPurple").unwrap().to_hex(), "#663399");
        assert_eq!(parse_color("transparent").unwrap().a, 0.0);
        assert!(parse_color("#12").is_err());
        assert!(parse_color("notacolor").is_err());
    }

    #[test]
    fn stops_with_functional_colors() {
        let s = parse_stops("rgb(0,0,0)@0,#ff8800@50%,#fff@1").unwrap();
        assert_eq!(s.len(), 3);
        assert!((s[1].pos - 0.5).abs() < 1e-6);
        let mid = sample_stops(&s, 0.25, Interp::Perceptual);
        assert!(mid.r > 0.0 && mid.r < 1.0);
        let even = parse_stops("red,green,blue").unwrap();
        assert!((even[1].pos - 0.5).abs() < 1e-6);
    }

    #[test]
    fn round_trips() {
        let (h, s, l) = rgb_to_hsl(0.2, 0.4, 0.8);
        let (r, g, b) = hsl_to_rgb(h, s, l);
        assert!((r - 0.2).abs() < 1e-4 && (g - 0.4).abs() < 1e-4 && (b - 0.8).abs() < 1e-4);
        for v in [0.0f32, 0.01, 0.2, 0.5, 0.99, 1.0] {
            assert!((linear_to_srgb(srgb_to_linear(v)) - v).abs() < 1e-5);
        }
        let (l, a, b) = rgb_to_oklab(0.3, 0.6, 0.1);
        let (r, g, bb) = oklab_to_rgb(l, a, b);
        assert!((r - 0.3).abs() < 1e-3 && (g - 0.6).abs() < 1e-3 && (bb - 0.1).abs() < 1e-3);
    }

    #[test]
    fn delta_e_reference_pair() {
        // Sharma et al. test pair 1: ΔE00 = 2.0425
        let d = delta_e00(50.0, 2.6772, -79.7751, 50.0, 0.0, -82.7485);
        assert!((d - 2.0425).abs() < 1e-3, "{d}");
        assert_eq!(delta_e00(50.0, 10.0, 10.0, 50.0, 10.0, 10.0), 0.0);
    }
}
