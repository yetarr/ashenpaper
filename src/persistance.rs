use std::{env, fs};

use anyhow::{self, Context};

use crate::backend::{Backend, BackendT, hypr::HyprpaperBackend, mpv::MpvpaperBackend};
use crate::utils;

const PERSIST_FILE: &str = "/.config/ashenpaper/persist.txt";
const CONFIG_DIR: &str = "/.config/ashenpaper/";

pub fn persist_wallpaper(backend: &BackendT, path: &str) -> anyhow::Result<()> {
    let home_dir = env::home_dir()
        .context("")?
        .into_string()
        .ok()
        .context("")?;
    if !fs::exists(format!("{home_dir}{CONFIG_DIR}"))? {
        create_persist_file()?;
    }
    if !fs::exists(format!("{home_dir}{PERSIST_FILE}"))? {
        create_persist_file()?;
    }

    let output = format!("{backend} {path}");
    fs::write(format!("{home_dir}{PERSIST_FILE}"), output)?;
    Ok(())
}

fn create_persist_file() -> anyhow::Result<()> {
    let home_dir = env::home_dir()
        .context("")?
        .into_string()
        .ok()
        .context("")?;
    let builder = fs::DirBuilder::new();
    builder.create(format!("{home_dir}{CONFIG_DIR}"))?;
    fs::File::create_new(format!("{home_dir}{PERSIST_FILE}"))?;
    Ok(())
}

fn read_last_wallpaper() -> anyhow::Result<(String, String)> {
    let home_dir = env::home_dir()
        .context("")?
        .into_string()
        .ok()
        .context("")?;
    let input = fs::read_to_string(format!("{home_dir}{PERSIST_FILE}"))?;

    let split: Vec<&str> = input.split(" ").collect();
    Ok((split[0].to_string(), split[1].to_string()))
}

pub fn apply_last_wallpaper() -> anyhow::Result<()> {
    let (backend, path) = read_last_wallpaper()?;
    let backend = match backend.as_str() {
        "hyprpaper" => BackendT::Hyprpaper(HyprpaperBackend {}),
        "mpvpaper" => BackendT::Mpvpaper(MpvpaperBackend {}),
        _ => BackendT::None,
    };
    let monitors = utils::get_monitors().unwrap_or_default();
    backend.apply(&monitors, &path)
}
