//! The QuickJS sandbox: one runtime per run, on its own thread, with a memory limit, a
//! wall-clock interrupt, a minimal set of intrinsics, and the `px` API from `prelude.js`.

use super::canvas::{self, Canvas, Gradient};
use pixelox_core::error::{Error, Result};
use rand_core::RngCore;
use rquickjs::context::intrinsic;
use rquickjs::function::Rest;
use rquickjs::{ArrayBuffer, Context, Ctx, Exception, Function, Module, Object, Runtime, Value};
use serde_json::{json, Value as Json};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tiny_skia::Pixmap;

const PRELUDE: &str = include_str!("prelude.js");
const MAX_LOG_LINES: usize = 1000;
/// Native stack of the sandbox thread, and the part of it QuickJS may use.
const THREAD_STACK: usize = 64 << 20;
const JS_STACK: usize = 16 << 20;

/// A library loaded before the script (`--lib`).
#[derive(Clone, Debug)]
pub struct Lib {
    /// File name (stack traces and module names).
    pub name: String,
    pub source: String,
    /// Evaluate as an ES module (`.mjs`, `*.module.js`) instead of a classic script.
    pub module: bool,
}

/// Everything one sandboxed run needs. Plain data, so it can move to the sandbox thread.
#[derive(Clone, Debug)]
pub struct JsJob {
    pub script: String,
    pub script_name: String,
    pub libs: Vec<Lib>,
    /// Global a preset library must define (`d3`, `THREE`).
    pub preset: Option<&'static str>,
    /// `{name: {kind: "json"|"table"|"text", text, columns?}}`.
    pub data: Json,
    pub params: Json,
    pub width: u32,
    pub height: u32,
    pub timeout: Duration,
    pub mem_bytes: usize,
    pub allow_time: bool,
    /// Seed for `px.random()` / `Math.random()` (ChaCha12 stream of the op).
    pub rng_seed: [u8; 32],
    pub pixel_limit: u64,
}

/// What the script passed to `px.output()`.
pub enum JsOutput {
    Canvas(Pixmap),
    Svg(String),
}

pub struct JsResult {
    pub output: JsOutput,
    pub logs: Vec<String>,
    pub logs_dropped: usize,
    /// Calls to `fillText`/`strokeText` (drawn as no-ops).
    pub text_calls: usize,
    /// The script read the clock (only possible with `--allow-time`).
    pub time_read: bool,
    pub canvases: usize,
}

#[derive(Default)]
struct State {
    canvases: Vec<Canvas>,
    grads: Vec<Gradient>,
    output: Option<JsOutput>,
    logs: Vec<String>,
    logs_dropped: usize,
    text_calls: usize,
    time_read: bool,
    canvas_bytes: u64,
    mem_limit: u64,
    pixel_limit: u64,
    rejection: Option<String>,
}

/// Run a job on a dedicated thread (QuickJS needs a deep native stack).
pub fn run(job: JsJob) -> Result<JsResult> {
    let handle = std::thread::Builder::new()
        .name("pixelox-js".into())
        .stack_size(THREAD_STACK)
        .spawn(move || run_here(job))
        .map_err(|e| Error::runtime(format!("could not start the script thread: {e}")))?;
    handle.join().unwrap_or_else(|_| Err(Error::internal("the script engine panicked")))
}

fn opts(name: &str) -> rquickjs::context::EvalOptions {
    let mut o = rquickjs::context::EvalOptions::default();
    o.global = true;
    o.strict = false;
    o.filename = Some(name.to_string());
    o
}

type Intrinsics = (
    intrinsic::Date,
    intrinsic::Eval,
    intrinsic::RegExpCompiler,
    intrinsic::RegExp,
    intrinsic::Json,
    intrinsic::Proxy,
    intrinsic::MapSet,
    intrinsic::TypedArrays,
    intrinsic::Promise,
);

