use eframe::egui::{self, Button};

use crate::app::AppSettings;
use crate::backend::{Backend, Monitor};

pub fn render_apply<'a>(settings: &AppSettings, text: String) -> Button<'a> {
    egui::Button::new(text).min_size(settings.btn_dimensions)
}

pub fn on_click<B: Backend>(path: &str, monitors: &[Monitor], backend: &B) {
    if let Err(e) = backend.apply(monitors, path) {
        eprintln!("error setting wallpaper: {:?}", e);
    }
}
