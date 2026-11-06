//! Painting media: `filter oil-paint` and `filter watercolor`.

use super::*;
use crate::util;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("filter oil-paint", CAT, "Oil painting: brushwork that follows the forms", oil_paint)
            .about("Repaints the layer with brush strokes: an anisotropic Kuwahara filter (Kyprianidis et al.) flattens detail into strokes stretched along the local edge direction while keeping edges, then impasto relief from a stroke texture that flows with the image (line integral convolution) is lit from --angle. Brushes larger than 6 px are computed on a downsampled copy, so the cost does not grow with the brush. Sizes scale with the layer. Alpha is kept.")
            .param(param("brush", ParamKind::Float).auto().default("auto").range("0.5..500").help("Brush radius in px; auto = 4.5 × √(w·h)/1000 (recorded)"))
            .param(param("sharpness", ParamKind::Float).default("50").range("0..100").help("Stroke edge crispness: 0 = soft blended strokes, 100 = hard flat strokes"))
            .param(param("impasto", ParamKind::Float).default("50").range("0..100").help("Relief of the paint (lit brush texture; 0 = flat)"))
            .param(param("angle", ParamKind::Angle).default("135").help("Light direction for the relief, degrees counter-clockwise from the right"))
            .params(common::region())
            .example("pixelox filter oil-paint photo.jpg -o painting.png", "Oil painting with default brush")
            .example("pixelox filter oil-paint landscape.jpg -o bold.png --brush 14 --impasto 80", "Bigger brush, thicker paint")
            .example("pixelox filter oil-paint portrait.jpg -o smooth.png --impasto 0 --sharpness 30", "Smooth painterly look without relief")
            .example("pixelox filter oil-paint doc.pxo --layer Photo --in-place", "Paint one layer of a document"),
        OpSpec::doc("filter watercolor", CAT, "Watercolor: soft washes, pooled edges, grain", watercolor)
            .about("Repaints the layer as watercolor on paper: an anisotropic Kuwahara filter simplifies it into washes, edges wobble with the paper, colors bleed into each other in places (wet in wet), pigment pools along the edges of washes and settles into the paper grain (granulation), and the result is lighter and transparent over a procedural paper texture (Bousseau et al. 2006 pigment model). Sizes scale with the layer. Alpha is kept.")
            .param(param("brush", ParamKind::Float).auto().default("auto").range("0.5..500").help("Wash simplification radius in px; auto = 5 × √(w·h)/1000 (recorded)"))
            .param(param("bleed", ParamKind::Float).default("40").range("0..100").help("Wet-in-wet color bleeding between areas"))
            .param(param("edges", ParamKind::Float).default("50").range("0..100").help("Pigment pooling (darker rims) along the edges of washes"))
            .param(param("granulation", ParamKind::Float).default("50").range("0..100").help("Pigment settling into the paper grain"))
            .param(param("paper", ParamKind::Color).default("#fbf8f0").help("Paper color"))
            .params(common::region())
            .example("pixelox filter watercolor photo.jpg -o watercolor.png", "Watercolor painting")
            .example("pixelox filter watercolor landscape.jpg -o wet.png --bleed 80 --edges 70", "Wetter washes with strong rims")
            .example("pixelox filter watercolor portrait.jpg -o clean.png --granulation 10 --bleed 10", "Clean, controlled washes")
            .example("pixelox filter watercolor doc.pxo --mask @sel --in-place", "Watercolor inside the selection"),
    ]
}

// ---------------------------------------------------------------------------------------
// oil paint

pub(crate) struct Oil {
    pub brush: f64,
    pub sharpness: f32,
    pub impasto: f32,
    pub angle: f64,
}

