use anyhow::Result;
use eframe;
use eframe::egui;
use std::path::PathBuf;

use crate::renderer::{AppSettings, Ashpaper};

mod hypr;
mod renderer;
mod ui;

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
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    let wallpapers = list_wallpapers("/home/yetar/Pictures/Wallpapers")?;
    eframe::run_native(
        "hyprpaper changer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Ashpaper::new(
                wallpapers,
                AppSettings::new(200.0, 20.0, hypr::get_target_aspect_ratio()?),
            )?))
        }),
    )?;
    Ok(())
}
