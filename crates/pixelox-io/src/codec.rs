//! Raster codecs on top of the image-rs family: decode any supported format to an
//! [`Image`] (with ICC, EXIF, orientation, depth), and encode an [`Image`] to PNG, JPEG,
//! WebP (lossless), TIFF (LZW/Deflate), GIF, and BMP with metadata carried through.

use image::codecs::bmp::BmpEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::codecs::webp::WebPEncoder;
use image::{DynamicImage, ExtendedColorType, ImageDecoder, ImageEncoder, ImageReader};
use pixelox_core::ctx::{EncodeOptions, ImageInfo, OpenOptions};
use pixelox_core::envelope::Warning;
use pixelox_core::{Error, Image, Result, Rgba};
use serde_json::Map;
use std::io::Cursor;

/// Everything decoding learns about a file besides its pixels.
#[derive(Clone, Debug, Default)]
pub struct Decoded {
    pub info: ImageInfo,
    pub icc: Option<Vec<u8>>,
    pub exif: Option<Vec<u8>>,
    pub warnings: Vec<Warning>,
}

pub fn warning(code: &str, message: impl Into<String>) -> Warning {
    Warning { code: code.into(), message: message.into(), context: Map::new() }
}

fn decode_err(e: impl std::fmt::Display) -> Error {
    Error::new("PX1103", format!("cannot decode image: {e}"))
}

/// Sniff a format from magic bytes (falls back to the hint).
pub fn sniff(bytes: &[u8], hint: Option<&str>) -> Option<&'static str> {
    let b = bytes;
    if b.starts_with(b"\x89PNG") {
        Some("png")
    } else if b.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Some("jpeg")
    } else if b.starts_with(b"GIF8") {
        Some("gif")
    } else if b.len() > 12 && &b[0..4] == b"RIFF" && &b[8..12] == b"WEBP" {
        Some("webp")
    } else if b.starts_with(b"II*\0") || b.starts_with(b"MM\0*") {
        Some("tiff")
    } else if b.starts_with(b"BM") {
        Some("bmp")
    } else if b.starts_with(b"8BPS") {
        Some("psd")
    } else if b.starts_with(b"PK\x03\x04") {
        Some("pxo")
    } else {
        let head = String::from_utf8_lossy(&b[..b.len().min(512)]).to_ascii_lowercase();
        if head.contains("<svg") || (head.trim_start().starts_with("<?xml") && head.contains("svg")) {
            return Some("svg");
        }
        match hint.map(|h| h.trim_start_matches('.').to_ascii_lowercase()) {
            Some(h) if h == "svg" => Some("svg"),
            _ => None,
        }
    }
}

/// Convert a decoded image to pixelox's f32 RGBA; returns the source bit depth.
pub fn from_dynamic(img: DynamicImage) -> (Image, u8) {
    let (w, h) = (img.width(), img.height());
    match img {
        DynamicImage::ImageRgba32F(b) => (Image { width: w, height: h, data: b.into_raw() }, 16),
        DynamicImage::ImageRgb32F(_) => (Image { width: w, height: h, data: img.to_rgba32f().into_raw() }, 16),
        DynamicImage::ImageLuma16(_)
        | DynamicImage::ImageLumaA16(_)
        | DynamicImage::ImageRgb16(_)
        | DynamicImage::ImageRgba16(_) => {
            let b = img.to_rgba16();
            (Image::from_rgba16(w, h, b.as_raw()), 16)
        }
        _ => {
            let b = img.to_rgba8();
            (Image::from_rgba8(w, h, b.as_raw()), 8)
        }
    }
}

fn channels_of(img: &DynamicImage) -> (u8, bool) {
    let c = img.color();
    (c.channel_count(), c.has_alpha())
}

