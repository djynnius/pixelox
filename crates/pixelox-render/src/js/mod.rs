//! `render js`, `render d3`, `render three`: scripts run in a QuickJS sandbox (PRD §7.10).

pub mod bundled;
mod canvas;
mod data;
mod engine;

pub use canvas::{image_to_pixmap, pixmap_to_image};

use crate::{base64, count_elements, output_format, rasterize_svg_fonts};
use bundled::Library;
use engine::{BundledLib, JsJob, JsOutput, Lib};
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
result; px.atlas(name) returns a bundled map as TopoJSON (world-110m, world-50m, land-110m, us-states, \
us-states-albers, us-nation; `pixelox docs atlas`); px.output(canvasOrSvg) must be called exactly once. --with \
d3,three,topojson loads bundled libraries (none by default in render js). There is no fetch, filesystem, timers, \
workers, WebAssembly, eval, or Function constructor; Date.now() returns 0 and dates are UTC unless --allow-time. \
Limits: --timeout (wall clock, PX6002) and --js-mem (heap and canvases, PX6003). The output format follows \
-o: .svg writes the SVG (a canvas is embedded as PNG), raster extensions rasterize an SVG at its own size or \
at --size when given (<text> uses installed fonts). Same script, data, params, and seed give identical output.";

const ABOUT_D3: &str = "The `render js` sandbox with the bundled d3 7.9.0 and topojson-client 3.1.0 loaded \
first: globals `d3` and `topojson`, plus a DOM-lite `document` for d3.select/d3.create. Scales, axes, shapes \
(line, area, arc, pie, stack), hierarchy layouts (treemap, pack, tree, partition), d3-geo projections, and \
formats all work; build the chart in px.svg() and px.output() it. Maps: px.atlas('us-states' | 'world-110m' | \
...) returns TopoJSON, topojson.feature(us, us.objects.states) gives GeoJSON for d3.geoPath, and features carry \
join codes (postal and fips on states, iso_a2 and iso_a3 on countries; `pixelox docs atlas` lists every name and \
code). Timers never fire: run force layouts synchronously (simulation.stop(); simulation.tick(300)) and skip \
transitions. --lib alone replaces the bundled d3 (PXW1550); with --with d3 a UMD plugin such as d3-sankey extends \
it. Library versions are pinned in the recipe (with: d3@7.9.0) and recorded with their SHA-256 in the sidecar \
(ADR 0018). All `render js` flags, limits, and determinism rules apply: Math.random and d3.random* draw from \
--seed.";

const ABOUT_THREE: &str = "The `render js` sandbox with the bundled three.js r187 loaded first (the core plus \
THREE.SVGRenderer, THREE.Projector, and THREE.RoundedBoxGeometry) and topojson-client: globals `THREE` and \
`topojson`, plus a DOM-lite `document`. WebGL is not available. For a shaded scene, render with new \
THREE.SVGRenderer() (flat-shaded polygons painted back to front; MeshBasic, Lambert, and Phong materials; \
ambient, directional, and point lights) and px.output(renderer.domElement); for line art or custom drawing, \
project points with vector.project(camera) and draw with px.canvas() or px.svg(). --lib alone replaces the bundled \
three (a self-contained build); with --with three an ES-module addon that imports from 'three' joins THREE. \
Versions are pinned in the recipe and recorded in the sidecar (ADR 0018). All `render js` flags, limits, and \
determinism rules apply.";

