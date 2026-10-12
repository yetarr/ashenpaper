use std::env;
use std::path::PathBuf;

use eframe::{self, egui};

use anyhow::Result;
use ashenpaper::app::{AppSettings, Ashpaper};
use ashenpaper::{persistance, utils};

fn list_wallpapers(path: &str) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let path = entry?.path();
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            if matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "mp4") {
                paths.push(path);
            }
        }
    }
    Ok(paths)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && matches!(args[1].as_str(), "--restore" | "-r") {
        persistance::apply_last_wallpaper()?;
        return Ok(());
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    let wallpapers = list_wallpapers("/home/yetar/pictures/wallpapers")?;
    let settings = AppSettings::new(200.0, 20.0, utils::get_target_aspect_ratio()?);
    eframe::run_native(
        "hyprpaper changer",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Ashpaper::new(wallpapers, settings, &cc.egui_ctx)?))
        }),
    )?;
    Ok(())
}
