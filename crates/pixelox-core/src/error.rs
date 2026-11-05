//! Errors with stable `PX####` codes and process exit codes (PRD §6.7, §6.8).
//!
//! Every error carries a `code` from the catalog below, a `message`, a `hint` (the next
//! action, a runnable command when one exists), and `context` (offending values). Codes are
//! append-only: a code's meaning never changes.

use serde::Serialize;
use serde_json::{Map, Value};
use std::fmt;

/// Error classes. Each maps to one exit code (PRD §6.7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// 1: an operation failed at runtime (op error, codec failure, corrupt input).
    Runtime,
    /// 2: usage error: unknown command or flag, invalid parameters.
    Usage,
    /// 3: input missing or unreadable.
    InputMissing,
    /// 4: refusing to overwrite or mutate without `--overwrite` / `--in-place`.
    WriteProtected,
    /// 5: unsupported format or feature in this build.
    Unsupported,
    /// 6: consent required for a model or asset download.
    ConsentRequired,
    /// 7: JS sandbox violation, script error, or resource-limit kill.
    Sandbox,
    /// 8: job error: unknown id, already finished, cancel failed.
    Job,
    /// 9: verification or replay mismatch.
    Verification,
    /// 10: interrupted or timed out.
    Interrupted,
    /// 11: memory budget exceeded.
    Memory,
    /// 12: command present but excluded from this build variant.
    FeatureGate,
}

impl ErrorKind {
    pub fn exit_code(self) -> i32 {
        match self {
            ErrorKind::Runtime => 1,
            ErrorKind::Usage => 2,
            ErrorKind::InputMissing => 3,
            ErrorKind::WriteProtected => 4,
            ErrorKind::Unsupported => 5,
            ErrorKind::ConsentRequired => 6,
            ErrorKind::Sandbox => 7,
            ErrorKind::Job => 8,
            ErrorKind::Verification => 9,
            ErrorKind::Interrupted => 10,
            ErrorKind::Memory => 11,
            ErrorKind::FeatureGate => 12,
        }
    }

    pub fn all() -> &'static [ErrorKind] {
        &[
            ErrorKind::Runtime,
            ErrorKind::Usage,
            ErrorKind::InputMissing,
            ErrorKind::WriteProtected,
            ErrorKind::Unsupported,
            ErrorKind::ConsentRequired,
            ErrorKind::Sandbox,
            ErrorKind::Job,
            ErrorKind::Verification,
            ErrorKind::Interrupted,
            ErrorKind::Memory,
            ErrorKind::FeatureGate,
        ]
    }

    pub fn describe(self) -> &'static str {
        match self {
            ErrorKind::Runtime => "Operation failed at runtime (op error, codec failure, corrupt input)",
            ErrorKind::Usage => "Usage error: unknown command or flag, invalid parameters",
            ErrorKind::InputMissing => "Input missing or unreadable",
            ErrorKind::WriteProtected => "Refusing to overwrite or mutate without --overwrite / --in-place",
            ErrorKind::Unsupported => "Unsupported format or feature in this build",
            ErrorKind::ConsentRequired => "Consent required: a model or asset download is needed and -y is absent",
            ErrorKind::Sandbox => "JS sandbox violation, script error, or resource-limit kill",
            ErrorKind::Job => "Job error: unknown id, already finished, or cancel failed",
            ErrorKind::Verification => "Verification or replay mismatch",
            ErrorKind::Interrupted => "Interrupted or timed out",
            ErrorKind::Memory => "Memory budget exceeded (op aborted cleanly)",
            ErrorKind::FeatureGate => "Command present but excluded from this build variant",
        }
    }
}

/// One entry of the error catalog (`pixelox docs errors`, `docs/errors.md`).
#[derive(Debug, Clone, Copy, Serialize)]
pub struct CatalogEntry {
    pub code: &'static str,
    pub kind: ErrorKind,
    pub title: &'static str,
    pub remedy: &'static str,
}

macro_rules! catalog {
    ($( $code:literal, $kind:ident, $title:literal, $remedy:literal; )*) => {
        /// The full, append-only error catalog.
        pub const CATALOG: &[CatalogEntry] = &[
            $( CatalogEntry { code: $code, kind: ErrorKind::$kind, title: $title, remedy: $remedy }, )*
        ];
    };
}

