//! `trace`: bitmap → SVG (PRD §8.17) with the visioncortex engine (the core of vtracer,
//! MIT/Apache): binary mode traces dark pixels as silhouettes; color mode posterizes to
//! `--colors` with a deterministic median cut and traces stacked color regions. Paths are
//! fitted with corner-preserving splines and written in absolute canvas coordinates.

use crate::spec::fmt_num;
use crate::xml::{WriteOpts, XDoc, XEl, SVG_NS};
use pixelox_core::prelude::*;
use std::path::PathBuf;
use visioncortex::color_clusters::{KeyingAction, Runner, RunnerConfig, HIERARCHICAL_MAX};
use visioncortex::{BinaryImage, Color, ColorImage, CompoundPath, CompoundPathElement, PathSimplifyMode};

/// Fitted segments shorter than this (px) are merged (vtracer's default).
const LENGTH_THRESHOLD: f64 = 4.0;
const MAX_ITERATIONS: usize = 10;
/// Spline splice angle in degrees (vtracer's default).
const SPLICE_DEG: f64 = 45.0;

pub fn spec() -> OpSpec {
    OpSpec::custom("trace", Category::Svg, "Vectorize a bitmap into an SVG", trace)
        .about("Traces an image (or a document's composite) into SVG paths. `--mode binary` traces pixels darker than `--threshold` as black silhouettes (one path per connected shape, holes kept); `--mode color` posterizes to `--colors` and traces stacked regions, largest first. `--speckle` drops small patches, `--corner` sets how sharp a turn must be to stay a corner, `--precision` sets decimals in the path data.")
        .param(param("mode", ParamKind::Choice).choices(&["color", "binary"]).default("color").help("color: posterized multi-color tracing; binary: single-color silhouette"))
        .param(param("colors", ParamKind::Int).default("8").range("2..64").help("Palette size for --mode color"))
        .param(param("threshold", ParamKind::Int).default("128").range("0..256").help("--mode binary: pixels with luma below this (0-255) are traced"))
        .param(param("speckle", ParamKind::Int).default("4").range("0..1024").help("Drop patches smaller than N x N px"))
        .param(param("corner", ParamKind::Int).default("60").range("0..180").help("Turns sharper than this many degrees stay corners (higher = smoother)"))
        .param(param("precision", ParamKind::Int).default("2").range("0..8").help("Decimal places in the path data"))
        .example("pixelox trace scan.png -o logo.svg --mode binary --threshold 128 --speckle 8", "Vectorize a scanned logo")
        .example("pixelox trace poster.jpg -o poster.svg --mode color --colors 12", "Posterize and trace in 12 colors")
        .example("pixelox trace sketch.png -o sketch.svg --mode binary --corner 90 --precision 1", "Smoother curves, shorter numbers")
}

fn pt(out: &mut String, x: f64, y: f64, prec: usize) {
    out.push_str(&fmt_num(x, prec));
    out.push(' ');
    out.push_str(&fmt_num(y, prec));
}

/// Absolute path data for a compound path (each element closed).
pub fn compound_d(cp: &CompoundPath, prec: usize) -> String {
    let mut out = String::new();
    let poly = |pts: Vec<(f64, f64)>, out: &mut String| {
        let mut pts = pts;
        if pts.len() > 1 && pts.first() == pts.last() {
            pts.pop();
        }
        if pts.len() < 3 {
            return;
        }
        for (i, (x, y)) in pts.iter().enumerate() {
            out.push_str(if i == 0 { "M" } else { "L" });
            pt(out, *x, *y, prec);
        }
        out.push('Z');
    };
    for el in cp.iter() {
        match el {
            CompoundPathElement::Spline(s) => {
                let p = &s.points;
                if p.len() < 4 || (p.len() - 1) % 3 != 0 {
                    continue;
                }
                out.push('M');
                pt(&mut out, p[0].x, p[0].y, prec);
                for c in p[1..].chunks(3) {
                    out.push('C');
                    pt(&mut out, c[0].x, c[0].y, prec);
                    out.push(' ');
                    pt(&mut out, c[1].x, c[1].y, prec);
                    out.push(' ');
                    pt(&mut out, c[2].x, c[2].y, prec);
                }
                out.push('Z');
            }
            CompoundPathElement::PathI32(p) => {
                poly(p.path.iter().map(|q| (q.x as f64, q.y as f64)).collect(), &mut out)
            }
            CompoundPathElement::PathF64(p) => poly(p.path.iter().map(|q| (q.x, q.y)).collect(), &mut out),
        }
    }
    out
}

struct Fit {
    speckle_area: usize,
    corner: f64,
    splice: f64,
    prec: usize,
}

