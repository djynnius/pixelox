//! The `adjust` commands: their specs, the destructive path (`apply_to_raster`), the
//! `--as-layer` path (an adjustment layer), run-time resolution (curve and LUT files are
//! embedded, `auto` values detected and recorded), and the compositor's adjuster hook.
//!
//! One code path serves the op and the hook: both build [`Args`] for the same spec and
//! call [`prepare`], so an adjustment layer renders exactly what the destructive op does.

use crate::util::{self, conflict, num};
use crate::{auto, curve, hue, local, lut, tone};
use pixelox_core::op::{json_to_pairs, DocFn, ParamSpec};
use pixelox_core::prelude::*;
use std::sync::OnceLock;

/// An adjustment ready to run on an image (colors only; alpha untouched).
pub type Job = Box<dyn Fn(&mut Image) + Send + Sync>;

/// A job that maps every pixel's RGB independently.
pub fn pixel_job(f: impl Fn([f32; 3]) -> [f32; 3] + Send + Sync + 'static) -> Job {
    Box::new(move |img: &mut Image| util::map_rgb(img, &f))
}

/// Every adjustment (command name without `adjust `) and the default name of its layer.
pub const OPS: &[(&str, &str)] = &[
    ("brightness-contrast", "Brightness/Contrast"),
    ("levels", "Levels"),
    ("curves", "Curves"),
    ("exposure", "Exposure"),
    ("shadows-highlights", "Shadows/Highlights"),
    ("tone-map", "Tone Map"),
    ("clarity", "Clarity"),
    ("texture", "Texture"),
    ("dehaze", "Dehaze"),
    ("hsl", "Hue/Saturation"),
    ("color-balance", "Color Balance"),
    ("vibrance", "Vibrance"),
    ("white-balance", "White Balance"),
    ("selective-color", "Selective Color"),
    ("channel-mixer", "Channel Mixer"),
    ("gradient-map", "Gradient Map"),
    ("lut", "Color Lookup"),
    ("bw", "Black & White"),
    ("invert", "Invert"),
    ("equalize", "Equalize"),
    ("posterize", "Posterize"),
    ("threshold", "Threshold"),
    ("auto", "Auto"),
];

/// Params that address the layer stack, not the adjustment; never stored in a layer.
const LAYER_ONLY: &[&str] = &["layer", "mask", "invert_mask", "as_layer", "name"];

/// Build the job for `op` from resolved args (no file reads, no detection).
pub fn prepare(op: &str, a: &Args) -> Result<Job> {
    match op {
        "brightness-contrast" => tone::brightness_contrast(a),
        "levels" => tone::levels(a),
        "curves" => tone::curves(a),
        "exposure" => tone::exposure(a),
        "shadows-highlights" => tone::shadows_highlights(a),
        "tone-map" => tone::tone_map(a),
        "clarity" => local::clarity(a),
        "texture" => local::texture(a),
        "dehaze" => local::dehaze(a),
        "hsl" => hue::hsl(a),
        "color-balance" => hue::color_balance(a),
        "vibrance" => hue::vibrance(a),
        "white-balance" => hue::white_balance(a),
        "selective-color" => hue::selective_color(a),
        "channel-mixer" => hue::channel_mixer(a),
        "gradient-map" => hue::gradient_map(a),
        "lut" => lut_job(a),
        "bw" => hue::bw(a),
        "invert" => tone::invert(a),
        "equalize" => tone::equalize(a),
        "posterize" => tone::posterize(a),
        "threshold" => tone::threshold(a),
        "auto" => auto_job(a),
        other => Err(Error::new("PX3001", format!("unknown adjustment '{other}'"))
            .with_context("known", OPS.iter().map(|(n, _)| *n).collect::<Vec<_>>())),
    }
}

/// Like [`prepare`], plus the slot where the job reports what it found while running
/// (dehaze: A, the patch, and the mean transmission).
fn prepare_reporting(op: &str, a: &Args) -> Result<(Job, Option<local::Report>)> {
    match op {
        "dehaze" => local::dehaze_reporting(a).map(|(job, r)| (job, Some(r))),
        _ => prepare(op, a).map(|job| (job, None)),
    }
}

fn lut_job(a: &Args) -> Result<Job> {
    let text = a.str("cube-text").ok_or_else(|| {
        Error::new("PX1003", "'adjust lut' needs --cube")
            .with_hint("pixelox adjust lut photo.jpg -o out.jpg --cube film.cube")
    })?;
    let table = lut::Lut::parse(&text)?;
    let s = a.f32("strength")? / 100.0;
    let interp = if a.choice("interp") == "trilinear" { lut::Interp::Trilinear } else { lut::Interp::Tetrahedral };
    Ok(pixel_job(move |p| {
        let o = table.apply(p, interp);
        [0, 1, 2].map(|c| p[c] + (o[c] - p[c]) * s)
    }))
}

fn auto_job(a: &Args) -> Result<Job> {
    if !(a.has("black") && a.has("white") && a.has("gamma")) {
        return Err(Error::internal("adjust auto ran before its endpoints were resolved"));
    }
    let black = util::triple(a, "black", 0.0, 255.0)?;
    let white = util::triple(a, "white", 0.0, 255.0)?;
    let gamma = util::triple(a, "gamma", 0.01, 9.99)?;
    Ok(tone::levels_rgb(black, white, gamma))
}

