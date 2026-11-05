//! Document and file commands (PRD §8.2, §8.18, `analyze exif`).

use crate::{anim, codec, meta};
use pixelox_core::ctx::{format_of, is_pxo, EncodeOptions};
use pixelox_core::doc::{InputRef, RecipeStep};
use pixelox_core::exec::recipe_with;
use pixelox_core::kernels::Filter;
use pixelox_core::prelude::*;
use pixelox_core::presets;
use std::path::{Path, PathBuf};

const PRESET_NAMES: &[&str] = &[
    "a4",
    "a3",
    "letter",
    "business-card",
    "poster-18x24",
    "flier-letter",
    "ig-post",
    "ig-portrait",
    "ig-story",
    "x-post",
    "yt-thumb",
    "og-card",
    "fb-cover",
];

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::source("new", Category::Document, "Create an empty document", new_doc)
            .about("Creates a document with one `Background` layer filled with --fill (white by default; `transparent` for none). Give the canvas with --size or a named --preset (see `pixelox comp presets`).")
            .param(param("size", ParamKind::Size).help("Canvas size in px, e.g. 1080x1350"))
            .param(param("preset", ParamKind::Choice).choices(PRESET_NAMES).help("Canvas preset (sets size and dpi)"))
            .param(param("fill", ParamKind::Color).default("white").help("Background fill color, or `transparent`"))
            .param(param("dpi", ParamKind::Float).range("1..10000").help("Resolution for pt/mm/in units [default: 72, or the preset's]"))
            .param(param("color-space", ParamKind::Choice).choices(&["srgb", "display-p3", "adobe-rgb"]).default("srgb").help("Working color space"))
            .param(param("depth", ParamKind::Choice).choices(&["8", "16"]).default("8").help("Bits per channel stored in the document"))
            .example("pixelox new -o doc.pxo --size 1080x1350", "A white 4:5 canvas")
            .example("pixelox new -o story.pxo --preset ig-story --fill '#101014'", "A dark Instagram story canvas")
            .example("pixelox new -o badge.pxo --size 1000x1000 --fill transparent", "A transparent canvas for a sticker"),
        OpSpec::source("open", Category::Document, "Open any image as a document; inspect without -o", open_doc)
            .inputs(Inputs::ONE)
            .about("Loads JPEG, PNG, GIF, TIFF, WebP, BMP, PSD (layers kept per the fidelity matrix), SVG, or .pxo. With -o doc.pxo it converts to a document; with an image path it converts formats; without -o it prints a summary. EXIF orientation is applied unless --no-auto-orient.")
            .param(param("page", ParamKind::Int).range("1..100000").help("Page of a multi-page TIFF (1-based)"))
            .param(param("frame", ParamKind::Int).range("1..100000").help("Frame of an animated GIF (1-based)"))
            .param(param("icc", ParamKind::Choice).choices(&["honor", "assign", "ignore"]).default("honor").help("Embedded ICC profile: honor (keep and re-embed), assign, or ignore"))
            .param(param("no-auto-orient", ParamKind::Flag).help("Keep pixels as stored instead of applying EXIF orientation"))
            .example("pixelox open shot.jpg -o shot.pxo", "Make a layered document from a photo")
            .example("pixelox open brand.psd -o brand.pxo --json", "Import a layered PSD")
            .example("pixelox open anim.gif -o frame3.png --frame 3", "Pull one frame out of an animated GIF"),
        OpSpec::doc("save", Category::Document, "Normalize a document and drop unused assets", save_doc)
            .about("Rewrites a .pxo document in normalized form and removes embedded assets no layer uses. Use with --in-place (or -o for a copy).")
            .param(param("no-cache", ParamKind::Flag).help("Do not store rendered caches (pixelox never stores them; accepted for compatibility)"))
            .example("pixelox save doc.pxo --in-place", "Normalize and garbage-collect a document")
            .example("pixelox save doc.pxo -o clean.pxo", "Write a normalized copy")
            .example("pixelox save doc.pxo --in-place --no-cache --json", "Report removed assets as JSON"),
        OpSpec::doc("info", Category::Document, "Inspect a file or document", info)
            .read_only()
            .about("Size, color space, depth, the layer tree, masks, the selection, guides, template slots, fonts, metadata, and the recipe length.")
            .example("pixelox info shot.jpg --json", "Inspect a photo")
            .example("pixelox info brand.pxo --json", "See a document's layers and masks")
            .example("pixelox info brand.psd", "Check how a PSD imports"),
        OpSpec::doc("export", Category::Document, "Render a deliverable (format from the -o extension)", export)
            .export_mode()
            .about("Flattens the document (or one --layer) and encodes it. PNG and TIFF keep 16 bits with --bit-depth 16; JPEG flattens transparency onto --background-color. --apply-mask multiplies a mask into alpha (the cutout path). --ladder writes several sizes and --formats several formats into the -o directory in one pass. PSD writes flattened by default or --layered-basic.")
            .param(param("quality", ParamKind::Int).range("1..100").help("JPEG quality [default: 90]"))
            .param(param("bit-depth", ParamKind::Choice).choices(&["8", "16"]).help("Bits per channel [default: the document's where the format allows]"))
            .param(param("scale", ParamKind::Float).range("0.001..100").help("Scale factor applied after rendering"))
            .param(param("size", ParamKind::Size).help("Output size; one side blank keeps the aspect (2048x)"))
            .param(param("interp", ParamKind::Choice).choices(kernels::FILTERS).help("Resampling filter [default: lanczos3 down, bicubic up]"))
            .param(common::layer().help("Export only this layer or group"))
            .param(param("apply-mask", ParamKind::Mask).help("Multiply a mask into the output alpha (named mask, @sel, @alpha:<layer>, or image)"))
            .param(param("region", ParamKind::Rect).help("Export only this region of the canvas"))
            .param(param("flatten", ParamKind::Flag).help("Flatten transparency onto --background-color"))
            .param(param("background-color", ParamKind::Color).default("white").help("Background for --flatten and formats without alpha (--background is the global run-as-job flag)"))
            .param(param("strip-metadata", ParamKind::Flag).help("Drop EXIF from the output (the ICC profile is kept)"))
            .param(param("profile", ParamKind::Str).help("Output profile: srgb (default); conversions need a color engine not in this build"))
            .param(param("progressive", ParamKind::Flag).help("Progressive JPEG (falls back to baseline in this build, with a warning)"))
            .param(param("compression", ParamKind::Choice).choices(&["lzw", "deflate", "none"]).help("TIFF compression [default: lzw]"))
            .param(param("layered-basic", ParamKind::Flag).help("PSD: write raster layers, groups, opacity, blend modes, and masks"))
            .param(param("colors", ParamKind::Int).range("2..256").help("GIF palette size [default: 256]"))
            .param(param("dither", ParamKind::Choice).choices(&["fs", "ordered", "none"]).help("GIF dithering [default: fs]"))
            .param(param("ladder", ParamKind::List).help("Longest-side sizes to export into the -o directory, e.g. 2048,1024,512"))
            .param(param("formats", ParamKind::List).help("Formats for --ladder, e.g. png,webp [default: the input's]"))
            .example("pixelox export shot.pxo -o cutout.png --apply-mask subject", "Transparent cutout through a mask")
            .example("pixelox export poster.pxo -o poster-og.png --size 1200x630 --region 0,840,3508x1842", "Crop a region and resize it")
            .example("pixelox export shot.pxo -o interior-final.tiff --bit-depth 16", "16-bit TIFF deliverable")
            .example("pixelox export hero.jpg --ladder 2048,1024,512 --formats jpg,webp -o dist/", "A size and format ladder in one pass"),
        OpSpec::custom("frames extract", Category::Frames, "Explode an animated GIF or multi-page TIFF to stills", frames_extract)
            .about("Writes each frame (GIF, composited) or page (TIFF) as its own image in the -o directory, plus frames.json with delays. Each still's sidecar replays it on its own.")
            .param(param("frames", ParamKind::Str).default("all").help("Which frames: all, N, or A..B (1-based, inclusive)"))
            .param(param("pattern", ParamKind::Str).default("frame-{n:04}.png").help("File name pattern; {n} or {n:04} is the frame number"))
            .example("pixelox frames extract anim.gif -o frames/", "Every frame as PNG")
            .example("pixelox frames extract anim.gif -o frames/ --frames 1..10 --pattern 'f-{n:03}.png'", "The first ten frames")
            .example("pixelox frames extract scan.tiff -o pages/ --pattern 'page-{n}.jpg'", "Every TIFF page as JPEG"),
        OpSpec::custom("frames assemble", Category::Frames, "Build an animated GIF from stills", frames_assemble)
            .inputs(Inputs::MANY)
            .about("Quantizes each frame with a deterministic median-cut palette and dithering, then writes an animated GIF. Frames must share one size.")
            .param(param("fps", ParamKind::Float).default("12").range("0.1..100").help("Frames per second"))
            .param(param("loop", ParamKind::Str).default("forever").help("Loop count: forever or a number"))
            .param(param("colors", ParamKind::Int).default("256").range("2..256").help("Palette size per frame"))
            .param(param("dither", ParamKind::Choice).choices(&["fs", "ordered", "none"]).default("fs").help("Dithering"))
            .example("pixelox frames assemble 'frames/frame-*.png' -o out.gif --fps 12", "Assemble stills into a GIF")
            .example("pixelox frames assemble a.png b.png c.png -o blink.gif --fps 2 --loop 3", "Three frames, three loops")
            .example("pixelox frames assemble 'frames/*.png' -o small.gif --colors 64 --dither ordered", "A smaller palette with ordered dithering"),
        OpSpec::custom("analyze exif", Category::Analyze, "EXIF, XMP, and ICC metadata", analyze_exif)
            .about("Reads metadata without decoding pixels: EXIF fields, orientation, the ICC profile's name, and XMP (its raw packet with --raw).")
            .param(param("raw", ParamKind::Flag).help("Include the raw XMP packet"))
            .example("pixelox analyze exif shot.jpg --json", "Camera, exposure, and orientation")
            .example("pixelox analyze exif shot.jpg --raw --json", "Include the XMP packet")
            .example("pixelox analyze exif scan.tiff", "Check a TIFF's embedded profile"),
    ]
}

