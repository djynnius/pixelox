//! The image side of segmentation models: turning a picture into the model's input tensor,
//! reading its output map back as a canvas-sized soft mask, and snapping the mask's edge to
//! the picture with a guided filter. None of it needs the runtime, so it is tested with a
//! stand-in model.

use crate::registry::{Head, Source};
use pixelox_core::kernels::{self, Edge, Filter};
use pixelox_core::prelude::*;
use rayon::prelude::*;

/// Runs a model on an NCHW `1×3×n×n` tensor and returns its first `n×n` output map.
pub type Infer<'a> = &'a mut dyn FnMut(&Source, Vec<f32>) -> Result<Vec<f32>>;

/// `img` resized to the model's square input, normalized, as NCHW. Transparent pixels sit on
/// mid gray so a cut-out layer reads as an object on a neutral backdrop.
pub fn to_tensor(img: &Image, src: &Source) -> Vec<f32> {
    let n = src.input;
    let filter = Filter::auto(img.width.max(img.height), n);
    let small = kernels::resize(img, n, n, filter);
    let plane = (n * n) as usize;
    let mut out = vec![0.0f32; 3 * plane];
    let (r, rest) = out.split_at_mut(plane);
    let (g, b) = rest.split_at_mut(plane);
    let planes = [r, g, b];
    for (c, dst) in planes.into_iter().enumerate() {
        dst.par_iter_mut().zip(small.data.par_chunks(4)).for_each(|(d, p)| {
            let a = p[3].clamp(0.0, 1.0);
            let v = p[c].clamp(0.0, 1.0) * a + 0.5 * (1.0 - a);
            *d = (v - src.mean[c]) / src.std[c];
        });
    }
    out
}

/// A model's raw `n×n` output as 0..1 probabilities.
pub fn read_map(data: &[f32], head: Head) -> Vec<f32> {
    match head {
        Head::Sigmoid => data.par_iter().map(|v| 1.0 / (1.0 + (-v).exp())).collect(),
        Head::MinMax => {
            let (lo, hi) = data.iter().fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), &v| (lo.min(v), hi.max(v)));
            if !(hi - lo).is_finite() || hi - lo < 1e-6 {
                return vec![0.0; data.len()];
            }
            data.par_iter().map(|v| (v - lo) / (hi - lo)).collect()
        }
    }
}

/// Luminance of straight-alpha sRGB pixels weighted by alpha (the guide for refinement).
fn guide_plane(img: &Image) -> Vec<f32> {
    img.data.par_chunks(4).map(|p| (0.299 * p[0] + 0.587 * p[1] + 0.114 * p[2]) * p[3]).collect()
}

/// Joint upsampling with a guided filter (He & Sun's fast guided filter): the linear
/// coefficients that map the guide's luminance to the low-resolution `map` are fitted at the
/// map's resolution (`radius` in map px), stretched to the full image, and applied to the
/// full-resolution guide, so the mask's edges come from the picture's own sharp edges.
pub fn guided_upsample(map: &Mask, img: &Image, radius: f64, eps: f32) -> Mask {
    let (n, m) = (map.width as usize, map.height as usize);
    let filter = Filter::auto(img.width.max(img.height), map.width.max(map.height));
    let small = kernels::resize(img, map.width, map.height, filter);
    let guide_small = guide_plane(&small);
    let mut stats: Vec<f32> =
        guide_small.par_iter().zip(map.data.par_iter()).flat_map_iter(|(&i, &p)| [i, p, i * i, i * p]).collect();
    kernels::gaussian_planar(&mut stats, n, m, 4, radius, radius, Edge::Clamp);
    let mut ab: Vec<f32> = stats
        .par_chunks(4)
        .flat_map_iter(|s| {
            let var = s[2] - s[0] * s[0];
            let cov = s[3] - s[0] * s[1];
            let a = cov / (var + eps);
            [a, s[1] - a * s[0]]
        })
        .collect();
    kernels::gaussian_planar(&mut ab, n, m, 2, radius, radius, Edge::Clamp);
    let guide = guide_plane(img);
    let (w, h) = (img.width as usize, img.height as usize);
    let mut out = Mask::new(img.width, img.height);
    out.data.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        // pixel-center aligned bilinear sampling of the (unclamped) coefficient planes
        let sy = ((y as f64 + 0.5) * m as f64 / h as f64 - 0.5).clamp(0.0, (m - 1) as f64);
        let (y0, fy) = (sy.floor() as usize, (sy - sy.floor()) as f32);
        let y1 = (y0 + 1).min(m - 1);
        for (x, o) in row.iter_mut().enumerate() {
            let sx = ((x as f64 + 0.5) * n as f64 / w as f64 - 0.5).clamp(0.0, (n - 1) as f64);
            let (x0, fx) = (sx.floor() as usize, (sx - sx.floor()) as f32);
            let x1 = (x0 + 1).min(n - 1);
            let at = |xx: usize, yy: usize, k: usize| ab[(yy * n + xx) * 2 + k];
            let lerp = |k: usize| {
                let top = at(x0, y0, k) + (at(x1, y0, k) - at(x0, y0, k)) * fx;
                let bot = at(x0, y1, k) + (at(x1, y1, k) - at(x0, y1, k)) * fx;
                top + (bot - top) * fy
            };
            *o = (lerp(0) * guide[y * w + x] + lerp(1)).clamp(0.0, 1.0);
        }
    });
    out
}

