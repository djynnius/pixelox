//! `analyze histogram|stats|colors|diff`: measurement for agents checking their own work.
//! Values are reported on the 0–255 scale (like the `adjust` flags) whatever the
//! document's bit depth. Float sums are per-row partials added in row order, so results
//! never depend on the thread count.

use crate::util::{bin_of, clamp01, per_row, quantile, round_to, sum_rows};
use pixelox_core::doc::RecipeStep;
use pixelox_core::op::ParamSpec;
use pixelox_core::prelude::*;
use rand_core::RngCore;
use rayon::prelude::*;
use std::path::PathBuf;

fn layer_param() -> ParamSpec {
    param("layer", ParamKind::Layer)
        .help("Measure one layer (name, #N, @top, @bottom, @active) instead of the flattened composite")
}

pub fn specs() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("analyze histogram", Category::Analyze, "Channel histogram with a text sparkline", histogram)
            .read_only()
            .param(param("channel", ParamKind::Choice).choices(&["rgb", "r", "g", "b", "l", "a"]).default("rgb").help("Channel: rgb (r, g, and b separately), r, g, b, l (Rec. 709 luma), or a (alpha)"))
            .param(param("bins", ParamKind::Int).default("256").range("2..4096").help("Number of equal-width bins over 0..255"))
            .param(layer_param())
            .about("Counts pixel values per bin for the composite (or one layer, placed on the canvas). Color channels count pixels with any opacity; alpha counts every canvas pixel. Also returns the mean, clipped counts, and a 64-column text sparkline per channel.")
            .example("pixelox analyze histogram photo.jpg --json", "RGB histograms")
            .example("pixelox analyze histogram photo.jpg --channel l --bins 32", "Coarse luminance histogram")
            .example("pixelox analyze histogram doc.pxo --layer Logo --channel a --json", "Alpha histogram of one layer"),
        OpSpec::doc("analyze stats", Category::Analyze, "Channel statistics; a 1×1 --rect is an eyedropper", stats)
            .read_only()
            .param(layer_param())
            .param(param("rect", ParamKind::Rect).help("Region X,Y,WxH in canvas coordinates (default: the whole canvas)"))
            .about("Min, max, mean, median, and standard deviation per channel (r, g, b, luma over pixels with any opacity; alpha over all), the share of clipped pixels, alpha coverage, and the mean color as hex. A 1×1 --rect also returns that pixel's hex and RGBA.")
            .example("pixelox analyze stats photo.jpg --json", "Whole-image statistics")
            .example("pixelox analyze stats photo.jpg --rect 120,80,1x1 --json", "Eyedropper: one pixel's color")
            .example("pixelox analyze stats doc.pxo --layer Title --rect 0,0,50%x50%", "Statistics of one layer's top-left quarter"),
        OpSpec::doc("analyze colors", Category::Analyze, "Dominant colors via seeded k-means", colors)
            .export_mode()
            .param(param("k", ParamKind::Int).default("5").range("1..32").help("Number of colors to find"))
            .param(param("space", ParamKind::Choice).choices(&["oklab", "lab", "srgb"]).default("oklab").help("Color space the clusters are formed in"))
            .about("Clusters a deterministic subsample (at most about 20,000 pixels on a regular grid, alpha ≥ 0.5) with k-means++ seeded from --seed; returns each color's hex and share in %, largest first. With -o swatch.svg also writes an SVG swatch.")
            .example("pixelox analyze colors photo.jpg --k 5 --json", "Five dominant colors")
            .example("pixelox analyze colors photo.jpg --k 6 --space lab -o palette.svg", "Palette swatch as SVG")
            .example("pixelox analyze colors poster.pxo --k 3 --seed 7 --json", "Three colors with another seed"),
        OpSpec::custom("analyze diff", Category::Analyze, "Compare two images: SSIM, ΔE00, changed pixels", diff)
            .inputs(Inputs::exactly(2))
            .param(param("threshold", ParamKind::Float).default("1").range("0..100").help("ΔE00 above which a pixel counts as changed (1 ≈ just noticeable)"))
            .param(param("out", ParamKind::Path).help("Write a heatmap image of the differences (PNG, JPEG, ...); -o works too"))
            .about("Perceptual comparison of two images or documents (flattened, then composited on white): SSIM on luma, CIEDE2000 mean/p95/p99/max, and the count and share of pixels above --threshold. Different sizes fail with PX1010. The heatmap shows unchanged pixels as dim gray and changes from red (small) to white (large).")
            .example("pixelox analyze diff before.png after.png --json", "How different are two renders?")
            .example("pixelox analyze diff expected.png actual.png --threshold 2 --out heatmap.png", "Changes above ΔE 2 with a heatmap")
            .example("pixelox analyze diff a.pxo b.pxo --json", "Compare two documents' composites"),
    ]
}

