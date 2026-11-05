//! # pixelox-io
//!
//! Codecs and containers (PRD §7.6, §7.12): JPEG, PNG (8/16-bit), GIF (animated), TIFF
//! (multi-page, LZW/Deflate), WebP (lossless write), BMP, layered PSD read with a basic
//! PSD writer, SVG read through resvg, and the `.pxo` document format. Implements
//! [`ImageIo`] for the executor and defines `new`, `open`, `save`, `export`, `info`,
//! `frames extract`, `frames assemble`, and `analyze exif`.

#![allow(clippy::too_many_arguments)]

pub mod anim;
pub mod codec;
pub mod meta;
pub mod ops;
pub mod psd;
pub mod pxo;
pub mod svg;

use codec::{sniff, EmbedMeta};
use pixelox_core::ctx::{format_of, EncodeOptions, ImageInfo, ImageIo, OpenOptions};
use pixelox_core::envelope::Warning;
use pixelox_core::{compose, Document, Error, Image, Result};
use std::path::Path;

pub use ops::ops;

/// Formats this build reads and writes.
pub const READ_FORMATS: &[&str] = &["png", "jpeg", "gif", "tiff", "webp", "bmp", "psd", "svg", "pxo"];
pub const WRITE_FORMATS: &[&str] = &["png", "jpeg", "gif", "tiff", "webp", "bmp", "psd", "pxo"];

/// Install the asset decoder the compositor uses for embedded images.
pub fn install() {
    compose::set_decoder(|bytes, hint| {
        decode_bytes(bytes, hint, &OpenOptions { limit_pixels: u64::MAX, ..Default::default() }).map(|(i, _)| i)
    });
}

fn detect(bytes: &[u8], hint: Option<&str>) -> Result<&'static str> {
    sniff(bytes, hint).ok_or_else(|| {
        Error::new(
            "PX1105",
            format!("unrecognized image format{}", hint.map(|h| format!(" (.{h})")).unwrap_or_default()),
        )
        .with_hint("supported: PNG, JPEG, GIF, TIFF, WebP, BMP, PSD, SVG, .pxo")
    })
}

/// Decode any supported bytes to one flattened image.
pub fn decode_bytes(bytes: &[u8], hint: Option<&str>, opts: &OpenOptions) -> Result<(Image, ImageInfo)> {
    let fmt = detect(bytes, hint)?;
    match fmt {
        "svg" => {
            let img = svg::decode(bytes, opts.limit_pixels)?;
            let info = ImageInfo {
                format: "svg".into(),
                width: img.width,
                height: img.height,
                depth: 8,
                channels: 4,
                has_alpha: true,
                ..Default::default()
            };
            Ok((img, info))
        }
        "psd" | "pxo" => {
            let doc = if fmt == "psd" { psd::read(bytes)?.0 } else { pxo::read(bytes)? };
            let img = compose::composite(&doc)?;
            let info = ImageInfo {
                format: fmt.into(),
                width: img.width,
                height: img.height,
                depth: doc.depth,
                channels: 4,
                has_alpha: true,
                ..Default::default()
            };
            Ok((img, info))
        }
        raster => {
            let (img, dec) = codec::decode_raster(bytes, raster, opts)?;
            Ok((img, dec.info))
        }
    }
}

/// The IO layer the CLI injects into the executor.
#[derive(Default, Clone, Copy)]
pub struct Io;

