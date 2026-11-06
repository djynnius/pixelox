//! Op specs: the single definition of every command. The CLI, JSON Schemas, pipeline
//! steps, MCP tools, `describe --agent`, and the docs are all generated from these, so a
//! flag's name and help text are written exactly once (ADR 0001).

use crate::color::{self, Rgba, Stop};
use crate::ctx::OpCtx;
use crate::doc::Document;
use crate::error::{Error, Result};
use crate::units::{self, Length, PointSpec, RectSpec, SizeSpec};
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// The value type of a parameter. Every value arrives as a string and is checked by the
/// shared parser, so CLI flags, pipeline YAML, and MCP JSON accept the same text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamKind {
    /// Presence flag (`true`/`false`/`on`/`off` in pipelines and MCP).
    Flag,
    /// `on`/`off`.
    Switch,
    Int,
    Float,
    Str,
    Path,
    /// One of a fixed set of words.
    Choice,
    /// Comma-separated list.
    List,
    /// `#RRGGBB[AA]`, `rgb()`, `hsl()`, CSS name, `transparent`.
    Color,
    /// `X,Y,WxH`.
    Rect,
    /// `X,Y`.
    Point,
    /// `WxH` (`2048x` keeps aspect).
    Size,
    /// `12`, `12px`, `50%`, `3mm`, `0.5in`, `12pt`.
    Length,
    /// Degrees.
    Angle,
    /// Gradient stops `#000@0,#fff@1`.
    Stops,
    /// Point list `x1,y1 x2,y2 ...`.
    Points,
    /// `a..b`.
    Range,
    /// Layer reference: name, `#N`, `@top`, `@bottom`, `@active`.
    Layer,
    /// Mask reference: mask name, `@sel`, `@alpha:<layer>`, `@mask:<layer>` (a layer mask), or a grayscale image path.
    Mask,
    /// `1.5s`, `500ms`.
    Duration,
}

impl ParamKind {
    pub fn type_name(self) -> &'static str {
        match self {
            ParamKind::Flag => "flag",
            ParamKind::Switch => "on|off",
            ParamKind::Int => "integer",
            ParamKind::Float => "number",
            ParamKind::Str => "text",
            ParamKind::Path => "path",
            ParamKind::Choice => "choice",
            ParamKind::List => "list",
            ParamKind::Color => "color",
            ParamKind::Rect => "X,Y,WxH",
            ParamKind::Point => "X,Y",
            ParamKind::Size => "WxH",
            ParamKind::Length => "length",
            ParamKind::Angle => "degrees",
            ParamKind::Stops => "stops",
            ParamKind::Points => "points",
            ParamKind::Range => "a..b",
            ParamKind::Layer => "layer",
            ParamKind::Mask => "mask",
            ParamKind::Duration => "duration",
        }
    }

    pub fn example(self) -> Option<&'static str> {
        Some(match self {
            ParamKind::Color => "#0ea5e9",
            ParamKind::Rect => "10,20,300x200",
            ParamKind::Point => "50%,120",
            ParamKind::Size => "1080x1350",
            ParamKind::Length => "12px",
            ParamKind::Angle => "45",
            ParamKind::Stops => "#000@0,#fff@1",
            ParamKind::Points => "0,0 128,150 255,255",
            ParamKind::Range => "0.2..0.8",
            ParamKind::Layer => "@top",
            ParamKind::Mask => "@sel",
            ParamKind::Duration => "1.5s",
            _ => return None,
        })
    }
}