fn new_doc(a: &Args, ctx: &mut OpCtx) -> Result<Document> {
    let (w, h, mut dpi) = match (a.str("preset"), a.size("size")?) {
        (Some(_), Some(_)) => return Err(Error::new("PX1005", "pass --size or --preset, not both")),
        (Some(p), None) => presets::find(&p).ok_or_else(|| Error::usage(format!("unknown preset '{p}'")))?,
        (None, Some(s)) => {
            if s.w.is_none() || s.h.is_none() || s.w.map(|l| l.unit == units::Unit::Pct).unwrap_or(false) {
                return Err(Error::new("PX1011", "--size needs both sides in px for a new document, e.g. 1080x1350"));
            }
            let dpi = a.opt_f64("dpi")?.unwrap_or(72.0);
            let (w, h) = s.resolve(0.0, 0.0, dpi);
            (w, h, dpi as f32)
        }
        (None, None) => {
            return Err(Error::new("PX1003", "'new' needs --size WxH or --preset NAME")
                .with_hint("pixelox new -o doc.pxo --size 1080x1350"))
        }
    };
    if let Some(d) = a.opt_f64("dpi")? {
        dpi = d as f32;
    }
    if w as u64 * h as u64 > ctx.globals.open.limit_pixels {
        return Err(Error::new("PX1104", format!("{w}x{h} exceeds the pixel limit")));
    }
    let fill = a.color("fill")?;
    let mut doc = Document::from_image(Image::filled(w, h, fill));
    doc.dpi = dpi;
    doc.color_space = a.choice("color-space");
    doc.depth = a.choice("depth").parse().unwrap_or(8);
    Ok(doc)
}

