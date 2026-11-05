//! Blurs: `filter gaussian`, `box`, `motion-blur`, `radial-blur`, `lens-blur`,
//! `surface-blur`.

use crate::kern::{self, Shape};
use crate::util::{self, Buf};
use pixelox_core::kernels::{self, Edge};
use pixelox_core::prelude::*;
use rayon::prelude::*;

const CAT: Category = Category::Filter;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("filter gaussian", CAT, "Gaussian blur", gaussian)
            .about("Separable Gaussian blur of the layer (premultiplied, so transparent pixels do not bleed dark). Direct convolution up to sigma 4, three running-box passes above, so any sigma costs O(pixels). Opaque layers clamp at the edges; layers with transparency fade out.")
            .param(param("sigma", ParamKind::Float).default("2").range("0..1000").help("Blur radius as the Gaussian standard deviation, in px (0 = no change)"))
            .params(common::region())
            .param(common::linear())
            .example("pixelox filter gaussian photo.jpg -o soft.jpg --sigma 8", "Blur a photo")
            .example("pixelox filter gaussian doc.pxo --in-place --sigma 6 --mask subject --invert-mask", "Blur everything except a mask")
            .example("pixelox filter gaussian doc.pxo --layer Glow --sigma 20 --linear --in-place", "Physically based glow blur in linear light"),
        OpSpec::doc("filter box", CAT, "Box blur", box_)
            .about("Running-sum box blur: each pass averages a (2R+1)² square in O(1) per pixel. Several passes approach a Gaussian (3 passes is a good tent-to-bell).")
            .param(param("radius", ParamKind::Int).default("2").range("0..2000").help("Half-width of the square window in px (0 = no change)"))
            .param(param("passes", ParamKind::Int).default("1").range("1..10").help("Number of box passes"))
            .params(common::region())
            .param(common::linear())
            .example("pixelox filter box photo.jpg -o boxed.png --radius 5", "Simple box blur")
            .example("pixelox filter box photo.jpg -o smooth.png --radius 4 --passes 3", "Near-Gaussian with three passes")
            .example("pixelox filter box doc.pxo --layer Background --radius 12 --mask @sel --in-place", "Blur inside the selection"),
        OpSpec::doc("filter motion-blur", CAT, "Linear motion blur along an angle", motion)
            .about("Averages each pixel along a line of --distance px at --angle (degrees counter-clockwise from horizontal). Lines are sampled once and prefix-summed, so cost is O(pixels) for any distance.")
            .param(param("angle", ParamKind::Angle).default("0").help("Direction of motion in degrees (0 = horizontal, 90 = vertical)"))
            .param(param("distance", ParamKind::Float).default("10").range("0..4000").help("Length of the blur streak in px (0 = no change)"))
            .params(common::region())
            .param(common::linear())
            .example("pixelox filter motion-blur car.jpg -o fast.jpg --angle 0 --distance 40", "Horizontal speed streaks")
            .example("pixelox filter motion-blur rain.png -o streaks.png --angle 75 --distance 25", "Near-vertical rain streaks")
            .example("pixelox filter motion-blur doc.pxo --layer Background --angle 30 --distance 60 --mask road --in-place", "Blur only the road"),
        OpSpec::doc("filter radial-blur", CAT, "Spin or zoom blur around a center", radial)
            .about("Spin blurs along circles around --center (arc of --amount degrees); zoom blurs along rays from it (scale range e^±(amount/200)). Built from log2(arc length) passes of two-tap averages with doubling offsets, so cost is O(pixels·log(amount·size)).")
            .param(param("kind", ParamKind::Choice).choices(&["spin", "zoom"]).default("spin").help("spin (rotational) or zoom (radial streaks)"))
            .param(param("amount", ParamKind::Float).default("10").range("0..100").help("Strength: spin arc in degrees, or zoom extent in % (0 = no change)"))
            .param(param("center", ParamKind::Point).default("50%,50%").help("Center X,Y in canvas coordinates"))
            .params(common::region())
            .param(common::linear())
            .example("pixelox filter radial-blur wheel.jpg -o spin.jpg --kind spin --amount 15", "Spinning wheel")
            .example("pixelox filter radial-blur street.jpg -o zoom.jpg --kind zoom --amount 30 --center 50%,40%", "Zoom burst toward a point")
            .example("pixelox filter radial-blur doc.pxo --kind zoom --amount 20 --mask subject --invert-mask --in-place", "Zoom the background, keep the subject"),
        OpSpec::doc("filter lens-blur", CAT, "Depth-aware lens blur with bokeh", lens)
            .about("Bokeh blur with a disc or hexagonal aperture. With --depth map.png the blur radius per pixel is --radius × the map's gray value (black = in focus, white = full blur); the image is split into up to 7 depth slices that are blurred separately and composited far to near, so sharp areas do not bleed into the blur. Without --depth the whole layer gets a uniform bokeh. --highlights boosts bright spots into bright bokeh discs. --depth auto needs the subject model (ML build).")
            .param(param("depth", ParamKind::Path).auto().help("Depth map image (grayscale, canvas-sized or resized to it; black = in focus, white = most blur), or auto (needs ML)"))
            .param(param("radius", ParamKind::Float).default("15").range("0..500").help("Largest blur radius in px (0 = no change)"))
            .param(param("shape", ParamKind::Choice).choices(&["hex", "circle"]).default("hex").help("Aperture shape of the bokeh: hex (six blades) or circle"))
            .param(param("highlights", ParamKind::Float).default("0").range("0..100").help("Specular highlight boost: bright areas become brighter bokeh discs (0 = off)"))
            .params(common::region())
            .param(common::linear())
            .example("pixelox filter lens-blur portrait.jpg -o bokeh.jpg --depth depth.png --radius 20", "Depth-of-field from a depth map")
            .example("pixelox filter lens-blur lights.jpg -o dreamy.jpg --radius 12 --shape circle --highlights 60", "Uniform bokeh with glowing highlights")
            .example("pixelox filter lens-blur doc.pxo --radius 10 --mask subject --invert-mask --in-place", "Blur the background behind a subject mask"),
        OpSpec::doc("filter surface-blur", CAT, "Edge-preserving surface blur (bilateral)", surface)
            .about("Bilateral blur: averages a (2R+1)² box but only with pixels within the --threshold of each channel (tent weight 1 - |Δ|/(2.5·threshold)). Bounded cost: R ≤ 5 is computed exactly over the window (≤ 121 taps); larger R use a piecewise-linear bilateral filter whose intensity range is sampled at ≤ 32 levels, each box-blurred on a grid downsampled by max(3, R/3), so cost is O(pixels × levels) for any radius.")
            .param(param("radius", ParamKind::Int).default("5").range("0..100").help("Spatial radius of the box window in px (0 = no change)"))
            .param(param("threshold", ParamKind::Float).default("15").range("0..255").help("Edge threshold in levels 0..255: larger differences are not averaged (0 = no change)"))
            .params(common::region())
            .example("pixelox filter surface-blur portrait.jpg -o smooth.jpg --radius 6 --threshold 20", "Smooth skin, keep edges")
            .example("pixelox filter surface-blur scan.png -o clean.png --radius 3 --threshold 10", "Flatten scan noise")
            .example("pixelox filter surface-blur doc.pxo --radius 8 --threshold 25 --mask skin --in-place", "Retouch inside a mask"),
    ]
}

