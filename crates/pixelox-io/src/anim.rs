//! Animated GIF: frame decoding (composited, with delays) and encoding with a
//! deterministic median-cut quantizer and Floyd–Steinberg or ordered dithering.
//! (libimagequant is GPL-dual and deliberately not used; PRD §7.6.)

use image::{AnimationDecoder, ImageDecoder};
use pixelox_core::{Error, Image, Result};
use std::io::Cursor;

/// Decode every frame of a GIF, composited to full size, with its delay in ms.
pub fn decode_gif_frames(bytes: &[u8]) -> Result<Vec<(Image, u32)>> {
    let dec = image::codecs::gif::GifDecoder::new(Cursor::new(bytes))
        .map_err(|e| Error::new("PX1103", format!("cannot decode GIF: {e}")))?;
    let frames = dec
        .into_frames()
        .collect_frames()
        .map_err(|e| Error::new("PX1103", format!("cannot decode GIF frames: {e}")))?;
    Ok(frames
        .into_iter()
        .map(|f| {
            let (n, d) = f.delay().numer_denom_ms();
            let ms = n.checked_div(d).unwrap_or(0);
            let buf = f.into_buffer();
            (Image::from_rgba8(buf.width(), buf.height(), buf.as_raw()), ms)
        })
        .collect())
}

pub fn count_gif_frames(bytes: &[u8]) -> Result<usize> {
    let mut opts = gif::DecodeOptions::new();
    opts.set_color_output(gif::ColorOutput::Indexed);
    let mut d =
        opts.read_info(Cursor::new(bytes)).map_err(|e| Error::new("PX1103", format!("cannot read GIF: {e}")))?;
    let mut n = 0;
    while d.next_frame_info().map_err(|e| Error::new("PX1103", format!("cannot read GIF: {e}")))?.is_some() {
        n += 1;
    }
    Ok(n)
}

/// Dimensions of a GIF without decoding pixels.
pub fn gif_dims(bytes: &[u8]) -> Result<(u32, u32)> {
    let d = image::codecs::gif::GifDecoder::new(Cursor::new(bytes))
        .map_err(|e| Error::new("PX1103", format!("cannot read GIF: {e}")))?;
    Ok(d.dimensions())
}

#[derive(Clone)]
struct Bin {
    count: u64,
    sum: [u64; 3],
    key: u16,
}

/// Median-cut palette (at most `n` colors) from opaque pixels, deterministic.
fn median_cut(img: &Image, n: usize) -> Vec<[u8; 3]> {
    let mut hist = vec![(0u64, [0u64; 3]); 1 << 15];
    for p in img.data.chunks(4) {
        if p[3] < 0.5 {
            continue;
        }
        let c = [pixelox_core::color::to_u8(p[0]), pixelox_core::color::to_u8(p[1]), pixelox_core::color::to_u8(p[2])];
        let k = ((c[0] as usize >> 3) << 10) | ((c[1] as usize >> 3) << 5) | (c[2] as usize >> 3);
        hist[k].0 += 1;
        for (acc, v) in hist[k].1.iter_mut().zip(c) {
            *acc += v as u64;
        }
    }
    let bins: Vec<Bin> = hist
        .iter()
        .enumerate()
        .filter(|(_, h)| h.0 > 0)
        .map(|(k, h)| Bin { count: h.0, sum: h.1, key: k as u16 })
        .collect();
    if bins.is_empty() {
        return vec![[0, 0, 0]];
    }
    let comp = |b: &Bin, ch: usize| -> u16 {
        match ch {
            0 => b.key >> 10,
            1 => (b.key >> 5) & 31,
            _ => b.key & 31,
        }
    };
    let mut boxes: Vec<Vec<Bin>> = vec![bins];
    while boxes.len() < n {
        // split the box with the largest (range × population)
        let mut best: Option<(usize, usize, u64)> = None;
        for (i, b) in boxes.iter().enumerate() {
            if b.len() < 2 {
                continue;
            }
            for ch in 0..3 {
                let lo = b.iter().map(|x| comp(x, ch)).min().unwrap_or(0);
                let hi = b.iter().map(|x| comp(x, ch)).max().unwrap_or(0);
                let pop: u64 = b.iter().map(|x| x.count).sum();
                let score = (hi - lo) as u64 * pop;
                if score > 0 && best.map(|(_, _, s)| score > s).unwrap_or(true) {
                    best = Some((i, ch, score));
                }
            }
        }
        let Some((i, ch, _)) = best else { break };
        let mut b = boxes.swap_remove(i);
        b.sort_by_key(|x| (comp(x, ch), x.key));
        let total: u64 = b.iter().map(|x| x.count).sum();
        let mut acc = 0;
        let mut cut = 1;
        for (j, x) in b.iter().enumerate() {
            acc += x.count;
            if acc * 2 >= total {
                cut = (j + 1).clamp(1, b.len() - 1);
                break;
            }
        }
        let right = b.split_off(cut);
        boxes.push(b);
        boxes.push(right);
    }
    let mut pal: Vec<[u8; 3]> = boxes
        .iter()
        .map(|b| {
            let c: u64 = b.iter().map(|x| x.count).sum::<u64>().max(1);
            let mut s = [0u64; 3];
            for x in b {
                for (acc, v) in s.iter_mut().zip(x.sum) {
                    *acc += v;
                }
            }
            [(s[0] / c) as u8, (s[1] / c) as u8, (s[2] / c) as u8]
        })
        .collect();
    pal.sort();
    pal.dedup();
    pal
}

