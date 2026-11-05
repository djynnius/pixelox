//! Test support: a tiny image format (`PXT` + u16 width + u16 height + RGBA8 pixels) and an
//! IO layer that decodes it, opens documents from memory, and records saved documents, so
//! placed-file commands run without `pixelox-io`.

use pixelox_core::ctx::{Globals, ImageInfo, ImageIo, NoProgress, OpenOptions};
use pixelox_core::envelope::Warning;
use pixelox_core::prelude::*;
use pixelox_core::Runner;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub fn encode(img: &Image) -> Vec<u8> {
    let mut b = b"PXT".to_vec();
    b.extend_from_slice(&(img.width as u16).to_le_bytes());
    b.extend_from_slice(&(img.height as u16).to_le_bytes());
    b.extend_from_slice(&img.to_rgba8());
    b
}

#[derive(Default)]
pub struct TestIo {
    /// Documents `open` returns, by path.
    pub docs: Mutex<BTreeMap<PathBuf, Document>>,
    /// Documents passed to `save_pxo`, in order.
    pub saved: Mutex<Vec<Document>>,
}

impl ImageIo for TestIo {
    fn open(&self, path: &Path, _: &OpenOptions) -> Result<(Document, Vec<Warning>)> {
        let docs = self.docs.lock().unwrap();
        docs.get(path)
            .cloned()
            .map(|d| (d, vec![]))
            .ok_or_else(|| Error::unsupported(format!("test io cannot open {}", path.display())))
    }

    fn decode(&self, bytes: &[u8], _: Option<&str>, _: &OpenOptions) -> Result<(Image, ImageInfo)> {
        if bytes.len() < 7 || &bytes[..3] != b"PXT" {
            return Err(Error::new("PX1103", "not a PXT test image"));
        }
        let w = u16::from_le_bytes([bytes[3], bytes[4]]) as u32;
        let h = u16::from_le_bytes([bytes[5], bytes[6]]) as u32;
        let img = Image::from_rgba8(w, h, &bytes[7..]);
        Ok((
            img,
            ImageInfo {
                format: "pxt".into(),
                width: w,
                height: h,
                depth: 8,
                channels: 4,
                has_alpha: true,
                ..Default::default()
            },
        ))
    }

    fn probe(&self, path: &Path) -> Result<ImageInfo> {
        Err(Error::unsupported(format!("test io cannot probe {}", path.display())))
    }

    fn encode(&self, _: &Image, _: Option<&Document>, path: &Path, _: &EncodeOptions) -> Result<Vec<Warning>> {
        std::fs::write(path, b"image")?;
        Ok(vec![])
    }

    fn save_pxo(&self, doc: &Document, path: &Path) -> Result<()> {
        std::fs::write(path, b"pxo")?;
        self.saved.lock().unwrap().push(doc.clone());
        Ok(())
    }

    fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec!["pxt"], vec![])
    }
}

/// Run a doc op on `doc` with `io`; returns the result and the warning codes.
pub fn run_io(
    spec: &OpSpec,
    pairs: &[(&str, &str)],
    doc: &mut Document,
    io: &dyn ImageIo,
) -> Result<(Value, Vec<String>)> {
    let a = pixelox_core::testing::args(spec, pairs)?;
    let mut ctx = OpCtx::new(spec.name, vec![], Globals::default(), io, &NoProgress);
    let v = match spec.runner {
        Runner::Doc(f) => f(&a, doc, &mut ctx)?,
        _ => panic!("{} is not a doc op", spec.name),
    };
    Ok((v, ctx.warnings.iter().map(|w| w.code.clone()).collect()))
}

/// Run a custom op with `inputs`, `globals`, and `io`.
pub fn run_custom(
    spec: &OpSpec,
    pairs: &[(&str, &str)],
    inputs: Vec<PathBuf>,
    globals: Globals,
    io: &dyn ImageIo,
) -> Result<(Value, Vec<String>)> {
    for i in &inputs {
        if !i.exists() {
            std::fs::write(i, b"pxo")?;
        }
    }
    let a = pixelox_core::testing::args(spec, pairs)?;
    let mut ctx = OpCtx::new(spec.name, inputs, globals, io, &NoProgress);
    let v = match spec.runner {
        Runner::Custom(f) => f(&a, &mut ctx)?,
        _ => panic!("{} is not a custom op", spec.name),
    };
    Ok((v, ctx.warnings.iter().map(|w| w.code.clone()).collect()))
}