/// Dark, opaque pixels as silhouettes.
pub fn trace_binary(img: &Image, threshold: u32, speckle: usize, corner_deg: f64, prec: usize) -> Vec<(String, Rgba)> {
    let f =
        Fit { speckle_area: speckle * speckle, corner: corner_deg.to_radians(), splice: SPLICE_DEG.to_radians(), prec };
    let (w, h) = (img.width as usize, img.height as usize);
    let mut bin = BinaryImage::new_w_h(w, h);
    for y in 0..h {
        for x in 0..w {
            let p = img.get(x as u32, y as u32);
            if p[3] >= 0.5 && (color::luma(p[0], p[1], p[2]) * 255.0) < threshold as f32 {
                bin.set_pixel(x, y, true);
            }
        }
    }
    let clusters = bin.to_clusters(false);
    let mut out = Vec::new();
    for i in 0..clusters.len() {
        let c = clusters.get_cluster(i);
        if c.size() < f.speckle_area.max(1) {
            continue;
        }
        let cp = c.to_compound_path(PathSimplifyMode::Spline, f.corner, LENGTH_THRESHOLD, MAX_ITERATIONS, f.splice);
        let d = compound_d(&cp, f.prec);
        if !d.is_empty() {
            out.push((d, Rgba::BLACK));
        }
    }
    out
}

/// Deterministic median-cut palette over 5-bit color bins of opaque pixels. Returns the
/// palette and, per bin, the palette index.
pub fn median_cut(img: &Image, n: usize) -> (Vec<[u8; 3]>, Vec<u16>) {
    const BINS: usize = 1 << 15;
    let mut cnt = vec![0u64; BINS];
    let mut sums = vec![[0u64; 3]; BINS];
    for p in img.data.chunks(4) {
        if p[3] < 0.5 {
            continue;
        }
        let c = [color::to_u8(p[0]), color::to_u8(p[1]), color::to_u8(p[2])];
        let i = ((c[0] as usize >> 3) << 10) | ((c[1] as usize >> 3) << 5) | (c[2] as usize >> 3);
        cnt[i] += 1;
        for k in 0..3 {
            sums[i][k] += c[k] as u64;
        }
    }
    // (bin, mean color, count)
    let entries: Vec<(usize, [f64; 3], u64)> = (0..BINS)
        .filter(|&i| cnt[i] > 0)
        .map(|i| (i, [0, 1, 2].map(|k| sums[i][k] as f64 / cnt[i] as f64), cnt[i]))
        .collect();
    let mut map = vec![0u16; BINS];
    if entries.is_empty() {
        return (Vec::new(), map);
    }
    let mut boxes: Vec<Vec<(usize, [f64; 3], u64)>> = vec![entries];
    while boxes.len() < n {
        let mut best: Option<(f64, usize, usize)> = None;
        for (bi, b) in boxes.iter().enumerate() {
            if b.len() < 2 {
                continue;
            }
            let count: u64 = b.iter().map(|e| e.2).sum();
            for ch in 0..3 {
                let (lo, hi) = b.iter().fold((f64::MAX, f64::MIN), |(lo, hi), e| (lo.min(e.1[ch]), hi.max(e.1[ch])));
                let score = (hi - lo) * (count as f64).sqrt();
                if score > 0.0 && best.map(|(s, _, _)| score > s).unwrap_or(true) {
                    best = Some((score, bi, ch));
                }
            }
        }
        let Some((_, bi, ch)) = best else { break };
        let mut b = boxes.remove(bi);
        b.sort_by(|x, y| x.1[ch].total_cmp(&y.1[ch]).then(x.0.cmp(&y.0)));
        let total: u64 = b.iter().map(|e| e.2).sum();
        let mut acc = 0;
        let mut cut = 1;
        for (j, e) in b.iter().enumerate() {
            acc += e.2;
            if acc * 2 >= total {
                cut = (j + 1).clamp(1, b.len() - 1);
                break;
            }
        }
        let right = b.split_off(cut);
        boxes.insert(bi, right);
        boxes.insert(bi, b);
    }
    let palette: Vec<[u8; 3]> = boxes
        .iter()
        .map(|b| {
            let count: f64 = b.iter().map(|e| e.2 as f64).sum();
            [0, 1, 2]
                .map(|k| (b.iter().map(|e| e.1[k] * e.2 as f64).sum::<f64>() / count).round().clamp(0.0, 255.0) as u8)
        })
        .collect();
    for (bin, mean, _) in boxes.iter().flatten() {
        let mut best = (f64::MAX, 0usize);
        for (pi, p) in palette.iter().enumerate() {
            let d: f64 = (0..3).map(|k| (mean[k] - p[k] as f64).powi(2)).sum();
            if d < best.0 {
                best = (d, pi);
            }
        }
        map[*bin] = best.1 as u16;
    }
    (palette, map)
}