catalog! {
    // PX10xx — CLI usage and parameters
    "PX1001", Usage, "Unknown command", "Run `pixelox describe --agent` or `pixelox schema` to list commands.";
    "PX1002", Usage, "Unknown flag or parameter", "Run `pixelox docs <command>` for the accepted flags.";
    "PX1003", Usage, "Missing required parameter or input", "Supply the parameter named in the message.";
    "PX1004", Usage, "Mutating command without an output", "Pass `-o <path>`, or `--in-place` when the input is a .pxo document.";
    "PX1005", Usage, "Conflicting flags", "Use only one of the flags named in the message.";
    "PX1010", Usage, "Parameter value fails its schema", "Check the value against `pixelox schema <command>`.";
    "PX1011", Usage, "Invalid geometry", "Geometry is `X,Y,WxH`, `X,Y`, or `WxH`; units px (default), %, pt, mm, in.";
    "PX1012", Usage, "Invalid color", "Colors are `#RRGGBB[AA]`, `rgb(...)`, `hsl(...)`, a CSS name, or `transparent`.";
    "PX1013", Usage, "Value out of range", "Use a value inside the range given in the message.";
    "PX1020", Usage, "Invalid configuration file", "Fix the key named in the message or run `pixelox config path`.";
    // PX11xx–13xx — IO, formats, codecs
    "PX1101", InputMissing, "Input not found", "Check the path; relative paths resolve against the current directory.";
    "PX1102", InputMissing, "Input unreadable", "Check file permissions.";
    "PX1103", Runtime, "Decode failed (corrupt or truncated input)", "Re-export the file from its source application.";
    "PX1104", Runtime, "Image exceeds the pixel limit", "Raise the limit with `--limit-pixels` if the file is trusted.";
    "PX1105", Unsupported, "Unsupported input format", "Convert the file to PNG, JPEG, TIFF, WebP, GIF, BMP, PSD, or SVG.";
    "PX1106", Unsupported, "Unsupported output format", "Choose an output extension this build can write (`pixelox doctor`).";
    "PX1107", Runtime, "Encode failed", "Try another format or bit depth.";
    "PX1108", Usage, "Source images are never edited in place", "Write a new file with `-o`, or convert to a document first: `pixelox open in.jpg -o doc.pxo`.";
    "PX1201", Unsupported, "Unsupported PSD feature", "The feature is rendered from the PSD composite; see the PSD fidelity matrix.";
    "PX1203", Unsupported, "Unsupported PSD color mode", "Convert the PSD to RGB in its source application.";
    "PX1210", Runtime, "Corrupt .pxo document", "Restore the document from its sidecar with `pixelox replay`.";
    "PX1211", Unsupported, "Document written by a newer pixelox", "Upgrade pixelox (`pixelox self-update`).";
    "PX1310", Usage, "Path outside the allowed roots", "Serve with `--allow <dir>` for that directory, or use a path inside an allowed root.";
    // PX14xx — write protection
    "PX1401", WriteProtected, "Output exists and --overwrite was not given", "Re-run with --overwrite, or choose a new path with -o.";
    "PX1402", WriteProtected, "--in-place needs a .pxo input", "Write a new file with -o instead.";
    "PX1403", WriteProtected, "Output path is the same as an input", "Choose a different -o path, or use --in-place on a .pxo document.";
    // PX15xx — models and network
    "PX1501", Usage, "Unknown model", "List models with `pixelox models list`.";
    "PX1502", ConsentRequired, "Model absent; consent required to download it", "Run the `pixelox models pull <id> -y` command in the hint, or pass -y.";
    "PX1503", Runtime, "Model SHA-256 mismatch", "Re-download with `pixelox models pull <id> -y`; a repeated mismatch means a corrupted mirror.";
    "PX1504", Runtime, "Download failed", "Check connectivity and retry.";
    "PX1510", FeatureGate, "ML features are not in this build", "Use a build with the `ml` feature (see `pixelox doctor`).";
    "PX1520", FeatureGate, "Feature not available in this build", "The message names the milestone or the local file that enables it.";
    // PX16xx — provenance
    "PX1601", Verification, "Input hash drift on replay", "Restore the recorded input, map a new one with `--input role=path`, or pass `--allow-input-drift`.";
    "PX1602", Verification, "Output does not match its sidecar", "The file changed after it was written; re-create it with `pixelox replay`.";
    "PX1603", Usage, "Not a pixelox sidecar", "Pass a `<output>.pixelox.json` file.";
    "PX1604", Runtime, "Recipe cannot be replayed", "The message names the step; check that its inputs still exist.";
    // PX2xxx — document model
    "PX2301", Usage, "No such layer", "List layers with `pixelox layer list <doc>`.";
    "PX2302", Usage, "Layer reference is ambiguous or required", "Pass `--layer <name|#N|@top|@bottom|@active>`.";
    "PX2303", Usage, "Wrong layer kind for this operation", "Rasterize first with `pixelox layer rasterize`, or target another layer.";
    "PX2304", Usage, "Layer is locked", "Unlock it with `pixelox layer unlock`.";
    "PX2305", Usage, "Layer name already in use", "Choose another name; layer names are unique.";
    "PX2310", Usage, "Mask name collision", "Choose another name or remove the existing mask with `pixelox mask rm`.";
    "PX2311", Usage, "No such mask", "List masks with `pixelox mask list <doc>`.";
    "PX2312", Usage, "No active selection", "Make one with `pixelox select ...` first.";
    "PX2313", Usage, "Mask size does not match the canvas", "Resize the mask image to the document size.";
    "PX2320", Usage, "Document has no layers", "Add one with `pixelox layer add` or `pixelox comp place`.";
    // PX3xxx — op semantics
    "PX3001", Runtime, "Operation failed", "Read the message; re-run with -v.";
    "PX3101", Usage, "Unknown blend mode", "See `pixelox docs layer.blend` for the 28 modes.";
    "PX3201", Usage, "Not a template", "Declare slots with `pixelox comp template init`.";
    "PX3202", Usage, "Required template slot unfilled", "Fill the slot named in the message with --set, --set-image, or --set-color.";
    "PX3203", Usage, "Unknown template slot", "List slots with `pixelox comp template slots <template>`.";
    "PX3204", Usage, "Template slot type mismatch", "Use --set for text, --set-image for image, --set-color for color slots.";
    "PX3310", Usage, "Curve points are not monotonic", "Give curve points with strictly increasing input values.";
    "PX3311", Usage, "Invalid LUT file", "Supply a valid .cube 3D LUT.";
    "PX3320", Usage, "Empty region", "The selection or mask covers no pixels.";
    // PX4xxx — text and fonts
    "PX4101", Runtime, "Font file unreadable", "Check the font path.";
    "PX4102", Usage, "Font family not found", "List fonts with `pixelox fonts list`, or add a folder with `pixelox fonts add <dir>`.";
    // PX5xxx — ML inference
    "PX5201", Memory, "Model input too large for the memory budget", "Raise `--mem-budget` or downscale the input.";
    // PX6xxx — JS sandbox
    "PX6001", Sandbox, "Script error", "Read the message for the line and column.";
    "PX6002", Sandbox, "Wall-clock limit exceeded", "Raise `--timeout` or simplify the script.";
    "PX6003", Sandbox, "Script memory limit exceeded", "Raise `--js-mem`.";
    "PX6010", Sandbox, "Forbidden API", "Scripts may only use the `px` API; no filesystem, network, timers, or eval.";
    "PX6020", Usage, "Script produced no output", "Call `px.output(canvasOrSvg)` exactly once.";
    // PX7xxx — jobs, pipelines, batches
    "PX7101", Usage, "Pipeline file is invalid", "Run `pixelox pipeline validate <file>`.";
    "PX7102", Usage, "Unknown step reference", "Every `needs` entry must name an earlier step id.";
    "PX7103", Usage, "Cyclic pipeline graph", "Remove the cycle among the steps named in the message.";
    "PX7104", Runtime, "Declared pipeline output missing", "The step producing it did not write it; check its `output`.";
    "PX7105", Usage, "Unresolved variable", "Define the variable under `vars:` or pass `--var k=v`.";
    "PX7201", Job, "Unknown job id", "List jobs with `pixelox jobs list`.";
    "PX7202", Job, "Job not finished", "Wait with `pixelox jobs wait <id>`.";
    "PX7203", Job, "Cancel failed", "The job may already have finished; check `pixelox jobs status <id>`.";
    "PX7301", Runtime, "One or more batch items failed", "See the per-item report in `result.items`.";
    // PX19xx — process
    "PX1901", Interrupted, "Interrupted or timed out", "Re-run; raise the timeout if one was set.";
    "PX1902", Memory, "Memory budget exceeded", "Raise `--mem-budget` or process a smaller image.";
    // PX9xxx — internal invariants
    "PX9001", Runtime, "Internal error (a bug)", "Report it at https://github.com/djynnius/pixelox/issues with the command and its sidecar.";
}

