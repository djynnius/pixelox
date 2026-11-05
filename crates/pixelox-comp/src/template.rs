//! `comp template init|slots|apply` (PRD §8.14, example E5): a template is a document with
//! typed slots. Text slots set a text layer's `content`, image slots replace a placed or
//! raster layer's content (fitted into the slot's box), and color slots set a text layer's
//! `color`, a shape's `fill`, or recolor a raster layer.
//!
//! Slot syntax: `kind:name[:required][:fit=cover|contain|fill][:layer=NAME][:max-lines=N][:min-size=LEN]`.

use crate::util::*;
use pixelox_core::doc::{LayerPath, Slot};
use pixelox_core::prelude::*;
use pixelox_core::units::{Length, Unit};

const KINDS: &[&str] = &["text", "image", "color"];
const FITS: &[&str] = &["cover", "contain", "fill"];
const SYNTAX: &str = "slot syntax: kind:name[:required][:fit=cover|contain|fill][:layer=NAME][:max-lines=N][:min-size=24pt], kind = text, image, or color";

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::doc("comp template init", Category::Comp, "Declare template slots on a document", init)
            .about(
                "Declares the slots that `comp template apply` fills, replacing any existing ones. Each \
                 `--slot` is `kind:name` plus options: `required`, `fit=cover|contain|fill` (image slots; \
                 default cover), `layer=NAME` (default: the layer named like the slot), and for text slots \
                 `max-lines=N` / `min-size=LEN`, which turn on auto-shrink. Text slots need a text layer, \
                 image slots a placed or raster layer, color slots a text, shape, or raster layer (PX2303). \
                 Without `--slot`, every text layer becomes an optional text slot and every placed layer an \
                 optional image slot.",
            )
            .param(param("slot", ParamKind::Str).repeatable().help("A slot: kind:name[:required][:fit=...][:layer=NAME][:max-lines=N][:min-size=LEN] (repeatable)"))
            .example(
                "pixelox comp template init poster.pxo -o poster-a3.pxo --slot text:headline:required --slot image:product:fit=cover --slot color:accent",
                "Declare the E5 poster slots",
            )
            .example(
                "pixelox comp template init card.pxo --in-place --slot text:title:required:max-lines=2:min-size=24pt",
                "A required title that shrinks to fit two lines",
            )
            .example("pixelox comp template init card.pxo --in-place", "Make every text and placed layer an optional slot"),
        OpSpec::doc("comp template slots", Category::Comp, "List a template's slots and constraints", slots)
            .about(
                "Lists a template's slots: name, kind, required, target layer, fit, constraints, and the \
                 current value. PX3201 when the document declares no slots. Read-only.",
            )
            .read_only()
            .example("pixelox comp template slots poster-a3.pxo", "List a template's slots")
            .example("pixelox comp template slots poster-a3.pxo --json", "Slots as JSON")
            .example("pixelox comp template slots card.pxo --json --quiet", "Slots as JSON, nothing on stderr"),
        OpSpec::doc("comp template apply", Category::Comp, "Fill template slots and write the finished document", apply)
            .about(
                "Fills slots by name: `--set name=TEXT` (text slots), `--set-image name=PATH` (image slots), \
                 `--set-color name=COLOR` (color slots); each is repeatable. Image slots fit the new image \
                 into the slot's box per its fit (`cover` crops to the box). Text slots with `max-lines` or \
                 `min-size` lower the font size until the text fits the slot's original box (needs the text \
                 renderer; skipped with a warning otherwise). Errors: not a template PX3201, a required slot \
                 left unfilled PX3202, unknown slot PX3203, wrong flag for the slot's kind PX3204.",
            )
            .param(param("set", ParamKind::Str).repeatable().help("Fill a text slot: name=TEXT (repeatable)"))
            .param(param("set-image", ParamKind::Str).repeatable().help("Fill an image slot: name=PATH (repeatable)"))
            .param(param("set-color", ParamKind::Str).repeatable().help("Fill a color slot: name=COLOR (repeatable)"))
            .example(
                "pixelox comp template apply poster-a3.pxo --set headline='FALL DROP 09' --set-image product=work/hero-cut.png --set-color accent='#0ea5e9' -o work/poster.pxo",
                "Build the E5 poster from its template",
            )
            .example("pixelox comp template apply card.pxo --set title='Hello world' -o card-1.pxo", "Fill one text slot")
            .example("pixelox comp template apply card.pxo --set title=Launch --set-color accent=teal -o card.png", "Fill slots and export a PNG"),
    ]
}

fn syntax_err(s: &str, msg: impl Into<String>) -> Error {
    Error::new("PX1010", format!("--slot '{s}': {}", msg.into())).with_hint(SYNTAX).with_context("slot", s)
}

