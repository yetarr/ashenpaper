use std::{ffi::OsStr, path::PathBuf};

use eframe::egui::{self, TextureHandle};

use crate::app::AppSettings;

pub struct Image {
    path: PathBuf,
    name: String,
    thumb: TextureHandle,
}

impl Image {
    pub fn new(path: PathBuf, thumb: TextureHandle) -> Self {
        let name = path.file_name().unwrap_or(OsStr::new("unnamed")).display().to_string();
        Image { path, name, thumb }
    }

    pub fn set_thumb(&mut self, thumb: TextureHandle) {
        self.thumb = thumb;
    }

    pub fn path_str(&self) -> String {
        self.path.display().to_string()
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn render_as_wallpaper(&self, settings: &AppSettings) -> egui::Image<'_> {
        egui::Image::from_texture(&self.thumb)
            .fit_to_exact_size(settings.img_dimensions)
            .maintain_aspect_ratio(false)
    }
}

pub fn compute_images(paths: &[PathBuf], placeholder: &TextureHandle) -> Vec<Image> {
    paths.iter().map(|p| Image::new(p.clone(), placeholder.clone())).collect()
}