/// The pixels to measure (canvas-sized) and a label for them.
fn source_image(a: &Args, doc: &Document) -> Result<(Image, String)> {
    match a.raw("layer") {
        None => Ok((compose::composite(doc)?, "composite".to_string())),
        Some(r) => {
            let p = doc.resolve(r)?;
            let l = doc.layer(&p);
            if let LayerKind::Adjustment { .. } = l.content {
                return Err(Error::new(
                    "PX2303",
                    format!("layer '{}' is an adjustment layer and has no pixels to measure", l.name),
                )
                .with_hint("measure the composite instead (drop --layer)")
                .with_context("layer", &l.name));
            }
            let img = match compose::render_layer(doc, l)? {
                Some(rd) => rd.to_canvas(doc.width, doc.height),
                None => Image::new(doc.width, doc.height),
            };
            Ok((img, l.name.clone()))
        }
    }
}

fn channel_index(c: &str) -> usize {
    match c {
        "r" => 0,
        "g" => 1,
        "b" => 2,
        "l" => 3,
        _ => 4,
    }
}

/// Value of channel `ci` (0..4 = r, g, b, luma, alpha) of a pixel.
#[inline]
fn chan(p: &[f32], ci: usize) -> f32 {
    match ci {
        3 => color::luma(p[0], p[1], p[2]),
        4 => p[3],
        c => p[c],
    }
}

/// Integer histograms of the requested channels (color channels skip transparent pixels).
fn channel_hists(img: &Image, chans: &[usize], bins: usize) -> Vec<Vec<u64>> {
    let empty = || vec![vec![0u64; bins]; chans.len()];
    if img.is_empty() {
        return empty();
    }
    let w = img.width as usize;
    img.data
        .par_chunks(w * 4)
        .fold(empty, |mut h, row| {
            for p in row.chunks_exact(4) {
                for (k, &ci) in chans.iter().enumerate() {
                    if ci != 4 && p[3] <= 0.0 {
                        continue;
                    }
                    h[k][bin_of(chan(p, ci), bins)] += 1;
                }
            }
            h
        })
        .reduce(empty, |mut a, b| {
            for (x, y) in a.iter_mut().zip(&b) {
                for (u, v) in x.iter_mut().zip(y) {
                    *u += v;
                }
            }
            a
        })
}

const SPARK: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

/// A text sparkline of `counts` in at most `width` columns (blank = empty).
pub fn sparkline(counts: &[u64], width: usize) -> String {
    let n = counts.len();
    let width = width.min(n).max(1);
    let groups: Vec<u64> = (0..width).map(|i| counts[i * n / width..(i + 1) * n / width].iter().sum()).collect();
    let max = groups.iter().copied().max().unwrap_or(0);
    groups
        .iter()
        .map(|&g| if g == 0 || max == 0 { ' ' } else { SPARK[((g as f64 / max as f64) * 7.0).round() as usize] })
        .collect()
}

fn r2(v: f64) -> f64 {
    round_to(v, 2)
}

