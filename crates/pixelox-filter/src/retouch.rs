//! Retouching region ops: `retouch erase`, `heal`, `clone`, `dodge`, `burn`, `sharpen`,
//! `blur`, `redeye`.

use crate::util::{self, Buf};
use pixelox_core::color;
use pixelox_core::kernels::{self, Edge};
use pixelox_core::prelude::*;
use pixelox_core::units::RectSpec;
use pixelox_core::ParamSpec;
use rayon::prelude::*;

const CAT: Category = Category::Retouch;

fn mask_required(help: &'static str) -> ParamSpec {
    param("mask", ParamKind::Mask).required().help(help)
}

fn tone_params(spec: OpSpec) -> OpSpec {
    spec.param(common::layer())
        .param(mask_required(
            "Region to work on: a named mask, @sel, @alpha:<layer>, @mask:<layer>, or a grayscale image path",
        ))
        .param(common::invert_mask())
        .param(
            param("exposure", ParamKind::Float)
                .default("0.5")
                .range("0..1")
                .help("Strength 0..1 (1 = two stops on the targeted tones)"),
        )
        .param(
            param("range", ParamKind::Choice)
                .choices(&["midtones", "shadows", "highlights"])
                .default("midtones")
                .help("Tones affected most: shadows, midtones, or highlights"),
        )
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("retouch erase", CAT, "Erase a region to transparency", erase)
            .about("Sets alpha to 0 inside --mask or --rect (soft masks erase partially). With --to-mask the pixels are kept and the region is painted out of the layer mask instead (a mask is created when the layer has none), which works on any layer kind and can be undone by editing the mask.")
            .param(common::layer())
            .param(param("mask", ParamKind::Mask).help("Region to erase: a named mask, @sel, @alpha:<layer>, @mask:<layer>, or a grayscale image path"))
            .param(common::invert_mask())
            .param(param("rect", ParamKind::Rect).help("Rectangle X,Y,WxH to erase, in canvas coordinates (instead of --mask)"))
            .param(param("to-mask", ParamKind::Flag).help("Hide the region through the layer mask instead of clearing pixels"))
            .example("pixelox retouch erase doc.pxo --layer Logo --rect 0,0,200x80 --in-place", "Clear a corner of a layer")
            .example("pixelox retouch erase photo.png -o cutout.png --mask background.png", "Erase the background to transparency")
            .example("pixelox retouch erase doc.pxo --layer Photo --mask @sel --to-mask --in-place", "Hide the selection non-destructively"),
        OpSpec::doc("retouch heal", CAT, "Spot heal: repair small regions seamlessly", heal)
            .about("Repairs each connected region of the hole (a disc at --at of --radius, or --mask): finds the nearby patch whose surroundings best match the hole's border (coarse-to-fine search), copies it in, and corrects it in the gradient domain (a membrane fit to the border difference, push-pull initialized then relaxed) so it blends seamlessly. Best for spots, blemishes, dust, and small objects; large areas need `fill content-aware`.")
            .param(common::layer())
            .param(param("at", ParamKind::Point).help("Center X,Y of a round spot to heal, in canvas coordinates"))
            .param(param("radius", ParamKind::Float).default("10").range("1..1000").help("Radius in px of the spot at --at"))
            .param(param("mask", ParamKind::Mask).help("Region(s) to heal instead of --at: a named mask, @sel, or a grayscale image path"))
            .example("pixelox retouch heal portrait.jpg -o clean.jpg --at 412,230 --radius 9", "Remove a blemish")
            .example("pixelox retouch heal scan.png -o clean.png --mask dust.png", "Heal every dust spot in a mask")
            .example("pixelox retouch heal doc.pxo --layer Photo --mask @sel --in-place", "Heal the selection"),
        OpSpec::doc("retouch clone", CAT, "Clone stamp from one point to another", clone_stamp)
            .about("Stamps a soft round brush of --radius at --to, copying the pixels around --from (sub-pixel offsets are interpolated), at --opacity %. The source is read before the stamp, so overlapping source and destination do not smear.")
            .param(param("from", ParamKind::Point).required().help("Source center X,Y in canvas coordinates"))
            .param(param("to", ParamKind::Point).required().help("Destination center X,Y in canvas coordinates"))
            .param(param("radius", ParamKind::Float).default("20").range("1..4000").help("Brush radius in px (soft edge over the outer half)"))
            .param(param("opacity", ParamKind::Float).default("100").range("0..100").help("Stamp opacity in %"))
            .params(common::region())
            .example("pixelox retouch clone photo.jpg -o fixed.jpg --from 300,200 --to 360,210 --radius 25", "Cover a defect with nearby texture")
            .example("pixelox retouch clone doc.pxo --from 50%,30% --to 60%,30% --radius 40 --opacity 60 --in-place", "Soft partial clone")
            .example("pixelox retouch clone doc.pxo --layer Photo --from 120,80 --to 120,140 --radius 15 --mask wall --in-place", "Clone only onto the wall"),
        tone_params(OpSpec::doc("retouch dodge", CAT, "Dodge (lighten) a region", dodge))
            .about("Lightens the masked region in linear light by up to 2 stops × --exposure, weighted toward the chosen tonal --range (shadows, midtones, or highlights) like Photoshop's Dodge tool.")
            .example("pixelox retouch dodge portrait.jpg -o lit.jpg --mask eyes.png --exposure 0.3", "Brighten the eyes")
            .example("pixelox retouch dodge doc.pxo --mask face --exposure 0.2 --range shadows --in-place", "Open up shadows on a face")
            .example("pixelox retouch dodge doc.pxo --layer Photo --mask @sel --exposure 0.5 --range highlights --in-place", "Make highlights pop"),
        tone_params(OpSpec::doc("retouch burn", CAT, "Burn (darken) a region", burn))
            .about("Darkens the masked region in linear light by up to 2 stops × --exposure, weighted toward the chosen tonal --range, like Photoshop's Burn tool.")
            .example("pixelox retouch burn photo.jpg -o deep.jpg --mask sky.png --exposure 0.4 --range highlights", "Deepen a bright sky")
            .example("pixelox retouch burn doc.pxo --mask edges --exposure 0.25 --in-place", "Darken the edges")
            .example("pixelox retouch burn doc.pxo --layer Photo --mask @sel --exposure 0.6 --range shadows --in-place", "Deepen shadows in the selection"),
        OpSpec::doc("retouch sharpen", CAT, "Sharpen inside a mask", sharpen)
            .about("Brush-equivalent sharpening: an unsharp mask (radius 1 px) of --amount % applied through --mask.")
            .param(common::layer())
            .param(mask_required("Region to sharpen: a named mask, @sel, @alpha:<layer>, @mask:<layer>, or a grayscale image path"))
            .param(common::invert_mask())
            .param(param("amount", ParamKind::Float).default("50").range("0..500").help("Sharpening strength in % (0 = no change)"))
            .example("pixelox retouch sharpen portrait.jpg -o crisp.jpg --mask eyes.png --amount 80", "Crisp eyes")
            .example("pixelox retouch sharpen doc.pxo --mask subject --amount 40 --in-place", "Sharpen only the subject")
            .example("pixelox retouch sharpen doc.pxo --layer Photo --mask @sel --amount 120 --in-place", "Strong local sharpening"),
        OpSpec::doc("retouch blur", CAT, "Blur inside a mask", blur)
            .about("Brush-equivalent blurring: a Gaussian blur of --sigma applied through --mask.")
            .param(common::layer())
            .param(mask_required("Region to blur: a named mask, @sel, @alpha:<layer>, @mask:<layer>, or a grayscale image path"))
            .param(common::invert_mask())
            .param(param("sigma", ParamKind::Float).default("2").range("0..1000").help("Blur radius (Gaussian sigma) in px (0 = no change)"))
            .example("pixelox retouch blur portrait.jpg -o soft.jpg --mask skin.png --sigma 3", "Soften skin")
            .example("pixelox retouch blur doc.pxo --mask plate --sigma 12 --in-place", "Blur a license plate")
            .example("pixelox retouch blur doc.pxo --layer Photo --mask @sel --sigma 6 --in-place", "Blur the selection"),
        OpSpec::doc("retouch redeye", CAT, "Remove red-eye at a point", redeye)
            .about("Desaturates red pupils: within the disc at --at, pixels whose red clearly exceeds green and blue have red pulled to the green/blue average (soft-weighted by redness) by --strength %. With --radius auto the red blob around --at is found (within 48 px) and its centroid and size are used and recorded. --auto (find eyes with face detection) needs the ML build.")
            .params(common::region())
            .param(param("at", ParamKind::Point).help("Pupil center X,Y in canvas coordinates"))
            .param(param("radius", ParamKind::Float).default("auto").auto().range("1..500").help("Pupil radius in px, or auto to measure the red blob"))
            .param(param("auto", ParamKind::Flag).help("Find eyes with face detection (needs the ML build)"))
            .param(param("strength", ParamKind::Float).default("80").range("0..100").help("Correction strength in %"))
            .example("pixelox retouch redeye party.jpg -o fixed.jpg --at 512,300", "Fix one eye, measuring the pupil")
            .example("pixelox retouch redeye party.jpg -o fixed.jpg --at 512,300 --radius 9 --strength 100", "Fix one eye with a given size")
            .example("pixelox retouch redeye doc.pxo --layer Photo --at 40%,35% --in-place", "Fix an eye in a document layer"),
    ]
}

