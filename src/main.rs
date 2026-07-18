use std::process::Command;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Monitor {
    name: String,
}

fn get_monitors() -> Result<Vec<Monitor>> {
    let output = Command::new("hyprctl")
        .args(["monitors", "-j"])
        .output()?;
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

const PATH: &str = "~/Pictures/Wallpapers/juno_heart_enhanced.jpg";

fn main() -> Result<()> {
    set_wallpaper(PATH)?;
    Ok(())
}
