//! The model registry (PRD §7.8): permissive-license weights only, each with its download
//! source pinned by SHA-256. Entries without a source are planned for later releases; their
//! commands stay gated.

use serde::Serialize;
use serde_json::{json, Value};

/// How a segmentation model's input is prepared and its output read.
#[derive(Clone, Copy, Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Head {
    /// One logit map: apply a sigmoid (BiRefNet).
    Sigmoid,
    /// A probability map: stretch to 0..1 by its min and max (U²-Net).
    MinMax,
}

/// Where a model's weights come from and how to feed it.
#[derive(Clone, Copy, Debug, Serialize)]
pub struct Source {
    pub url: &'static str,
    pub sha256: &'static str,
    pub bytes: u64,
    pub homepage: &'static str,
    /// Square input size in px.
    pub input: u32,
    /// ImageNet-style normalization of RGB in 0..1.
    pub mean: [f32; 3],
    pub std: [f32; 3],
    pub head: Head,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub struct ModelEntry {
    pub name: &'static str,
    pub task: &'static str,
    pub class: &'static str,
    pub license: &'static str,
    pub approx_mb: u32,
    pub used_by: &'static str,
    /// None: not downloadable in this release.
    pub source: Option<Source>,
}

const IMAGENET_MEAN: [f32; 3] = [0.485, 0.456, 0.406];
const IMAGENET_STD: [f32; 3] = [0.229, 0.224, 0.225];

/// The default registry (Apache-2.0/MIT weights only).
pub const MODELS: &[ModelEntry] = &[
    ModelEntry {
        name: "birefnet-lite",
        task: "subject",
        class: "BiRefNet (Swin-T)",
        license: "MIT",
        approx_mb: 214,
        used_by: "detect subject",
        source: Some(Source {
            url: "https://github.com/ZhengPeng7/BiRefNet/releases/download/v1/BiRefNet-general-bb_swin_v1_tiny-epoch_232.onnx",
            sha256: "5600024376f572a557870a5eb0afb1e5961636bef4e1e22132025467d0f03333",
            bytes: 224_005_088,
            homepage: "https://github.com/ZhengPeng7/BiRefNet",
            input: 1024,
            mean: IMAGENET_MEAN,
            std: IMAGENET_STD,
            head: Head::Sigmoid,
        }),
    },
    ModelEntry {
        name: "u2netp",
        task: "salient",
        class: "U²-Net (small)",
        license: "Apache-2.0",
        approx_mb: 5,
        used_by: "detect salient, detect subject --fast",
        source: Some(Source {
            url: "https://github.com/danielgatis/rembg/releases/download/v0.0.0/u2netp.onnx",
            sha256: "309c8469258dda742793dce0ebea8e6dd393174f89934733ecc8b14c76f4ddd8",
            bytes: 4_574_861,
            homepage: "https://github.com/xuebinqin/U-2-Net",
            input: 320,
            mean: IMAGENET_MEAN,
            std: IMAGENET_STD,
            head: Head::MinMax,
        }),
    },
    ModelEntry {
        name: "mobilesam",
        task: "segment",
        class: "SAM-class",
        license: "Apache-2.0",
        approx_mb: 40,
        used_by: "detect segment",
        source: None,
    },
    ModelEntry {
        name: "vitmatte-s",
        task: "matte",
        class: "ViTMatte",
        license: "Apache-2.0",
        approx_mb: 100,
        used_by: "mask refine, detect subject --matte",
        source: None,
    },
    ModelEntry {
        name: "lama",
        task: "inpaint",
        class: "LaMa (big-lama)",
        license: "Apache-2.0",
        approx_mb: 200,
        used_by: "fill content-aware",
        source: None,
    },
    ModelEntry {
        name: "yolox-s",
        task: "objects",
        class: "YOLOX-s",
        license: "Apache-2.0",
        approx_mb: 35,
        used_by: "detect objects",
        source: None,
    },
    ModelEntry {
        name: "yunet",
        task: "faces",
        class: "YuNet",
        license: "MIT",
        approx_mb: 1,
        used_by: "detect faces, retouch redeye --auto",
        source: None,
    },
    ModelEntry {
        name: "ppocrv4-det",
        task: "text",
        class: "PP-OCRv4 det",
        license: "Apache-2.0",
        approx_mb: 5,
        used_by: "detect text",
        source: None,
    },
];

pub fn model(name: &str) -> Option<&'static ModelEntry> {
    MODELS.iter().find(|m| m.name == name)
}

impl ModelEntry {
    /// Download size in bytes (exact when the source is pinned).
    pub fn download_bytes(&self) -> u64 {
        self.source.map(|s| s.bytes).unwrap_or(self.approx_mb as u64 * 1024 * 1024)
    }

    pub fn available(&self) -> bool {
        self.source.is_some()
    }

    /// The registry row agents see (`models list`, dry-run plans).
    pub fn to_json(&self, installed: bool) -> Value {
        json!({
            "name": self.name, "task": self.task, "class": self.class, "license": self.license,
            "bytes": self.download_bytes(), "used_by": self.used_by, "available": self.available(),
            "installed": installed,
            "sha256": self.source.map(|s| s.sha256), "url": self.source.map(|s| s.url),
            "homepage": self.source.map(|s| s.homepage),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_is_permissive_and_pinned() {
        for m in MODELS {
            assert!(matches!(m.license, "MIT" | "Apache-2.0"), "{}", m.name);
            if let Some(s) = m.source {
                assert_eq!(s.sha256.len(), 64, "{}", m.name);
                assert!(s.sha256.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
                assert!(s.url.starts_with("https://"));
                assert!(s.bytes > 0 && s.input >= 64);
            }
        }
        assert!(model("birefnet-lite").unwrap().available());
        assert!(!model("lama").unwrap().available());
    }
}