// ---------------------------------------------------------------------------------------
// helpers

/// Anti-aliased coverage of a canvas rectangle.
fn rect_mask(doc: &Document, spec: RectSpec) -> Mask {
    let r = spec.resolve(doc.width as f64, doc.height as f64, doc.dpi());
    let cov = |p: u32, a: f64, len: f64| ((p as f64 + 1.0).min(a + len) - (p as f64).max(a)).clamp(0.0, 1.0) as f32;
    Mask::from_fn(doc.width, doc.height, |x, y| cov(x, r.x, r.w) * cov(y, r.y, r.h))
}

/// Blend straight pixel `dst` toward premultiplied `src` by `t`, premultiplied.
#[inline]
fn mix_premul(dst: &mut [f32], src: &[f32], t: f32) {
    let a = dst[3];
    let mut pm = [dst[0] * a, dst[1] * a, dst[2] * a, a];
    for c in 0..4 {
        pm[c] += (src[c] - pm[c]) * t;
    }
    let na = pm[3].clamp(0.0, 1.0);
    if na > 1e-6 {
        for c in 0..3 {
            dst[c] = (pm[c] / pm[3]).clamp(0.0, 1.0);
        }
    } else {
        dst[..3].fill(0.0);
    }
    dst[3] = na;
}

// ---------------------------------------------------------------------------------------
// erase