fn oil_paint(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (lx, ly) = util::layer_offset(a, doc)?;
    let mut p =
        Oil { brush: 0.0, sharpness: pct(a, "sharpness")?, impasto: pct(a, "impasto")?, angle: a.angle("angle")? };
    let v = apply_to_raster(a, doc, ctx, |img, ctx| {
        if img.is_empty() {
            return Ok(img.clone());
        }
        p.brush = size_param(a, ctx, "brush", 4.5 * unit(img.width as usize, img.height as usize))?;
        let tex = Tex::new(ctx, lx, ly);
        Ok(with_alpha(img, &oil_render(&solid_rgb(img), &p, tex)))
    })?;
    Ok(if p.brush > 0.0 { util::merge(v, json!({"brush": p.brush})) } else { v })
}

/// Line integral convolution of `noise` along the tangent field: each pixel averages the
/// noise along the streamline through it, `steps` px each way.
pub(crate) fn lic(noise: &Buf, fl: &Flow, steps: usize) -> Buf {
    let (w, h) = (noise.w, noise.h);
    let mut out = Buf::new(w, h, 1);
    if w == 0 || h == 0 {
        return out;
    }
    let tangent = |x: f32, y: f32| {
        let xi = (x.round() as i64).clamp(0, w as i64 - 1) as usize;
        let yi = (y.round() as i64).clamp(0, h as i64 - 1) as usize;
        let i = yi * w + xi;
        (fl.tx[i], fl.ty[i])
    };
    out.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        let mut o = [0f32];
        for x in 0..w {
            let (x0, y0) = (x as f32, y as f32);
            let mut sum = noise.data[y * w + x];
            let mut wsum = 1.0;
            for dir in [1.0f32, -1.0] {
                let (mut px, mut py) = (x0, y0);
                let (mut dx0, mut dy0) = tangent(px, py);
                dx0 *= dir;
                dy0 *= dir;
                for k in 0..steps {
                    let (mut dx, mut dy) = tangent(px, py);
                    // keep heading the same way (the tangent is a line field)
                    if dx * dx0 + dy * dy0 < 0.0 {
                        dx = -dx;
                        dy = -dy;
                    }
                    px += dx;
                    py += dy;
                    if px < -0.5 || py < -0.5 || px > w as f32 - 0.5 || py > h as f32 - 0.5 {
                        break;
                    }
                    let wt = 1.0 - k as f32 / steps as f32;
                    util::sample(noise, px, py, Edge::Clamp, &mut o);
                    sum += o[0] * wt;
                    wsum += wt;
                    dx0 = dx;
                    dy0 = dy;
                }
            }
            row[x] = sum / wsum;
        }
    });
    out
}

