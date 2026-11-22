//! `shape qr`: a QR code as a live vector shape layer.
//!
//! The symbol comes from the `qrcode` crate (ISO/IEC 18004: byte/alphanumeric/numeric
//! segments, the smallest version that fits, the best of the eight masks). The dark modules
//! become one path: `square` traces the outline of each connected region (clean contours,
//! no seams), `rounded` rounds every corner of those contours by half a module, and `dots`
//! draws one circle per module; the finder patterns stay square in every style, because
//! decoders that locate a code by its finder corners (quirc and ports) miss rounded ones.
//! Modules of 3 px or more are whole pixels, so codes stay crisp. A logo gap clears a centered square
//! only when the error correction can absorb it: the codewords it touches must stay within
//! two thirds of the level's correction capacity, and the decoder (`rqrr`) must still read the
//! code with the gap blank, filled, or noisy. Every code is read back by that decoder before
//! the layer is added.

use crate::prims::{self, Built};
use crate::spec::{self, ShapeSpec, PREC};
use kurbo::BezPath;
use pixelox_core::prelude::*;
use qrcode::canvas::Canvas;
use qrcode::{Color, EcLevel, QrCode, Version};

/// Corner-rounding constant for a quarter circle drawn as one cubic.
const K: f64 = 0.552_284_749_830_793_6;

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::doc("shape qr", Category::Vector, "Add a QR code as a shape layer", qr)
        .about(
            "Encodes --data (a URL or any text, UTF-8) as a QR code and adds it as a live vector shape layer: the \
             smallest symbol that fits at --ec (error correction: l 7%, m 15%, q 25%, h 30% of the codewords), \
             --size wide including the --quiet-zone margin. --style square traces clean module outlines, rounded \
             rounds them, dots draws round modules (finder patterns stay square, for every scanner). Modules of 3 \
             px or more are snapped to whole pixels (the result's box is the exact size). --background-color adds a plate under \
             the code (then the result is a group: '<name> background' and '<name> code'). --logo-gap clears a \
             centered square (percent of the code's width) for a logo, only when the error correction can \
             absorb it (h by default); the result's logo_box says where to place the logo. Every code is \
             decoded back before it is added, and a warning flags low contrast against what lies beneath.",
        )
        .param(param("data", ParamKind::Str).required().help("Text or URL to encode (UTF-8)"))
        .param(param("ec", ParamKind::Choice).choices(&["l", "m", "q", "h"]).auto().default("auto").help("Error correction level; auto = m, or h with --logo-gap"))
        .param(param("size", ParamKind::Length).default("25%").help("Side of the code including the quiet zone, in px or % of the shorter canvas side"))
        .param(param("quiet-zone", ParamKind::Int).default("4").range("0..20").help("Quiet zone (empty margin) in modules; 4 is the standard, 2 the practical minimum"))
        .param(param("style", ParamKind::Choice).choices(&["square", "rounded", "dots"]).default("square").help("Module style"))
        .param(param("color", ParamKind::Color).default("#000000").help("Module color (keep it darker than the background)"))
        .param(param("background-color", ParamKind::Str).default("none").help("Background plate color under the code and its quiet zone (CSS color), or none"))
        .param(param("radius", ParamKind::Float).default("0").range("0..10000").help("Corner radius of the background plate in px"))
        .param(param("logo-gap", ParamKind::Float).default("0").range("0..40").help("Clear a centered square this percent of the code's width for a logo (0 = none)"))
        .params(prims::place_params("code"))
        .param(param("opacity", ParamKind::Float).range("0..100").default("100").help("Layer opacity 0-100"))
        .param(param("name", ParamKind::Str).help("Layer name (default: QR code, made unique)"))
        .example("pixelox shape qr flyer.pxo --in-place --data 'https://example.com/tickets' --size 220 --anchor br --at -60,-60", "A ticket link in the bottom-right corner")
        .example("pixelox shape qr card.pxo --in-place --data 'https://example.com' --style dots --color '#1e3a8a' --background-color white --radius 16 --size 30%", "Round dots on a white rounded plate")
        .example("pixelox shape qr poster.pxo --in-place --data 'https://example.com/app' --logo-gap 22 --background-color white --size 300 --at 50%,80% --anchor c", "A code with room for a logo (error correction h)")
        .example("pixelox shape qr doc.pxo --in-place --data 'WIFI:T:WPA;S:Guest;P:secret;;' --ec q --style rounded --size 240", "A Wi-Fi join code with rounded modules")]
}