fn erase(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let path = target_layer(a, doc)?;
    let rect = a.rect("rect")?;
    let region = match (a.has("mask"), rect) {
        (true, Some(_)) => return Err(Error::new("PX1005", "give --mask or --rect, not both")),
        (false, None) => {
            return Err(Error::new("PX1003", "retouch erase needs --mask or --rect")
                .with_hint("pixelox retouch erase doc.pxo --rect X,Y,WxH --in-place"))
        }
        (false, Some(r)) => {
            let mut m = rect_mask(doc, r);
            if a.flag("invert-mask") {
                m.invert();
            }
            m
        }
        (true, None) => ctx.mask_arg(a, doc)?.expect("--mask present"),
    };
    let (cw, ch) = (doc.width, doc.height);
    let layer = doc.layer_mut(&path);
    let name = layer.name.clone();
    if a.flag("to-mask") {
        let created = layer.mask.is_none();
        let lm = layer.mask.get_or_insert_with(|| LayerMask::new(Mask::filled(cw, ch, 1.0)));
        let mw = lm.mask.width as usize;
        let counts: Vec<usize> = lm
            .mask
            .data
            .par_chunks_mut(mw.max(1))
            .enumerate()
            .map(|(y, row)| {
                let mut n = 0;
                for (x, v) in row.iter_mut().enumerate() {
                    let k = region.at(x as i64, y as i64);
                    if k > 0.0 && *v > 0.0 {
                        *v *= 1.0 - k;
                        n += 1;
                    }
                }
                n
            })
            .collect();
        return Ok(
            json!({"layer": name, "target": "mask", "created_mask": created, "pixels_changed": counts.iter().sum::<usize>()}),
        );
    }
    let (lx, ly) = (layer.x as i64, layer.y as i64);
    let img = layer
        .raster_mut()
        .map_err(|e| e.with_hint("rasterize the layer first, or hide the region non-destructively with --to-mask"))?;
    let w = img.width as usize;
    let counts: Vec<usize> = img
        .data
        .par_chunks_mut((w * 4).max(1))
        .enumerate()
        .map(|(y, row)| {
            let mut n = 0;
            for (x, p) in row.chunks_mut(4).enumerate() {
                let k = region.at(x as i64 + lx, y as i64 + ly);
                if k > 0.0 && p[3] > 0.0 {
                    p[3] *= 1.0 - k;
                    if p[3] <= 0.0 {
                        p.fill(0.0);
                    }
                    n += 1;
                }
            }
            n
        })
        .collect();
    Ok(json!({"layer": name, "target": "pixels", "pixels_changed": counts.iter().sum::<usize>()}))
}

// ---------------------------------------------------------------------------------------
// heal

/// Fill pixels with zero weight from weighted neighbors (Gortler-style push-pull).
fn push_pull(v: &mut [f32], wt: &[f32], w: usize, h: usize) {
    if (w <= 1 && h <= 1) || wt.iter().all(|&x| x >= 1.0) {
        return;
    }
    let (cw, ch) = (w.div_ceil(2), h.div_ceil(2));
    let mut cv = vec![0f32; cw * ch * 4];
    let mut cwt = vec![0f32; cw * ch];
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let j = (y / 2) * cw + x / 2;
            cwt[j] += wt[i];
            for c in 0..4 {
                cv[j * 4 + c] += v[i * 4 + c] * wt[i];
            }
        }
    }
    for j in 0..cw * ch {
        if cwt[j] > 0.0 {
            for c in 0..4 {
                cv[j * 4 + c] /= cwt[j];
            }
            cwt[j] = cwt[j].min(1.0);
        }
    }
    push_pull(&mut cv, &cwt, cw, ch);
    let coarse = Buf { w: cw, h: ch, c: 4, data: cv };
    let mut up = [0f32; 4];
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            if wt[i] >= 1.0 {
                continue;
            }
            util::sample(&coarse, (x as f32 - 0.5) / 2.0, (y as f32 - 0.5) / 2.0, Edge::Clamp, &mut up);
            for c in 0..4 {
                v[i * 4 + c] = wt[i] * v[i * 4 + c] + (1.0 - wt[i]) * up[c];
            }
        }
    }
}

struct Healed {
    bbox: [i64; 4],
    source: Option<(i64, i64)>,
    pixels: usize,
}

/// Summed-area table of a boolean plane.
fn sat(hole: &[bool], w: usize, h: usize) -> Vec<u32> {
    let mut s = vec![0u32; (w + 1) * (h + 1)];
    for y in 0..h {
        let mut row = 0u32;
        for x in 0..w {
            row += hole[y * w + x] as u32;
            s[(y + 1) * (w + 1) + x + 1] = s[y * (w + 1) + x + 1] + row;
        }
    }
    s
}

fn sat_count(s: &[u32], w: usize, x0: usize, y0: usize, x1: usize, y1: usize) -> u32 {
    // inclusive-exclusive box [x0, x1) × [y0, y1)
    let st = w + 1;
    s[y1 * st + x1] + s[y0 * st + x0] - s[y0 * st + x1] - s[y1 * st + x0]
}

