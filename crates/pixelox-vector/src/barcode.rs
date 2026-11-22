//! `shape barcode`: linear barcodes (EAN-13, EAN-8, UPC-A, Code 128, Code 39) as live
//! vector shape layers.
//!
//! The encoders are small and table driven (ISO/IEC 15420 for EAN/UPC, ISO/IEC 15417 for
//! Code 128 with automatic code set selection, ISO/IEC 16388 for Code 39 at a 3:1 wide to
//! narrow ratio). Bars are merged runs of modules; the human-readable line is drawn as
//! vector outlines (`glyphs`) in the same path, in the standard positions (EAN/UPC digits
//! between the extended guard bars, others centered).

use crate::glyphs;
use crate::prims::{self, Built};
use crate::spec::{self, ShapeSpec, PREC};
use kurbo::BezPath;
use pixelox_core::prelude::*;

pub const TYPES: &[&str] = &["code128", "ean13", "ean8", "upca", "code39"];

// ---------------------------------------------------------------------------------------
// tables

/// EAN/UPC L (odd parity) patterns; R is the complement, G is R reversed.
const EAN_L: [&str; 10] =
    ["0001101", "0011001", "0010011", "0111101", "0100011", "0110001", "0101111", "0111011", "0110111", "0001011"];

/// EAN-13: which of the six left digits use G (1) for each leading digit.
const EAN13_PARITY: [&str; 10] =
    ["000000", "001011", "001101", "001110", "010011", "011001", "011100", "010101", "010110", "011010"];

/// Code 128 symbols 0..=105 as bar/space widths (bar first, 11 modules each).
const C128: [&str; 106] = [
    "212222", "222122", "222221", "121223", "121322", "131222", "122213", "122312", "132212", "221213", "221312",
    "231212", "112232", "122132", "122231", "113222", "123122", "123221", "223211", "221132", "221231", "213212",
    "223112", "312131", "311222", "321122", "321221", "312212", "322112", "322211", "212123", "212321", "232121",
    "111323", "131123", "131321", "112313", "132113", "132311", "211313", "231113", "231311", "112133", "112331",
    "132131", "113123", "113321", "133121", "313121", "211331", "231131", "213113", "213311", "213131", "311123",
    "311321", "331121", "312113", "312311", "332111", "314111", "221411", "431111", "111224", "111422", "121124",
    "121421", "141122", "141221", "112214", "112412", "122114", "122411", "142112", "142211", "241211", "221114",
    "413111", "241112", "134111", "111242", "121142", "121241", "114212", "124112", "124211", "411212", "421112",
    "421211", "212141", "214121", "412121", "111143", "111341", "131141", "114113", "114311", "411113", "411311",
    "113141", "114131", "311141", "411131", "211412", "211214", "211232",
];
const C128_STOP: &str = "2331112";
const START_A: u32 = 103;
const START_B: u32 = 104;
const START_C: u32 = 105;
const CODE_A: u32 = 101;
const CODE_B: u32 = 100;
const CODE_C: u32 = 99;

/// Code 39: characters and their 9 elements (bar first), W = wide.
const C39_CHARS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ-. $/+%*";
const C39: [&str; 44] = [
    "NNNWWNWNN",
    "WNNWNNNNW",
    "NNWWNNNNW",
    "WNWWNNNNN",
    "NNNWWNNNW",
    "WNNWWNNNN",
    "NNWWWNNNN",
    "NNNWNNWNW",
    "WNNWNNWNN",
    "NNWWNNWNN",
    "WNNNNWNNW",
    "NNWNNWNNW",
    "WNWNNWNNN",
    "NNNNWWNNW",
    "WNNNWWNNN",
    "NNWNWWNNN",
    "NNNNNWWNW",
    "WNNNNWWNN",
    "NNWNNWWNN",
    "NNNNWWWNN",
    "WNNNNNNWW",
    "NNWNNNNWW",
    "WNWNNNNWN",
    "NNNNWNNWW",
    "WNNNWNNWN",
    "NNWNWNNWN",
    "NNNNNNWWW",
    "WNNNNNWWN",
    "NNWNNNWWN",
    "NNNNWNWWN",
    "WWNNNNNNW",
    "NWWNNNNNW",
    "WWWNNNNNN",
    "NWNNWNNNW",
    "WWNNWNNNN",
    "NWWNWNNNN",
    "NWNNNNWNW",
    "WWNNNNWNN",
    "NWWNNNWNN",
    "NWNWNWNNN",
    "NWNWNNNWN",
    "NWNNNWNWN",
    "NNNWNWNWN",
    "NWNNWNWNN",
];