fn ec_level(s: &str) -> EcLevel {
    match s {
        "l" => EcLevel::L,
        "q" => EcLevel::Q,
        "h" => EcLevel::H,
        _ => EcLevel::M,
    }
}

fn ec_name(l: EcLevel) -> &'static str {
    match l {
        EcLevel::L => "l",
        EcLevel::M => "m",
        EcLevel::Q => "q",
        EcLevel::H => "h",
    }
}

/// A square module grid (row-major, `true` = dark).
#[derive(Clone)]
pub struct Grid {
    pub n: usize,
    pub dark: Vec<bool>,
}

impl Grid {
    fn get(&self, x: i64, y: i64) -> bool {
        x >= 0
            && y >= 0
            && (x as usize) < self.n
            && (y as usize) < self.n
            && self.dark[y as usize * self.n + x as usize]
    }
}

/// Encode `data` at `ec` (QR spec, smallest version).
pub fn encode(data: &str, ec: EcLevel) -> Result<QrCode> {
    QrCode::with_error_correction_level(data.as_bytes(), ec).map_err(|e| {
        let msg = match e {
            qrcode::types::QrError::DataTooLong => {
                format!("{} bytes do not fit in a QR code at error correction {}", data.len(), ec_name(ec))
            }
            other => format!("cannot encode the data: {other}"),
        };
        Error::new("PX6033", msg)
            .with_hint("shorten the data (a short URL), or lower --ec (l holds the most)")
            .with_context("bytes", data.len())
            .with_context("ec", ec_name(ec))
    })
}

fn to_grid(code: &QrCode) -> Grid {
    Grid { n: code.width(), dark: code.to_colors().into_iter().map(|c| c == Color::Dark).collect() }
}

/// Read a module grid back with `rqrr`: the decoded bytes, or None.
pub fn read_back(g: &Grid) -> Option<Vec<u8>> {
    let sg = rqrr::SimpleGrid::from_func(g.n, |x, y| g.dark[y * g.n + x]);
    let mut out = Vec::new();
    rqrr::Grid::new(sg).decode_to(&mut out).ok().map(|_| out)
}

/// The codeword index (in placement order) of every data module (`None` for function
/// patterns and remainder bits), found by drawing index bits through the encoder's own
/// placement (`Canvas::draw_data`).
fn codeword_map(code: &QrCode) -> Vec<Option<usize>> {
    let (version, ec) = (code.version(), code.error_correction_level());
    let w = code.width();
    let total = (0..w * w).filter(|i| !code.is_functional(i % w, i / w)).count() / 8;
    let bits = usize::BITS - total.leading_zeros();
    let mut idx = vec![0usize; w * w];
    let mut seen = vec![false; w * w];
    for b in 0..bits.max(1) {
        let words: Vec<u8> = (0..total).map(|i| if (i >> b) & 1 == 1 { 0xFF } else { 0x00 }).collect();
        // the all-ones pass finds which modules carry codeword bits at all
        let mut c = Canvas::new(version, ec);
        c.draw_all_functional_patterns();
        c.draw_data(&words, &[]);
        let mut ones = Canvas::new(version, ec);
        ones.draw_all_functional_patterns();
        ones.draw_data(&vec![0xFF; total], &[]);
        for y in 0..w {
            for x in 0..w {
                if code.is_functional(x, y) || !ones.get(x as i16, y as i16).is_dark() {
                    continue;
                }
                seen[y * w + x] = true;
                if c.get(x as i16, y as i16).is_dark() {
                    idx[y * w + x] |= 1 << b;
                }
            }
        }
    }
    (0..w * w).map(|i| seen[i].then_some(idx[i])).collect()
}

/// What clearing the centered `gap`×`gap` modules costs: distinct codewords touched.
fn gap_damage(code: &QrCode, map: &[Option<usize>], gap: usize) -> usize {
    let w = code.width();
    let lo = (w - gap) / 2;
    let mut touched: Vec<usize> =
        (lo..lo + gap).flat_map(|y| (lo..lo + gap).filter_map(move |x| map[y * w + x])).collect();
    touched.sort_unstable();
    touched.dedup();
    touched.len()
}

/// A deterministic pseudo-random bit for module (x, y).
fn noise(x: usize, y: usize) -> bool {
    let mut z = (x as u64) << 32 | y as u64;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    (z ^ (z >> 31)) & 1 == 1
}