fn histogram(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let (img, source) = source_image(a, doc)?;
    let bins = a.usize("bins")?;
    let ch = a.choice("channel");
    let keys: Vec<&str> = if ch == "rgb" { vec!["r", "g", "b"] } else { vec![ch.as_str()] };
    let idx: Vec<usize> = keys.iter().map(|k| channel_index(k)).collect();
    let hists = channel_hists(&img, &idx, bins);
    // exact means: per-row sums, added in row order
    let rows = per_row(&img, |_, row| {
        let mut acc = [0.0f64; 10];
        for p in row.chunks_exact(4) {
            for (k, &ci) in idx.iter().enumerate() {
                if ci != 4 && p[3] <= 0.0 {
                    continue;
                }
                acc[k * 2] += chan(p, ci) as f64;
                acc[k * 2 + 1] += 1.0;
            }
        }
        acc
    });
    let sums = sum_rows(&rows);
    let mut counts = Map::new();
    let mut sparks = Map::new();
    let mut means = Map::new();
    let mut clipped = Map::new();
    let mut pixels = 0u64;
    for (k, key) in keys.iter().enumerate() {
        let h = &hists[k];
        let total: u64 = h.iter().sum();
        pixels = pixels.max(total);
        counts.insert(key.to_string(), json!(h));
        sparks.insert(key.to_string(), json!(sparkline(h, 64)));
        means.insert(
            key.to_string(),
            if sums[k * 2 + 1] > 0.0 { json!(r2(sums[k * 2] / sums[k * 2 + 1] * 255.0)) } else { Value::Null },
        );
        let pct = |c: u64| if total > 0 { r2(c as f64 / total as f64 * 100.0) } else { 0.0 };
        clipped.insert(key.to_string(), json!({"shadows_percent": pct(h[0]), "highlights_percent": pct(h[bins - 1])}));
    }
    Ok(json!({
        "source": source,
        "channel": ch,
        "bins": bins,
        "pixels": pixels,
        "counts": counts,
        "sparkline": sparks,
        "mean": means,
        "clipped": clipped,
    }))
}

/// Statistics of one channel from exact sums and a 16-bit histogram (for the median).
fn channel_stats(sum: f64, sum_sq: f64, n: f64, min: f32, max: f32, h: &[u64]) -> Value {
    if n <= 0.0 {
        return Value::Null;
    }
    let mean = sum / n;
    let var = (sum_sq / n - mean * mean).max(0.0);
    let total: u64 = h.iter().sum();
    let pct = |c: u64| if total > 0 { r2(c as f64 / total as f64 * 100.0) } else { 0.0 };
    // values within half an 8-bit step of the ends count as clipped
    let lo_edge = (0.5 / 255.0 * h.len() as f64) as usize;
    let hi_edge = ((254.5 / 255.0) * h.len() as f64) as usize;
    json!({
        "min": r2(min as f64 * 255.0),
        "max": r2(max as f64 * 255.0),
        "mean": r2(mean * 255.0),
        "median": quantile(h, 0.5).map(|m| r2(m as f64 * 255.0)),
        "stddev": r2(var.sqrt() * 255.0),
        "clipped_shadows_percent": pct(h[..=lo_edge].iter().sum()),
        "clipped_highlights_percent": pct(h[hi_edge..].iter().sum()),
    })
}

fn stats(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let (img, source) = source_image(a, doc)?;
    let canvas = doc.canvas();
    let rect = match a.rect("rect")? {
        Some(spec) => {
            let r = spec.resolve(doc.width as f64, doc.height as f64, doc.dpi()).round();
            let clipped = r.intersect(&canvas);
            if clipped.is_empty() {
                return Err(Error::new(
                    "PX3320",
                    format!("--rect {} lies outside the {}x{} canvas", a.str_or("rect", ""), doc.width, doc.height),
                )
                .with_context("rect", r.to_json())
                .with_context("canvas", [doc.width, doc.height]));
            }
            clipped
        }
        None => canvas,
    };
    let img = if rect == canvas { img } else { img.crop(rect) };
    const HB: usize = 65536;
    let chans = [0usize, 1, 2, 3, 4];
    let hists = channel_hists(&img, &chans, HB);
    let rows = per_row(&img, |_, row| {
        // per channel: sum, sum of squares, count, min, max
        let mut acc = [[0.0f64, 0.0, 0.0, f64::INFINITY, f64::NEG_INFINITY]; 5];
        let mut alpha = [0.0f64; 3];
        for p in row.chunks_exact(4) {
            for ci in chans {
                if ci != 4 && p[3] <= 0.0 {
                    continue;
                }
                let v = chan(p, ci) as f64;
                let s = &mut acc[ci];
                s[0] += v;
                s[1] += v * v;
                s[2] += 1.0;
                s[3] = s[3].min(v);
                s[4] = s[4].max(v);
            }
            alpha[0] += p[3] as f64;
            if p[3] >= 0.999 {
                alpha[1] += 1.0;
            }
            if p[3] <= 0.0 {
                alpha[2] += 1.0;
            }
        }
        (acc, alpha)
    });
    let mut tot = [[0.0f64, 0.0, 0.0, f64::INFINITY, f64::NEG_INFINITY]; 5];
    let mut alpha = [0.0f64; 3];
    for (acc, al) in &rows {
        for ci in 0..5 {
            tot[ci][0] += acc[ci][0];
            tot[ci][1] += acc[ci][1];
            tot[ci][2] += acc[ci][2];
            tot[ci][3] = tot[ci][3].min(acc[ci][3]);
            tot[ci][4] = tot[ci][4].max(acc[ci][4]);
        }
        for k in 0..3 {
            alpha[k] += al[k];
        }
    }
    let n = img.pixels().max(1) as f64;
    let mut channels = Map::new();
    for (ci, key) in ["r", "g", "b", "l", "a"].iter().enumerate() {
        let t = &tot[ci];
        channels.insert(key.to_string(), channel_stats(t[0], t[1], t[2], t[3] as f32, t[4] as f32, &hists[ci]));
    }
    let mean_of = |ci: usize| if tot[ci][2] > 0.0 { (tot[ci][0] / tot[ci][2]) as f32 } else { 0.0 };
    let mut v = json!({
        "source": source,
        "rect": rect.to_json(),
        "pixels": img.pixels(),
        "channels": channels,
        "alpha": {
            "coverage_percent": r2(alpha[0] / n * 100.0),
            "opaque_percent": r2(alpha[1] / n * 100.0),
            "transparent_percent": r2(alpha[2] / n * 100.0),
        },
        "mean_hex": if tot[0][2] > 0.0 { json!(Rgba::new(mean_of(0), mean_of(1), mean_of(2), 1.0).to_hex()) } else { Value::Null },
    });
    if rect.w == 1 && rect.h == 1 {
        let p = img.get(0, 0);
        let c = Rgba::from_array(p);
        v["hex"] = json!(c.to_hex());
        v["rgba"] = json!(c.to_u8());
    }
    Ok(v)
}

