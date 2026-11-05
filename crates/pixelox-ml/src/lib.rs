//! AI-assisted ops (PRD §7.8, §8.6): `detect *`, `mask refine`, `fill content-aware`,
//! and the model registry (`models *`).
//!
//! This build is compiled without the `ml` feature (no ONNX Runtime, no model weights),
//! the same variant the PRD ships as the static musl build. Every command is registered
//! with its full flag surface so schemas, docs, pipelines, and MCP stay complete:
//! `--dry-run` returns the plan (which model, its size and license, the consent it would
//! need), and a real run fails with PX1510 (exit 12) naming the remedy. `models list`
//! and `models info` show the registry. The document commands are registered in export mode
//! here (they write nothing in this build), so the gate is reported before any `-o` check.

use pixelox_core::prelude::*;
use pixelox_core::ParamSpec;

/// The default registry (Apache-2.0/MIT weights only; PRD §7.8).
pub const MODELS: &[ModelEntry] = &[
    ModelEntry {
        name: "mobilesam",
        task: "segment",
        class: "SAM-class",
        license: "Apache-2.0",
        approx_mb: 40,
        used_by: "detect segment",
    },
    ModelEntry {
        name: "birefnet-lite",
        task: "subject",
        class: "DIS-class",
        license: "MIT",
        approx_mb: 170,
        used_by: "detect subject",
    },
    ModelEntry {
        name: "u2netp",
        task: "salient",
        class: "U²-Net",
        license: "Apache-2.0",
        approx_mb: 5,
        used_by: "detect salient --fast",
    },
    ModelEntry {
        name: "vitmatte-s",
        task: "matte",
        class: "ViTMatte",
        license: "Apache-2.0",
        approx_mb: 100,
        used_by: "mask refine, detect subject --matte",
    },
    ModelEntry {
        name: "lama",
        task: "inpaint",
        class: "LaMa (big-lama)",
        license: "Apache-2.0",
        approx_mb: 200,
        used_by: "fill content-aware",
    },
    ModelEntry {
        name: "yolox-s",
        task: "objects",
        class: "YOLOX-s",
        license: "Apache-2.0",
        approx_mb: 35,
        used_by: "detect objects",
    },
    ModelEntry {
        name: "yunet",
        task: "faces",
        class: "YuNet",
        license: "MIT",
        approx_mb: 1,
        used_by: "detect faces, retouch redeye --auto",
    },
    ModelEntry {
        name: "ppocrv4-det",
        task: "text",
        class: "PP-OCRv4 det",
        license: "Apache-2.0",
        approx_mb: 5,
        used_by: "detect text",
    },
];

#[derive(Clone, Copy, Debug, serde::Serialize)]
pub struct ModelEntry {
    pub name: &'static str,
    pub task: &'static str,
    pub class: &'static str,
    pub license: &'static str,
    pub approx_mb: u32,
    pub used_by: &'static str,
}

/// True when this build can run models.
pub const ML_AVAILABLE: bool = false;

fn model(name: &str) -> Option<&'static ModelEntry> {
    MODELS.iter().find(|m| m.name == name)
}

fn gated(what: &str, models: &[&str]) -> Error {
    Error::new("PX1510", format!("{what} needs the {} model, and this build of pixelox has no ML runtime", models.join(" + ")))
        .with_hint("use a build with the `ml` feature; meanwhile `select color --contiguous`, `select luma`, and `select path` make masks without models")
        .with_context("models", models)
}

/// Dry run: the plan; real run: PX1510.
fn plan_or_gate(ctx: &mut OpCtx, what: &str, models: &[&str]) -> Result<Value> {
    if ctx.dry_run() {
        for m in models {
            if let Some(e) = model(m) {
                ctx.plan_add(
                    "models_required",
                    json!({"name": e.name, "license": e.license, "installed": false, "download_bytes": e.approx_mb as u64 * 1024 * 1024,
                           "remedy": format!("pixelox models pull {} -y", e.name)}),
                );
            }
        }
        ctx.plan_add("consent_required", json!("model-download"));
        ctx.plan.insert("build_supports_ml".into(), json!(ML_AVAILABLE));
        ctx.warn("PXW0001", format!("{what} is planned only: this build has no ML runtime (PX1510 on a real run)"));
        return Ok(json!({"planned": true}));
    }
    Err(gated(what, models))
}

