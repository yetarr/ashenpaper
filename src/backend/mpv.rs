use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use anyhow::{Context, Result, bail};

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

fn send_loadfile(mut stream: UnixStream, path: &str) -> Result<()> {
    stream.set_read_timeout(Some(Duration::from_millis(500)))?;
    let cmd = serde_json::json!({ "command": ["loadfile", path, "replace"] });
    writeln!(stream, "{cmd}")?;

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
                Ok(stream) => send_loadfile(stream, path)?,
                Err(_) => {
                    let _ = fs::remove_file(&sock);
                    spawn(&mon.name, &sock, path)?;
                }
            }
        }
        Ok(())
    }

    fn stop(&self) -> Result<()> {
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
