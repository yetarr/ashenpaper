use anyhow::Result;
use serde::Deserialize;
use std::{path::PathBuf, process::Command};
use eframe;

mod renderer;

#[derive(Deserialize)]
struct Monitor {
    name: String,
}

fn get_monitors() -> Result<Vec<Monitor>> {
    let output = Command::new("hyprctl").args(["monitors", "-j"]).output()?;
    let monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout).unwrap();
    Ok(monitors)
}

fn set_wallpaper(path: &str) -> Result<()> {
    for monitor in get_monitors()? {
        Command::new("hyprctl")
            .args(["hyprpaper", "wallpaper", &format!("{},{}", monitor.name, path)])
            .status()?;
    }
    Ok(())
}

fn list_wallpapers(path: &str) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let path = entry?.path(); 
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            if matches!(ext.as_str(), "png" | "jpg" | "jpeg") {
                paths.push(path);
            }
        }
    }
    Ok(paths)
}

fn main() -> Result<()> {
    let options = eframe::NativeOptions::default();
    let wallpapers = list_wallpapers("/home/yetar/Pictures/Wallpapers")?;
    eframe::run_native(
        "hyprpaper changer",
        options,
        Box::new(|_cc| Ok(Box::new(renderer::MyApp::new(wallpapers)))),
    )?;
    Ok(())
}
