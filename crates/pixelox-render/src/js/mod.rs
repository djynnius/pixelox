//! `render js`, `render d3`, `render three`: scripts run in a QuickJS sandbox (PRD §7.10).

mod canvas;
mod data;
mod engine;

pub use canvas::{image_to_pixmap, pixmap_to_image};

use crate::{base64, count_elements, output_format, rasterize_svg};
use engine::{JsJob, JsOutput, Lib};
use pixelox_core::prelude::*;
use pixelox_core::ParamSpec;
use rand_core::RngCore;
use std::path::{Path, PathBuf};
use std::time::Duration;

const ABOUT_JS: &str = "Runs a script in a QuickJS sandbox (not a browser). The only global is a frozen `px`: \
px.canvas(w, h) returns a Canvas2D context drawn in Rust (rects, paths, arcs, ellipses, beziers, fill/stroke, \
CSS colors, line styles, globalAlpha, transforms, save/restore, linear/radial gradients, clip, drawImage, \
get/putImageData; fillText is a no-op with a warning); px.svg(w, h) returns a DOM-lite SVG document that \
d3-selection can drive; px.data holds --data files (CSV/TSV → row objects, JSON → parsed, else text); \
px.params holds --param values (strings); px.random() is seeded by --seed; px.log() records lines in the \
result; px.output(canvasOrSvg) must be called exactly once. There is no fetch, filesystem, timers, workers, \
WebAssembly, eval, or Function constructor; Date.now() returns 0 and dates are UTC unless --allow-time. \
Limits: --timeout (wall clock, PX6002) and --js-mem (heap and canvases, PX6003). The output format follows \
-o: .svg writes the SVG (a canvas is embedded as PNG), raster extensions rasterize an SVG at its own size or \
at --size when given. Same script, data, params, and seed give identical output.";

const ABOUT_D3: &str = "The `render js` sandbox with a d3 bundle loaded first, so the script sees a global `d3` \
(and a DOM-lite `document` for d3.select/d3.create). This build does not vendor d3: pass a local bundle with \
--lib (UMD such as d3.v7.min.js, or an ES-module build); the bundle's hash is recorded in the sidecar. All \
`render js` flags, limits, and determinism rules apply.";

const ABOUT_THREE: &str = "The `render js` sandbox with a three.js bundle loaded first, so the script sees a \
global `THREE` (and a DOM-lite `document`, enough for SVGRenderer-style code that builds SVG elements). This \
build does not vendor three.js: pass a local build with --lib (three.cjs, or a self-contained three.module.js). \
WebGL is not available; project geometry with THREE's math and draw it with px.canvas() or px.svg(). All \
`render js` flags, limits, and determinism rules apply.";