fn materialize_params() -> Vec<ParamSpec> {
    vec![
        param("save-mask", ParamKind::Str)
            .help("Store the result in the document's mask library under this name (comma list for several)"),
        param("select", ParamKind::Flag).help("Make the result the active selection"),
        param("mask-out", ParamKind::Path).help("Write the result as a grayscale mask image"),
        param("annotate", ParamKind::Path).help("Write a debug overlay image of the detections"),
    ]
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("detect subject", Category::Detect, "Separate the main subject (background removal)", detect_subject)
            .export_mode()
            .about("Primary subject / background separation (BiRefNet-lite) into a soft mask; --matte refines hair-grade edges (ViTMatte); --cutout writes the RGBA cutout directly. Needs the ml feature.")
            .milestone("v0.2")
            .param(param("matte", ParamKind::Flag).help("Refine edges with the matting model"))
            .param(param("cutout", ParamKind::Path).help("Write the RGBA cutout to this path"))
            .params(materialize_params())
            .example("pixelox detect subject shot.jpg --matte --cutout cutout.png --json", "Remove a background in one step")
            .example("pixelox detect subject shot.pxo --in-place --save-mask subject", "Store the subject mask in a document")
            .example("pixelox detect subject shot.pxo --save-mask subject --dry-run --json", "See which model it needs"),
        OpSpec::doc("detect objects", Category::Detect, "Detect objects (80 classes) and mask them", detect_objects)
            .export_mode()
            .about("YOLOX-s object detection; boxes are chained through the segmenter to produce masks. --prompt (open vocabulary) is a stretch item. Needs the ml feature.")
            .milestone("v0.2")
            .param(param("classes", ParamKind::List).help("Only these classes, e.g. dog,car"))
            .param(param("min-score", ParamKind::Float).default("0.5").range("0..1").help("Minimum detection score"))
            .param(param("prompt", ParamKind::Str).help("Open-vocabulary prompt, e.g. \"red car\" (stretch)"))
            .params(materialize_params())
            .example("pixelox detect objects walk.pxo --in-place --classes dog --min-score 0.5 --save-mask dog", "Mask the dog")
            .example("pixelox detect objects street.jpg --annotate boxes.png --json", "Draw the detections")
            .example("pixelox detect objects walk.pxo --classes dog --dry-run --json", "Plan the detection"),
        OpSpec::doc("detect salient", Category::Detect, "Salient-region map", detect_salient)
            .export_mode()
            .milestone("v0.2")
            .about("A saliency map (u2netp with --fast). Needs the ml feature.")
            .param(param("fast", ParamKind::Flag).help("Use the small u2netp model"))
            .params(materialize_params())
            .example("pixelox detect salient shot.jpg --fast --mask-out salient.png", "Write a saliency map")
            .example("pixelox detect salient shot.pxo --in-place --save-mask salient", "Store it as a mask")
            .example("pixelox detect salient shot.jpg --dry-run --json", "Plan it"),
        OpSpec::doc("detect faces", Category::Detect, "Detect faces and landmarks", detect_faces)
            .export_mode()
            .milestone("v0.2")
            .about("YuNet faces with optional 5-point landmarks. Needs the ml feature.")
            .param(param("landmarks", ParamKind::Flag).help("Include 5-point landmarks"))
            .params(materialize_params())
            .example("pixelox detect faces group.jpg --landmarks --json", "Find faces")
            .example("pixelox detect faces group.pxo --in-place --save-mask faces", "Mask every face")
            .example("pixelox detect faces group.jpg --annotate faces.png", "Draw the boxes"),
        OpSpec::doc("detect text", Category::Detect, "Find text regions (OCR boxes)", detect_text)
            .export_mode()
            .milestone("v0.2")
            .about("PP-OCRv4 text detection; --recognize (stretch) also reads the text. Needs the ml feature.")
            .param(param("recognize", ParamKind::Flag).help("Also recognize the text (stretch)"))
            .param(param("lang", ParamKind::Str).default("en").help("Recognition language pack"))
            .params(materialize_params())
            .example("pixelox detect text poster.jpg --json", "Boxes around text")
            .example("pixelox detect text poster.pxo --in-place --save-mask text", "Mask the text regions")
            .example("pixelox detect text poster.jpg --annotate text-boxes.png", "Draw them"),
        OpSpec::doc("detect segment", Category::Detect, "Promptable segmentation from points or a box", detect_segment)
            .export_mode()
            .milestone("v0.2")
            .about("MobileSAM masks from point prompts (--point X,Y, repeatable) or a box (--box X,Y,WxH); --prompt routes through open-vocabulary detection (stretch). Needs the ml feature.")
            .param(param("point", ParamKind::Point).repeatable().help("A point on the object (repeatable)"))
            .param(param("box", ParamKind::Rect).help("A box around the object"))
            .param(param("prompt", ParamKind::Str).help("Describe the object (stretch)"))
            .params(materialize_params())
            .example("pixelox detect segment street.pxo --in-place --point 820,440 --save-mask car", "Mask the object under a point")
            .example("pixelox detect segment street.jpg --box 600,300,500x320 --mask-out car.png", "Mask inside a box")
            .example("pixelox detect segment street.jpg --point 820,440 --dry-run --json", "Plan it"),
        OpSpec::custom("detect list-models", Category::Detect, "Which model serves which detect command", list_models_detect)
            .inputs(Inputs::NONE)
            .example("pixelox detect list-models --json", "The detect model table")
            .example("pixelox detect list-models", "As text")
            .example("pixelox models list --for detect --json", "The same through models list"),
        OpSpec::doc("mask refine", Category::Mask, "Refine mask edges for hair and fur (matting)", mask_refine)
            .export_mode()
            .milestone("v0.2")
            .about("Re-solves a band around the mask edge with ViTMatte (a trimap built from --band). Needs the ml feature.")
            .param(param("mask", ParamKind::Mask).required().help("Mask to refine (name or @sel)"))
            .param(param("model", ParamKind::Choice).choices(&["vitmatte"]).default("vitmatte").help("Matting model"))
            .param(param("band", ParamKind::Length).default("16").help("Width of the unknown band around the edge, px"))
            .example("pixelox mask refine shot.pxo --in-place --mask subject --model vitmatte --band 16", "Refine a subject mask")
            .example("pixelox mask refine shot.pxo --in-place --mask @sel --band 24", "Refine the selection")
            .example("pixelox mask refine shot.pxo --mask subject --dry-run --json", "Plan it"),
        OpSpec::doc("fill content-aware", Category::Retouch, "Fill a region from its surroundings (inpaint)", fill_content_aware)
            .export_mode()
            .milestone("v0.2")
            .alias("fill inpaint")
            .about("LaMa-class inpainting of the masked region. Needs the ml feature; `retouch heal` is the classical alternative for small regions.")
            .param(param("mask", ParamKind::Mask).required().help("Region to fill"))
            .param(param("model", ParamKind::Choice).choices(&["lama"]).default("lama").help("Inpainting model"))
            .param(param("expand", ParamKind::Length).default("0").help("Grow the region by this many px first"))
            .param(common::layer())
            .example("pixelox fill content-aware street.pxo --mask wires --dry-run --json", "Plan it: which model, is it installed?")
            .example("pixelox fill content-aware street.pxo --in-place --mask wires -y", "Remove power lines")
            .example("pixelox fill content-aware photo.jpg --mask spots.png -o clean.jpg --expand 4", "One-shot from a mask image"),
        OpSpec::custom("models list", Category::Models, "List models: installed, available, sizes, licenses", models_list)
            .inputs(Inputs::NONE)
            .param(param("for", ParamKind::Str).help("Only models used by this command group, e.g. detect"))
            .example("pixelox models list --json", "The registry")
            .example("pixelox models list --for detect", "Models behind detect")
            .example("pixelox models list --for fill", "Models behind fill"),
        OpSpec::custom("models info", Category::Models, "Show one model's task, license, and size", models_info)
            .inputs(Inputs::NONE)
            .param(param("name", ParamKind::Str).required().help("Model name"))
            .positional("name")
            .example("pixelox models info lama --json", "About the inpainting model")
            .example("pixelox models info birefnet-lite", "About the subject model")
            .example("pixelox models info yunet --json", "About the face model"),
        OpSpec::custom("models pull", Category::Models, "Download a model (consent-gated, SHA-256 verified)", models_gate)
            .inputs(Inputs::NONE)
            .param(param("name", ParamKind::Str).required().help("Model name"))
            .positional("name")
            .example("pixelox models pull lama -y", "Download the inpainting model")
            .example("pixelox models pull birefnet-lite -y --json", "Download the subject model")
            .example("pixelox models pull yunet --dry-run --json", "See the download size first"),
        OpSpec::custom("models rm", Category::Models, "Remove a downloaded model", models_gate)
            .inputs(Inputs::NONE)
            .param(param("name", ParamKind::Str).required().help("Model name"))
            .positional("name")
            .example("pixelox models rm lama", "Free the space")
            .example("pixelox models rm yolox-s --json", "Remove the detector")
            .example("pixelox models rm u2netp --dry-run --json", "See what would be removed"),
        OpSpec::custom("models verify", Category::Models, "Re-hash downloaded models", models_gate)
            .inputs(Inputs::NONE)
            .param(param("name", ParamKind::Str).help("Only this model"))
            .positional("name")
            .example("pixelox models verify", "Check every model")
            .example("pixelox models verify lama --json", "Check one")
            .example("pixelox models verify --json", "As JSON"),
        OpSpec::custom("models add", Category::Models, "Register a custom ONNX model", models_gate)
            .inputs(Inputs::NONE)
            .param(param("from", ParamKind::Path).required().help("The .onnx file"))
            .param(param("task", ParamKind::Choice).choices(&["segment", "subject", "salient", "matte", "inpaint", "objects", "faces", "text"]).required().help("Which task it serves"))
            .param(param("name", ParamKind::Str).required().help("Registry name"))
            .example("pixelox models add --from my.onnx --task segment --name custom-x", "Register a model")
            .example("pixelox models add --from matte.onnx --task matte --name my-matte --json", "As JSON")
            .example("pixelox models add --from seg.onnx --task subject --name seg2 --dry-run", "Plan it"),
    ]
}