fn open_doc(a: &Args, ctx: &mut OpCtx) -> Result<Document> {
    let path = ctx.input(0)?.to_path_buf();
    if let Some(p) = a.opt_int("page")? {
        ctx.globals.open.page = Some(p as usize);
    }
    if let Some(f) = a.opt_int("frame")? {
        ctx.globals.open.frame = Some(f as usize);
    }
    ctx.globals.open.icc = a.choice("icc");
    if a.flag("no-auto-orient") {
        ctx.globals.open.auto_orient = false;
    }
    ctx.load_document(&path)
}

fn referenced_assets(doc: &Document) -> Vec<String> {
    let mut v = Vec::new();
    for (_, l) in doc.walk() {
        if let LayerKind::Placed { asset, .. } = &l.content {
            v.push(asset.clone());
        }
        for f in &l.fx {
            if let Some(Value::String(k)) = f.params.get("asset") {
                v.push(k.clone());
            }
        }
        for key in ["asset", "font_asset"] {
            if let LayerKind::Text { spec } | LayerKind::Shape { spec } = &l.content {
                if let Some(Value::String(k)) = spec.get(key) {
                    v.push(k.clone());
                }
            }
        }
    }
    v
}

fn save_doc(_a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let used = referenced_assets(doc);
    let before = doc.assets.len();
    doc.assets.retain(|k, _| used.contains(k));
    Ok(json!({"assets_removed": before - doc.assets.len(), "assets_kept": doc.assets.len()}))
}