fn clear_gap(g: &Grid, gap: usize, fill: impl Fn(usize, usize) -> bool) -> Grid {
    let lo = (g.n - gap) / 2;
    let mut out = g.clone();
    for y in lo..lo + gap {
        for x in lo..lo + gap {
            out.dark[y * g.n + x] = fill(x, y);
        }
    }
    out
}

// ---------------------------------------------------------------------------------------
// outlines

/// Trace the outlines of the dark cells: closed polygons (module corners) with the dark
/// side on the right, so outer contours run clockwise on screen and holes counter-clockwise
/// (non-zero filling keeps the holes). Diagonal neighbours stay separate.
pub fn trace(g: &Grid) -> Vec<Vec<(i64, i64)>> {
    let n = g.n as i64;
    // edges: (from, to) for every dark/light boundary, dark on the right
    let mut edges: Vec<((i64, i64), (i64, i64))> = Vec::new();
    for y in 0..n {
        for x in 0..n {
            if !g.get(x, y) {
                continue;
            }
            if !g.get(x, y - 1) {
                edges.push(((x, y), (x + 1, y)));
            }
            if !g.get(x + 1, y) {
                edges.push(((x + 1, y), (x + 1, y + 1)));
            }
            if !g.get(x, y + 1) {
                edges.push(((x + 1, y + 1), (x, y + 1)));
            }
            if !g.get(x - 1, y) {
                edges.push(((x, y + 1), (x, y)));
            }
        }
    }
    let vid = |p: (i64, i64)| (p.1 * (n + 1) + p.0) as usize;
    let mut out_of: Vec<Vec<usize>> = vec![Vec::new(); ((n + 1) * (n + 1)) as usize];
    for (i, e) in edges.iter().enumerate() {
        out_of[vid(e.0)].push(i);
    }
    let mut used = vec![false; edges.len()];
    let mut loops = Vec::new();
    for start in 0..edges.len() {
        if used[start] {
            continue;
        }
        let mut pts = Vec::new();
        let mut cur = start;
        loop {
            used[cur] = true;
            let (a, b) = edges[cur];
            pts.push(a);
            let dir = (b.0 - a.0, b.1 - a.1);
            // prefer a right turn, then straight on, then left: keeps diagonal cells apart
            let right = (-dir.1, dir.0);
            let left = (dir.1, -dir.0);
            let next = [right, dir, left].into_iter().find_map(|want| {
                out_of[vid(b)].iter().copied().find(|&j| {
                    let (c, d) = edges[j];
                    !used[j] && (d.0 - c.0, d.1 - c.1) == want
                })
            });
            match next {
                Some(j) => cur = j,
                None => break,
            }
        }
        // keep only the corners
        let m = pts.len();
        let corners: Vec<(i64, i64)> = (0..m)
            .filter(|&i| {
                let (p, c, q) = (pts[(i + m - 1) % m], pts[i], pts[(i + 1) % m]);
                (c.0 - p.0, c.1 - p.1) != (q.0 - c.0, q.1 - c.1)
            })
            .map(|i| pts[i])
            .collect();
        if corners.len() >= 4 {
            loops.push(corners);
        }
    }
    loops
}

/// Polygons (module units) to a path in canvas px; corners rounded by `r` modules.
fn polys_path(loops: &[Vec<(i64, i64)>], ox: f64, oy: f64, m: f64, r: f64) -> BezPath {
    let mut p = BezPath::new();
    let at = |q: (i64, i64)| (ox + q.0 as f64 * m, oy + q.1 as f64 * m);
    for poly in loops {
        let k = poly.len();
        if r <= 0.0 {
            p.move_to(at(poly[0]));
            for &q in &poly[1..] {
                p.line_to(at(q));
            }
            p.close_path();
            continue;
        }
        let rr = r * m;
        for i in 0..k {
            let (prev, c, next) = (poly[(i + k - 1) % k], poly[i], poly[(i + 1) % k]);
            let unit = |a: (i64, i64), b: (i64, i64)| {
                let (dx, dy) = ((b.0 - a.0) as f64, (b.1 - a.1) as f64);
                let l = dx.hypot(dy);
                (dx / l, dy / l)
            };
            let din = unit(prev, c);
            let dout = unit(c, next);
            let cp = at(c);
            let s = (cp.0 - din.0 * rr, cp.1 - din.1 * rr);
            let e = (cp.0 + dout.0 * rr, cp.1 + dout.1 * rr);
            if i == 0 {
                p.move_to(s);
            } else {
                p.line_to(s);
            }
            p.curve_to((s.0 + din.0 * rr * K, s.1 + din.1 * rr * K), (e.0 - dout.0 * rr * K, e.1 - dout.1 * rr * K), e);
        }
        p.close_path();
    }
    p
}

