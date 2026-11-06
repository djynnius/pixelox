//! Resumable, verified model downloads (ported from ferraus-models).
//!
//! * Partial data lives in `<dest>.part`; a retry sends `Range: bytes=<have>-` and appends
//!   when the server answers `206`, so completed ranges are never fetched twice.
//! * The finished file is hashed (SHA-256) before it is renamed into place. A mismatch
//!   deletes the partial file and fails with PX1503; network failures fail with PX1504 and
//!   keep the partial data for the next attempt.

use pixelox_core::{Error, Result};
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Attempts while each attempt makes progress (resume after a dropped connection).
const RESUME_ATTEMPTS: usize = 5;

pub struct Request<'a> {
    pub url: &'a str,
    pub sha256: &'a str,
    pub bytes: u64,
    pub dest: &'a Path,
    /// Progress lines for stderr.
    pub log: &'a dyn Fn(&str),
    /// Fraction done (0..1).
    pub progress: &'a dyn Fn(f64),
    /// Called between chunks; an error aborts (cancellation).
    pub check_cancel: &'a dyn Fn() -> Result<()>,
}

#[derive(Debug, Default)]
pub struct Report {
    pub bytes_fetched: u64,
    pub sha256: String,
    pub size: u64,
}

enum Fetch {
    Retry { progressed: bool, message: String },
    Fail(String),
    Abort(Error),
}

pub fn part_path(dest: &Path) -> PathBuf {
    let mut s = dest.as_os_str().to_owned();
    s.push(".part");
    PathBuf::from(s)
}

pub fn sha256_file(path: &Path) -> Result<String> {
    let mut f = std::fs::File::open(path)?;
    let mut h = Sha256::new();
    let mut buf = vec![0u8; 1 << 16];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(hex::encode(h.finalize()))
}

fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .timeout_connect(Some(Duration::from_secs(20)))
        .timeout_recv_response(Some(Duration::from_secs(60)))
        .timeout_recv_body(None)
        .user_agent(format!("pixelox/{}", pixelox_core::PIXELOX_VERSION))
        .build()
        .into()
}

fn failed(url: &str, msg: &str) -> Error {
    Error::new("PX1504", format!("could not download {url}: {msg}"))
        .with_hint("check the network connection and retry; the partial download is kept and resumed")
        .with_context("url", url)
}

/// Download `req.url` to `req.dest`, verifying its SHA-256.
pub fn download(req: Request<'_>) -> Result<Report> {
    if let Some(parent) = req.dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let part = part_path(req.dest);
    let agent = agent();
    let mut report = Report::default();
    let mut attempts = 0;
    loop {
        (req.check_cancel)()?;
        attempts += 1;
        let have = std::fs::metadata(&part).map(|m| m.len()).unwrap_or(0);
        if have > 0 {
            (req.log)(&format!("resuming {} from byte {have}", req.url));
        } else {
            (req.log)(&format!("downloading {} ({} MB)", req.url, req.bytes / 1_000_000));
        }
        match fetch_once(&agent, &req, &part, have, &mut report.bytes_fetched) {
            Ok(()) => break,
            Err(Fetch::Abort(e)) => return Err(e),
            Err(Fetch::Fail(m)) => return Err(failed(req.url, &m)),
            Err(Fetch::Retry { progressed, message }) => {
                (req.log)(&format!("{}: {message}", req.url));
                if !progressed || attempts >= RESUME_ATTEMPTS {
                    return Err(failed(req.url, &message));
                }
            }
        }
    }
    let digest = sha256_file(&part)?;
    if !digest.eq_ignore_ascii_case(req.sha256) {
        let _ = std::fs::remove_file(&part);
        return Err(Error::new(
            "PX1503",
            format!("{} failed SHA-256 verification (got {digest}, expected {})", req.url, req.sha256),
        )
        .with_hint("retry once; a repeated mismatch means the source changed or is corrupted")
        .with_context("url", req.url));
    }
    report.size = std::fs::metadata(&part)?.len();
    report.sha256 = digest;
    std::fs::rename(&part, req.dest)?;
    Ok(report)
}