/// Heal one 8-connected hole component in `work` (premultiplied RGBA, layer-sized).
fn heal_component(work: &mut Buf, hole: &[bool], holes_sat: &[u32], comp: &[usize]) -> Healed {
    let (w, h) = (work.w, work.h);
    let (mut x0, mut y0, mut x1, mut y1) = (usize::MAX, usize::MAX, 0, 0);
    for &i in comp {
        let (x, y) = (i % w, i / w);
        x0 = x0.min(x);
        y0 = y0.min(y);
        x1 = x1.max(x);
        y1 = y1.max(y);
    }
    let size = (x1 - x0 + 1).max(y1 - y0 + 1);
    let b = ((size as f32 / 4.0).round() as usize).clamp(2, 8);
    // working box E: the component plus a border ring of width b (+1 for neighbors)
    let ex0 = x0.saturating_sub(b + 1);
    let ey0 = y0.saturating_sub(b + 1);
    let ex1 = (x1 + b + 1).min(w - 1);
    let ey1 = (y1 + b + 1).min(h - 1);
    let (ew, eh) = (ex1 - ex0 + 1, ey1 - ey0 + 1);
    let mut cm = Mask::new(ew as u32, eh as u32);
    for &i in comp {
        cm.set((i % w - ex0) as u32, (i / w - ey0) as u32, 1.0);
    }
    let dil = kernels::dilate(&cm, b as f64);
    let mut ring = Vec::new();
    for ly in 0..eh {
        for lx in 0..ew {
            let gi = (ey0 + ly) * w + ex0 + lx;
            if dil.get(lx as u32, ly as u32) > 0.5 && !hole[gi] {
                ring.push(gi);
            }
        }
    }
    let bbox = [x0 as i64, y0 as i64, (x1 - x0 + 1) as i64, (y1 - y0 + 1) as i64];
    if ring.is_empty() {
        return Healed { bbox, source: None, pixels: 0 };
    }
    // ---- source search: the shifted box must be inside the layer and free of holes
    let valid = |ox: i64, oy: i64| -> bool {
        let (sx0, sy0) = (ex0 as i64 + ox, ey0 as i64 + oy);
        let (sx1, sy1) = (ex1 as i64 + ox, ey1 as i64 + oy);
        if (ox == 0 && oy == 0) || sx0 < 0 || sy0 < 0 || sx1 >= w as i64 || sy1 >= h as i64 {
            return false;
        }
        sat_count(holes_sat, w, sx0 as usize, sy0 as usize, sx1 as usize + 1, sy1 as usize + 1) == 0
    };
    let s_max = ((3 * size + 2 * b) as i64).clamp(16, w.max(h) as i64);
    let cost = |ox: i64, oy: i64, samples: &[usize]| -> f64 {
        let off = oy * w as i64 + ox;
        let mut acc = 0f64;
        for &i in samples {
            let j = (i as i64 + off) as usize;
            for c in 0..4 {
                let d = (work.data[i * 4 + c] - work.data[j * 4 + c]) as f64;
                acc += d * d;
            }
        }
        acc / samples.len() as f64 + 1e-4 * ((ox * ox + oy * oy) as f64 / (s_max * s_max) as f64)
    };
    let sub_step = ring.len().div_ceil(400);
    let sub: Vec<usize> = ring.iter().copied().step_by(sub_step).collect();
    let st = (size as i64 / 3).max(1);
    let mut cands = Vec::new();
    let n = s_max / st;
    for j in -n..=n {
        for i in -n..=n {
            let (ox, oy) = (i * st, j * st);
            if valid(ox, oy) {
                cands.push((ox, oy));
            }
        }
    }
    let scored: Vec<f64> = cands.par_iter().map(|&(ox, oy)| cost(ox, oy, &sub)).collect();
    let mut best: Option<((i64, i64), f64)> = None;
    for (k, &c) in scored.iter().enumerate() {
        if best.is_none_or(|(_, bc)| c < bc) {
            best = Some((cands[k], c));
        }
    }
    let source = best.map(|(o, _)| {
        // coarse-to-fine refinement with the full ring
        let (mut o, mut bc) = (o, cost(o.0, o.1, &ring));
        let mut step = st;
        for _ in 0..64 {
            step = (step / 2).max(1);
            let mut improved = false;
            for (dx, dy) in [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)] {
                let c = (o.0 + dx * step, o.1 + dy * step);
                if valid(c.0, c.1) {
                    let cc = cost(c.0, c.1, &ring);
                    if cc < bc {
                        bc = cc;
                        o = c;
                        improved = true;
                    }
                }
            }
            if step == 1 && !improved {
                break;
            }
        }
        o
    });
    // ---- gradient-domain fill: result = source + D, D harmonic with D = work - source on the ring
    let li = |gi: usize| (gi / w - ey0) * ew + (gi % w - ex0);
    let mut d = vec![0f32; ew * eh * 4];
    let mut wt = vec![0f32; ew * eh];
    let off = source.map(|(ox, oy)| oy * w as i64 + ox);
    let src_at = |gi: usize, c: usize| -> f32 {
        match off {
            Some(o) => work.data[(gi as i64 + o) as usize * 4 + c],
            None => 0.0,
        }
    };
    for &gi in &ring {
        let l = li(gi);
        wt[l] = 1.0;
        for c in 0..4 {
            d[l * 4 + c] = work.data[gi * 4 + c] - src_at(gi, c);
        }
    }
    push_pull(&mut d, &wt, ew, eh);
    let mut in_comp = vec![false; ew * eh];
    for &gi in comp {
        in_comp[li(gi)] = true;
    }
    let iters = size.clamp(30, 150);
    let omega = 1.8f32;
    for _ in 0..iters {
        for parity in 0..2 {
            for &gi in comp {
                let (x, y) = (gi % w - ex0, gi / w - ey0);
                if (x + y) % 2 != parity {
                    continue;
                }
                let mut sum = [0f32; 4];
                let mut nn = 0.0;
                for (dx, dy) in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)] {
                    let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                    if nx < 0 || ny < 0 || nx >= ew as i64 || ny >= eh as i64 {
                        continue;
                    }
                    let nl = ny as usize * ew + nx as usize;
                    for c in 0..4 {
                        sum[c] += d[nl * 4 + c];
                    }
                    nn += 1.0;
                }
                if nn > 0.0 {
                    let l = y * ew + x;
                    for c in 0..4 {
                        d[l * 4 + c] += omega * (sum[c] / nn - d[l * 4 + c]);
                    }
                }
            }
        }
    }
    let srcs: Vec<[f32; 4]> =
        comp.iter().map(|&gi| [src_at(gi, 0), src_at(gi, 1), src_at(gi, 2), src_at(gi, 3)]).collect();
    for (&gi, s) in comp.iter().zip(&srcs) {
        let l = li(gi);
        let a = (s[3] + d[l * 4 + 3]).clamp(0.0, 1.0);
        for c in 0..3 {
            work.data[gi * 4 + c] = (s[c] + d[l * 4 + c]).clamp(0.0, a);
        }
        work.data[gi * 4 + 3] = a;
    }
    Healed { bbox, source, pixels: comp.len() }
}

