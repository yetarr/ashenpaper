use anyhow::Result;
use std::path::PathBuf;
use eframe;
use eframe::egui;

mod renderer;
mod hypr;

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
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    let wallpapers = list_wallpapers("/home/yetar/Pictures/Wallpapers")?;
    eframe::run_native(
        "hyprpaper changer",
        options,
        Box::new(|cc| { 
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(renderer::MyApp::new(wallpapers))) 
        }),
    )?;
    Ok(())
}