/// Encoded sRGB → the clustering space.
fn to_space(space: &str, p: [f32; 3]) -> [f32; 3] {
    match space {
        "lab" => {
            let (l, a, b) = color::rgb_to_lab(p[0], p[1], p[2]);
            [l, a, b]
        }
        "srgb" => p,
        _ => {
            let (l, a, b) = color::rgb_to_oklab(p[0], p[1], p[2]);
            [l, a, b]
        }
    }
}

/// CIE L*a*b* (D65) → encoded sRGB (inverse of `color::rgb_to_lab`).
pub fn lab_to_rgb(l: f32, a: f32, b: f32) -> [f32; 3] {
    let fy = (l + 16.0) / 116.0;
    let fx = fy + a / 500.0;
    let fz = fy - b / 200.0;
    let finv = |t: f32| if t * t * t > 0.008_856 { t * t * t } else { (t - 16.0 / 116.0) / 7.787 };
    let (x, y, z) = (0.950_47 * finv(fx), finv(fy), 1.088_83 * finv(fz));
    let r = 3.240_454_2 * x - 1.537_138_5 * y - 0.498_531_4 * z;
    let g = -0.969_266 * x + 1.876_010_8 * y + 0.041_556 * z;
    let bl = 0.055_643_4 * x - 0.204_025_9 * y + 1.057_225_2 * z;
    [r, g, bl].map(|v| clamp01(color::linear_to_srgb(v.max(0.0))))
}

fn from_space(space: &str, p: [f32; 3]) -> [f32; 3] {
    match space {
        "lab" => lab_to_rgb(p[0], p[1], p[2]),
        "srgb" => p.map(clamp01),
        _ => {
            let (r, g, b) = color::oklab_to_rgb(p[0], p[1], p[2]);
            [r, g, b].map(clamp01)
        }
    }
}

#[inline]
fn d2(a: &[f32; 3], b: &[f32; 3]) -> f32 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}

fn nearest(p: &[f32; 3], centers: &[[f32; 3]]) -> (usize, f32) {
    let mut best = (0, f32::INFINITY);
    for (i, c) in centers.iter().enumerate() {
        let d = d2(p, c);
        if d < best.1 {
            best = (i, d);
        }
    }
    best
}

/// A uniform draw in (0, 1] from the op's seeded stream.
fn unit(rng: &mut impl RngCore) -> f64 {
    ((rng.next_u64() >> 11) + 1) as f64 / (1u64 << 53) as f64
}

