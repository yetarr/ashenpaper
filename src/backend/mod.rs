use std::path::Path;

use serde::Deserialize;

use crate::backend::{hypr::HyprpaperBackend, mpv::MpvpaperBackend};

pub mod hypr;
pub mod mpv;

#[derive(Deserialize)]
pub struct Monitor {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

pub enum BackendT {
    Hyprpaper(HyprpaperBackend),
    Mpvpaper(MpvpaperBackend),
    None,
}

impl Backend for BackendT {
    fn apply(&self, monitors: &[Monitor], path: &str) -> anyhow::Result<()> {
        match self {
            BackendT::Hyprpaper(h) => h.apply(monitors, path),
            BackendT::Mpvpaper(m) => m.apply(monitors, path),
            BackendT::None => Ok(()),
        }
    }

    fn stop(&self) -> anyhow::Result<()> {
        match self {
            BackendT::Hyprpaper(h) => h.stop(),
            BackendT::Mpvpaper(m) => m.stop(),
            BackendT::None => Ok(()),
        }
    }
}

pub enum WallpaperKind {
    Static,
    Video,
}

pub fn kind_of(path: &Path) -> WallpaperKind {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .as_deref()
    {
        Some("mp4" | "webm" | "mkv" | "mov" | "gif") => WallpaperKind::Video,
        _ => WallpaperKind::Static,
    }
}

pub trait Backend {
    fn apply(&self, monitors: &[Monitor], path: &str) -> anyhow::Result<()>;
    fn stop(&self) -> anyhow::Result<()>;
}
