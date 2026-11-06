//! The execution context handed to every op: global flags, the image IO layer, input and
//! output helpers (sandbox checks, atomic writes, overwrite protection), mask resolution,
//! seeded randomness, and the collectors for warnings, recipe inputs, models, and fonts.

use crate::doc::{Document, InputRef, RecipeStep};
use crate::envelope::{file_record, FileRecord, OutputInfo, Warning};
use crate::error::{Error, Result};
use crate::image::{Image, Mask};
use crate::op::Args;
use rand_chacha::ChaCha12Rng;
use rand_core::SeedableRng;
use serde::Serialize;
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Options for opening files (PRD §8.2 `open`).
#[derive(Clone, Debug, Serialize)]
pub struct OpenOptions {
    /// Page of a multi-page TIFF (1-based).
    pub page: Option<usize>,
    /// Frame of an animated GIF (1-based).
    pub frame: Option<usize>,
    /// `honor`, `assign`, or `ignore`.
    pub icc: String,
    pub auto_orient: bool,
    /// Decompression-bomb guard.
    pub limit_pixels: u64,
}

impl Default for OpenOptions {
    fn default() -> Self {
        OpenOptions { page: None, frame: None, icc: "honor".into(), auto_orient: true, limit_pixels: 512_000_000 }
    }
}

/// Encoder settings for a deliverable.
#[derive(Clone, Debug, Serialize, Default)]
pub struct EncodeOptions {
    /// `png`, `jpeg`, `webp`, `tiff`, `gif`, `bmp`, `psd`.
    pub format: String,
    pub quality: Option<u8>,
    /// 8 or 16.
    pub bit_depth: Option<u8>,
    pub progressive: bool,
    /// TIFF: `lzw`, `deflate`, `none`.
    pub compression: Option<String>,
    pub strip_metadata: bool,
    /// Output profile: `srgb`, `display-p3`, `adobe-rgb`, or an .icc path.
    pub profile: Option<String>,
    /// PSD: write raster layers (`--layered-basic`).
    pub layered: bool,
    /// WebP: lossless.
    pub lossless: Option<bool>,
    /// GIF: dithering (`fs`, `ordered`, `none`) and palette size.
    pub dither: Option<String>,
    pub colors: Option<u16>,
    pub dpi: Option<f32>,
}

impl EncodeOptions {
    /// Defaults for a path's extension.
    pub fn for_path(path: &Path) -> EncodeOptions {
        EncodeOptions { format: format_of(path).unwrap_or_else(|| "png".into()), ..Default::default() }
    }
}

/// Canonical format name for a path's extension.
pub fn format_of(path: &Path) -> Option<String> {
    let ext = path.extension()?.to_string_lossy().to_ascii_lowercase();
    Some(
        match ext.as_str() {
            "jpg" | "jpeg" | "jpe" => "jpeg",
            "tif" | "tiff" => "tiff",
            "png" | "apng" => "png",
            "webp" => "webp",
            "gif" => "gif",
            "bmp" => "bmp",
            "psd" => "psd",
            "svg" => "svg",
            "pxo" => "pxo",
            other => return Some(other.to_string()),
        }
        .to_string(),
    )
}

pub fn is_pxo(path: &Path) -> bool {
    format_of(path).as_deref() == Some("pxo")
}

/// What `probe` reports about a file.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ImageInfo {
    pub format: String,
    pub width: u32,
    pub height: u32,
    /// Bits per channel.
    pub depth: u8,
    pub channels: u8,
    pub has_alpha: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frames: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icc: Option<String>,
    pub exif: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dpi: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<u16>,
}

