use std::fs;
use std::process::Command;

use anyhow::Result;
use dirs;

use crate::backend::mpv::MpvpaperBackend;
use crate::backend::{Backend, Monitor};

pub struct HyprpaperBackend {}

impl HyprpaperBackend {
    fn update_config_wallpaper(new_path: &str) -> Result<()> {
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
}

impl Backend for HyprpaperBackend {
    fn apply(&self, monitors: &[Monitor], path: &str) -> anyhow::Result<()> {
        MpvpaperBackend {}.stop()?;
        for monitor in monitors {
            Command::new("hyprctl")
                .args([
                    "hyprpaper",
                    "wallpaper",
                    &format!("{},{}", monitor.name, path),
                ])
                .status()?;
        }
        Self::update_config_wallpaper(path)?;
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        Command::new("pkill").arg("hyprpaper").status()?;
        Ok(())
    }
}