// ---------------------------------------------------------------------------------------
// symbols

/// How the human-readable line is laid out.
#[derive(Debug, Clone, PartialEq)]
pub enum Caption {
    /// EAN/UPC: an optional digit left of the start guard, the left and right halves
    /// centered over their bars, and an optional digit right of the end guard.
    Ean {
        first: Option<char>,
        left: String,
        left_span: (usize, usize),
        right: String,
        right_span: (usize, usize),
        last: Option<char>,
    },
    /// Centered under the bars.
    Centered(String),
}

/// An encoded symbol: modules (bar = true) without quiet zones, which of them are
/// extended guard bars, the caption, and the standard quiet zones in modules.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub modules: Vec<bool>,
    pub guard: Vec<bool>,
    pub caption: Caption,
    pub quiet: (usize, usize),
    /// The data as encoded (with a computed check digit).
    pub data: String,
    pub check: Option<char>,
}

fn bad_data(kind: &str, msg: String, hint: &str) -> Error {
    Error::new("PX6031", msg).with_hint(hint.to_string()).with_context("type", kind)
}

fn push_widths(out: &mut Vec<bool>, widths: &str, wide: usize) {
    for (i, c) in widths.chars().enumerate() {
        let n = match c {
            'N' => 1,
            'W' => wide,
            d => d.to_digit(10).unwrap_or(1) as usize,
        };
        out.extend(std::iter::repeat_n(i % 2 == 0, n));
    }
}

fn push_bits(out: &mut Vec<bool>, bits: &str) {
    out.extend(bits.chars().map(|c| c == '1'));
}

/// Mod-10 check digit with weights 3,1,3,... from the rightmost data digit.
pub fn mod10(digits: &[u32]) -> u32 {
    let s: u32 = digits.iter().rev().enumerate().map(|(i, d)| if i % 2 == 0 { d * 3 } else { *d }).sum();
    (10 - s % 10) % 10
}

/// Parse `n` data digits (plus an optional check digit) and return all n + 1 digits.
fn ean_digits(kind: &str, data: &str, n: usize) -> Result<(Vec<u32>, char)> {
    let digits: Vec<u32> = data.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() != data.chars().count() {
        return Err(bad_data(
            kind,
            format!("{kind} encodes digits only, got '{data}'"),
            "pass digits only (no spaces or dashes), or use --type code128 for text",
        )
        .with_context("data", data));
    }
    if digits.len() != n && digits.len() != n + 1 {
        return Err(bad_data(
            kind,
            format!(
                "{kind} takes {n} digits (the check digit is computed) or {} with the check digit, got {}",
                n + 1,
                digits.len()
            ),
            "check the number of digits",
        )
        .with_context("data", data)
        .with_context("digits", digits.len()));
    }
    let want = mod10(&digits[..n]);
    if digits.len() == n + 1 && digits[n] != want {
        let fixed: String = digits[..n].iter().map(|d| char::from_digit(*d, 10).unwrap()).collect();
        return Err(Error::new(
            "PX6032",
            format!("{kind} check digit of '{data}' is {}, but the data ends in {}", want, digits[n]),
        )
        .with_hint(format!("use {fixed}{want}, or give the first {n} digits ({fixed}) and let pixelox add it"))
        .with_context("data", data)
        .with_context("expected", want)
        .with_context("given", digits[n]));
    }
    let mut all = digits[..n].to_vec();
    all.push(want);
    Ok((all, char::from_digit(want, 10).unwrap()))
}

fn r_code(d: u32) -> String {
    EAN_L[d as usize].chars().map(|c| if c == '0' { '1' } else { '0' }).collect()
}

