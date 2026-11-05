//! Progress on stderr (PRD §6.1): a bar on a terminal, NDJSON events with
//! `--progress json`, or nothing. stdout stays reserved for the single envelope.

use pixelox_core::ctx::Progress;
use serde_json::{json, Value};
use std::io::{IsTerminal, Write};
use std::sync::Mutex;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    None,
    Bar,
    Json,
}

pub struct StderrProgress {
    mode: Mode,
    verbosity: i32,
    state: Mutex<State>,
}

struct State {
    started: Instant,
    last_draw: Option<Instant>,
    drew_bar: bool,
}

impl StderrProgress {
    pub fn new(mode: &str, verbosity: i32) -> StderrProgress {
        let mode = match mode {
            "none" => Mode::None,
            "json" => Mode::Json,
            "bar" => Mode::Bar,
            _ => {
                if std::io::stderr().is_terminal() && verbosity >= 0 {
                    Mode::Bar
                } else {
                    Mode::None
                }
            }
        };
        StderrProgress {
            mode,
            verbosity,
            state: Mutex::new(State { started: Instant::now(), last_draw: None, drew_bar: false }),
        }
    }

    fn emit(&self, v: Value) {
        let mut e = std::io::stderr().lock();
        let _ = writeln!(e, "{v}");
    }

    fn clear_bar(&self, st: &mut State) {
        if st.drew_bar {
            let _ = write!(std::io::stderr(), "\r\x1b[2K");
            st.drew_bar = false;
        }
    }

    pub fn finish(&self) {
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        self.clear_bar(&mut st);
    }
}

impl Progress for StderrProgress {
    fn stage(&self, stage: &str, index: usize, of: usize) {
        match self.mode {
            Mode::Json => self.emit(json!({"event": "stage", "step": stage, "index": index, "of": of})),
            Mode::Bar if self.verbosity > 0 => {
                let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
                self.clear_bar(&mut st);
                eprintln!("[{index}/{of}] {stage}");
            }
            _ => {}
        }
    }

    fn progress(&self, stage: &str, fraction: f64) {
        match self.mode {
            Mode::Json => self.emit(json!({"event": "progress", "command": stage.replace(' ', "."), "pct": (fraction * 1000.0).round() / 10.0})),
            Mode::Bar => {
                let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
                let now = Instant::now();
                if fraction < 1.0 && st.last_draw.map(|t| now.duration_since(t).as_millis() < 100).unwrap_or(false) {
                    return;
                }
                if now.duration_since(st.started).as_millis() < 300 && fraction < 1.0 {
                    return;
                }
                st.last_draw = Some(now);
                let width = 30;
                let filled = ((fraction * width as f64).round() as usize).min(width);
                let _ = write!(std::io::stderr(), "\r\x1b[2K{stage} [{}{}] {:>5.1}%", "#".repeat(filled), "-".repeat(width - filled), fraction * 100.0);
                st.drew_bar = true;
            }
            Mode::None => {}
        }
    }

    fn output(&self, path: &str) {
        if self.mode == Mode::Json {
            self.emit(json!({"event": "output", "path": path}));
        }
    }

    fn warning(&self, code: &str, message: &str) {
        match self.mode {
            Mode::Json => self.emit(json!({"event": "warning", "code": code, "message": message})),
            _ if self.verbosity >= 0 => {
                let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
                self.clear_bar(&mut st);
                eprintln!("warning: {code}: {message}");
            }
            _ => {}
        }
    }

    fn log(&self, message: &str) {
        if self.verbosity > 0 && self.mode != Mode::Json {
            let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
            self.clear_bar(&mut st);
            eprintln!("{message}");
        }
    }

    fn event(&self, event: &Value) {
        if self.mode == Mode::Json {
            self.emit(event.clone());
        }
    }
}
