//! `render 3d`: the native 3D-lite renderer (PRD §7.11). Mockup and diagram grade, not CAD.

mod extrude;
mod mesh;
mod render;

use crate::js::image_to_pixmap;
use crate::{js::pixmap_to_image, output_format};
use mesh::{Mesh, V3};
use pixelox_core::ctx::is_pxo;
use pixelox_core::doc::RecipeStep;
use pixelox_core::prelude::*;
use render::{fmt_num, polylines, Camera, Light, Scene, Style};
use std::path::{Path, PathBuf};
use tiny_skia::{LineCap, LineJoin, Paint, PathBuilder, Pixmap, PixmapPaint, Stroke, Transform};

const ABOUT: &str = "Native CPU 3D-lite (no GPU): one source of geometry, a camera, and a style. Geometry: \
--primitive (box, sphere, cylinder, cone, torus, plane) sized by --dims X,Y,Z (box extents; sphere diameters; \
cylinder/cone base X×Z and height Y; torus outer diameter X/Z and tube diameter Y; plane X×Z); --extrude an \
SVG's path outlines (holes kept) by --depth SVG units; or --scene file.p3d.json with \
{\"objects\":[{\"type\":\"box\",\"dims\":[2,1,1],\"position\":[0,0,0],\"rotation\":[0,30,0],\"scale\":1,\
\"color\":\"#f97316\"},{\"type\":\"obj\",\"path\":\"mesh.obj\"},{\"type\":\"extrude\",\"path\":\"logo.svg\",\
\"depth\":40}],\"camera\":{\"preset\":\"persp\",\"position\":[4,3,5],\"look_at\":[0,0,0],\"fov\":40},\
\"light\":{\"direction\":[0.3,1,0.6],\"ambient\":0.3,\"intensity\":0.75}} (paths relative to the scene file). \
Note: the PRD's `--size 2,1,1` for primitive dimensions is spelled --dims here; --size is always the output \
size. Cameras: iso, dimetric, front, top (orthographic) and persp (--fov, --look-at); the model is always \
fitted to the canvas. Styles: wireframe draws every modelled edge; hidden-line draws the visible parts of \
creases, silhouettes, and open boundaries (back-face culling plus a z-buffer test, edges split where their \
visibility changes; --edges all keeps every visible mesh edge, e.g. a sphere's grid); shaded is flat Lambert + ambient with 2×2 supersampling (painter's algorithm in SVG), \
with --outline adding the hidden-line edges. -o .svg writes vector paths; raster extensions rasterize. \
--as-layer DOC adds the render as a raster layer to a .pxo document instead (--in-place, or -o).";

const PRIMS: &[&str] = &["box", "sphere", "cylinder", "cone", "torus", "plane"];