/// 8-connected components of `hole`, in raster order of their first pixel.
fn components(hole: &[bool], w: usize, h: usize) -> Vec<Vec<usize>> {
    let mut label = vec![false; hole.len()];
    let mut out = Vec::new();
    for start in 0..hole.len() {
        if !hole[start] || label[start] {
            continue;
        }
        let mut comp = vec![start];
        label[start] = true;
        let mut k = 0;
        while k < comp.len() {
            let i = comp[k];
            k += 1;
            let (x, y) = ((i % w) as i64, (i / w) as i64);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let (nx, ny) = (x + dx, y + dy);
                    if nx < 0 || ny < 0 || nx >= w as i64 || ny >= h as i64 {
                        continue;
                    }
                    let j = ny as usize * w + nx as usize;
                    if hole[j] && !label[j] {
                        label[j] = true;
                        comp.push(j);
                    }
                }
            }
        }
        comp.sort_unstable();
        out.push(comp);
    }
    out
}

fn heal(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let at = util::canvas_point(a, "at", doc)?;
    let region = match (at, a.has("mask")) {
        (Some(_), true) => return Err(Error::new("PX1005", "give --at or --mask, not both")),
        (None, false) => {
            return Err(Error::new("PX1003", "retouch heal needs --at X,Y (with --radius) or --mask")
                .with_hint("pixelox retouch heal photo.jpg -o out.jpg --at 120,80 --radius 8"))
        }
        (Some((cx, cy)), false) => {
            let r = a.f64("radius")?;
            Mask::from_fn(doc.width, doc.height, |x, y| {
                let d = ((x as f64 + 0.5 - cx).powi(2) + (y as f64 + 0.5 - cy).powi(2)).sqrt();
                (r + 0.5 - d).clamp(0.0, 1.0) as f32
            })
        }
        (None, true) => ctx.mask_arg(a, doc)?.expect("--mask present"),
    };
    let path = target_layer(a, doc)?;
    let layer = doc.layer(&path);
    if !layer.visible {
        ctx.warn("PXW2001", format!("layer '{}' is hidden", layer.name));
    }
    let (lx, ly, name) = (layer.x as i64, layer.y as i64, layer.name.clone());
    let img = layer.raster()?;
    let (w, h) = (img.width as usize, img.height as usize);
    let soft: Vec<f32> = (0..w * h).map(|i| region.at((i % w) as i64 + lx, (i / w) as i64 + ly)).collect();
    let hole: Vec<bool> = soft.iter().map(|&v| v > 0.01).collect();
    if !hole.iter().any(|&b| b) {
        return Err(
            Error::new("PX3320", "the region to heal covers no pixels of the layer").with_context("layer", &name)
        );
    }
    if hole.iter().all(|&b| b) {
        return Err(Error::new("PX3320", "the region to heal covers the whole layer; nothing is left to sample")
            .with_hint("heal smaller regions, or use fill content-aware for large areas")
            .with_context("layer", &name));
    }
    let orig = Buf::premul(img);
    let mut work = orig.clone();
    let holes_sat = sat(&hole, w, h);
    let mut regions = Vec::new();
    let mut changed = 0;
    for comp in components(&hole, w, h) {
        let r = heal_component(&mut work, &hole, &holes_sat, &comp);
        changed += r.pixels;
        regions.push(json!({
            "bbox": [r.bbox[0] + lx, r.bbox[1] + ly, r.bbox[2], r.bbox[3]],
            "source_offset": r.source.map(|(x, y)| json!([x, y])).unwrap_or(Value::Null),
            "pixels": r.pixels,
        }));
    }
    let img = doc.layer_mut(&path).raster_mut()?;
    img.data.par_chunks_mut(4).enumerate().for_each(|(i, p)| {
        let t = soft[i];
        if t > 0.0 {
            mix_premul(p, &work.data[i * 4..i * 4 + 4], t.min(1.0));
        }
    });
    Ok(json!({"layer": name, "regions": regions, "pixels_changed": changed}))
}

// ---------------------------------------------------------------------------------------
// clone

fn clone_stamp(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let from = util::canvas_point(a, "from", doc)?.ok_or_else(|| Error::new("PX1003", "missing --from"))?;
    let to = util::canvas_point(a, "to", doc)?.ok_or_else(|| Error::new("PX1003", "missing --to"))?;
    let radius = a.f64("radius")?;
    let opacity = (a.f64("opacity")? / 100.0) as f32;
    let (lx, ly) = util::layer_offset(a, doc)?;
    let (dx, dy) = (from.0 - to.0, from.1 - to.1);
    let (tx, ty) = (to.0 - lx as f64, to.1 - ly as f64);
    let mut changed = 0usize;
    let v = apply_to_raster(a, doc, ctx, |img, _| {
        let mut out = img.clone();
        if opacity <= 0.0 || img.is_empty() {
            return Ok(out);
        }
        let src = Buf::premul(img);
        let (w, h) = (img.width as i64, img.height as i64);
        let ya = ((ty - radius).floor() as i64).clamp(0, h);
        let yb = ((ty + radius).ceil() as i64 + 1).clamp(0, h);
        let xa = ((tx - radius).floor() as i64).clamp(0, w);
        let xb = ((tx + radius).ceil() as i64 + 1).clamp(0, w);
        let mut s = [0f32; 4];
        for y in ya..yb {
            for x in xa..xb {
                let d = ((x as f64 + 0.5 - tx).powi(2) + (y as f64 + 0.5 - ty).powi(2)).sqrt();
                let wgt = (1.0 - util::smoothstep(0.5 * radius as f32, radius as f32, d as f32)) * opacity;
                if wgt <= 0.0 {
                    continue;
                }
                util::sample(&src, (x as f64 + dx) as f32, (y as f64 + dy) as f32, Edge::Zero, &mut s);
                let i = out.idx(x as u32, y as u32);
                mix_premul(&mut out.data[i..i + 4], &s, wgt);
                changed += 1;
            }
        }
        Ok(out)
    })?;
    Ok(util::merge(v, json!({"pixels_changed": changed, "offset": [dx, dy]})))
}