/// Decoding and encoding, implemented by `pixelox-io` and injected by the CLI.
pub trait ImageIo: Send + Sync {
    /// Open any supported file as a document (plain images → one `Background` layer;
    /// `.pxo` → the stored document with its recipe; PSD → layers per the fidelity matrix).
    fn open(&self, path: &Path, opts: &OpenOptions) -> Result<(Document, Vec<Warning>)>;
    /// Decode image bytes (placed files, mask images, patterns).
    fn decode(&self, bytes: &[u8], hint: Option<&str>, opts: &OpenOptions) -> Result<(Image, ImageInfo)>;
    fn probe(&self, path: &Path) -> Result<ImageInfo>;
    /// Encode a flattened image to `path` (a temp file; the caller renames it).
    fn encode(&self, image: &Image, doc: Option<&Document>, path: &Path, opts: &EncodeOptions) -> Result<Vec<Warning>>;
    /// Write a `.pxo` document to `path` (a temp file; the caller renames it).
    fn save_pxo(&self, doc: &Document, path: &Path) -> Result<()>;
    /// Formats this build can read and write.
    fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>);
}

/// An IO layer that supports nothing (unit tests of ops that never touch files).
pub struct NoIo;

impl ImageIo for NoIo {
    fn open(&self, path: &Path, _: &OpenOptions) -> Result<(Document, Vec<Warning>)> {
        Err(Error::unsupported(format!("no IO layer to open {}", path.display())))
    }
    fn decode(&self, _: &[u8], _: Option<&str>, _: &OpenOptions) -> Result<(Image, ImageInfo)> {
        Err(Error::unsupported("no IO layer"))
    }
    fn probe(&self, path: &Path) -> Result<ImageInfo> {
        Err(Error::unsupported(format!("no IO layer to probe {}", path.display())))
    }
    fn encode(&self, _: &Image, _: Option<&Document>, _: &Path, _: &EncodeOptions) -> Result<Vec<Warning>> {
        Err(Error::new("PX1106", "no IO layer"))
    }
    fn save_pxo(&self, _: &Document, _: &Path) -> Result<()> {
        Err(Error::new("PX1106", "no IO layer"))
    }
    fn formats(&self) -> (Vec<&'static str>, Vec<&'static str>) {
        (vec![], vec![])
    }
}

/// Progress sink: a bar or NDJSON events on stderr, or nothing.
pub trait Progress: Send + Sync {
    fn stage(&self, _stage: &str, _index: usize, _of: usize) {}
    fn progress(&self, _stage: &str, _fraction: f64) {}
    fn output(&self, _path: &str) {}
    fn warning(&self, _code: &str, _message: &str) {}
    fn log(&self, _message: &str) {}
    /// A structured event (batch items) for `--progress json`.
    fn event(&self, _event: &Value) {}
}

pub struct NoProgress;
impl Progress for NoProgress {}

/// Flags accepted by every command (PRD §8.1).
#[derive(Clone, Debug, Serialize)]
pub struct Globals {
    pub output: Option<String>,
    /// Output format override when the extension is ambiguous.
    pub format: Option<String>,
    pub overwrite: bool,
    pub in_place: bool,
    pub dry_run: bool,
    pub seed: u64,
    /// The seed was drawn from OS entropy (`--seed random`).
    pub seed_drawn: bool,
    /// Consent for network actions (`-y`).
    pub yes: bool,
    pub verbose: i32,
    pub threads: usize,
    pub mem_budget_mb: u64,
    pub sidecar: bool,
    pub redact_paths: bool,
    pub config_path: Option<PathBuf>,
    /// When set, every read and write must resolve inside one of these (MCP sandbox).
    pub allow: Vec<PathBuf>,
    /// MCP `--read-only`.
    pub read_only: bool,
    pub open: OpenOptions,
}

impl Default for Globals {
    fn default() -> Self {
        Globals {
            output: None,
            format: None,
            overwrite: false,
            in_place: false,
            dry_run: false,
            seed: 0,
            seed_drawn: false,
            yes: false,
            verbose: 0,
            threads: 0,
            mem_budget_mb: 4096,
            sidecar: true,
            redact_paths: false,
            config_path: None,
            allow: Vec::new(),
            read_only: false,
            open: OpenOptions::default(),
        }
    }
}

/// Cancellation flag shared with signal handlers and job runners.
#[derive(Clone, Default)]
pub struct CancelToken(Arc<AtomicBool>);