pub fn ops() -> Vec<OpSpec> {
    vec![OpSpec::custom("render 3d", Category::Render, "Render primitives, extrusions, or OBJ scenes (CPU 3D)", render_3d)
        .inputs(Inputs::NONE)
        .about(ABOUT)
        .param(param("scene", ParamKind::Path).help("Scene file (.p3d.json): objects (primitives, obj meshes, SVG extrusions), camera, light"))
        .param(param("primitive", ParamKind::Choice).choices(PRIMS).help("Render one primitive: box, sphere, cylinder, cone, torus, or plane"))
        .param(param("dims", ParamKind::Str).help("Primitive dimensions X,Y,Z (default 1,1,1; torus 1,0.3,1); see `about` for each shape"))
        .param(param("extrude", ParamKind::Path).help("SVG whose path outlines are extruded (filled paths; holes kept)"))
        .param(param("depth", ParamKind::Float).auto().default("auto").range("0..1000000").help("Extrusion depth in SVG units (auto: 20% of the larger side)"))
        .param(
            param("camera", ParamKind::Choice)
                .choices(&["iso", "dimetric", "front", "top", "persp"])
                .default("iso")
                .help("Camera: iso, dimetric, front, top (orthographic) or persp (perspective)"),
        )
        .param(param("fov", ParamKind::Float).default("40").range("1..170").help("Vertical field of view in degrees (persp)"))
        .param(param("look-at", ParamKind::Str).help("Point X,Y,Z the camera aims at (default: the model's center)"))
        .param(
            param("style", ParamKind::Choice)
                .choices(&["wireframe", "hidden-line", "shaded"])
                .default("hidden-line")
                .help("wireframe (all edges), hidden-line (visible edges only), or shaded (flat Lambert)"),
        )
        .param(
            param("edges", ParamKind::Choice)
                .choices(&["feature", "all"])
                .default("feature")
                .help("hidden-line/outline edges: feature (creases, silhouettes, boundaries) or all (every visible mesh edge)"),
        )
        .param(param("line-width", ParamKind::Float).default("1.5").range("0.05..200").help("Line width in output pixels"))
        .param(param("size", ParamKind::Size).default("1024x768").help("Output size WxH (with --as-layer: the document size unless given)"))
        .param(param("as-layer", ParamKind::Path).help("Add the render as a raster layer to this .pxo document (--in-place, or -o for a copy)"))
        .param(common::name().help("Name of the layer added with --as-layer (default: 3D render)"))
        .param(param("color", ParamKind::Color).default("#8fa6c8").help("Surface color of the model (scene objects may set their own)"))
        .param(param("line-color", ParamKind::Color).default("#1f2937").help("Line color for wireframe, hidden-line, and --outline"))
        .param(param("bg-color", ParamKind::Color).default("transparent").help("Background color (transparent by default; a scene file may set `background`)"))
        .param(param("outline", ParamKind::Flag).help("With --style shaded: draw the hidden-line edges on top"))
        .param(param("segments", ParamKind::Int).default("32").range("3..512").advanced().help("Tessellation segments around curved primitives"))
        .example("pixelox render 3d --primitive box --dims 2,1,1 --camera iso --style hidden-line -o box.svg", "Isometric hidden-line box")
        .example(
            "pixelox render 3d --extrude logo.svg --depth 40 --camera iso --style hidden-line --line-width 1.5 -o logo-wire.svg",
            "Extruded logo wireframe (PRD example E10)",
        )
        .example(
            "pixelox render 3d --scene scene.p3d.json --camera persp --fov 35 --style shaded --outline -o scene.png --size 1600x1200",
            "Shaded perspective render of a scene file",
        )
        .example(
            "pixelox render 3d --extrude logo.svg --depth 40 --camera iso --style hidden-line --as-layer deck.pxo --in-place",
            "Render straight into a document as a layer",
        )]
}

fn bad(msg: impl Into<String>) -> Error {
    Error::new("PX1010", msg)
}

fn vec3(v: &Value, key: &str) -> Result<V3> {
    match v {
        Value::Array(a) if a.len() == 3 => {
            let mut out = [0.0; 3];
            for (i, x) in a.iter().enumerate() {
                out[i] = x
                    .as_f64()
                    .filter(|f| f.is_finite())
                    .ok_or_else(|| bad(format!("scene: `{key}` must hold numbers")))?;
            }
            Ok(out)
        }
        Value::Number(n) => {
            let f = n.as_f64().unwrap_or(0.0);
            Ok([f, f, f])
        }
        _ => Err(bad(format!("scene: `{key}` must be [x, y, z]"))),
    }
}

fn parse_xyz(s: &str, flag: &str) -> Result<V3> {
    let parts: Vec<f64> =
        s.split(',').map(|p| p.trim().parse::<f64>()).collect::<std::result::Result<_, _>>().map_err(|_| {
            Error::new("PX1010", format!("--{flag} '{s}' is not X,Y,Z")).with_hint(format!("--{flag} 1,2,3"))
        })?;
    if parts.len() != 3 || parts.iter().any(|v| !v.is_finite()) {
        return Err(Error::new("PX1010", format!("--{flag} '{s}' needs three numbers X,Y,Z"))
            .with_hint(format!("--{flag} 1,2,3")));
    }
    Ok([parts[0], parts[1], parts[2]])
}

fn rotate(v: V3, r: V3) -> V3 {
    let (sx, cx) = r[0].to_radians().sin_cos();
    let (sy, cy) = r[1].to_radians().sin_cos();
    let (sz, cz) = r[2].to_radians().sin_cos();
    let v = [v[0], v[1] * cx - v[2] * sx, v[1] * sx + v[2] * cx];
    let v = [v[0] * cy + v[2] * sy, v[1], -v[0] * sy + v[2] * cy];
    [v[0] * cz - v[1] * sz, v[0] * sz + v[1] * cz, v[2]]
}