// ---------------------------------------------------------------------------------------
// dodge / burn / sharpen / blur

fn tone(a: &Args, doc: &mut Document, ctx: &mut OpCtx, sign: f32) -> Result<Value> {
    let exposure = a.f64("exposure")? as f32;
    let range = a.choice("range");
    let weight = move |l: f32| -> f32 {
        let l = l.clamp(0.0, 1.0);
        match range.as_str() {
            "shadows" => (1.0 - l) * (1.0 - l),
            "highlights" => l * l,
            _ => 4.0 * l * (1.0 - l),
        }
    };
    apply_to_raster(a, doc, ctx, |img, _| {
        let mut out = img.clone();
        if exposure <= 0.0 {
            return Ok(out);
        }
        let tf = util::Transfer::get();
        out.map_pixels(|p| {
            let wv = weight(color::luma(p[0], p[1], p[2]));
            if wv <= 0.0 {
                return;
            }
            let gain = (sign * 2.0 * exposure * wv).exp2();
            for v in p.iter_mut().take(3) {
                *v = tf.to_srgb(tf.to_linear(*v) * gain);
            }
        });
        Ok(out)
    })
}

fn dodge(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    tone(a, doc, ctx, 1.0)
}

fn burn(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    tone(a, doc, ctx, -1.0)
}

fn sharpen(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let amount = (a.f64("amount")? / 100.0) as f32;
    apply_to_raster(a, doc, ctx, |img, _| Ok(crate::sharpen::unsharp_image(img, amount, 1.0, 0.0)))
}

fn blur(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let sigma = a.f64("sigma")?;
    apply_to_raster(a, doc, ctx, |img, _| {
        if sigma <= 0.0 {
            return Ok(img.clone());
        }
        Ok(crate::kern::gaussian_image(img, sigma))
    })
}

// ---------------------------------------------------------------------------------------
// red-eye

/// Relative redness: how much red exceeds green and blue, as a fraction of red.
#[inline]
fn redness(p: &[f32]) -> f32 {
    if p[3] <= 0.0 {
        return 0.0;
    }
    (p[0] - p[1].max(p[2])) / p[0].max(1e-3)
}

/// Find the red blob nearest to (cx, cy) within 48 px: returns its centroid and radius.
fn find_pupil(img: &Image, cx: f64, cy: f64) -> Option<(f64, f64, f64)> {
    const WIN: i64 = 48;
    let (w, h) = (img.width as i64, img.height as i64);
    let (x0, x1) = ((cx as i64 - WIN).max(0), (cx as i64 + WIN).min(w - 1));
    let (y0, y1) = ((cy as i64 - WIN).max(0), (cy as i64 + WIN).min(h - 1));
    if x0 > x1 || y0 > y1 {
        return None;
    }
    let red = |x: i64, y: i64| {
        let p = img.get(x as u32, y as u32);
        redness(&p) > 0.4 && p[0] > 0.2
    };
    let mut seed: Option<(i64, i64, f64)> = None;
    for y in y0..=y1 {
        for x in x0..=x1 {
            if red(x, y) {
                let d = (x as f64 + 0.5 - cx).powi(2) + (y as f64 + 0.5 - cy).powi(2);
                if seed.is_none_or(|s| d < s.2) {
                    seed = Some((x, y, d));
                }
            }
        }
    }
    let (sx, sy, _) = seed?;
    let ww = (x1 - x0 + 1) as usize;
    let mut seen = vec![false; ww * (y1 - y0 + 1) as usize];
    let mut stack = vec![(sx, sy)];
    seen[((sy - y0) as usize) * ww + (sx - x0) as usize] = true;
    let (mut n, mut mx, mut my) = (0f64, 0f64, 0f64);
    while let Some((x, y)) = stack.pop() {
        n += 1.0;
        mx += x as f64 + 0.5;
        my += y as f64 + 0.5;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < x0 || ny < y0 || nx > x1 || ny > y1 {
                continue;
            }
            let k = ((ny - y0) as usize) * ww + (nx - x0) as usize;
            if !seen[k] && red(nx, ny) {
                seen[k] = true;
                stack.push((nx, ny));
            }
        }
    }
    Some((mx / n, my / n, (n / std::f64::consts::PI).sqrt() + 1.5))
}