fn run_here(job: JsJob) -> Result<JsResult> {
    let rt = Runtime::new().map_err(|e| Error::runtime(format!("could not start QuickJS: {e}")))?;
    rt.set_memory_limit(job.mem_bytes);
    rt.set_max_stack_size(JS_STACK);
    let interrupted = Arc::new(AtomicBool::new(false));
    let deadline = Instant::now() + job.timeout;
    {
        let flag = interrupted.clone();
        rt.set_interrupt_handler(Some(Box::new(move || {
            if flag.load(Ordering::Relaxed) || Instant::now() >= deadline {
                flag.store(true, Ordering::Relaxed);
                return true;
            }
            false
        })));
    }
    let state = Rc::new(RefCell::new(State {
        mem_limit: job.mem_bytes as u64,
        pixel_limit: job.pixel_limit,
        ..Default::default()
    }));
    {
        let st = state.clone();
        rt.set_host_promise_rejection_tracker(Some(Box::new(move |ctx, _promise, reason, handled| {
            if !handled {
                let mut s = st.borrow_mut();
                if s.rejection.is_none() {
                    s.rejection = Some(describe_value(&ctx, &reason));
                }
            }
        })));
    }
    let ctx = Context::custom::<Intrinsics>(&rt)
        .map_err(|e| Error::runtime(format!("could not create a JS context: {e}")))?;
    let rng = Rc::new(RefCell::new(rand_chacha_from(job.rng_seed)));

    let outcome: Result<()> = ctx.with(|ctx| {
        let fail = |e: rquickjs::Error, file: &str| map_error(&ctx, e, file, &interrupted);
        let natives = natives(&ctx, state.clone(), rng.clone()).map_err(|e| fail(e, "pixelox:prelude"))?;
        let cfg = json!({
            "width": job.width,
            "height": job.height,
            "params": job.params,
            "data": job.data,
            "allowTime": job.allow_time,
            "version": pixelox_core::PIXELOX_VERSION,
        });
        let cfg = ctx.json_parse(cfg.to_string()).map_err(|e| fail(e, "pixelox:prelude"))?;
        let prelude: Function =
            ctx.eval_with_options(PRELUDE, opts("pixelox:prelude")).map_err(|e| fail(e, "pixelox:prelude"))?;
        let hooks: Object = prelude.call((natives, cfg)).map_err(|e| fail(e, "pixelox:prelude"))?;
        let hook = |name: &str| -> Result<Function> { hooks.get(name).map_err(|e| fail(e, "pixelox:prelude")) };

        if job.preset.is_some() {
            hook("defineDocument")?.call::<_, ()>(()).map_err(|e| fail(e, "pixelox:prelude"))?;
        }
        for lib in &job.libs {
            if lib.module {
                let ns = (|| -> rquickjs::Result<Object> {
                    let m = Module::declare(ctx.clone(), lib.name.clone(), lib.source.clone())?;
                    let (m, done) = m.eval()?;
                    done.finish::<()>()?;
                    m.namespace()
                })()
                .map_err(|e| fail(e, &lib.name))?;
                hook("exposeModule")?.call::<_, ()>((job.preset, ns)).map_err(|e| fail(e, &lib.name))?;
            } else {
                if job.preset.is_some() {
                    hook("beginLib")?.call::<_, ()>(()).map_err(|e| fail(e, &lib.name))?;
                }
                ctx.eval_with_options::<(), _>(lib.source.as_bytes(), opts(&lib.name))
                    .map_err(|e| fail(e, &lib.name))?;
                if job.preset.is_some() {
                    hook("endLib")?.call::<_, bool>((job.preset,)).map_err(|e| fail(e, &lib.name))?;
                }
            }
        }
        if let Some(global) = job.preset {
            let defined: bool = ctx
                .eval::<bool, _>(format!("typeof globalThis[{}] !== 'undefined'", Json::String(global.into())))
                .map_err(|e| fail(e, "pixelox:prelude"))?;
            if !defined {
                let libs: Vec<&str> = job.libs.iter().map(|l| l.name.as_str()).collect();
                return Err(Error::new(
                    "PX1520",
                    format!("the --lib bundle(s) {} did not define a global `{global}`", libs.join(", ")),
                )
                .with_hint(format!(
                    "pass a UMD/CommonJS or ES-module build that defines `{global}`, e.g. --lib {}",
                    preset_file(global)
                ))
                .with_context("global", global));
            }
        }
        ctx.eval_with_options::<(), _>(job.script.as_bytes(), opts(&job.script_name))
            .map_err(|e| fail(e, &job.script_name))?;
        while ctx.execute_pending_job() {
            if interrupted.load(Ordering::Relaxed) {
                break;
            }
        }
        if interrupted.load(Ordering::Relaxed) {
            return Err(timeout_error(job.timeout));
        }
        Ok(())
    });
    outcome?;
    let mut st = state.borrow_mut();
    let canvases = st.canvases.len();
    let Some(output) = st.output.take() else {
        if let Some(reason) = st.rejection.take() {
            return Err(Error::new("PX6001", format!("{}: unhandled promise rejection: {reason}", job.script_name))
                .with_context("file", &job.script_name));
        }
        return Err(Error::new("PX6020", format!("{} finished without calling px.output()", job.script_name))
            .with_hint("end the script with px.output(canvas) or px.output(svgDocument)"));
    };
    Ok(JsResult {
        output,
        logs: std::mem::take(&mut st.logs),
        logs_dropped: st.logs_dropped,
        text_calls: st.text_calls,
        time_read: st.time_read,
        canvases,
    })
}