fn sigma_edge(img: &Image) -> Edge {
    kernels::auto_edge(img)
}

fn gaussian(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let sigma = a.f64("sigma")?;
    let linear = a.flag("linear");
    let v = apply_to_raster(a, doc, ctx, |img, _| {
        if sigma <= 0.0 {
            return Ok(img.clone());
        }
        util::maybe_linear(img, linear, |i| Ok(kern::gaussian_image(i, sigma)))
    })?;
    Ok(util::merge(v, json!({"sigma": sigma})))
}

fn box_(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let r = a.usize("radius")?;
    let passes = a.usize("passes")?.max(1);
    let linear = a.flag("linear");
    let v = apply_to_raster(a, doc, ctx, |img, _| {
        if r == 0 {
            return Ok(img.clone());
        }
        util::maybe_linear(img, linear, |i| {
            let edge = sigma_edge(i);
            let mut b = Buf::premul(i);
            for _ in 0..passes {
                kern::box_blur(&mut b, r, r, edge);
            }
            Ok(b.into_unpremul())
        })
    })?;
    Ok(util::merge(v, json!({"radius": r, "passes": passes})))
}

fn motion(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let angle = a.angle("angle")?;
    let dist = a.f64("distance")?;
    let linear = a.flag("linear");
    let v = apply_to_raster(a, doc, ctx, |img, _| {
        if dist < 0.5 {
            return Ok(img.clone());
        }
        util::maybe_linear(img, linear, |i| {
            Ok(kern::motion_blur(&Buf::premul(i), angle, dist, sigma_edge(i)).into_unpremul())
        })
    })?;
    Ok(util::merge(v, json!({"angle": angle, "distance": dist})))
}