fn detect_subject(a: &Args, _d: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let models: &[&str] = if a.flag("matte") { &["birefnet-lite", "vitmatte-s"] } else { &["birefnet-lite"] };
    plan_or_gate(ctx, "detect subject", models)
}
fn detect_objects(_a: &Args, _d: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    plan_or_gate(ctx, "detect objects", &["yolox-s", "mobilesam"])
}
fn detect_salient(_a: &Args, _d: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    plan_or_gate(ctx, "detect salient", &["u2netp"])
}
fn detect_faces(_a: &Args, _d: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    plan_or_gate(ctx, "detect faces", &["yunet"])
}
fn detect_text(_a: &Args, _d: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    plan_or_gate(ctx, "detect text", &["ppocrv4-det"])
}
fn detect_segment(a: &Args, _d: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    if a.all("point").is_empty() && a.raw("box").is_none() && a.raw("prompt").is_none() {
        return Err(Error::new("PX1003", "detect segment needs --point X,Y, --box X,Y,WxH, or --prompt"));
    }
    plan_or_gate(ctx, "detect segment", &["mobilesam"])
}
fn mask_refine(_a: &Args, _d: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    plan_or_gate(ctx, "mask refine", &["vitmatte-s"])
}
fn fill_content_aware(_a: &Args, _d: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    plan_or_gate(ctx, "fill content-aware", &["lama"])
}

