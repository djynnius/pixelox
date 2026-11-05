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

use pixelox_core::Registry;

/// Install every renderer hook (text, shapes, adjustments, layer styles, asset decoding).
pub fn install() {
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
