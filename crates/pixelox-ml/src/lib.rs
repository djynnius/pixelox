//! AI-assisted ops (PRD §7.8, §8.6): `detect *`, `fill content-aware`, and the model
//! registry (`models *`). (`mask refine` lives in pixelox-select: closed-form matting needs
//! no model; its `--model vitmatte` choice exits 12 there.)
//!
//! With the `ml` feature (on supported targets, see build.rs) `detect subject` and
//! `detect salient` run local ONNX models through ONNX Runtime: BiRefNet-lite and U²-Netp,
//! downloaded on first use only with consent (`-y`) and verified by SHA-256. The other
//! detect commands and `fill content-aware` are registered with their full
//! flag surface but their models are not in this release: `--dry-run` returns the plan and a
//! real run exits 12 (PX1520). Builds without the feature exit 12 with PX1510 for every
//! model-backed command.

mod registry;
pub mod segment;

#[cfg(pixelox_ml)]
mod download;
#[cfg(pixelox_ml)]
mod runtime;
#[cfg(pixelox_ml)]
mod store;

pub use registry::{model, Head, ModelEntry, Source, MODELS};

use pixelox_core::ctx::EncodeOptions;
use pixelox_core::doc::RecipeStep;
use pixelox_core::prelude::*;
use pixelox_core::ParamSpec;
use rayon::prelude::*;
use std::path::PathBuf;

/// True when this build can run models.
pub const ML_AVAILABLE: bool = cfg!(pixelox_ml);

fn is_installed(name: &str) -> bool {
    #[cfg(pixelox_ml)]
    {
        store::installed(name).is_some()
    }
    #[cfg(not(pixelox_ml))]
    {
        let _ = name;
        false
    }
}

fn no_runtime(what: &str, models: &[&str]) -> Error {
    Error::new("PX1510", format!("{what} needs the {} model, and this build of pixelox has no ML runtime", models.join(" + ")))
        .with_hint("use a build with the `ml` feature; meanwhile `select sky`, `select color --contiguous`, `select luma`, and `select path` make masks without models")
        .with_context("models", models)
}

fn not_in_release(what: &str, models: &[&str]) -> Error {
    Error::new("PX1520", format!("{what} needs the {} model, which is not in this release", models.join(" + ")))
        .with_hint("`detect subject` and `detect salient` run locally today; `select sky`, `select color --contiguous`, `select luma`, and `select path` make masks without models")
        .with_context("models", models)
}

fn plan_models(ctx: &mut OpCtx, models: &[&str]) -> bool {
    let mut all_installed = true;
    for m in models {
        if let Some(e) = model(m) {
            let installed = is_installed(e.name);
            all_installed &= installed;
            ctx.plan_add(
                "models_required",
                json!({"name": e.name, "license": e.license, "installed": installed, "available": e.available(),
                       "download_bytes": e.download_bytes(), "remedy": format!("pixelox models pull {} -y", e.name)}),
            );
        }
    }
    if !all_installed {
        ctx.plan_add("consent_required", json!("model-download"));
    }
    ctx.plan.insert("build_supports_ml".into(), json!(ML_AVAILABLE));
    all_installed
}