/// Definition of one parameter.
#[derive(Clone, Debug, Serialize)]
pub struct ParamSpec {
    /// kebab-case; the CLI flag is `--<name>`, pipeline and MCP keys use underscores.
    pub name: &'static str,
    pub kind: ParamKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<&'static str>,
    pub help: &'static str,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub choices: &'static [&'static str],
    pub repeatable: bool,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short: Option<char>,
    /// Inclusive numeric range, e.g. `"0..100"`; enforced for Int and Float (PX1013).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<&'static str>,
    /// Also accepts the word `auto`.
    pub auto: bool,
    /// Hidden from the compact agent catalog.
    #[serde(skip)]
    pub advanced: bool,
}

/// Start a parameter: `param("sigma", ParamKind::Float).default("2").help("...")`.
pub fn param(name: &'static str, kind: ParamKind) -> ParamSpec {
    ParamSpec {
        name,
        kind,
        default: None,
        help: "",
        choices: &[],
        repeatable: false,
        required: false,
        short: None,
        range: None,
        auto: false,
        advanced: false,
    }
}

impl ParamSpec {
    pub fn default(mut self, d: &'static str) -> Self {
        self.default = Some(d);
        self
    }
    pub fn help(mut self, h: &'static str) -> Self {
        self.help = h;
        self
    }
    pub fn choices(mut self, c: &'static [&'static str]) -> Self {
        self.choices = c;
        self
    }
    pub fn repeatable(mut self) -> Self {
        self.repeatable = true;
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn short(mut self, c: char) -> Self {
        self.short = Some(c);
        self
    }
    pub fn range(mut self, r: &'static str) -> Self {
        self.range = Some(r);
        self
    }
    pub fn auto(mut self) -> Self {
        self.auto = true;
        self
    }
    pub fn advanced(mut self) -> Self {
        self.advanced = true;
        self
    }
    /// Pipeline/JSON key (`in-black` → `in_black`).
    pub fn key(&self) -> String {
        self.name.replace('-', "_")
    }

    /// Check a raw value without needing a document.
    pub fn validate(&self, raw: &str) -> Result<()> {
        let fail = |e: Error| -> Error {
            let mut e = e;
            e.message = format!("--{}: {}", self.name, e.message);
            e
        };
        if self.auto && raw.trim() == "auto" {
            return Ok(());
        }
        let r = match self.kind {
            ParamKind::Flag | ParamKind::Switch => parse_bool(raw).map(|_| ()),
            ParamKind::Int => raw
                .trim()
                .parse::<i64>()
                .map_err(|_| Error::new("PX1010", format!("'{raw}' is not an integer")))
                .and_then(|v| self.check_range(v as f64)),
            ParamKind::Float => raw
                .trim()
                .parse::<f64>()
                .ok()
                .filter(|v| v.is_finite())
                .ok_or_else(|| Error::new("PX1010", format!("'{raw}' is not a number")))
                .and_then(|v| self.check_range(v)),
            ParamKind::Str | ParamKind::Path | ParamKind::List | ParamKind::Layer | ParamKind::Mask => Ok(()),
            ParamKind::Choice => {
                if self.choices.is_empty() || self.choices.contains(&raw.trim()) {
                    Ok(())
                } else {
                    Err(Error::new("PX1010", format!("'{raw}' is not one of: {}", self.choices.join(", "))))
                }
            }
            ParamKind::Color => color::parse_color(raw).map(|_| ()),
            ParamKind::Rect => RectSpec::parse(raw).map(|_| ()),
            ParamKind::Point => PointSpec::parse(raw).map(|_| ()),
            ParamKind::Size => SizeSpec::parse(raw).map(|_| ()),
            ParamKind::Length => Length::parse(raw).map(|_| ()),
            ParamKind::Angle => units::parse_angle(raw).map(|_| ()),
            ParamKind::Stops => color::parse_stops(raw).map(|_| ()),
            ParamKind::Points => units::parse_points(raw).map(|_| ()),
            ParamKind::Range => units::parse_range(raw).map(|_| ()),
            ParamKind::Duration => units::parse_duration_s(raw).map(|_| ()),
        };
        r.map_err(fail)
    }

    fn check_range(&self, v: f64) -> Result<()> {
        if let Some(r) = self.range {
            if let Ok((lo, hi)) = units::parse_range(r) {
                if v < lo || v > hi {
                    return Err(Error::new("PX1013", format!("{v} is outside {r}"))
                        .with_context("param", self.name)
                        .with_context("range", r));
                }
            }
        }
        Ok(())
    }
}

/// Command categories: the sections of the docs and the agent catalog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Document,
    Layer,
    Select,
    Mask,
    Detect,
    Adjust,
    Filter,
    Retouch,
    Paint,
    Text,
    Vector,
    Transform,
    Fx,
    Comp,
    Svg,
    Render,
    Frames,
    Analyze,
    Automation,
    Models,
    Agents,
    System,
}

