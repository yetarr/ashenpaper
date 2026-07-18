use std::path::PathBuf;

use eframe::egui;

use crate::hypr;

pub struct MyApp {
    wallpapers: Vec<PathBuf>,
}

impl MyApp {
    pub fn new(wallpapers: Vec<PathBuf>) -> Self {
        MyApp { wallpapers }
    }

    fn render_wallpapers(&mut self, ui: &mut egui::Ui) {
        for path in &self.wallpapers {
            let name = path.file_name().unwrap().display().to_string();
            let btn = ui.button(name);

            if btn.clicked() {
                if let Err(e) = hypr::set_wallpaper(&path.display().to_string()) {
                    eprintln!("error setting wallpaper: {:?}", e);
                }
            }
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.render_wallpapers(ui);
    }
}