/// Decode raster bytes (not PSD/SVG/pxo; see `crate::open_bytes` for those).
pub fn decode_raster(bytes: &[u8], format: &str, opts: &OpenOptions) -> Result<(Image, Decoded)> {
    let mut dec = Decoded::default();
    let fmt = match format {
        "png" => image::ImageFormat::Png,
        "jpeg" => image::ImageFormat::Jpeg,
        "gif" => image::ImageFormat::Gif,
        "webp" => image::ImageFormat::WebP,
        "tiff" => image::ImageFormat::Tiff,
        "bmp" => image::ImageFormat::Bmp,
        other => return Err(Error::new("PX1105", format!("cannot decode format '{other}'"))),
    };
    if format == "tiff" && opts.page.map(|p| p > 1).unwrap_or(false) {
        return decode_tiff_page(bytes, opts.page.unwrap_or(1), opts);
    }
    if format == "gif" && opts.frame.map(|f| f > 1).unwrap_or(false) {
        let frames = crate::anim::decode_gif_frames(bytes)?;
        let n = opts.frame.unwrap_or(1);
        let total = frames.len();
        let (img, _) = frames.into_iter().nth(n - 1).ok_or_else(|| {
            Error::new("PX1010", format!("--frame {n} is out of range: the GIF has {total} frame(s)"))
        })?;
        dec.info = ImageInfo {
            format: "gif".into(),
            width: img.width,
            height: img.height,
            depth: 8,
            channels: 4,
            has_alpha: true,
            frames: Some(total),
            ..Default::default()
        };
        return Ok((img, dec));
    }
    let mut reader = ImageReader::with_format(Cursor::new(bytes), fmt);
    reader.no_limits();
    let mut decoder = reader.into_decoder().map_err(decode_err)?;
    let (w, h) = decoder.dimensions();
    if w as u64 * h as u64 > opts.limit_pixels {
        return Err(Error::new(
            "PX1104",
            format!(
                "image is {w}x{h} ({} MP), above the {} MP limit",
                (w as u64 * h as u64) / 1_000_000,
                opts.limit_pixels / 1_000_000
            ),
        )
        .with_hint("raise it with --limit-pixels <count> if the file is trusted"));
    }
    dec.icc = decoder.icc_profile().ok().flatten();
    dec.exif = decoder.exif_metadata().ok().flatten();
    let orientation = decoder.orientation().ok();
    let mut dynimg = DynamicImage::from_decoder(decoder).map_err(decode_err)?;
    let (channels, has_alpha) = channels_of(&dynimg);
    let mut oriented = None;
    if let Some(o) = orientation {
        if opts.auto_orient && o != image::metadata::Orientation::NoTransforms {
            dynimg.apply_orientation(o);
            oriented = Some(format!("{o:?}"));
            if let Some(ex) = dec.exif.as_mut() {
                crate::meta::reset_orientation(ex);
            }
        }
    }
    let (img, depth) = from_dynamic(dynimg);
    if let Some(o) = oriented {
        dec.warnings.push(warning("PXW0102", format!("EXIF orientation applied on open ({o})")));
    }
    dec.info = ImageInfo {
        format: format.into(),
        width: img.width,
        height: img.height,
        depth,
        channels,
        has_alpha,
        frames: None,
        icc: dec.icc.as_deref().map(crate::meta::icc_name),
        exif: dec.exif.is_some(),
        dpi: None,
        orientation: None,
    };
    if format == "gif" {
        dec.info.frames = crate::anim::count_gif_frames(bytes).ok();
    }
    Ok((img, dec))
}

/// Header-level facts about raster bytes without decoding the pixels.
pub fn probe_raster(bytes: &[u8], format: &str) -> Result<ImageInfo> {
    let mut reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format().map_err(decode_err)?;
    reader.no_limits();
    let mut d = reader.into_decoder().map_err(decode_err)?;
    let (width, height) = d.dimensions();
    let ct = d.color_type();
    let icc = d.icc_profile().ok().flatten();
    let exif = d.exif_metadata().ok().flatten();
    let orientation = d.orientation().ok().map(|o| o.to_exif() as u16);
    Ok(ImageInfo {
        format: format.into(),
        width,
        height,
        depth: (ct.bits_per_pixel() / ct.channel_count() as u16) as u8,
        channels: ct.channel_count(),
        has_alpha: ct.has_alpha(),
        frames: None,
        icc: icc.as_deref().map(crate::meta::icc_name),
        exif: exif.is_some(),
        dpi: None,
        orientation,
    })
}

