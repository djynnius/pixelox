//! JSON Schemas (draft 2020-12) generated from op specs: `pixelox schema <command>`, MCP
//! input validation, and the snapshot-tested public contract (PRD §6.2).

use crate::op::{OpSpec, ParamKind, ParamSpec};
use serde_json::{json, Map, Value};

pub const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";
pub const BASE: &str = "https://djynnius.github.io/pixelox/schema/v1";
/// Per-command schema version (semver); bumped when a command's params or result change.
pub const SCHEMA_VERSION: &str = "1.0.0";

fn param_schema(p: &ParamSpec) -> Value {
    let mut s = Map::new();
    let mut desc = p.help.to_string();
    if let Some(r) = p.range {
        desc.push_str(&format!(" Range: {r}."));
    }
    desc.push_str(&format!(" Type: {}.", p.kind.type_name()));
    match p.kind {
        ParamKind::Flag | ParamKind::Switch => {
            s.insert(
                "oneOf".into(),
                json!([{"type": "boolean"}, {"type": "string", "enum": ["on", "off", "true", "false"]}]),
            );
        }
        ParamKind::Int => {
            let mut int = json!({"type": "integer"});
            if let Some((lo, hi)) = p.range.and_then(|r| crate::units::parse_range(r).ok()) {
                if lo.is_finite() {
                    int["minimum"] = json!(lo);
                }
                if hi.is_finite() {
                    int["maximum"] = json!(hi);
                }
            }
            s.insert("oneOf".into(), json!([int, {"type": "string", "pattern": "^(-?[0-9]+|auto)$"}]));
        }
        ParamKind::Float => {
            let mut num = json!({"type": "number"});
            if let Some((lo, hi)) = p.range.and_then(|r| crate::units::parse_range(r).ok()) {
                if lo.is_finite() {
                    num["minimum"] = json!(lo);
                }
                if hi.is_finite() {
                    num["maximum"] = json!(hi);
                }
            }
            s.insert("oneOf".into(), json!([num, {"type": "string"}]));
        }
        ParamKind::Choice if !p.choices.is_empty() => {
            s.insert("type".into(), json!("string"));
            s.insert("enum".into(), json!(p.choices));
        }
        _ => {
            s.insert("type".into(), json!("string"));
        }
    }
    if let Some(ex) = p.kind.example() {
        s.insert("examples".into(), json!([ex]));
    }
    s.insert("description".into(), json!(desc));
    if let Some(d) = p.default {
        s.insert("default".into(), json!(d));
    }
    s.insert("x-pixelox-kind".into(), json!(p.kind));
    if p.repeatable || p.kind == ParamKind::List {
        let item = Value::Object(s.clone());
        let mut arr = Map::new();
        arr.insert("description".into(), json!(format!("{desc} Repeatable.")));
        arr.insert("oneOf".into(), json!([item, {"type": "array", "items": item}]));
        return Value::Object(arr);
    }
    Value::Object(s)
}

/// Schema of a command's parameters (what MCP `pixelox_run` and pipeline `with:` take).
pub fn params_schema(spec: &OpSpec) -> Value {
    let mut props = Map::new();
    let mut required = Vec::new();
    if spec.inputs.max != Some(0) {
        if spec.inputs.max == Some(1) {
            props.insert(
                "input".into(),
                json!({"type": "string", "description": "Input file: an image, PSD, SVG, or .pxo document"}),
            );
        } else {
            props.insert("inputs".into(), json!({"type": "array", "items": {"type": "string"}, "minItems": spec.inputs.min, "description": "Input files (globs allowed)"}));
        }
        if spec.inputs.min > 0 {
            required.push(if spec.inputs.max == Some(1) { "input" } else { "inputs" }.to_string());
        }
    }
    if !spec.is_read_only() {
        props.insert(
            "output".into(),
            json!({"type": "string", "description": "Output path (-o); format from the extension"}),
        );
        if spec.is_doc() {
            props.insert(
                "in_place".into(),
                json!({"type": "boolean", "description": "Mutate the input .pxo document instead of writing -o"}),
            );
        }
    }
    for p in &spec.params {
        props.insert(p.key(), param_schema(p));
        if p.required && p.default.is_none() {
            required.push(p.key());
        }
    }
    json!({
        "$schema": DIALECT,
        "$id": format!("{BASE}/{}.params.json", spec.id()),
        "title": format!("pixelox {} parameters", spec.id()),
        "description": spec.summary,
        "type": "object",
        "properties": props,
        "required": required,
        "additionalProperties": false,
        "x-pixelox-schema-version": SCHEMA_VERSION,
    })
}

