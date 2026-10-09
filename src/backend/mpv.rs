use std::{
    fs,
    io::{BufRead, BufReader, Write},
    os::unix::net::UnixStream,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use anyhow::{bail, Context, Result};

use crate::backend::{Backend, Monitor};

const SOCK_PREFIX: &str = "ashenpaper-mpv-";

pub struct MpvpaperBackend {}

fn runtime_dir() -> PathBuf {
    std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
}

fn sock_path(monitor: &str) -> PathBuf {
    runtime_dir().join(format!("{SOCK_PREFIX}{monitor}.sock"))
}

/// Tell a running mpvpaper to switch files via mpv's JSON IPC.
fn send_loadfile(mut stream: UnixStream, path: &str) -> Result<()> {
    stream.set_read_timeout(Some(Duration::from_millis(500)))?;
    let cmd = serde_json::json!({ "command": ["loadfile", path, "replace"] });
    writeln!(stream, "{cmd}")?;

    // mpv may interleave event lines; the reply is the one with an "error" key
    for line in BufReader::new(stream).lines() {
        let v: serde_json::Value = serde_json::from_str(&line?)?;
        if let Some(err) = v.get("error") {
            if err == "success" {
                return Ok(());
            }
            bail!("mpv rejected loadfile: {err}");
        }
    }
    bail!("mpv closed the socket without replying")
}

fn spawn(monitor: &str, sock: &Path, path: &str) -> Result<()> {
    let opts = format!("no-audio loop input-ipc-server={}", sock.display());
    let status = Command::new("mpvpaper")
        .args(["-f", "-o", &opts, monitor, path])
        .status()
        .context("failed to run mpvpaper (is it installed and on PATH?)")?;
    if !status.success() {
        bail!("mpvpaper exited with {status}");
    }
    Ok(())
}

impl Backend for MpvpaperBackend {
    fn apply(&self, monitors: &[Monitor], path: &str) -> Result<()> {
        for mon in monitors {
            let sock = sock_path(&mon.name);
            match UnixStream::connect(&sock) {
                // already running on this output: switch in place, no flicker
                Ok(stream) => send_loadfile(stream, path)?,
                // nothing listening: clear any stale socket file and start fresh
                Err(_) => {
                    let _ = fs::remove_file(&sock);
                    spawn(&mon.name, &sock, path)?;
                }
            }
        }
        Ok(())
    }

    fn stop(&self) -> Result<()> {
        // -x matches the exact process name; -f's fork means we have no pids to track
        let _ = Command::new("pkill").args(["-x", "mpvpaper"]).status();
        if let Ok(entries) = fs::read_dir(runtime_dir()) {
            for e in entries.flatten() {
                if e.file_name().to_string_lossy().starts_with(SOCK_PREFIX) {
                    let _ = fs::remove_file(e.path());
                }
            }
        }
        Ok(())
    }
}