fn finish_object(mut m: Mesh, color: Rgba) -> Mesh {
    m.weld();
    m.fix_inside_out();
    m.set_color(color);
    m
}

fn load_extrusion(ctx: &mut OpCtx, path: &Path, depth: Option<f64>, param_name: Option<&str>) -> Result<(Mesh, f64)> {
    let bytes = ctx.read_aux(path, "profile", param_name)?;
    let contours = extrude::svg_contours(&bytes, &path.display().to_string())?;
    Ok(extrude::extrude(contours, depth))
}

struct Built {
    mesh: Mesh,
    camera: Option<Camera>,
    light: Light,
    background: Option<Rgba>,
    objects: usize,
    depth: Option<f64>,
}

fn build_scene(ctx: &mut OpCtx, path: &Path, default_color: Rgba, segments: usize) -> Result<Built> {
    let bytes = ctx.read_aux(path, "scene", Some("scene"))?;
    let v: Value = serde_json::from_slice(&bytes).map_err(|e| bad(format!("{}: invalid JSON: {e}", path.display())))?;
    let base = path.parent().map(Path::to_path_buf).unwrap_or_default();
    let objs = v.get("objects").and_then(Value::as_array).ok_or_else(|| {
        bad(format!("{}: needs an `objects` array", path.display()))
            .with_hint(r#"{"objects":[{"type":"box","dims":[2,1,1]}]}"#)
    })?;
    if objs.is_empty() {
        return Err(bad(format!("{}: `objects` is empty", path.display())));
    }
    let mut mesh = Mesh::default();
    for (i, o) in objs.iter().enumerate() {
        let ty = o
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| bad(format!("scene object {}: missing `type`", i + 1)))?;
        let color = match o.get("color").and_then(Value::as_str) {
            Some(c) => color::parse_color(c)?,
            None => default_color,
        };
        let seg = o.get("segments").and_then(Value::as_u64).map(|s| s.clamp(3, 512) as usize).unwrap_or(segments);
        let file = || -> Result<PathBuf> {
            let p = o
                .get("path")
                .and_then(Value::as_str)
                .ok_or_else(|| bad(format!("scene object {} ({ty}): missing `path`", i + 1)))?;
            let p = PathBuf::from(p);
            Ok(if p.is_absolute() { p } else { base.join(p) })
        };
        let m = match ty {
            "obj" => {
                let p = file()?;
                let b = ctx.read_aux(&p, "mesh", None)?;
                let text = String::from_utf8_lossy(&b);
                mesh::parse_obj(&text, &p.display().to_string())?
            }
            "extrude" | "svg" => {
                let p = file()?;
                let depth = o.get("depth").and_then(Value::as_f64);
                load_extrusion(ctx, &p, depth, None)?.0
            }
            prim if PRIMS.contains(&prim) => {
                let dims = match o.get("dims").or_else(|| o.get("size")) {
                    Some(d) => vec3(d, "dims")?,
                    None => mesh::default_dims(prim),
                };
                mesh::primitive(prim, dims, seg)?
            }
            other => {
                return Err(bad(format!("scene object {}: unknown type '{other}'", i + 1))
                    .with_hint("types: box, sphere, cylinder, cone, torus, plane, obj, extrude"))
            }
        };
        let mut m = finish_object(m, color);
        let scale = match o.get("scale") {
            Some(s) => vec3(s, "scale")?,
            None => [1.0; 3],
        };
        let rot = match o.get("rotation") {
            Some(Value::Number(n)) => [0.0, n.as_f64().unwrap_or(0.0), 0.0],
            Some(r) => vec3(r, "rotation")?,
            None => [0.0; 3],
        };
        let pos = match o.get("position") {
            Some(p) => vec3(p, "position")?,
            None => [0.0; 3],
        };
        m.map(|p| {
            let p = rotate([p[0] * scale[0], p[1] * scale[1], p[2] * scale[2]], rot);
            [p[0] + pos[0], p[1] + pos[1], p[2] + pos[2]]
        });
        if scale.iter().filter(|s| **s < 0.0).count() % 2 == 1 {
            m.fix_inside_out();
        }
        mesh.append(m);
    }
    let camera = match v.get("camera") {
        Some(c) => {
            let name = c
                .get("preset")
                .or_else(|| c.get("type"))
                .and_then(Value::as_str)
                .unwrap_or(if c.get("position").is_some() { "persp" } else { "iso" });
            let mut cam = Camera::preset(name);
            if let Some(p) = c.get("position") {
                cam.eye = Some(vec3(p, "camera.position")?);
            }
            if let Some(t) = c.get("look_at").or_else(|| c.get("target")) {
                cam.target = Some(vec3(t, "camera.look_at")?);
            }
            if let Some(f) = c.get("fov").and_then(Value::as_f64) {
                cam.fov_deg = f;
            }
            if let Some(u) = c.get("up") {
                cam.up = vec3(u, "camera.up")?;
            }
            Some(cam)
        }
        None => None,
    };
    let mut light = Light::default();
    if let Some(l) = v.get("light") {
        if let Some(d) = l.get("direction") {
            light.dir = Some(vec3(d, "light.direction")?);
        }
        if let Some(a) = l.get("ambient").and_then(Value::as_f64) {
            light.ambient = a.clamp(0.0, 1.0);
        }
        if let Some(i) = l.get("intensity").and_then(Value::as_f64) {
            light.intensity = i.clamp(0.0, 4.0);
        }
    }
    let background = match v.get("background").and_then(Value::as_str) {
        Some(b) => Some(color::parse_color(b)?),
        None => None,
    };
    Ok(Built { mesh, camera, light, background, objects: objs.len(), depth: None })
}