/// Auto refinement radius in canvas px: two pixels of the model's output map.
pub fn auto_refine(w: u32, h: u32, src: &Source) -> f64 {
    2.0 * w.max(h) as f64 / src.input as f64
}

/// Edge threshold of the refinement (guide variance; luminance in 0..1).
const REFINE_EPS: f32 = 1e-4;

/// Segment `img` (canvas-sized) into a soft mask: model input, inference, the output map
/// brought back to the canvas (guided joint upsampling within `refine` px, or plain bilinear
/// with 0), and the source's alpha multiplied in.
pub fn segment(img: &Image, src: &Source, infer: Infer<'_>, refine: f64) -> Result<Mask> {
    let n = src.input as usize;
    let raw = infer(src, to_tensor(img, src))?;
    if raw.len() != n * n {
        return Err(Error::runtime(format!("the model returned {} values, expected a {n}×{n} map", raw.len())));
    }
    let map = Mask { width: src.input, height: src.input, data: read_map(&raw, src.head) };
    let mut m = if refine > 0.0 {
        // the radius is given in canvas px; the filter runs at the map's resolution
        let r = (refine * src.input as f64 / img.width.max(img.height) as f64).max(0.5);
        guided_upsample(&map, img, r, REFINE_EPS)
    } else {
        kernels::resize_mask(&map, img.width, img.height, Filter::Bilinear)
    };
    m.data.par_iter_mut().zip(img.data.par_chunks(4)).for_each(|(v, p)| *v *= p[3].clamp(0.0, 1.0));
    Ok(m)
}

