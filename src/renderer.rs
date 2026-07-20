use std::path::PathBuf;

use anyhow::Result;
use eframe::egui::{self, Vec2};

use crate::ui;

pub struct AppSettings {
    pub img_dimensions: Vec2,
    pub btn_dimensions: Vec2,
    pub target_aspect: f32,
}

impl AppSettings {
    pub fn new(img_width: f32, btn_height: f32, target_aspect: f32) -> Self {
        let img_dimensions = Vec2::new(img_width, img_width / target_aspect);
        let btn_dimensions = Vec2::new(img_width, btn_height);
        AppSettings {
            img_dimensions,
            btn_dimensions,
            target_aspect,
        }
    }
}

pub struct Ashpaper {
    wallpapers: Vec<ui::image::Image>,
    settings: AppSettings,
}

impl Ashpaper {
    pub fn new(wallpapers: Vec<PathBuf>, settings: AppSettings) -> Result<Self> {
        let computed = ui::image::compute_images(&wallpapers)?;
        Ok(Ashpaper {
            wallpapers: computed,
            settings,
        })
    }

    fn render_all_wallpapers(&mut self, ui: &mut egui::Ui) {
        let (columns, padding) = self.compute_wallpaper_grid_spacing(ui);

        ui.add_space(padding / 2.0);
        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for chunk in self.wallpapers.chunks(columns) {
                    ui.horizontal(|ui| {
                        ui.add_space(padding.max(0.0));
                        for img in chunk {
                            self.render_wallpaper(img, ui);
                        }
                    });
                }
            });
    }

    fn compute_wallpaper_grid_spacing(&self, ui: &egui::Ui) -> (usize, f32) {
        let item_width = self.settings.img_dimensions.x;
        let spacing = ui.spacing().item_spacing.x;
        let available = ui.available_width();
    
        let columns = ((available + spacing) / (item_width + spacing)).floor().max(1.0) as usize;
        let used_width = columns as f32 * item_width + (columns.saturating_sub(1)) as f32 * spacing;
        let padding = (available - used_width) / 2.0;

        (columns, padding)
    }

    fn render_wallpaper(&self, img: &ui::image::Image, ui: &mut egui::Ui) {
        let settings = &self.settings;
        ui.allocate_ui_with_layout(
            Vec2::new(
                settings.img_dimensions.x,
                settings.img_dimensions.y + settings.btn_dimensions.y + 10.0,
            ),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                ui.add(img.render_as_wallpaper(settings));
                let btn = ui.add(ui::button::render_apply(settings, img.name().to_string()));
                if btn.clicked() { ui::button::on_click(&img.path_str()) };
            },
        );
    }
}

impl eframe::App for Ashpaper {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.render_all_wallpapers(ui);
    }
}