fn redeye(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    if a.flag("auto") {
        if a.has("at") {
            return Err(Error::new("PX1005", "give --auto or --at, not both"));
        }
        return Err(Error::new("PX1510", "retouch redeye --auto needs face detection, which is not in this build")
            .with_hint("give the pupil position instead: pixelox retouch redeye photo.jpg -o out.jpg --at X,Y"));
    }
    let at = util::canvas_point(a, "at", doc)?.ok_or_else(|| {
        Error::new("PX1003", "retouch redeye needs --at X,Y (or --auto with the ML build)")
            .with_hint("pixelox retouch redeye photo.jpg -o out.jpg --at 512,300")
    })?;
    let strength = (a.f64("strength")? / 100.0) as f32;
    let path = target_layer(a, doc)?;
    let (lx, ly) = (doc.layer(&path).x as f64, doc.layer(&path).y as f64);
    let (mut cx, mut cy) = (at.0 - lx, at.1 - ly);
    let radius = if a.is_auto("radius") {
        let img = doc.layer(&path).raster()?;
        let (px, py, r) = find_pupil(img, cx, cy).ok_or_else(|| {
            Error::new("PX3320", format!("no red pupil found within 48 px of {},{}", at.0, at.1))
                .with_hint("check the point, or give the size explicitly with --radius")
                .with_context("at", [at.0, at.1])
        })?;
        cx = px;
        cy = py;
        let r = (r * 100.0).round() / 100.0;
        let round2 = |v: f64| (v * 100.0).round() / 100.0;
        ctx.resolve_param("at", format!("{},{}", round2(cx + lx), round2(cy + ly)));
        ctx.resolve_param("radius", r.to_string());
        r
    } else {
        a.f64("radius")?
    };
    let mut changed = 0usize;
    let v = apply_to_raster(a, doc, ctx, |img, _| {
        let mut out = img.clone();
        let (w, h) = (img.width as i64, img.height as i64);
        let ya = ((cy - radius - 1.0).floor() as i64).clamp(0, h);
        let yb = ((cy + radius + 1.0).ceil() as i64 + 1).clamp(0, h);
        let xa = ((cx - radius - 1.0).floor() as i64).clamp(0, w);
        let xb = ((cx + radius + 1.0).ceil() as i64 + 1).clamp(0, w);
        for y in ya..yb {
            for x in xa..xb {
                let d = ((x as f64 + 0.5 - cx).powi(2) + (y as f64 + 0.5 - cy).powi(2)).sqrt();
                let disc = (radius + 0.5 - d).clamp(0.0, 1.0) as f32;
                let i = out.idx(x as u32, y as u32);
                let p = &mut out.data[i..i + 4];
                let k = disc * strength * util::smoothstep(0.25, 0.5, redness(p));
                if k <= 0.0 {
                    continue;
                }
                let target = 0.5 * (p[1] + p[2]);
                p[0] += (target - p[0]) * k;
                changed += 1;
            }
        }
        Ok(out)
    })?;
    Ok(util::merge(v, json!({"center": [cx + lx, cy + ly], "radius": radius, "pixels_changed": changed})))
}

