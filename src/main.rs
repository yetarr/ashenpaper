use anyhow::Result;
use serde::Deserialize;
use std::process::Command;
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

const PATH: &str = "~/Pictures/Wallpapers/juno_heart_enhanced.jpg";

fn main() -> Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "hyprpaper changer",
        options,
        Box::new(|_cc| Ok(Box::new(renderer::MyApp))),
    )?;
    
    set_wallpaper(PATH)?;
    Ok(())
}
