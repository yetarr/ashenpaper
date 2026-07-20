use std::process::Command;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Monitor {
    pub name: String,
    pub width: u32,
    pub height: u32,
}

pub fn get_monitors() -> Result<Vec<Monitor>> {
    let output = Command::new("hyprctl").args(["monitors", "-j"]).output()?;
    let monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout)?;
    Ok(monitors)
}

pub fn set_wallpaper(path: &str) -> Result<()> {
    for monitor in get_monitors()? {
        Command::new("hyprctl")
            .args([
                "hyprpaper",
                "wallpaper",
                &format!("{},{}", monitor.name, path),
            ])
            .status()?;
    }
    Ok(())
}

pub fn get_target_aspect_ratio() -> Result<f32> {
    let monitors = get_monitors()?;
    let primary = &monitors[0];
    Ok(primary.width as f32 / primary.height as f32)
}