fn mask_info(m: &Mask) -> Value {
    json!({"coverage": pixelox_core::doc::round3(m.coverage()), "bbox": m.bbox(0.0).map(|r| r.to_json())})
}

fn info(_a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let path = ctx.input(0)?.to_path_buf();
    let file = ctx.io.probe(&path).ok().map(|i| serde_json::to_value(i).unwrap_or(Value::Null));
    let masks: Map<String, Value> = doc.masks.iter().map(|(k, m)| (k.clone(), mask_info(m))).collect();
    let fonts: Vec<Value> = doc.recipe.iter().flat_map(|s| s.fonts.clone()).collect();
    Ok(json!({
        "file": file,
        "size": [doc.width, doc.height],
        "color": doc.color_space,
        "depth": doc.depth,
        "dpi": doc.dpi,
        "layers": doc.tree_json(),
        "layer_count": doc.layer_count(),
        "masks": masks,
        "selection": doc.selection.as_ref().map(mask_info),
        "guides": doc.guides,
        "grid": doc.grid,
        "slots": doc.slots,
        "fonts": fonts,
        "meta": {
            "source_format": doc.meta.source_format,
            "icc": doc.meta.icc_name,
            "exif": doc.meta.exif.is_some(),
        },
        "assets": doc.assets.len(),
        "recipe_steps": doc.recipe.len(),
    }))
}

/// The ids of a layer and everything inside it.
fn ids_of(l: &Layer, out: &mut Vec<String>) {
    out.push(l.id.clone());
    if let Some(ch) = l.children() {
        for c in ch {
            ids_of(c, out);
        }
    }
}

fn ext_for(format: &str) -> &str {
    match format {
        "jpeg" => "jpg",
        "tiff" => "tiff",
        other => other,
    }
}

fn canonical_format(f: &str) -> Result<String> {
    let p = PathBuf::from(format!("x.{}", f.trim().trim_start_matches('.')));
    let fmt = format_of(&p).unwrap_or_default();
    if crate::WRITE_FORMATS.contains(&fmt.as_str()) && fmt != "pxo" {
        Ok(fmt)
    } else {
        Err(Error::new("PX1106", format!("cannot export format '{f}'")))
    }
}