/// Schema of the envelope a command returns.
pub fn result_schema(spec: &OpSpec) -> Value {
    let file = json!({"type": "object", "required": ["path"], "properties": {
        "path": {"type": "string"}, "sha256": {"type": "string"}, "bytes": {"type": "integer"},
        "role": {"type": "string"}, "sidecar": {"type": "string"}, "size": {"type": "array", "items": {"type": "integer"}},
        "format": {"type": "string"}, "action": {"type": "string"}}});
    json!({
        "$schema": DIALECT,
        "$id": format!("{BASE}/{}.result.json", spec.id()),
        "title": format!("pixelox {} envelope", spec.id()),
        "type": "object",
        "required": ["tool", "version", "envelope", "command", "ok", "dry_run", "seed", "inputs", "outputs", "warnings", "timing_ms"],
        "properties": {
            "tool": {"const": "pixelox"},
            "version": {"type": "string"},
            "envelope": {"const": crate::envelope::ENVELOPE_VERSION},
            "command": {"const": spec.id()},
            "ok": {"type": "boolean"},
            "dry_run": {"type": "boolean"},
            "seed": {"type": "integer"},
            "result": {"type": "object", "description": "Command-specific; doc ops include op, params, and document. Dry runs include plan."},
            "inputs": {"type": "array", "items": file},
            "outputs": {"type": "array", "items": file},
            "warnings": {"type": "array", "items": {"type": "object", "required": ["code", "message"], "properties": {
                "code": {"type": "string", "pattern": "^PXW[0-9]{4}$"}, "message": {"type": "string"}, "context": {"type": "object"}}}},
            "error": {"type": "object", "required": ["code", "message", "docs", "exit_code"], "properties": {
                "code": {"type": "string", "pattern": "^PX[0-9]{4}$"}, "message": {"type": "string"}, "hint": {"type": "string"},
                "docs": {"type": "string"}, "context": {"type": "object"}, "exit_code": {"type": "integer"}}},
            "timing_ms": {"type": "integer"}
        }
    })
}

/// Error codes a command can raise (documentation, not exhaustive at the code level).
pub fn error_codes(spec: &OpSpec) -> Vec<&'static str> {
    let mut v = vec!["PX1002", "PX1003", "PX1010", "PX9001"];
    if spec.inputs.min > 0 {
        v.extend(["PX1101", "PX1103", "PX1105"]);
    }
    if !spec.is_read_only() {
        v.extend(["PX1004", "PX1106", "PX1401", "PX1403"]);
    }
    if spec.is_doc() {
        v.extend(["PX1108", "PX2301", "PX2302"]);
        if spec.params.iter().any(|p| p.kind == ParamKind::Mask) {
            v.extend(["PX2311", "PX2312", "PX2313"]);
        }
        if !spec.is_read_only() {
            v.push("PX2304");
        }
    }
    if spec
        .params
        .iter()
        .any(|p| matches!(p.kind, ParamKind::Rect | ParamKind::Point | ParamKind::Size | ParamKind::Length))
    {
        v.push("PX1011");
    }
    if spec.params.iter().any(|p| matches!(p.kind, ParamKind::Color | ParamKind::Stops)) {
        v.push("PX1012");
    }
    if spec.params.iter().any(|p| p.range.is_some()) {
        v.push("PX1013");
    }
    v.sort();
    v.dedup();
    v
}

/// The document `pixelox schema <command>` prints.
pub fn command_schema(spec: &OpSpec) -> Value {
    json!({
        "command": spec.id(),
        "cli": format!("pixelox {}", spec.name),
        "summary": spec.summary,
        "category": spec.category,
        "mode": spec.mode,
        "milestone": spec.milestone,
        "x-pixelox-schema-version": SCHEMA_VERSION,
        "params": params_schema(spec),
        "result": result_schema(spec),
        "errors": error_codes(spec),
    })
}

/// Validate a JSON argument object against an op (names and values via the shared parser).
pub fn validate_params(spec: &OpSpec, args: &Value) -> crate::Result<()> {
    let pairs = crate::op::json_to_pairs(args)?;
    let filtered: Vec<(String, String)> =
        pairs.into_iter().filter(|(k, _)| !matches!(k.as_str(), "input" | "inputs" | "output" | "in_place")).collect();
    crate::op::Args::build(spec, filtered).map(|_| ())
}