/// Parse one `--slot` value.
pub(crate) fn parse_slot(s: &str) -> Result<Slot> {
    let parts: Vec<&str> = s.split(':').map(str::trim).collect();
    if parts.len() < 2 || parts[1].is_empty() {
        return Err(syntax_err(s, "expected kind:name"));
    }
    let kind = parts[0].to_ascii_lowercase();
    if !KINDS.contains(&kind.as_str()) {
        return Err(syntax_err(s, format!("'{}' is not a slot kind (text, image, color)", parts[0])));
    }
    let name = parts[1];
    if name.contains('=') || name.contains(',') {
        return Err(syntax_err(s, format!("'{name}' is not a valid slot name")));
    }
    let mut slot = Slot {
        name: name.to_string(),
        kind: kind.clone(),
        required: false,
        layer: None,
        fit: (kind == "image").then(|| "cover".to_string()),
        constraints: Map::new(),
    };
    for opt in &parts[2..] {
        match opt.split_once('=').map(|(k, v)| (k.trim(), v.trim())) {
            None if *opt == "required" => slot.required = true,
            None if *opt == "optional" => slot.required = false,
            Some(("fit", v)) => {
                if kind != "image" {
                    return Err(syntax_err(s, "fit= applies to image slots only"));
                }
                if !FITS.contains(&v) {
                    return Err(syntax_err(s, format!("fit '{v}' is not cover, contain, or fill")));
                }
                slot.fit = Some(v.to_string());
            }
            Some(("layer", v)) if !v.is_empty() => slot.layer = Some(v.to_string()),
            Some(("max-lines" | "max_lines", v)) => {
                if kind != "text" {
                    return Err(syntax_err(s, "max-lines= applies to text slots only"));
                }
                let n: u64 = v
                    .parse()
                    .ok()
                    .filter(|n| *n >= 1)
                    .ok_or_else(|| syntax_err(s, format!("max-lines '{v}' is not a whole number >= 1")))?;
                slot.constraints.insert("max_lines".into(), json!(n));
            }
            Some(("min-size" | "min_size", v)) => {
                if kind != "text" {
                    return Err(syntax_err(s, "min-size= applies to text slots only"));
                }
                Length::parse(v).map_err(|_| syntax_err(s, format!("min-size '{v}' is not a length such as 24pt")))?;
                slot.constraints.insert("min_size".into(), json!(v));
            }
            _ => return Err(syntax_err(s, format!("unknown option '{opt}'"))),
        }
    }
    Ok(slot)
}

/// The layer a slot fills: `layer=` or the layer named like the slot (PX2301).
fn slot_target(doc: &Document, slot: &Slot) -> Result<LayerPath> {
    let r = slot.layer.as_deref().unwrap_or(&slot.name);
    doc.resolve(r).map_err(|e| {
        let mut e = e.with_context("slot", &slot.name);
        if e.code == "PX2301" {
            e = e.with_hint(format!(
                "name the target layer '{r}', or say which layer fills the slot: --slot {}:{}:layer=NAME",
                slot.kind, slot.name
            ));
        }
        e
    })
}

/// PX2303 when a slot's kind cannot fill this layer kind.
fn check_kind(slot: &Slot, l: &Layer) -> Result<()> {
    let ok = match slot.kind.as_str() {
        "text" => matches!(l.content, LayerKind::Text { .. }),
        "image" => matches!(l.content, LayerKind::Placed { .. } | LayerKind::Raster { .. }),
        "color" => matches!(l.content, LayerKind::Text { .. } | LayerKind::Shape { .. } | LayerKind::Raster { .. }),
        _ => false,
    };
    if ok {
        return Ok(());
    }
    Err(Error::new("PX2303", format!("slot '{}' is a {} slot but layer '{}' is a {} layer", slot.name, slot.kind, l.name, l.kind_name()))
        .with_hint("text slots fill text layers; image slots fill placed or raster layers; color slots fill text, shape, or raster layers")
        .with_context("slot", &slot.name)
        .with_context("layer", &l.name)
        .with_context("kind", l.kind_name()))
}

fn slot_box(slot: &Slot) -> Option<[f64; 4]> {
    let a = slot.constraints.get("box")?.as_array()?;
    if a.len() != 4 {
        return None;
    }
    let v: Vec<f64> = a.iter().filter_map(|x| x.as_f64()).collect();
    (v.len() == 4).then(|| [v[0], v[1], v[2], v[3]])
}

fn current_value(slot: &Slot, l: &Layer) -> Value {
    match (slot.kind.as_str(), &l.content) {
        ("text", LayerKind::Text { spec }) => spec.get("content").cloned().unwrap_or(Value::Null),
        ("color", LayerKind::Text { spec }) => spec.get("color").cloned().unwrap_or(Value::Null),
        ("color", LayerKind::Shape { spec }) => spec.get("fill").cloned().unwrap_or(Value::Null),
        ("image", LayerKind::Placed { path, rect, .. }) => json!({"path": path, "rect": rect}),
        ("image", LayerKind::Raster { image }) => json!({"size": [image.width, image.height], "offset": [l.x, l.y]}),
        _ => Value::Null,
    }
}

fn slots_json(doc: &Document) -> Vec<Value> {
    doc.slots
        .iter()
        .map(|s| {
            let mut v = json!({
                "name": s.name,
                "kind": s.kind,
                "required": s.required,
                "layer": Value::Null,
                "fit": s.fit,
                "constraints": s.constraints,
            });
            if let Some(r) = &s.layer {
                v["layer_ref"] = json!(r);
            }
            match slot_target(doc, s) {
                Ok(p) => {
                    let l = doc.layer(&p);
                    v["layer"] = json!(l.name);
                    v["layer_kind"] = json!(l.kind_name());
                    v["current"] = current_value(s, l);
                }
                Err(e) => v["error"] = json!(e.code),
            }
            v
        })
        .collect()
}

fn not_a_template() -> Error {
    Error::new("PX3201", "this document declares no template slots")
        .with_hint("declare them: pixelox comp template init <doc> --slot text:headline:required -o template.pxo")
}