/// Seeded k-means++ followed by Lloyd iterations; returns centers and member counts.
pub fn kmeans(points: &[[f32; 3]], k: usize, rng: &mut impl RngCore) -> (Vec<[f32; 3]>, Vec<usize>) {
    let n = points.len();
    if n == 0 || k == 0 {
        return (vec![], vec![]);
    }
    let mut centers = vec![points[(rng.next_u64() % n as u64) as usize]];
    while centers.len() < k {
        let dist: Vec<f32> = points.par_iter().map(|p| nearest(p, &centers).1).collect();
        let total: f64 = dist.iter().map(|&d| d as f64).sum();
        if total <= 0.0 {
            break;
        }
        let target = unit(rng) * total;
        let mut acc = 0.0;
        let mut pick = n - 1;
        for (i, &d) in dist.iter().enumerate() {
            acc += d as f64;
            if acc >= target && d > 0.0 {
                pick = i;
                break;
            }
        }
        centers.push(points[pick]);
    }
    let mut labels: Vec<usize> = vec![usize::MAX; n];
    for _ in 0..50 {
        let new: Vec<usize> = points.par_iter().map(|p| nearest(p, &centers).0).collect();
        if new == labels {
            break;
        }
        labels = new;
        let mut sums = vec![[0.0f64; 4]; centers.len()];
        for (p, &l) in points.iter().zip(&labels) {
            let s = &mut sums[l];
            s[0] += p[0] as f64;
            s[1] += p[1] as f64;
            s[2] += p[2] as f64;
            s[3] += 1.0;
        }
        for (c, s) in centers.iter_mut().zip(&sums) {
            if s[3] > 0.0 {
                *c = [(s[0] / s[3]) as f32, (s[1] / s[3]) as f32, (s[2] / s[3]) as f32];
            }
        }
    }
    let mut counts = vec![0usize; centers.len()];
    for &l in &labels {
        counts[l] += 1;
    }
    (centers, counts)
}

fn swatch_svg(colors: &[(String, f64)]) -> String {
    let (w, h, label) = (640.0, 120.0, 28.0);
    let mut s = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{}\" viewBox=\"0 0 {w} {}\">\n",
        h + label,
        h + label
    );
    let mut x = 0.0;
    for (i, (hex, share)) in colors.iter().enumerate() {
        let cw = if i + 1 == colors.len() { w - x } else { (w * share / 100.0).max(0.0) };
        s.push_str(&format!("  <rect x=\"{x:.2}\" y=\"0\" width=\"{cw:.2}\" height=\"{h}\" fill=\"{hex}\"/>\n"));
        s.push_str(&format!(
            "  <text x=\"{:.2}\" y=\"{}\" font-family=\"sans-serif\" font-size=\"11\" text-anchor=\"middle\" fill=\"#333\">{hex} {share}%</text>\n",
            x + cw / 2.0,
            h + 18.0
        ));
        x += cw;
    }
    s.push_str("</svg>\n");
    s
}

fn colors(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let k = a.usize("k")?;
    let space = a.choice("space");
    let out = ctx.globals.output.clone().map(PathBuf::from);
    if let Some(p) = &out {
        if pixelox_core::ctx::format_of(p).as_deref() != Some("svg") {
            return Err(Error::new("PX1106", format!("analyze colors writes an SVG swatch, not {}", p.display()))
                .with_hint("use -o palette.svg, or drop -o and read the colors from --json"));
        }
    }
    let img = compose::composite(doc)?;
    let n = img.pixels();
    let stride = ((n as f64 / 20_000.0).sqrt().ceil() as usize).max(1);
    let mut samples: Vec<[f32; 3]> = Vec::new();
    for y in (0..img.height as usize).step_by(stride) {
        for x in (0..img.width as usize).step_by(stride) {
            let p = img.get(x as u32, y as u32);
            if p[3] >= 0.5 {
                samples.push(to_space(&space, [p[0], p[1], p[2]]));
            }
        }
    }
    let mut rng = ctx.rng();
    let (centers, counts) = kmeans(&samples, k.min(samples.len()), &mut rng);
    let total = samples.len().max(1) as f64;
    let mut found: Vec<(String, [u8; 4], usize)> = centers
        .iter()
        .zip(&counts)
        .filter(|(_, &c)| c > 0)
        .map(|(c, &cnt)| {
            let rgb = from_space(&space, *c);
            let col = Rgba::new(rgb[0], rgb[1], rgb[2], 1.0);
            (col.to_hex(), col.to_u8(), cnt)
        })
        .collect();
    found.sort_by(|x, y| y.2.cmp(&x.2).then(x.0.cmp(&y.0)));
    let list: Vec<Value> = found
        .iter()
        .map(|(hex, u, cnt)| json!({"hex": hex, "rgb": [u[0], u[1], u[2]], "share": r2(*cnt as f64 / total * 100.0)}))
        .collect();
    let mut v = json!({
        "k": found.len(),
        "space": space,
        "pixels_sampled": samples.len(),
        "colors": list,
    });
    if let Some(p) = out {
        let svg =
            swatch_svg(&found.iter().map(|(h, _, c)| (h.clone(), r2(*c as f64 / total * 100.0))).collect::<Vec<_>>());
        let mut params = a.to_json();
        for (key, val) in &ctx.resolved {
            params.insert(key.clone(), val.clone());
        }
        let step = RecipeStep {
            op: "analyze.colors".into(),
            params,
            inputs: ctx.step_inputs.clone(),
            model: None,
            fonts: vec![],
        };
        let mut recipe = doc.recipe.clone();
        recipe.push(step);
        ctx.write_file(&p, svg.as_bytes(), Some("swatch"), Some(recipe))?;
        v["swatch"] = json!(p.display().to_string());
    }
    Ok(v)
}