/// Commands whose models are not in this release: the plan on a dry run, exit 12 otherwise.
fn plan_or_gate(ctx: &mut OpCtx, what: &str, models: &[&str]) -> Result<Value> {
    if ctx.dry_run() {
        plan_models(ctx, models);
        ctx.warn(
            "PXW0001",
            format!("{what} is planned only: its model is not in this release (exit 12 on a real run)"),
        );
        return Ok(json!({"planned": true}));
    }
    Err(if ML_AVAILABLE { not_in_release(what, models) } else { no_runtime(what, models) })
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

/// Params of the commands that produce one soft mask (`detect subject`, `detect salient`).
fn mask_params() -> Vec<ParamSpec> {
    vec![
        param("layer", ParamKind::Layer)
            .help("Detect on this layer (placed on the canvas) instead of the composite; also the layer --layer-mask masks"),
        param("refine", ParamKind::Length)
            .auto()
            .default("auto")
            .help("Edge refinement radius in px (guided upsampling snaps the mask's edge to the image); auto = two pixels of the model's map, 0 = plain bilinear"),
        param("select", ParamKind::Flag).help("Make the mask the active selection (the default when no other result is asked for)"),
        param("save-mask", ParamKind::Str).help("Store the mask in the document's mask library under this name"),
        param("layer-mask", ParamKind::Flag)
            .help("Attach the mask to --layer (or the only layer) as its layer mask; with -o cut.png this is a one-step cutout"),
        param("cutout", ParamKind::Path)
            .help("Also write the composite with the mask as alpha (an RGBA PNG, WebP, or TIFF); works without -o"),
        param("mask-out", ParamKind::Path).help("Also write the mask as a grayscale image; works without -o"),
    ]
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("detect subject", Category::Detect, "Separate the main subject (background removal)", detect_subject)
            .target_optional()
            .about("Primary subject / background separation into a soft mask with a local model: BiRefNet-lite (MIT, 224 MB, 1024 px) by default, U²-Netp (Apache-2.0, 5 MB, 320 px) with --fast. The model downloads on first use only with -y (or `pixelox models pull birefnet-lite -y`), verified by SHA-256; inference runs on the CPU with a fixed thread count, so results are reproducible. The mask's edge is refined with a guided filter (--refine). Results go to the selection (the default), --save-mask, --layer-mask, and side files (--cutout, --mask-out; these work without -o, and their sidecars replay through `export --apply-mask @sel` and `mask export`). --matte (ViTMatte) is not in this release. Builds without the ml feature exit 12 (PX1510).")
            .param(param("fast", ParamKind::Flag).help("Use the small U²-Netp model (5 MB, faster, coarser edges)"))
            .param(param("matte", ParamKind::Flag).help("Refine hair-grade edges with ViTMatte (not in this release: exit 12)"))
            .params(mask_params())
            .example("pixelox detect subject shot.jpg -o cutout.png --layer-mask -y", "Remove a background in one step")
            .example("pixelox detect subject shot.jpg --cutout cutout.png --mask-out subject.png --json", "Write a cutout and its mask without a document")
            .example("pixelox detect subject shot.pxo --in-place --save-mask subject", "Store the subject mask in a document")
            .example("pixelox detect subject shot.pxo --in-place --fast --select", "Quick selection with the small model")
            .example("pixelox detect subject shot.pxo --save-mask subject --dry-run --json", "See which model it needs and whether it is installed"),
        OpSpec::doc("detect objects", Category::Detect, "Detect objects (80 classes) and mask them", detect_objects)
            .export_mode()
            .about("YOLOX-s object detection; boxes are chained through the segmenter to produce masks. --prompt (open vocabulary) is a stretch item. Its models are not in this release.")
            .milestone("v0.2")
            .param(param("classes", ParamKind::List).help("Only these classes, e.g. dog,car"))
            .param(param("min-score", ParamKind::Float).default("0.5").range("0..1").help("Minimum detection score"))
            .param(param("prompt", ParamKind::Str).help("Open-vocabulary prompt, e.g. \"red car\" (stretch)"))
            .params(materialize_params())
            .example("pixelox detect objects walk.pxo --in-place --classes dog --min-score 0.5 --save-mask dog", "Mask the dog")
            .example("pixelox detect objects street.jpg --annotate boxes.png --json", "Draw the detections")
            .example("pixelox detect objects walk.pxo --classes dog --dry-run --json", "Plan the detection"),
        OpSpec::doc("detect salient", Category::Detect, "Salient-region map", detect_salient)
            .target_optional()
            .about("A saliency map (what draws the eye) from the U²-Netp model (Apache-2.0, 5 MB), downloaded on first use with -y. Results go where `detect subject` sends them: the selection (default), --save-mask, --layer-mask, --cutout, --mask-out. --fast is accepted for compatibility (U²-Netp is the only salient model).")
            .param(param("fast", ParamKind::Flag).help("Use the small U²-Netp model (the default and only salient model)"))
            .params(mask_params())
            .example("pixelox detect salient shot.jpg --mask-out salient.png -y", "Write a saliency map")
            .example("pixelox detect salient shot.pxo --in-place --save-mask salient", "Store it as a mask")
            .example("pixelox detect salient shot.jpg --dry-run --json", "Plan it"),
        OpSpec::doc("detect faces", Category::Detect, "Detect faces and landmarks", detect_faces)
            .export_mode()
            .milestone("v0.2")
            .about("YuNet faces with optional 5-point landmarks. Its model is not in this release.")
            .param(param("landmarks", ParamKind::Flag).help("Include 5-point landmarks"))
            .params(materialize_params())
            .example("pixelox detect faces group.jpg --landmarks --json", "Find faces")
            .example("pixelox detect faces group.pxo --in-place --save-mask faces", "Mask every face")
            .example("pixelox detect faces group.jpg --annotate faces.png", "Draw the boxes"),
        OpSpec::doc("detect text", Category::Detect, "Find text regions (OCR boxes)", detect_text)
            .export_mode()
            .milestone("v0.2")
            .about("PP-OCRv4 text detection; --recognize (stretch) also reads the text. Its model is not in this release.")
            .param(param("recognize", ParamKind::Flag).help("Also recognize the text (stretch)"))
            .param(param("lang", ParamKind::Str).default("en").help("Recognition language pack"))
            .params(materialize_params())
            .example("pixelox detect text poster.jpg --json", "Boxes around text")
            .example("pixelox detect text poster.pxo --in-place --save-mask text", "Mask the text regions")
            .example("pixelox detect text poster.jpg --annotate text-boxes.png", "Draw them"),
        OpSpec::doc("detect segment", Category::Detect, "Promptable segmentation from points or a box", detect_segment)
            .export_mode()
            .milestone("v0.2")
            .about("MobileSAM masks from point prompts (--point X,Y, repeatable) or a box (--box X,Y,WxH); --prompt routes through open-vocabulary detection (stretch). Its model is not in this release.")
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
        OpSpec::doc("fill content-aware", Category::Retouch, "Fill a region from its surroundings (inpaint)", fill_content_aware)
            .export_mode()
            .milestone("v0.2")
            .alias("fill inpaint")
            .about("LaMa-class inpainting of the masked region. Its model is not in this release; `retouch heal` is the classical alternative for small regions.")
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
        OpSpec::custom("models pull", Category::Models, "Download a model (consent-gated, SHA-256 verified)", models_pull)
            .inputs(Inputs::NONE)
            .about("Downloads a model into the models directory (`pixelox models list` prints it) and verifies its SHA-256 before using it. Needs -y: the download is the user's decision. Interrupted downloads resume; PIXELOX_OFFLINE=1 forbids network access.")
            .param(param("name", ParamKind::Str).required().help("Model name"))
            .positional("name")
            .example("pixelox models pull birefnet-lite -y", "Download the subject model")
            .example("pixelox models pull u2netp -y --json", "Download the small salient model")
            .example("pixelox models pull birefnet-lite --dry-run --json", "See the download size first"),
        OpSpec::custom("models rm", Category::Models, "Remove a downloaded model", models_rm)
            .inputs(Inputs::NONE)
            .param(param("name", ParamKind::Str).required().help("Model name"))
            .positional("name")
            .example("pixelox models rm birefnet-lite", "Free the space")
            .example("pixelox models rm u2netp --json", "Remove the small model")
            .example("pixelox models rm u2netp --dry-run --json", "See what would be removed"),
        OpSpec::custom("models verify", Category::Models, "Re-hash downloaded models", models_verify)
            .inputs(Inputs::NONE)
            .about("Re-hashes downloaded models against the registry's SHA-256 (PX1503 on a mismatch: remove and pull again).")
            .param(param("name", ParamKind::Str).help("Only this model"))
            .positional("name")
            .example("pixelox models verify", "Check every model")
            .example("pixelox models verify birefnet-lite --json", "Check one")
            .example("pixelox models verify --json", "As JSON"),
        OpSpec::custom("models add", Category::Models, "Register a custom ONNX model", models_add)
            .inputs(Inputs::NONE)
            .milestone("v0.3")
            .about("Registers your own ONNX model for a task. Not in this release (exit 12): the detect commands run the registry's models only.")
            .param(param("from", ParamKind::Path).required().help("The .onnx file"))
            .param(param("task", ParamKind::Choice).choices(&["segment", "subject", "salient", "matte", "inpaint", "objects", "faces", "text"]).required().help("Which task it serves"))
            .param(param("name", ParamKind::Str).required().help("Registry name"))
            .example("pixelox models add --from my.onnx --task segment --name custom-x", "Register a model")
            .example("pixelox models add --from matte.onnx --task matte --name my-matte --json", "As JSON")
            .example("pixelox models add --from seg.onnx --task subject --name seg2 --dry-run", "Plan it"),
    ]
}