/// Is (x, y) inside one of the three 7×7 finder patterns?
fn in_finder(n: usize, x: usize, y: usize) -> bool {
    (x < 7 && y < 7) || (x >= n - 7 && y < 7) || (x < 7 && y >= n - 7)
}

/// The three finder patterns ("eyes"): a 7×7 ring around a 3×3 core. They stay square in
/// every style: decoders that locate a code by the corners of its finder patterns (quirc and
/// its ports) lose codes whose eyes are rounded.
fn eyes_path(n: usize, ox: f64, oy: f64, m: f64) -> BezPath {
    let mut p = BezPath::new();
    for (fx, fy) in [(0usize, 0usize), (n - 7, 0), (0, n - 7)] {
        let (x0, y0) = (ox + fx as f64 * m, oy + fy as f64 * m);
        // the ring: a 7×7 square minus a 5×5 one traversed the other way
        p.extend(spec::rect_path(x0, y0, 7.0 * m, 7.0 * m, [0.0; 4]).elements().iter().copied());
        let hole = spec::rect_path(x0 + m, y0 + m, 5.0 * m, 5.0 * m, [0.0; 4]).reverse_subpaths();
        p.extend(hole.elements().iter().copied());
        p.extend(spec::rect_path(x0 + 2.0 * m, y0 + 2.0 * m, 3.0 * m, 3.0 * m, [0.0; 4]).elements().iter().copied());
    }
    p
}

/// Path data for a module grid drawn at (`ox`, `oy`) with modules `m` px wide: the data
/// modules in `style`, the finder patterns square.
pub fn grid_path(g: &Grid, style: &str, ox: f64, oy: f64, m: f64) -> BezPath {
    let n = g.n;
    let mut data = g.clone();
    for y in 0..n {
        for x in 0..n {
            if in_finder(n, x, y) {
                data.dark[y * n + x] = false;
            }
        }
    }
    let mut p = match style {
        "dots" => {
            let mut p = BezPath::new();
            let r = m * 0.45;
            for y in 0..n {
                for x in 0..n {
                    if data.dark[y * n + x] {
                        let (cx, cy) = (ox + (x as f64 + 0.5) * m, oy + (y as f64 + 0.5) * m);
                        p.extend(spec::ellipse_path(cx - r, cy - r, 2.0 * r, 2.0 * r).elements().iter().copied());
                    }
                }
            }
            p
        }
        "rounded" => polys_path(&trace(&data), ox, oy, m, 0.5),
        _ => polys_path(&trace(&data), ox, oy, m, 0.0),
    };
    p.extend(eyes_path(n, ox, oy, m).elements().iter().copied());
    p
}