fn g_code(d: u32) -> String {
    r_code(d).chars().rev().collect()
}

fn digits_str(d: &[u32]) -> String {
    d.iter().map(|v| char::from_digit(*v, 10).unwrap()).collect()
}

/// EAN-13 (also UPC-A, which is EAN-13 with a leading 0).
pub fn ean13(data: &str) -> Result<Symbol> {
    let (d, check) = ean_digits("ean13", data, 12)?;
    Ok(ean13_symbol(&d, check, false))
}

fn ean13_symbol(d: &[u32], check: char, upc: bool) -> Symbol {
    let mut m = Vec::with_capacity(95);
    push_bits(&mut m, "101");
    let parity = EAN13_PARITY[d[0] as usize].as_bytes();
    for i in 0..6 {
        let code = if parity[i] == b'1' { g_code(d[i + 1]) } else { EAN_L[d[i + 1] as usize].to_string() };
        push_bits(&mut m, &code);
    }
    push_bits(&mut m, "01010");
    for &v in &d[7..13] {
        push_bits(&mut m, &r_code(v));
    }
    push_bits(&mut m, "101");
    let mut guard = vec![false; m.len()];
    for r in [0..3, 45..50, 92..95] {
        guard[r].fill(true);
    }
    let all = digits_str(d);
    if upc {
        // UPC-A: the number-system and check digits' bars run as long as the guards
        guard[3..10].fill(true);
        guard[85..92].fill(true);
        Symbol {
            modules: m,
            guard,
            caption: Caption::Ean {
                first: all[1..2].chars().next(),
                left: all[2..7].to_string(),
                left_span: (10, 45),
                right: all[7..12].to_string(),
                right_span: (50, 85),
                last: all[12..13].chars().next(),
            },
            quiet: (9, 9),
            data: all[1..].to_string(),
            check: Some(check),
        }
    } else {
        Symbol {
            modules: m,
            guard,
            caption: Caption::Ean {
                first: all.chars().next(),
                left: all[1..7].to_string(),
                left_span: (3, 45),
                right: all[7..13].to_string(),
                right_span: (50, 92),
                last: None,
            },
            quiet: (11, 7),
            data: all,
            check: Some(check),
        }
    }
}

pub fn upca(data: &str) -> Result<Symbol> {
    let (d, check) = ean_digits("upca", data, 11)?;
    let mut full = vec![0];
    full.extend(d);
    Ok(ean13_symbol(&full, check, true))
}

pub fn ean8(data: &str) -> Result<Symbol> {
    let (d, check) = ean_digits("ean8", data, 7)?;
    let mut m = Vec::with_capacity(67);
    push_bits(&mut m, "101");
    for &v in &d[..4] {
        push_bits(&mut m, EAN_L[v as usize]);
    }
    push_bits(&mut m, "01010");
    for &v in &d[4..] {
        push_bits(&mut m, &r_code(v));
    }
    push_bits(&mut m, "101");
    let mut guard = vec![false; m.len()];
    for r in [0..3, 31..36, 64..67] {
        guard[r].fill(true);
    }
    let all = digits_str(&d);
    Ok(Symbol {
        modules: m,
        guard,
        caption: Caption::Ean {
            first: None,
            left: all[..4].to_string(),
            left_span: (3, 31),
            right: all[4..].to_string(),
            right_span: (36, 64),
            last: None,
        },
        quiet: (7, 7),
        data: all,
        check: Some(check),
    })
}

pub fn code39(data: &str) -> Result<Symbol> {
    let mut m = Vec::new();
    let star = C39[43];
    push_widths(&mut m, star, 3);
    for c in data.chars() {
        let i = C39_CHARS.find(c).filter(|&i| i < 43).ok_or_else(|| {
            let hint = if c.is_ascii_lowercase() {
                "Code 39 has no lower case: write the data in capitals, or use --type code128"
            } else {
                "Code 39 holds 0-9, A-Z, space and - . $ / + %; use --type code128 for other text"
            };
            bad_data("code39", format!("Code 39 cannot encode '{c}' in '{data}'"), hint)
                .with_context("char", c.to_string())
        })?;
        m.push(false); // inter-character gap
        push_widths(&mut m, C39[i], 3);
    }
    m.push(false);
    push_widths(&mut m, star, 3);
    let guard = vec![false; m.len()];
    Ok(Symbol {
        modules: m,
        guard,
        caption: Caption::Centered(data.to_string()),
        quiet: (10, 10),
        data: data.to_string(),
        check: None,
    })
}