// ---------------------------------------------------------------------------------------
// init and slots

fn init(a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    let specs = a.all("slot");
    let mut slots: Vec<Slot> = Vec::new();
    let derived = specs.is_empty();
    if derived {
        for (_, l) in doc.walk() {
            let kind = match l.content {
                LayerKind::Text { .. } => "text",
                LayerKind::Placed { .. } => "image",
                _ => continue,
            };
            let fit = (kind == "image").then(|| "cover".to_string());
            slots.push(Slot {
                name: l.name.clone(),
                kind: kind.into(),
                required: false,
                layer: None,
                fit,
                constraints: Map::new(),
            });
        }
        if slots.is_empty() {
            return Err(Error::new(
                "PX1003",
                "no --slot given and the document has no text or placed layers to derive slots from",
            )
            .with_hint("declare slots: --slot text:headline:required --slot image:product --slot color:accent"));
        }
    } else {
        for s in &specs {
            let slot = parse_slot(s)?;
            if slots.iter().any(|x| x.name == slot.name) {
                return Err(syntax_err(s, format!("slot '{}' is declared twice", slot.name)));
            }
            slots.push(slot);
        }
    }
    for slot in &slots {
        let p = slot_target(doc, slot)?;
        check_kind(slot, doc.layer(&p))?;
    }
    let replaced = doc.slots.len();
    doc.slots = slots;
    Ok(json!({"slots": slots_json(doc), "count": doc.slots.len(), "derived": derived, "replaced": replaced}))
}

fn slots(_a: &Args, doc: &mut Document, _ctx: &mut OpCtx) -> Result<Value> {
    if doc.slots.is_empty() {
        return Err(not_a_template());
    }
    let required: Vec<&str> = doc.slots.iter().filter(|s| s.required).map(|s| s.name.as_str()).collect();
    Ok(json!({"template": true, "slots": slots_json(doc), "count": doc.slots.len(), "required": required}))
}

// ---------------------------------------------------------------------------------------
// apply

/// `name=value` pairs of a repeatable flag; a JSON object (`{"headline": "..."}`, how
/// pipelines pass a map) is accepted too.
fn assignments(a: &Args, flag: &str) -> Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    for raw in a.all(flag) {
        let t = raw.trim();
        if t.starts_with('{') {
            if let Ok(Value::Object(m)) = serde_json::from_str::<Value>(t) {
                for (k, v) in m {
                    out.push((
                        k,
                        match v {
                            Value::String(s) => s,
                            other => other.to_string(),
                        },
                    ));
                }
                continue;
            }
        }
        let (k, v) = raw.split_once('=').ok_or_else(|| {
            Error::new("PX1010", format!("--{flag} expects name=value, got '{raw}'")).with_context("param", flag)
        })?;
        let k = k.trim();
        if k.is_empty() {
            return Err(
                Error::new("PX1010", format!("--{flag} expects name=value, got '{raw}'")).with_context("param", flag)
            );
        }
        out.push((k.to_string(), v.to_string()));
    }
    Ok(out)
}

fn flag_for(kind: &str) -> &'static str {
    match kind {
        "image" => "set-image",
        "color" => "set-color",
        _ => "set",
    }
}

fn apply(a: &Args, doc: &mut Document, ctx: &mut OpCtx) -> Result<Value> {
    if doc.slots.is_empty() {
        return Err(not_a_template());
    }
    let names: Vec<String> = doc.slots.iter().map(|s| s.name.clone()).collect();
    // (slot, value), last value wins
    let mut fills: Vec<(String, String)> = Vec::new();
    for kind in KINDS {
        let flag = flag_for(kind);
        for (k, v) in assignments(a, flag)? {
            let Some(slot) = doc.slots.iter().find(|s| s.name == k) else {
                return Err(Error::new("PX3203", format!("the template has no slot named '{k}'"))
                    .with_hint("list them with `pixelox comp template slots <template>`")
                    .with_context("slot", &k)
                    .with_context("slots", &names));
            };
            if slot.kind != *kind {
                return Err(Error::new(
                    "PX3204",
                    format!(
                        "slot '{k}' is a {} slot; fill it with --{} instead of --{flag}",
                        slot.kind,
                        flag_for(&slot.kind)
                    ),
                )
                .with_context("slot", &k)
                .with_context("kind", &slot.kind));
            }
            match fills.iter_mut().find(|f| f.0 == k) {
                Some(f) => f.1 = v,
                None => fills.push((k, v)),
            }
        }
    }
    let missing: Vec<&Slot> = doc.slots.iter().filter(|s| s.required && !fills.iter().any(|f| f.0 == s.name)).collect();
    if !missing.is_empty() {
        let quoted: Vec<String> = missing.iter().map(|s| format!("'{}'", s.name)).collect();
        let msg = if missing.len() == 1 {
            format!("required slot {} is not filled", quoted[0])
        } else {
            format!("required slots {} are not filled", quoted.join(", "))
        };
        let hint: Vec<String> = missing.iter().map(|s| format!("--{} {}=...", flag_for(&s.kind), s.name)).collect();
        return Err(Error::new("PX3202", msg)
            .with_hint(format!("add {}", hint.join(" ")))
            .with_context("missing", missing.iter().map(|s| s.name.clone()).collect::<Vec<_>>()));
    }
    // resolve and check every target before changing anything
    let mut plan: Vec<(usize, LayerPath, String)> = Vec::new();
    for (k, v) in fills {
        let si = doc.slots.iter().position(|s| s.name == k).unwrap_or(0);
        let p = slot_target(doc, &doc.slots[si])?;
        let l = doc.layer(&p);
        check_kind(&doc.slots[si], l)?;
        l.check_unlocked()?;
        if doc.slots[si].kind == "color" {
            color::parse_color(v.trim()).map_err(|e| e.with_context("slot", &k))?;
        }
        plan.push((si, p, v));
    }
    let mut filled = Vec::new();
    for (si, p, v) in &plan {
        let r = match doc.slots[*si].kind.as_str() {
            "text" => fill_text(doc, ctx, p, *si, v)?,
            "image" => fill_image(doc, ctx, p, *si, v.trim())?,
            _ => fill_color(doc, p, *si, v.trim())?,
        };
        filled.push(r);
    }
    let unfilled: Vec<&String> =
        names.iter().filter(|n| !plan.iter().any(|(si, _, _)| doc.slots[*si].name == **n)).collect();
    Ok(json!({"filled": filled, "unfilled": unfilled}))
}