// ---------------------------------------------------------------------------------------
// radial blur

#[derive(Clone, Copy, PartialEq, Eq)]
enum Radial {
    Spin,
    Zoom,
}

/// Spin/zoom blur by recursive doubling: pass i averages the two samples at ±δ·2^i
/// (rotation angle or log-scale), so k passes give 2^k evenly spaced taps.
fn radial_blur(src: Buf, kind: Radial, amount: f64, cx: f64, cy: f64, edge: Edge) -> (Buf, usize) {
    let (w, h) = (src.w as f64, src.h as f64);
    let rmax = [(0.0, 0.0), (w, 0.0), (0.0, h), (w, h)]
        .iter()
        .map(|(x, y)| ((x - cx).powi(2) + (y - cy).powi(2)).sqrt())
        .fold(0.0, f64::max);
    let span = match kind {
        Radial::Spin => amount.to_radians(),
        Radial::Zoom => amount / 100.0,
    };
    // taps no more than ~1.5 px apart at the farthest corner
    let taps = span * rmax / 1.5;
    if taps < 1.0 || src.w == 0 || src.h == 0 {
        return (src, 0);
    }
    let k = (taps.log2().ceil() as usize).clamp(1, 14);
    let delta = span / (2.0 * ((1usize << k) - 1) as f64);
    let w = src.w;
    let mut cur = src;
    let mut next = Buf::new(cur.w, cur.h, 4);
    for i in 0..k {
        let d = delta * (1u64 << i) as f64;
        let (s, co) = d.sin_cos();
        let (gp, gm) = (d.exp(), (-d).exp());
        let src = &cur;
        next.data.par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
            let ry = y as f64 - cy;
            for (x, o) in row.chunks_mut(4).enumerate() {
                let rx = x as f64 - cx;
                let ((ax, ay), (bx, by)) = match kind {
                    Radial::Spin => {
                        ((cx + rx * co - ry * s, cy + rx * s + ry * co), (cx + rx * co + ry * s, cy - rx * s + ry * co))
                    }
                    Radial::Zoom => ((cx + rx * gp, cy + ry * gp), (cx + rx * gm, cy + ry * gm)),
                };
                let pa = util::sample4(src, ax as f32, ay as f32, edge);
                let pb = util::sample4(src, bx as f32, by as f32, edge);
                for c in 0..4 {
                    o[c] = 0.5 * (pa[c] + pb[c]);
                }
            }
        });
        std::mem::swap(&mut cur, &mut next);
    }
    (cur, k)
}