// ---------------------------------------------------------------------------------------
// detect subject / detect salient

fn detect_subject(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    if a.flag("matte") {
        return Err(not_in_release("detect subject --matte", &["vitmatte-s"])
            .with_hint("drop --matte: detect subject refines edges with a guided filter (--refine)"));
    }
    let entry = model(if a.flag("fast") { "u2netp" } else { "birefnet-lite" }).expect("registry entry");
    detect_mask(a, doc, ctx, entry)
}

fn detect_salient(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    detect_mask(a, doc, ctx, model("u2netp").expect("registry entry"))
}

/// Where a detection's mask goes.
struct Outputs {
    select: bool,
    save: Option<String>,
    layer_mask: bool,
    cutout: Option<PathBuf>,
    mask_out: Option<PathBuf>,
}

impl Outputs {
    fn from(a: &Args) -> Outputs {
        let mut o = Outputs {
            select: a.flag("select"),
            save: a.str("save-mask").filter(|s| !s.is_empty()),
            layer_mask: a.flag("layer-mask"),
            cutout: a.str("cutout").map(PathBuf::from),
            mask_out: a.str("mask-out").map(PathBuf::from),
        };
        if !o.select && o.save.is_none() && !o.layer_mask && o.cutout.is_none() && o.mask_out.is_none() {
            o.select = true;
        }
        o
    }