/// Warning codes (`PXW####`). Warnings never change the exit code.
pub const WARNINGS: &[(&str, &str)] = &[
    ("PXW0001", "--dry-run was given to a read-only command and ignored"),
    ("PXW0002", "the seed was drawn from OS entropy; it is recorded so the run can be replayed"),
    ("PXW0101", "the input carried an ICC profile that was assigned, not converted"),
    ("PXW0102", "EXIF orientation was applied on open"),
    ("PXW0201", "the result clips: some channels were outside 0..1 before encoding"),
    ("PXW0202", "the output format cannot store transparency; alpha was flattened onto white"),
    ("PXW0203", "the output format cannot store 16 bits; written at 8 bits"),
    ("PXW0204", "lossy WebP encoding is not in this build; wrote lossless WebP"),
    ("PXW1203", "a PSD feature was rendered from the PSD composite"),
    ("PXW1204", "PSD text was imported as raster; the text is kept in the layer's properties"),
    ("PXW1205", "a layer kind was rasterized when writing PSD"),
    ("PXW2001", "the target layer is hidden"),
    ("PXW2002", "the mask or selection is empty"),
    ("PXW2003", "a live layer (text, shape, placed) was rasterized to apply a transform"),
    ("PXW2004", "the strokes painted nothing (off the canvas, or masked out)"),
    ("PXW3002", "text auto-shrink was skipped or the text still overflows its slot"),
    ("PXW3001", "a parameter was clamped to its valid range"),
    ("PXW4001", "a glyph was missing from the font and drawn with a fallback"),
    ("PXW4002", "synthetic bold or italic was used because the face was not installed"),
    ("PXW6001", "the script read the clock (--allow-time): output is not deterministic"),
    ("PXW6002", "canvas fillText/strokeText drew nothing (text on a JS canvas is not supported; use px.svg() text)"),
];