fn nearest(pal: &[[u8; 3]], c: [f32; 3]) -> usize {
    let mut best = 0;
    let mut bd = f32::MAX;
    for (i, p) in pal.iter().enumerate() {
        let d = (p[0] as f32 - c[0]).powi(2) + (p[1] as f32 - c[1]).powi(2) + (p[2] as f32 - c[2]).powi(2);
        if d < bd {
            bd = d;
            best = i;
        }
    }
    best
}

const BAYER8: [[u8; 8]; 8] = [
    [0, 32, 8, 40, 2, 34, 10, 42],
    [48, 16, 56, 24, 50, 18, 58, 26],
    [12, 44, 4, 36, 14, 46, 6, 38],
    [60, 28, 52, 20, 62, 30, 54, 22],
    [3, 35, 11, 43, 1, 33, 9, 41],
    [51, 19, 59, 27, 49, 17, 57, 25],
    [15, 47, 7, 39, 13, 45, 5, 37],
    [63, 31, 55, 23, 61, 29, 53, 21],
];

/// Quantize to a palette; returns (palette, indices, transparent index).
pub fn quantize(img: &Image, colors: u16, dither: &str) -> Result<(Vec<[u8; 3]>, Vec<u8>, Option<u8>)> {
    let has_transparency = img.data.chunks(4).any(|p| p[3] < 0.5);
    let max = (colors.clamp(2, 256) as usize) - usize::from(has_transparency);
    let pal = median_cut(img, max.max(1));
    let (w, h) = (img.width as usize, img.height as usize);
    // lookup cache on 5-bit keys for the undithered path
    let mut cache = vec![u16::MAX; 1 << 15];
    let mut idx = vec![0u8; w * h];
    let transparent = if has_transparency { Some(pal.len() as u8) } else { None };
    match dither {
        "fs" => {
            let mut err = vec![[0f32; 3]; (w + 2) * 2];
            for y in 0..h {
                let (cur, next) = err.split_at_mut(w + 2);
                let (cur, next) = if y % 2 == 0 { (cur, next) } else { (next, cur) };
                for e in next.iter_mut() {
                    *e = [0.0; 3];
                }
                for x in 0..w {
                    let p = &img.data[(y * w + x) * 4..(y * w + x) * 4 + 4];
                    if p[3] < 0.5 {
                        idx[y * w + x] = transparent.unwrap_or(0);
                        continue;
                    }
                    let c = [
                        (p[0] * 255.0 + cur[x + 1][0]).clamp(0.0, 255.0),
                        (p[1] * 255.0 + cur[x + 1][1]).clamp(0.0, 255.0),
                        (p[2] * 255.0 + cur[x + 1][2]).clamp(0.0, 255.0),
                    ];
                    let i = nearest(&pal, c);
                    idx[y * w + x] = i as u8;
                    let q = pal[i];
                    for ch in 0..3 {
                        let e = c[ch] - q[ch] as f32;
                        cur[x + 2][ch] += e * 7.0 / 16.0;
                        next[x][ch] += e * 3.0 / 16.0;
                        next[x + 1][ch] += e * 5.0 / 16.0;
                        next[x + 2][ch] += e / 16.0;
                    }
                }
            }
        }
        "ordered" | "none" => {
            let ordered = dither == "ordered";
            for y in 0..h {
                for x in 0..w {
                    let p = &img.data[(y * w + x) * 4..(y * w + x) * 4 + 4];
                    if p[3] < 0.5 {
                        idx[y * w + x] = transparent.unwrap_or(0);
                        continue;
                    }
                    let t = if ordered { (BAYER8[y % 8][x % 8] as f32 / 64.0 - 0.5) * 32.0 } else { 0.0 };
                    let c = [
                        (p[0] * 255.0 + t).clamp(0.0, 255.0),
                        (p[1] * 255.0 + t).clamp(0.0, 255.0),
                        (p[2] * 255.0 + t).clamp(0.0, 255.0),
                    ];
                    let k = ((c[0] as usize >> 3) << 10) | ((c[1] as usize >> 3) << 5) | (c[2] as usize >> 3);
                    if !ordered {
                        if cache[k] == u16::MAX {
                            cache[k] = nearest(&pal, c) as u16;
                        }
                        idx[y * w + x] = cache[k] as u8;
                    } else {
                        idx[y * w + x] = nearest(&pal, c) as u8;
                    }
                }
            }
        }
        other => return Err(Error::new("PX1010", format!("--dither must be fs, ordered, or none, got '{other}'"))),
    }
    Ok((pal, idx, transparent))
}

