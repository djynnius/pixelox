//! Per-platform directories. `PIXELOX_HOME` overrides all of them (config, cache, data,
//! state); `PIXELOX_CONFIG` overrides the config file.

use std::path::PathBuf;

fn home() -> PathBuf {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn env_path(k: &str) -> Option<PathBuf> {
    std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from)
}

/// Directory holding `config.toml`.
pub fn config_dir() -> PathBuf {
    if let Some(c) = env_path("PIXELOX_CONFIG") {
        if let Some(p) = c.parent() {
            return p.to_path_buf();
        }
    }
    if let Some(h) = env_path("PIXELOX_HOME") {
        return h.join("config");
    }
    if cfg!(target_os = "macos") {
        home().join("Library/Application Support/pixelox")
    } else if cfg!(windows) {
        env_path("APPDATA").unwrap_or_else(home).join("pixelox")
    } else {
        env_path("XDG_CONFIG_HOME").unwrap_or_else(|| home().join(".config")).join("pixelox")
    }
}

pub fn config_file() -> PathBuf {
    env_path("PIXELOX_CONFIG").unwrap_or_else(|| config_dir().join("config.toml"))
}

/// Cache: downloaded models.
pub fn cache_dir() -> PathBuf {
    if let Some(h) = env_path("PIXELOX_HOME") {
        return h.join("cache");
    }
    if cfg!(target_os = "macos") {
        home().join("Library/Caches/pixelox")
    } else if cfg!(windows) {
        env_path("LOCALAPPDATA").unwrap_or_else(home).join("pixelox").join("cache")
    } else {
        env_path("XDG_CACHE_HOME").unwrap_or_else(|| home().join(".cache")).join("pixelox")
    }
}

/// Data: model weights, registered fonts.
pub fn data_dir() -> PathBuf {
    if let Some(h) = env_path("PIXELOX_HOME") {
        return h.join("data");
    }
    if cfg!(target_os = "macos") {
        home().join("Library/Application Support/pixelox/data")
    } else if cfg!(windows) {
        env_path("LOCALAPPDATA").unwrap_or_else(home).join("pixelox").join("data")
    } else {
        env_path("XDG_DATA_HOME").unwrap_or_else(|| home().join(".local/share")).join("pixelox")
    }
}

/// State: background jobs and the MCP audit log (PRD §7.13).
pub fn state_dir() -> PathBuf {
    if let Some(h) = env_path("PIXELOX_HOME") {
        return h.join("state");
    }
    if cfg!(target_os = "macos") {
        home().join("Library/Application Support/pixelox/state")
    } else if cfg!(windows) {
        env_path("LOCALAPPDATA").unwrap_or_else(home).join("pixelox").join("state")
    } else {
        env_path("XDG_STATE_HOME").unwrap_or_else(|| home().join(".local/state")).join("pixelox")
    }
}

pub fn models_dir() -> PathBuf {
    data_dir().join("models")
}

pub fn jobs_dir() -> PathBuf {
    state_dir().join("jobs")
}

/// Font folders registered with `pixelox fonts add` (one path per line).
pub fn fonts_file() -> PathBuf {
    config_dir().join("fonts.txt")
}

pub fn audit_log() -> PathBuf {
    state_dir().join("mcp-audit.ndjson")
}