fn render_3d(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let sources = ["scene", "primitive", "extrude"].iter().filter(|k| a.has(k)).count();
    if sources == 0 {
        return Err(Error::new("PX1003", "'render 3d' needs one of --scene, --primitive, or --extrude")
            .with_hint("pixelox render 3d --primitive box --dims 2,1,1 -o box.svg"));
    }
    if sources > 1 {
        return Err(Error::new("PX1005", "pass only one of --scene, --primitive, or --extrude"));
    }
    if a.has("dims") && !a.has("primitive") {
        return Err(Error::new("PX1005", "--dims applies to --primitive; scene objects set `dims` in the scene file"));
    }
    let color = a.color("color")?;
    let segments = a.usize("segments")?;
    let mut built = if let Some(p) = a.path("scene") {
        build_scene(ctx, &p, color, segments)?
    } else if let Some(kind) = a.str("primitive") {
        let dims = match a.str("dims") {
            Some(d) => parse_xyz(&d, "dims")?,
            None => mesh::default_dims(&kind),
        };
        if dims.iter().any(|d| *d < 0.0) {
            return Err(Error::new("PX1013", "--dims must not be negative").with_context("param", "dims"));
        }
        let m = finish_object(mesh::primitive(&kind, dims, segments)?, color);
        Built { mesh: m, camera: None, light: Light::default(), background: None, objects: 1, depth: None }
    } else {
        let p = a.path("extrude").unwrap_or_default();
        let (m, depth) = load_extrusion(ctx, &p, a.opt_f64("depth")?, Some("extrude"))?;
        if a.is_auto("depth") {
            ctx.resolve_param("depth", depth);
        }
        Built {
            mesh: finish_object(m, color),
            camera: None,
            light: Light::default(),
            background: None,
            objects: 1,
            depth: Some(depth),
        }
    };
    if built.mesh.faces.is_empty() {
        return Err(Error::new("PX3320", "the scene has no faces to render"));
    }

    // camera: flags override the scene file when given explicitly
    let mut cam = match (&built.camera, a.is_explicit("camera")) {
        (Some(c), false) => c.clone(),
        _ => Camera::preset(&a.choice("camera")),
    };
    if a.is_explicit("fov") || built.camera.is_none() {
        cam.fov_deg = a.f64("fov")?;
    }
    if let Some(l) = a.str("look-at") {
        cam.target = Some(parse_xyz(&l, "look-at")?);
    }
    let style = match a.choice("style").as_str() {
        "wireframe" => Style::Wireframe,
        "shaded" => Style::Shaded,
        _ => Style::HiddenLine,
    };
    let line_width = a.f64("line-width")?;
    let line_color = a.color("line-color")?;
    let background = if a.is_explicit("bg-color") { Some(a.color("bg-color")?) } else { built.background.take() }
        .filter(|c| c.a > 0.0);

    // output target
    let as_layer = a.path("as-layer");
    let out = ctx.globals.output.clone().map(PathBuf::from);
    if ctx.globals.in_place && as_layer.is_none() {
        return Err(
            Error::new("PX1402", "--in-place needs --as-layer DOC.pxo").with_hint("write a new file with -o instead")
        );
    }
    if as_layer.is_none() && out.is_none() {
        return Err(Error::new("PX1004", "'render 3d' writes a file; say where")
            .with_hint("pass -o out.svg or -o out.png, or --as-layer doc.pxo --in-place"));
    }
    let mut doc = match &as_layer {
        Some(p) => {
            if ctx.globals.in_place && out.is_some() {
                return Err(Error::new("PX1005", "pass either -o or --in-place, not both"));
            }
            if ctx.globals.in_place && !is_pxo(p) {
                return Err(Error::new("PX1402", format!("--in-place needs a .pxo document, not {}", p.display())));
            }
            if out.is_none() && !ctx.globals.in_place {
                return Err(Error::new("PX1004", "--as-layer needs --in-place or -o")
                    .with_hint(format!("pixelox render 3d ... --as-layer {} --in-place", p.display())));
            }
            let d = ctx.load_document(p)?;
            ctx.inputs.push(p.clone());
            Some(d)
        }
        None => None,
    };
    let (w, h) = match (&doc, a.is_explicit("size")) {
        (Some(d), false) => (d.width, d.height),
        _ => a.size("size")?.map(|s| s.resolve(1024.0, 768.0, 72.0)).unwrap_or((1024, 768)),
    };
    let all_edges = a.choice("edges") == "all";
    let margin = (line_width * 2.0).max(0.04 * w.min(h) as f64);
    let scene = Scene::new(&built.mesh, &cam, w, h, margin);
    let segs = match style {
        Style::Wireframe => scene.wireframe(),
        Style::HiddenLine => scene.hidden_line(all_edges),
        Style::Shaded if a.flag("outline") => scene.hidden_line(all_edges),
        Style::Shaded => Vec::new(),
    };
    ctx.check_cancel()?;

    let mut result = json!({
        "style": a.choice("style"),
        "camera": if cam.persp { "persp".to_string() } else { a.choice("camera") },
        "size": [w, h],
        "objects": built.objects,
        "vertices": built.mesh.verts.len(),
        "faces": built.mesh.faces.len(),
        "triangles": scene.tris.len(),
        "edges": scene.edges.len(),
        "segments": segs.len(),
    });
    if let Some(d) = built.depth {
        result["depth"] = json!(d);
    }
    let raster = |scene: &Scene| -> Result<Image> {
        let mut pm =
            Pixmap::new(w, h).ok_or_else(|| Error::new("PX1104", format!("cannot allocate a {w}x{h} raster")))?;
        if let Some(bg) = background {
            pm.fill(tiny_skia::Color::from_rgba(bg.r, bg.g, bg.b, bg.a).unwrap_or(tiny_skia::Color::TRANSPARENT));
        }
        if style == Style::Shaded {
            let img = scene.shaded(&built.light);
            if let Some(layer) = image_to_pixmap(&img) {
                pm.draw_pixmap(0, 0, layer.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
            }
        }
        stroke_segments(&mut pm, &segs, line_width, line_color);
        Ok(pixmap_to_image(&pm))
    };

    if let (Some(mut d), Some(p)) = (doc.take(), as_layer) {
        let img = raster(&scene)?;
        let name = a.str("name").unwrap_or_else(|| "3D render".into());
        let layer = d.new_layer(&name, LayerKind::Raster { image: img });
        result["layer"] = json!(layer.name);
        d.insert_above(None, layer);
        let mut params = a.to_json();
        for (k, v) in &ctx.resolved {
            params.insert(k.clone(), v.clone());
        }
        d.recipe.push(RecipeStep {
            op: "render.3d".into(),
            params,
            inputs: ctx.step_inputs.clone(),
            model: None,
            fonts: vec![],
        });
        let target = if ctx.globals.in_place { p } else { out.clone().unwrap_or(p) };
        result["output"] = json!(target.display().to_string());
        result["document"] = d.summary();
        pixelox_core::exec::write_document(ctx, &d, &target)?;
        return Ok(result);
    }

    let out = out.unwrap_or_default();
    let fmt = output_format(ctx, &out);
    result["output"] = json!(out.display().to_string());
    result["format"] = json!(fmt);
    if fmt == "svg" {
        let svg = svg_doc(&scene, style, &segs, w, h, line_width, line_color, background, &built.light);
        ctx.write_file(&out, svg.as_bytes(), None, None)?;
    } else {
        let img = raster(&scene)?;
        let mut opts = EncodeOptions::for_path(&out);
        if let Some(f) = &ctx.globals.format {
            opts.format = f.clone();
        }
        ctx.write_image(&out, &img, None, &opts, None, None)?;
    }
    Ok(result)
}

fn stroke_segments(pm: &mut Pixmap, segs: &[[f64; 4]], width: f64, color: Rgba) {
    if segs.is_empty() {
        return;
    }
    let mut pb = PathBuilder::new();
    for pl in polylines(segs) {
        pb.move_to(pl[0][0] as f32, pl[0][1] as f32);
        for p in &pl[1..] {
            pb.line_to(p[0] as f32, p[1] as f32);
        }
    }
    let Some(path) = pb.finish() else { return };
    let mut paint = Paint { anti_alias: true, ..Default::default() };
    paint.set_color(tiny_skia::Color::from_rgba(color.r, color.g, color.b, color.a).unwrap_or(tiny_skia::Color::BLACK));
    let stroke =
        Stroke { width: width as f32, line_cap: LineCap::Round, line_join: LineJoin::Round, ..Default::default() };
    pm.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
}

fn path_data(segs: &[[f64; 4]]) -> String {
    let mut d = String::new();
    for pl in polylines(segs) {
        d.push_str(&format!("M{} {}", fmt_num(pl[0][0]), fmt_num(pl[0][1])));
        for p in &pl[1..] {
            d.push_str(&format!("L{} {}", fmt_num(p[0]), fmt_num(p[1])));
        }
    }
    d
}

fn svg_doc(
    scene: &Scene,
    style: Style,
    segs: &[[f64; 4]],
    w: u32,
    h: u32,
    lw: f64,
    lc: Rgba,
    bg: Option<Rgba>,
    light: &Light,
) -> String {
    let paint = |c: Rgba| -> String {
        let hex = c.with_alpha(1.0).to_hex();
        if c.a < 1.0 {
            format!("{hex}\" fill-opacity=\"{}", fmt_num(c.a as f64))
        } else {
            hex
        }
    };
    let mut s =
        format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\">");
    if let Some(b) = bg {
        s.push_str(&format!("<rect width=\"100%\" height=\"100%\" fill=\"{}\"/>", paint(b)));
    }
    if style == Style::Shaded {
        s.push_str("<g stroke-linejoin=\"round\" stroke-width=\"0.6\">");
        for (loops, c) in scene.painter_faces(light) {
            let mut d = String::new();
            for l in loops.iter().filter(|l| l.len() >= 3) {
                d.push_str(&format!("M{} {}", fmt_num(l[0][0]), fmt_num(l[0][1])));
                for p in &l[1..] {
                    d.push_str(&format!("L{} {}", fmt_num(p[0]), fmt_num(p[1])));
                }
                d.push('Z');
            }
            let hex = c.with_alpha(1.0).to_hex();
            s.push_str(&format!("<path d=\"{d}\" fill=\"{hex}\" stroke=\"{hex}\" fill-rule=\"evenodd\"/>"));
        }
        s.push_str("</g>");
    }
    if !segs.is_empty() {
        let stroke = lc.with_alpha(1.0).to_hex();
        let opacity = if lc.a < 1.0 { format!(" stroke-opacity=\"{}\"", fmt_num(lc.a as f64)) } else { String::new() };
        s.push_str(&format!(
            "<path d=\"{}\" fill=\"none\" stroke=\"{stroke}\"{opacity} stroke-width=\"{}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>",
            path_data(segs),
            fmt_num(lw)
        ));
    }
    s.push_str("</svg>");
    s
}