fn decode_tiff_page(bytes: &[u8], page: usize, _opts: &OpenOptions) -> Result<(Image, Decoded)> {
    use tiff::decoder::{Decoder, DecodingResult};
    use tiff::ColorType;
    let mut d = Decoder::new(Cursor::new(bytes)).map_err(decode_err)?;
    d.seek_to_image(page - 1)
        .map_err(|_| Error::new("PX1010", format!("--page {page} does not exist in this TIFF")))?;
    let (w, h) = d.dimensions().map_err(decode_err)?;
    let ct = d.colortype().map_err(decode_err)?;
    let data = d.read_image().map_err(decode_err)?;
    let (ch, alpha) = match ct {
        ColorType::Gray(_) => (1usize, false),
        ColorType::GrayA(_) => (2, true),
        ColorType::RGB(_) => (3, false),
        ColorType::RGBA(_) => (4, true),
        other => return Err(Error::unsupported(format!("TIFF color type {other:?} on page {page} is not supported"))),
    };
    let (samples, depth): (Vec<f32>, u8) = match data {
        DecodingResult::U8(v) => (v.iter().map(|&x| x as f32 / 255.0).collect(), 8),
        DecodingResult::U16(v) => (v.iter().map(|&x| x as f32 / 65535.0).collect(), 16),
        DecodingResult::F32(v) => (v, 16),
        _ => return Err(Error::unsupported(format!("TIFF sample format on page {page} is not supported"))),
    };
    let mut img = Image::new(w, h);
    for (i, px) in img.data.chunks_mut(4).enumerate() {
        let s = &samples[i * ch..i * ch + ch];
        match ch {
            1 => px.copy_from_slice(&[s[0], s[0], s[0], 1.0]),
            2 => px.copy_from_slice(&[s[0], s[0], s[0], s[1]]),
            3 => px.copy_from_slice(&[s[0], s[1], s[2], 1.0]),
            _ => px.copy_from_slice(&[s[0], s[1], s[2], s[3]]),
        }
    }
    let dec = Decoded {
        info: ImageInfo {
            format: "tiff".into(),
            width: w,
            height: h,
            depth,
            channels: ch as u8,
            has_alpha: alpha,
            ..Default::default()
        },
        ..Default::default()
    };
    Ok((img, dec))
}

fn u16_bytes(v: &[u16]) -> Vec<u8> {
    let mut out = Vec::with_capacity(v.len() * 2);
    for x in v {
        out.extend_from_slice(&x.to_ne_bytes());
    }
    out
}

fn rgb8(img: &Image) -> Vec<u8> {
    img.data
        .chunks(4)
        .flat_map(|p| {
            [pixelox_core::color::to_u8(p[0]), pixelox_core::color::to_u8(p[1]), pixelox_core::color::to_u8(p[2])]
        })
        .collect()
}

fn rgb16(img: &Image) -> Vec<u16> {
    img.data
        .chunks(4)
        .flat_map(|p| {
            [pixelox_core::color::to_u16(p[0]), pixelox_core::color::to_u16(p[1]), pixelox_core::color::to_u16(p[2])]
        })
        .collect()
}

fn enc_err(e: impl std::fmt::Display) -> Error {
    Error::new("PX1107", format!("encode failed: {e}"))
}

/// Metadata to embed.
#[derive(Clone, Debug, Default)]
pub struct EmbedMeta {
    pub icc: Option<Vec<u8>>,
    pub exif: Option<Vec<u8>>,
}