struct Rendering {
    image: Image,
}

fn render_for_export(a: &Args, doc: &Document, ctx: &mut OpCtx) -> Result<Rendering> {
    let mut img = match a.raw("layer") {
        Some(r) => {
            let p = doc.resolve(r)?;
            let mut ids = Vec::new();
            ids_of(doc.layer(&p), &mut ids);
            compose::composite_subset(doc, &ids)?
        }
        None => compose::composite(doc)?,
    };
    if let Some(m) = a.str("apply-mask") {
        let mask = ctx.resolve_mask(doc, &m)?;
        let w = img.width as usize;
        for (i, p) in img.data.chunks_mut(4).enumerate() {
            p[3] *= mask.data.get(i).copied().unwrap_or(0.0);
            let _ = w;
        }
    }
    if let Some(r) = a.rect("region")? {
        let rf = r.resolve(doc.width as f64, doc.height as f64, doc.dpi()).round();
        if rf.is_empty() {
            return Err(Error::new("PX3320", "--region is empty"));
        }
        img = img.crop(rf);
    }
    Ok(Rendering { image: img })
}

fn resize_for(img: &Image, a: &Args, size_override: Option<&units::SizeSpec>, dpi: f64) -> Result<Image> {
    let (w0, h0) = (img.width as f64, img.height as f64);
    let target = if let Some(s) = size_override.copied().or(a.size("size")?) {
        Some(s.resolve(w0, h0, dpi))
    } else {
        a.opt_f64("scale")?.map(|k| (((w0 * k).round().max(1.0)) as u32, ((h0 * k).round().max(1.0)) as u32))
    };
    match target {
        Some((w, h)) if (w, h) != (img.width, img.height) => {
            let f = match a.str("interp") {
                Some(i) => Filter::parse(&i)?,
                None => Filter::auto(img.width.max(img.height), w.max(h)),
            };
            Ok(kernels::resize(img, w, h, f))
        }
        _ => Ok(img.clone()),
    }
}

fn encode_opts(a: &Args, doc: &Document, format: &str) -> Result<EncodeOptions> {
    let bit_depth = match a.str("bit-depth") {
        Some(b) => Some(b.parse::<u8>().unwrap_or(8)),
        None if matches!(format, "png" | "tiff") => Some(doc.depth),
        None => Some(8),
    };
    Ok(EncodeOptions {
        format: format.to_string(),
        quality: a.opt_int("quality")?.map(|q| q as u8),
        bit_depth,
        progressive: a.flag("progressive"),
        compression: a.str("compression"),
        strip_metadata: a.flag("strip-metadata"),
        profile: a.str("profile"),
        layered: a.flag("layered-basic"),
        lossless: None,
        dither: a.str("dither"),
        colors: a.opt_int("colors")?.map(|c| c as u16),
        dpi: Some(doc.dpi),
    })
}