/// Posterize to `colors` and trace stacked regions (largest first).
pub fn trace_color(
    img: &Image,
    colors: usize,
    speckle: usize,
    corner_deg: f64,
    prec: usize,
) -> (Vec<(String, Rgba)>, Vec<[u8; 3]>) {
    let f =
        Fit { speckle_area: speckle * speckle, corner: corner_deg.to_radians(), splice: SPLICE_DEG.to_radians(), prec };
    let (palette, map) = median_cut(img, colors);
    let (w, h) = (img.width as usize, img.height as usize);
    let candidates = [[255, 0, 255], [0, 255, 0], [0, 0, 255], [255, 255, 0], [0, 255, 255], [255, 0, 0], [1, 254, 3]];
    let key = candidates
        .into_iter()
        .chain((0..=255u8).map(|r| [r, 7, 251]))
        .find(|c| !palette.contains(c))
        .unwrap_or([1, 2, 3]);
    let mut transparent = false;
    let mut px = Vec::with_capacity(w * h * 4);
    for p in img.data.chunks(4) {
        if p[3] < 0.5 || palette.is_empty() {
            transparent = true;
            px.extend_from_slice(&[key[0], key[1], key[2], 255]);
            continue;
        }
        let i = ((color::to_u8(p[0]) as usize >> 3) << 10)
            | ((color::to_u8(p[1]) as usize >> 3) << 5)
            | (color::to_u8(p[2]) as usize >> 3);
        let c = palette[map[i] as usize];
        px.extend_from_slice(&[c[0], c[1], c[2], 255]);
    }
    if palette.is_empty() {
        return (Vec::new(), palette);
    }
    let key_color = if transparent { Color::new(key[0], key[1], key[2]) } else { Color::default() };
    let runner = Runner::new(
        RunnerConfig {
            diagonal: false,
            hierarchical: HIERARCHICAL_MAX,
            batch_size: 25600,
            good_min_area: f.speckle_area,
            good_max_area: w * h,
            is_same_color_a: 0,
            is_same_color_b: 1,
            deepen_diff: 0,
            hollow_neighbours: 1,
            key_color,
            keying_action: KeyingAction::Discard,
        },
        ColorImage { pixels: px, width: w, height: h },
    );
    let clusters = runner.run();
    let view = clusters.view();
    let mut out = Vec::new();
    for &ci in view.clusters_output.iter().rev() {
        let c = view.get_cluster(ci);
        let col = c.residue_color();
        if transparent && (col.r, col.g, col.b) == (key[0], key[1], key[2]) {
            continue;
        }
        let cp = c.to_compound_path(
            &view,
            false,
            PathSimplifyMode::Spline,
            f.corner,
            LENGTH_THRESHOLD,
            MAX_ITERATIONS,
            f.splice,
        );
        let d = compound_d(&cp, f.prec);
        if !d.is_empty() {
            out.push((d, Rgba::from_u8(col.r, col.g, col.b, 255)));
        }
    }
    (out, palette)
}

/// Assemble traced paths into an SVG document.
pub fn paths_svg(w: u32, h: u32, paths: &[(String, Rgba)]) -> XDoc {
    let mut root = XEl::new("svg")
        .attr("xmlns", SVG_NS)
        .attr("width", w.to_string())
        .attr("height", h.to_string())
        .attr("viewBox", format!("0 0 {w} {h}"));
    for (d, c) in paths {
        root.push(XEl::new("path").attr("d", d).attr("fill", c.to_hex()).attr("fill-rule", "evenodd"));
    }
    XDoc::new(root)
}

fn trace(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let input = ctx.input(0)?.to_path_buf();
    if ctx.globals.in_place {
        return Err(Error::new("PX1005", "trace writes a new SVG; use -o out.svg instead of --in-place"));
    }
    let to = match &ctx.globals.output {
        Some(o) => Some(PathBuf::from(o)),
        None if ctx.dry_run() => None,
        None => return Err(Error::new("PX1004", "'trace' writes an SVG; say where").with_hint("pass -o out.svg")),
    };
    ctx.read_aux(&input, "source", None)?;
    let doc = ctx.load_document(&input)?;
    let img = compose::composite(&doc)?;
    let mode = a.choice("mode");
    let speckle = a.usize("speckle")?;
    let corner = a.int("corner")? as f64;
    let prec = a.usize("precision")?;
    let (paths, palette) = if mode == "binary" {
        (trace_binary(&img, a.usize("threshold")? as u32, speckle, corner, prec), Vec::new())
    } else {
        trace_color(&img, a.usize("colors")?, speckle, corner, prec)
    };
    let svg = paths_svg(img.width, img.height, &paths).serialize(WriteOpts::PRETTY);
    if let Some(p) = &to {
        ctx.write_file(p, svg.as_bytes(), None, None)?;
    }
    let colors: Vec<String> = if mode == "binary" {
        vec!["#000000".into()]
    } else {
        palette.iter().map(|c| Rgba::from_u8(c[0], c[1], c[2], 255).to_hex()).collect()
    };
    Ok(json!({
        "path": to.map(|p| p.display().to_string()),
        "mode": mode,
        "paths": paths.len(),
        "colors": colors,
        "size": [img.width, img.height],
        "bytes": svg.len(),
    }))
}
