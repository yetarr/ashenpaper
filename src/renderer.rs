use std::path::PathBuf;

use eframe::egui::{self, Vec2};

use crate::hypr;

pub struct MyApp {
    wallpapers: Vec<PathBuf>,
}

impl MyApp {
    pub fn new(wallpapers: Vec<PathBuf>) -> Self {
        MyApp { wallpapers }
    }

    fn render_all_wallpapers(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for path in &self.wallpapers {
                    self.render_wallpaper(path, ui);
                }
            });
        });
    }

    fn render_wallpaper(&self, path: &PathBuf, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            let path_str = path.display().to_string();
            ui.add(
                egui::Image::new(format!("file://{}", path_str))
                    .fit_to_exact_size(Vec2::new(200.0, 150.0))
            );
            let btn = ui.add(
                egui::Button::new(path.file_name().unwrap().display().to_string())
                    .min_size(Vec2::new(200.0, 20.0))
            );
            if btn.clicked() {
                if let Err(e) = hypr::set_wallpaper(&path_str) {
                    eprintln!("error setting wallpaper: {:?}", e);
                };
            }
        });
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.render_all_wallpapers(ui);
    }
}