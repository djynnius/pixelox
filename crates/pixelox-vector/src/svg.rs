//! `svg new|el add|el set|el rm|defs add|use|query|optimize|rasterize` (PRD §8.15): custom
//! ops on standalone SVG files. Each reads the SVG (recorded as the step's `source`),
//! edits the owned DOM ([`crate::xml`]), and writes `-o` or, with `--in-place`, the input.

use crate::spec::{fmt_num, parse_d};
use crate::xml::{local_name, valid_id, ElPath, Selector, WriteOpts, XDoc, XEl, XNode, SVG_NS};
use pixelox_core::prelude::*;
use std::path::{Path, PathBuf};

const KINDS: &[&str] = &["rect", "circle", "ellipse", "line", "polyline", "polygon", "path", "text", "g", "image"];
const DEF_KINDS: &[&str] =
    &["linear-gradient", "radial-gradient", "pattern", "clip-path", "mask", "filter", "symbol", "marker"];

fn attr_param() -> pixelox_core::op::ParamSpec {
    param("attr", ParamKind::Str)
        .repeatable()
        .help("Attribute k=v to set (repeatable), e.g. --attr fill='#0b5fff' --attr stroke-width=2")
}

fn id_or_where() -> Vec<pixelox_core::op::ParamSpec> {
    vec![
        param("id", ParamKind::Str).help("Target the element with this id"),
        param("where", ParamKind::Str)
            .help("Target every element matching a selector: tag, .class, #id, [attr=value], descendant 'g.axis text'"),
    ]
}

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::custom("svg new", Category::Svg, "Create a new empty SVG document", new)
            .inputs(Inputs::NONE)
            .about("Writes an empty `<svg>` of `--size` (user units = px) with a matching viewBox unless `--viewbox` is given.")
            .param(param("size", ParamKind::Size).default("800x600").help("Width x height in px"))
            .param(param("viewbox", ParamKind::Str).help("viewBox 'minX minY width height' (default '0 0 W H')"))
            .example("pixelox svg new -o logo.svg --size 512x512", "A 512 px square canvas")
            .example("pixelox svg new -o chart.svg --size 960x540 --viewbox '0 0 1920 1080'", "Draw in 1080p units, display at 960x540")
            .example("pixelox svg new -o icon.svg --size 24x24 --overwrite", "Replace an existing icon file"),
        OpSpec::custom("svg el add", Category::Svg, "Add an element to an SVG", el_add)
            .about("Appends an element (rect, circle, ellipse, line, polyline, polygon, path, text, g, image) to the root or to `--parent`. Presentation attributes (fill, stroke, transform, stroke-dasharray, ...) go in `--attr k=v`. Ids are generated (`rect-1`) when `--id` is omitted.")
            .param(param("kind", ParamKind::Choice).choices(KINDS).required().help("Element type to add"))
            .param(param("id", ParamKind::Str).help("Element id (default: generated, e.g. rect-1)"))
            .param(param("parent", ParamKind::Str).help("Id of the parent element (default: the root <svg>)"))
            .param(attr_param())
            .param(param("d", ParamKind::Str).help("Path data for --kind path"))
            .param(param("text", ParamKind::Str).help("Text content for --kind text"))
            .example("pixelox svg el add logo.svg --in-place --kind rect --id bg --attr width=512 --attr height=512 --attr fill='#0b5fff'", "A full-size background")
            .example("pixelox svg el add logo.svg --in-place --kind path --id mark --d 'M 64 448 L 256 64 L 448 448 Z' --attr fill=white", "A white triangle path")
            .example("pixelox svg el add logo.svg -o titled.svg --kind text --id title --text 'Hello' --attr x=256 --attr y=480 --attr text-anchor=middle", "Centered text")
            .example("pixelox svg el add logo.svg --in-place --kind circle --id dot --parent layer1 --attr cx=20 --attr cy=20 --attr r=8", "A circle inside a group"),
        OpSpec::custom("svg el set", Category::Svg, "Set attributes or text on SVG elements", el_set)
            .about("Sets attributes (and optionally text) on the element `--id` or on every element matching `--where`. A same-named property inside a `style` attribute is removed so the new value takes effect.")
            .params(id_or_where())
            .param(attr_param())
            .param(param("unset", ParamKind::Str).repeatable().help("Attribute name to remove (repeatable)"))
            .param(param("text", ParamKind::Str).help("Replace the element's text content"))
            .example("pixelox svg el set logo.svg --in-place --where 'path' --attr fill='#0b5fff'", "Recolor every path")
            .example("pixelox svg el set chart.svg --in-place --where 'g.axis text' --attr font-size=14 --attr fill='#334155'", "Restyle axis labels")
            .example("pixelox svg el set logo.svg -o renamed.svg --id title --text 'New title'", "Change a text element's content"),
        OpSpec::custom("svg el rm", Category::Svg, "Remove SVG elements by id or selector", el_rm)
            .about("Removes the element `--id` or every element matching `--where` (with their children). The root cannot be removed.")
            .params(id_or_where())
            .example("pixelox svg el rm logo.svg --in-place --id bg", "Remove one element")
            .example("pixelox svg el rm chart.svg --in-place --where 'g.grid line'", "Remove grid lines")
            .example("pixelox svg el rm logo.svg -o clean.svg --where '[opacity=0]'", "Drop invisible elements"),
        OpSpec::custom("svg defs add", Category::Svg, "Add a gradient, filter, symbol, or other def", defs_add)
            .about("Adds a definition to `<defs>` from a JSON spec. Top-level scalar keys become attributes of the def element; `units` and `content_units` map to the kind's units attributes; gradients take `stops` ('#000@0,#fff@1' or [{\"offset\": 0, \"color\": \"#000\", \"opacity\": 1}]); filters take `primitives` ([{\"type\": \"feGaussianBlur\", \"stdDeviation\": 4}], kebab names like gaussian-blur work too); clip paths, masks, symbols, patterns, and markers take `children` ([{\"tag\": \"circle\", \"r\": 10}]) or a `d` shorthand. Nested objects may have `children` and `text`.")
            .param(param("kind", ParamKind::Choice).choices(DEF_KINDS).required().help("Kind of definition"))
            .param(param("id", ParamKind::Str).required().help("Id to reference it by, e.g. url(#id) or href='#id'"))
            .param(param("spec", ParamKind::Str).required().help("JSON spec file, or an inline JSON object"))
            .example("pixelox svg defs add logo.svg --in-place --kind linear-gradient --id sky --spec sky.json", "A gradient from a spec file")
            .example("pixelox svg defs add logo.svg --in-place --kind filter --id shadow --spec shadow.json", "A filter composed of SVG primitives")
            .example("pixelox svg defs add logo.svg --in-place --kind radial-gradient --id glow --spec '{\"stops\": \"#ffffff@0,#ffffff00@1\"}'", "An inline radial gradient"),
        OpSpec::custom("svg use", Category::Svg, "Instance a symbol or element with <use>", use_)
            .about("Adds `<use href=\"#ref\">` at `--at` (to the root or `--parent`). `--size` sets width/height for symbols with a viewBox.")
            .param(param("ref", ParamKind::Str).required().help("Id of the symbol or element to instance ('#star' or 'star')"))
            .param(param("at", ParamKind::Point).default("0,0").help("Position X,Y in user units"))
            .param(param("id", ParamKind::Str).help("Id of the new <use> (default: generated)"))
            .param(param("parent", ParamKind::Str).help("Id of the parent element (default: the root <svg>)"))
            .param(param("size", ParamKind::Size).help("Width x height of the instance (symbols with a viewBox)"))
            .example("pixelox svg use icons.svg --in-place --ref '#star' --at 100,100", "Place a symbol")
            .example("pixelox svg use icons.svg --in-place --ref star --at 200,100 --id star-2", "Place a second instance with an id")
            .example("pixelox svg use icons.svg -o sheet.svg --ref '#star' --at 0,0 --size 48x48", "Instance at a fixed size"),
        OpSpec::custom("svg query", Category::Svg, "Inspect SVG elements matching a selector", query)
            .read_only()
            .about("Lists matching elements with tag, id, attributes, text, and their computed bounding box (via usvg, in root user units; `null` for elements that do not render, such as defs).")
            .param(param("where", ParamKind::Str).default("*").help("Selector: tag, .class, #id, [attr=value], descendant 'g.axis text'"))
            .example("pixelox svg query logo.svg --where path --json", "Every path with its bbox")
            .example("pixelox svg query chart.svg --where 'g.axis text'", "Axis labels")
            .example("pixelox svg query logo.svg --where '#mark' --json", "One element by id"),
        OpSpec::custom("svg optimize", Category::Svg, "Minify an SVG conservatively", optimize)
            .about("Drops comments, processing instructions, <metadata>, and editor cruft (Inkscape, Sodipodi, Illustrator, Sketch, Serif, Figma namespaces), removes unused namespace declarations and empty groups, shortens numbers in geometry attributes, transforms, and path data to `--precision` decimals, and strips indentation. Ids, styles, and structure are kept.")
            .param(param("precision", ParamKind::Int).default("3").range("0..8").help("Decimal places kept in numbers"))
            .example("pixelox svg optimize logo.svg -o logo.min.svg --precision 2", "Minify a traced logo")
            .example("pixelox svg optimize drawing.svg -o drawing.min.svg", "Minify with the default precision")
            .example("pixelox svg optimize icon.svg --in-place --precision 1", "Minify in place"),
        OpSpec::custom("svg rasterize", Category::Svg, "Render an SVG to a raster image", rasterize)
            .about("Renders through usvg + resvg (static SVG 1.1 with filters; system fonts for text; external hrefs disabled). Default size is the SVG's own; `--size` with a blank side keeps the aspect ratio.")
            .param(param("size", ParamKind::Size).help("Output size WxH; '1600x' keeps the aspect ratio"))
            .param(param("scale", ParamKind::Float).range("0.001..1000").help("Scale factor instead of --size"))
            .param(param("background-color", ParamKind::Color).help("Fill behind the drawing (default transparent)"))
            .example("pixelox svg rasterize chart.svg -o chart@2x.png --scale 2", "A 2x PNG")
            .example("pixelox svg rasterize logo.svg -o logo.png --size 1600x --background-color '#ffffff'", "1600 px wide on white")
            .example("pixelox svg rasterize icon.svg -o icon-64.png --size 64x64", "A 64 px icon"),
    ]
}