impl Category {
    pub fn title(self) -> &'static str {
        match self {
            Category::Document => "Documents and files",
            Category::Layer => "Layers",
            Category::Select => "Selections",
            Category::Mask => "Masks",
            Category::Detect => "AI detection",
            Category::Adjust => "Adjustments",
            Category::Filter => "Filters",
            Category::Retouch => "Retouch and fill",
            Category::Paint => "Painting",
            Category::Text => "Type",
            Category::Vector => "Shapes and paths",
            Category::Transform => "Transform",
            Category::Fx => "Layer styles",
            Category::Comp => "Composition and templates",
            Category::Svg => "SVG and tracing",
            Category::Render => "Generative rendering",
            Category::Frames => "Frames",
            Category::Analyze => "Analysis",
            Category::Automation => "Pipelines, batches, jobs",
            Category::Models => "Models, fonts, config",
            Category::Agents => "Agent contract",
            Category::System => "System",
        }
    }
}

/// How many positional input files a command takes.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
pub struct Inputs {
    pub min: usize,
    /// `None` = unbounded.
    pub max: Option<usize>,
}

impl Inputs {
    pub const NONE: Inputs = Inputs { min: 0, max: Some(0) };
    pub const ONE: Inputs = Inputs { min: 1, max: Some(1) };
    pub const OPTIONAL: Inputs = Inputs { min: 0, max: Some(1) };
    pub const MANY: Inputs = Inputs { min: 1, max: None };
    pub fn exactly(n: usize) -> Inputs {
        Inputs { min: n, max: Some(n) }
    }
}

/// A worked example, shown in help and docs and validated against the CLI in tests.
#[derive(Clone, Debug, Serialize)]
pub struct Example {
    pub cmd: &'static str,
    pub explain: &'static str,
}

/// A document op: the executor loads INPUT (image, PSD, SVG, or .pxo) into a [`Document`],
/// runs the op, and writes the result (`-o` or `--in-place`). The return value becomes part
/// of `result` in the envelope.
pub type DocFn = fn(&Args, &mut Document, &mut OpCtx) -> Result<Value>;
/// A source op creates a document (`new`, `open`); the executor writes it like a doc op.
pub type SourceFn = fn(&Args, &mut OpCtx) -> Result<Document>;
/// A custom op reads its own inputs and writes its own outputs through the context.
pub type CustomFn = fn(&Args, &mut OpCtx) -> Result<Value>;

#[derive(Clone, Copy)]
pub enum Runner {
    Doc(DocFn),
    Source(SourceFn),
    Custom(CustomFn),
}

impl std::fmt::Debug for Runner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Runner::Doc(_) => write!(f, "Doc"),
            Runner::Source(_) => write!(f, "Source"),
            Runner::Custom(_) => write!(f, "Custom"),
        }
    }
}

/// What a doc op does with the document.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DocMode {
    /// Changes the document; needs `-o` or `--in-place` (the default).
    Mutate,
    /// Inspects only; writes nothing unless it writes its own side file.
    ReadOnly,
    /// Renders deliverables itself (`export`); never rewrites the document.
    Export,
}

/// The definition of one command.
#[derive(Clone, Debug, Serialize)]
pub struct OpSpec {
    /// Space-separated command path: `"filter gaussian"`, `"comp template apply"`.
    pub name: &'static str,
    pub category: Category,
    /// One line: what it does (kept short for `describe --agent`).
    pub summary: &'static str,
    /// Longer explanation for people.
    pub about: &'static str,
    pub params: Vec<ParamSpec>,
    pub inputs: Inputs,
    /// Parameters filled, in order, by positional words after the input files
    /// (`pixelox select load doc.pxo wires`, `pixelox models pull lama`).
    pub positionals: Vec<&'static str>,
    pub examples: Vec<Example>,
    pub aliases: Vec<&'static str>,
    pub mode: DocMode,
    /// A mutating command that may run without `-o`/`--in-place` because it can write side
    /// files instead (`detect subject --cutout`); the command checks it has somewhere to write.
    #[serde(skip)]
    pub target_optional: bool,
    /// Milestone tag when the command is gated or partial in this build (`"v0.2"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub milestone: Option<&'static str>,
    #[serde(skip)]
    pub runner: Runner,
}

impl OpSpec {
    fn new(name: &'static str, category: Category, summary: &'static str, runner: Runner, inputs: Inputs) -> OpSpec {
        OpSpec {
            name,
            category,
            summary,
            about: "",
            params: Vec::new(),
            inputs,
            positionals: Vec::new(),
            examples: Vec::new(),
            aliases: Vec::new(),
            mode: DocMode::Mutate,
            target_optional: false,
            milestone: None,
            runner,
        }
    }