fn qr(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let data = a.str("data").unwrap_or_default();
    if data.is_empty() {
        return Err(Error::new("PX1003", "--data is empty")
            .with_hint("pass the text or URL, e.g. --data 'https://example.com'"));
    }
    let gap_pct = a.f64("logo-gap")?;
    let ec = match a.raw("ec").map(str::trim) {
        None | Some("auto") | Some("") => {
            let e = if gap_pct > 0.0 { "h" } else { "m" };
            ctx.resolve_param("ec", e);
            e.to_string()
        }
        Some(e) => e.to_string(),
    };
    let level = ec_level(&ec);
    let code = encode(&data, level)?;
    let mut grid = to_grid(&code);
    let n = grid.n;
    if read_back(&grid).as_deref() != Some(data.as_bytes()) {
        return Err(Error::runtime("the QR code did not read back; this is a bug").with_context("data", &data));
    }

    // the logo gap: same parity as the symbol so it sits on module boundaries, centered
    let mut gap = 0usize;
    if gap_pct > 0.0 {
        gap = ((n as f64 * gap_pct / 100.0).round() as usize).max(1);
        if (n - gap) % 2 == 1 {
            gap += 1;
        }
        let map = codeword_map(&code);
        let damage = gap_damage(&code, &map, gap);
        let budget = code.max_allowed_errors() * 2 / 3;
        let fills: [&dyn Fn(usize, usize) -> bool; 3] = [&|_, _| false, &|_, _| true, &noise];
        let reads = fills.iter().all(|f| read_back(&clear_gap(&grid, gap, f)).as_deref() == Some(data.as_bytes()));
        if damage > budget || !reads || gap > n / 2 {
            // the largest gap that would pass at this level, for the hint
            let mut best = 0;
            let mut g2 = if n % 2 == 1 { 1 } else { 2 };
            while g2 < n / 2 && gap_damage(&code, &map, g2) <= budget {
                best = g2;
                g2 += 2;
            }
            let best_pct = (best as f64 / n as f64 * 100.0).floor();
            let hint = if level != EcLevel::H {
                format!("use --ec h (holds the largest gap), or --logo-gap {best_pct} or less at --ec {ec}")
            } else {
                format!(
                    "use --logo-gap {best_pct} or less, or shorten --data so the code has fewer codewords per module"
                )
            };
            return Err(Error::new(
                "PX6034",
                format!(
                    "a {gap_pct}% logo gap ({gap}x{gap} of {n}x{n} modules) touches {damage} codewords; error correction {ec} can safely absorb {budget}"
                ),
            )
            .with_hint(hint)
            .with_context("gap_modules", gap)
            .with_context("codewords_touched", damage)
            .with_context("budget", budget)
            .with_context("max_gap_percent", best_pct));
        }
        grid = clear_gap(&grid, gap, |_, _| false);
    }

    let short = doc.width.min(doc.height) as f64;
    let size = a.px("size", short, doc.dpi())?;
    let quiet = a.usize("quiet-zone")?;
    let modules = n + 2 * quiet;
    if !size.is_finite() || size <= 0.0 {
        return Err(Error::new("PX1013", format!("--size must be positive, got {size}")).with_context("param", "size"));
    }
    // whole-pixel modules (rounded down) keep the code crisp once they are 3 px or more
    let m = match size / modules as f64 {
        m if m >= 3.0 => m.floor(),
        m => m,
    };
    let size = m * modules as f64;
    let (x, y) = prims::place_box(a, doc, size, size)?;
    let (ox, oy) = (x + quiet as f64 * m, y + quiet as f64 * m);
    let style = a.choice("style");
    let bez = grid_path(&grid, &style, ox, oy, m);
    let color = a.color("color")?;
    let bg = prims::background(a)?;
    prims::check_contrast(doc, ctx, color, bg, [x, y, size, size], "QR code");
    if m < 2.0 {
        ctx.warn_ctx(
            "PXW6011",
            format!("QR modules are {:.2} px wide; below 2 px a code may not scan when displayed or printed", m),
            json!({"module_px": spec::round(m, 3), "modules": modules}),
        );
    }
    let version = match code.version() {
        Version::Normal(v) | Version::Micro(v) => v,
    };
    let geometry = json!({
        "type": "qr", "data": data, "ec": ec, "version": version, "modules": n, "quiet": quiet,
        "style": style, "logo_gap": gap, "box": prims::rect_str(x, y, size, size),
    });
    let mut s = ShapeSpec::new(spec::bez_to_d(&bez, PREC), geometry);
    s.fill = Some(color).filter(|c| c.a > 0.0);
    let plate = bg.map(|c| {
        let r = a.f64("radius").unwrap_or(0.0);
        (c, spec::bez_to_d(&spec::rect_path(x, y, size, size, [r; 4]), PREC))
    });
    let name = prims::layer_name(a, doc, "QR code")?;
    let mut r = prims::add(a, doc, Built { name, spec: s, plate, opacity: a.f64("opacity")? as f32 }, "code")?;
    r["version"] = json!(version);
    r["ec"] = json!(ec);
    r["modules"] = json!(n);
    r["module_px"] = json!(spec::round(m, 3));
    r["box"] = json!([x, y, size, size]);
    if gap > 0 {
        let lo = ox + ((n - gap) / 2) as f64 * m;
        let side = gap as f64 * m;
        r["logo_box"] = json!([
            spec::round(lo, 2),
            spec::round(oy + ((n - gap) / 2) as f64 * m, 2),
            spec::round(side, 2),
            spec::round(side, 2)
        ]);
    }
    Ok(r)
}