pub(crate) fn oil_render(rgb: &Buf, p: &Oil, tex: Tex) -> Buf {
    let (w, h) = (rgb.w, rgb.h);
    let q = 3.0 + 12.0 * p.sharpness;
    let (paint, f) = abstraction(rgb, p.brush, q, 1);
    let (ww, wh) = (paint.w, paint.h);
    let rb = (p.brush / f as f64).max(1.0);
    // stroke texture flowing with the painted image: fine bristles plus coarser strokes
    let mut fl = flow(&paint, 2.5 * rb);
    // where the image has no clear direction, strokes follow a smooth seeded field
    let psi = smooth_noise(ww, wh, tex.salt(52), 8.0 * rb);
    fl.tx.par_iter_mut().zip(fl.ty.par_iter_mut()).zip(fl.aniso.par_iter().zip(psi.data.par_iter())).for_each(
        |((tx, ty), (&an, &z))| {
            let k = util::smoothstep(0.05, 0.4, an);
            let a = 0.8 + 1.1 * z;
            let (mut rx, mut ry) = (a.cos(), a.sin());
            if rx * *tx + ry * *ty < 0.0 {
                rx = -rx;
                ry = -ry;
            }
            let (vx, vy) = (*tx * k + rx * (1.0 - k), *ty * k + ry * (1.0 - k));
            let n = (vx * vx + vy * vy).sqrt();
            if n > 1e-6 {
                *tx = vx / n;
                *ty = vy / n;
            }
        },
    );
    let mut fine = white(ww, wh, tex.salt(50));
    gauss_in(&mut fine, 0.6);
    let coarse = smooth_noise(ww, wh, tex.salt(51), 0.6 * rb);
    let steps = (2.2 * rb).ceil() as usize;
    let mut hf = lic(&fine, &fl, steps);
    standardize(&mut hf);
    let mut hc = lic(&coarse, &fl, steps);
    standardize(&mut hc);
    let mut height = Buf::new(ww, wh, 1);
    height.data.par_iter_mut().enumerate().for_each(|(i, v)| *v = 0.6 * hf.data[i] + 0.4 * hc.data[i]);
    let (paint, height) =
        if f > 1 { (util::upsample(&paint, w, h, f), util::upsample(&height, w, h, f)) } else { (paint, height) };
    // relief lighting, light at 45° elevation
    let (gx, gy) = gradient(&height);
    // the texture lives on the working grid: its slope per full-resolution px is 1/f of
    // the slope per working px, so scale by f to light it the same at every resolution
    let s = 0.8 * p.impasto * f as f32 / (rb as f32).sqrt();
    let az = p.angle.to_radians();
    let el = std::f64::consts::FRAC_1_SQRT_2; // 45° elevation
    let (lx, ly, lz) = ((az.cos() * el) as f32, (-az.sin() * el) as f32, el as f32);
    let (hx, hy, hz) = {
        let (x, y, z) = (lx, ly, lz + 1.0);
        let n = (x * x + y * y + z * z).sqrt();
        (x / n, y / n, z / n)
    };
    let mut out = Buf::new(w, h, 3);
    out.data.par_chunks_mut(3).enumerate().for_each(|(i, o)| {
        let (nx, ny, nz) = (-s * gx.data[i], -s * gy.data[i], 1.0f32);
        let n = (nx * nx + ny * ny + nz * nz).sqrt();
        let (nx, ny, nz) = (nx / n, ny / n, nz / n);
        let diffuse = (nx * lx + ny * ly + nz * lz).max(0.0) / lz;
        let spec = (nx * hx + ny * hy + nz * hz).max(0.0).powi(40) * 0.25 * p.impasto;
        let shade = 1.0 + (diffuse - 1.0) * 0.9;
        let c = &paint.data[i * 3..i * 3 + 3];
        // a little more saturation, as paint
        let m = (c[0] + c[1] + c[2]) / 3.0;
        for ch in 0..3 {
            let v = m + (c[ch] - m) * 1.12;
            o[ch] = (v * shade + spec).clamp(0.0, 1.0);
        }
    });
    out
}

// ---------------------------------------------------------------------------------------
// watercolor

pub(crate) struct Water {
    pub brush: f64,
    pub bleed: f32,
    pub edges: f32,
    pub granulation: f32,
    pub paper: [f32; 3],
}

fn watercolor(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let (lx, ly) = util::layer_offset(a, doc)?;
    let paper = a.color("paper")?;
    let mut p = Water {
        brush: 0.0,
        bleed: pct(a, "bleed")?,
        edges: pct(a, "edges")?,
        granulation: pct(a, "granulation")?,
        paper: [paper.r, paper.g, paper.b],
    };
    let v = apply_to_raster(a, doc, ctx, |img, ctx| {
        if img.is_empty() {
            return Ok(img.clone());
        }
        p.brush = size_param(a, ctx, "brush", 5.0 * unit(img.width as usize, img.height as usize))?;
        let tex = Tex::new(ctx, lx, ly);
        Ok(with_alpha(img, &water_render(&solid_rgb(img), &p, tex)))
    })?;
    Ok(if p.brush > 0.0 { util::merge(v, json!({"brush": p.brush})) } else { v })
}