    pub fn doc(name: &'static str, category: Category, summary: &'static str, f: DocFn) -> OpSpec {
        OpSpec::new(name, category, summary, Runner::Doc(f), Inputs::ONE)
    }

    pub fn source(name: &'static str, category: Category, summary: &'static str, f: SourceFn) -> OpSpec {
        OpSpec::new(name, category, summary, Runner::Source(f), Inputs::NONE)
    }

    pub fn custom(name: &'static str, category: Category, summary: &'static str, f: CustomFn) -> OpSpec {
        OpSpec::new(name, category, summary, Runner::Custom(f), Inputs::ONE)
    }

    pub fn about(mut self, a: &'static str) -> Self {
        self.about = a;
        self
    }
    pub fn param(mut self, p: ParamSpec) -> Self {
        self.params.push(p);
        self
    }
    pub fn params(mut self, ps: impl IntoIterator<Item = ParamSpec>) -> Self {
        self.params.extend(ps);
        self
    }
    pub fn inputs(mut self, i: Inputs) -> Self {
        self.inputs = i;
        self
    }
    /// Positional words after the inputs fill these params in order.
    pub fn positional(mut self, name: &'static str) -> Self {
        self.positionals.push(name);
        self
    }
    pub fn example(mut self, cmd: &'static str, explain: &'static str) -> Self {
        self.examples.push(Example { cmd, explain });
        self
    }
    pub fn alias(mut self, a: &'static str) -> Self {
        self.aliases.push(a);
        self
    }
    pub fn read_only(mut self) -> Self {
        self.mode = DocMode::ReadOnly;
        self
    }
    pub fn export_mode(mut self) -> Self {
        self.mode = DocMode::Export;
        self
    }
    /// See [`OpSpec::target_optional`].
    pub fn target_optional(mut self) -> Self {
        self.target_optional = true;
        self
    }
    pub fn milestone(mut self, m: &'static str) -> Self {
        self.milestone = Some(m);
        self
    }

    /// Canonical dotted id (`filter.gaussian`): used in envelopes, sidecars, pipelines,
    /// schemas, and MCP.
    pub fn id(&self) -> String {
        self.name.replace(' ', ".")
    }

    pub fn is_doc(&self) -> bool {
        matches!(self.runner, Runner::Doc(_))
    }

    /// True when the command never writes (inspects only).
    pub fn is_read_only(&self) -> bool {
        match self.runner {
            Runner::Doc(_) => self.mode == DocMode::ReadOnly,
            _ => false,
        }
    }

    pub fn find_param(&self, name: &str) -> Option<&ParamSpec> {
        let norm = name.replace('_', "-");
        self.params.iter().find(|p| p.name == norm)
    }
}

/// Parse booleans the way every surface accepts them.
pub fn parse_bool(s: &str) -> Result<bool> {
    match s.trim().to_ascii_lowercase().as_str() {
        "" | "on" | "true" | "yes" | "1" => Ok(true),
        "off" | "false" | "no" | "0" => Ok(false),
        other => Err(Error::new("PX1010", format!("'{other}' is not on/off or true/false"))),
    }
}

/// Resolved arguments: raw strings keyed by kebab-case name, defaults filled in. Typed
/// accessors use the shared parser.
#[derive(Clone, Debug, Default)]
pub struct Args {
    values: BTreeMap<String, Vec<String>>,
    explicit: Vec<String>,
}

impl Args {
    pub fn new() -> Args {
        Args::default()
    }