/// Round a mask exactly as the document stores it (8 or 16 bits).
pub fn quantize(m: &mut Mask, depth: u8) {
    let levels = if depth == 16 { 65535.0f32 } else { 255.0f32 };
    m.data.par_iter_mut().for_each(|v| *v = (v.clamp(0.0, 1.0) * levels).round() / levels);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::model;

    fn src(n: u32, head: Head) -> Source {
        Source { input: n, head, ..model("u2netp").unwrap().source.unwrap() }
    }

    /// A dark disc on a light backdrop.
    fn disc(w: u32, h: u32) -> Image {
        Image::from_fn(w, h, |x, y| {
            let (dx, dy) = (x as f32 - w as f32 / 2.0, y as f32 - h as f32 / 2.0);
            if (dx * dx + dy * dy).sqrt() < w.min(h) as f32 / 4.0 {
                [0.15, 0.1, 0.1, 1.0]
            } else {
                [0.9, 0.9, 0.85, 1.0]
            }
        })
    }

    #[test]
    fn tensor_layout_and_normalization() {
        let s = src(4, Head::MinMax);
        let img = Image::filled(8, 8, Rgba::new(1.0, 0.0, 0.5, 1.0));
        let t = to_tensor(&img, &s);
        assert_eq!(t.len(), 3 * 16);
        assert!((t[0] - (1.0 - 0.485) / 0.229).abs() < 1e-5);
        assert!((t[16] - (0.0 - 0.456) / 0.224).abs() < 1e-5);
        assert!((t[32] - (0.5 - 0.406) / 0.225).abs() < 1e-4);
        // a transparent pixel reads as mid gray
        let t = to_tensor(&Image::new(4, 4), &s);
        assert!((t[0] - (0.5 - 0.485) / 0.229).abs() < 1e-5);
    }

    #[test]
    fn heads() {
        let m = read_map(&[0.0, 2.0, -2.0], Head::Sigmoid);
        assert!((m[0] - 0.5).abs() < 1e-6 && m[1] > 0.88 && m[2] < 0.12);
        assert_eq!(read_map(&[2.0, 4.0, 3.0], Head::MinMax), vec![0.0, 1.0, 0.5]);
        assert_eq!(read_map(&[3.0, 3.0], Head::MinMax), vec![0.0, 0.0]);
    }

    #[test]
    fn segment_with_a_stand_in_model() {
        // the "model" thresholds the input's darkness: dark disc = subject
        let img = disc(120, 80);
        let s = src(32, Head::Sigmoid);
        let mut infer = |s: &Source, t: Vec<f32>| -> Result<Vec<f32>> {
            let plane = (s.input * s.input) as usize;
            Ok(t[..plane].iter().map(|v| if *v < 0.0 { 8.0 } else { -8.0 }).collect())
        };
        let m = segment(&img, &s, &mut infer, 3.0).unwrap();
        assert_eq!((m.width, m.height), (120, 80));
        assert!(m.at(60, 40) > 0.95, "center {}", m.at(60, 40));
        assert!(m.at(2, 2) < 0.05, "corner {}", m.at(2, 2));
        // the source's alpha cuts the mask
        let mut half = img.clone();
        for p in half.data.chunks_mut(4).take(120 * 40) {
            p[3] = 0.0;
        }
        let m2 = segment(&half, &s, &mut infer, 0.0).unwrap();
        assert_eq!(m2.at(60, 10), 0.0);
        // a wrong output size is an error, not a panic
        let mut bad = |_: &Source, _: Vec<f32>| -> Result<Vec<f32>> { Ok(vec![0.0; 7]) };
        assert!(segment(&img, &s, &mut bad, 0.0).is_err());
    }

    #[test]
    fn guided_upsampling_snaps_a_low_res_map_to_the_picture() {
        // a disc photo; the "model" saw it at a quarter of the resolution
        let img = disc(400, 400);
        let truth = Mask::from_fn(400, 400, |x, y| {
            let (dx, dy) = (x as f32 + 0.5 - 200.0, y as f32 + 0.5 - 200.0);
            if (dx * dx + dy * dy).sqrt() < 100.0 {
                1.0
            } else {
                0.0
            }
        });
        let small = kernels::resize_mask(&truth, 100, 100, Filter::Bilinear);
        let plain = kernels::resize_mask(&small, 400, 400, Filter::Bilinear);
        let refined = guided_upsample(&small, &img, 2.0, REFINE_EPS);
        let err = |m: &Mask| m.data.iter().zip(&truth.data).map(|(a, b)| (a - b).abs()).sum::<f32>();
        assert!(err(&refined) < err(&plain) * 0.75, "refined {} vs bilinear {}", err(&refined), err(&plain));
        assert!(refined.data.iter().all(|v| (0.0..=1.0).contains(v)));
        // a dark subject: the coefficients are negative, so they must not be clamped
        assert!(refined.at(200, 200) > 0.95 && refined.at(5, 5) < 0.05);
    }

    #[test]
    fn quantize_matches_storage() {
        let mut m = Mask::from_fn(3, 1, |x, _| [0.1234, 0.5, 1.2][x as usize]);
        quantize(&mut m, 8);
        assert_eq!(m.data, vec![31.0 / 255.0, 128.0 / 255.0, 1.0]);
    }
}