impl CancelToken {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst)
    }
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
    pub fn check(&self) -> Result<()> {
        if self.is_cancelled() {
            Err(Error::interrupted("cancelled"))
        } else {
            Ok(())
        }
    }
}

/// One written output and the recipe its sidecar records.
#[derive(Clone, Debug)]
pub struct OutputEntry {
    pub record: FileRecord,
    pub info: OutputInfo,
    /// `None`: the executor uses the command's own step.
    pub recipe: Option<Vec<RecipeStep>>,
}

/// Context for one op invocation.
pub struct OpCtx<'a> {
    /// Command path (`filter gaussian`).
    pub command: String,
    /// Canonical id (`filter.gaussian`).
    pub id: String,
    pub inputs: Vec<PathBuf>,
    pub globals: Globals,
    pub io: &'a dyn ImageIo,
    pub progress: &'a dyn Progress,
    pub cancel: CancelToken,
    pub registry: Option<&'a crate::registry::Registry>,
    /// Files read (envelope `inputs`).
    pub input_records: Vec<FileRecord>,
    pub outputs: Vec<OutputEntry>,
    pub warnings: Vec<Warning>,
    /// Inputs of this op's recipe step (aux files: placed images, mask files, LUTs).
    pub step_inputs: Vec<InputRef>,
    /// Values resolved at run time (`auto` → number); merged into the recipe step params
    /// so a replay uses them.
    pub resolved: Map<String, Value>,
    pub models: Vec<Value>,
    pub fonts: Vec<Value>,
    /// Dry-run plan additions (`models_required`, `consent_required`, ...).
    pub plan: Map<String, Value>,
    /// Bytes for stdout (`-o -`).
    pub stdout: Option<Vec<u8>>,
    /// Index of this op in its document's recipe (seed lane).
    pub op_index: usize,
    /// Files created by this op that must be removed if it fails.
    pub temp_files: Vec<PathBuf>,
    /// This op is a step being re-executed by `replay`/`verify`/a pipeline rebuild: side
    /// files it would write as a command (`detect subject --cutout`) are skipped.
    pub replaying: bool,
}

impl<'a> OpCtx<'a> {
    pub fn new(
        command: &str,
        inputs: Vec<PathBuf>,
        globals: Globals,
        io: &'a dyn ImageIo,
        progress: &'a dyn Progress,
    ) -> Self {
        OpCtx {
            command: command.to_string(),
            id: command.replace(' ', "."),
            inputs,
            globals,
            io,
            progress,
            cancel: CancelToken::default(),
            registry: None,
            input_records: Vec::new(),
            outputs: Vec::new(),
            warnings: Vec::new(),
            step_inputs: Vec::new(),
            resolved: Map::new(),
            models: Vec::new(),
            fonts: Vec::new(),
            plan: Map::new(),
            stdout: None,
            op_index: 0,
            temp_files: Vec::new(),
            replaying: false,
        }
    }