/// Code 128 symbol values for `data` (start, data, no checksum), switching code sets
/// the usual way: C for runs of 4+ digits (2+ when the data is all digits), A for control
/// characters, B otherwise.
pub fn code128_values(data: &str) -> Result<Vec<u32>> {
    let chars: Vec<char> = data.chars().collect();
    if let Some(c) = chars.iter().find(|c| (**c as u32) > 127) {
        return Err(bad_data(
            "code128",
            format!("Code 128 encodes ASCII only; '{c}' is not ASCII"),
            "transliterate the text to ASCII, or use a QR code (shape qr) for any text",
        )
        .with_context("char", c.to_string()));
    }
    let is_digit = |i: usize| chars.get(i).is_some_and(|c| c.is_ascii_digit());
    let digit_run = |i: usize| (i..chars.len()).take_while(|&j| is_digit(j)).count();
    let ctrl = |c: char| (c as u32) < 32;
    let lower = |c: char| (32 * 3..128).contains(&(c as u32));
    // which of A/B suits the text from i on: A if a control char comes before a lowercase one
    let pick_ab = |i: usize| -> u32 {
        for &c in &chars[i..] {
            if ctrl(c) {
                return CODE_A;
            }
            if lower(c) {
                return CODE_B;
            }
        }
        CODE_B
    };
    let mut v = Vec::new();
    let mut set;
    let mut i = 0;
    let run0 = digit_run(0);
    if run0 >= 4 || (run0 == chars.len() && run0 >= 2) {
        set = CODE_C;
        v.push(START_C);
    } else {
        set = pick_ab(0);
        v.push(if set == CODE_A { START_A } else { START_B });
    }
    while i < chars.len() {
        if set == CODE_C {
            if digit_run(i) >= 2 {
                let pair = chars[i].to_digit(10).unwrap() * 10 + chars[i + 1].to_digit(10).unwrap();
                v.push(pair);
                i += 2;
                continue;
            }
            set = pick_ab(i);
            v.push(set);
            continue;
        }
        let run = digit_run(i);
        let to_end = i + run == chars.len();
        if run >= 6 || (to_end && run >= 4) {
            if run % 2 == 1 {
                // one digit in the current set so the rest pairs up
                v.push(chars[i] as u32 - 32);
                i += 1;
            }
            set = CODE_C;
            v.push(CODE_C);
            continue;
        }
        let c = chars[i];
        if set == CODE_B && ctrl(c) {
            set = CODE_A;
            v.push(CODE_A);
        } else if set == CODE_A && lower(c) {
            set = CODE_B;
            v.push(CODE_B);
        }
        let code = c as u32;
        v.push(if set == CODE_A && ctrl(c) { code + 64 } else { code - 32 });
        i += 1;
    }
    Ok(v)
}

/// The Code 128 checksum of the values (start included).
pub fn code128_checksum(values: &[u32]) -> u32 {
    let s: u32 = values.iter().enumerate().map(|(i, v)| if i == 0 { *v } else { *v * i as u32 }).sum();
    s % 103
}

pub fn code128(data: &str) -> Result<Symbol> {
    if data.is_empty() {
        return Err(bad_data("code128", "--data is empty".into(), "pass the text to encode"));
    }
    let mut values = code128_values(data)?;
    values.push(code128_checksum(&values));
    let mut m = Vec::new();
    for v in &values {
        push_widths(&mut m, C128[*v as usize], 1);
    }
    push_widths(&mut m, C128_STOP, 1);
    let guard = vec![false; m.len()];
    let shown: String = data.chars().map(|c| if glyphs::drawable(c) { c } else { ' ' }).collect();
    Ok(Symbol {
        modules: m,
        guard,
        caption: Caption::Centered(shown),
        quiet: (10, 10),
        data: data.to_string(),
        check: None,
    })
}