fn js_params() -> Vec<ParamSpec> {
    vec![
        param("size", ParamKind::Size)
            .default("800x600")
            .help("Default size for px.canvas()/px.svg() called without arguments (px.width, px.height); also the raster size when an SVG result is written to PNG and --size is given"),
        param("data", ParamKind::Str)
            .repeatable()
            .help("name=file: expose a file as px.data.name (.csv/.tsv → array of row objects, .json → parsed, else text); repeatable"),
        param("param", ParamKind::Str).repeatable().help("k=v: expose a string value as px.params.k; repeatable"),
        param("lib", ParamKind::Path)
            .repeatable()
            .help("Local JavaScript library to load before the script (a classic/UMD script, or a self-contained ES module: .mjs, *.module.js, or top-level export); repeatable"),
        param("timeout", ParamKind::Duration).default("30s").help("Wall-clock limit for the whole run (seconds, or 500ms, 2m); exceeding it fails with PX6002"),
        param("js-mem", ParamKind::Int)
            .default("512")
            .range("16..16384")
            .help("Memory limit in MB for the JS heap plus canvases; exceeding it fails with PX6003"),
        param("allow-time", ParamKind::Flag)
            .help("Let Date.now()/new Date() read the real clock (otherwise 0); breaks determinism and warns PXW6001"),
    ]
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::custom("render js", Category::Render, "Run a sandboxed JS script that draws to canvas or SVG", render_js)
            .about(ABOUT_JS)
            .params(js_params())
            .example("pixelox render js art.js -o art.png --size 1200x800 --seed 42", "Generative art on a 1200×800 canvas")
            .example(
                "pixelox render js chart.js -o chart.svg --size 960x540 --data rows=sales.csv --param title=Q3",
                "A data-driven SVG from a CSV file",
            )
            .example(
                "pixelox render js sketch.js -o sketch.png --lib noise.js --timeout 5 --js-mem 256",
                "Load a helper library and tighten the limits",
            ),
        OpSpec::custom("render d3", Category::Render, "Run a d3 script (d3 bundle via --lib) to SVG or PNG", render_d3)
            .about(ABOUT_D3)
            .params(js_params())
            .example(
                "pixelox render d3 chart.js --lib d3.v7.min.js --data rows=sales.csv -o chart.svg --size 960x540 --seed 7",
                "Bar chart from CSV to SVG (PRD example E9)",
            )
            .example(
                "pixelox render d3 chart.js --lib d3.v7.min.js --data rows=sales.csv -o chart.png --size 1920x1080",
                "The same chart rasterized to PNG",
            )
            .example(
                "pixelox render d3 map.js --lib d3.v7.min.js --data world=countries.geojson -o map.svg --param projection=mercator",
                "A map from GeoJSON",
            ),
        OpSpec::custom("render three", Category::Render, "Run a three.js script (bundle via --lib) to SVG/PNG", render_three)
            .about(ABOUT_THREE)
            .params(js_params())
            .example("pixelox render three scene.js --lib three.module.js -o scene.png --size 1024x768", "Raster scene drawn on a px canvas")
            .example("pixelox render three wire.js --lib three.cjs -o wire.svg --param angle=30", "Line-art projection to SVG")
            .example(
                "pixelox render three scene.js --lib three.module.js -o scene.png --seed 3 --timeout 60",
                "Seeded scene with a longer time limit",
            ),
    ]
}

fn render_js(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    run_js_op(a, ctx, None)
}

fn render_d3(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    run_js_op(a, ctx, Some("d3"))
}

fn render_three(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    run_js_op(a, ctx, Some("THREE"))
}

fn split_kv(flag: &str, raw: &str) -> Result<(String, String)> {
    match raw.split_once('=') {
        Some((k, v)) if !k.trim().is_empty() => Ok((k.trim().to_string(), v.to_string())),
        _ => Err(Error::new("PX1010", format!("--{flag} '{raw}' is not name=value"))
            .with_hint(format!("--{flag} name=value"))),
    }
}

fn file_name(p: &Path) -> String {
    p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| p.display().to_string())
}

fn utf8(bytes: Vec<u8>, what: &str, p: &Path) -> Result<String> {
    String::from_utf8(bytes).map_err(|_| {
        Error::new("PX1010", format!("{what} {} is not UTF-8 text", p.display()))
            .with_context("path", p.display().to_string())
    })
}