fn radial(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let kind = if a.choice("kind") == "zoom" { Radial::Zoom } else { Radial::Spin };
    let amount = a.f64("amount")?;
    let (ccx, ccy) = util::canvas_point(a, "center", doc)?.unwrap_or((doc.width as f64 / 2.0, doc.height as f64 / 2.0));
    let (lx, ly) = util::layer_offset(a, doc)?;
    // pixel centers are integers in the kernel's coordinates
    let (cx, cy) = (ccx - lx as f64 - 0.5, ccy - ly as f64 - 0.5);
    let linear = a.flag("linear");
    let mut passes = 0;
    let v = apply_to_raster(a, doc, ctx, |img, _| {
        if amount <= 0.0 {
            return Ok(img.clone());
        }
        util::maybe_linear(img, linear, |i| {
            let (b, k) = radial_blur(Buf::premul(i), kind, amount, cx, cy, sigma_edge(i));
            passes = k;
            Ok(b.into_unpremul())
        })
    })?;
    Ok(util::merge(v, json!({"center": [ccx, ccy], "passes": passes})))
}

// ---------------------------------------------------------------------------------------
// lens blur

/// Lens blur of premultiplied RGBA. `depth` (layer-sized, 0..1) scales the radius per
/// pixel; `straight` supplies luma for the highlight boost.
fn lens_blur(
    p: &Buf,
    straight: &Image,
    depth: Option<&[f32]>,
    radius: f32,
    shape: Shape,
    highlights: f32,
    edge: Edge,
) -> (Buf, usize) {
    let (w, h) = (p.w, p.h);
    // bright areas carry extra energy into the blurred levels
    let gain = highlights / 100.0 * 8.0;
    let boosted_buf = (gain > 0.0).then(|| {
        let mut b = p.clone();
        b.data.par_chunks_mut(4).zip(straight.data.par_chunks(4)).for_each(|(b, s)| {
            let l = pixelox_core::color::luma(s[0], s[1], s[2]);
            let g = 1.0 + gain * util::smoothstep(0.75, 1.0, l);
            for v in b.iter_mut().take(3) {
                *v *= g;
            }
        });
        b
    });
    let boosted = boosted_buf.as_ref().unwrap_or(p);
    let Some(depth) = depth else {
        return (kern::shape_blur(boosted, radius, shape, edge), 1);
    };
    let levels = ((radius / 4.0).ceil() as usize + 1).clamp(2, 7);
    let step = (levels - 1) as f32;
    // composite slices far (most blur) to near: acc = C_k + acc·(1 - A_k)
    let mut acc = Buf::new(w, h, 5);
    let mut used = 0;
    for k in (0..levels).rev() {
        let membership = |d: f32| (1.0 - (d.clamp(0.0, 1.0) * step - k as f32).abs()).max(0.0);
        if !depth.par_iter().any(|&d| membership(d) > 0.0) {
            continue;
        }
        used += 1;
        let src = if k == 0 { p } else { boosted };
        let r = radius * k as f32 / step;
        // large radii are blurred on a grid downsampled by f (built directly at that size)
        let f = if r > kern::SHAPE_CAP { (r / kern::SHAPE_CAP).ceil() as usize } else { 1 };
        let (sw, sh) = (w.div_ceil(f), h.div_ceil(f));
        let mut slice = Buf::new(sw, sh, 5);
        slice.data.par_chunks_mut(sw * 5).enumerate().for_each(|(oy, row)| {
            let (y0, y1) = (oy * f, ((oy + 1) * f).min(h));
            for y in y0..y1 {
                for x in 0..w {
                    let i = y * w + x;
                    let m = membership(depth[i]);
                    if m <= 0.0 {
                        continue;
                    }
                    let o = &mut row[(x / f) * 5..(x / f) * 5 + 5];
                    for ch in 0..4 {
                        o[ch] += src.data[i * 4 + ch] * m;
                    }
                    o[4] += m;
                }
            }
            if f > 1 {
                for (ox, o) in row.chunks_mut(5).enumerate() {
                    let n = ((y1 - y0) * (((ox + 1) * f).min(w) - ox * f)) as f32;
                    o.iter_mut().for_each(|v| *v /= n);
                }
            }
        });
        let blurred = if k == 0 { slice } else { kern::shape_blur_direct(&slice, r / f as f32, shape, edge) };
        let ff = f as f32;
        acc.data.par_chunks_mut(w * 5).enumerate().for_each(|(y, arow)| {
            let mut b = [0f32; 5];
            let sy = (y as f32 + 0.5) / ff - 0.5;
            for (x, a) in arow.chunks_mut(5).enumerate() {
                if f == 1 {
                    b.copy_from_slice(&blurred.data[(y * w + x) * 5..(y * w + x) * 5 + 5]);
                } else {
                    util::sample(&blurred, (x as f32 + 0.5) / ff - 0.5, sy, Edge::Clamp, &mut b);
                }
                let t = 1.0 - b[4].clamp(0.0, 1.0);
                for ch in 0..5 {
                    a[ch] = b[ch] + a[ch] * t;
                }
            }
        });
    }
    let mut out = Buf::new(w, h, 4);
    out.data.par_chunks_mut(4).zip(acc.data.par_chunks(5)).for_each(|(o, a)| {
        if a[4] > 1e-6 {
            for ch in 0..4 {
                o[ch] = a[ch] / a[4];
            }
        }
    });
    (out, used)
}