/// Encode frames (`(image, delay_ms)`) as a GIF; `repeat` None = play once, Some(0) = forever.
pub fn encode_gif(frames: &[(Image, u32)], colors: u16, dither: &str, repeat: Option<u16>) -> Result<Vec<u8>> {
    let first = frames.first().ok_or_else(|| Error::new("PX1003", "no frames to encode"))?;
    let (w, h) = (first.0.width, first.0.height);
    if w > 65535 || h > 65535 {
        return Err(Error::new("PX1107", "GIF dimensions are limited to 65535 px"));
    }
    let mut out = Vec::new();
    {
        let mut enc = gif::Encoder::new(&mut out, w as u16, h as u16, &[])
            .map_err(|e| Error::new("PX1107", format!("GIF encode failed: {e}")))?;
        if frames.len() > 1 || repeat.is_some() {
            let r = match repeat {
                None | Some(0) => gif::Repeat::Infinite,
                Some(n) => gif::Repeat::Finite(n),
            };
            enc.set_repeat(r).map_err(|e| Error::new("PX1107", format!("GIF encode failed: {e}")))?;
        }
        for (img, delay) in frames {
            if (img.width, img.height) != (w, h) {
                return Err(Error::new(
                    "PX1010",
                    format!(
                        "frame is {}x{}, the first frame is {w}x{h}; frames must share one size",
                        img.width, img.height
                    ),
                ));
            }
            let (pal, idx, transparent) = quantize(img, colors, dither)?;
            let mut palette: Vec<u8> = pal.iter().flat_map(|c| c.iter().copied()).collect();
            if transparent.is_some() {
                palette.extend_from_slice(&[0, 0, 0]);
            }
            let mut f = gif::Frame { width: w as u16, height: h as u16, ..Default::default() };
            f.buffer = std::borrow::Cow::Owned(idx);
            f.palette = Some(palette);
            f.transparent = transparent;
            f.delay = ((*delay as f64) / 10.0).round() as u16;
            f.dispose = gif::DisposalMethod::Background;
            enc.write_frame(&f).map_err(|e| Error::new("PX1107", format!("GIF encode failed: {e}")))?;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantizer_keeps_flat_colors_exact() {
        let img = Image::from_fn(8, 8, |x, _| if x < 4 { [1.0, 0.0, 0.0, 1.0] } else { [0.0, 0.0, 1.0, 1.0] });
        for d in ["fs", "ordered", "none"] {
            let (pal, idx, t) = quantize(&img, 256, d).unwrap();
            assert!(t.is_none());
            assert!(pal.contains(&[255, 0, 0]) && pal.contains(&[0, 0, 255]), "{d}");
            assert_eq!(pal[idx[0] as usize], [255, 0, 0], "{d}");
            assert_eq!(pal[idx[7] as usize], [0, 0, 255], "{d}");
        }
    }

    #[test]
    fn animation_round_trip() {
        let frames: Vec<(Image, u32)> = (0..3)
            .map(|i| {
                (
                    Image::from_fn(
                        10,
                        6,
                        move |x, _| if x as i32 == i { [1.0, 1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0, 1.0] },
                    ),
                    100,
                )
            })
            .collect();
        let bytes = encode_gif(&frames, 16, "none", Some(0)).unwrap();
        assert_eq!(count_gif_frames(&bytes).unwrap(), 3);
        let back = decode_gif_frames(&bytes).unwrap();
        assert_eq!(back.len(), 3);
        assert_eq!(back[1].1, 100);
        assert_eq!(back[1].0.get(1, 0)[0], 1.0);
        assert_eq!(encode_gif(&frames, 16, "none", Some(0)).unwrap(), bytes, "deterministic");
    }
}