/// Encode `img` per `opts`; returns bytes and warnings.
pub fn encode(img: &Image, opts: &EncodeOptions, meta: &EmbedMeta) -> Result<(Vec<u8>, Vec<Warning>)> {
    let mut warnings = Vec::new();
    let (w, h) = (img.width, img.height);
    let opaque = img.is_opaque();
    let want16 = opts.bit_depth == Some(16);
    let mut out = Vec::new();
    match opts.format.as_str() {
        "png" => {
            let mut e = PngEncoder::new_with_quality(&mut out, CompressionType::Default, FilterType::Adaptive);
            if let Some(icc) = &meta.icc {
                let _ = e.set_icc_profile(icc.clone());
            }
            if let Some(ex) = &meta.exif {
                let _ = e.set_exif_metadata(ex.clone());
            }
            match (want16, opaque) {
                (true, true) => e.write_image(&u16_bytes(&rgb16(img)), w, h, ExtendedColorType::Rgb16),
                (true, false) => e.write_image(&u16_bytes(&img.to_rgba16()), w, h, ExtendedColorType::Rgba16),
                (false, true) => e.write_image(&rgb8(img), w, h, ExtendedColorType::Rgb8),
                (false, false) => e.write_image(&img.to_rgba8(), w, h, ExtendedColorType::Rgba8),
            }
            .map_err(enc_err)?;
        }
        "jpeg" => {
            let flat = if opaque {
                img.clone()
            } else {
                warnings.push(warning("PXW0202", "JPEG cannot store transparency; alpha was flattened onto white"));
                img.flatten_onto(Rgba::WHITE)
            };
            if want16 {
                warnings.push(warning("PXW0203", "JPEG stores 8 bits per channel; written at 8 bits"));
            }
            if opts.progressive {
                warnings
                    .push(warning("PXW0203", "progressive JPEG encoding is not in this build; wrote baseline JPEG"));
            }
            let q = opts.quality.unwrap_or(90).clamp(1, 100);
            let mut e = JpegEncoder::new_with_quality(&mut out, q);
            if let Some(icc) = &meta.icc {
                let _ = e.set_icc_profile(icc.clone());
            }
            if let Some(ex) = &meta.exif {
                let _ = e.set_exif_metadata(ex.clone());
            }
            e.write_image(&rgb8(&flat), w, h, ExtendedColorType::Rgb8).map_err(enc_err)?;
        }
        "webp" => {
            if opts.lossless == Some(false) || opts.quality.is_some() {
                warnings.push(warning("PXW0204", "lossy WebP encoding is not in this build; wrote lossless WebP"));
            }
            if want16 {
                warnings.push(warning("PXW0203", "WebP stores 8 bits per channel; written at 8 bits"));
            }
            let mut e = WebPEncoder::new_lossless(&mut out);
            if let Some(icc) = &meta.icc {
                let _ = e.set_icc_profile(icc.clone());
            }
            if let Some(ex) = &meta.exif {
                let _ = e.set_exif_metadata(ex.clone());
            }
            if opaque {
                e.write_image(&rgb8(img), w, h, ExtendedColorType::Rgb8)
            } else {
                e.write_image(&img.to_rgba8(), w, h, ExtendedColorType::Rgba8)
            }
            .map_err(enc_err)?;
        }
        "tiff" => {
            out = encode_tiff(img, opts, meta, want16, opaque)?;
        }
        "gif" => {
            if want16 {
                warnings.push(warning("PXW0203", "GIF is palette-based; written at 8 bits"));
            }
            out = crate::anim::encode_gif(
                &[(img.clone(), 0)],
                opts.colors.unwrap_or(256),
                opts.dither.as_deref().unwrap_or("fs"),
                None,
            )?;
        }
        "bmp" => {
            if want16 {
                warnings.push(warning("PXW0203", "BMP stores 8 bits per channel; written at 8 bits"));
            }
            let mut e = BmpEncoder::new(&mut out);
            if opaque {
                e.encode(&rgb8(img), w, h, ExtendedColorType::Rgb8)
            } else {
                e.encode(&img.to_rgba8(), w, h, ExtendedColorType::Rgba8)
            }
            .map_err(enc_err)?;
        }
        other => {
            return Err(Error::new("PX1106", format!("cannot write format '{other}'"))
                .with_hint("use .png, .jpg, .webp, .tiff, .gif, .bmp, .psd, or .pxo"))
        }
    }
    Ok((out, warnings))
}