fn preset_file(global: &str) -> &'static str {
    if global == "d3" {
        "d3.v7.min.js"
    } else {
        "three.module.js"
    }
}

fn rand_chacha_from(seed: [u8; 32]) -> rand_chacha::ChaCha12Rng {
    use rand_core::SeedableRng;
    rand_chacha::ChaCha12Rng::from_seed(seed)
}

fn timeout_error(t: Duration) -> Error {
    Error::new("PX6002", format!("the script exceeded its {:.3}s wall-clock limit", t.as_secs_f64()))
        .with_hint(format!("raise the limit with --timeout {}", (t.as_secs_f64() * 4.0).ceil().max(1.0)))
        .with_context("timeout_s", t.as_secs_f64())
}

fn describe_value<'js>(ctx: &Ctx<'js>, v: &Value<'js>) -> String {
    if let Some(o) = v.as_object() {
        let name: Option<String> = o.get("name").ok();
        let msg: Option<String> = o.get("message").ok();
        if let Some(m) = msg {
            return match name {
                Some(n) if !n.is_empty() => format!("{n}: {m}"),
                _ => m,
            };
        }
    }
    ctx.json_stringify(v.clone())
        .ok()
        .flatten()
        .and_then(|s| s.to_string().ok())
        .or_else(|| v.as_string().and_then(|s| s.to_string().ok()))
        .unwrap_or_else(|| format!("{v:?}"))
}

/// `file:line:col` of the first stack frame in `file`.
fn locate(stack: &str, file: &str) -> Option<(u32, u32)> {
    let needle = format!("{file}:");
    let mut rest = stack;
    while let Some(i) = rest.find(&needle) {
        let after = &rest[i + needle.len()..];
        let mut parts = after.splitn(3, |c: char| !c.is_ascii_digit());
        let line = parts.next().and_then(|s| s.parse::<u32>().ok());
        let col = after
            .split_once(':')
            .map(|(_, c)| c.chars().take_while(|ch| ch.is_ascii_digit()).collect::<String>())
            .and_then(|s| s.parse::<u32>().ok());
        if let Some(l) = line {
            return Some((l, col.unwrap_or(0)));
        }
        rest = &rest[i + needle.len()..];
    }
    None
}

fn map_error(ctx: &Ctx, e: rquickjs::Error, file: &str, interrupted: &AtomicBool) -> Error {
    let timeout = interrupted.load(Ordering::Relaxed);
    if let rquickjs::Error::Allocation = e {
        return mem_error("QuickJS could not allocate memory");
    }
    if !matches!(e, rquickjs::Error::Exception) {
        if timeout {
            return Error::new("PX6002", "the script exceeded its wall-clock limit").with_hint("raise --timeout");
        }
        return Error::new("PX6001", format!("{file}: {e}")).with_context("file", file);
    }
    let v = ctx.catch();
    let (name, message, stack) = match v.as_object() {
        Some(o) if v.is_error() || o.get::<_, Option<String>>("message").ok().flatten().is_some() => (
            o.get::<_, Option<String>>("name").ok().flatten().unwrap_or_default(),
            o.get::<_, Option<String>>("message").ok().flatten().unwrap_or_default(),
            o.get::<_, Option<String>>("stack").ok().flatten().unwrap_or_default(),
        ),
        _ => (String::new(), describe_value(ctx, &v), String::new()),
    };
    if timeout || (name == "InternalError" && message == "interrupted") {
        return Error::new("PX6002", "the script exceeded its wall-clock limit (--timeout)")
            .with_hint("raise --timeout or simplify the script");
    }
    if message.contains("out of memory") || message.starts_with("PX6003") {
        return mem_error(message.trim_start_matches("PX6003: "));
    }
    for code in ["PX6010", "PX1104"] {
        if let Some(rest) = message.strip_prefix(code) {
            let msg = rest.trim_start_matches(':').trim();
            let mut err = Error::new(code, msg.to_string()).with_context("file", file);
            if let Some((l, c)) = locate(&stack, file) {
                err = err.with_context("line", l).with_context("column", c);
            }
            return err;
        }
    }
    // the frame may belong to the script even when a library threw
    let (at_file, loc) = match locate(&stack, file) {
        Some(lc) => (file.to_string(), Some(lc)),
        None => (file.to_string(), None),
    };
    let label = if name.is_empty() { message.clone() } else { format!("{name}: {message}") };
    let mut err = match loc {
        Some((l, c)) => Error::new("PX6001", format!("{at_file}:{l}:{c}: {label}"))
            .with_context("line", l)
            .with_context("column", c),
        None => Error::new("PX6001", format!("{at_file}: {label}")),
    };
    err = err.with_context("file", at_file);
    if !name.is_empty() {
        err = err.with_context("error", name);
    }
    if !stack.trim().is_empty() {
        err = err.with_context("stack", stack.trim());
    }
    err
}