    /// Build args for `spec` from raw pairs (keys may use `_` or `-`): validates names and
    /// values, fills defaults, and checks required params.
    pub fn build(spec: &OpSpec, raw: Vec<(String, String)>) -> Result<Args> {
        let mut args = Args::new();
        for (k, v) in raw {
            let name = k.replace('_', "-");
            let p = spec.params.iter().find(|p| p.name == name).ok_or_else(|| {
                let known: Vec<&str> = spec.params.iter().map(|p| p.name).collect();
                Error::new("PX1002", format!("'{}' does not accept '--{}'", spec.name, name))
                    .with_hint(format!(
                        "accepted: {}; see `pixelox docs {}`",
                        if known.is_empty() { "(no parameters)".to_string() } else { known.join(", ") },
                        spec.id()
                    ))
                    .with_context("param", &name)
            })?;
            p.validate(&v)?;
            let entry = args.values.entry(name.clone()).or_default();
            if p.repeatable {
                entry.push(v);
            } else if p.kind == ParamKind::List && !entry.is_empty() {
                let joined = format!("{},{}", entry[0], v);
                *entry = vec![joined];
            } else {
                *entry = vec![v];
            }
            if !args.explicit.contains(&name) {
                args.explicit.push(name);
            }
        }
        for p in &spec.params {
            if !args.values.contains_key(p.name) {
                if let Some(d) = p.default {
                    args.values.insert(p.name.to_string(), vec![d.to_string()]);
                } else if p.required {
                    return Err(Error::new("PX1003", format!("'{}' requires --{}", spec.name, p.name))
                        .with_hint(format!("pixelox docs {}", spec.id()))
                        .with_context("param", p.name));
                }
            }
        }
        Ok(args)
    }

    pub fn set(&mut self, name: &str, value: impl Into<String>) -> &mut Self {
        self.values.insert(name.replace('_', "-"), vec![value.into()]);
        self
    }

    pub fn with(mut self, name: &str, value: impl Into<String>) -> Self {
        self.set(name, value);
        self
    }

    pub fn remove(&mut self, name: &str) {
        self.values.remove(&name.replace('_', "-"));
    }