pub(crate) fn water_render(rgb: &Buf, p: &Water, tex: Tex) -> Buf {
    let (w, h) = (rgb.w, rgb.h);
    // texture scale follows the brush (auto brush = 5 units)
    let u = (p.brush / 5.0).max(0.2);
    let (wash, f) = abstraction(rgb, p.brush, 5.0, 2);
    let mut wash = if f > 1 { util::upsample(&wash, w, h, f) } else { wash };
    gauss_in(&mut wash, 0.3 * p.brush);
    // irregular edges: displace by smooth noise at two scales
    let nx = smooth_noise(w, h, tex.salt(60), 6.0 * u);
    let ny = smooth_noise(w, h, tex.salt(61), 6.0 * u);
    let fx = smooth_noise(w, h, tex.salt(66), 1.5 * u);
    let fy = smooth_noise(w, h, tex.salt(67), 1.5 * u);
    let (amp, famp) = ((1.2 * u) as f32, (0.4 * u) as f32);
    let mut a = Buf::new(w, h, 3);
    a.data.par_chunks_mut(w * 3).enumerate().for_each(|(y, row)| {
        for x in 0..w {
            let i = y * w + x;
            let sx = x as f32 + amp * nx.data[i] + famp * fx.data[i];
            let sy = y as f32 + amp * ny.data[i] + famp * fy.data[i];
            util::sample(&wash, sx, sy, Edge::Clamp, &mut row[x * 3..x * 3 + 3]);
        }
    });
    // wet in wet: colors bleed where a low-frequency mask is high
    if p.bleed > 0.0 {
        let m = smooth_noise(w, h, tex.salt(62), 30.0 * u);
        let soft = gauss(&a, 1.2 * p.brush);
        a.data.par_chunks_mut(3).enumerate().for_each(|(i, c)| {
            let k = util::smoothstep(0.0, 1.3, m.data[i]) * p.bleed;
            for ch in 0..3 {
                c[ch] += (soft.data[i * 3 + ch] - c[ch]) * k;
            }
        });
    }
    // transparent pigment: lighter than the photo, highlights left as paper
    a.data.par_chunks_mut(3).for_each(|c| {
        let l = color::luma(c[0], c[1], c[2]);
        let lift = util::smoothstep(0.78, 0.95, l);
        for v in c.iter_mut() {
            let t = 1.0 - (1.0 - v.clamp(0.0, 1.0)) * 0.72;
            *v = t + (1.0 - t) * lift;
        }
    });
    // pigment pools on the inside of darker washes (one-sided rims)
    let la = luma(&a);
    let rim_in = gauss(&la, 0.8 * u);
    let rim_out = gauss(&la, 3.0 * u);
    let pg = {
        let fine = smooth_noise(w, h, tex.salt(63), 0.8 * u);
        let mid = smooth_noise(w, h, tex.salt(64), 3.0 * u);
        let mut b = fine;
        b.data.par_iter_mut().zip(mid.data.par_iter()).for_each(|(v, &m)| *v = 0.7 * *v + 0.5 * m);
        b
    };
    let turb = smooth_noise(w, h, tex.salt(65), 14.0 * u);
    let mut out = Buf::new(w, h, 3);
    out.data.par_chunks_mut(3).enumerate().for_each(|(i, o)| {
        let rim = ((rim_out.data[i] - rim_in.data[i]) / 0.05).clamp(0.0, 1.0);
        let d = 1.0
            + 1.3 * p.edges * rim
            + 0.25 * p.granulation * (-pg.data[i]).clamp(-2.0, 2.0)
            + 0.1 * turb.data[i].clamp(-2.5, 2.5);
        for ch in 0..3 {
            let c = a.data[i * 3 + ch];
            let v = c - (c - c * c) * (d - 1.0);
            o[ch] = (v.clamp(0.0, 1.0) * p.paper[ch]) * (1.0 - 0.03 * pg.data[i].clamp(-2.0, 2.0));
        }
    });
    out
}

#[cfg(test)]
mod tests {
    use super::super::fixture::*;
    use super::*;