fn lens(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let radius = a.f64("radius")? as f32;
    let shape = if a.choice("shape") == "circle" { Shape::Circle } else { Shape::Hex };
    let highlights = a.f64("highlights")? as f32;
    let linear = a.flag("linear");
    let path = target_layer(a, doc)?;
    let (lx, ly) = (doc.layer(&path).x, doc.layer(&path).y);
    let (lw, lh) = {
        let r = doc.layer(&path).raster()?;
        (r.width as usize, r.height as usize)
    };
    let depth: Option<Vec<f32>> = match a.raw("depth") {
        None => None,
        Some(v) if v.trim() == "auto" => {
            return Err(Error::new("PX1510", "--depth auto needs the subject/depth model, which is not in this build")
                .with_hint("pass a depth map instead: --depth depth.png (black = in focus, white = most blur)")
                .with_context("param", "depth"));
        }
        Some(p) => {
            let (img, _) = ctx.decode_aux(std::path::Path::new(p), "depth", Some("depth"))?;
            let (cw, ch) = (doc.width, doc.height);
            let img = if img.width != cw || img.height != ch {
                kernels::resize(&img, cw, ch, kernels::Filter::Bilinear)
            } else {
                img
            };
            let canvas = util::luma_plane(&img);
            let (cw, ch) = (cw as i64, ch as i64);
            let mut d = vec![0f32; lw * lh];
            d.par_chunks_mut(lw.max(1)).enumerate().for_each(|(y, row)| {
                let sy = (y as i64 + ly as i64).clamp(0, ch - 1) as usize;
                for (x, v) in row.iter_mut().enumerate() {
                    let sx = (x as i64 + lx as i64).clamp(0, cw - 1) as usize;
                    *v = canvas[sy * cw as usize + sx];
                }
            });
            Some(d)
        }
    };
    let mut slices = 0;
    let v = apply_to_raster(a, doc, ctx, |img, _| {
        if radius < 0.5 {
            return Ok(img.clone());
        }
        util::maybe_linear(img, linear, |i| {
            let (b, n) = lens_blur(&Buf::premul(i), i, depth.as_deref(), radius, shape, highlights, sigma_edge(i));
            slices = n;
            Ok(b.into_unpremul())
        })
    })?;
    Ok(util::merge(v, json!({"radius": radius, "depth_slices": slices})))
}

// ---------------------------------------------------------------------------------------
// surface blur

fn surface(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let r = a.usize("radius")?;
    let threshold = a.f64("threshold")? as f32;
    let mut levels = 0;
    let v = apply_to_raster(a, doc, ctx, |img, _| {
        if r == 0 || threshold <= 0.0 {
            return Ok(img.clone());
        }
        let (o, n) = kern::surface_blur(img, r, 2.5 * threshold / 255.0);
        levels = n;
        Ok(o)
    })?;
    Ok(util::merge(v, json!({"range_levels": levels})))
}