/// Flag combinations that make no sense (checked on the arguments as given).
fn check(op: &str, a: &Args) -> Result<()> {
    match op {
        "curves" => match (a.has("points"), a.has("curve")) {
            (true, true) => Err(conflict("points", "curve")),
            (false, false) => Err(Error::new("PX1003", "'adjust curves' needs --points or --curve")
                .with_hint("pixelox adjust curves photo.jpg -o out.jpg --points '0,0 64,56 192,210 255,255'")),
            _ => Ok(()),
        },
        "lut" => match (a.has("cube"), a.has("cube-text")) {
            (true, true) => Err(conflict("cube", "cube-text")),
            (false, false) => Err(Error::new("PX1003", "'adjust lut' needs --cube")
                .with_hint("pixelox adjust lut photo.jpg -o out.jpg --cube film.cube")),
            _ => Ok(()),
        },
        "white-balance" => {
            let manual = a.is_explicit("temp") || a.is_explicit("tint");
            if a.flag("auto") && a.has("from-point") {
                Err(conflict("auto", "from-point"))
            } else if a.flag("auto") && manual {
                Err(conflict("auto", "temp/--tint"))
            } else if a.has("from-point") && manual {
                Err(conflict("from-point", "temp/--tint"))
            } else {
                Ok(())
            }
        }
        "auto" => {
            let modes: Vec<&str> = ["levels", "contrast", "color", "tone"].into_iter().filter(|m| a.flag(m)).collect();
            if modes.len() > 1 {
                Err(Error::new("PX1005", format!("pick one auto correction, got --{}", modes.join(" --")))
                    .with_context("flags", modes))
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}

/// Read the files an adjustment references and embed their content in `a`: curve points
/// replace `--curve`, LUT text (`cube-text`) replaces `--cube`.
pub fn resolve_files(op: &str, a: &mut Args, ctx: &mut OpCtx) -> Result<()> {
    match op {
        "curves" => {
            if let Some(p) = a.path("curve") {
                let bytes = ctx.read_aux(&p, "curve", Some("curve"))?;
                let pts =
                    curve::parse_curve_json(&bytes).map_err(|e| e.with_context("file", p.display().to_string()))?;
                curve::Curve::new(&pts).map_err(|e| e.with_context("file", p.display().to_string()))?;
                a.set("points", curve::format_points(&pts));
                a.remove("curve");
            }
        }
        "lut" => {
            if let Some(p) = a.path("cube") {
                let bytes = ctx.read_aux(&p, "cube", Some("cube"))?;
                let text = String::from_utf8(bytes)
                    .map_err(|_| Error::new("PX3311", format!("{} is not a text .cube file", p.display())))?;
                lut::Lut::parse(&text).map_err(|e| e.with_context("file", p.display().to_string()))?;
                a.set("cube-text", text);
                a.remove("cube");
            }
        }
        _ => {}
    }
    Ok(())
}

/// The pixels an adjustment sees while image-dependent values are resolved: the target
/// layer (placed at `x`, `y`) or the composite below a new adjustment layer.
pub struct Source<'a> {
    pub img: &'a Image,
    pub x: i32,
    pub y: i32,
    /// Canvas width, height, and dpi (for `--from-point` units).
    pub canvas: (f64, f64, f64),
}

/// What resolution decided: values recorded for replay, facts for the result, warnings.
#[derive(Default)]
pub struct Resolved {
    pub record: Vec<(&'static str, Value)>,
    pub info: Map<String, Value>,
    pub warnings: Vec<(&'static str, String)>,
}

fn wb_auto(a: &Args) -> bool {
    a.flag("auto") || (!a.has("from-point") && !a.is_explicit("temp") && !a.is_explicit("tint"))
}

/// Whether `op` must look at pixels before it can run.
pub fn needs_image(op: &str, a: &Args) -> bool {
    match op {
        "white-balance" => a.has("from-point") || wb_auto(a),
        "auto" => !(a.has("black") && a.has("white") && a.has("gamma")),
        "dehaze" => a.is_auto("atmosphere") && a.f64("amount").map(|v| v != 0.0).unwrap_or(false),
        _ => false,
    }
}

/// Detect image-dependent values (`white-balance` auto and point, `adjust auto`) and set
/// them in `a` as explicit params.
pub fn resolve_image(op: &str, a: &mut Args, src: &Source) -> Result<Resolved> {
    let mut r = Resolved::default();
    if !needs_image(op, a) {
        return Ok(r);
    }
    match op {
        "white-balance" => {
            let (lin, method) = if let Some(pt) = a.point("from-point")? {
                let (cx, cy) = pt.resolve(src.canvas.0, src.canvas.1, src.canvas.2);
                let (ix, iy) = (cx.floor() as i64 - src.x as i64, cy.floor() as i64 - src.y as i64);
                let c = hue::sample_point(src.img, ix, iy).ok_or_else(|| {
                    Error::new("PX1010", format!("--from-point {},{} has no pixels to sample", num(cx, 1), num(cy, 1)))
                        .with_context("point", [cx, cy])
                        .with_hint("pick a point inside the layer's visible pixels (canvas coordinates)")
                })?;
                r.info.insert("sampled".into(), json!(Rgba::new(c[0], c[1], c[2], 1.0).to_hex()));
                (Some(c.map(|v| color::srgb_to_linear(v) as f64)), "point")
            } else {
                (hue::gray_world(src.img), "auto")
            };
            let (t, n, clamped) = match lin {
                Some(l) => hue::wb_params_for(l),
                None => (0.0, 0.0, false),
            };
            if clamped {
                r.warnings
                    .push(("PXW3001", "the color cast exceeds --temp/--tint ±100; the correction was clamped".into()));
            }
            a.set("temp", num(t, 2));
            a.set("tint", num(n, 2));
            a.remove("auto");
            a.remove("from-point");
            r.record = vec![
                ("temp", json!(num(t, 2))),
                ("tint", json!(num(n, 2))),
                ("auto", json!("false")),
                ("from_point", Value::Null),
            ];
            r.info.insert("white_balance".into(), json!({"method": method, "temp": t, "tint": n}));
        }
        "auto" => {
            let mode = if a.flag("levels") {
                auto::Mode::Levels
            } else if a.flag("contrast") {
                auto::Mode::Contrast
            } else if a.flag("color") {
                auto::Mode::Color
            } else {
                auto::Mode::Tone
            };
            let e = auto::detect(src.img, mode);
            let fmt = |v: [f32; 3], scale: f64, places: i32| {
                v.iter().map(|x| num(*x as f64 * scale, places)).collect::<Vec<_>>().join(",")
            };
            let (black, white, gamma) = (fmt(e.black, 255.0, 2), fmt(e.white, 255.0, 2), fmt(e.gamma, 1.0, 3));
            a.set("black", black.clone());
            a.set("white", white.clone());
            a.set("gamma", gamma.clone());
            let parsed = |s: &str| s.split(',').map(|v| v.parse::<f64>().unwrap_or(0.0)).collect::<Vec<_>>();
            r.info.insert(
                "auto".into(),
                json!({
                    "mode": format!("{mode:?}").to_lowercase(),
                    "black": parsed(&black),
                    "white": parsed(&white),
                    "gamma": parsed(&gamma),
                }),
            );
            r.record = vec![("black", json!(black)), ("white", json!(white)), ("gamma", json!(gamma))];
        }
        "dehaze" => {
            let patch = match a.opt_int("patch")? {
                Some(p) => p.max(1) as usize,
                None => local::auto_scale(op, src.canvas.0.max(src.canvas.1)).map_or(1, |(_, v)| v as usize),
            };
            // nothing visible: stay `auto` (the job then leaves the pixels alone)
            if let Some(c) = local::atmosphere(src.img, patch) {
                let hex = local::hex_of_linear(c);
                a.set("atmosphere", hex.clone());
                r.record = vec![("atmosphere", json!(hex))];
            }
        }
        _ => {}
    }
    Ok(r)
}

fn record(ctx: &mut OpCtx, r: &Resolved) {
    for (k, v) in &r.record {
        ctx.resolve_param(k, v);
    }
    for (code, msg) in &r.warnings {
        ctx.warn(code, msg.clone());
    }
}

/// The composite the new adjustment layer will see: every layer at or below `at` in
/// panel order (the whole document when `at` is None).
fn composite_below(doc: &Document, at: Option<&[usize]>) -> Result<Image> {
    match at {
        None => compose::composite(doc),
        Some(p) => {
            let ids: Vec<String> =
                doc.walk().into_iter().skip_while(|(q, _)| q.as_slice() != p).map(|(_, l)| l.id.clone()).collect();
            compose::composite_subset(doc, &ids)
        }
    }
}

fn merge(v: &mut Value, info: Map<String, Value>) {
    if let Value::Object(m) = v {
        for (k, val) in info {
            m.insert(k, val);
        }
    }
}

/// The runner shared by every `adjust` command.
pub fn run(op: &'static str, a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    check(op, a)?;
    if a.flag("as-layer") {
        return as_layer(op, a, doc, ctx);
    }
    if a.has("name") {
        return Err(Error::new("PX1005", "--name names the layer made by --as-layer")
            .with_hint("add --as-layer to make an adjustment layer, or drop --name"));
    }
    let path = target_layer(a, doc)?;
    if matches!(doc.layer(&path).content, LayerKind::Placed { .. }) && compose::smart_filter_eligible(&ctx.id, a) {
        // recorded as a smart filter on the placed layer (re-run at render time)
        return apply_to_raster(a, doc, ctx, |img, _| Ok(img.clone()));
    }
    let mut ra = a.clone();
    resolve_files(op, &mut ra, ctx)?;
    // radii that scale with the canvas (`auto`) become px before anything measures pixels
    let scale = local::resolve_scale(op, &mut ra, doc.width.max(doc.height) as f64)?;
    let resolved = {
        let layer = doc.layer(&path);
        let src = Source {
            img: layer.raster()?,
            x: layer.x,
            y: layer.y,
            canvas: (doc.width as f64, doc.height as f64, doc.dpi()),
        };
        resolve_image(op, &mut ra, &src)?
    };
    record(ctx, &resolved);
    let (job, report) = prepare_reporting(op, &ra)?;
    let mut v = apply_to_raster(a, doc, ctx, |img, _| {
        let mut out = img.clone();
        job(&mut out);
        Ok(out)
    })?;
    merge(&mut v, resolved.info);
    let report = report.map(|r| r.lock().map(|m| m.clone()).unwrap_or_default());
    merge(&mut v, local::info(op, scale, report));
    Ok(v)
}

fn as_layer(op: &'static str, a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    let at = match a.raw("layer") {
        Some(r) => Some(doc.resolve(r)?),
        None => None,
    };
    let mut ra = a.clone();
    resolve_files(op, &mut ra, ctx)?;
    let mut below = None;
    let resolved = if needs_image(op, &ra) {
        let img = composite_below(doc, at.as_deref())?;
        let src = Source { img: &img, x: 0, y: 0, canvas: (doc.width as f64, doc.height as f64, doc.dpi()) };
        let r = resolve_image(op, &mut ra, &src)?;
        below = Some(img);
        r
    } else {
        Resolved::default()
    };
    record(ctx, &resolved);
    // validate every value now rather than when the layer first renders
    let _validated: Job = prepare(op, &ra)?;
    // the layer keeps `auto` radii (they follow the canvas at render time); report the px
    let scale = local::resolve_scale(op, &mut ra.clone(), doc.width.max(doc.height) as f64)?;
    let report = if op == "dehaze" && ra.f64("amount")? != 0.0 {
        let img = match below {
            Some(img) => img,
            None => composite_below(doc, at.as_deref())?,
        };
        local::dehaze_summary(&ra, &img)?
    } else {
        None
    };
    let mut params = ra.to_json();
    for k in LAYER_ONLY {
        params.remove(*k);
    }
    let mask = ctx.mask_arg(a, doc)?;
    let name = match a.str("name") {
        Some(n) => {
            doc.check_name_free(&n, None)?;
            n
        }
        None => OPS.iter().find(|(n, _)| *n == op).map(|(_, t)| t.to_string()).unwrap_or_else(|| op.to_string()),
    };
    let mut layer = doc.new_layer(&name, LayerKind::Adjustment { op: op.to_string(), params: params.clone() });
    if let Some(m) = mask {
        layer.mask = Some(LayerMask::new(m));
    }
    let (id, lname) = (layer.id.clone(), layer.name.clone());
    let masked = layer.mask.is_some();
    doc.insert_above(at.as_deref(), layer);
    let mut shown = params;
    if let Some(Value::String(t)) = shown.remove("cube_text") {
        shown.insert("cube_text".into(), json!(format!("<{} bytes of .cube text>", t.len())));
    }
    let mut v = json!({
        "layer": lname,
        "layer_id": id,
        "as_layer": true,
        "masked": masked,
        "adjustment": {"op": op, "params": shown},
    });
    if let Some(p) = &at {
        v["above"] = json!(doc.layer(p).name);
    }
    merge(&mut v, resolved.info);
    merge(&mut v, local::info(op, scale, report));
    Ok(v)
}

/// The compositor hook (`compose::set_adjuster`): render an adjustment layer's `params`
/// (string-valued, as written by `--as-layer`) onto `img` in place.
pub fn adjuster(op: &str, params: &Map<String, Value>, img: &mut Image) -> Result<()> {
    static SPECS: OnceLock<Vec<OpSpec>> = OnceLock::new();
    let specs = SPECS.get_or_init(specs);
    let full = format!("adjust {op}");
    let spec = specs.iter().find(|s| s.name == full).ok_or_else(|| {
        Error::new("PX3001", format!("unknown adjustment layer op '{op}'"))
            .with_context("known", OPS.iter().map(|(n, _)| *n).collect::<Vec<_>>())
    })?;
    let mut a = Args::build(spec, json_to_pairs(&Value::Object(params.clone()))?)?;
    if (op == "curves" && !a.has("points")) || (op == "lut" && !a.has("cube-text")) {
        return Err(Error::new(
            "PX3001",
            format!("the {op} adjustment layer does not embed its data (no file reads at render time)"),
        )
        .with_hint(format!("re-create it with `pixelox adjust {op} <doc> --as-layer ...`")));
    }
    if needs_image(op, &a) {
        let snapshot = img.clone();
        let src = Source { img: &snapshot, x: 0, y: 0, canvas: (img.width as f64, img.height as f64, 72.0) };
        resolve_image(op, &mut a, &src)?;
    }
    let job = prepare(op, &a)?;
    job(img);
    Ok(())
}

// ---------------------------------------------------------------------------------------
// specs

macro_rules! runners {
    ($($f:ident => $op:literal),* $(,)?) => {
        $(fn $f(a: &Args, d: &mut Document, c: &mut OpCtx) -> Result<Value> { run($op, a, d, c) })*
    };
}

runners! {
    run_brightness_contrast => "brightness-contrast",
    run_levels => "levels",
    run_curves => "curves",
    run_exposure => "exposure",
    run_shadows_highlights => "shadows-highlights",
    run_tone_map => "tone-map",
    run_clarity => "clarity",
    run_texture => "texture",
    run_dehaze => "dehaze",
    run_hsl => "hsl",
    run_color_balance => "color-balance",
    run_vibrance => "vibrance",
    run_white_balance => "white-balance",
    run_selective_color => "selective-color",
    run_channel_mixer => "channel-mixer",
    run_gradient_map => "gradient-map",
    run_lut => "lut",
    run_bw => "bw",
    run_invert => "invert",
    run_equalize => "equalize",
    run_posterize => "posterize",
    run_threshold => "threshold",
    run_auto => "auto",
}

/// An adjust command with the shared `--layer/--mask/--invert-mask/--as-layer/--name`.
fn adj(name: &'static str, summary: &'static str, f: DocFn, params: Vec<ParamSpec>) -> OpSpec {
    OpSpec::doc(name, Category::Adjust, summary, f)
        .params(params)
        .params(common::region())
        .param(common::as_layer())
        .param(common::name())
}

fn amount(name: &'static str, range: &'static str, help: &'static str) -> ParamSpec {
    param(name, ParamKind::Float).default("0").range(range).help(help)
}

const CHANNELS: &[&str] = &["rgb", "r", "g", "b"];

pub fn specs() -> Vec<OpSpec> {
    vec![
        adj(
            "adjust brightness-contrast",
            "Brightness and contrast",
            run_brightness_contrast,
            vec![
                amount("brightness", "-100..100", "Lift (+) or darken (−) the midtones; black and white stay put"),
                amount("contrast", "-100..100", "S-curve around mid gray (+) or flatten toward it (−)"),
            ],
        )
        .about("Photoshop's (non-legacy) Brightness/Contrast: brightness bends the midtones with a gamma-like curve and contrast applies an endpoint-preserving S-curve, so neither clips black or white.")
        .example("pixelox adjust brightness-contrast photo.jpg -o out.jpg --brightness 20 --contrast 15", "Brighten and add a little punch")
        .example("pixelox adjust brightness-contrast doc.pxo --in-place --contrast 30 --as-layer --name Punch", "Non-destructive contrast layer")
        .example("pixelox adjust brightness-contrast doc.pxo --in-place --layer Background --brightness=-15 --mask sky", "Darken only the sky"),
        adj(
            "adjust levels",
            "Levels: input/output black and white points, gamma",
            run_levels,
            vec![
                param("channel", ParamKind::Choice).choices(CHANNELS).default("rgb").help("Channel to adjust: rgb (all three), r, g, or b"),
                param("in-black", ParamKind::Float).default("0").range("0..255").help("Input value mapped to black (0–255)"),
                param("in-white", ParamKind::Float).default("255").range("0..255").help("Input value mapped to white (0–255); must exceed --in-black"),
                param("gamma", ParamKind::Float).default("1").range("0.01..9.99").help("Midtone gamma: above 1 brightens, below 1 darkens"),
                param("out-black", ParamKind::Float).default("0").range("0..255").help("Output value for black (0–255)"),
                param("out-white", ParamKind::Float).default("255").range("0..255").help("Output value for white (0–255); below --out-black inverts"),
            ],
        )
        .about("Photoshop Levels on the 0–255 scale (8- and 16-bit documents alike): values below --in-black clip to --out-black, above --in-white to --out-white, and --gamma bends the midtones.")
        .example("pixelox adjust levels photo.jpg -o out.jpg --in-black 12 --in-white 240 --gamma 1.1", "Stretch tones and lift the midtones")
        .example("pixelox adjust levels photo.jpg -o out.jpg --channel b --out-white 230", "Tame a blue cast in the highlights")
        .example("pixelox adjust levels doc.pxo --in-place --in-black 20 --as-layer", "Levels adjustment layer"),
        adj(
            "adjust curves",
            "Tone curve through control points (monotone cubic)",
            run_curves,
            vec![
                param("points", ParamKind::Points).help("Control points 'in,out in,out ...' on the 0–255 scale; inputs must strictly increase"),
                param("curve", ParamKind::Path).help("JSON curve file: [[0,0],[64,80],[255,255]] or {\"points\": ...}"),
                param("channel", ParamKind::Choice).choices(CHANNELS).default("rgb").help("Channel to adjust: rgb (all three), r, g, or b"),
            ],
        )
        .about("Curves through the given points with a monotone cubic (Fritsch–Carlson), so the curve never overshoots between points; flat beyond the first and last point. Inputs that do not strictly increase fail with PX3310. With --as-layer the points of a --curve file are embedded in the layer.")
        .example("pixelox adjust curves photo.jpg -o out.jpg --points '0,0 64,56 192,210 255,255'", "Gentle S-curve")
        .example("pixelox adjust curves photo.jpg -o out.jpg --curve film.json --channel r", "Red-channel curve from a file")
        .example("pixelox adjust curves doc.pxo --in-place --points '0,20 255,235' --as-layer --name Fade", "Faded-film look as a layer"),
        adj(
            "adjust exposure",
            "Exposure in stops, offset, and gamma (linear light)",
            run_exposure,
            vec![
                amount("ev", "-5..5", "Exposure change in stops (+1 doubles the light)"),
                amount("offset", "-0.5..0.5", "Added to linear values: negative deepens shadows, positive lifts them"),
                param("gamma", ParamKind::Float).default("1").range("0.01..9.99").help("Gamma correction applied after exposure and offset"),
            ],
        )
        .about("Photoshop's Exposure adjustment: values are linearized, multiplied by 2^ev, offset, raised to 1/gamma, and re-encoded. Highlights pushed past white clip.")
        .example("pixelox adjust exposure photo.jpg -o out.jpg --ev 1", "One stop brighter")
        .example("pixelox adjust exposure photo.jpg -o out.jpg --ev=-0.5 --gamma 1.2", "Half a stop darker, open midtones")
        .example("pixelox adjust exposure doc.pxo --in-place --ev 0.7 --mask subject", "Brighten only the subject"),
        adj(
            "adjust shadows-highlights",
            "Lift shadows and recover highlights locally",
            run_shadows_highlights,
            vec![
                param("shadows", ParamKind::Float).default("35").range("0..100").help("Shadow lift (Photoshop's default is 35)"),
                param("highlights", ParamKind::Float).default("0").range("0..100").help("Highlight recovery"),
                param("radius", ParamKind::Float).default("30").range("0..2500").help("Neighborhood radius in px that decides what counts as shadow or highlight"),
            ],
        )
        .about("Brightens dark regions and darkens bright regions using the local average brightness (a Gaussian of --radius), so detail opens up without flattening global contrast.")
        .example("pixelox adjust shadows-highlights photo.jpg -o out.jpg", "Default shadow lift")
        .example("pixelox adjust shadows-highlights photo.jpg -o out.jpg --shadows 50 --highlights 30 --radius 60", "Backlit portrait rescue")
        .example("pixelox adjust shadows-highlights doc.pxo --in-place --shadows 40 --as-layer", "As an adjustment layer"),
        adj(
            "adjust tone-map",
            "Local HDR-style tone mapping",
            run_tone_map,
            vec![param("strength", ParamKind::Float).default("50").range("0..100").help("How strongly to compress large-scale contrast and boost local detail")],
        )
        .about("Splits log luminance into a smooth base (Gaussian, about 2% of the image size) and detail, compresses the base around the image's mean brightness, and boosts the detail; colors keep their hue.")
        .example("pixelox adjust tone-map photo.jpg -o out.jpg", "Default tone mapping")
        .example("pixelox adjust tone-map photo.jpg -o out.jpg --strength 80", "Strong HDR look")
        .example("pixelox adjust tone-map doc.pxo --in-place --strength 40 --as-layer", "As an adjustment layer"),
        adj(
            "adjust clarity",
            "Midtone local contrast (Lightroom Clarity)",
            run_clarity,
            vec![
                param("amount", ParamKind::Float).default("25").range("-100..100").help("Midtone local contrast: + adds punch, − softens into a glow; 0 changes nothing"),
                param("radius", ParamKind::Float).default("auto").auto().range("0.5..2500").help("Size of the neighborhood in px (Gaussian sigma); auto = 1.5% of the canvas's long side, at least 2"),
            ],
        )
        .about("Lightroom/Camera Raw Clarity. Perceptual luminance is split into a smooth, edge-aware base (a guided filter over --radius) and detail; the detail is scaled by --amount, weighted toward the midtones, and written back through linear light so hues are kept. Strong edges count as edges, not detail, so they gain no wide halos. Negative amounts fade the detail into the base for a soft, Orton-like look. The radius used is reported.")
        .example("pixelox adjust clarity photo.jpg -o out.jpg", "Default clarity (+25)")
        .example("pixelox adjust clarity photo.jpg -o out.jpg --amount 60 --radius 40", "Strong local contrast over a fixed radius")
        .example("pixelox adjust clarity doc.pxo --in-place --amount=-40 --as-layer --name Glow", "Dreamy softening as an adjustment layer"),
        adj(
            "adjust texture",
            "Boost or smooth medium-sized texture",
            run_texture,
            vec![
                param("amount", ParamKind::Float).default("25").range("-100..100").help("Surface detail: + brings out pores, fabric, and rock; − smooths it; 0 changes nothing"),
                param("radius", ParamKind::Float).default("auto").auto().range("0.5..500").help("Largest detail treated as texture, in px (Gaussian sigma); auto = 0.25% of the canvas's long side, at least 1.5"),
            ],
        )
        .about("Lightroom/Camera Raw Texture. The band between pixel-level noise (a 0.8 px blur) and --radius is taken from perceptual luminance with an edge-aware guided filter, so large edges and high-contrast outlines barely change while low-contrast surface detail is scaled by --amount; the change is written back through linear light so hues are kept. Negative amounts smooth skin and surfaces without blurring edges. The radius used is reported.")
        .example("pixelox adjust texture photo.jpg -o out.jpg", "Default texture boost (+25)")
        .example("pixelox adjust texture portrait.jpg -o out.jpg --amount=-50", "Smooth skin, keep edges sharp")
        .example("pixelox adjust texture doc.pxo --in-place --amount 60 --mask rocks", "Bring out rock texture inside a mask"),
        adj(
            "adjust dehaze",
            "Remove or add atmospheric haze",
            run_dehaze,
            vec![
                param("amount", ParamKind::Float).default("25").range("-100..100").help("Haze removed (+) or added (−); 0 changes nothing"),
                param("atmosphere", ParamKind::Color).default("auto").auto().help("Haze color A; auto = mean of the brightest 0.1% of the dark channel (detected values are recorded)"),
                param("patch", ParamKind::Int).default("auto").auto().range("1..500").advanced().help("Dark-channel patch radius in px; auto = 1.2% of the canvas's long side"),
            ],
        )
        .about("Lightroom/Camera Raw Dehaze by the dark channel prior (He, Sun & Tang 2009), in linear light. The dark channel (each pixel's smallest channel, min-filtered over a square --patch) locates haze; A is the mean color of its brightest 0.1%; the transmission t = 1 − ω·dark(I/A), ω = 0.95·amount/100, uses a morphological opening of the dark channel (so it keeps the scene's outlines) and is refined with a guided filter; and the scene is recovered as (I − A)/max(t, 0.1) + A. Negative amounts blend toward A, most where t is low (far away). The result reports A and the mean transmission; a detected A is recorded for replay and embedded in adjustment layers.")
        .example("pixelox adjust dehaze landscape.jpg -o out.jpg", "Default haze removal (+25)")
        .example("pixelox adjust dehaze landscape.jpg -o out.jpg --amount 70 --json", "Strong dehaze; the result reports A and mean transmission")
        .example("pixelox adjust dehaze doc.pxo --in-place --amount=-30 --as-layer --name Mist", "Add atmospheric haze as a layer"),
        adj(
            "adjust hsl",
            "Hue/saturation/lightness, per color range or colorize",
            run_hsl,
            vec![
                param("range", ParamKind::Choice)
                    .choices(&["master", "reds", "yellows", "greens", "cyans", "blues", "magentas"])
                    .default("master")
                    .help("Colors to affect: master (all) or one hue range with Photoshop's 30° falloff"),
                amount("hue", "-180..180", "Hue rotation in degrees (with --colorize: the tint hue; negative values wrap, −120 = 240°)"),
                amount("saturation", "-100..100", "Saturation change (with --colorize: tint saturation, 25 when 0)"),
                amount("lightness", "-100..100", "Blend toward white (+) or black (−)"),
                param("colorize", ParamKind::Flag).help("Replace hues with one tint, keeping the image's brightness"),
            ],
        )
        .about("Photoshop Hue/Saturation in HSL. A --range limits the change to one color family (full within ±15° of its hue, fading out by ±45°, and sparing near-neutral pixels). --colorize tints the whole image.")
        .example("pixelox adjust hsl photo.jpg -o out.jpg --saturation 20", "More saturated overall")
        .example("pixelox adjust hsl photo.jpg -o out.jpg --range blues --hue=-20 --saturation 30", "Deeper, more cyan-free blues")
        .example("pixelox adjust hsl photo.jpg -o out.jpg --colorize --hue 30 --saturation 40", "Warm monochrome tint"),
        adj(
            "adjust color-balance",
            "Color balance for shadows, midtones, highlights",
            run_color_balance,
            vec![
                param("shadows", ParamKind::List).default("0,0,0").help("Cyan–red, magenta–green, yellow–blue shifts for the shadows, each −100..100"),
                param("midtones", ParamKind::List).default("0,0,0").help("Cyan–red, magenta–green, yellow–blue shifts for the midtones, each −100..100"),
                param("highlights", ParamKind::List).default("0,0,0").help("Cyan–red, magenta–green, yellow–blue shifts for the highlights, each −100..100"),
                param("preserve-luma", ParamKind::Flag).help("Keep each pixel's lightness (Photoshop's Preserve Luminosity)"),
            ],
        )
        .about("Photoshop Color Balance: each triple pushes red, green, and blue (positive = red, green, blue; negative = cyan, magenta, yellow) within its tonal range, using overlapping range masks.")
        .example("pixelox adjust color-balance photo.jpg -o out.jpg --shadows=-10,0,15 --highlights 10,0,-10", "Teal shadows, warm highlights")
        .example("pixelox adjust color-balance photo.jpg -o out.jpg --midtones 15,0,-5 --preserve-luma", "Warm the midtones without brightening")
        .example("pixelox adjust color-balance doc.pxo --in-place --midtones=-5,5,0 --as-layer", "As an adjustment layer"),
        adj(
            "adjust vibrance",
            "Vibrance (protects saturated colors and skin)",
            run_vibrance,
            vec![
                amount("vibrance", "-100..100", "Saturation change weighted toward muted colors, sparing skin tones"),
                amount("saturation", "-100..100", "Uniform saturation change applied on top"),
            ],
        )
        .about("Photoshop Vibrance: boosts muted colors more than already-saturated ones and halves the boost on skin-tone hues; chroma is scaled around luma and limited to stay in gamut.")
        .example("pixelox adjust vibrance photo.jpg -o out.jpg --vibrance 40", "Livelier colors, natural skin")
        .example("pixelox adjust vibrance photo.jpg -o out.jpg --vibrance 30 --saturation=-10", "Vibrant but restrained")
        .example("pixelox adjust vibrance doc.pxo --in-place --vibrance 25 --as-layer", "As an adjustment layer"),
        adj(
            "adjust white-balance",
            "White balance: auto, temperature/tint, or gray point",
            run_white_balance,
            vec![
                param("auto", ParamKind::Flag).help("Detect and remove the color cast (gray world; the default when no other flag is given)"),
                amount("temp", "-100..100", "Temperature: warmer (+) or cooler (−)"),
                amount("tint", "-100..100", "Tint: magenta (+) or green (−)"),
                param("from-point", ParamKind::Point).help("Canvas point (X,Y) that should be neutral gray; a 3×3 average is sampled"),
            ],
        )
        .about("Channel gains in linear light. --auto and --from-point detect the temperature and tint that neutralize the cast; the detected values are recorded in the recipe (so replays do not re-detect) and embedded in adjustment layers.")
        .example("pixelox adjust white-balance photo.jpg -o out.jpg --auto", "Remove a color cast automatically")
        .example("pixelox adjust white-balance photo.jpg -o out.jpg --temp=-20 --tint 5", "Cool down a tungsten shot")
        .example("pixelox adjust white-balance photo.jpg -o out.jpg --from-point 120,340", "Make a gray card neutral"),
        adj(
            "adjust selective-color",
            "Selective color: CMYK shifts within one color family",
            run_selective_color,
            vec![
                param("target", ParamKind::Choice)
                    .choices(&["reds", "yellows", "greens", "cyans", "blues", "magentas", "whites", "neutrals", "blacks"])
                    .default("reds")
                    .help("Color family to change"),
                amount("c", "-100..100", "Cyan: + removes red, − adds it"),
                amount("m", "-100..100", "Magenta: + removes green, − adds it"),
                amount("y", "-100..100", "Yellow: + removes blue, − adds it"),
                amount("k", "-100..100", "Black: + darkens, − lightens"),
                param("relative", ParamKind::Flag).help("Scale changes by the ink already present (Photoshop's Relative method) instead of absolute"),
            ],
        )
        .about("Photoshop Selective Color: adds or removes cyan, magenta, yellow, and black in pixels belonging to the target family, weighted by how strongly each pixel belongs to it.")
        .example("pixelox adjust selective-color photo.jpg -o out.jpg --target reds --c=-20 --y 10", "Richer, warmer reds")
        .example("pixelox adjust selective-color photo.jpg -o out.jpg --target neutrals --k 10 --relative", "Deepen the neutrals")
        .example("pixelox adjust selective-color doc.pxo --in-place --target blues --c 20 --as-layer", "Cyan-shift the blues as a layer"),
        adj(
            "adjust channel-mixer",
            "Channel mixer: rebuild a channel from R, G, B",
            run_channel_mixer,
            vec![
                param("out", ParamKind::Choice).choices(&["r", "g", "b"]).default("r").help("Output channel to rebuild (ignored with --monochrome)"),
                param("r", ParamKind::Float).range("-200..200").help("Red contribution in % (default 100 for the output channel, else 0; 40 with --monochrome)"),
                param("g", ParamKind::Float).range("-200..200").help("Green contribution in % (default 100 for the output channel, else 0; 40 with --monochrome)"),
                param("b", ParamKind::Float).range("-200..200").help("Blue contribution in % (default 100 for the output channel, else 0; 20 with --monochrome)"),
                amount("constant", "-200..200", "Constant added to the output, in % of white"),
                param("monochrome", ParamKind::Flag).help("Write the mix to all three channels (grayscale)"),
            ],
        )
        .about("Photoshop Channel Mixer: the output channel becomes r%·R + g%·G + b%·B + constant. Unset weights keep the channel as it was.")
        .example("pixelox adjust channel-mixer photo.jpg -o out.jpg --out r --r 90 --g 10", "Mix a little green into red")
        .example("pixelox adjust channel-mixer photo.jpg -o out.jpg --monochrome --r 50 --g 40 --b 10", "Custom grayscale mix")
        .example("pixelox adjust channel-mixer doc.pxo --in-place --out b --b 110 --constant=-5 --as-layer", "Stronger blue channel as a layer"),
        adj(
            "adjust gradient-map",
            "Map brightness onto a gradient",
            run_gradient_map,
            vec![
                param("stops", ParamKind::Stops).default("#000@0,#fff@1").help("Gradient stops 'color@pos,...' from shadows (0) to highlights (1)"),
                param("interp", ParamKind::Choice)
                    .choices(&["perceptual", "linear", "oklab"])
                    .default("perceptual")
                    .help("Interpolation between stops: perceptual (sRGB values, Photoshop), linear light, or OKLab"),
            ],
        )
        .about("Photoshop Gradient Map: each pixel's luma picks a color from the gradient. Stops with alpha blend over the original.")
        .example("pixelox adjust gradient-map photo.jpg -o out.jpg --stops '#000@0,#ff8800@0.5,#fff@1'", "Warm duotone-like map")
        .example("pixelox adjust gradient-map photo.jpg -o out.jpg --stops '#1e3a8a@0,#fde68a@1' --interp oklab", "Navy-to-cream duotone")
        .example("pixelox adjust gradient-map doc.pxo --in-place --stops '#2b1055@0,#d53369@1' --as-layer", "Gradient map layer"),
        adj(
            "adjust lut",
            "Apply a .cube 3D LUT",
            run_lut,
            vec![
                param("cube", ParamKind::Path).help("The .cube file (3D or 1D; LUT_3D_SIZE, DOMAIN_MIN/MAX honored)"),
                param("strength", ParamKind::Float).default("100").range("0..100").help("Blend between the original (0) and the LUT result (100)"),
                param("interp", ParamKind::Choice)
                    .choices(&["tetrahedral", "trilinear"])
                    .default("tetrahedral")
                    .help("3D interpolation: tetrahedral (smoother, the grading standard) or trilinear"),
                param("cube-text", ParamKind::Str).advanced().help("Inline .cube text (written by --as-layer so the layer needs no file)"),
            ],
        )
        .about("Color lookup with an Adobe/Resolve .cube LUT applied to sRGB-encoded values. Malformed files fail with PX3311 naming the line. With --as-layer the table is embedded in the layer.")
        .example("pixelox adjust lut photo.jpg -o out.jpg --cube film.cube", "Apply a film emulation LUT")
        .example("pixelox adjust lut photo.jpg -o out.jpg --cube teal-orange.cube --strength 60", "Apply a grade at 60%")
        .example("pixelox adjust lut doc.pxo --in-place --cube film.cube --as-layer --name Film", "LUT as an adjustment layer"),
        adj(
            "adjust bw",
            "Black & white with per-color weights",
            run_bw,
            vec![
                param("preset", ParamKind::Choice).choices(hue::BW_PRESET_NAMES).default("default").help("Starting weights (Photoshop-like filter presets)"),
                param("r", ParamKind::Float).range("-200..300").help("Reds weight in % (overrides the preset)"),
                param("y", ParamKind::Float).range("-200..300").help("Yellows weight in % (overrides the preset)"),
                param("g", ParamKind::Float).range("-200..300").help("Greens weight in % (overrides the preset)"),
                param("c", ParamKind::Float).range("-200..300").help("Cyans weight in % (overrides the preset)"),
                param("b", ParamKind::Float).range("-200..300").help("Blues weight in % (overrides the preset)"),
                param("m", ParamKind::Float).range("-200..300").help("Magentas weight in % (overrides the preset)"),
            ],
        )
        .about("Photoshop Black & White: gray = min + (max − mid)·primary weight + (mid − min)·secondary weight, so neutrals keep their value and each of the six color families can be lightened or darkened.")
        .example("pixelox adjust bw photo.jpg -o out.jpg", "Default conversion")
        .example("pixelox adjust bw photo.jpg -o out.jpg --preset red-filter", "Dramatic skies with a red filter")
        .example("pixelox adjust bw photo.jpg -o out.jpg --r 60 --y 80 --b 10", "Custom weights"),
        adj("adjust invert", "Invert colors", run_invert, vec![])
            .about("Photoshop Invert: every color channel becomes 1 − value; alpha is kept.")
            .example("pixelox adjust invert photo.jpg -o out.jpg", "Negative image")
            .example("pixelox adjust invert doc.pxo --in-place --mask @sel", "Invert the selection only")
            .example("pixelox adjust invert doc.pxo --in-place --as-layer", "Invert adjustment layer"),
        adj("adjust equalize", "Equalize the brightness histogram", run_equalize, vec![])
            .about("Photoshop Equalize: remaps values so the luminance histogram is spread evenly between black and white (the same mapping is applied to R, G, and B). As an adjustment layer it equalizes whatever is below it at render time.")
            .example("pixelox adjust equalize photo.jpg -o out.jpg", "Equalize a flat scan")
            .example("pixelox adjust equalize doc.pxo --in-place --layer Background", "Equalize one layer")
            .example("pixelox adjust equalize doc.pxo --in-place --mask sky --invert-mask", "Equalize everything but the sky"),
        adj(
            "adjust posterize",
            "Posterize to N levels per channel",
            run_posterize,
            vec![param("levels", ParamKind::Int).default("4").range("2..255").help("Tone levels per channel")],
        )
        .about("Photoshop Posterize: each channel is cut into --levels equal bands.")
        .example("pixelox adjust posterize photo.jpg -o out.png --levels 4", "Poster look")
        .example("pixelox adjust posterize doc.pxo --in-place --levels 8 --as-layer", "Posterize layer")
        .example("pixelox adjust posterize doc.pxo --in-place --levels 2 --mask subject", "Two-tone subject"),
        adj(
            "adjust threshold",
            "Threshold to black and white",
            run_threshold,
            vec![param("level", ParamKind::Int).default("128").range("1..255").help("Luma (0–255) at and above which pixels turn white")],
        )
        .about("Photoshop Threshold: pixels whose luma reaches --level become white, the rest black.")
        .example("pixelox adjust threshold scan.png -o bw.png", "Binarize a scan")
        .example("pixelox adjust threshold scan.png -o bw.png --level 160", "Higher threshold for faint paper")
        .example("pixelox adjust threshold doc.pxo --in-place --level 100 --as-layer", "Threshold layer"),
        adj(
            "adjust auto",
            "Auto levels, contrast, color, or tone",
            run_auto,
            vec![
                param("levels", ParamKind::Flag).help("Per-channel black and white points (removes casts at the extremes)"),
                param("contrast", ParamKind::Flag).help("One contrast stretch for all channels (keeps colors)"),
                param("color", ParamKind::Flag).help("Map the darkest and lightest colors to black and white and neutralize midtones"),
                param("tone", ParamKind::Flag).help("Contrast stretch plus neutral midtones (the default)"),
                param("black", ParamKind::List).advanced().help("Per-channel black points R,G,B (0–255); set by detection and recorded for replay"),
                param("white", ParamKind::List).advanced().help("Per-channel white points R,G,B (0–255); set by detection and recorded for replay"),
                param("gamma", ParamKind::List).advanced().help("Per-channel gammas R,G,B; set by detection and recorded for replay"),
            ],
        )
        .about("Automatic corrections like Photoshop's Auto options, clipping 0.1% at each end. The detected endpoints and gammas are returned, recorded in the recipe, and embedded in adjustment layers.")
        .example("pixelox adjust auto photo.jpg -o out.jpg", "Auto tone")
        .example("pixelox adjust auto photo.jpg -o out.jpg --color", "Remove a color cast")
        .example("pixelox adjust auto doc.pxo --in-place --levels --as-layer", "Auto levels as a layer"),
    ]
}
