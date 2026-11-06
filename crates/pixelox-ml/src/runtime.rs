//! ONNX Runtime sessions (the `ml` feature): one process-wide environment, a session per
//! model file, and a CPU configuration fixed for determinism (PRD §6.6): a constant
//! intra-op thread count independent of the machine and `--threads`, one inter-op thread,
//! and deterministic kernels, so a model gives the same bytes on every run of the same
//! build and target.

use crate::registry::Source;
use ort::session::builder::SessionBuilder;
use ort::session::Session;
use ort::value::Tensor;
use pixelox_core::{Error, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};

/// Intra-op threads: a constant, so results do not depend on the core count.
pub const ML_THREADS: usize = 4;

fn environment() -> Result<&'static ort::environment::Environment> {
    static ENV: OnceLock<std::result::Result<ort::environment::Environment, String>> = OnceLock::new();
    ENV.get_or_init(|| ort::init().with_name("pixelox").with_telemetry(false).build().map_err(|e| e.to_string()))
        .as_ref()
        .map_err(|e| Error::runtime(format!("ONNX Runtime failed to start: {e}")))
}

type Shared = Arc<Mutex<Session>>;

fn sessions() -> &'static Mutex<HashMap<PathBuf, Shared>> {
    static S: OnceLock<Mutex<HashMap<PathBuf, Shared>>> = OnceLock::new();
    S.get_or_init(|| Mutex::new(HashMap::new()))
}

fn build(path: &Path) -> std::result::Result<Session, String> {
    let env = environment().map_err(|e| e.message)?;
    let s = |e: ort::Error<SessionBuilder>| e.to_string();
    // `ort` gives every new builder a "max efficiency" automatic device policy that silently
    // picks CoreML / NPU devices (on Apple silicon the Neural Engine compile of BiRefNet never
    // finishes); reset it and register the CPU provider so the result is the CPU's, the same
    // on every machine of a target (as ferraus does).
    Session::builder(env)
        .map_err(|e| e.to_string())?
        .with_auto_device(ort::session::builder::AutoDevicePolicy::Default)
        .map_err(s)?
        .with_execution_providers([ort::ep::CPU::default().with_arena_allocator(true).build().error_on_failure()])
        .map_err(s)?
        .with_intra_threads(ML_THREADS)
        .map_err(s)?
        .with_inter_threads(1)
        .map_err(s)?
        .with_parallel_execution(false)
        .map_err(s)?
        .with_deterministic_compute(true)
        .map_err(s)?
        .commit_from_file(path)
        .map_err(|e| e.to_string())
}

fn session(path: &Path) -> Result<Shared> {
    let key = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    if let Some(s) = sessions().lock().unwrap_or_else(|e| e.into_inner()).get(&key) {
        return Ok(s.clone());
    }
    let s = build(path).map_err(|e| {
        Error::runtime(format!("cannot load model {}: {e}", path.display()))
            .with_hint("re-download it: pixelox models rm <name> && pixelox models pull <name> -y")
    })?;
    let shared = Arc::new(Mutex::new(s));
    sessions().lock().unwrap_or_else(|e| e.into_inner()).insert(key, shared.clone());
    Ok(shared)
}

/// Run a segmentation model on a `1×3×n×n` tensor; returns its first output with `n×n`
/// values (BiRefNet and U²-Net list the full-resolution map first).
pub fn run_map(path: &Path, src: &Source, input: Vec<f32>) -> Result<Vec<f32>> {
    let n = src.input as usize;
    let shared = session(path)?;
    let mut s = shared.lock().unwrap_or_else(|e| e.into_inner());
    let in_name =
        s.inputs().first().map(|i| i.name().to_string()).ok_or_else(|| Error::runtime("the model has no input"))?;
    let out_names: Vec<String> = s.outputs().iter().map(|o| o.name().to_string()).collect();
    let tensor = Tensor::from_array((vec![1usize, 3, n, n], input))
        .map_err(|e| Error::runtime(format!("cannot build the input tensor: {e}")))?;
    let outputs = s
        .run(vec![(in_name, ort::session::SessionInputValue::from(tensor))])
        .map_err(|e| Error::runtime(format!("model inference failed: {e}")))?;
    for name in &out_names {
        let Some(v) = outputs.get(name) else { continue };
        if let Ok((_, data)) = v.try_extract_tensor::<f32>() {
            if data.len() == n * n {
                return Ok(data.to_vec());
            }
        }
    }
    Err(Error::runtime(format!("the model has no {n}×{n} output map (outputs: {})", out_names.join(", "))))
}

/// Drop cached sessions (after `models rm`).
pub fn evict(path: &Path) {
    let key = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    sessions().lock().unwrap_or_else(|e| e.into_inner()).remove(&key);
}