    fn files(&self) -> bool {
        self.cutout.is_some() || self.mask_out.is_some()
    }
}

/// The canvas-sized image a detection looks at: the composite, or one layer.
fn sample(doc: &Document, layer: Option<&str>) -> Result<Image> {
    match layer {
        None => compose::composite(doc),
        Some(r) => {
            let p = doc.resolve(r)?;
            Ok(match compose::render_layer(doc, doc.layer(&p))? {
                Some(rd) => rd.to_canvas(doc.width, doc.height),
                None => Image::new(doc.width, doc.height),
            })
        }
    }
}

fn mask_stats(m: &Mask) -> Value {
    let n = (m.width as usize * m.height as usize).max(1);
    let rows: Vec<(f64, Option<(u32, u32)>)> = m
        .data
        .par_chunks(m.width.max(1) as usize)
        .map(|row| {
            let sum: f64 = row.iter().map(|v| *v as f64).sum();
            let first = row.iter().position(|v| *v >= 0.5);
            let last = row.iter().rposition(|v| *v >= 0.5);
            (sum, first.zip(last).map(|(a, b)| (a as u32, b as u32)))
        })
        .collect();
    let total: f64 = rows.iter().map(|r| r.0).sum();
    let mut bb: Option<(u32, u32, u32, u32)> = None;
    for (y, (_, span)) in rows.iter().enumerate() {
        if let Some((x0, x1)) = span {
            let y = y as u32;
            bb = Some(match bb {
                None => (*x0, y, *x1, y),
                Some((a, b, c, _)) => (a.min(*x0), b, c.max(*x1), y),
            });
        }
    }
    json!({
        "coverage": ((total / n as f64) * 100_000.0).round() / 1000.0,
        "bounds": bb.map(|(x0, y0, x1, y1)| json!([x0, y0, x1 - x0 + 1, y1 - y0 + 1])),
    })
}