fn fill_text(doc: &mut Document, ctx: &mut OpCtx, p: &LayerPath, si: usize, value: &str) -> Result<Value> {
    let slot = doc.slots[si].clone();
    let shrink = slot.constraints.contains_key("max_lines") || slot.constraints.contains_key("min_size");
    let mut sbox = if shrink { slot_box(&slot) } else { None };
    if shrink && sbox.is_none() {
        match compose::layer_bounds(doc, doc.layer(p)) {
            Ok(b) => sbox = b.map(|r| [r.x as f64, r.y as f64, r.w as f64, r.h as f64]),
            Err(e) => ctx.warn(
                "PXW3002",
                format!("auto-shrink for slot '{}' skipped: the text could not be measured ({})", slot.name, e.message),
            ),
        }
    }
    let l = doc.layer_mut(p);
    let name = l.name.clone();
    if let LayerKind::Text { spec } = &mut l.content {
        if !spec.is_object() {
            *spec = json!({});
        }
        spec["content"] = json!(value);
    }
    let mut out = json!({"slot": slot.name, "kind": "text", "layer": name, "value": value});
    if let Some(b) = sbox {
        doc.slots[si].constraints.insert("box".into(), json!(b));
        if let Some(r) = auto_shrink(doc, ctx, p, &slot, b) {
            out["size"] = r;
        }
    }
    Ok(out)
}

fn unit_suffix(u: Unit) -> &'static str {
    match u {
        Unit::Px => "px",
        Unit::Pct => "%",
        Unit::Pt => "pt",
        Unit::Mm => "mm",
        Unit::In => "in",
    }
}