fn export(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let out =
        ctx.globals.output.clone().ok_or_else(|| {
            Error::new("PX1004", "export needs -o <path>").with_hint("pixelox export doc.pxo -o out.png")
        })?;
    let out = PathBuf::from(out);
    let rendering = render_for_export(a, doc, ctx)?;
    // the step records the mask file (if any) read while rendering
    let spec_step = RecipeStep {
        op: "export".into(),
        params: a.to_json(),
        inputs: ctx.step_inputs.clone(),
        model: None,
        fonts: vec![],
    };
    let bg = a.color("background-color")?;
    let flatten = |img: Image| if a.flag("flatten") { img.flatten_onto(bg) } else { img };
    let ladder = a.list("ladder");
    if ladder.is_empty() {
        if is_pxo(&out) {
            return Err(Error::new(
                "PX1106",
                "export writes deliverables; save documents with -o doc.pxo on any command, or `pixelox save`",
            ));
        }
        let format = match &ctx.globals.format {
            Some(f) => canonical_format(f)?,
            None => canonical_format(&format_of(&out).unwrap_or_else(|| "png".into()))?,
        };
        let img = flatten(resize_for(&rendering.image, a, None, doc.dpi())?);
        let opts = encode_opts(a, doc, &format)?;
        let recipe = recipe_with(doc, spec_step);
        ctx.write_image(&out, &img, Some(doc), &opts, None, Some(recipe))?;
        return Ok(json!({"size": [img.width, img.height], "format": format}));
    }
    // ladder: -o is a directory
    let stem = ctx.input(0)?.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_else(|| "export".into());
    let formats: Vec<String> = {
        let f = a.list("formats");
        if f.is_empty() {
            vec![canonical_format(
                &format_of(ctx.input(0)?)
                    .filter(|f| f != "pxo" && f != "psd" && f != "svg")
                    .unwrap_or_else(|| "png".into()),
            )?]
        } else {
            f.iter().map(|x| canonical_format(x)).collect::<Result<Vec<_>>>()?
        }
    };
    let mut written = Vec::new();
    for rung in &ladder {
        let n: u32 = rung
            .trim()
            .parse()
            .map_err(|_| Error::new("PX1010", format!("--ladder entries are pixel sizes, got '{rung}'")))?;
        let (w0, h0) = (rendering.image.width, rendering.image.height);
        let size = if w0 >= h0 {
            units::SizeSpec::parse(&format!("{n}x"))?
        } else {
            units::SizeSpec::parse(&format!("x{n}"))?
        };
        let img = flatten(resize_for(&rendering.image, a, Some(&size), doc.dpi())?);
        for fmt in &formats {
            let path = out.join(format!("{stem}-{n}.{}", ext_for(fmt)));
            let opts = encode_opts(a, doc, fmt)?;
            let mut step = spec_step.clone();
            step.params.remove("ladder");
            step.params.remove("formats");
            step.params.remove("scale");
            step.params.insert("size".into(), json!(if w0 >= h0 { format!("{n}x") } else { format!("x{n}") }));
            ctx.write_image(&path, &img, Some(doc), &opts, None, Some(recipe_with(doc, step)))?;
            written.push(json!({"path": path.display().to_string(), "size": [img.width, img.height], "format": fmt}));
        }
    }
    Ok(json!({"ladder": written}))
}

fn frame_selection(spec: &str, total: usize) -> Result<Vec<usize>> {
    let s = spec.trim();
    if s == "all" {
        return Ok((1..=total).collect());
    }
    if let Some((a, b)) = s.split_once("..") {
        let a: usize = a.trim().parse().map_err(|_| Error::new("PX1010", format!("bad --frames '{s}'")))?;
        let b: usize = if b.trim().is_empty() {
            total
        } else {
            b.trim().parse().map_err(|_| Error::new("PX1010", format!("bad --frames '{s}'")))?
        };
        return Ok((a.max(1)..=b.min(total)).collect());
    }
    let n: usize = s.parse().map_err(|_| Error::new("PX1010", format!("--frames is all, N, or A..B; got '{s}'")))?;
    if n == 0 || n > total {
        return Err(Error::new("PX1013", format!("frame {n} is out of range 1..{total}")));
    }
    Ok(vec![n])
}