fn mask_image(m: &Mask) -> Image {
    let mut img = Image::new(m.width, m.height);
    img.data.par_chunks_mut(4).zip(m.data.par_iter()).for_each(|(p, &v)| {
        let v = v.clamp(0.0, 1.0);
        p.copy_from_slice(&[v, v, v, 1.0]);
    });
    img
}

/// The recipe a side file's sidecar records: the document's history, this step with its
/// result kept as the selection, then the command that renders the file from it.
fn side_recipe(doc: &Document, a: &Args, ctx: &OpCtx, last: RecipeStep) -> Vec<RecipeStep> {
    let mut params = a.to_json();
    for k in ["cutout", "mask-out", "mask_out", "save-mask", "save_mask", "layer-mask", "layer_mask", "select"] {
        params.remove(k);
    }
    params.insert("select".into(), json!(true));
    for (k, v) in &ctx.resolved {
        params.insert(k.clone(), v.clone());
    }
    let step = RecipeStep {
        op: ctx.id.clone(),
        params,
        inputs: ctx.step_inputs.clone(),
        model: ctx.models.first().cloned(),
        fonts: vec![],
    };
    let mut r = doc.recipe.clone();
    r.push(step);
    r.push(last);
    r
}

fn final_step(op: &str, params: Value) -> RecipeStep {
    RecipeStep {
        op: op.into(),
        params: params.as_object().cloned().unwrap_or_default(),
        inputs: vec![],
        model: None,
        fonts: vec![],
    }
}

fn encode_opts(ctx: &OpCtx, path: &std::path::Path) -> EncodeOptions {
    let mut o = EncodeOptions::for_path(path);
    if let Some(f) = &ctx.globals.format {
        o.format = f.clone();
    }
    o
}

#[cfg(pixelox_ml)]
fn run_model(ctx: &mut OpCtx, entry: &'static ModelEntry, img: &Image, refine: f64) -> Result<Mask> {
    let consent = ctx.globals.yes;
    let (path, rec) = store::ensure(ctx, entry, consent)?;
    ctx.models.push(json!({"name": entry.name, "sha256": rec.sha256, "bytes": rec.bytes, "license": entry.license}));
    let src = entry.source.expect("available model");
    ctx.set_progress(0.2);
    let mut infer = |s: &Source, t: Vec<f32>| runtime::run_map(&path, s, t);
    let m = segment::segment(img, &src, &mut infer, refine)?;
    ctx.set_progress(0.9);
    Ok(m)
}

#[cfg(not(pixelox_ml))]
fn run_model(ctx: &mut OpCtx, entry: &'static ModelEntry, _img: &Image, _refine: f64) -> Result<Mask> {
    Err(no_runtime(&ctx.command, &[entry.name]))
}