/// One HTTP request, appending to `part` from offset `have`.
fn fetch_once(
    agent: &ureq::Agent,
    req: &Request<'_>,
    part: &Path,
    have: u64,
    fetched: &mut u64,
) -> std::result::Result<(), Fetch> {
    let mut r = agent.get(req.url);
    if have > 0 {
        r = r.header("Range", format!("bytes={have}-"));
    }
    let resp = match r.call() {
        Ok(r) => r,
        Err(ureq::Error::Io(e)) => return Err(Fetch::Retry { progressed: false, message: e.to_string() }),
        Err(ureq::Error::Timeout(t)) => {
            return Err(Fetch::Retry { progressed: false, message: format!("timeout ({t})") })
        }
        Err(e) => return Err(Fetch::Fail(e.to_string())),
    };
    let status = resp.status().as_u16();
    let append = match status {
        206 => {
            let start = resp
                .headers()
                .get("content-range")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("bytes "))
                .and_then(|v| v.split('-').next())
                .and_then(|v| v.trim().parse::<u64>().ok());
            if start != Some(have) {
                let _ = std::fs::remove_file(part);
                return Err(Fetch::Retry { progressed: true, message: "unexpected Content-Range; restarting".into() });
            }
            true
        }
        200 => false,
        416 if have > 0 => return Ok(()),
        s if s >= 500 => return Err(Fetch::Retry { progressed: false, message: format!("HTTP {s}") }),
        s => return Err(Fetch::Fail(format!("HTTP {s}"))),
    };
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(append)
        .truncate(!append)
        .open(part)
        .map_err(|e| Fetch::Abort(e.into()))?;
    let start = if append { have } else { 0 };
    let mut done = start;
    let mut reader = resp.into_body().into_reader();
    let mut buf = vec![0u8; 1 << 16];
    let mut progressed = false;
    let mut last_tick = 0u64;
    loop {
        (req.check_cancel)().map_err(Fetch::Abort)?;
        let n = match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => {
                let _ = file.flush();
                return Err(Fetch::Retry { progressed, message: format!("connection interrupted: {e}") });
            }
        };
        file.write_all(&buf[..n]).map_err(|e| Fetch::Abort(e.into()))?;
        *fetched += n as u64;
        done += n as u64;
        progressed = true;
        if req.bytes > 0 && done - last_tick >= 4 << 20 {
            last_tick = done;
            (req.progress)((done as f64 / req.bytes as f64).min(1.0));
        }
    }
    file.flush().map_err(|e| Fetch::Abort(e.into()))?;
    (req.progress)(1.0);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::net::TcpListener;

    /// Serve `body` once per connection, honoring Range; the first response is cut after
    /// `cut` bytes when given (a dropped connection).
    fn serve(body: Vec<u8>, cut: Option<usize>, connections: usize) -> String {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = l.local_addr().unwrap();
        std::thread::spawn(move || {
            for (i, s) in l.incoming().take(connections).enumerate() {
                let mut s = s.unwrap();
                let mut start = 0usize;
                let mut reader = BufReader::new(s.try_clone().unwrap());
                loop {
                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap() == 0 || line == "\r\n" {
                        break;
                    }
                    if let Some(v) = line.to_ascii_lowercase().strip_prefix("range: bytes=") {
                        start = v.trim().trim_end_matches('-').parse().unwrap();
                    }
                }
                let slice = &body[start..];
                let head = if start > 0 {
                    format!(
                        "HTTP/1.1 206 Partial Content\r\nContent-Length: {}\r\nContent-Range: bytes {start}-{}/{}\r\n\r\n",
                        slice.len(),
                        body.len() - 1,
                        body.len()
                    )
                } else {
                    format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", slice.len())
                };
                s.write_all(head.as_bytes()).unwrap();
                let n = if i == 0 { cut.unwrap_or(slice.len()) } else { slice.len() };
                let _ = s.write_all(&slice[..n]);
            }
        });
        format!("http://{addr}/m.onnx")
    }

    fn sha(b: &[u8]) -> String {
        hex::encode(Sha256::digest(b))
    }

    fn get(url: &str, want: &str, len: u64, dest: &Path) -> Result<Report> {
        download(Request {
            url,
            sha256: want,
            bytes: len,
            dest,
            log: &|_| {},
            progress: &|_| {},
            check_cancel: &|| Ok(()),
        })
    }

    #[test]
    fn downloads_resumes_and_verifies() {
        let body: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
        let dir = pixelox_core::testing::TempDir::new("ml-dl");
        let dest = dir.path("m.onnx");
        // the first connection drops halfway; the retry resumes with Range
        let url = serve(body.clone(), Some(90_000), 2);
        let r = get(&url, &sha(&body), body.len() as u64, &dest).unwrap();
        assert_eq!(std::fs::read(&dest).unwrap(), body);
        assert_eq!(r.sha256, sha(&body));
        assert!(!part_path(&dest).exists());
        // a wrong hash is refused and leaves nothing behind
        let dest2 = dir.path("bad.onnx");
        let url = serve(body.clone(), None, 1);
        let e = get(&url, &"0".repeat(64), body.len() as u64, &dest2).unwrap_err();
        assert_eq!(e.code, "PX1503");
        assert!(!dest2.exists() && !part_path(&dest2).exists());
    }

    #[test]
    fn http_errors_are_download_failures() {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = l.local_addr().unwrap();
        std::thread::spawn(move || {
            let (mut s, _) = l.accept().unwrap();
            let mut buf = [0u8; 1024];
            let _ = s.read(&mut buf);
            let _ = s.write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n");
        });
        let dir = pixelox_core::testing::TempDir::new("ml-dl404");
        let e = get(&format!("http://{addr}/x"), &"0".repeat(64), 1, &dir.path("x.onnx")).unwrap_err();
        assert_eq!(e.code, "PX1504");
    }
}
