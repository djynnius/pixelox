//! The pixelox command-line front end: assembles every crate's ops into one registry,
//! builds the clap tree from it, parses global flags and config, runs the invocation, and
//! prints the envelope.

pub mod batch;
pub mod builtin;
pub mod cli;
pub mod config;
pub mod docgen;
pub mod jobs;
pub mod pipeline;
pub mod progress;

use pixelox_core::compose::{set_smart_filter_hook, SmartFilterHook};
use pixelox_core::ctx::{NoIo, NoProgress};
use pixelox_core::op::{json_to_pairs, ParamKind};
use pixelox_core::{
    Args, Document, Error, Globals, Image, LayerKind, OpCtx, OpSpec, Registry, Result, Runner, SmartFilter,
};
use serde_json::Value;
use std::sync::OnceLock;

/// The registry the smart-filter hook resolves commands in (built once).
fn shared_registry() -> &'static Registry {
    static REG: OnceLock<Registry> = OnceLock::new();
    REG.get_or_init(registry)
}

/// `filter *` and `adjust *` pixel commands can be smart filters when they read no files
/// (no path or extra mask argument given).
fn smart_eligible_spec(spec: &OpSpec, args: &Args) -> bool {
    (spec.name.starts_with("filter ") || spec.name.starts_with("adjust "))
        && matches!(spec.runner, Runner::Doc(_))
        && !spec.is_read_only()
        && spec.params.iter().any(|p| p.name == "layer")
        && !spec
            .params
            .iter()
            .any(|p| (p.kind == ParamKind::Path || (p.kind == ParamKind::Mask && p.name != "mask")) && args.has(p.name))
}

fn smart_eligible(id: &str, args: &Args) -> bool {
    shared_registry().get(id).map(|s| smart_eligible_spec(s, args)).unwrap_or(false)
}

/// Re-run a recorded smart filter on a canvas-sized image (one temporary raster layer).
fn run_smart_filter(f: &SmartFilter, img: &mut Image) -> Result<()> {
    let spec = shared_registry().require(&f.op)?;
    let Runner::Doc(run) = spec.runner else {
        return Err(Error::internal(format!("{} is not a document command", f.op)));
    };
    let args = Args::build(spec, json_to_pairs(&Value::Object(f.params.clone()))?)?;
    let mut doc = Document::from_image(std::mem::take(img));
    let globals = Globals { seed: f.seed, ..Globals::default() };
    let mut ctx = OpCtx::new(spec.name, vec![], globals, &NoIo, &NoProgress);
    ctx.op_index = f.op_index;
    run(&args, &mut doc, &mut ctx)?;
    match doc.layers.pop().map(|l| l.content) {
        Some(LayerKind::Raster { image }) if doc.layers.is_empty() => {
            *img = image;
            Ok(())
        }
        _ => Err(Error::internal(format!("smart filter {} did not leave one raster layer", f.op))),
    }
}

/// Install every renderer hook (text, shapes, adjustments, layer styles, asset decoding).
pub fn install() {
    set_smart_filter_hook(SmartFilterHook { eligible: smart_eligible, run: run_smart_filter });
    pixelox_io::install();
    pixelox_adjust::install();
    pixelox_filter::install();
    pixelox_select::install();
    pixelox_fx::install();
    pixelox_comp::install();
    pixelox_text::install();
    pixelox_vector::install();
    pixelox_ml::install();
    pixelox_render::install();
}

/// Every op from every crate: the CLI, MCP, schemas, docs, and pipelines all use this.
pub fn registry() -> Registry {
    install();
    let mut r = Registry::new();
    r.extend(pixelox_io::ops());
    r.extend(pixelox_comp::ops());
    r.extend(pixelox_select::ops());
    r.extend(pixelox_adjust::ops());
    r.extend(pixelox_filter::ops());
    r.extend(pixelox_text::ops());
    r.extend(pixelox_vector::ops());
    r.extend(pixelox_fx::ops());
    r.extend(pixelox_ml::ops());
    r.extend(pixelox_render::ops());
    r.extend(pixelox_mcp::ops());
    r.extend(builtin::ops());
    r.extend(pipeline::ops());
    r.extend(batch::ops());
    r.extend(jobs::ops());
    r.extend(config::ops());
    r
}

/// Run the CLI with the given arguments (including argv[0]); returns the exit code.
pub fn main_with_args(argv: Vec<String>) -> i32 {
    cli::run(argv)
}