fn flatten_white(p: &[f32]) -> [f32; 3] {
    let a = p[3];
    [p[0] * a + 1.0 - a, p[1] * a + 1.0 - a, p[2] * a + 1.0 - a]
}

/// k-th smallest of `v` (exact percentile, O(n)).
fn percentile(v: &[f32], q: f64) -> f32 {
    if v.is_empty() {
        return 0.0;
    }
    let mut c = v.to_vec();
    let k = ((q * c.len() as f64).ceil() as usize).clamp(1, c.len()) - 1;
    *c.select_nth_unstable_by(k, |a, b| a.total_cmp(b)).1
}

/// Heat color for a changed pixel: red (small) → yellow → white (large).
fn heat(t: f32) -> [f32; 3] {
    let t = 0.34 + 0.66 * t.clamp(0.0, 1.0);
    let x = t * 3.0;
    [x.min(1.0), (x - 1.0).clamp(0.0, 1.0), (x - 2.0).clamp(0.0, 1.0)]
}

fn diff(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let pa = ctx.input(0)?.to_path_buf();
    let pb = ctx.input(1)?.to_path_buf();
    let da = ctx.load_document(&pa)?;
    let db = ctx.load_document(&pb)?;
    if (da.width, da.height) != (db.width, db.height) {
        return Err(Error::new(
            "PX1010",
            format!(
                "the images differ in size: {} is {}x{}, {} is {}x{}",
                pa.display(),
                da.width,
                da.height,
                pb.display(),
                db.width,
                db.height
            ),
        )
        .with_context("a_size", [da.width, da.height])
        .with_context("b_size", [db.width, db.height])
        .with_hint(format!(
            "resize one first: pixelox transform resize {} --size {}x{} -o resized.png",
            pb.display(),
            da.width,
            da.height
        )));
    }
    let threshold = a.f64("threshold")? as f32;
    let ia = compose::composite(&da)?;
    let ib = compose::composite(&db)?;
    let (w, h) = (ia.width as usize, ia.height as usize);
    let n = w * h;
    // per-pixel ΔE00 of the colors as seen on white; rows computed in parallel, kept in order
    let rows: Vec<(Vec<f32>, f32)> = if n == 0 {
        Vec::new()
    } else {
        ia.data
            .par_chunks(w * 4)
            .zip(ib.data.par_chunks(w * 4))
            .map(|(ra, rb)| {
                let mut out = Vec::with_capacity(w);
                let mut amax = 0f32;
                for (p, q) in ra.chunks_exact(4).zip(rb.chunks_exact(4)) {
                    let (x, y) = (flatten_white(p), flatten_white(q));
                    let (l1, a1, b1) = color::rgb_to_lab(x[0], x[1], x[2]);
                    let (l2, a2, b2) = color::rgb_to_lab(y[0], y[1], y[2]);
                    out.push(if p == q { 0.0 } else { color::delta_e00(l1, a1, b1, l2, a2, b2) });
                    amax = amax.max((p[3] - q[3]).abs());
                }
                (out, amax)
            })
            .collect()
    };
    let mut sum = 0.0f64;
    let mut alpha_max = 0f32;
    let mut de: Vec<f32> = Vec::with_capacity(n);
    for (r, am) in rows {
        let mut s = 0.0f64;
        for &v in &r {
            s += v as f64;
        }
        sum += s;
        alpha_max = alpha_max.max(am);
        de.extend(r);
    }
    let changed = de.iter().filter(|&&v| v > threshold).count();
    let max = de.iter().copied().fold(0f32, f32::max);
    let ssim = if n == 0 { 1.0 } else { kernels::ssim(&ia, &ib) };
    let identical = ia.data == ib.data;
    let mut v = json!({
        "a": pa.display().to_string(),
        "b": pb.display().to_string(),
        "size": [w, h],
        "identical": identical,
        "ssim": round_to(ssim, 6),
        "delta_e00": {
            "mean": round_to(if n > 0 { sum / n as f64 } else { 0.0 }, 4),
            "p95": round_to(percentile(&de, 0.95) as f64, 4),
            "p99": round_to(percentile(&de, 0.99) as f64, 4),
            "max": round_to(max as f64, 4),
        },
        "threshold": threshold,
        "changed_pixels": changed,
        "changed_percent": round_to(if n > 0 { changed as f64 / n as f64 * 100.0 } else { 0.0 }, 4),
        "alpha_max_diff": round_to(alpha_max as f64 * 255.0, 2),
    });
    let out = a.path("out").or_else(|| ctx.globals.output.clone().map(PathBuf::from));
    if let Some(path) = out {
        let scale = (threshold * 10.0).max(10.0);
        let mut img = Image::new(w as u32, h as u32);
        img.data.par_chunks_mut((w * 4).max(4)).enumerate().for_each(|(y, row)| {
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let i = y * w + x;
                let d = de[i];
                let rgb = if d > threshold {
                    heat(d / scale)
                } else {
                    let c = flatten_white(&ia.data[i * 4..i * 4 + 4]);
                    [0.25 * color::luma(c[0], c[1], c[2]); 3]
                };
                px.copy_from_slice(&[rgb[0], rgb[1], rgb[2], 1.0]);
            }
        });
        ctx.write_image(&path, &img, None, &EncodeOptions::for_path(&path), Some("heatmap"), None)?;
        v["heatmap"] = json!(path.display().to_string());
    }
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparkline_shapes() {
        let mut h = vec![0u64; 256];
        h[0] = 10;
        h[255] = 5;
        let s = sparkline(&h, 64);
        assert_eq!(s.chars().count(), 64);
        assert!(s.starts_with('█'));
        assert!(s.ends_with('▅'));
        assert_eq!(sparkline(&[0, 0], 64), "  ");
    }

    #[test]
    fn lab_round_trip() {
        for p in [[0.2f32, 0.5, 0.8], [1.0, 1.0, 1.0], [0.0, 0.0, 0.0], [0.9, 0.1, 0.3]] {
            let (l, a, b) = color::rgb_to_lab(p[0], p[1], p[2]);
            let q = lab_to_rgb(l, a, b);
            for c in 0..3 {
                assert!((p[c] - q[c]).abs() < 2e-3, "{p:?} -> {q:?}");
            }
        }
    }

    #[test]
    fn kmeans_separates_two_clusters() {
        use rand_core::SeedableRng;
        let mut pts = vec![[0.0f32, 0.0, 0.0]; 30];
        pts.extend(vec![[1.0f32, 1.0, 1.0]; 10]);
        let mut rng = rand_chacha::ChaCha12Rng::seed_from_u64(1);
        let (c, n) = kmeans(&pts, 2, &mut rng);
        assert_eq!(c.len(), 2);
        let mut counts = n.clone();
        counts.sort();
        assert_eq!(counts, vec![10, 30]);
        // fewer distinct colors than k: duplicates are never chosen
        let (c, _) = kmeans(&[[0.5f32; 3]; 9], 4, &mut rng);
        assert_eq!(c.len(), 1);
    }

    #[test]
    fn percentiles_are_exact() {
        let v: Vec<f32> = (1..=100).map(|i| i as f32).collect();
        assert_eq!(percentile(&v, 0.95), 95.0);
        assert_eq!(percentile(&v, 0.99), 99.0);
        assert_eq!(percentile(&[], 0.5), 0.0);
    }
}