// ---------------------------------------------------------------------------------------
// shared plumbing

/// Read the input SVG (recorded as this step's `source`) and parse it.
pub fn load_svg(ctx: &mut OpCtx) -> Result<(PathBuf, Vec<u8>, XDoc)> {
    let input = ctx.input(0)?.to_path_buf();
    let bytes = ctx.read_aux(&input, "source", None)?;
    if bytes.starts_with(&[0x1f, 0x8b]) {
        return Err(Error::new("PX1105", format!("{} is gzip-compressed (svgz)", input.display()))
            .with_hint("decompress it first: gunzip -c in.svgz > in.svg"));
    }
    let text = std::str::from_utf8(&bytes)
        .map_err(|_| Error::new("PX1103", format!("{} is not UTF-8 text", input.display())))?;
    let doc = XDoc::parse_svg(text)?;
    Ok((input, bytes, doc))
}

/// `-o` or, with `--in-place`, the input; None in a dry run without either.
pub fn target(ctx: &OpCtx) -> Result<Option<PathBuf>> {
    if ctx.globals.in_place {
        if ctx.globals.output.is_some() {
            return Err(Error::new("PX1005", "pass either -o or --in-place, not both"));
        }
        return Ok(Some(ctx.input(0)?.to_path_buf()));
    }
    match &ctx.globals.output {
        Some(o) => Ok(Some(PathBuf::from(o))),
        None if ctx.dry_run() => Ok(None),
        None => Err(Error::new("PX1004", format!("'{}' writes an SVG; say where", ctx.command))
            .with_hint("pass -o out.svg, or --in-place to rewrite the input")),
    }
}

fn write_svg(ctx: &mut OpCtx, to: Option<&Path>, doc: &XDoc, o: WriteOpts) -> Result<usize> {
    let text = doc.serialize(o);
    if let Some(p) = to {
        ctx.write_file(p, text.as_bytes(), None, None)?;
    }
    Ok(text.len())
}

fn out_json(to: &Option<PathBuf>) -> Value {
    to.as_ref().map(|p| json!(p.display().to_string())).unwrap_or(Value::Null)
}