pub fn catalog_entry(code: &str) -> Option<&'static CatalogEntry> {
    CATALOG.iter().find(|e| e.code == code)
}

/// The error carried through every command.
#[derive(Debug, Clone, Serialize)]
pub struct Error {
    #[serde(skip)]
    pub kind: ErrorKind,
    /// Stable `PX####` identifier.
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    /// `errors/PX1401`: the page `pixelox docs` prints for this code.
    pub docs: String,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub context: Map<String, Value>,
    pub exit_code: i32,
}

impl Error {
    /// An error with a catalog code; the kind (exit code) and default hint come from the catalog.
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        let (kind, remedy) = match catalog_entry(code) {
            Some(e) => (e.kind, Some(e.remedy.to_string())),
            None => (ErrorKind::Runtime, None),
        };
        Error {
            kind,
            code: code.to_string(),
            message: message.into(),
            hint: remedy,
            docs: format!("errors/{code}"),
            context: Map::new(),
            exit_code: kind.exit_code(),
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn with_context(mut self, key: &str, value: impl Serialize) -> Self {
        self.context.insert(key.to_string(), serde_json::to_value(value).unwrap_or(Value::Null));
        self
    }

    /// Generic usage error (PX1010).
    pub fn usage(message: impl Into<String>) -> Self {
        Error::new("PX1010", message)
    }

    /// Generic runtime failure (PX3001).
    pub fn runtime(message: impl Into<String>) -> Self {
        Error::new("PX3001", message)
    }

    pub fn input_missing(path: &std::path::Path) -> Self {
        Error::new("PX1101", format!("input not found: {}", path.display()))
            .with_context("path", path.display().to_string())
    }

    pub fn unsupported(message: impl Into<String>) -> Self {
        Error::new("PX1105", message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Error::new("PX9001", message)
    }

    pub fn verification(code: &str, message: impl Into<String>) -> Self {
        Error::new(code, message)
    }

    pub fn interrupted(message: impl Into<String>) -> Self {
        Error::new("PX1901", message)
    }

    pub fn no_layer(name: &str) -> Self {
        Error::new("PX2301", format!("no layer '{name}'")).with_context("layer", name)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)?;
        if let Some(h) = &self.hint {
            write!(f, " (hint: {h})")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => Error::new("PX1101", e.to_string()),
            std::io::ErrorKind::PermissionDenied => Error::new("PX1102", e.to_string()),
            _ => Error::new("PX3001", format!("I/O error: {e}")),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::usage(format!("invalid JSON: {e}"))
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_codes_are_unique_and_well_formed() {
        let mut seen = std::collections::BTreeSet::new();
        for e in CATALOG {
            assert!(seen.insert(e.code), "duplicate {}", e.code);
            assert!(e.code.starts_with("PX") && e.code.len() == 6, "bad code {}", e.code);
        }
        for (w, _) in WARNINGS {
            assert!(w.starts_with("PXW") && w.len() == 7);
        }
    }

    #[test]
    fn kinds_map_to_prd_exit_codes() {
        assert_eq!(Error::new("PX1401", "x").exit_code, 4);
        assert_eq!(Error::new("PX1502", "x").exit_code, 6);
        assert_eq!(Error::new("PX1601", "x").exit_code, 9);
        assert_eq!(Error::new("PX1510", "x").exit_code, 12);
        assert_eq!(Error::new("PX2301", "x").exit_code, 2);
    }
}