pub fn encode(kind: &str, data: &str) -> Result<Symbol> {
    let data = data.trim();
    if data.is_empty() {
        return Err(Error::new("PX1003", "--data is empty").with_hint("pass what to encode, e.g. --data 4006381333931"));
    }
    match kind {
        "ean13" => ean13(data),
        "ean8" => ean8(data),
        "upca" => upca(data),
        "code39" => code39(data),
        _ => code128(data),
    }
}

// ---------------------------------------------------------------------------------------
// drawing

/// Layout of a drawn symbol (canvas px).
pub struct Drawn {
    pub path: BezPath,
    pub width: f64,
    pub height: f64,
}

/// Draw `sym` with its top-left (quiet zone and padding included) at (`x`, `y`).
pub fn draw(sym: &Symbol, x: f64, y: f64, m: f64, bars_h: f64, text: bool) -> Drawn {
    let pad = 2.0 * m;
    let total = sym.quiet.0 + sym.modules.len() + sym.quiet.1;
    let x0 = x + sym.quiet.0 as f64 * m;
    let top = y + pad;
    let ean = matches!(sym.caption, Caption::Ean { .. });
    let em = 11.0 * m;
    let dh = glyphs::digit_height(em);
    let guard_h = if text && ean { bars_h + 5.0 * m } else { bars_h };
    let mut p = BezPath::new();
    // bars: runs of dark modules with the same height
    let mut i = 0;
    let n = sym.modules.len();
    while i < n {
        if !sym.modules[i] {
            i += 1;
            continue;
        }
        let g = sym.guard[i];
        let mut j = i;
        while j < n && sym.modules[j] && sym.guard[j] == g {
            j += 1;
        }
        let h = if g { guard_h } else { bars_h };
        let (bx, bw) = (x0 + i as f64 * m, (j - i) as f64 * m);
        p.move_to((bx, top));
        p.line_to((bx + bw, top));
        p.line_to((bx + bw, top + h));
        p.line_to((bx, top + h));
        p.close_path();
        i = j;
    }
    let mut bottom = top + guard_h;
    if text {
        match &sym.caption {
            Caption::Ean { first, left, left_span, right, right_span, last } => {
                let base = top + bars_h + m + dh;
                let group = |p: &mut BezPath, s: &str, span: (usize, usize)| {
                    // spread the digits evenly over their bars, one per 7 modules
                    let w = (span.1 - span.0) as f64 * m;
                    let step = w / s.chars().count().max(1) as f64;
                    for (k, c) in s.chars().enumerate() {
                        let cx = x0 + span.0 as f64 * m + step * (k as f64 + 0.5);
                        p.extend(glyphs::char_path_centered(c, cx, base, em).elements().iter().copied());
                    }
                };
                group(&mut p, left, *left_span);
                group(&mut p, right, *right_span);
                let small = if last.is_some() { em * 0.75 } else { em };
                if let Some(c) = first {
                    let cx = x0 - (sym.quiet.0 as f64 * m) / 2.0;
                    p.extend(glyphs::char_path_centered(*c, cx, base, small).elements().iter().copied());
                }
                if let Some(c) = last {
                    let cx = x0 + (n as f64 + sym.quiet.1 as f64 / 2.0) * m;
                    p.extend(glyphs::char_path_centered(*c, cx, base, small).elements().iter().copied());
                }
                bottom = bottom.max(base + 0.25 * dh);
            }
            Caption::Centered(s) => {
                let base = top + bars_h + 1.5 * m + dh;
                let w = glyphs::width(s, em);
                let cx = x0 + n as f64 * m / 2.0;
                p.extend(glyphs::text_path(s, cx - w / 2.0, base, em).elements().iter().copied());
                bottom = base + 0.3 * dh;
            }
        }
    }
    Drawn { path: p, width: total as f64 * m, height: bottom + pad - y }
}

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("shape barcode", Category::Vector, "Add an EAN, UPC, or Code 128/39 barcode", barcode)
        .about(
            "Encodes --data as a linear barcode and adds it as a live vector shape layer: ean13 (12 digits, or 13 \
             with a check digit that must match), ean8 (7 or 8), upca (11 or 12), code128 (any ASCII text; code \
             sets chosen automatically), or code39 (0-9, A-Z, space, - . $ / + %). --width is the whole symbol \
             with its standard quiet zones; --height is the bar height (auto: the standard proportion for EAN/UPC, \
             40% of the width otherwise). The human-readable line (--text, on by default) is drawn as vector \
             outlines in the same layer, EAN/UPC digits between the extended guard bars. --background-color adds a \
             plate (the result is then a group). Wrong check digits fail with the correct digit in the hint.",
        )
        .param(param("type", ParamKind::Choice).choices(TYPES).default("code128").help("Symbology"))
        .param(param("data", ParamKind::Str).required().help("What to encode: digits for EAN/UPC, text for Code 128/39"))
        .param(param("width", ParamKind::Length).default("30%").help("Width of the whole symbol (quiet zones included) in px or % of the canvas width"))
        .param(param("module", ParamKind::Float).auto().default("auto").range("0..1000").help("Narrow bar width in px; overrides --width"))
        .param(param("height", ParamKind::Length).auto().default("auto").help("Bar height in px or % of the canvas height; auto = standard proportion"))
        .param(param("text", ParamKind::Switch).default("on").help("Draw the human-readable line under the bars (on|off)"))
        .param(param("color", ParamKind::Color).default("#000000").help("Bar and text color"))
        .param(param("background-color", ParamKind::Str).default("none").help("Background plate color (CSS color), or none"))
        .param(param("radius", ParamKind::Float).default("0").range("0..10000").help("Corner radius of the background plate in px"))
        .params(prims::place_params("code"))
        .param(param("opacity", ParamKind::Float).range("0..100").default("100").help("Layer opacity 0-100"))
        .param(param("name", ParamKind::Str).help("Layer name (default: Barcode, made unique)"))
        .example("pixelox shape barcode label.pxo --in-place --type ean13 --data 400638133393 --width 360 --anchor br --at -40,-40", "An EAN-13 (check digit added) in the corner")
        .example("pixelox shape barcode ticket.pxo --in-place --type code128 --data 'TKT-2027-0042' --width 50% --height 90 --at 50%,88% --anchor c", "A ticket number")
        .example("pixelox shape barcode box.pxo --in-place --type upca --data 036000291452 --background-color white --radius 8 --width 300", "A UPC-A on a white plate")
        .example("pixelox shape barcode doc.pxo --in-place --type code39 --data 'PX-0042' --module 3 --height 80 --text off", "A Code 39 without the text line")]
}

