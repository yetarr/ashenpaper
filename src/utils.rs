use std::process::Command;

use crate::backend::Monitor;

pub fn get_monitors() -> anyhow::Result<Vec<Monitor>> {
    let output = Command::new("hyprctl").args(["monitors", "-j"]).output()?;
    let monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout)?;
    Ok(monitors)
}

pub fn get_target_aspect_ratio() -> anyhow::Result<f32> {
    let monitors = get_monitors()?;
    let primary = &monitors[0];
    Ok(primary.width as f32 / primary.height as f32)
}