fn registry_json(filter: Option<&str>) -> Vec<Value> {
    MODELS
        .iter()
        .filter(|m| filter.map(|f| m.used_by.contains(f)).unwrap_or(true))
        .map(|m| json!({"name": m.name, "task": m.task, "class": m.class, "license": m.license, "approx_bytes": m.approx_mb as u64 * 1024 * 1024, "used_by": m.used_by, "installed": false}))
        .collect()
}

fn list_models_detect(_a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    Ok(json!({"models": registry_json(Some("detect")), "build_supports_ml": ML_AVAILABLE}))
}

fn models_list(a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    Ok(
        json!({"models": registry_json(a.raw("for")), "models_dir": pixelox_core::paths::models_dir().display().to_string(), "build_supports_ml": ML_AVAILABLE}),
    )
}

fn models_info(a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    let n = a.str_or("name", "");
    let m = model(&n)
        .ok_or_else(|| Error::new("PX1501", format!("unknown model '{n}'")).with_hint("pixelox models list"))?;
    Ok(json!({"model": m, "installed": false, "build_supports_ml": ML_AVAILABLE}))
}

fn models_gate(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    if let Some(n) = a.raw("name") {
        if ctx.command != "models add" && model(n).is_none() {
            return Err(Error::new("PX1501", format!("unknown model '{n}'")).with_hint("pixelox models list"));
        }
    }
    if ctx.dry_run() {
        return Ok(
            json!({"planned": true, "build_supports_ml": ML_AVAILABLE, "models": registry_json(None).into_iter().filter(|m| a.raw("name").map(|n| m["name"] == n).unwrap_or(true)).collect::<Vec<_>>()}),
        );
    }
    Err(Error::new(
        "PX1510",
        format!("'{}' manages model weights, and this build of pixelox has no ML runtime", ctx.command),
    )
    .with_hint("use a build with the `ml` feature"))
}

/// Installs nothing: there is no model runtime in this build.
pub fn install() {}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing;

    #[test]
    fn gated_commands_fail_with_feature_gate() {
        let spec = testing::find(&ops(), "detect subject");
        let mut d = testing::doc_from(testing::gradient(8, 8));
        let e = testing::run(&spec, &[], &mut d).unwrap_err();
        assert_eq!(e.code, "PX1510");
        assert_eq!(e.exit_code, 12);
    }

    #[test]
    fn registry_is_permissive_only() {
        for m in MODELS {
            assert!(matches!(m.license, "MIT" | "Apache-2.0"), "{}", m.name);
        }
    }
}