    /// A child context sharing IO, globals, and registry (pipelines, replay, composites).
    pub fn child(&self, command: &str, inputs: Vec<PathBuf>) -> OpCtx<'a> {
        let mut c = OpCtx::new(command, inputs, self.globals.clone(), self.io, self.progress);
        c.cancel = self.cancel.clone();
        c.registry = self.registry;
        c
    }

    /// Merge a child's records (inputs, outputs, warnings, models, fonts) into this one.
    pub fn absorb(&mut self, child: OpCtx<'a>) {
        for r in child.input_records {
            if !self.input_records.iter().any(|x| x.path == r.path) {
                self.input_records.push(r);
            }
        }
        self.outputs.extend(child.outputs);
        self.warnings.extend(child.warnings);
        for m in child.models {
            if !self.models.contains(&m) {
                self.models.push(m);
            }
        }
        for f in child.fonts {
            if !self.fonts.contains(&f) {
                self.fonts.push(f);
            }
        }
    }

    pub fn dry_run(&self) -> bool {
        self.globals.dry_run
    }

    pub fn seed(&self) -> u64 {
        self.globals.seed
    }

    pub fn registry(&self) -> Result<&'a crate::registry::Registry> {
        self.registry.ok_or_else(|| Error::internal("no registry in context"))
    }

    /// The deterministic RNG of this op: ChaCha12 keyed by (seed, op index, op id), so
    /// inserting an op does not shift other ops' draws (PRD §6.6).
    pub fn rng(&self) -> ChaCha12Rng {
        self.rng_lane(0)
    }

    /// A sub-stream of this op's RNG (for parallel work: one lane per row or tile).
    pub fn rng_lane(&self, lane: u64) -> ChaCha12Rng {
        let mut h = Sha256::new();
        h.update(self.globals.seed.to_le_bytes());
        h.update((self.op_index as u64).to_le_bytes());
        h.update(self.id.as_bytes());
        h.update(lane.to_le_bytes());
        let seed: [u8; 32] = h.finalize().into();
        ChaCha12Rng::from_seed(seed)
    }

    pub fn input(&self, i: usize) -> Result<&Path> {
        self.inputs.get(i).map(|p| p.as_path()).ok_or_else(|| {
            Error::new("PX1003", format!("'{}' needs at least {} input file(s)", self.command, i + 1))
                .with_hint(format!("pixelox docs {}", self.id))
        })
    }

    /// Check a path against the MCP sandbox (symlinks resolved): PX1310 when outside.
    pub fn check_allowed(&self, path: &Path) -> Result<()> {
        if self.globals.allow.is_empty() {
            return Ok(());
        }
        let p = resolve_for_sandbox(path);
        let ok = self.globals.allow.iter().any(|root| {
            let r = resolve_for_sandbox(root);
            p.starts_with(&r)
        });
        if ok || is_models_path(&p) {
            Ok(())
        } else {
            Err(Error::new("PX1310", format!("{} is outside the allowed directories", path.display()))
                .with_context("path", path.display().to_string())
                .with_context(
                    "allowed",
                    self.globals.allow.iter().map(|a| a.display().to_string()).collect::<Vec<_>>(),
                ))
        }
    }

    fn check_readable(&self, path: &Path) -> Result<()> {
        self.check_allowed(path)?;
        if !path.exists() {
            return Err(Error::input_missing(path));
        }
        Ok(())
    }

    /// Open a document from a path (records the input). For non-`.pxo` inputs the recipe
    /// starts with an `open` step recording the source and its hash.
    pub fn load_document(&mut self, path: &Path) -> Result<Document> {
        self.check_readable(path)?;
        let rec = file_record(path, Some("source"))?;
        let (mut doc, warnings) = self.io.open(path, &self.globals.open)?;
        for w in warnings {
            self.push_warning(w);
        }
        if !is_pxo(path) {
            let mut params = Map::new();
            let o = &self.globals.open;
            if let Some(p) = o.page {
                params.insert("page".into(), Value::String(p.to_string()));
            }
            if let Some(f) = o.frame {
                params.insert("frame".into(), Value::String(f.to_string()));
            }
            if o.icc != "honor" {
                params.insert("icc".into(), Value::String(o.icc.clone()));
            }
            if !o.auto_orient {
                params.insert("no_auto_orient".into(), Value::String("true".into()));
            }
            doc.recipe = vec![RecipeStep {
                op: "open".into(),
                params,
                inputs: vec![InputRef {
                    role: "source".into(),
                    path: rec.path.clone(),
                    sha256: rec.sha256.clone(),
                    bytes: rec.bytes,
                    param: None,
                }],
                model: None,
                fonts: vec![],
            }];
        }
        self.record_input(rec);
        Ok(doc)
    }

    fn record_input(&mut self, rec: FileRecord) {
        if !self.input_records.iter().any(|r| r.path == rec.path) {
            self.input_records.push(rec);
        }
    }

    /// Read an auxiliary file (placed image, LUT, mask image, script) and record it as an
    /// input of this op's recipe step under `role`, substituting `param` on replay.
    pub fn read_aux(&mut self, path: &Path, role: &str, param: Option<&str>) -> Result<Vec<u8>> {
        self.check_readable(path)?;
        let bytes = std::fs::read(path)?;
        let sha = crate::envelope::sha256_bytes(&bytes);
        let rec = FileRecord {
            path: path.display().to_string(),
            sha256: sha.clone(),
            bytes: bytes.len() as u64,
            role: Some(role.to_string()),
            ..Default::default()
        };
        self.record_input(rec);
        if !self.step_inputs.iter().any(|i| i.path == path.display().to_string() && i.role == role) {
            self.step_inputs.push(InputRef {
                role: role.to_string(),
                path: path.display().to_string(),
                sha256: sha,
                bytes: bytes.len() as u64,
                param: param.map(String::from),
            });
        }
        Ok(bytes)
    }

    /// Read and decode an auxiliary image.
    pub fn decode_aux(
        &mut self,
        path: &Path,
        role: &str,
        param: Option<&str>,
    ) -> Result<(Image, crate::ctx::ImageInfo)> {
        let bytes = self.read_aux(path, role, param)?;
        let hint = path.extension().map(|e| e.to_string_lossy().to_string());
        self.io.decode(&bytes, hint.as_deref(), &self.globals.open)
    }

    /// Resolve a mask reference against a document: a named mask, `@sel`,
    /// `@alpha:<layer>`, `@mask:<layer>` (a layer mask), or a grayscale image path (luma, or alpha when the image has
    /// transparency). The result is canvas-sized.
    pub fn resolve_mask(&mut self, doc: &Document, r: &str) -> Result<Mask> {
        let r = r.trim();
        if let Some(layer) = r.strip_prefix("@mask:") {
            let p = doc.resolve(layer)?;
            let l = doc.layer(&p);
            return match &l.mask {
                Some(lm) => Ok(lm.mask.clone()),
                None => Err(Error::new("PX2311", format!("layer '{}' has no layer mask", l.name))
                    .with_hint(format!("add one with `pixelox mask add <doc> --layer '{}' --in-place`", l.name))
                    .with_context("layer", l.name.clone())),
            };
        }
        if let Some(layer) = r.strip_prefix("@alpha:") {
            let p = doc.resolve(layer)?;
            let l = doc.layer(&p);
            let rendered = crate::compose::render_layer(doc, l)?;
            return Ok(match rendered {
                Some(rd) => rd.to_canvas(doc.width, doc.height).alpha_mask(),
                None => Mask::new(doc.width, doc.height),
            });
        }
        if r == "@sel" || doc.masks.contains_key(r) {
            return doc.named_mask(r);
        }
        let path = Path::new(r);
        if r.starts_with('@') || (!path.exists() && path.extension().is_none()) {
            // looks like a name, not a file
            return doc.named_mask(r);
        }
        let (img, _) = self.decode_aux(path, "mask", Some("mask"))?;
        if img.width != doc.width || img.height != doc.height {
            return Err(Error::new(
                "PX2313",
                format!("mask image {} is {}x{}, the canvas is {}x{}", r, img.width, img.height, doc.width, doc.height),
            )
            .with_hint(format!(
                "resize it first: pixelox transform resize {r} --size {}x{} -o mask-resized.png",
                doc.width, doc.height
            )));
        }
        Ok(image_to_mask(&img))
    }

    /// `--mask` / `--invert-mask` from args (None when no mask was given).
    pub fn mask_arg(&mut self, args: &Args, doc: &Document) -> Result<Option<Mask>> {
        let Some(r) = args.str("mask") else {
            if args.flag("invert-mask") {
                return Err(Error::new("PX1005", "--invert-mask needs --mask"));
            }
            return Ok(None);
        };
        let mut m = self.resolve_mask(doc, &r)?;
        if args.flag("invert-mask") {
            m.invert();
        }
        if m.is_empty() {
            self.warn("PXW2002", format!("mask '{r}' covers no pixels"));
        }
        Ok(Some(m))
    }

    /// Overwrite policy, sandbox, and same-as-input checks for an output path; creates
    /// its directory (not in dry runs). Returns the dry-run action word.
    pub fn prepare_output(&self, path: &Path) -> Result<&'static str> {
        if path.as_os_str() == "-" {
            return Ok("write");
        }
        self.check_allowed(path)?;
        if self.globals.read_only {
            return Err(Error::new("PX1310", "this server is read-only; commands that write files are disabled"));
        }
        let is_input = self.inputs.iter().any(|i| same_path(i, path));
        if is_input && !self.globals.in_place {
            return Err(Error::new("PX1403", format!("output {} is the same as an input", path.display()))
                .with_context("path", path.display().to_string()));
        }
        let action = if is_input {
            "mutate-in-place"
        } else if path.exists() {
            if !self.globals.overwrite {
                return Err(Error::new(
                    "PX1401",
                    format!("output '{}' exists and --overwrite was not given", path.display()),
                )
                .with_context("path", path.display().to_string()));
            }
            "overwrite"
        } else {
            "write"
        };
        if !self.globals.dry_run {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent)?;
                }
            }
        }
        Ok(action)
    }

    fn finish_write(
        &mut self,
        path: &Path,
        action: &str,
        role: Option<&str>,
        mut info: OutputInfo,
        recipe: Option<Vec<RecipeStep>>,
    ) -> Result<()> {
        let mut record = if self.globals.dry_run || path.as_os_str() == "-" {
            FileRecord {
                path: path.display().to_string(),
                role: role.map(String::from),
                action: Some(action.to_string()),
                ..Default::default()
            }
        } else {
            let mut r = file_record(path, role)?;
            if self.globals.sidecar && !path.to_string_lossy().ends_with(".pixelox.json") {
                r.sidecar = Some(crate::envelope::Sidecar::path_for(path).display().to_string());
            }
            r
        };
        record.size = info.size;
        record.format = info.format.clone();
        info.path = record.path.clone();
        info.sha256 = record.sha256.clone();
        info.bytes = record.bytes;
        info.role = role.map(String::from);
        self.progress.output(&record.path);
        self.outputs.push(OutputEntry { record, info, recipe });
        Ok(())
    }

    /// Encode an image to `path` atomically and record it. `recipe` is what its sidecar
    /// records (None: this op's own step).
    pub fn write_image(
        &mut self,
        path: &Path,
        image: &Image,
        doc: Option<&Document>,
        opts: &EncodeOptions,
        role: Option<&str>,
        recipe: Option<Vec<RecipeStep>>,
    ) -> Result<()> {
        let action = self.prepare_output(path)?;
        let bit_depth = opts.bit_depth.or(doc.map(|d| d.depth)).unwrap_or(8);
        let info = OutputInfo {
            size: Some([image.width, image.height]),
            format: Some(opts.format.clone()),
            color_profile: Some(profile_name(opts.profile.as_deref(), doc)),
            bit_depth: Some(bit_depth),
            ..Default::default()
        };
        if !self.globals.dry_run {
            let tmp = tmp_path(path);
            self.temp_files.push(tmp.clone());
            let res = self.io.encode(image, doc, &tmp, opts);
            match res {
                Ok(ws) => {
                    for w in ws {
                        self.push_warning(w);
                    }
                }
                Err(e) => {
                    let _ = std::fs::remove_file(&tmp);
                    return Err(e);
                }
            }
            std::fs::rename(&tmp, path)?;
        }
        self.finish_write(path, action, role, info, recipe)
    }

    /// Save a document as `.pxo` atomically and record it.
    pub fn write_doc(&mut self, path: &Path, doc: &Document, role: Option<&str>) -> Result<()> {
        let action = self.prepare_output(path)?;
        let info = OutputInfo {
            size: Some([doc.width, doc.height]),
            format: Some("pxo".into()),
            bit_depth: Some(doc.depth),
            ..Default::default()
        };
        if !self.globals.dry_run {
            let tmp = tmp_path(path);
            self.temp_files.push(tmp.clone());
            if let Err(e) = self.io.save_pxo(doc, &tmp) {
                let _ = std::fs::remove_file(&tmp);
                return Err(e);
            }
            std::fs::rename(&tmp, path)?;
        }
        let mut recipe = doc.recipe.clone();
        recipe.push(RecipeStep { op: "save".into(), params: Map::new(), inputs: vec![], model: None, fonts: vec![] });
        self.finish_write(path, action, role, info, Some(recipe))
    }

    /// Write any bytes atomically (`-` = stdout) and record them.
    pub fn write_file(
        &mut self,
        path: &Path,
        bytes: &[u8],
        role: Option<&str>,
        recipe: Option<Vec<RecipeStep>>,
    ) -> Result<()> {
        let action = self.prepare_output(path)?;
        if !self.globals.dry_run {
            if path.as_os_str() == "-" {
                self.stdout.get_or_insert_with(Vec::new).extend_from_slice(bytes);
            } else {
                let tmp = tmp_path(path);
                self.temp_files.push(tmp.clone());
                std::fs::write(&tmp, bytes)?;
                std::fs::rename(&tmp, path)?;
            }
        }
        let info = OutputInfo { format: format_of(path), ..Default::default() };
        self.finish_write(path, action, role, info, recipe)
    }

    pub fn warn(&mut self, code: &str, message: impl Into<String>) {
        self.push_warning(Warning { code: code.to_string(), message: message.into(), context: Map::new() });
    }

    pub fn warn_ctx(&mut self, code: &str, message: impl Into<String>, context: Value) {
        let context = match context {
            Value::Object(m) => m,
            _ => Map::new(),
        };
        self.push_warning(Warning { code: code.to_string(), message: message.into(), context });
    }

    pub fn push_warning(&mut self, w: Warning) {
        self.progress.warning(&w.code, &w.message);
        if !self.warnings.contains(&w) {
            self.warnings.push(w);
        }
    }

    /// Record a value resolved at run time (e.g. `--angle auto` → the detected angle).
    pub fn resolve_param(&mut self, key: &str, value: impl Serialize) {
        self.resolved.insert(key.replace('-', "_"), serde_json::to_value(value).unwrap_or(Value::Null));
    }

    pub fn model_used(&mut self, model: Value) {
        if !self.models.contains(&model) {
            self.models.push(model);
        }
    }

    pub fn font_used(&mut self, font: Value) {
        if !self.fonts.contains(&font) {
            self.fonts.push(font);
        }
    }

    /// Add to the dry-run plan under `key` (arrays are appended).
    pub fn plan_add(&mut self, key: &str, value: Value) {
        match self.plan.get_mut(key) {
            Some(Value::Array(a)) => a.push(value),
            _ => {
                self.plan.insert(key.to_string(), Value::Array(vec![value]));
            }
        }
    }

    pub fn set_progress(&self, fraction: f64) {
        self.progress.progress(&self.command, fraction.clamp(0.0, 1.0));
    }

    pub fn check_cancel(&self) -> Result<()> {
        self.cancel.check()
    }

    pub fn threads(&self) -> usize {
        if self.globals.threads == 0 {
            std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1)
        } else {
            self.globals.threads
        }
    }
}