    pub fn is_explicit(&self, name: &str) -> bool {
        self.explicit.iter().any(|n| n == name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn raw(&self, name: &str) -> Option<&str> {
        self.values.get(name).and_then(|v| v.last()).map(|s| s.as_str())
    }

    /// Every value of a repeatable param.
    pub fn all(&self, name: &str) -> Vec<String> {
        self.values.get(name).cloned().unwrap_or_default()
    }

    /// Every value, splitting comma lists.
    pub fn list(&self, name: &str) -> Vec<String> {
        self.all(name).iter().flat_map(|v| units::split_list(v)).collect()
    }

    pub fn str(&self, name: &str) -> Option<String> {
        self.raw(name).map(|s| s.to_string())
    }

    pub fn str_or(&self, name: &str, d: &str) -> String {
        self.raw(name).unwrap_or(d).to_string()
    }

    fn need(&self, name: &str) -> Result<&str> {
        self.raw(name).ok_or_else(|| Error::new("PX1003", format!("missing --{name}")).with_context("param", name))
    }

    fn ctx<T>(&self, name: &str, r: Result<T>) -> Result<T> {
        r.map_err(|mut e| {
            if !e.message.starts_with("--") {
                e.message = format!("--{name}: {}", e.message);
            }
            e
        })
    }

    pub fn is_auto(&self, name: &str) -> bool {
        self.raw(name).map(|v| v.trim() == "auto").unwrap_or(false)
    }

    pub fn flag(&self, name: &str) -> bool {
        self.raw(name).map(|v| parse_bool(v).unwrap_or(false)).unwrap_or(false)
    }

    pub fn switch(&self, name: &str, default: bool) -> bool {
        self.raw(name).map(|v| parse_bool(v).unwrap_or(default)).unwrap_or(default)
    }

    pub fn f64(&self, name: &str) -> Result<f64> {
        let v = self.need(name)?;
        self.ctx(name, v.trim().parse::<f64>().map_err(|_| Error::new("PX1010", format!("'{v}' is not a number"))))
    }

    pub fn f32(&self, name: &str) -> Result<f32> {
        self.f64(name).map(|v| v as f32)
    }

    /// `None` when absent or `auto`.
    pub fn opt_f64(&self, name: &str) -> Result<Option<f64>> {
        match self.raw(name) {
            None | Some("auto") => Ok(None),
            Some(_) => self.f64(name).map(Some),
        }
    }

    pub fn int(&self, name: &str) -> Result<i64> {
        let v = self.need(name)?;
        self.ctx(name, v.trim().parse::<i64>().map_err(|_| Error::new("PX1010", format!("'{v}' is not an integer"))))
    }

    pub fn opt_int(&self, name: &str) -> Result<Option<i64>> {
        match self.raw(name) {
            None | Some("auto") => Ok(None),
            Some(_) => self.int(name).map(Some),
        }
    }

    pub fn usize(&self, name: &str) -> Result<usize> {
        let v = self.int(name)?;
        if v < 0 {
            return Err(Error::new("PX1013", format!("--{name} must not be negative")));
        }
        Ok(v as usize)
    }

    pub fn choice(&self, name: &str) -> String {
        self.raw(name).unwrap_or("").trim().to_string()
    }

    pub fn path(&self, name: &str) -> Option<PathBuf> {
        self.raw(name).map(PathBuf::from)
    }

    pub fn color(&self, name: &str) -> Result<Rgba> {
        let v = self.need(name)?;
        self.ctx(name, color::parse_color(v))
    }

    pub fn opt_color(&self, name: &str) -> Result<Option<Rgba>> {
        match self.raw(name) {
            None => Ok(None),
            Some(_) => self.color(name).map(Some),
        }
    }

    pub fn rect(&self, name: &str) -> Result<Option<RectSpec>> {
        match self.raw(name) {
            None => Ok(None),
            Some(v) => self.ctx(name, RectSpec::parse(v)).map(Some),
        }
    }

    pub fn point(&self, name: &str) -> Result<Option<PointSpec>> {
        match self.raw(name) {
            None => Ok(None),
            Some(v) => self.ctx(name, PointSpec::parse(v)).map(Some),
        }
    }

    pub fn size(&self, name: &str) -> Result<Option<SizeSpec>> {
        match self.raw(name) {
            None => Ok(None),
            Some(v) => self.ctx(name, SizeSpec::parse(v)).map(Some),
        }
    }

    pub fn length(&self, name: &str) -> Result<Option<Length>> {
        match self.raw(name) {
            None | Some("auto") => Ok(None),
            Some(v) => self.ctx(name, Length::parse(v)).map(Some),
        }
    }

    /// A length resolved to pixels (`%` of `reference`, physical units through `dpi`).
    pub fn px(&self, name: &str, reference: f64, dpi: f64) -> Result<f64> {
        match self.length(name)? {
            Some(l) => Ok(l.to_px(reference, dpi)),
            None => Err(Error::new("PX1003", format!("missing --{name}"))),
        }
    }

    pub fn angle(&self, name: &str) -> Result<f64> {
        let v = self.need(name)?;
        self.ctx(name, units::parse_angle(v))
    }

    pub fn opt_angle(&self, name: &str) -> Result<Option<f64>> {
        match self.raw(name) {
            None | Some("auto") => Ok(None),
            Some(_) => self.angle(name).map(Some),
        }
    }

    pub fn stops(&self, name: &str) -> Result<Vec<Stop>> {
        let v = self.need(name)?;
        self.ctx(name, color::parse_stops(v))
    }

    pub fn points(&self, name: &str) -> Result<Vec<(f64, f64)>> {
        let v = self.need(name)?;
        self.ctx(name, units::parse_points(v))
    }

    pub fn range(&self, name: &str) -> Result<Option<(f64, f64)>> {
        match self.raw(name) {
            None => Ok(None),
            Some(v) => self.ctx(name, units::parse_range(v)).map(Some),
        }
    }

    pub fn duration(&self, name: &str) -> Result<Option<f64>> {
        match self.raw(name) {
            None => Ok(None),
            Some(v) => self.ctx(name, units::parse_duration_s(v)).map(Some),
        }
    }

    /// Resolved parameters as JSON (envelope `params`, sidecars, recipes).
    pub fn to_json(&self) -> Map<String, Value> {
        let mut m = Map::new();
        for (k, v) in &self.values {
            let key = k.replace('-', "_");
            if v.len() == 1 {
                m.insert(key, Value::String(v[0].clone()));
            } else {
                m.insert(key, Value::Array(v.iter().cloned().map(Value::String).collect()));
            }
        }
        m
    }

    /// Raw key/value pairs (for replay and pipelines).
    pub fn pairs(&self) -> Vec<(String, String)> {
        self.values.iter().flat_map(|(k, vs)| vs.iter().map(move |v| (k.clone(), v.clone()))).collect()
    }
}

/// Convert a JSON object (MCP arguments, pipeline `with:`, recipe params) to raw pairs.
pub fn json_to_pairs(v: &Value) -> Result<Vec<(String, String)>> {
    let mut out = Vec::new();
    let obj = match v {
        Value::Object(o) => o,
        Value::Null => return Ok(out),
        _ => return Err(Error::usage("arguments must be a JSON object")),
    };
    for (k, val) in obj {
        match val {
            Value::Null => {}
            Value::Array(items) => {
                for it in items {
                    out.push((k.clone(), scalar_to_string(it)?));
                }
            }
            other => out.push((k.clone(), scalar_to_string(other)?)),
        }
    }
    Ok(out)
}

pub fn scalar_to_string(v: &Value) -> Result<String> {
    Ok(match v {
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Null => String::new(),
        Value::Array(a) => a.iter().map(scalar_to_string).collect::<Result<Vec<_>>>()?.join(","),
        Value::Object(_) => serde_json::to_string(v)?,
    })
}

/// Reusable parameter definitions so the same flag means the same thing everywhere.
pub mod common {
    use super::*;

    pub fn layer() -> ParamSpec {
        param("layer", ParamKind::Layer)
            .help("Target layer: name, #N (1 = top), @top, @bottom, or @active; required when the document has several layers")
    }
    pub fn mask() -> ParamSpec {
        param("mask", ParamKind::Mask).help(
            "Limit the effect to a mask: a named mask, @sel, @alpha:<layer>, @mask:<layer>, or a grayscale image path",
        )
    }
    pub fn invert_mask() -> ParamSpec {
        param("invert-mask", ParamKind::Flag).help("Apply outside the mask instead of inside it")
    }
    pub fn as_layer() -> ParamSpec {
        param("as-layer", ParamKind::Flag).help("Create a non-destructive adjustment layer instead of changing pixels")
    }
    pub fn name() -> ParamSpec {
        param("name", ParamKind::Str).help("Name for the new layer (default: generated, unique)")
    }
    pub fn linear() -> ParamSpec {
        param("linear", ParamKind::Flag).advanced().help("Process in linear light instead of sRGB-encoded values")
    }
    /// `--layer`, `--mask`, `--invert-mask`.
    pub fn region() -> Vec<ParamSpec> {
        vec![layer(), mask(), invert_mask()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn noop(_: &Args, _: &mut Document, _: &mut OpCtx) -> Result<Value> {
        Ok(Value::Null)
    }

    fn spec() -> OpSpec {
        OpSpec::doc("filter gaussian", Category::Filter, "Gaussian blur", noop)
            .param(param("sigma", ParamKind::Float).default("2").range("0..1000").help("x"))
            .param(param("color", ParamKind::Color).help("x"))
            .param(param("rect", ParamKind::Rect).help("x"))
            .param(param("kind", ParamKind::Choice).choices(&["a", "b"]).default("a").help("x"))
            .param(param("angle", ParamKind::Angle).auto().help("x"))
    }

    #[test]
    fn defaults_explicit_and_typed() {
        let a = Args::build(&spec(), vec![("color".into(), "#ff0000".into()), ("rect".into(), "0,0,10x10".into())])
            .unwrap();
        assert_eq!(a.f64("sigma").unwrap(), 2.0);
        assert_eq!(a.color("color").unwrap().to_hex(), "#ff0000");
        assert!(a.rect("rect").unwrap().is_some());
        assert!(a.is_explicit("color"));
        assert!(!a.is_explicit("sigma"));
        assert_eq!(spec().id(), "filter.gaussian");
    }

    #[test]
    fn rejects_unknown_bad_and_out_of_range() {
        let e = Args::build(&spec(), vec![("nope".into(), "1".into())]).unwrap_err();
        assert_eq!(e.code, "PX1002");
        let e = Args::build(&spec(), vec![("sigma".into(), "-1".into())]).unwrap_err();
        assert_eq!(e.code, "PX1013");
        let e = Args::build(&spec(), vec![("color".into(), "nah".into())]).unwrap_err();
        assert_eq!(e.code, "PX1012");
        assert!(e.message.starts_with("--color"));
        let e = Args::build(&spec(), vec![("kind".into(), "c".into())]).unwrap_err();
        assert_eq!(e.code, "PX1010");
        assert!(Args::build(&spec(), vec![("angle".into(), "auto".into())]).is_ok());
    }

    #[test]
    fn json_pairs() {
        let p =
            json_to_pairs(&serde_json::json!({"sigma": 3, "flag": true, "off": false, "list": ["a", "b"]})).unwrap();
        assert!(p.contains(&("sigma".into(), "3".into())));
        assert!(p.contains(&("flag".into(), "true".into())));
        assert!(p.contains(&("off".into(), "false".into())));
        assert_eq!(p.iter().filter(|(k, _)| k == "list").count(), 2);
    }
}