// ---------------------------------------------------------------------------------------
// tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_io;
    use pixelox_core::testing::{self, find, max_diff, raster, run};

    fn op(name: &str) -> OpSpec {
        find(&ops(), name)
    }

    fn spot(w: u32, h: u32) -> Image {
        Image::from_fn(w, h, |x, y| if x == w / 2 && y == h / 2 { [1.0, 1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0] })
    }

    #[test]
    fn gaussian_blurs_identity_mask_and_transparent() {
        let spec = op("filter gaussian");
        let mut doc = testing::doc_from(testing::checker(32, 32));
        run(&spec, &[("sigma", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &testing::checker(32, 32)), 0.0);
        run(&spec, &[("sigma", "3")], &mut doc).unwrap();
        let m = testing::mean(raster(&doc, "Background"), 0);
        assert!((m - 0.5).abs() < 0.05);
        // mask: only the left half changes
        let mut doc = testing::doc_from(testing::checker(32, 32));
        doc.put_mask("left", testing::half_mask(32, 32), false).unwrap();
        run(&spec, &[("sigma", "3"), ("mask", "left")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert_eq!(r.get(20, 4), testing::checker(32, 32).get(20, 4));
        assert_ne!(r.get(4, 4), testing::checker(32, 32).get(4, 4));
        // transparent layer stays transparent, 1x1 works
        let mut doc = testing::doc_from(Image::new(8, 8));
        run(&spec, &[("sigma", "2")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").alpha_bbox(0.0), None);
        let mut doc = testing::doc_from(testing::solid(1, 1, Rgba::WHITE));
        run(&spec, &[("sigma", "5"), ("linear", "true")], &mut doc).unwrap();
        assert!((raster(&doc, "Background").get(0, 0)[0] - 1.0).abs() < 1e-4);
    }

    #[test]
    fn box_blur_spreads_and_identity() {
        let spec = op("filter box");
        let mut doc = testing::doc_from(spot(21, 21));
        run(&spec, &[("radius", "0")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").get(10, 10)[0], 1.0);
        run(&spec, &[("radius", "2"), ("passes", "2")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.get(10, 10)[0] < 0.2 && r.get(12, 12)[0] > 0.0 && r.get(16, 10)[0] == 0.0);
        let mut doc = testing::doc_from(testing::solid(1, 1, Rgba::new(0.3, 0.3, 0.3, 1.0)));
        run(&spec, &[("radius", "4")], &mut doc).unwrap();
        assert!((raster(&doc, "Background").get(0, 0)[0] - 0.3).abs() < 1e-5);
    }

    #[test]
    fn motion_blur_direction_and_locked() {
        let spec = op("filter motion-blur");
        let mut doc = testing::doc_from(spot(31, 31));
        run(&spec, &[("angle", "90"), ("distance", "9")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.get(15, 18)[0] > 0.05, "vertical streak");
        assert!(r.get(18, 15)[0] < 1e-4, "not horizontal");
        let mut doc = testing::doc_from(spot(9, 9));
        run(&spec, &[("distance", "0")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").get(4, 4)[0], 1.0);
        doc.layers[0].locked = true;
        let e = run(&spec, &[("distance", "5")], &mut doc).unwrap_err();
        assert_eq!(e.code, "PX2304");
    }

    #[test]
    fn radial_blur_spin_and_zoom() {
        let spec = op("filter radial-blur");
        let ring = Image::from_fn(41, 41, |x, y| {
            if (x as i32 - 30).abs() < 2 && (y as i32 - 20).abs() < 2 {
                [1.0; 4]
            } else {
                [0.0, 0.0, 0.0, 1.0]
            }
        });
        let mut doc = testing::doc_from(ring.clone());
        run(&spec, &[("amount", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &ring), 0.0);
        let v = run(&spec, &[("kind", "spin"), ("amount", "40")], &mut doc).unwrap();
        assert!(v["passes"].as_u64().unwrap() >= 2);
        let r = raster(&doc, "Background");
        // the dot at angle 0 smears along the circle of radius 10 around (20.5, 20.5),
        // ±20° for a 40° arc: (29, 23) sits at 18°, (29, 26) at 34° is outside
        assert!(r.get(29, 23)[0] > 0.05, "spread along the arc: {}", r.get(29, 23)[0]);
        assert!(r.get(29, 26)[0] < 0.02, "but only over the arc: {}", r.get(29, 26)[0]);
        assert!(r.get(30, 20)[0] < 0.9);
        let mut doc = testing::doc_from(ring.clone());
        run(&spec, &[("kind", "zoom"), ("amount", "50")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.get(34, 20)[0] > 0.02, "spread along the ray");
        assert!(r.get(30, 27)[0] < 0.01, "not across it");
    }

    #[test]
    fn lens_blur_uniform_depth_and_auto_gate() {
        let spec = op("filter lens-blur");
        let mut doc = testing::doc_from(spot(41, 41));
        run(&spec, &[("radius", "0")], &mut doc).unwrap();
        assert_eq!(raster(&doc, "Background").get(20, 20)[0], 1.0);
        run(&spec, &[("radius", "6"), ("shape", "circle"), ("highlights", "50")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!(r.get(20, 25)[0] > 0.0 && r.get(20, 28)[0] == 0.0, "disc of radius 6");
        // depth map: left half in focus (black), right half blurred (white)
        let td = testing::TempDir::new("lensblur");
        let depth = Image::from_fn(41, 41, |x, _| if x < 20 { [0.0, 0.0, 0.0, 1.0] } else { [1.0; 4] });
        let p = td.path("depth.pxt");
        std::fs::write(&p, test_io::encode(&depth)).unwrap();
        let src = testing::checker(41, 41);
        let mut doc = testing::doc_from(src.clone());
        let v = test_io::run_io(&spec, &[("radius", "5"), ("depth", p.to_str().unwrap())], &mut doc).unwrap();
        assert_eq!(v["depth_slices"], 2);
        let r = raster(&doc, "Background");
        assert_eq!(r.get(3, 3), src.get(3, 3), "in focus stays sharp");
        assert!(
            (r.get(35, 35)[0] - 0.5).abs() < 0.3 && r.get(35, 35)[0] > 0.05 && r.get(35, 35)[0] < 0.95,
            "blurred side"
        );
        let e = run(&spec, &[("depth", "auto")], &mut doc).unwrap_err();
        assert_eq!(e.code, "PX1510");
    }

    #[test]
    fn surface_blur_keeps_edges_smooths_noise() {
        let spec = op("filter surface-blur");
        let img = Image::from_fn(32, 32, |x, y| {
            let n = (((x * 7 + y * 13) % 5) as f32 - 2.0) * 0.01;
            let base = if x < 16 { 0.2 } else { 0.8 };
            [base + n, base + n, base + n, 1.0]
        });
        let mut doc = testing::doc_from(img.clone());
        run(&spec, &[("threshold", "0")], &mut doc).unwrap();
        assert_eq!(max_diff(raster(&doc, "Background"), &img), 0.0);
        run(&spec, &[("radius", "4"), ("threshold", "20")], &mut doc).unwrap();
        let r = raster(&doc, "Background");
        assert!((r.get(14, 10)[0] - 0.2).abs() < 0.02, "edge kept: {}", r.get(14, 10)[0]);
        assert!((r.get(17, 10)[0] - 0.8).abs() < 0.02, "edge kept: {}", r.get(17, 10)[0]);
        let dev = |i: &Image| (0..16).map(|x| (i.get(x, 20)[0] - 0.2).abs()).fold(0.0, f32::max);
        assert!(dev(r) < dev(&img));
        // large radius goes through the downsampled path
        let mut doc = testing::doc_from(img);
        run(&spec, &[("radius", "12"), ("threshold", "20")], &mut doc).unwrap();
        assert!((raster(&doc, "Background").get(2, 2)[0] - 0.2).abs() < 0.03);
    }
}