fn detect_mask(a: &Args, doc: &mut Document, ctx: &mut OpCtx, entry: &'static ModelEntry) -> Result<Value> {
    let out = Outputs::from(a);
    let has_target = ctx.globals.output.is_some() || ctx.globals.in_place;
    if !has_target && !out.files() && !ctx.dry_run() && !ctx.replaying {
        return Err(Error::new(
            "PX1004",
            format!("'{}' keeps its mask in the document; say where to write it", ctx.command),
        )
        .with_hint("add -o doc.pxo or --in-place, or ask for files with --cutout cut.png / --mask-out mask.png"));
    }
    if let Some(name) = &out.save {
        if name.starts_with('@') {
            return Err(Error::new("PX1010", format!("--save-mask '{name}': mask names cannot start with @")));
        }
    }
    if let Some(p) = &out.cutout {
        let fmt = EncodeOptions::for_path(p).format;
        if !matches!(fmt.as_str(), "png" | "webp" | "tiff") {
            return Err(Error::new(
                "PX1106",
                format!("--cutout {}: a cutout needs a format with transparency", p.display()),
            )
            .with_hint("write PNG, WebP, or TIFF, e.g. --cutout cutout.png"));
        }
    }
    let target = if out.layer_mask {
        let p = doc.resolve_or_only(a.raw("layer"))?;
        doc.layer(&p).check_unlocked()?;
        Some(p)
    } else {
        None
    };
    let source = sample(doc, a.raw("layer"))?;
    let refine = match a.length("refine")? {
        None => segment::auto_refine(
            doc.width,
            doc.height,
            &entry.source.ok_or_else(|| not_in_release(&ctx.command, &[entry.name]))?,
        ),
        Some(l) => {
            let v = l.to_px(doc.width.min(doc.height) as f64, doc.dpi());
            if !(v >= 0.0 && v.is_finite()) {
                return Err(Error::new("PX1013", "--refine must not be negative"));
            }
            v
        }
    };
    if ctx.dry_run() {
        let ready = plan_models(ctx, &[entry.name]);
        if !ML_AVAILABLE {
            ctx.warn(
                "PXW0001",
                format!("{} is planned only: this build has no ML runtime (PX1510 on a real run)", ctx.command),
            );
        }
        return Ok(json!({"planned": true, "model": entry.name, "installed": ready, "refine": refine}));
    }
    let mut mask = run_model(ctx, entry, &source, refine)?;
    if a.is_auto("refine") {
        // full precision: a replay must rebuild exactly the same radius
        ctx.resolve_param("refine", refine.to_string());
    }
    segment::quantize(&mut mask, doc.depth);
    let stats = mask_stats(&mask);
    let mut v = json!({"model": entry.name, "refine": refine, "mask": stats});
    // side files first, from the document as it was before this step
    if !ctx.replaying {
        if let Some(p) = &out.cutout {
            let mut img = compose::composite(doc)?;
            img.data.par_chunks_mut(4).zip(mask.data.par_iter()).for_each(|(px, m)| px[3] *= m);
            let recipe = side_recipe(doc, a, ctx, final_step("export", json!({"apply_mask": "@sel"})));
            ctx.write_image(p, &img, Some(doc), &encode_opts(ctx, p), Some("cutout"), Some(recipe))?;
            v["cutout"] = json!(p.display().to_string());
        }
        if let Some(p) = &out.mask_out {
            let recipe = side_recipe(doc, a, ctx, final_step("mask.export", json!({"mask": "@sel"})));
            ctx.write_image(p, &mask_image(&mask), Some(doc), &encode_opts(ctx, p), Some("mask"), Some(recipe))?;
            v["mask_out"] = json!(p.display().to_string());
        }
    }
    if let Some(name) = &out.save {
        doc.put_mask(name, mask.clone(), true)?;
        v["saved_mask"] = json!(name);
    }
    if let Some(p) = target {
        let l = doc.layer_mut(&p);
        let replaced = l.mask.is_some();
        l.mask = Some(LayerMask::new(mask.clone()));
        l.vector_mask = None;
        v["layer_mask"] = json!({"layer": l.name, "replaced": replaced});
    }
    if out.select {
        doc.selection = Some(mask);
        v["selection"] = json!(true);
    }
    Ok(v)
}

// ---------------------------------------------------------------------------------------
// gated detect commands