impl ImageIo for Io {
    fn open(&self, path: &Path, opts: &OpenOptions) -> Result<(Document, Vec<Warning>)> {
        let bytes = std::fs::read(path).map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => Error::input_missing(path),
            _ => Error::new("PX1102", format!("cannot read {}: {e}", path.display())),
        })?;
        let ext = path.extension().map(|e| e.to_string_lossy().to_ascii_lowercase());
        let fmt = detect(&bytes, ext.as_deref())?;
        match fmt {
            "pxo" => Ok((pxo::read(&bytes)?, vec![])),
            "psd" => psd::read(&bytes),
            "svg" => {
                let img = svg::decode(&bytes, opts.limit_pixels)?;
                let mut d = Document::from_image(img);
                d.meta.source_format = Some("svg".into());
                d.dpi = 96.0;
                Ok((d, vec![]))
            }
            raster => {
                let (img, dec) = codec::decode_raster(&bytes, raster, opts)?;
                let mut warnings = dec.warnings;
                let mut d = Document::from_image(img);
                d.depth = if dec.info.depth >= 16 { 16 } else { 8 };
                d.meta.source_format = Some(raster.to_string());
                d.meta.exif = dec.exif;
                match (opts.icc.as_str(), dec.icc) {
                    ("ignore", _) | (_, None) => {}
                    (mode, Some(icc)) => {
                        let name = meta::icc_name(&icc);
                        if !meta::is_srgb(&icc) && mode == "honor" {
                            warnings.push(codec::warning(
                                "PXW0101",
                                format!("the input's ICC profile '{name}' was assigned, not converted (no color-management engine in this build); it is embedded again on export"),
                            ));
                        }
                        d.meta.icc_name = Some(name);
                        d.meta.icc = Some(icc);
                    }
                }
                Ok((d, warnings))
            }
        }
    }

    fn decode(&self, bytes: &[u8], hint: Option<&str>, opts: &OpenOptions) -> Result<(Image, ImageInfo)> {
        decode_bytes(bytes, hint, opts)
    }

    fn probe(&self, path: &Path) -> Result<ImageInfo> {
        let bytes = std::fs::read(path).map_err(|_| Error::input_missing(path))?;
        let ext = path.extension().map(|e| e.to_string_lossy().to_ascii_lowercase());
        let fmt = detect(&bytes, ext.as_deref())?;
        match fmt {
            "pxo" => {
                let d = pxo::read(&bytes)?;
                Ok(ImageInfo {
                    format: "pxo".into(),
                    width: d.width,
                    height: d.height,
                    depth: d.depth,
                    channels: 4,
                    has_alpha: true,
                    ..Default::default()
                })
            }
            "psd" | "svg" => decode_bytes(&bytes, ext.as_deref(), &OpenOptions::default()).map(|(_, i)| i),
            raster => {
                let mut info = codec::probe_raster(&bytes, raster)?;
                if raster == "gif" {
                    info.frames = anim::count_gif_frames(&bytes).ok();
                }
                Ok(info)
            }
        }
    }

    fn encode(&self, image: &Image, doc: Option<&Document>, path: &Path, opts: &EncodeOptions) -> Result<Vec<Warning>> {
        let format =
            if opts.format.is_empty() { format_of(path).unwrap_or_else(|| "png".into()) } else { opts.format.clone() };
        let doc_icc = doc.and_then(|d| d.meta.icc.clone());
        if let Some(p) = &opts.profile {
            let same = match (&doc_icc, p.as_str()) {
                (None, "srgb") => true,
                (Some(icc), "srgb") => meta::is_srgb(icc),
                _ => false,
            };
            if !same {
                return Err(Error::new("PX1520", format!("converting to the '{p}' profile needs a color-management engine, which this build does not include"))
                    .with_hint("omit --profile to keep the document's own profile (it is embedded in the output)"));
            }
        }
        let meta = EmbedMeta {
            icc: doc_icc,
            exif: if opts.strip_metadata { None } else { doc.and_then(|d| d.meta.exif.clone()) },
        };
        let mut o = opts.clone();
        o.format = format.clone();
        let (bytes, warnings) =
            if format == "psd" { psd::write(image, doc, opts.layered)? } else { codec::encode(image, &o, &meta)? };
        std::fs::write(path, bytes)?;
        Ok(warnings)
    }

    fn save_pxo(&self, doc: &Document, path: &Path) -> Result<()> {
        std::fs::write(path, pxo::write(doc)?)?;
        Ok(())
    }

    fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (READ_FORMATS.to_vec(), WRITE_FORMATS.to_vec())
    }
}