fn parse_attrs(a: &Args) -> Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    for raw in a.all("attr") {
        let (k, v) = raw.split_once('=').ok_or_else(|| {
            Error::new("PX1010", format!("--attr '{raw}' is not k=v"))
                .with_hint("e.g. --attr fill='#0b5fff' --attr stroke-width=2")
        })?;
        let k = k.trim();
        if k.is_empty() || !k.chars().all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | ':' | '.')) {
            return Err(Error::new("PX1010", format!("--attr: '{k}' is not a valid attribute name")));
        }
        let v = v.trim();
        let v = v
            .strip_prefix('"')
            .and_then(|x| x.strip_suffix('"'))
            .or_else(|| v.strip_prefix('\'').and_then(|x| x.strip_suffix('\'')))
            .unwrap_or(v);
        out.push((k.to_string(), v.to_string()));
    }
    Ok(out)
}

fn parent_path(doc: &XDoc, a: &Args) -> Result<ElPath> {
    match a.str("parent") {
        None => Ok(Vec::new()),
        Some(p) => {
            let id = p.trim().trim_start_matches('#');
            doc.find_id(id).ok_or_else(|| no_id(id))
        }
    }
}

fn no_id(id: &str) -> Error {
    Error::new("PX1010", format!("no element with id '{id}'"))
        .with_hint("list ids with: pixelox svg query <file.svg> --where '[id]' --json")
        .with_context("id", id)
}

fn new_id(doc: &XDoc, a: &Args, base: &str) -> Result<String> {
    let ids = doc.ids();
    match a.str("id") {
        Some(i) => {
            let i = i.trim().to_string();
            if !valid_id(&i) {
                return Err(Error::new("PX1010", format!("--id '{i}' is not a valid XML id"))
                    .with_hint("start with a letter; use letters, digits, '-', '_', '.'"));
            }
            if ids.contains(&i) {
                return Err(
                    Error::new("PX1010", format!("an element with id '{i}' already exists")).with_context("id", &i)
                );
            }
            Ok(i)
        }
        None => Ok((1..).map(|n| format!("{base}-{n}")).find(|c| !ids.contains(c)).expect("unbounded")),
    }
}

/// Elements addressed by `--id` or `--where`.
fn targets(doc: &XDoc, a: &Args) -> Result<Vec<ElPath>> {
    match (a.str("id"), a.str("where")) {
        (Some(_), Some(_)) => Err(Error::new("PX1005", "pass either --id or --where, not both")),
        (None, None) => Err(Error::new("PX1003", "say which elements: --id ID or --where SELECTOR")),
        (Some(id), None) => {
            let id = id.trim().trim_start_matches('#');
            Ok(vec![doc.find_id(id).ok_or_else(|| no_id(id))?])
        }
        (None, Some(w)) => {
            let m = doc.select(&Selector::parse(&w)?);
            if m.is_empty() {
                return Err(Error::new("PX1010", format!("no elements match '{w}'"))
                    .with_hint("inspect the file with: pixelox svg query <file.svg> --json")
                    .with_context("where", &w));
            }
            Ok(m)
        }
    }
}

fn describe_el(e: &XEl) -> String {
    e.id().map(|i| format!("#{i}")).unwrap_or_else(|| e.name.clone())
}

/// Remove `prop` from an element's `style` attribute (so a presentation attribute wins).
fn strip_style(e: &mut XEl, prop: &str) {
    let Some(style) = e.get("style").map(String::from) else { return };
    let kept: Vec<&str> = style
        .split(';')
        .filter(|d| !d.trim().is_empty())
        .filter(|d| d.split(':').next().map(|p| p.trim() != prop).unwrap_or(true))
        .collect();
    if kept.is_empty() {
        e.remove("style");
    } else {
        e.set("style", kept.iter().map(|s| s.trim()).collect::<Vec<_>>().join(";"));
    }
}

/// usvg options: external hrefs disabled; system fonts only when the SVG has text.
pub fn usvg_options(fonts: bool) -> usvg::Options<'static> {
    let mut opt = usvg::Options {
        image_href_resolver: usvg::ImageHrefResolver {
            resolve_data: usvg::ImageHrefResolver::default_data_resolver(),
            resolve_string: Box::new(|_: &str, _: &usvg::Options| None),
        },
        ..Default::default()
    };
    if fonts {
        opt.fontdb_mut().load_system_fonts();
    }
    opt
}

pub fn has_text(doc: &XDoc) -> bool {
    doc.walk().iter().any(|(_, e)| matches!(e.local(), "text" | "tspan" | "textPath"))
}

// ---------------------------------------------------------------------------------------
// commands

fn new(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let to = match &ctx.globals.output {
        Some(o) => Some(PathBuf::from(o)),
        None if ctx.dry_run() => None,
        None => {
            return Err(Error::new("PX1004", "'svg new' creates a file; say where").with_hint("pass -o drawing.svg"))
        }
    };
    let s = a.size("size")?.ok_or_else(|| Error::new("PX1003", "missing --size"))?;
    let (Some(w), Some(h)) = (s.w, s.h) else {
        return Err(Error::new("PX1011", "--size needs both sides, e.g. 800x600"));
    };
    let (w, h) = (w.to_px(0.0, 96.0), h.to_px(0.0, 96.0));
    if w <= 0.0 || h <= 0.0 {
        return Err(Error::new("PX1011", "--size must be positive"));
    }
    let vb = match a.str("viewbox") {
        Some(v) => {
            let n: Vec<f64> = v
                .split(|c: char| c == ',' || c.is_whitespace())
                .filter(|p| !p.is_empty())
                .map(|p| p.parse::<f64>())
                .collect::<std::result::Result<_, _>>()
                .map_err(|_| bad_viewbox(&v))?;
            if n.len() != 4 || n[2] <= 0.0 || n[3] <= 0.0 {
                return Err(bad_viewbox(&v));
            }
            n.iter().map(|x| fmt_num(*x, 6)).collect::<Vec<_>>().join(" ")
        }
        None => format!("0 0 {} {}", fmt_num(w, 6), fmt_num(h, 6)),
    };
    let root = XEl::new("svg")
        .attr("xmlns", SVG_NS)
        .attr("width", fmt_num(w, 6))
        .attr("height", fmt_num(h, 6))
        .attr("viewBox", &vb);
    let doc = XDoc::new(root);
    write_svg(ctx, to.as_deref(), &doc, WriteOpts::PRETTY)?;
    Ok(json!({"path": out_json(&to), "size": [w, h], "viewbox": vb}))
}