/// Convert an image to a mask: its alpha when it has transparency, else its luma.
pub fn image_to_mask(img: &Image) -> Mask {
    if img.is_opaque() {
        Mask {
            width: img.width,
            height: img.height,
            data: img.data.chunks(4).map(|p| crate::color::luma(p[0], p[1], p[2])).collect(),
        }
    } else {
        img.alpha_mask()
    }
}

fn profile_name(p: Option<&str>, doc: Option<&Document>) -> String {
    match p.or(doc.map(|d| d.color_space.as_str())) {
        Some("display-p3") => "Display P3".into(),
        Some("adobe-rgb") => "Adobe RGB (1998)".into(),
        Some(other) if other.ends_with(".icc") || other.ends_with(".icm") => other.to_string(),
        _ => "sRGB IEC61966-2.1".into(),
    }
}

/// Temporary sibling path for atomic writes; keeps the real extension last.
pub fn tmp_path(path: &Path) -> PathBuf {
    let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let ext = path.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
    path.with_file_name(format!(".{name}.{}.part.{ext}", std::process::id()))
}

/// Make a path absolute and lexically normalized (no symlink resolution).
pub fn absolutize(p: &Path) -> PathBuf {
    let expanded = if let Ok(s) = p.strip_prefix("~") {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join(s)).unwrap_or_else(|| p.to_path_buf())
    } else {
        p.to_path_buf()
    };
    let abs =
        if expanded.is_absolute() { expanded } else { std::env::current_dir().unwrap_or_default().join(expanded) };
    let mut out = PathBuf::new();
    for c in abs.components() {
        match c {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

/// Absolute path with symlinks resolved for the deepest existing ancestor (so outputs that
/// do not exist yet are checked through their real parent directory).
pub fn resolve_for_sandbox(p: &Path) -> PathBuf {
    let abs = absolutize(p);
    let mut existing = abs.clone();
    let mut rest: Vec<std::ffi::OsString> = Vec::new();
    while !existing.exists() {
        match existing.file_name() {
            Some(n) => {
                rest.push(n.to_os_string());
                existing.pop();
            }
            None => break,
        }
    }
    let mut real = std::fs::canonicalize(&existing).unwrap_or(existing);
    for n in rest.into_iter().rev() {
        real.push(n);
    }
    real
}

fn is_models_path(p: &Path) -> bool {
    p.starts_with(resolve_for_sandbox(&crate::paths::models_dir()))
}

pub fn same_path(a: &Path, b: &Path) -> bool {
    resolve_for_sandbox(a) == resolve_for_sandbox(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx(globals: Globals) -> OpCtx<'static> {
        OpCtx::new("filter gaussian", vec![], globals, &NoIo, &crate::ctx::NoProgress)
    }

    #[test]
    fn layer_masks_resolve_as_mask_references() {
        use crate::doc::LayerMask;
        let mut d = Document::from_image(Image::new(4, 2));
        let p = d.resolve("Background").unwrap();
        let mut c = ctx(Globals::default());
        assert_eq!(c.resolve_mask(&d, "@mask:Background").unwrap_err().code, "PX2311");
        d.layer_mut(&p).mask = Some(LayerMask::new(Mask::from_fn(4, 2, |x, _| if x < 1 { 1.0 } else { 0.0 })));
        let m = c.resolve_mask(&d, "@mask:Background").unwrap();
        assert_eq!((m.at(0, 0), m.at(3, 1)), (1.0, 0.0));
        assert_eq!(c.resolve_mask(&d, "@mask:Nope").unwrap_err().code, "PX2301");
    }

    #[test]
    fn rng_lanes_are_deterministic_and_distinct() {
        use rand_core::RngCore;
        let c = ctx(Globals::default());
        let a = c.rng().next_u64();
        let b = c.rng().next_u64();
        assert_eq!(a, b);
        assert_ne!(c.rng_lane(1).next_u64(), a);
        let g = Globals { seed: 7, ..Default::default() };
        assert_ne!(ctx(g).rng().next_u64(), a);
    }

    #[test]
    fn overwrite_protection() {
        let dir = std::env::temp_dir().join(format!("pxo-ctx-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("exists.png");
        std::fs::write(&f, b"x").unwrap();
        let c = ctx(Globals::default());
        assert_eq!(c.prepare_output(&f).unwrap_err().code, "PX1401");
        let g = Globals { overwrite: true, ..Default::default() };
        assert_eq!(ctx(g).prepare_output(&f).unwrap(), "overwrite");
        let g = Globals { allow: vec![dir.join("sub")], ..Default::default() };
        assert_eq!(ctx(g).prepare_output(&dir.join("x.png")).unwrap_err().code, "PX1310");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn formats_from_extensions() {
        assert_eq!(format_of(Path::new("a.JPG")).as_deref(), Some("jpeg"));
        assert_eq!(format_of(Path::new("a.tif")).as_deref(), Some("tiff"));
        assert!(is_pxo(Path::new("doc.pxo")));
        assert!(tmp_path(Path::new("/x/out.png")).to_string_lossy().ends_with(".png"));
    }
}