// ---------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing::{self, find, max_diff, raster, run};

    fn op(n: &str) -> OpSpec {
        find(&ops(), n)
    }

    #[test]
    fn erase_rect_mask_and_to_mask() {
        let spec = op("retouch erase");
        let mut doc = testing::doc_from(testing::solid(20, 10, Rgba::WHITE));
        let v = run(&spec, &[("rect", "0,0,5x10")], &mut doc).unwrap();
        assert_eq!(v["pixels_changed"], 50);
        let r = raster(&doc, "Background");
        assert_eq!(r.get(2, 2)[3], 0.0);
        assert_eq!(r.get(6, 2)[3], 1.0);
        // inverted rect erases the rest
        run(&spec, &[("rect", "0,0,5x10"), ("invert-mask", "true")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").alpha_bbox(0.0), None);
        // to-mask keeps pixels and creates a layer mask
        let mut doc = testing::doc_from(testing::solid(20, 10, Rgba::WHITE));
        doc.put_mask("left", testing::half_mask(20, 10), false).unwrap();
        let v = run(&spec, &[("mask", "left"), ("to-mask", "true")], &mut doc).unwrap();
        assert_eq!(v["created_mask"], true);
        let l = &doc.layers[0];
        assert_eq!(l.raster().unwrap().get(2, 2)[3], 1.0);
        let lm = l.mask.as_ref().unwrap();
        assert_eq!(lm.mask.get(2, 2), 0.0);
        assert_eq!(lm.mask.get(15, 2), 1.0);
        // errors: neither or both
        assert_eq!(run(&spec, &[], &mut doc).unwrap_err().code, "PX1003");
        assert_eq!(run(&spec, &[("mask", "left"), ("rect", "0,0,2x2")], &mut doc).unwrap_err().code, "PX1005");
        // transparent layer: nothing to change
        let mut doc = testing::doc_from(Image::new(4, 4));
        let v = run(&spec, &[("rect", "0,0,4x4")], &mut doc).unwrap();
        assert_eq!(v["pixels_changed"], 0);
    }

    #[test]
    fn heal_removes_a_spot_on_texture() {
        let spec = op("retouch heal");
        let base = Image::from_fn(64, 64, |x, y| {
            let tau = std::f32::consts::TAU;
            let v = 0.4 + 0.1 * ((x as f32 * tau / 10.0).sin() * (y as f32 * tau / 10.0).cos());
            [v, v * 0.9, v * 0.8, 1.0]
        });
        let mut spotted = base.clone();
        for y in 28..36 {
            for x in 28..36 {
                spotted.set(x, y, [1.0, 0.0, 0.0, 1.0]);
            }
        }
        let mut doc = testing::doc_from(spotted.clone());
        let v = run(&spec, &[("at", "32,32"), ("radius", "6")], &mut doc).unwrap();
        assert_eq!(v["regions"].as_array().unwrap().len(), 1);
        assert!(v["regions"][0]["source_offset"].is_array());
        let r = raster(&doc, "Background");
        let err = |i: &Image| {
            (28..36)
                .flat_map(|y| (28..36).map(move |x| (x, y)))
                .map(|(x, y)| (i.get(x, y)[0] - base.get(x, y)[0]).abs())
                .fold(0.0, f32::max)
        };
        assert!(err(r) < 0.15, "healed: {}", err(r));
        assert_eq!(r.get(5, 5), spotted.get(5, 5), "outside untouched");
        // mask with two spots heals both
        let mut doc = testing::doc_from(spotted.clone());
        let m = Mask::from_fn(64, 64, |x, y| {
            if (28..36).contains(&x) && (28..36).contains(&y) || (5..8).contains(&x) && (5..8).contains(&y) {
                1.0
            } else {
                0.0
            }
        });
        doc.put_mask("spots", m, false).unwrap();
        let v = run(&spec, &[("mask", "spots")], &mut doc).unwrap();
        assert_eq!(v["regions"].as_array().unwrap().len(), 2);
        assert!(err(raster(&doc, "Background")) < 0.15);
        // errors
        assert_eq!(run(&spec, &[], &mut doc).unwrap_err().code, "PX1003");
        doc.put_mask("all", Mask::filled(64, 64, 1.0), false).unwrap();
        assert_eq!(run(&spec, &[("mask", "all")], &mut doc).unwrap_err().code, "PX3320");
        let mut doc = testing::doc_from(testing::solid(1, 1, Rgba::WHITE));
        assert_eq!(run(&spec, &[("at", "0,0"), ("radius", "3")], &mut doc).unwrap_err().code, "PX3320");
    }

    #[test]
    fn clone_copies_source() {
        let spec = op("retouch clone");
        let src = Image::from_fn(40, 20, |x, _| if x < 20 { [1.0, 0.0, 0.0, 1.0] } else { [0.0, 0.0, 1.0, 1.0] });
        let mut doc = testing::doc_from(src.clone());
        let v = run(&spec, &[("from", "10,10"), ("to", "30,10"), ("radius", "6")], &mut doc).unwrap();
        assert!(v["pixels_changed"].as_u64().unwrap() > 50);
        let r = raster(&doc, "Background");
        assert!((r.get(30, 10)[0] - 1.0).abs() < 1e-5, "red stamped onto blue");
        assert_eq!(r.get(38, 10), src.get(38, 10));
        let mut doc = testing::doc_from(src.clone());
        run(&spec, &[("from", "10,10"), ("to", "30,10"), ("opacity", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &src), 0.0);
        // stamping onto a transparent layer from outside it copies nothing visible
        let mut doc = testing::doc_from(Image::new(10, 10));
        run(&spec, &[("from", "100,100"), ("to", "5,5"), ("radius", "3")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").alpha_bbox(0.0), None);
    }

    #[test]
    fn dodge_burn_ranges_and_mask() {
        let gray = testing::solid(10, 10, Rgba::new(0.5, 0.5, 0.5, 1.0));
        let mut doc = testing::doc_from(gray.clone());
        doc.put_mask("left", testing::half_mask(10, 10), false).unwrap();
        run(&op("retouch dodge"), &[("mask", "left"), ("exposure", "0.5")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.get(2, 2)[0] > 0.6 && r.get(8, 2)[0] == 0.5);
        let mut doc = testing::doc_from(gray.clone());
        doc.put_mask("all", Mask::filled(10, 10, 1.0), false).unwrap();
        run(&op("retouch burn"), &[("mask", "all"), ("exposure", "0.5"), ("range", "shadows")], &mut doc).unwrap();
        let shadows = raster(&doc, "Background").get(2, 2)[0];
        let mut doc = testing::doc_from(gray.clone());
        doc.put_mask("all", Mask::filled(10, 10, 1.0), false).unwrap();
        run(&op("retouch burn"), &[("mask", "all"), ("exposure", "0.5"), ("range", "midtones")], &mut doc).unwrap();
        let mids = raster(&doc, "Background").get(2, 2)[0];
        assert!(mids < shadows && shadows < 0.5, "midtones hit hardest at 50% gray");
        run(&op("retouch burn"), &[("mask", "all"), ("exposure", "0")], &mut doc).unwrap();
        // --mask is required
        assert_eq!(run(&op("retouch dodge"), &[("exposure", "0.5")], &mut doc).unwrap_err().code, "PX1003");
    }

    #[test]
    fn regional_sharpen_and_blur() {
        let edge = Image::from_fn(20, 8, |x, _| if x < 10 { [0.3, 0.3, 0.3, 1.0] } else { [0.7, 0.7, 0.7, 1.0] });
        let mut doc = testing::doc_from(edge.clone());
        doc.put_mask("all", Mask::filled(20, 8, 1.0), false).unwrap();
        run(&op("retouch sharpen"), &[("mask", "all"), ("amount", "100")], &mut doc).unwrap();
        assert!(raster(&doc, "Background").get(10, 4)[0] > 0.7);
        let mut doc = testing::doc_from(edge.clone());
        doc.put_mask("all", Mask::filled(20, 8, 1.0), false).unwrap();
        run(&op("retouch blur"), &[("mask", "all"), ("sigma", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &edge), 0.0);
        run(&op("retouch blur"), &[("mask", "all"), ("sigma", "2")], &mut doc).unwrap();
        let p = raster(&doc, "Background").get(10, 4)[0];
        assert!(p < 0.7 && p > 0.3);
    }

    #[test]
    fn redeye_fixes_red_pupil() {
        let spec = op("retouch redeye");
        let img = Image::from_fn(60, 60, |x, y| {
            let d = ((x as f32 - 30.0).powi(2) + (y as f32 - 30.0).powi(2)).sqrt();
            if d < 5.0 {
                [0.85, 0.1, 0.12, 1.0]
            } else {
                [0.8, 0.65, 0.55, 1.0]
            }
        });
        let mut doc = testing::doc_from(img.clone());
        let v = run(&spec, &[("at", "31,29"), ("strength", "100")], &mut doc).unwrap();
        let r = v["radius"].as_f64().unwrap();
        assert!((4.0..9.0).contains(&r), "measured radius {r}");
        let out = raster(&doc, "Background");
        assert!(out.get(30, 30)[0] < 0.2, "pupil no longer red: {}", out.get(30, 30)[0]);
        assert_eq!(out.get(30, 45), img.get(30, 45), "skin untouched");
        let mut doc = testing::doc_from(img.clone());
        run(&spec, &[("at", "30,30"), ("radius", "6"), ("strength", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &img), 0.0);
        assert_eq!(run(&spec, &[("auto", "true")], &mut doc).unwrap_err().code, "PX1510");
        assert_eq!(run(&spec, &[], &mut doc).unwrap_err().code, "PX1003");
        let mut doc = testing::doc_from(testing::solid(30, 30, Rgba::new(0.2, 0.6, 0.2, 1.0)));
        assert_eq!(run(&spec, &[("at", "15,15")], &mut doc).unwrap_err().code, "PX3320");
    }
}