fn mem_error(msg: &str) -> Error {
    Error::new("PX6003", format!("the script ran out of memory: {msg}")).with_hint("raise the limit with --js-mem (MB)")
}

fn throw(ctx: &Ctx, msg: &str) -> rquickjs::Error {
    Exception::throw_range(ctx, msg)
}

/// The native functions handed to the prelude (never placed on the global object).
fn natives<'js>(
    ctx: &Ctx<'js>,
    st: Rc<RefCell<State>>,
    rng: Rc<RefCell<rand_chacha::ChaCha12Rng>>,
) -> rquickjs::Result<Object<'js>> {
    let n = Object::new(ctx.clone())?;

    let s = st.clone();
    n.set(
        "cvNew",
        Function::new(ctx.clone(), move |ctx: Ctx<'js>, w: f64, h: f64| -> rquickjs::Result<u32> {
            let mut s = s.borrow_mut();
            if !(w >= 1.0 && h >= 1.0 && w <= 65535.0 && h <= 65535.0) {
                return Err(throw(&ctx, &format!("px.canvas({w}, {h}): sizes must be 1..65535")));
            }
            let (w, h) = (w as u32, h as u32);
            let px = w as u64 * h as u64;
            if px > s.pixel_limit {
                return Err(throw(
                    &ctx,
                    &format!("PX1104: canvas {w}x{h} exceeds the pixel limit of {}", s.pixel_limit),
                ));
            }
            if s.canvas_bytes + px * 4 > s.mem_limit {
                return Err(throw(&ctx, &format!("PX6003: canvas {w}x{h} would exceed the --js-mem budget")));
            }
            let c = Canvas::new(w, h).ok_or_else(|| throw(&ctx, "could not allocate the canvas"))?;
            s.canvas_bytes += px * 4;
            s.canvases.push(c);
            Ok((s.canvases.len() - 1) as u32)
        })?,
    )?;

    let s = st.clone();
    n.set(
        "cvOp",
        Function::new(ctx.clone(), move |id: u32, code: u32, args: Rest<f64>| {
            let mut s = s.borrow_mut();
            let State { canvases, grads, .. } = &mut *s;
            if let Some(c) = canvases.get_mut(id as usize) {
                c.op(code, &args.0, grads);
            }
        })?,
    )?;

    let s = st.clone();
    n.set(
        "cvColor",
        Function::new(ctx.clone(), move |id: u32, which: u32, css: String| -> bool {
            s.borrow_mut().canvases.get_mut(id as usize).map(|c| c.set_color(which, &css)).unwrap_or(false)
        })?,
    )?;

    let s = st.clone();
    n.set(
        "cvGrad",
        Function::new(ctx.clone(), move |id: u32, which: u32, gid: u32| {
            if let Some(c) = s.borrow_mut().canvases.get_mut(id as usize) {
                c.set_gradient(which, gid as usize);
            }
        })?,
    )?;

    let s = st.clone();
    n.set(
        "cvStr",
        Function::new(ctx.clone(), move |id: u32, prop: u32, v: String| -> bool {
            s.borrow_mut().canvases.get_mut(id as usize).map(|c| c.set_str(prop, &v)).unwrap_or(false)
        })?,
    )?;

    let s = st.clone();
    n.set(
        "cvDash",
        Function::new(ctx.clone(), move |id: u32, dash: Rest<f64>| {
            if let Some(c) = s.borrow_mut().canvases.get_mut(id as usize) {
                c.set_dash(&dash.0);
            }
        })?,
    )?;

    let s = st.clone();
    n.set(
        "cvGetTransform",
        Function::new(ctx.clone(), move |id: u32| -> Vec<f64> {
            s.borrow()
                .canvases
                .get(id as usize)
                .map(|c| c.transform().to_vec())
                .unwrap_or_else(|| vec![1.0, 0.0, 0.0, 1.0, 0.0, 0.0])
        })?,
    )?;

    let s = st.clone();
    n.set(
        "cvDraw",
        Function::new(ctx.clone(), move |id: u32, src: u32, args: Rest<f64>| {
            let mut s = s.borrow_mut();
            let Some(source) = s.canvases.get(src as usize).map(|c| c.pixmap.clone()) else { return };
            if let Some(c) = s.canvases.get_mut(id as usize) {
                c.draw_canvas(&source, &args.0);
            }
        })?,
    )?;

    let s = st.clone();
    n.set(
        "cvGetImage",
        Function::new(
            ctx.clone(),
            move |ctx: Ctx<'js>, id: u32, x: i32, y: i32, w: u32, h: u32| -> rquickjs::Result<ArrayBuffer<'js>> {
                let s = s.borrow();
                let c = s.canvases.get(id as usize).ok_or_else(|| throw(&ctx, "unknown canvas"))?;
                if w as u64 * h as u64 > s.pixel_limit {
                    return Err(throw(&ctx, "PX1104: getImageData area exceeds the pixel limit"));
                }
                ArrayBuffer::new(ctx.clone(), c.get_image(x, y, w, h))
            },
        )?,
    )?;

    let s = st.clone();
    n.set(
        "cvPutImage",
        Function::new(ctx.clone(), move |id: u32, buf: ArrayBuffer<'js>, w: u32, h: u32, dx: i32, dy: i32| {
            // SAFETY: the bytes are copied before any JavaScript can run again.
            let data = unsafe { buf.as_bytes() }.map(|b| b.to_vec()).unwrap_or_default();
            if let Some(c) = s.borrow_mut().canvases.get_mut(id as usize) {
                c.put_image(&data, w, h, dx, dy);
            }
        })?,
    )?;

    let s = st.clone();
    n.set("cvText", Function::new(ctx.clone(), move |_id: u32| s.borrow_mut().text_calls += 1)?)?;

    let s = st.clone();
    n.set(
        "gradNew",
        Function::new(ctx.clone(), move |kind: u32, args: Rest<f64>| -> u32 {
            let mut s = s.borrow_mut();
            s.grads.push(canvas::new_gradient(kind == 1, &args.0));
            (s.grads.len() - 1) as u32
        })?,
    )?;

    let s = st.clone();
    n.set(
        "gradStop",
        Function::new(ctx.clone(), move |gid: u32, offset: f64, css: String| -> bool {
            s.borrow_mut().grads.get_mut(gid as usize).map(|g| g.add_stop(offset, &css)).unwrap_or(false)
        })?,
    )?;

    let s = st.clone();
    n.set(
        "outCanvas",
        Function::new(ctx.clone(), move |ctx: Ctx<'js>, id: u32| -> rquickjs::Result<()> {
            let mut s = s.borrow_mut();
            let p =
                s.canvases.get(id as usize).map(|c| c.pixmap.clone()).ok_or_else(|| throw(&ctx, "unknown canvas"))?;
            s.output = Some(JsOutput::Canvas(p));
            Ok(())
        })?,
    )?;

    let s = st.clone();
    n.set("outSvg", Function::new(ctx.clone(), move |svg: String| s.borrow_mut().output = Some(JsOutput::Svg(svg)))?)?;

    let s = st.clone();
    n.set(
        "log",
        Function::new(ctx.clone(), move |msg: String| {
            let mut s = s.borrow_mut();
            if s.logs.len() < MAX_LOG_LINES {
                s.logs.push(msg);
            } else {
                s.logs_dropped += 1;
            }
        })?,
    )?;

    let s = st.clone();
    n.set("timeRead", Function::new(ctx.clone(), move || s.borrow_mut().time_read = true)?)?;

    n.set(
        "random",
        Function::new(ctx.clone(), move || -> f64 {
            (rng.borrow_mut().next_u64() >> 11) as f64 / (1u64 << 53) as f64
        })?,
    )?;

    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locates_frames() {
        assert_eq!(locate("    at f (chart.js:12:5)\n    at <eval> (chart.js:20:1)", "chart.js"), Some((12, 5)));
        assert_eq!(locate("    at chart.js:3:9\n", "chart.js"), Some((3, 9)));
        assert_eq!(locate("    at lib.js:3:9\n", "chart.js"), None);
    }
}
