use std::process::Command;
use std::fs;

use dirs;
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
    update_config_wallpaper(path)?;
    Ok(())
}

pub fn update_config_wallpaper(new_path: &str) -> Result<()> {
    let config_path = dirs::home_dir()
        .unwrap()
        .join(".config/hypr/hyprpaper.conf");
    let contents = fs::read_to_string(&config_path)?;

    let mut in_wallpaper_block = false;
    let mut output = String::new();

    for line in contents.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("wallpaper") && trimmed.contains('{') {
            in_wallpaper_block = true;
            output.push_str(line);
            output.push('\n');
            continue;
        }

        if trimmed == "}" {
            in_wallpaper_block = false;
            output.push_str(line);
            output.push('\n');
            continue;
        }

        if in_wallpaper_block && trimmed.starts_with("path") {
            output.push_str(&format!("path = {}\n", new_path));
            continue;
        }

        output.push_str(line);
        output.push('\n');
    }

    fs::write(&config_path, output)?;
    Ok(())
}

pub fn get_target_aspect_ratio() -> Result<f32> {
    let monitors = get_monitors()?;
    let primary = &monitors[0];
    Ok(primary.width as f32 / primary.height as f32)
}