fn format_pattern(pattern: &str, n: usize) -> String {
    let mut out = String::new();
    let mut rest = pattern;
    while let Some(start) = rest.find("{n") {
        out.push_str(&rest[..start]);
        let after = &rest[start..];
        let Some(end) = after.find('}') else {
            out.push_str(after);
            return out;
        };
        let spec = &after[2..end];
        let width: usize = spec.trim_start_matches(':').trim_start_matches('0').parse().unwrap_or(0);
        if spec.starts_with(":0") {
            out.push_str(&format!("{n:0width$}"));
        } else if width > 0 {
            out.push_str(&format!("{n:width$}"));
        } else {
            out.push_str(&n.to_string());
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

fn frames_extract(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let input = ctx.input(0)?.to_path_buf();
    let dir = PathBuf::from(
        ctx.globals.output.clone().ok_or_else(|| Error::new("PX1004", "frames extract needs -o <directory>"))?,
    );
    let bytes = ctx.read_aux(&input, "source", None)?;
    let src_ref = ctx.step_inputs.last().cloned().unwrap_or(InputRef {
        role: "source".into(),
        path: input.display().to_string(),
        sha256: String::new(),
        bytes: 0,
        param: None,
    });
    let fmt = codec::sniff(&bytes, input.extension().and_then(|e| e.to_str())).unwrap_or("");
    let (frames, key): (Vec<(Image, u32)>, &str) = match fmt {
        "gif" => (anim::decode_gif_frames(&bytes)?, "frame"),
        "tiff" => {
            let mut pages = Vec::new();
            let mut n = 1;
            loop {
                let o = pixelox_core::ctx::OpenOptions { page: Some(n), auto_orient: false, ..Default::default() };
                match codec::decode_raster(&bytes, "tiff", &o) {
                    Ok((img, _)) => pages.push((img, 0)),
                    Err(_) if n > 1 => break,
                    Err(e) => return Err(e),
                }
                n += 1;
                if n > 10_000 {
                    break;
                }
            }
            (pages, "page")
        }
        other => {
            return Err(Error::new(
                "PX1105",
                format!(
                    "frames extract needs an animated GIF or a multi-page TIFF, got {}",
                    if other.is_empty() { "an unknown format" } else { other }
                ),
            ))
        }
    };
    let sel = frame_selection(&a.str_or("frames", "all"), frames.len())?;
    let pattern = a.str_or("pattern", "frame-{n:04}.png");
    let mut manifest = Vec::new();
    for n in sel {
        let (img, delay) = &frames[n - 1];
        let path = dir.join(format_pattern(&pattern, n));
        let fmt = format_of(&path).unwrap_or_else(|| "png".into());
        let opts = EncodeOptions { format: fmt, ..Default::default() };
        let mut params = Map::new();
        params.insert(key.into(), json!(n.to_string()));
        let recipe = vec![
            RecipeStep { op: "open".into(), params, inputs: vec![src_ref.clone()], model: None, fonts: vec![] },
            RecipeStep { op: "export".into(), params: Map::new(), inputs: vec![], model: None, fonts: vec![] },
        ];
        ctx.write_image(&path, img, None, &opts, None, Some(recipe))?;
        manifest.push(json!({"index": n, "path": path.display().to_string(), "delay_ms": delay}));
    }
    let m = json!({"source": input.display().to_string(), "frames": manifest});
    ctx.write_file(&dir.join("frames.json"), &serde_json::to_vec_pretty(&m)?, Some("manifest"), None)?;
    Ok(json!({"frames": m["frames"].as_array().map(|a| a.len()).unwrap_or(0), "total_in_source": frames.len()}))
}

fn frames_assemble(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let out = PathBuf::from(
        ctx.globals.output.clone().ok_or_else(|| Error::new("PX1004", "frames assemble needs -o out.gif"))?,
    );
    if format_of(&out).as_deref() != Some("gif") {
        return Err(Error::new("PX1106", "frames assemble writes animated GIF; use an .gif output"));
    }
    let fps = a.f64("fps")?;
    let delay = (1000.0 / fps).round() as u32;
    let repeat = match a.str_or("loop", "forever").as_str() {
        "forever" | "infinite" | "0" => Some(0u16),
        n => Some(
            n.parse::<u16>().map_err(|_| Error::new("PX1010", format!("--loop is forever or a number, got '{n}'")))?,
        ),
    };
    let inputs: Vec<PathBuf> = ctx.inputs.clone();
    let mut frames = Vec::new();
    for p in &inputs {
        let (img, _) = ctx.decode_aux(p, "source", None)?;
        frames.push((img, delay));
    }
    let bytes = anim::encode_gif(&frames, a.int("colors")? as u16, &a.choice("dither"), repeat)?;
    ctx.write_file(&out, &bytes, None, None)?;
    Ok(json!({"frames": frames.len(), "size": [frames[0].0.width, frames[0].0.height], "delay_ms": delay}))
}

fn analyze_exif(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    use image::{ImageDecoder, ImageReader};
    let input = ctx.input(0)?.to_path_buf();
    let bytes = ctx.read_aux(&input, "source", None)?;
    let fmt = codec::sniff(&bytes, input.extension().and_then(|e| e.to_str())).unwrap_or("unknown");
    let (mut exif, mut icc, mut orientation) = (None, None, None);
    if matches!(fmt, "png" | "jpeg" | "webp" | "tiff" | "gif" | "bmp") {
        if let Ok(r) = ImageReader::new(std::io::Cursor::new(&bytes)).with_guessed_format() {
            if let Ok(mut d) = r.into_decoder() {
                exif = d.exif_metadata().ok().flatten();
                icc = d.icc_profile().ok().flatten();
                orientation = d.orientation().ok().map(|o| o.to_exif());
            }
        }
    }
    let xmp = find_xmp(&bytes);
    Ok(json!({
        "format": fmt,
        "bytes": bytes.len(),
        "exif": exif.as_deref().map(meta::exif_fields),
        "orientation": orientation,
        "icc": icc.as_deref().map(|i| json!({"name": meta::icc_name(i), "bytes": i.len()})),
        "xmp": match (&xmp, a.flag("raw")) {
            (Some(x), true) => json!({"present": true, "packet": x}),
            (Some(x), false) => json!({"present": true, "bytes": x.len()}),
            (None, _) => json!({"present": false}),
        },
    }))
}

fn find_xmp(bytes: &[u8]) -> Option<String> {
    let start = find_sub(bytes, b"<x:xmpmeta")?;
    let end = find_sub(&bytes[start..], b"</x:xmpmeta>")? + start + "</x:xmpmeta>".len();
    Some(String::from_utf8_lossy(&bytes[start..end]).to_string())
}

fn find_sub(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

/// Used by tests and examples: the path a ladder rung writes.
pub fn ladder_path(dir: &Path, stem: &str, n: u32, format: &str) -> PathBuf {
    dir.join(format!("{stem}-{n}.{}", ext_for(format)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patterns_and_selections() {
        assert_eq!(format_pattern("frame-{n:04}.png", 7), "frame-0007.png");
        assert_eq!(format_pattern("f{n}.png", 12), "f12.png");
        assert_eq!(frame_selection("all", 3).unwrap(), vec![1, 2, 3]);
        assert_eq!(frame_selection("2..", 4).unwrap(), vec![2, 3, 4]);
        assert!(frame_selection("9", 3).is_err());
    }

    #[test]
    fn new_document_from_preset_and_size() {
        let spec = pixelox_core::testing::find(&ops(), "new");
        let d = pixelox_core::testing::run_source(&spec, &[("preset", "ig-story")]).unwrap();
        assert_eq!((d.width, d.height), (1080, 1920));
        let d = pixelox_core::testing::run_source(&spec, &[("size", "40x30"), ("fill", "transparent")]).unwrap();
        assert_eq!(d.layers[0].raster().unwrap().get(0, 0)[3], 0.0);
        assert!(pixelox_core::testing::run_source(&spec, &[]).is_err());
        assert_eq!(
            pixelox_core::testing::run_source(&spec, &[("size", "40x30"), ("preset", "a4")]).unwrap_err().code,
            "PX1005"
        );
    }
}