fn detect_objects(_a: &Args, _d: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    plan_or_gate(ctx, "detect objects", &["yolox-s", "mobilesam"])
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
fn fill_content_aware(_a: &Args, _d: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    plan_or_gate(ctx, "fill content-aware", &["lama"])
}

// ---------------------------------------------------------------------------------------
// models *

fn registry_json(filter: Option<&str>) -> Vec<Value> {
    MODELS
        .iter()
        .filter(|m| filter.map(|f| m.used_by.contains(f)).unwrap_or(true))
        .map(|m| m.to_json(is_installed(m.name)))
        .collect()
}

fn lookup(name: &str) -> Result<&'static ModelEntry> {
    model(name).ok_or_else(|| {
        Error::new("PX1501", format!("unknown model '{name}'"))
            .with_hint("pixelox models list")
            .with_context("model", name)
    })
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
    let m = lookup(&a.str_or("name", ""))?;
    Ok(json!({"model": m.to_json(is_installed(m.name)), "build_supports_ml": ML_AVAILABLE}))
}

fn models_pull(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let m = lookup(&a.str_or("name", ""))?;
    if !m.available() {
        return Err(not_in_release("models pull", &[m.name]));
    }
    if ctx.dry_run() {
        let installed = plan_models(ctx, &[m.name]);
        return Ok(json!({"planned": true, "model": m.name, "installed": installed, "bytes": m.download_bytes()}));
    }
    if !ML_AVAILABLE {
        return Err(no_runtime("models pull", &[m.name]));
    }
    pull(ctx, m)
}

#[cfg(pixelox_ml)]
fn pull(ctx: &mut OpCtx, m: &'static ModelEntry) -> Result<Value> {
    let had = store::installed(m.name).is_some();
    let consent = ctx.globals.yes;
    let (path, rec) = store::ensure(ctx, m, consent)?;
    Ok(json!({"model": m.name, "path": path.display().to_string(), "sha256": rec.sha256, "bytes": rec.bytes,
              "downloaded": !had, "license": m.license}))
}

#[cfg(not(pixelox_ml))]
fn pull(_ctx: &mut OpCtx, m: &'static ModelEntry) -> Result<Value> {
    Err(no_runtime("models pull", &[m.name]))
}

fn models_rm(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let m = lookup(&a.str_or("name", ""))?;
    let installed = is_installed(m.name);
    if ctx.dry_run() {
        return Ok(json!({"planned": true, "model": m.name, "installed": installed}));
    }
    #[cfg(pixelox_ml)]
    {
        let rec = store::forget(m.name)?;
        Ok(json!({"model": m.name, "removed": rec.is_some(), "freed_bytes": rec.map(|r| r.bytes).unwrap_or(0)}))
    }
    #[cfg(not(pixelox_ml))]
    {
        let _ = installed;
        Err(no_runtime("models rm", &[m.name]))
    }
}

fn models_verify(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let names: Vec<&'static ModelEntry> = match a.raw("name") {
        Some(n) => vec![lookup(n)?],
        None => MODELS.iter().filter(|m| is_installed(m.name)).collect(),
    };
    if ctx.dry_run() {
        return Ok(json!({"planned": true, "models": names.iter().map(|m| m.name).collect::<Vec<_>>()}));
    }
    #[cfg(pixelox_ml)]
    {
        let mut rows = Vec::new();
        let mut bad = Vec::new();
        for m in names {
            let Some(rec) = store::installed(m.name) else {
                rows.push(json!({"model": m.name, "installed": false}));
                continue;
            };
            let got = download::sha256_file(&store::model_path(m.name))?;
            let want = m.source.map(|s| s.sha256).unwrap_or(rec.sha256.as_str()).to_string();
            let ok = got == want;
            if !ok {
                bad.push(m.name);
            }
            rows.push(json!({"model": m.name, "installed": true, "ok": ok, "sha256": got, "expected": want}));
        }
        if !bad.is_empty() {
            return Err(Error::new("PX1503", format!("SHA-256 mismatch: {}", bad.join(", ")))
                .with_hint(format!("pixelox models rm {0} && pixelox models pull {0} -y", bad[0]))
                .with_context("models", rows));
        }
        Ok(json!({"models": rows}))
    }
    #[cfg(not(pixelox_ml))]
    {
        Err(no_runtime("models verify", &names.iter().map(|m| m.name).collect::<Vec<_>>()))
    }
}