fn bad_viewbox(v: &str) -> Error {
    Error::new("PX1010", format!("--viewbox '{v}' is not 'minX minY width height' with positive width and height"))
}

fn el_add(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let (_, _, mut doc) = load_svg(ctx)?;
    let to = target(ctx)?;
    let kind = a.choice("kind");
    let id = new_id(&doc, a, &kind)?;
    let parent = parent_path(&doc, a)?;
    let mut el = XEl::new(&kind).attr("id", &id);
    for (k, v) in parse_attrs(a)? {
        if k == "id" {
            return Err(Error::new("PX1005", "give the id with --id, not --attr id=..."));
        }
        el.set(&k, v);
    }
    if let Some(d) = a.str("d") {
        if kind != "path" {
            return Err(Error::new("PX1010", "--d applies to --kind path")
                .with_hint("for other kinds use --attr, e.g. --attr points='0,0 10,0 5,8'"));
        }
        parse_d(&d)?;
        el.set("d", d.trim());
    }
    if kind == "path" && el.get("d").is_none() {
        return Err(Error::new("PX1003", "--kind path needs --d 'M ...'"));
    }
    if let Some(t) = a.str("text") {
        if kind != "text" {
            return Err(Error::new("PX1010", "--text applies to --kind text"));
        }
        el.set_text(&t);
    }
    let parent_desc = describe_el(doc.el(&parent));
    doc.el_mut(&parent).push(el);
    doc.ensure_xlink();
    write_svg(ctx, to.as_deref(), &doc, WriteOpts::PRETTY)?;
    Ok(json!({"path": out_json(&to), "id": id, "tag": kind, "parent": parent_desc, "elements": doc.walk().len()}))
}

fn el_set(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let (_, _, mut doc) = load_svg(ctx)?;
    let to = target(ctx)?;
    let paths = targets(&doc, a)?;
    let attrs = parse_attrs(a)?;
    let unset = a.all("unset");
    let text = a.str("text");
    if attrs.is_empty() && unset.is_empty() && text.is_none() {
        return Err(Error::new("PX1003", "'svg el set' needs --attr k=v, --unset NAME, or --text"));
    }
    if let Some((_, new)) = attrs.iter().find(|(k, _)| k == "id") {
        if paths.len() > 1 {
            return Err(Error::new("PX1010", "cannot give several elements the same id"));
        }
        if !valid_id(new) || doc.find_id(new).map(|p| p != paths[0]).unwrap_or(false) {
            return Err(Error::new("PX1010", format!("id '{new}' is invalid or already used")));
        }
    }
    let mut touched = Vec::new();
    for p in &paths {
        let e = doc.el_mut(p);
        for (k, v) in &attrs {
            e.set(k, v.clone());
            strip_style(e, k);
        }
        for k in &unset {
            e.remove(k.trim());
            strip_style(e, k.trim());
        }
        if let Some(t) = &text {
            e.set_text(t);
        }
        touched.push(describe_el(e));
    }
    doc.ensure_xlink();
    write_svg(ctx, to.as_deref(), &doc, WriteOpts::PRETTY)?;
    Ok(json!({"path": out_json(&to), "matched": paths.len(), "elements": touched}))
}

fn el_rm(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let (_, _, mut doc) = load_svg(ctx)?;
    let to = target(ctx)?;
    let mut paths = targets(&doc, a)?;
    if paths.iter().any(|p| p.is_empty()) {
        return Err(Error::new("PX1010", "cannot remove the root <svg> element"));
    }
    let matched = paths.len();
    // descendants of removed elements go with them
    let all = paths.clone();
    paths.retain(|p| !all.iter().any(|q| q.len() < p.len() && p.starts_with(q)));
    paths.sort();
    let mut removed = Vec::new();
    for p in paths.iter().rev() {
        if let XNode::El(e) = doc.remove(p) {
            removed.push(describe_el(&e));
        }
    }
    removed.reverse();
    write_svg(ctx, to.as_deref(), &doc, WriteOpts::PRETTY)?;
    Ok(json!({"path": out_json(&to), "matched": matched, "removed": removed.len(), "elements": removed}))
}

fn scalar(v: &Value) -> Option<String> {
    Some(match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.as_f64().map(|f| fmt_num(f, 6)).unwrap_or_else(|| n.to_string()),
        Value::Bool(b) => b.to_string(),
        Value::Array(items) => items.iter().map(scalar).collect::<Option<Vec<_>>>()?.join(" "),
        _ => return None,
    })
}

const FE: &[&str] = &[
    "feBlend",
    "feColorMatrix",
    "feComponentTransfer",
    "feComposite",
    "feConvolveMatrix",
    "feDiffuseLighting",
    "feDisplacementMap",
    "feDropShadow",
    "feFlood",
    "feGaussianBlur",
    "feImage",
    "feMerge",
    "feMorphology",
    "feOffset",
    "feSpecularLighting",
    "feTile",
    "feTurbulence",
];

