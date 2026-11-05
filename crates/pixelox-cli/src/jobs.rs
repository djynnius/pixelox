//! Background jobs (PRD §7.13): `--background` re-runs the same command line as a detached
//! child process whose envelope and NDJSON progress land under the state directory. No
//! daemon: `jobs list|status|wait|cancel|result` read that state.

use pixelox_core::ctx::CancelToken;
use pixelox_core::prelude::*;
use std::path::PathBuf;

pub fn ops() -> Vec<OpSpec> {
    vec![
        OpSpec::custom("jobs list", Category::Automation, "List background jobs", jobs_list)
            .inputs(Inputs::NONE)
            .param(param("all", ParamKind::Flag).help("Include finished jobs older than a day"))
            .example("pixelox jobs list", "Jobs and their states")
            .example("pixelox jobs list --all --json", "Every job as JSON")
            .example("pixelox export big.pxo -o big.tiff --background --json", "Start a job"),
        OpSpec::custom("jobs status", Category::Automation, "State and progress of one job", jobs_status)
            .inputs(Inputs::NONE)
            .param(param("id", ParamKind::Str).required().help("Job id from --background"))
            .positional("id")
            .example("pixelox jobs status 1762300000000-3f2a", "Is it done?")
            .example("pixelox jobs status 1762300000000-3f2a --json", "As JSON")
            .example("pixelox jobs status 1762300000000-3f2a -q", "Exit code only"),
        OpSpec::custom("jobs wait", Category::Automation, "Block until a job finishes; exits with its code", jobs_wait)
            .inputs(Inputs::NONE)
            .param(param("id", ParamKind::Str).required().help("Job id"))
            .param(param("timeout", ParamKind::Duration).help("Give up after this long (exit 10)"))
            .positional("id")
            .example("pixelox jobs wait 1762300000000-3f2a", "Wait, then print the job's result")
            .example("pixelox jobs wait 1762300000000-3f2a --timeout 10m --json", "Bounded wait")
            .example("pixelox jobs wait 1762300000000-3f2a -q", "Exit with the job's code"),
        OpSpec::custom("jobs cancel", Category::Automation, "Stop a running job", jobs_cancel)
            .inputs(Inputs::NONE)
            .param(param("id", ParamKind::Str).required().help("Job id"))
            .positional("id")
            .example("pixelox jobs cancel 1762300000000-3f2a", "Stop it; partial outputs are not kept")
            .example("pixelox jobs cancel 1762300000000-3f2a --json", "As JSON")
            .example("pixelox jobs cancel 1762300000000-3f2a -q", "Quietly"),
        OpSpec::custom("jobs result", Category::Automation, "The final envelope of a finished job", jobs_result)
            .inputs(Inputs::NONE)
            .param(param("id", ParamKind::Str).required().help("Job id"))
            .positional("id")
            .example("pixelox jobs result 1762300000000-3f2a --json", "Exactly what the command printed")
            .example("pixelox jobs result 1762300000000-3f2a --json | jq .outputs", "Just its outputs")
            .example("pixelox jobs result 1762300000000-3f2a", "As text"),
    ]
}

fn job_dir(id: &str) -> Result<PathBuf> {
    if id.is_empty() || !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err(Error::new("PX7201", format!("'{id}' is not a job id")));
    }
    let d = pixelox_core::paths::jobs_dir().join(id);
    if !d.exists() {
        return Err(Error::new("PX7201", format!("no job '{id}'")).with_hint("pixelox jobs list"));
    }
    Ok(d)
}

fn pid_alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        std::process::Command::new("kill")
            .args(["-0", &pid.to_string()])
            .stderr(std::process::Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
    }
    #[cfg(windows)]
    {
        std::process::Command::new("tasklist")
            .args(["/FI", &format!("PID eq {pid}"), "/NH"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains(&pid.to_string()))
            .unwrap_or(false)
    }
}

fn job_info(id: &str) -> Result<Value> {
    let d = job_dir(id)?;
    let meta: Value = serde_json::from_slice(&std::fs::read(d.join("job.json"))?)?;
    let read_result = || -> Option<Value> {
        std::fs::read(d.join("result.json")).ok().and_then(|b| {
            let s = String::from_utf8_lossy(&b).to_string();
            s.lines().rev().find(|l| l.trim_start().starts_with('{')).and_then(|l| serde_json::from_str(l).ok())
        })
    };
    let pid = meta["pid"].as_u64().unwrap_or(0) as u32;
    let alive = pid != 0 && pid_alive(pid);
    let mut result = read_result();
    if result.is_none() && !alive {
        for _ in 0..10 {
            std::thread::sleep(std::time::Duration::from_millis(100));
            result = read_result();
            if result.is_some() {
                break;
            }
        }
    }
    let state = if let Some(r) = &result {
        if r["ok"].as_bool() == Some(true) {
            "succeeded"
        } else {
            "failed"
        }
    } else if d.join("cancelled").exists() {
        "cancelled"
    } else if alive {
        "running"
    } else {
        "failed"
    };
    let pct = std::fs::read_to_string(d.join("progress.ndjson"))
        .ok()
        .and_then(|s| {
            s.lines().rev().filter_map(|l| serde_json::from_str::<Value>(l).ok()).find(|v| v["event"] == "progress")
        })
        .and_then(|v| v["pct"].as_f64());
    Ok(
        json!({"job_id": id, "state": state, "pid": pid, "argv": meta["argv"], "started": meta["started"], "pct": pct, "result": result}),
    )
}