    /// Two noisy flat halves (dark | light).
    fn noisy_step(w: u32, h: u32) -> Image {
        Image::from_fn(w, h, |x, y| {
            let n = ((x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503)) % 101) as f32 / 101.0 * 0.2 - 0.1;
            let v = if x < w / 2 { 0.3 } else { 0.7 } + n;
            [v, v * 0.9, v * 0.8, 1.0]
        })
    }

    #[test]
    fn oil_paint_smooths_regions_and_keeps_edges() {
        let src = noisy_step(120, 60);
        let (out, v) = run("filter oil-paint", &[("impasto", "0"), ("brush", "4")], &src);
        assert_eq!(v["brush"], json!(4.0));
        let (r0, r1) = (roughness(&src, 5, 5, 50, 55), roughness(&out, 5, 5, 50, 55));
        assert!(r1 < 0.4 * r0, "smoother: {r0} → {r1}");
        // the step stays sharp: 3 px either side keep their sides' levels
        let (a, b) = (mean_rect(&out, 54, 10, 57, 50), mean_rect(&out, 63, 10, 66, 50));
        let (sa, sb) = (mean_rect(&src, 5, 10, 50, 50), mean_rect(&src, 70, 10, 115, 50));
        assert!((a - sa).abs() < 0.06 && (b - sb).abs() < 0.06, "edge kept: {a} {b} vs {sa} {sb}");
    }

    #[test]
    fn oil_paint_impasto_adds_relief() {
        let src = noisy_step(120, 60);
        let (flat, _) = run("filter oil-paint", &[("impasto", "0")], &src);
        let (relief, _) = run("filter oil-paint", &[("impasto", "100")], &src);
        assert!(roughness(&relief, 5, 5, 50, 55) > roughness(&flat, 5, 5, 50, 55));
    }

    #[test]
    fn watercolor_is_soft_lighter_with_darker_rims() {
        let src = Image::from_fn(140, 100, |x, y| {
            let n = ((x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503)) % 97) as f32 / 97.0 * 0.1 - 0.05;
            let inside = (35..105).contains(&x) && (25..75).contains(&y);
            let c = if inside { [0.2 + n, 0.35 + n, 0.7 + n] } else { [0.92, 0.9, 0.85] };
            [c[0], c[1], c[2], 1.0]
        });
        let (out, _) = run("filter watercolor", &[("granulation", "0"), ("bleed", "0"), ("brush", "8")], &src);
        // lighter overall and inside the wash
        let (li, lo) = (mean_rect(&src, 50, 40, 90, 60), mean_rect(&out, 50, 40, 90, 60));
        assert!(lo > li + 0.05, "wash is lighter: {li} → {lo}");
        // pigment pools on the inside of the wash's edge
        let rim = (36..46).map(|x| mean_rect(&out, x, 40, x + 1, 60)).fold(1.0f32, f32::min);
        assert!(rim < lo - 0.02, "rim {rim} vs center {lo}");
        // soft: the photo's fine noise is gone
        assert!(roughness(&out, 50, 40, 90, 60) < 0.5 * roughness(&src, 50, 40, 90, 60));
        // the paper stays light
        assert!(mean_rect(&out, 3, 3, 30, 20) > 0.8);
    }

    #[test]
    fn lic_follows_the_field() {
        let (w, h) = (40usize, 40usize);
        let noise = white(w, h, Tex { key: 3, ox: 0, oy: 0 });
        let fl = Flow { tx: vec![1.0; w * h], ty: vec![0.0; w * h], aniso: vec![1.0; w * h] };
        let o = lic(&noise, &fl, 8);
        let d = |dx: usize, dy: usize| {
            (5..30)
                .flat_map(|y| (5..30).map(move |x| (x, y)))
                .map(|(x, y)| (o.data[y * w + x] - o.data[(y + dy) * w + x + dx]).abs())
                .sum::<f32>()
        };
        assert!(d(2, 0) < 0.5 * d(0, 2), "streaks run along x");
    }
}