/// `gaussian-blur` / `gaussianBlur` / `feGaussianBlur` → `feGaussianBlur`.
fn fe_name(t: &str) -> String {
    if t.starts_with("fe") && t.chars().nth(2).map(|c| c.is_uppercase()).unwrap_or(false) {
        return t.to_string();
    }
    let mut out = String::from("fe");
    let mut up = true;
    for c in t.chars() {
        if c == '-' || c == '_' {
            up = true;
        } else if up {
            out.extend(c.to_uppercase());
            up = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// Build an element from a JSON object: `tag`/`type`, `attrs`, `children`, `text`, and
/// scalar keys as attributes.
fn json_el(v: &Value, filter_child: bool) -> Result<XEl> {
    let o = v.as_object().ok_or_else(|| Error::new("PX1010", format!("spec element {v} is not a JSON object")))?;
    let tag = ["tag", "type", "kind"]
        .iter()
        .find_map(|k| o.get(*k).and_then(|t| t.as_str()))
        .ok_or_else(|| Error::new("PX1010", format!("spec element {v} has no \"tag\" (or \"type\")")))?;
    let tag = if filter_child { fe_name(tag) } else { tag.to_string() };
    if filter_child && !FE.contains(&tag.as_str()) {
        return Err(
            Error::new("PX1010", format!("'{tag}' is not an SVG filter primitive")).with_context("primitives", FE)
        );
    }
    let mut el = XEl::new(&tag);
    fill_el(&mut el, o, &["tag", "type", "kind"])?;
    Ok(el)
}

fn fill_el(el: &mut XEl, o: &Map<String, Value>, skip: &[&str]) -> Result<()> {
    if let Some(Value::Object(attrs)) = o.get("attrs") {
        for (k, v) in attrs {
            if let Some(s) = scalar(v) {
                el.set(k, s);
            }
        }
    }
    for (k, v) in o {
        if skip.contains(&k.as_str())
            || matches!(
                k.as_str(),
                "attrs" | "children" | "primitives" | "text" | "stops" | "units" | "content_units" | "d"
            )
        {
            continue;
        }
        if let Some(s) = scalar(v) {
            el.set(k, s);
        }
    }
    if let Some(d) = o.get("d").and_then(|d| d.as_str()) {
        parse_d(d)?;
        if el.local() == "path" {
            el.set("d", d);
        } else {
            el.push(XEl::new("path").attr("d", d));
        }
    }
    for key in ["children", "primitives"] {
        if let Some(ch) = o.get(key) {
            let arr = ch.as_array().ok_or_else(|| Error::new("PX1010", format!("\"{key}\" must be an array")))?;
            for c in arr {
                // nested filter-primitive children (feMergeNode, feFuncR, lights) keep their tag
                el.push(json_el(c, key == "primitives")?);
            }
        }
    }
    if let Some(t) = o.get("text").and_then(|t| t.as_str()) {
        el.children.push(XNode::Text(t.to_string()));
    }
    Ok(())
}

fn stops_el(v: &Value) -> Result<Vec<XEl>> {
    let stop = |offset: f64, c: Rgba, extra_op: Option<f64>| {
        let mut e = XEl::new("stop").attr("offset", fmt_num(offset, 4)).attr("stop-color", c.with_alpha(1.0).to_hex());
        let op = c.a as f64 * extra_op.unwrap_or(1.0);
        if op < 1.0 {
            e.set("stop-opacity", fmt_num(op, 4));
        }
        e
    };
    match v {
        Value::String(s) => {
            Ok(color::parse_stops(s)?.into_iter().map(|st| stop(st.pos as f64, st.color, None)).collect())
        }
        Value::Array(items) => items
            .iter()
            .map(|it| {
                let off = it
                    .get("offset")
                    .and_then(|o| {
                        o.as_f64().or_else(|| {
                            o.as_str().and_then(|s| {
                                s.trim_end_matches('%').parse::<f64>().ok().map(|x| {
                                    if s.ends_with('%') {
                                        x / 100.0
                                    } else {
                                        x
                                    }
                                })
                            })
                        })
                    })
                    .unwrap_or(0.0);
                let c = color::parse_color(it.get("color").and_then(|c| c.as_str()).unwrap_or("#000"))?;
                Ok(stop(off, c, it.get("opacity").and_then(|o| o.as_f64())))
            })
            .collect(),
        _ => Err(Error::new("PX1010", "\"stops\" must be '#000@0,#fff@1' or an array of {offset, color, opacity}")),
    }
}

fn defs_add(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let (_, _, mut doc) = load_svg(ctx)?;
    let to = target(ctx)?;
    let kind = a.choice("kind");
    let id = new_id(&doc, a, &kind)?;
    let raw = a.str("spec").unwrap_or_default();
    let spec: Value = if raw.trim_start().starts_with('{') {
        serde_json::from_str(&raw)?
    } else {
        let bytes = ctx.read_aux(Path::new(&raw), "spec", Some("spec"))?;
        serde_json::from_slice(&bytes)?
    };
    let o = spec.as_object().ok_or_else(|| Error::new("PX1010", "--spec must be a JSON object"))?;
    let (tag, units, content_units) = match kind.as_str() {
        "linear-gradient" => ("linearGradient", "gradientUnits", None),
        "radial-gradient" => ("radialGradient", "gradientUnits", None),
        "pattern" => ("pattern", "patternUnits", Some("patternContentUnits")),
        "clip-path" => ("clipPath", "clipPathUnits", None),
        "mask" => ("mask", "maskUnits", Some("maskContentUnits")),
        "filter" => ("filter", "filterUnits", Some("primitiveUnits")),
        "symbol" => ("symbol", "", None),
        _ => ("marker", "markerUnits", None),
    };
    let mut el = XEl::new(tag).attr("id", &id);
    if let Some(u) = o.get("units").and_then(scalar) {
        if units.is_empty() {
            return Err(Error::new("PX1010", format!("a {kind} has no units attribute")));
        }
        el.set(units, u);
    }
    if let Some(u) = o.get("content_units").and_then(scalar) {
        let cu =
            content_units.ok_or_else(|| Error::new("PX1010", format!("a {kind} has no content units attribute")))?;
        el.set(cu, u);
    }
    if kind == "filter" {
        if let Some(c) = o.get("children") {
            return Err(Error::new("PX1010", format!("filters take \"primitives\", not \"children\" ({c})")));
        }
    } else if o.contains_key("primitives") {
        return Err(Error::new("PX1010", "\"primitives\" is only for --kind filter"));
    }
    fill_el(&mut el, o, &[])?;
    if kind.ends_with("gradient") {
        let stops = o.get("stops").ok_or_else(|| {
            Error::new("PX1003", "a gradient spec needs \"stops\"")
                .with_hint("e.g. {\"stops\": \"#0b5fff@0,#00d4ff@1\"}")
        })?;
        for s in stops_el(stops)? {
            el.push(s);
        }
    }
    let children = el.elements().count();
    // into the first <defs> child of the root (created when missing)
    let di = doc.root.children.iter().position(|c| matches!(c, XNode::El(e) if e.local() == "defs"));
    let di = match di {
        Some(i) => i,
        None => {
            doc.root.children.insert(0, XNode::El(XEl::new("defs")));
            0
        }
    };
    doc.el_mut(&[di]).push(el);
    doc.ensure_xlink();
    write_svg(ctx, to.as_deref(), &doc, WriteOpts::PRETTY)?;
    Ok(json!({"path": out_json(&to), "id": id, "tag": tag, "children": children, "ref": format!("url(#{id})")}))
}

fn use_(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let (_, _, mut doc) = load_svg(ctx)?;
    let to = target(ctx)?;
    let r = a.str("ref").unwrap_or_default();
    let rid = r.trim().trim_start_matches('#').to_string();
    if doc.find_id(&rid).is_none() {
        return Err(no_id(&rid));
    }
    let id = new_id(&doc, a, &format!("{rid}-use"))?;
    let parent = parent_path(&doc, a)?;
    let at = a.point("at")?.map(|p| p.resolve(0.0, 0.0, 96.0)).unwrap_or((0.0, 0.0));
    let mut el = XEl::new("use")
        .attr("id", &id)
        .attr("href", format!("#{rid}"))
        .attr("x", fmt_num(at.0, 4))
        .attr("y", fmt_num(at.1, 4));
    if let Some(s) = a.size("size")? {
        if let Some(w) = s.w {
            el.set("width", fmt_num(w.to_px(0.0, 96.0), 4));
        }
        if let Some(h) = s.h {
            el.set("height", fmt_num(h.to_px(0.0, 96.0), 4));
        }
    }
    doc.el_mut(&parent).push(el);
    write_svg(ctx, to.as_deref(), &doc, WriteOpts::PRETTY)?;
    Ok(json!({"path": out_json(&to), "id": id, "ref": format!("#{rid}"), "at": [at.0, at.1]}))
}

fn rect_json(r: usvg::Rect) -> Value {
    let f = |v: f32| crate::spec::round(v as f64, 3);
    json!([f(r.x()), f(r.y()), f(r.width()), f(r.height())])
}

fn query(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let (_, _, doc) = load_svg(ctx)?;
    let w = a.str_or("where", "*");
    let paths = doc.select(&Selector::parse(&w)?);
    // tag every match with an id so usvg can report its box
    let mut tmp = doc.clone();
    let mut keys = Vec::new();
    for (i, p) in paths.iter().enumerate() {
        let e = tmp.el_mut(p);
        let k = match e.id() {
            Some(id) => id.to_string(),
            None => {
                let k = format!("__pxq{i}");
                e.set("id", &k);
                k
            }
        };
        keys.push(k);
    }
    let opt = usvg_options(has_text(&tmp));
    let tree = usvg::Tree::from_str(&tmp.serialize(WriteOpts::PRETTY), &opt);
    let mut els = Vec::new();
    for (p, k) in paths.iter().zip(&keys) {
        let e = doc.el(p);
        let mut attrs = Map::new();
        for (n, v) in &e.attrs {
            attrs.insert(n.clone(), json!(v));
        }
        let (bbox, sbbox) = match &tree {
            Ok(t) if p.is_empty() => {
                let r = json!([
                    0.0,
                    0.0,
                    crate::spec::round(t.size().width() as f64, 3),
                    crate::spec::round(t.size().height() as f64, 3)
                ]);
                (r.clone(), r)
            }
            Ok(t) => match t.node_by_id(k) {
                Some(n) => (rect_json(n.abs_bounding_box()), rect_json(n.abs_stroke_bounding_box())),
                None => (Value::Null, Value::Null),
            },
            Err(_) => (Value::Null, Value::Null),
        };
        let text = e.text_content();
        els.push(json!({
            "tag": e.name,
            "id": e.id(),
            "attrs": attrs,
            "text": if text.trim().is_empty() { Value::Null } else { json!(text) },
            "children": e.elements().count(),
            "bbox": bbox,
            "stroke_bbox": sbbox,
        }));
    }
    let mut r = json!({"where": w, "count": els.len(), "elements": els});
    if let Err(e) = tree {
        r["bbox_error"] = json!(e.to_string());
    }
    Ok(r)
}

// ---------------------------------------------------------------------------------------
// optimize

const EDITOR_NS: &[&str] = &[
    "http://www.inkscape.org/namespaces/inkscape",
    "http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd",
    "http://inkscape.sourceforge.net/DTD/sodipodi-0.dtd",
    "http://www.bohemiancoding.com/sketch/ns",
    "http://ns.adobe.com/AdobeIllustrator/10.0/",
    "http://ns.adobe.com/Graphs/1.0/",
    "http://ns.adobe.com/AdobeSVGViewerExtensions/3.0/",
    "http://ns.adobe.com/Variables/1.0/",
    "http://ns.adobe.com/SaveForWeb/1.0/",
    "http://ns.adobe.com/Extensibility/1.0/",
    "http://ns.adobe.com/Flows/1.0/",
    "http://ns.adobe.com/ImageReplacement/1.0/",
    "http://ns.adobe.com/GenericCustomNamespace/1.0/",
    "http://ns.adobe.com/XPath/1.0/",
    "http://www.serif.com/",
    "http://www.vector.evaxdesign.sk",
    "http://www.figma.com/figma/ns",
    "http://creativecommons.org/ns#",
    "http://purl.org/dc/elements/1.1/",
    "http://www.w3.org/1999/02/22-rdf-syntax-ns#",
];

const NUMERIC_ATTRS: &[&str] = &[
    "x",
    "y",
    "width",
    "height",
    "cx",
    "cy",
    "r",
    "rx",
    "ry",
    "x1",
    "y1",
    "x2",
    "y2",
    "dx",
    "dy",
    "fx",
    "fy",
    "fr",
    "refX",
    "refY",
    "markerWidth",
    "markerHeight",
    "stroke-width",
    "stroke-dashoffset",
    "stroke-dasharray",
    "stroke-miterlimit",
    "font-size",
    "letter-spacing",
    "word-spacing",
    "opacity",
    "fill-opacity",
    "stroke-opacity",
    "stop-opacity",
    "flood-opacity",
    "offset",
    "stdDeviation",
    "viewBox",
    "points",
    "transform",
    "gradientTransform",
    "patternTransform",
    "rotate",
    "radius",
    "scale",
];

/// `0.5` → `.5`, `-0.5` → `-.5`.
fn short_num(v: f64, prec: usize) -> String {
    let s = fmt_num(v, prec);
    if let Some(r) = s.strip_prefix("0.") {
        format!(".{r}")
    } else if let Some(r) = s.strip_prefix("-0.") {
        format!("-.{r}")
    } else {
        s
    }
}

/// Length of the number token at the start of `b` (sign, mantissa, exponent), 0 if none.
fn number_len(b: &[u8]) -> usize {
    let mut i = 0;
    if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        i += 1;
    }
    let ds = i;
    while i < b.len() && b[i].is_ascii_digit() {
        i += 1;
    }
    let mut digits = i > ds;
    if i < b.len() && b[i] == b'.' {
        i += 1;
        let fs = i;
        while i < b.len() && b[i].is_ascii_digit() {
            i += 1;
        }
        digits |= i > fs;
    }
    if !digits {
        return 0;
    }
    if i < b.len() && (b[i] == b'e' || b[i] == b'E') {
        let mut j = i + 1;
        if j < b.len() && (b[j] == b'+' || b[j] == b'-') {
            j += 1;
        }
        let es = j;
        while j < b.len() && b[j].is_ascii_digit() {
            j += 1;
        }
        if j > es {
            i = j;
        }
    }
    i
}

/// Round every number token in a value; other characters are kept.
fn shorten_numbers(v: &str, prec: usize) -> String {
    let b = v.as_bytes();
    let mut out = String::with_capacity(v.len());
    let mut i = 0;
    // the number just written, when nothing separates it from what follows
    let mut last: Option<String> = None;
    while i < b.len() {
        let prev_alpha = i > 0 && (b[i - 1].is_ascii_alphabetic() || b[i - 1] == b'#' || b[i - 1] == b'_');
        let n = if prev_alpha { 0 } else { number_len(&b[i..]) };
        if n > 0 {
            let tok = &v[i..i + n];
            let s = tok.parse::<f64>().map(|x| short_num(x, prec)).unwrap_or_else(|_| tok.to_string());
            if let Some(p) = &last {
                // compact lists ("1.5.5") must not merge after rounding
                if !s.starts_with('-')
                    && !s.starts_with('+')
                    && !(s.starts_with('.') && (p.contains('.') || p.contains('e')))
                {
                    out.push(' ');
                }
            }
            out.push_str(&s);
            last = Some(s);
            i += n;
        } else {
            let c = v[i..].chars().next().expect("char");
            out.push(c);
            last = None;
            i += c.len_utf8();
        }
    }
    out
}

fn push_num(out: &mut String, prev: &mut Option<String>, tok: String) {
    if let Some(p) = prev {
        let joinable = tok.starts_with('-') || (tok.starts_with('.') && (p.contains('.') || p.contains('e')));
        if !joinable {
            out.push(' ');
        }
    }
    out.push_str(&tok);
    *prev = Some(tok);
}

/// Re-serialize path data compactly (None when it does not tokenize; then it is kept).
pub fn minify_path(d: &str, prec: usize) -> Option<String> {
    let b = d.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    let mut cmd: Option<u8> = None;
    let mut argi = 0usize;
    let mut prev: Option<String> = None;
    let nargs = |c: u8| match c.to_ascii_uppercase() {
        b'M' | b'L' | b'T' => 2,
        b'H' | b'V' => 1,
        b'C' => 6,
        b'S' | b'Q' => 4,
        b'A' => 7,
        _ => 0,
    };
    loop {
        while i < b.len() && (b[i].is_ascii_whitespace() || b[i] == b',') {
            i += 1;
        }
        if i >= b.len() {
            break;
        }
        let c = b[i];
        if c.is_ascii_alphabetic() {
            if !b"MLHVCSQTAZmlhvcsqtaz".contains(&c) {
                return None;
            }
            let up = c.to_ascii_uppercase();
            let repeat =
                cmd == Some(c) && up != b'M' && up != b'Z' && nargs(c) > 0 && argi.is_multiple_of(nargs(c)) && argi > 0;
            if !repeat {
                out.push(c as char);
                prev = None;
                argi = 0;
            }
            cmd = Some(c);
            i += 1;
            continue;
        }
        let cm = cmd?;
        let n = nargs(cm);
        if n == 0 {
            return None;
        }
        if cm.eq_ignore_ascii_case(&b'A') && matches!(argi % 7, 3 | 4) {
            if b[i] != b'0' && b[i] != b'1' {
                return None;
            }
            push_num(&mut out, &mut prev, (b[i] as char).to_string());
            i += 1;
        } else {
            let len = number_len(&b[i..]);
            if len == 0 {
                return None;
            }
            let x: f64 = d[i..i + len].parse().ok()?;
            push_num(&mut out, &mut prev, short_num(x, prec));
            i += len;
        }
        argi += 1;
    }
    Some(out)
}

#[derive(Default)]
struct OptStats {
    comments: usize,
    metadata: usize,
    editor: usize,
    empty_groups: usize,
    namespaces: usize,
}

fn strip(e: &mut XEl, editor: &[String], st: &mut OptStats, prec: usize) {
    let is_editor = |n: &str| n.split_once(':').map(|(p, _)| editor.iter().any(|x| x == p)).unwrap_or(false);
    let before = e.attrs.len();
    e.attrs.retain(|(k, _)| {
        let decl = k.strip_prefix("xmlns:").map(|p| editor.iter().any(|x| x == p)).unwrap_or(false);
        !(is_editor(k) || decl)
    });
    st.editor += before - e.attrs.len();
    for (k, v) in e.attrs.iter_mut() {
        if k == "d" {
            if let Some(m) = minify_path(v, prec) {
                *v = m;
            }
        } else if NUMERIC_ATTRS.contains(&local_name(k)) {
            *v = shorten_numbers(v, prec);
        }
    }
    let mut kept = Vec::with_capacity(e.children.len());
    for c in std::mem::take(&mut e.children) {
        match c {
            XNode::Comment(_) | XNode::Pi { .. } => st.comments += 1,
            XNode::El(el) if el.local() == "metadata" => st.metadata += 1,
            XNode::El(el) if is_editor(&el.name) => st.editor += 1,
            XNode::El(mut el) => {
                strip(&mut el, editor, st, prec);
                let empty = matches!(el.local(), "g" | "defs")
                    && el.id().is_none()
                    && el.children.iter().all(|c| matches!(c, XNode::Text(t) if t.trim().is_empty()));
                if empty {
                    st.empty_groups += 1;
                } else {
                    kept.push(XNode::El(el));
                }
            }
            other => kept.push(other),
        }
    }
    e.children = kept;
}

fn used_prefixes(e: &XEl, out: &mut std::collections::BTreeSet<String>) {
    if let Some((p, _)) = e.name.split_once(':') {
        out.insert(p.to_string());
    }
    for (k, _) in &e.attrs {
        if let Some((p, _)) = k.split_once(':') {
            if p != "xmlns" {
                out.insert(p.to_string());
            }
        }
    }
    for c in e.elements() {
        used_prefixes(c, out);
    }
}

fn drop_unused_ns(e: &mut XEl, used: &std::collections::BTreeSet<String>, st: &mut OptStats) {
    let before = e.attrs.len();
    e.attrs.retain(|(k, _)| k.strip_prefix("xmlns:").map(|p| used.contains(p)).unwrap_or(true));
    st.namespaces += before - e.attrs.len();
    for c in e.children.iter_mut() {
        if let XNode::El(el) = c {
            drop_unused_ns(el, used, st);
        }
    }
}

fn optimize(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let (_, bytes, mut doc) = load_svg(ctx)?;
    let to = target(ctx)?;
    let prec = a.usize("precision")?;
    let mut editor: Vec<String> = Vec::new();
    for (_, e) in doc.walk() {
        for (k, v) in &e.attrs {
            if let Some(p) = k.strip_prefix("xmlns:") {
                if EDITOR_NS.contains(&v.as_str()) && !editor.contains(&p.to_string()) {
                    editor.push(p.to_string());
                }
            }
        }
    }
    let mut st = OptStats { comments: doc.prolog.len() + doc.epilog.len(), ..Default::default() };
    doc.prolog.clear();
    doc.epilog.clear();
    strip(&mut doc.root, &editor, &mut st, prec);
    let mut used = std::collections::BTreeSet::new();
    used_prefixes(&doc.root, &mut used);
    drop_unused_ns(&mut doc.root, &used, &mut st);
    let after = write_svg(ctx, to.as_deref(), &doc, WriteOpts::MINIFIED)?;
    let before = bytes.len();
    Ok(json!({
        "path": out_json(&to),
        "bytes_before": before,
        "bytes_after": after,
        "saved_pct": crate::spec::round(100.0 * (1.0 - after as f64 / before.max(1) as f64), 1),
        "removed": {
            "comments": st.comments,
            "metadata": st.metadata,
            "editor_items": st.editor,
            "empty_groups": st.empty_groups,
            "namespaces": st.namespaces,
        },
        "precision": prec,
    }))
}

// ---------------------------------------------------------------------------------------
// rasterize

/// Premultiplied 8-bit pixmap → straight-alpha `f32` image.
pub fn pixmap_to_image(p: &tiny_skia::Pixmap) -> Image {
    let mut bytes = Vec::with_capacity(p.pixels().len() * 4);
    for px in p.pixels() {
        let c = px.demultiply();
        bytes.extend_from_slice(&[c.red(), c.green(), c.blue(), c.alpha()]);
    }
    Image::from_rgba8(p.width(), p.height(), &bytes)
}

/// Render an SVG to an image of `w`×`h` with an optional background.
pub fn render_svg(tree: &usvg::Tree, w: u32, h: u32, bg: Option<Rgba>) -> Result<Image> {
    let mut pm =
        tiny_skia::Pixmap::new(w, h).ok_or_else(|| Error::new("PX3001", format!("cannot allocate a {w}x{h} image")))?;
    if let Some(c) = bg {
        if let Some(col) = tiny_skia::Color::from_rgba(c.r, c.g, c.b, c.a) {
            pm.fill(col);
        }
    }
    let s = tree.size();
    let tf = tiny_skia::Transform::from_scale(w as f32 / s.width(), h as f32 / s.height());
    resvg::render(tree, tf, &mut pm.as_mut());
    Ok(pixmap_to_image(&pm))
}

fn rasterize(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let (_, bytes, doc) = load_svg(ctx)?;
    let to = match &ctx.globals.output {
        Some(o) => Some(PathBuf::from(o)),
        None if ctx.dry_run() => None,
        None => {
            return Err(Error::new("PX1004", "'svg rasterize' writes an image; say where").with_hint("pass -o out.png"))
        }
    };
    if ctx.globals.in_place {
        return Err(Error::new("PX1005", "svg rasterize writes a new image; use -o out.png instead of --in-place"));
    }
    let opt = usvg_options(has_text(&doc));
    let tree =
        usvg::Tree::from_data(&bytes, &opt).map_err(|e| Error::new("PX1103", format!("cannot render the SVG: {e}")))?;
    let (tw, th) = (tree.size().width() as f64, tree.size().height() as f64);
    let (w, h) = match (a.size("size")?, a.opt_f64("scale")?) {
        (Some(_), Some(_)) => return Err(Error::new("PX1005", "pass either --size or --scale, not both")),
        (Some(s), None) => s.resolve(tw, th, 96.0),
        (None, Some(k)) => (((tw * k).round().max(1.0)) as u32, ((th * k).round().max(1.0)) as u32),
        (None, None) => (tw.ceil().max(1.0) as u32, th.ceil().max(1.0) as u32),
    };
    let limit = ctx.globals.open.limit_pixels;
    if w as u64 * h as u64 > limit {
        return Err(Error::new("PX1104", format!("{w}x{h} exceeds the pixel limit ({limit})"))
            .with_hint("lower --size/--scale or raise --limit-pixels"));
    }
    let bg = a.opt_color("background-color")?;
    let mut r = json!({"path": out_json(&to), "size": [w, h], "svg_size": [tw, th], "scale": [crate::spec::round(w as f64 / tw, 4), crate::spec::round(h as f64 / th, 4)]});
    if let Some(p) = &to {
        let img = render_svg(&tree, w, h, bg)?;
        let mut opts = EncodeOptions::for_path(p);
        if let Some(f) = &ctx.globals.format {
            opts.format = f.clone();
        }
        ctx.write_image(p, &img, None, &opts, None, None)?;
        r["coverage"] = json!(crate::spec::round(
            img.data.chunks(4).filter(|p| p[3] > 0.0).count() as f64 / img.pixels().max(1) as f64,
            4
        ));
    }
    Ok(r)
}
