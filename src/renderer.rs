use std::path::PathBuf;

use eframe::egui;

pub struct MyApp {
    wallpapers: Vec<PathBuf>,
}

impl MyApp {
    pub fn new(wallpapers: Vec<PathBuf>) -> Self {
        MyApp { wallpapers }
    }

    fn render_wallpapers_names(&mut self, ui: &mut egui::Ui) {
        for path in &self.wallpapers {
            ui.label(path.file_name().unwrap().display().to_string());
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.render_wallpapers_names(ui);
    }
}