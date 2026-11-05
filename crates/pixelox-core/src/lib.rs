//! # pixelox-core
//!
//! The foundation every pixelox crate builds on:
//!
//! * the agent contract: [`envelope`] (the `--json` envelope and provenance sidecars),
//!   [`error`] (stable `PX####` codes and exit codes), [`schema`] (JSON Schemas);
//! * [`op`]: [`OpSpec`], the single definition of a command from which the CLI, schemas,
//!   pipelines, MCP tools, and docs are generated;
//! * [`exec`]: the executor shared by the CLI, MCP, pipelines, batches, and `replay`;
//! * [`doc`] and [`compose`]: the layered document model and its compositor;
//! * [`image`], [`color`], [`units`], [`kernels`], [`blend`]: pixels, color math, the shared
//!   value parser, blur/resample/morphology kernels, and Photoshop's blend modes.

#![allow(clippy::type_complexity, clippy::too_many_arguments)]

pub mod blend;
pub mod color;
pub mod compose;
pub mod ctx;
pub mod doc;
pub mod envelope;
pub mod error;
pub mod exec;
pub mod geom;
pub mod image;
pub mod kernels;
pub mod op;
pub mod paths;
pub mod presets;
pub mod region;
pub mod registry;
pub mod schema;
pub mod testing;
pub mod units;

pub use blend::BlendMode;
pub use color::Rgba;
pub use ctx::{EncodeOptions, Globals, ImageInfo, ImageIo, OpCtx, OpenOptions, Progress};
pub use doc::{Document, Layer, LayerKind, RecipeStep};
pub use envelope::{Envelope, Sidecar, PIXELOX_VERSION};
pub use error::{Error, ErrorKind, Result};
pub use image::{Image, Mask, Rendered};
pub use op::{param, Args, Category, DocMode, Inputs, OpSpec, ParamKind, ParamSpec, Runner};
pub use registry::Registry;

/// Everything an op-defining crate usually needs.
pub mod prelude {
    pub use crate::blend::BlendMode;
    pub use crate::color::{self, Rgba};
    pub use crate::compose;
    pub use crate::ctx::{EncodeOptions, OpCtx};
    pub use crate::doc::{Document, Layer, LayerKind, LayerMask};
    pub use crate::error::{Error, Result};
    pub use crate::geom;
    pub use crate::image::{Image, Mask, Rendered};
    pub use crate::kernels;
    pub use crate::op::{common, param, Args, Category, Inputs, OpSpec, ParamKind};
    pub use crate::region::{apply_to_raster, mix_by_mask, target_layer};
    pub use crate::units::{self, Anchor, RectI};
    pub use serde_json::{json, Map, Value};
}
