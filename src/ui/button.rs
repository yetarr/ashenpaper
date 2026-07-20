use eframe::egui::{self, Button};

use crate::renderer::AppSettings;
use crate::hypr;

pub fn render_apply<'a>(settings: &AppSettings, text: String) -> Button<'a> {
    egui::Button::new(text).min_size(settings.btn_dimensions)
}

pub fn on_click(path: &str) {
    if let Err(e) = hypr::set_wallpaper(path) {
        eprintln!("error setting wallpaper: {:?}", e);
    };
}