fn js_params() -> Vec<ParamSpec> {
    vec![
        param("size", ParamKind::Size)
            .default("800x600")
            .help("Default size for px.canvas()/px.svg() called without arguments (px.width, px.height); also the raster size when an SVG result is written to PNG and --size is given"),
        param("data", ParamKind::Str)
            .repeatable()
            .help("name=file: expose a file as px.data.name (.csv/.tsv → array of row objects, .json → parsed, else text); repeatable"),
        param("param", ParamKind::Str).repeatable().help("k=v: expose a string value as px.params.k; repeatable"),
        param("with", ParamKind::List)
            .help("Bundled libraries to load first: d3, three, topojson (comma-separated, each optionally name@version), or none. Default: d3,topojson for render d3, three,topojson for render three, none for render js; a --lib file replaces the bundled d3/three unless --with names it. The versions that ran are pinned in the recipe (ADR 0018)"),
        param("lib", ParamKind::Path)
            .repeatable()
            .help("Local JavaScript library to load after the bundled ones and before the script (a classic/UMD script, or a self-contained ES module: .mjs, *.module.js, or top-level export); repeatable"),
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
        OpSpec::custom(
            "render js",
            Category::Render,
            "Run a sandboxed JS script that draws to canvas or SVG",
            render_js,
        )
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
        )
        .example(
            "pixelox render js map.js --with d3,topojson -o map.svg --size 960x600",
            "Opt into the bundled d3 and topojson (px.atlas maps)",
        ),
        OpSpec::custom("render d3", Category::Render, "Run a d3 script (d3 7.9 bundled) to SVG or PNG", render_d3)
            .about(ABOUT_D3)
            .params(js_params())
            .example(
                "pixelox render d3 chart.js --data rows=sales.csv -o chart.svg --size 960x540 --seed 7",
                "Bar chart from CSV to SVG (PRD example E9)",
            )
            .example(
                "pixelox render d3 chart.js --data rows=sales.csv -o chart.png --size 1920x1080",
                "The same chart rasterized to PNG",
            )
            .example(
                "pixelox render d3 choropleth.js --data rates=rates.csv -o map.png --size 1950x1220",
                "A U.S. choropleth from px.atlas('us-states')",
            )
            .example(
                "pixelox render d3 flows.js --with d3,topojson --lib d3-sankey.min.js -o flows.svg",
                "Extend the bundled d3 with a UMD plugin",
            )
            .example(
                "pixelox render d3 chart.js --lib d3.v7.min.js --data rows=sales.csv -o chart.svg",
                "Use your own d3 build instead of the bundled one",
            ),
        OpSpec::custom(
            "render three",
            Category::Render,
            "Run a three.js script (r187 bundled) to SVG or PNG",
            render_three,
        )
        .about(ABOUT_THREE)
        .params(js_params())
        .example(
            "pixelox render three bars.js --data rows=sales.csv -o bars.png --size 1200x800",
            "A shaded 3D bar chart (THREE.SVGRenderer)",
        )
        .example("pixelox render three wire.js -o wire.svg --param angle=30", "Line-art projection to SVG")
        .example(
            "pixelox render three scene.js -o scene.png --seed 3 --timeout 60",
            "Seeded scene with a longer time limit",
        )
        .example(
            "pixelox render three scene.js --lib three.module.js -o scene.png --size 1024x768",
            "Your own self-contained three.js build instead of the bundled one",
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

/// The bundled libraries a run loads: `--with` when given, else the command's default
/// (`render d3`: d3 and topojson; `render three`: three and topojson; `render js`: none), where
/// a `--lib` file replaces the bundled d3/three.
fn libraries(a: &Args, ctx: &mut OpCtx, preset: Option<&'static str>, has_lib: bool) -> Result<Vec<&'static Library>> {
    if a.is_explicit("with") {
        return bundled::parse_with(&a.str("with").unwrap_or_default());
    }
    let main = match preset {
        Some("d3") => "d3",
        Some(_) => "three",
        None => return Ok(Vec::new()),
    };
    let lib = Library::find(main).expect("bundled");
    if has_lib {
        ctx.warn(
            "PXW1550",
            format!(
                "--lib replaces the bundled {} {}; to extend the bundled one with a plugin instead, pass --with {main},topojson",
                lib.name,
                lib.version()
            ),
        );
        return bundled::parse_with("topojson");
    }
    bundled::parse_with(&format!("{main},topojson"))
}

fn run_js_op(a: &Args, ctx: &mut OpCtx, preset: Option<&'static str>) -> Result<Value> {
    let libs = a.all("lib");
    let with = libraries(a, ctx, preset, !libs.is_empty())?;
    let mut bundled_libs = Vec::new();
    let mut lib_records = Vec::new();
    for lib in &with {
        let mut files = Vec::new();
        for (path, module) in lib.files {
            let source = bundled::text(path)?;
            let f = bundled::info(path).expect("in the manifest");
            ctx.bundled_used(bundled::record("library", lib.name, f));
            files.push((format!("bundled:{path}"), source, *module));
        }
        let main = lib.main();
        lib_records.push(json!({
            "name": lib.name,
            "global": lib.global,
            "package": main.package,
            "version": main.version,
            "license": main.license,
            "sha256": main.sha256,
        }));
        bundled_libs.push(BundledLib { global: lib.global, files });
    }
    // pin what ran, so a replay loads the same versions or stops with PX1552
    if a.is_explicit("with") || !with.is_empty() {
        let pinned: Vec<String> = with.iter().map(|l| l.pinned()).collect();
        ctx.resolve_param("with", if pinned.is_empty() { "none".to_string() } else { pinned.join(",") });
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
    if !size_explicit {
        // the recipe must not turn the default into an explicit --size: that would rasterize
        // an SVG result at 800x600 on replay instead of at its own size (null = not given)
        ctx.resolve_param("size", Value::Null);
    }
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
        bundled: bundled_libs,
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
    if !lib_records.is_empty() {
        result["libraries"] = json!(lib_records);
    }
    if res.timers > 0 {
        // d3 schedules a timer per force simulation or transition; they never run
        result["timers_ignored"] = json!(res.timers);
    }
    if !res.atlases.is_empty() {
        let mut used = Vec::new();
        for name in &res.atlases {
            let atlas = bundled::Atlas::find(name).expect("the engine checked the name");
            let rec = bundled::record("atlas", atlas.name, atlas.file_info());
            ctx.bundled_used(rec.clone());
            if let Some(table) = atlas.codes() {
                ctx.bundled_used(bundled::record("codes", atlas.name, table));
            }
            used.push(rec);
        }
        result["atlases"] = json!(used);
    }
    match res.output {
        JsOutput::Svg(svg) => {
            result["kind"] = json!("svg");
            result["elements"] = json!(count_elements(&svg));
            if fmt == "svg" {
                ctx.write_file(&out, svg.as_bytes(), None, None)?;
            } else {
                let size = if size_explicit { Some((w, h)) } else { None };
                let (pm, fonts) = rasterize_svg_fonts(&svg, size)?;
                for f in fonts {
                    ctx.font_used(f);
                }
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
