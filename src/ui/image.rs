use std::{ffi::OsStr, path::PathBuf};

use anyhow::Result;
use eframe::egui::{self, Vec2};

use crate::renderer::AppSettings;

pub struct Image {
    path: PathBuf,
    dimensions: Vec2,
    name: String,
}

impl Image {
    pub fn new(path: PathBuf, dimensions: Vec2) -> Self {
        let name = path.file_name().unwrap_or(OsStr::new("unnamed")).display().to_string();
        Image { path, dimensions, name }
    }

    pub fn path_str(&self) -> String {
        self.path.display().to_string()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn render_as_wallpaper(&self, settings: &AppSettings) -> egui::Image<'_> {
        egui::Image::new(format!("file://{}", self.path.display().to_string()))
            .uv(self.cover_uv(settings.target_aspect))
            .fit_to_exact_size(settings.img_dimensions)
            .maintain_aspect_ratio(false)
    }

    fn cover_uv(&self, target_aspect: f32) -> egui::Rect {
        let image_aspect = self.dimensions.x / self.dimensions.y;
        if image_aspect > target_aspect {
            let visible_fraction = target_aspect / image_aspect;
            let margin = (1.0 - visible_fraction) / 2.0;
            egui::Rect::from_min_max(egui::pos2(margin, 0.0), egui::pos2(1.0 - margin, 1.0))
        } else {
            let visible_fraction = image_aspect / target_aspect;
            let margin = (1.0 - visible_fraction) / 2.0;
            egui::Rect::from_min_max(egui::pos2(0.0, margin), egui::pos2(1.0, 1.0 - margin))
        }
    }
}

pub fn compute_images(paths: &Vec<PathBuf>) -> Result<Vec<Image>> {
    let mut images = Vec::new();
    for path in paths {
        let (img_w, img_h) = image::image_dimensions(path)?;
        let img = Image::new(path.clone(), Vec2::new(img_w as f32, img_h as f32));
        images.push(img);
    }
    Ok(images)
}