fn models_add(_a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    if ctx.dry_run() {
        return Ok(json!({"planned": true, "build_supports_ml": ML_AVAILABLE}));
    }
    Err(Error::new("PX1520", "custom models are not in this release; the detect commands run the registry's models")
        .with_hint("pixelox models list"))
}

/// Names of the downloaded models (for `doctor`).
pub fn installed_models() -> Vec<&'static str> {
    MODELS.iter().filter(|m| is_installed(m.name)).map(|m| m.name).collect()
}

/// No renderer hooks: models run inside their commands.
pub fn install() {}

#[cfg(test)]
mod tests {
    use super::*;
    use pixelox_core::testing;

    #[test]
    fn unreleased_models_are_gated() {
        let spec = testing::find(&ops(), "detect faces");
        let mut d = testing::doc_from(testing::gradient(8, 8));
        let e = testing::run(&spec, &[], &mut d).unwrap_err();
        assert!(e.code == "PX1520" || e.code == "PX1510", "{}", e.code);
        assert_eq!(e.exit_code, 12);
        let spec = testing::find(&ops(), "detect subject");
        let e = testing::run(&spec, &[("matte", "true")], &mut d).unwrap_err();
        assert_eq!(e.exit_code, 12);
    }

    #[test]
    fn detect_needs_somewhere_to_write() {
        let spec = testing::find(&ops(), "detect subject");
        let mut d = testing::doc_from(testing::gradient(8, 8));
        assert_eq!(testing::run(&spec, &[], &mut d).unwrap_err().code, "PX1004");
        assert_eq!(testing::run(&spec, &[("cutout", "cut.jpg")], &mut d).unwrap_err().code, "PX1106");
        assert_eq!(
            testing::run(&spec, &[("save-mask", "@x"), ("cutout", "c.png")], &mut d).unwrap_err().code,
            "PX1010"
        );
    }

    #[test]
    fn mask_stats_report_coverage_and_bounds() {
        let m = Mask::from_fn(10, 4, |x, y| if (2..5).contains(&x) && y >= 1 { 1.0 } else { 0.0 });
        let s = mask_stats(&m);
        assert_eq!(s["coverage"], 22.5);
        assert_eq!(s["bounds"], json!([2, 1, 3, 3]));
        assert_eq!(mask_stats(&Mask::new(3, 3))["bounds"], Value::Null);
    }

    #[test]
    fn side_recipes_end_in_a_replayable_export() {
        let spec = testing::find(&ops(), "detect subject");
        let a = pixelox_core::op::Args::build(
            &spec,
            vec![("cutout".into(), "c.png".into()), ("save-mask".into(), "s".into()), ("fast".into(), "true".into())],
        )
        .unwrap();
        let d = testing::doc_from(testing::gradient(8, 8));
        let ctx = OpCtx::new(
            "detect subject",
            vec![],
            Default::default(),
            &pixelox_core::ctx::NoIo,
            &pixelox_core::ctx::NoProgress,
        );
        let r = side_recipe(&d, &a, &ctx, final_step("export", json!({"apply_mask": "@sel"})));
        let step = &r[r.len() - 2];
        assert_eq!(step.op, "detect.subject");
        assert_eq!(step.params["select"], json!(true));
        assert!(step.params.get("cutout").is_none() && step.params.get("save_mask").is_none());
        assert_eq!(r.last().unwrap().op, "export");
    }
}
