use anyhow::Result;
use std::path::PathBuf;
use eframe;

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
    let options = eframe::NativeOptions::default();
    let wallpapers = list_wallpapers("/home/yetar/Pictures/Wallpapers")?;
    eframe::run_native(
        "hyprpaper changer",
        options,
        Box::new(|_cc| Ok(Box::new(renderer::MyApp::new(wallpapers)))),
    )?;
    Ok(())
}