/// Lower the text spec's `size` until the rendered text fits the slot box (width, and
/// height or `max_lines`), but not below `min_size` (default: half the original size).
/// Returns `{from, to, fits}` when the size changed.
fn auto_shrink(doc: &mut Document, ctx: &mut OpCtx, p: &LayerPath, slot: &Slot, b: [f64; 4]) -> Option<Value> {
    let (dpi, href) = (doc.dpi(), doc.height as f64);
    let base = doc.layer(p).clone();
    let LayerKind::Text { spec } = &base.content else { return None };
    let from = spec.get("size").cloned();
    let (orig, numeric) = match &from {
        Some(Value::String(s)) => (Length::parse(s).ok()?, false),
        Some(Value::Number(n)) => (Length::px(n.as_f64().unwrap_or(0.0)), true),
        _ => {
            ctx.warn("PXW3002", format!("auto-shrink for slot '{}' skipped: the text layer has no size", slot.name));
            return None;
        }
    };
    let orig_px = orig.to_px(href, dpi);
    if orig_px.is_nan() || orig_px <= 0.0 {
        return None;
    }
    let min_px = slot
        .constraints
        .get("min_size")
        .and_then(|v| v.as_str())
        .and_then(|s| Length::parse(s).ok())
        .map(|l| l.to_px(href, dpi))
        .unwrap_or(orig_px * 0.5)
        .clamp(0.1, orig_px);
    let max_lines = slot.constraints.get("max_lines").and_then(|v| v.as_u64());
    let fmt = |px: f64| -> Value {
        let v = ((orig.value * px / orig_px) * 10.0).floor() / 10.0;
        if numeric {
            json!(v)
        } else {
            json!(format!("{v}{}", unit_suffix(orig.unit)))
        }
    };
    let fits = |px: f64| -> Result<bool> {
        let mut l = base.clone();
        if let LayerKind::Text { spec } = &mut l.content {
            spec["size"] = fmt(px);
        }
        let Some(r) = compose::layer_bounds(doc, &l)? else { return Ok(true) };
        let w_ok = r.w as f64 <= b[2] + 1.0;
        let h_ok = match max_lines {
            Some(n) => r.h as f64 <= ((n.max(1) - 1) as f64 * 1.2 + 1.25) * px + 1.0,
            None => r.h as f64 <= b[3] + 1.0,
        };
        Ok(w_ok && h_ok)
    };
    let search = || -> Result<Option<(f64, bool)>> {
        if fits(orig_px)? {
            return Ok(None);
        }
        if !fits(min_px)? {
            return Ok(Some((min_px, false)));
        }
        let (mut lo, mut hi) = (min_px, orig_px);
        for _ in 0..14 {
            let mid = (lo + hi) / 2.0;
            if fits(mid)? {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Ok(Some((lo, true)))
    };
    let (px, ok) = match search() {
        Ok(Some(r)) => r,
        Ok(None) => return None,
        Err(e) => {
            ctx.warn(
                "PXW3002",
                format!("auto-shrink for slot '{}' skipped: the text could not be measured ({})", slot.name, e.message),
            );
            return None;
        }
    };
    let to = fmt(px);
    if !ok {
        ctx.warn("PXW3002", format!("text in slot '{}' still overflows its box at the minimum size {to}", slot.name));
    }
    if let LayerKind::Text { spec } = &mut doc.layer_mut(p).content {
        spec["size"] = to.clone();
    }
    Some(json!({"from": from, "to": to, "fits": ok}))
}

fn fill_image(doc: &mut Document, ctx: &mut OpCtx, p: &LayerPath, si: usize, value: &str) -> Result<Value> {
    let slot = doc.slots[si].clone();
    let fit = slot.fit.clone().unwrap_or_else(|| "cover".into());
    let (img, bytes, ext) = read_image(ctx, value, &slot.name, Some("set-image"))?;
    let (iw, ih) = (img.width as f64, img.height as f64);
    let l = doc.layer(p);
    let b = match slot_box(&slot) {
        Some(b) => b,
        None => match &l.content {
            LayerKind::Placed { rect, .. } => *rect,
            LayerKind::Raster { image } => match compose::layer_bounds(doc, l)? {
                Some(r) => [r.x as f64, r.y as f64, r.w as f64, r.h as f64],
                None if image.is_empty() => [0.0, 0.0, doc.width as f64, doc.height as f64],
                None => [l.x as f64, l.y as f64, image.width as f64, image.height as f64],
            },
            _ => return Err(check_kind(&slot, l).err().unwrap_or_else(|| Error::internal("unexpected layer kind"))),
        },
    };
    let name = l.name.clone();
    let (bx, by) = (b[0].round(), b[1].round());
    let (bw, bh) = (b[2].round().max(1.0), b[3].round().max(1.0));
    let out = if matches!(l.content, LayerKind::Placed { .. }) {
        let rect = fit_rect(iw, ih, bx, by, bw, bh, &fit);
        let clip = rect != [bx, by, bw, bh] && fit == "cover";
        let clip_mask = clip.then(|| {
            let (x0, y0, x1, y1) = (bx as i64, by as i64, (bx + bw) as i64, (by + bh) as i64);
            Mask::from_fn(doc.width, doc.height, |x, y| {
                let (x, y) = (x as i64, y as i64);
                if x >= x0 && x < x1 && y >= y0 && y < y1 {
                    1.0
                } else {
                    0.0
                }
            })
        });
        let key = doc.add_asset(bytes, &ext);
        let l = doc.layer_mut(p);
        if let LayerKind::Placed { asset, path, rect: r, source, .. } = &mut l.content {
            *asset = key.clone();
            *path = value.to_string();
            *r = rect;
            *source = Some(std::sync::Arc::new(img));
        }
        if let Some(m) = clip_mask {
            match &mut l.mask {
                Some(lm) if lm.mask.width == m.width && lm.mask.height == m.height => {
                    for (a, c) in lm.mask.data.iter_mut().zip(&m.data) {
                        *a *= c;
                    }
                }
                _ => l.mask = Some(LayerMask::new(m)),
            }
        }
        json!({"slot": slot.name, "kind": "image", "layer": name, "value": value, "fit": fit, "rect": rect, "clipped": clip, "asset": key})
    } else {
        let image = fit_image(&img, bw as u32, bh as u32, &fit);
        let l = doc.layer_mut(p);
        l.content = LayerKind::Raster { image };
        l.x = bx as i32;
        l.y = by as i32;
        json!({"slot": slot.name, "kind": "image", "layer": name, "value": value, "fit": fit, "offset": [bx, by], "size": [bw, bh]})
    };
    doc.slots[si].constraints.insert("box".into(), json!([bx, by, bw, bh]));
    Ok(out)
}

fn fill_color(doc: &mut Document, p: &LayerPath, si: usize, value: &str) -> Result<Value> {
    let c = color::parse_color(value)?;
    let hex = c.to_hex();
    let slot = doc.slots[si].name.clone();
    let l = doc.layer_mut(p);
    let key = if matches!(l.content, LayerKind::Text { .. }) { "color" } else { "fill" };
    let target = match &mut l.content {
        LayerKind::Text { spec } | LayerKind::Shape { spec } => {
            if !spec.is_object() {
                *spec = json!({});
            }
            spec[key] = json!(hex);
            key
        }
        LayerKind::Raster { image } => {
            image.map_pixels(|px| {
                px[0] = c.r;
                px[1] = c.g;
                px[2] = c.b;
                px[3] *= c.a;
            });
            "pixels"
        }
        _ => return Err(Error::new("PX2303", format!("color slot '{slot}' cannot fill layer '{}'", l.name))),
    };
    Ok(json!({"slot": slot, "kind": "color", "layer": l.name, "value": hex, "target": target}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testio::{encode, run_io, TestIo};
    use pixelox_core::testing::{self, find, run, run_with_warnings, TempDir};
    use std::sync::Arc;

    fn op(name: &str) -> OpSpec {
        find(&ops(), name)
    }

    /// A stand-in text renderer: a block 0.5·size wide per character and 1.2·size per line
    /// (1·size for the last), top-left at the spec's x/y. `"fail": true` makes it error.
    fn fake_text(l: &Layer, doc: &Document) -> Result<Option<Rendered>> {
        let LayerKind::Text { spec } = &l.content else { return Ok(None) };
        if spec.get("fail").is_some() {
            return Err(Error::runtime("no font"));
        }
        let px = match spec.get("size") {
            Some(Value::String(s)) => Length::parse(s)?.to_px(doc.height as f64, doc.dpi()),
            Some(Value::Number(n)) => n.as_f64().unwrap_or(10.0),
            _ => 10.0,
        };
        let content = spec.get("content").and_then(|v| v.as_str()).unwrap_or("");
        let lines: Vec<&str> = content.split('\n').collect();
        let chars = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
        let w = (chars as f64 * px * 0.5).ceil() as u32;
        let h = ((lines.len() - 1) as f64 * px * 1.2 + px).ceil() as u32;
        if w == 0 || h == 0 {
            return Ok(None);
        }
        let x = spec.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0) as i32;
        let y = spec.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0) as i32;
        Ok(Some(Rendered { image: Image::filled(w, h, Rgba::BLACK), x, y }))
    }

    /// 200×100 poster: Background, Product (placed 40×40 at 10,10), Accent (shape),
    /// Headline (text), Swatch (raster 10×10 at 150,80).
    fn poster() -> Document {
        let mut d = testing::doc_from(Image::filled(200, 100, Rgba::WHITE));
        let mut add = |name: &str, kind: LayerKind| {
            let l = d.new_layer(name, kind);
            d.insert_above(None, l);
        };
        add(
            "Product",
            LayerKind::Placed {
                asset: "a".into(),
                path: "placeholder.png".into(),
                linked: false,
                rect: [10.0, 10.0, 40.0, 40.0],
                angle: 0.0,
                source: Some(Arc::new(Image::filled(4, 4, Rgba::BLACK))),
            },
        );
        add("Accent", LayerKind::Shape { spec: json!({"d": "M 0 0 L 10 0 L 10 10 Z", "fill": "#000000"}) });
        add(
            "Headline",
            LayerKind::Text {
                spec: json!({"content": "HEADLINE", "size": "20px", "x": 60, "y": 10, "color": "#111111"}),
            },
        );
        add("Swatch", LayerKind::Raster { image: Image::filled(10, 10, Rgba::BLACK) });
        let p = d.resolve("Swatch").unwrap();
        d.layer_mut(&p).x = 150;
        d.layer_mut(&p).y = 80;
        d
    }

    fn template() -> Document {
        let mut d = poster();
        let slots = [
            ("slot", "text:headline:required:layer=Headline"),
            ("slot", "image:product:layer=Product"),
            ("slot", "color:accent:layer=Accent"),
        ];
        run(&op("comp template init"), &slots, &mut d).unwrap();
        d
    }

    fn close(a: [f32; 4], b: [f32; 4]) -> bool {
        a.iter().zip(&b).all(|(x, y)| (x - y).abs() < 1e-3)
    }

    fn spec_of<'a>(d: &'a Document, name: &str) -> &'a Value {
        match &d.layer(&d.resolve(name).unwrap()).content {
            LayerKind::Text { spec } | LayerKind::Shape { spec } => spec,
            _ => panic!("no spec"),
        }
    }

    #[test]
    fn slot_syntax() {
        let s = parse_slot("text:headline:required:max-lines=2:min-size=24pt:layer=Title").unwrap();
        assert_eq!((s.kind.as_str(), s.name.as_str(), s.required), ("text", "headline", true));
        assert_eq!(s.layer.as_deref(), Some("Title"));
        assert_eq!(s.constraints["max_lines"], 2);
        assert_eq!(s.constraints["min_size"], "24pt");
        let s = parse_slot("image:product").unwrap();
        assert_eq!(s.fit.as_deref(), Some("cover"));
        assert!(!s.required);
        assert_eq!(parse_slot("image:product:fit=contain").unwrap().fit.as_deref(), Some("contain"));
        for bad in [
            "text",
            "video:x",
            "text:x:fit=cover",
            "image:x:fit=stretch",
            "image:x:max-lines=2",
            "text:x:max-lines=0",
            "text:x:min-size=big",
            "text:x:bogus",
            "text:",
        ] {
            assert_eq!(parse_slot(bad).unwrap_err().code, "PX1010", "{bad}");
        }
    }

    #[test]
    fn init_declares_and_validates_slots() {
        let mut d = poster();
        let v = run(
            &op("comp template init"),
            &[("slot", "text:Headline:required"), ("slot", "image:product:layer=Product")],
            &mut d,
        )
        .unwrap();
        assert_eq!(v["count"], 2);
        assert_eq!(v["slots"][0]["layer"], "Headline");
        assert_eq!(v["slots"][1]["layer"], "Product");
        assert_eq!(d.slots.len(), 2);
        let e = run(&op("comp template init"), &[("slot", "text:headline")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX2301");
        let e = run(&op("comp template init"), &[("slot", "text:Product")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX2303");
        let e = run(&op("comp template init"), &[("slot", "image:x:layer=Accent")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX2303");
        let e =
            run(&op("comp template init"), &[("slot", "color:Accent"), ("slot", "color:Accent")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX1010");
        assert_eq!(d.slots.len(), 2, "failed inits change nothing");
        // no --slot: derive from text and placed layers
        let v = run(&op("comp template init"), &[], &mut d).unwrap();
        assert_eq!(v["derived"], true);
        let kinds: Vec<(String, String)> = d.slots.iter().map(|s| (s.name.clone(), s.kind.clone())).collect();
        assert_eq!(kinds, [("Headline".into(), "text".into()), ("Product".into(), "image".into())]);
        let mut plain = testing::doc_from(Image::new(2, 2));
        assert_eq!(run(&op("comp template init"), &[], &mut plain).unwrap_err().code, "PX1003");
    }

    #[test]
    fn slots_lists_or_fails_on_plain_documents() {
        let mut d = template();
        let v = run(&op("comp template slots"), &[], &mut d).unwrap();
        assert_eq!(v["required"], json!(["headline"]));
        assert_eq!(v["slots"][0]["current"], "HEADLINE");
        assert_eq!(v["slots"][1]["current"]["path"], "placeholder.png");
        assert_eq!(v["slots"][2]["current"], "#000000");
        let mut plain = poster();
        assert_eq!(run(&op("comp template slots"), &[], &mut plain).unwrap_err().code, "PX3201");
        assert!(op("comp template slots").is_read_only());
    }

    #[test]
    fn apply_errors() {
        let mut plain = poster();
        assert_eq!(run(&op("comp template apply"), &[("set", "headline=x")], &mut plain).unwrap_err().code, "PX3201");
        let mut d = template();
        let e = run(&op("comp template apply"), &[("set-color", "accent=#ff0000")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX3202");
        assert!(e.message.contains("'headline'"), "{}", e.message);
        let e = run(&op("comp template apply"), &[("set", "headline=x"), ("set", "subtitle=y")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX3203");
        let e = run(&op("comp template apply"), &[("set", "headline=x"), ("set", "accent=red")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX3204");
        let e = run(&op("comp template apply"), &[("set-image", "headline=a.png")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX3204");
        let e = run(&op("comp template apply"), &[("set", "headline")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX1010");
        let e = run(&op("comp template apply"), &[("set", "headline=x"), ("set-color", "accent=notacolor")], &mut d)
            .unwrap_err();
        assert_eq!(e.code, "PX1012");
        // a removed target layer
        let p = d.resolve("Accent").unwrap();
        d.remove(&p);
        let e =
            run(&op("comp template apply"), &[("set", "headline=x"), ("set-color", "accent=red")], &mut d).unwrap_err();
        assert_eq!(e.code, "PX2301");
        let p = d.resolve("Headline").unwrap();
        d.layer_mut(&p).locked = true;
        assert_eq!(run(&op("comp template apply"), &[("set", "headline=x")], &mut d).unwrap_err().code, "PX2304");
        assert_eq!(spec_of(&d, "Headline")["content"], "HEADLINE", "nothing changed");
    }

    #[test]
    fn apply_fills_text_color_and_placed_image() {
        let dir = TempDir::new("tpl");
        let shot = dir.path("hero.pxt");
        std::fs::write(&shot, encode(&Image::filled(80, 40, Rgba::new(1.0, 0.0, 0.0, 1.0)))).unwrap();
        let mut d = template();
        let shot_s = shot.display().to_string();
        let pairs = [
            ("set", "headline=FALL DROP 09"),
            ("set-image", &*format!("product={shot_s}")),
            ("set-color", "accent=#0ea5e9"),
        ];
        let (v, _) = run_io(&op("comp template apply"), &pairs, &mut d, &TestIo::default()).unwrap();
        assert_eq!(v["filled"].as_array().unwrap().len(), 3);
        assert_eq!(spec_of(&d, "Headline")["content"], "FALL DROP 09");
        assert_eq!(spec_of(&d, "Accent")["fill"], "#0ea5e9");
        let l = d.layer(&d.resolve("Product").unwrap());
        let LayerKind::Placed { rect, path, .. } = &l.content else { panic!() };
        // cover: 80×40 into the 40×40 box → 80×40 centered, clipped to the box
        assert_eq!(*rect, [-10.0, 10.0, 80.0, 40.0]);
        assert_eq!(path, &shot_s);
        let m = &l.mask.as_ref().unwrap().mask;
        assert_eq!((m.get(10, 10), m.get(9, 10), m.get(49, 49), m.get(50, 49)), (1.0, 0.0, 1.0, 0.0));
        // composite only the background and the product (no shape/text renderer here)
        let ids: Vec<String> =
            ["Background", "Product"].iter().map(|n| d.layer(&d.resolve(n).unwrap()).id.clone()).collect();
        let c = compose::composite_subset(&d, &ids).unwrap();
        assert!(close(c.get(5, 20), [1.0, 1.0, 1.0, 1.0]), "overflow is clipped");
        assert!(close(c.get(20, 20), [1.0, 0.0, 0.0, 1.0]));
        assert_eq!(d.slots[1].constraints["box"], json!([10.0, 10.0, 40.0, 40.0]));
        // the output stays a template; re-applying fills the same box
        let (v, _) = run_io(&op("comp template apply"), &pairs, &mut d, &TestIo::default()).unwrap();
        assert_eq!(v["filled"][1]["rect"], json!([-10.0, 10.0, 80.0, 40.0]));
    }

    #[test]
    fn apply_image_into_raster_and_color_raster_and_text() {
        let dir = TempDir::new("tpl-raster");
        let shot = dir.path("shot.pxt");
        std::fs::write(&shot, encode(&Image::filled(20, 10, Rgba::new(0.0, 1.0, 0.0, 1.0)))).unwrap();
        let mut d = poster();
        let slots = [("slot", "image:photo:fit=contain:layer=Swatch"), ("slot", "color:ink:layer=Headline")];
        run(&op("comp template init"), &slots, &mut d).unwrap();
        let shot_s = format!("photo={}", shot.display());
        run_io(
            &op("comp template apply"),
            &[("set-image", &*shot_s), ("set-color", "ink=teal")],
            &mut d,
            &TestIo::default(),
        )
        .unwrap();
        let l = d.layer(&d.resolve("Swatch").unwrap());
        let img = l.raster().unwrap();
        assert_eq!((l.x, l.y, img.width, img.height), (150, 80, 10, 10));
        assert_eq!(img.get(5, 0)[3], 0.0, "contain pads");
        assert!(close(img.get(5, 5), [0.0, 1.0, 0.0, 1.0]));
        assert_eq!(spec_of(&d, "Headline")["color"], "#008080");
        // recolor a raster layer, keeping its alpha
        let mut d = poster();
        let p = d.resolve("Swatch").unwrap();
        d.layer_mut(&p).raster_mut().unwrap().set(0, 0, [0.0, 0.0, 0.0, 0.25]);
        run(&op("comp template init"), &[("slot", "color:Swatch")], &mut d).unwrap();
        run(&op("comp template apply"), &[("set-color", "Swatch=#ff0000")], &mut d).unwrap();
        let img = testing::raster(&d, "Swatch");
        assert_eq!(img.get(0, 0), [1.0, 0.0, 0.0, 0.25]);
        assert_eq!(img.get(5, 5), [1.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn apply_accepts_pipeline_maps() {
        let mut d = template();
        run(
            &op("comp template apply"),
            &[("set", r#"{"headline": "WINTER DROP 01"}"#), ("set-color", r#"{"accent": "red"}"#)],
            &mut d,
        )
        .unwrap();
        assert_eq!(spec_of(&d, "Headline")["content"], "WINTER DROP 01");
        assert_eq!(spec_of(&d, "Accent")["fill"], "#ff0000");
    }

    #[test]
    fn auto_shrink_fits_the_original_box() {
        compose::set_text_renderer(fake_text);
        let mut d = poster();
        // HEADLINE at 20px: 8 chars → 80×20 box at (60,10)
        run(&op("comp template init"), &[("slot", "text:Headline:min-size=5px")], &mut d).unwrap();
        let (v, w) =
            run_with_warnings(&op("comp template apply"), &[("set", "Headline=FALL DROP 09 SALE")], &mut d).unwrap();
        assert!(w.is_empty(), "{w:?}");
        // 17 chars fit 80 (+1) px wide at 9.5 px: ceil(17·0.5·9.5) = 81
        assert_eq!(v["filled"][0]["size"]["to"], "9.5px");
        assert_eq!(spec_of(&d, "Headline")["size"], "9.5px");
        assert_eq!(d.slots[0].constraints["box"], json!([60.0, 10.0, 80.0, 20.0]));
        // too long even at the minimum: clamped with a warning
        let (v, w) = run_with_warnings(
            &op("comp template apply"),
            &[("set", "Headline=AN EXTREMELY LONG HEADLINE THAT NEVER FITS IN EIGHTY PX")],
            &mut d,
        )
        .unwrap();
        assert_eq!(v["filled"][0]["size"]["to"], "5px");
        assert_eq!(w, ["PXW3002"]);
        // short text needs no shrink
        let (v, _) = run_with_warnings(&op("comp template apply"), &[("set", "Headline=HI")], &mut d).unwrap();
        assert!(v["filled"][0].get("size").is_none());
        // max-lines: two lines of 20 px fit 2 lines; three lines are shrunk
        let mut d = poster();
        run(&op("comp template init"), &[("slot", "text:Headline:max-lines=2")], &mut d).unwrap();
        let (v, _) = run_with_warnings(&op("comp template apply"), &[("set", "Headline=AB\nCD")], &mut d).unwrap();
        assert!(v["filled"][0].get("size").is_none());
        let (v, _) = run_with_warnings(&op("comp template apply"), &[("set", "Headline=AB\nCD\nEF")], &mut d).unwrap();
        assert_eq!(v["filled"][0]["size"]["fits"], false);
    }

    #[test]
    fn auto_shrink_is_skipped_with_a_warning_when_text_cannot_render() {
        compose::set_text_renderer(fake_text);
        let mut d = poster();
        let p = d.resolve("Headline").unwrap();
        if let LayerKind::Text { spec } = &mut d.layer_mut(&p).content {
            spec["fail"] = json!(true);
        }
        run(&op("comp template init"), &[("slot", "text:Headline:max-lines=1")], &mut d).unwrap();
        let (_, w) =
            run_with_warnings(&op("comp template apply"), &[("set", "Headline=A MUCH LONGER HEADLINE")], &mut d)
                .unwrap();
        assert_eq!(w, ["PXW3002"]);
        assert_eq!(spec_of(&d, "Headline")["content"], "A MUCH LONGER HEADLINE");
        assert_eq!(spec_of(&d, "Headline")["size"], "20px");
    }
}
