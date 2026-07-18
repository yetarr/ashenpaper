use std::process::Command;

use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize)]
pub struct Monitor {
    name: String,
}

pub fn get_monitors() -> Result<Vec<Monitor>> {
    let output = Command::new("hyprctl").args(["monitors", "-j"]).output()?;
    let monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout).unwrap();
    Ok(monitors)
}

pub fn set_wallpaper(path: &str) -> Result<()> {
    for monitor in get_monitors()? {
        Command::new("hyprctl")
            .args(["hyprpaper", "wallpaper", &format!("{},{}", monitor.name, path)])
            .status()?;
    }
    Ok(())
}