fn encode_tiff(img: &Image, opts: &EncodeOptions, meta: &EmbedMeta, want16: bool, opaque: bool) -> Result<Vec<u8>> {
    use tiff::encoder::{colortype, Compression, DeflateLevel, TiffEncoder};
    use tiff::tags::Tag;
    let compression = match opts.compression.as_deref().unwrap_or("lzw") {
        "lzw" => Compression::Lzw,
        "deflate" => Compression::Deflate(DeflateLevel::Balanced),
        "none" => Compression::Uncompressed,
        other => {
            return Err(Error::new("PX1010", format!("--compression must be lzw, deflate, or none, got '{other}'")))
        }
    };
    let mut cur = Cursor::new(Vec::new());
    {
        let mut enc = TiffEncoder::new(&mut cur).map_err(enc_err)?.with_compression(compression);
        let (w, h) = (img.width, img.height);
        macro_rules! write_with {
            ($ct:ty, $data:expr) => {{
                let mut ie = enc.new_image::<$ct>(w, h).map_err(enc_err)?;
                if let Some(icc) = &meta.icc {
                    ie.encoder().write_tag(Tag::Unknown(34675), &icc[..]).map_err(enc_err)?;
                }
                ie.write_data(&$data).map_err(enc_err)?;
            }};
        }
        match (want16, opaque) {
            (true, true) => write_with!(colortype::RGB16, rgb16(img)),
            (true, false) => write_with!(colortype::RGBA16, img.to_rgba16()),
            (false, true) => write_with!(colortype::RGB8, rgb8(img)),
            (false, false) => write_with!(colortype::RGBA8, img.to_rgba8()),
        }
    }
    Ok(cur.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Image {
        Image::from_fn(16, 8, |x, y| [x as f32 / 15.0, y as f32 / 7.0, 0.5, if x < 8 { 1.0 } else { 0.5 }])
    }

    fn opts(f: &str) -> EncodeOptions {
        EncodeOptions { format: f.into(), ..Default::default() }
    }

    #[test]
    fn png_round_trip_8_and_16() {
        let img = sample();
        let (bytes, w) = encode(&img, &opts("png"), &EmbedMeta::default()).unwrap();
        assert!(w.is_empty());
        let (back, dec) = decode_raster(&bytes, "png", &OpenOptions::default()).unwrap();
        assert_eq!(dec.info.depth, 8);
        assert!(pixelox_core::testing::max_diff(&img, &back) <= 1.0 / 255.0 + 1e-6);
        let mut o = opts("png");
        o.bit_depth = Some(16);
        let (bytes, _) = encode(&img, &o, &EmbedMeta::default()).unwrap();
        let (back, dec) = decode_raster(&bytes, "png", &OpenOptions::default()).unwrap();
        assert_eq!(dec.info.depth, 16);
        assert!(pixelox_core::testing::max_diff(&img, &back) <= 1.0 / 65535.0 + 1e-6);
    }

    #[test]
    fn every_writer_decodes_back() {
        let img = sample();
        for f in ["jpeg", "webp", "tiff", "gif", "bmp"] {
            let (bytes, _) = encode(&img, &opts(f), &EmbedMeta::default()).unwrap();
            assert_eq!(sniff(&bytes, None), Some(f));
            let (back, _) = decode_raster(&bytes, f, &OpenOptions::default()).unwrap();
            assert_eq!((back.width, back.height), (16, 8), "{f}");
        }
        let (_, warns) = encode(&img, &opts("jpeg"), &EmbedMeta::default()).unwrap();
        assert_eq!(warns[0].code, "PXW0202");
    }

    #[test]
    fn deterministic_bytes() {
        let img = sample();
        for f in ["png", "jpeg", "webp", "tiff", "gif", "bmp"] {
            let a = encode(&img, &opts(f), &EmbedMeta::default()).unwrap().0;
            let b = encode(&img, &opts(f), &EmbedMeta::default()).unwrap().0;
            assert_eq!(a, b, "{f}");
        }
    }

    #[test]
    fn pixel_limit_guard() {
        let (bytes, _) = encode(&sample(), &opts("png"), &EmbedMeta::default()).unwrap();
        let o = OpenOptions { limit_pixels: 10, ..Default::default() };
        assert_eq!(decode_raster(&bytes, "png", &o).unwrap_err().code, "PX1104");
    }

    #[test]
    fn metadata_round_trip_png_jpeg() {
        let meta = EmbedMeta { icc: None, exif: Some(crate::meta::tests_exif_with_orientation(6)) };
        for f in ["png", "jpeg"] {
            let (bytes, _) = encode(&sample(), &opts(f), &meta).unwrap();
            let o = OpenOptions { auto_orient: true, ..Default::default() };
            let (img, dec) = decode_raster(&bytes, f, &o).unwrap();
            // orientation 6 = rotate 90° clockwise: 16x8 → 8x16
            assert_eq!((img.width, img.height), (8, 16), "{f}");
            assert!(dec.warnings.iter().any(|w| w.code == "PXW0102"));
            assert_eq!(crate::meta::orientation(dec.exif.as_ref().unwrap()), Some(1));
        }
    }
}