fn run_js_op(a: &Args, ctx: &mut OpCtx, preset: Option<&'static str>) -> Result<Value> {
    let libs = a.all("lib");
    if let Some(global) = preset {
        if libs.is_empty() {
            let (cmd, file) = if global == "d3" { ("d3", "d3.v7.min.js") } else { ("three", "three.module.js") };
            return Err(Error::new(
                "PX1520",
                format!(
                    "`render {cmd}` needs a local {global} bundle: this build does not vendor {}",
                    if global == "d3" { "d3" } else { "three.js" }
                ),
            )
            .with_hint(format!("pixelox render {cmd} script.js --lib {file} -o out.svg"))
            .with_context("global", global));
        }
    }
    let out = ctx.globals.output.clone().ok_or_else(|| {
        Error::new("PX1004", format!("'{}' writes a file; say where", ctx.command))
            .with_hint("pass -o out.svg or -o out.png")
    })?;
    let out = PathBuf::from(out);
    let script_path = ctx.input(0)?.to_path_buf();
    let script = utf8(ctx.read_aux(&script_path, "source", None)?, "script", &script_path)?;

    let mut data = Map::new();
    for raw in a.all("data") {
        let (name, file) = split_kv("data", &raw)?;
        let p = PathBuf::from(&file);
        let bytes = ctx.read_aux(&p, "data", None)?;
        data.insert(name, data::data_entry(&p, &bytes)?);
    }
    let mut params = Map::new();
    for raw in a.all("param") {
        let (k, v) = split_kv("param", &raw)?;
        params.insert(k, Value::String(v));
    }
    let mut lib_list = Vec::new();
    let single = libs.len() == 1;
    for l in &libs {
        let p = PathBuf::from(l);
        let src = utf8(ctx.read_aux(&p, "lib", if single { Some("lib") } else { None })?, "library", &p)?;
        let name = file_name(&p);
        let module = name.ends_with(".mjs") || name.contains(".module.") || looks_like_module(&src);
        lib_list.push(Lib { name, source: src, module });
    }
    let size_explicit = a.is_explicit("size");
    let (w, h) = a.size("size")?.map(|s| s.resolve(800.0, 600.0, 72.0)).unwrap_or((800, 600));
    let timeout = a.duration("timeout")?.unwrap_or(30.0);
    if !(timeout > 0.0 && timeout.is_finite()) {
        return Err(Error::new("PX1013", "--timeout must be positive").with_context("param", "timeout"));
    }
    let mem_mb = a.int("js-mem")?;
    let mut seed = [0u8; 32];
    ctx.rng().fill_bytes(&mut seed);
    let job = JsJob {
        script,
        script_name: file_name(&script_path),
        libs: lib_list,
        preset,
        data: Value::Object(data),
        params: Value::Object(params),
        width: w,
        height: h,
        timeout: Duration::from_secs_f64(timeout.min(1e7)),
        mem_bytes: (mem_mb as usize) << 20,
        allow_time: a.flag("allow-time"),
        rng_seed: seed,
        pixel_limit: ctx.globals.open.limit_pixels,
    };
    ctx.check_cancel()?;
    let res = engine::run(job)?;
    if res.text_calls > 0 {
        ctx.warn(
            "PXW6002",
            format!("fillText/strokeText were called {} time(s) and drew nothing: the px canvas has no text; use px.svg() <text> elements", res.text_calls),
        );
    }
    if res.time_read {
        ctx.warn("PXW6001", "the script read the clock (--allow-time): output is not deterministic");
    }
    for l in &res.logs {
        ctx.progress.log(l);
    }

    let fmt = output_format(ctx, &out);
    let mut result = json!({
        "output": out.display().to_string(),
        "format": fmt,
        "logs": res.logs,
        "canvases": res.canvases,
    });
    if res.logs_dropped > 0 {
        result["logs_dropped"] = json!(res.logs_dropped);
    }
    if let Some(g) = preset {
        result["preset"] = json!(g);
    }
    match res.output {
        JsOutput::Svg(svg) => {
            result["kind"] = json!("svg");
            result["elements"] = json!(count_elements(&svg));
            if fmt == "svg" {
                ctx.write_file(&out, svg.as_bytes(), None, None)?;
            } else {
                let size = if size_explicit { Some((w, h)) } else { None };
                let pm = rasterize_svg(&svg, size)?;
                let img = pixmap_to_image(&pm);
                result["size"] = json!([img.width, img.height]);
                write_raster(ctx, &out, &img)?;
            }
        }
        JsOutput::Canvas(pm) => {
            result["kind"] = json!("canvas");
            result["size"] = json!([pm.width(), pm.height()]);
            if fmt == "svg" {
                let png = pm.encode_png().map_err(|e| Error::new("PX1107", format!("PNG encode failed: {e}")))?;
                let svg = format!(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\"><image width=\"{w}\" height=\"{h}\" href=\"data:image/png;base64,{}\"/></svg>",
                    base64(&png),
                    w = pm.width(),
                    h = pm.height()
                );
                ctx.write_file(&out, svg.as_bytes(), None, None)?;
            } else {
                write_raster(ctx, &out, &pixmap_to_image(&pm))?;
            }
        }
    }
    Ok(result)
}

/// An ES module has top-level `import ... from` or `export` statements.
fn looks_like_module(src: &str) -> bool {
    src.lines().any(|l| {
        let t = l.trim_start();
        (t.starts_with("export ") || t.starts_with("export{")) || (t.starts_with("import ") && t.contains(" from "))
    })
}

fn write_raster(ctx: &mut OpCtx, out: &Path, img: &Image) -> Result<()> {
    let mut opts = EncodeOptions::for_path(out);
    if let Some(f) = &ctx.globals.format {
        opts.format = f.clone();
    }
    ctx.write_image(out, img, None, &opts, None, None)
}