fn barcode(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let kind = a.choice("type");
    let sym = encode(&kind, &a.str("data").unwrap_or_default())?;
    let total = sym.quiet.0 + sym.modules.len() + sym.quiet.1;
    let m = match a.opt_f64("module")? {
        Some(v) if v > 0.0 => v,
        Some(v) => return Err(Error::new("PX1013", format!("--module must be positive, got {v}"))),
        None => {
            let w = a.px("width", doc.width as f64, doc.dpi())?;
            if !(w > 0.0) {
                return Err(
                    Error::new("PX1013", format!("--width must be positive, got {w}")).with_context("param", "width")
                );
            }
            let m = w / total as f64;
            ctx.resolve_param("module", m);
            m
        }
    };
    let bars_w = sym.modules.len() as f64 * m;
    let bars_h = match a.raw("height").map(str::trim) {
        None | Some("auto") | Some("") => {
            let h = if matches!(sym.caption, Caption::Ean { .. }) { 69.0 * m } else { (0.4 * bars_w).max(24.0 * m) };
            ctx.resolve_param("height", h);
            h
        }
        Some(_) => a.px("height", doc.height as f64, doc.dpi())?,
    };
    if !(bars_h > 0.0) {
        return Err(Error::new("PX1013", "--height must be positive").with_context("param", "height"));
    }
    let text = a.switch("text", true);
    // measure once at the origin to know the box, then draw in place
    let probe = draw(&sym, 0.0, 0.0, m, bars_h, text);
    let (x, y) = prims::place_box(a, doc, probe.width, probe.height)?;
    let drawn = draw(&sym, x, y, m, bars_h, text);
    let color = a.color("color")?;
    let bg = prims::background(a)?;
    prims::check_contrast(doc, ctx, color, bg, [x, y, drawn.width, drawn.height], "barcode");
    if m < 1.0 {
        ctx.warn_ctx(
            "PXW6011",
            format!("bars are {:.2} px per module; below 1 px they blur together and may not scan", m),
            json!({"module_px": spec::round(m, 3)}),
        );
    }
    let geometry = json!({
        "type": "barcode", "symbology": kind, "data": sym.data, "text": text,
        "module": spec::round(m, 4), "bar_height": spec::round(bars_h, 3),
        "box": prims::rect_str(x, y, drawn.width, drawn.height),
    });
    let mut s = ShapeSpec::new(spec::bez_to_d(&drawn.path, PREC), geometry);
    s.fill = Some(color).filter(|c| c.a > 0.0);
    let plate = bg.map(|c| {
        let r = a.f64("radius").unwrap_or(0.0);
        (c, spec::bez_to_d(&spec::rect_path(x, y, drawn.width, drawn.height, [r; 4]), PREC))
    });
    let name = prims::layer_name(a, doc, "Barcode")?;
    let mut r = prims::add(doc, Built { name, spec: s, plate, opacity: a.f64("opacity")? as f32 }, "code");
    r["type"] = json!(kind);
    r["data"] = json!(sym.data);
    if let Some(c) = sym.check {
        r["check_digit"] = json!(c.to_string());
    }
    r["modules"] = json!(total);
    r["module_px"] = json!(spec::round(m, 4));
    r["box"] = json!([x, y, spec::round(drawn.width, 3), spec::round(drawn.height, 3)]);
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(m: &[bool]) -> String {
        m.iter().map(|b| if *b { '1' } else { '0' }).collect()
    }

    /// Decode EAN-13 modules back to digits with the inverse tables (independent of the
    /// encoder's control flow).
    fn decode_ean13(m: &[bool]) -> String {
        let s = bits(m);
        assert_eq!(&s[..3], "101");
        assert_eq!(&s[45..50], "01010");
        assert_eq!(&s[92..], "101");
        let mut parity = String::new();
        let mut out = String::new();
        for k in 0..6 {
            let c = &s[3 + 7 * k..10 + 7 * k];
            if let Some(d) = EAN_L.iter().position(|l| *l == c) {
                parity.push('0');
                out.push(char::from_digit(d as u32, 10).unwrap());
            } else {
                let d = (0..10).find(|d| g_code(*d) == c).expect("an L or G code");
                parity.push('1');
                out.push(char::from_digit(d, 10).unwrap());
            }
        }
        for k in 0..6 {
            let c = &s[50 + 7 * k..57 + 7 * k];
            let d = (0..10).find(|d| r_code(*d) == c).expect("an R code");
            out.push(char::from_digit(d, 10).unwrap());
        }
        let first = EAN13_PARITY.iter().position(|p| *p == parity).expect("a parity pattern");
        format!("{first}{out}")
    }

    #[test]
    fn tables_are_well_formed() {
        for (i, w) in C128.iter().enumerate() {
            let sum: u32 = w.chars().map(|c| c.to_digit(10).unwrap()).sum();
            assert_eq!(sum, 11, "code 128 symbol {i}");
        }
        let mut seen = C128.to_vec();
        seen.sort();
        seen.dedup();
        assert_eq!(seen.len(), 106, "code 128 patterns are unique");
        for p in C39 {
            assert_eq!(p.len(), 9);
            assert_eq!(p.chars().filter(|c| *c == 'W').count(), 3, "Code 39 is 3 wide of 9: {p}");
        }
        let mut s39 = C39.to_vec();
        s39.sort();
        s39.dedup();
        assert_eq!(s39.len(), 44);
        for l in EAN_L {
            assert!(l.starts_with('0') && l.ends_with('1'));
            assert_eq!(l.chars().filter(|c| *c == '1').count() % 2, 1, "L codes have odd parity: {l}");
        }
    }

    #[test]
    fn known_check_digits() {
        // real products: EAN-13, UPC-A, EAN-8
        assert_eq!(ean13("400638133393").unwrap().data, "4006381333931");
        assert_eq!(ean13("590123412345").unwrap().data, "5901234123457");
        assert_eq!(upca("03600029145").unwrap().data, "036000291452");
        assert_eq!(ean8("9638507").unwrap().data, "96385074");
        let e = ean13("4006381333932").unwrap_err();
        assert_eq!(e.code, "PX6032");
        assert!(e.hint.as_deref().unwrap_or("").contains("4006381333931"), "{:?}", e.hint);
        assert_eq!(ean13("40063813339").unwrap_err().code, "PX6031");
        assert_eq!(ean13("40063813339x").unwrap_err().code, "PX6031");
    }

    #[test]
    fn ean13_round_trips_and_has_95_modules() {
        for data in ["4006381333931", "5901234123457", "0000000000000", "9780201379624"] {
            let s = ean13(data).unwrap();
            assert_eq!(s.modules.len(), 95);
            assert_eq!(decode_ean13(&s.modules), data);
        }
        // UPC-A is EAN-13 with a leading zero
        let u = upca("036000291452").unwrap();
        assert_eq!(decode_ean13(&u.modules), "0036000291452");
        assert_eq!(bits(&u.modules[..10]), "1010001101");
    }

    #[test]
    fn ean13_reference_pattern() {
        // 4006381333931 (the classic example): the left half uses parity LGLLGG
        let s = ean13("4006381333931").unwrap();
        assert_eq!(
            bits(&s.modules),
            "101\
             0001101 0100111 0101111 0111101 0001001 0110011\
             01010\
             1000010 1000010 1000010 1110100 1000010 1100110\
             101"
            .replace(' ', "")
        );
    }

    #[test]
    fn ean8_pattern() {
        let s = ean8("96385074").unwrap();
        assert_eq!(s.modules.len(), 67);
        assert_eq!(
            bits(&s.modules),
            "101 0001011 0101111 0111101 0110111 01010 1001110 1110010 1000100 1011100 101".replace(' ', "")
        );
    }

    #[test]
    fn code39_pattern() {
        let s = code39("A1").unwrap();
        // *A1* at 3:1, narrow gaps between characters
        let star = "100010111011101";
        let a = "111010100010111";
        let one = "111010001010111";
        assert_eq!(bits(&s.modules), format!("{star}0{a}0{one}0{star}"));
        assert_eq!(code39("abc").unwrap_err().code, "PX6031");
    }

    #[test]
    fn code128_sets_and_checksum() {
        // "Wikipedia": start B, then B values
        let v = code128_values("Wikipedia").unwrap();
        assert_eq!(v[0], START_B);
        assert_eq!(v[1..], [55, 73, 75, 73, 80, 69, 68, 73, 65]);
        // checksum: 104 + 1*55 + 2*73 + ... (mod 103)
        let manual = (104 + 55 + 2 * 73 + 3 * 75 + 4 * 73 + 5 * 80 + 6 * 69 + 7 * 68 + 8 * 73 + 9 * 65) % 103;
        assert_eq!(code128_checksum(&v), manual);
        // all digits: start C pairs
        assert_eq!(code128_values("123456").unwrap(), vec![START_C, 12, 34, 56]);
        // odd digit count: C for the pairs, B for the last digit
        assert_eq!(code128_values("12345").unwrap(), vec![START_C, 12, 34, CODE_B, 21]);
        // a long digit run in the middle switches to C
        assert_eq!(code128_values("AB123456").unwrap(), vec![START_B, 33, 34, CODE_C, 12, 34, 56]);
        // control characters use A
        assert_eq!(code128_values("A\tB").unwrap(), vec![START_A, 33, 73, 34]);
        assert_eq!(code128("é").unwrap_err().code, "PX6031");
        let s = code128("Hi").unwrap();
        // start B (11) + 2 chars (22) + checksum (11) + stop (13)
        assert_eq!(s.modules.len(), 57);
        assert!(s.modules[0] && !s.modules[s.modules.len() - 3]);
    }
}