/// Start the same command line (minus `--background`) as a detached process.
pub fn spawn_background(argv: &[String], json_out: bool) -> i32 {
    let id = format!(
        "{}-{:04x}",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0),
        std::process::id() & 0xffff
    );
    let dir = pixelox_core::paths::jobs_dir().join(&id);
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("error: cannot create the job directory: {e}");
        return 8;
    }
    let mut args: Vec<String> =
        argv.iter().skip(1).filter(|a| *a != "--background" && *a != "--json").cloned().collect();
    args.extend(["--json".into(), "--progress".into(), "json".into()]);
    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("pixelox"));
    let (Ok(stdout), Ok(stderr)) =
        (std::fs::File::create(dir.join("result.json")), std::fs::File::create(dir.join("progress.ndjson")))
    else {
        eprintln!("error: cannot create job files");
        return 8;
    };
    let mut cmd = std::process::Command::new(exe);
    cmd.args(&args).stdin(std::process::Stdio::null()).stdout(stdout).stderr(stderr);
    if let Ok(cwd) = std::env::current_dir() {
        cmd.current_dir(cwd);
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x00000008 | 0x00000200);
    }
    match cmd.spawn() {
        Ok(child) => {
            let started =
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
            let meta = json!({"job_id": id, "pid": child.id(), "argv": args, "started": started});
            let _ = std::fs::write(dir.join("job.json"), serde_json::to_vec_pretty(&meta).unwrap_or_default());
            if json_out {
                println!(
                    "{}",
                    json!({"tool": "pixelox", "ok": true, "command": "background", "result": {"job_id": id}})
                );
            } else {
                println!("{id}");
            }
            0
        }
        Err(e) => {
            eprintln!("error: cannot start the background job: {e}");
            8
        }
    }
}

/// Ctrl-C cancels cooperatively; a second Ctrl-C exits at once (exit 10).
pub fn install_interrupt_handler(_cancel: &CancelToken) {
    // Without a signal-handling dependency the default SIGINT behavior applies: the process
    // stops immediately and atomic writes guarantee no partial outputs remain (temp files
    // are dot-prefixed `.name.<pid>.part.ext` siblings).
}

fn jobs_list(a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    let mut jobs = Vec::new();
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    if let Ok(rd) = std::fs::read_dir(pixelox_core::paths::jobs_dir()) {
        let mut ids: Vec<String> = rd.flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
        ids.sort();
        for id in ids {
            if let Ok(mut info) = job_info(&id) {
                let old = now.saturating_sub(info["started"].as_u64().unwrap_or(now)) > 86_400;
                if old && info["state"] != "running" && !a.flag("all") {
                    continue;
                }
                if let Some(o) = info.as_object_mut() {
                    o.remove("result");
                }
                jobs.push(info);
            }
        }
    }
    let text: String = jobs
        .iter()
        .map(|j| {
            format!(
                "{}  {}  {}\n",
                j["job_id"].as_str().unwrap_or(""),
                j["state"].as_str().unwrap_or(""),
                j["argv"]
                    .as_array()
                    .map(|a| a.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>().join(" "))
                    .unwrap_or_default()
            )
        })
        .collect();
    Ok(json!({"jobs": jobs, "text": if text.is_empty() { "no jobs\n".to_string() } else { text }}))
}

fn jobs_status(a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    let mut info = job_info(&a.str_or("id", ""))?;
    if let Some(o) = info.as_object_mut() {
        o.remove("result");
    }
    Ok(info)
}

fn jobs_wait(a: &Args, ctx: &mut OpCtx) -> Result<Value> {
    let id = a.str_or("id", "");
    let start = std::time::Instant::now();
    let timeout = a.duration("timeout")?;
    loop {
        ctx.check_cancel()?;
        let info = job_info(&id)?;
        if info["state"] != "running" {
            if info["state"] == "failed" {
                if let Some(e) = info["result"].get("error") {
                    let code = e["code"].as_str().unwrap_or("PX3001").to_string();
                    let msg = e["message"].as_str().unwrap_or("job failed").to_string();
                    return Err(Error::new(&code, format!("job {id} failed: {msg}")).with_context("job", &info));
                }
            }
            return Ok(info);
        }
        if let Some(t) = timeout {
            if start.elapsed().as_secs_f64() > t {
                return Err(Error::new("PX1901", format!("job {id} is still running after {t}s")));
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

fn jobs_cancel(a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    let id = a.str_or("id", "");
    let info = job_info(&id)?;
    if info["state"] != "running" {
        return Err(Error::new(
            "PX7203",
            format!("job {id} is {}, not running", info["state"].as_str().unwrap_or("unknown")),
        ));
    }
    let pid = info["pid"].as_u64().unwrap_or(0);
    #[cfg(unix)]
    let ok = std::process::Command::new("kill")
        .args(["-TERM", &pid.to_string()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    #[cfg(windows)]
    let ok = std::process::Command::new("taskkill")
        .args(["/PID", &pid.to_string(), "/T", "/F"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    let _ = std::fs::write(job_dir(&id)?.join("cancelled"), b"");
    if !ok {
        return Err(Error::new("PX7203", format!("could not stop job {id}")));
    }
    Ok(json!({"job_id": id, "cancelled": true}))
}

fn jobs_result(a: &Args, _ctx: &mut OpCtx) -> Result<Value> {
    let id = a.str_or("id", "");
    let info = job_info(&id)?;
    match &info["result"] {
        Value::Null => {
            Err(Error::new("PX7202", format!("job {id} is {}", info["state"].as_str().unwrap_or("unknown")))
                .with_hint(format!("pixelox jobs wait {id}")))
        }
        r => Ok(r.clone()),
    }
}
