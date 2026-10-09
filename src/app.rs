use std::path::{Path, PathBuf};
use std::sync::mpsc::Receiver;

use anyhow::Result;
use eframe::egui::{self, Color32, ColorImage, TextureOptions, Vec2};

use crate::backend::hypr::HyprpaperBackend;
use crate::backend::mpv::MpvpaperBackend;
use crate::backend::{BackendT, WallpaperKind};
use crate::ui::video::{VideoPreview, uv_cover};
use crate::{backend, ui, utils};

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
    preview: Option<(PathBuf, VideoPreview)>,
    thumbs_rx: Receiver<(usize, ColorImage)>,
    settings: AppSettings,
}

impl Ashpaper {
    pub fn new(
        wallpapers: Vec<PathBuf>,
        settings: AppSettings,
        ctx: &egui::Context,
    ) -> Result<Self> {
        let placeholder = ctx.load_texture(
            "placeholder",
            ColorImage::filled([1, 1], Color32::from_gray(40)),
            TextureOptions::LINEAR,
        );
        let images = ui::image::compute_images(&wallpapers, &placeholder);

        let w = (settings.img_dimensions.x * 2.0) as u32;
        let h = (w as f32 / settings.target_aspect) as u32;
        let thumbs_rx = ui::thumbs::spawn_all(wallpapers, (w, h), ctx.clone());

        Ok(Ashpaper {
            wallpapers: images,
            preview: None,
            thumbs_rx,
            settings,
        })
    }

    fn poll_thumbnails(&mut self, ctx: &egui::Context) {
        while let Ok((i, img)) = self.thumbs_rx.try_recv() {
            let tex = ctx.load_texture(format!("thumb:{i}"), img, TextureOptions::LINEAR);
            self.wallpapers[i].set_thumb(tex);
        }
    }

    fn render_all_wallpapers(&mut self, ui: &mut egui::Ui) {
        let (columns, padding) = self.compute_wallpaper_grid_spacing(ui);
        let mut hovered_video = false;

        ui.add_space(padding / 2.0);
        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for chunk in self.wallpapers.chunks(columns) {
                    ui.horizontal(|ui| {
                        ui.add_space(padding.max(0.0));
                        for img in chunk {
                            let backend = match backend::kind_of(Path::new(&img.path_str())) {
                                WallpaperKind::Static => BackendT::Hyprpaper(HyprpaperBackend {}),
                                WallpaperKind::Video => BackendT::Mpvpaper(MpvpaperBackend {}),
                            };
                            Self::render_wallpaper(
                                img,
                                ui,
                                &self.settings,
                                &backend,
                                &mut self.preview,
                                &mut hovered_video,
                            );
                        }
                    });
                }
            });

        if !hovered_video {
            self.preview = None;
        }
    }

    fn compute_wallpaper_grid_spacing(&self, ui: &egui::Ui) -> (usize, f32) {
        let item_width = self.settings.img_dimensions.x;
        let spacing = ui.spacing().item_spacing.x;
        let available = ui.available_width();

        let columns = ((available + spacing) / (item_width + spacing))
            .floor()
            .max(1.0) as usize;
        let used_width = columns as f32 * item_width + (columns.saturating_sub(1)) as f32 * spacing;
        let padding = (available - used_width) / 2.0;

        (columns, padding)
    }

    fn render_wallpaper(
        img: &ui::image::Image,
        ui: &mut egui::Ui,
        settings: &AppSettings,
        backend: &BackendT,
        preview: &mut Option<(PathBuf, VideoPreview)>,
        hovered_video: &mut bool,
    ) {
        ui.allocate_ui_with_layout(
            Vec2::new(
                settings.img_dimensions.x,
                settings.img_dimensions.y + settings.btn_dimensions.y + 10.0,
            ),
            egui::Layout::top_down(egui::Align::Center),
            |ui| {
                let tile = ui.add(img.render_as_wallpaper(settings));

                if matches!(backend, BackendT::Mpvpaper(_)) && tile.hovered() {
                    *hovered_video = true;
                    let path = PathBuf::from(img.path_str());

                    if preview.as_ref().is_none_or(|(p, _)| *p != path) {
                        *preview = VideoPreview::start(&path, ui.ctx())
                            .ok()
                            .map(|pv| (path, pv));
                    }
                    if let Some((_, pv)) = preview
                        && let Some(tex) = pv.texture(ui.ctx())
                    {
                        ui.painter().image(
                            tex.id(),
                            tile.rect,
                            uv_cover(settings.target_aspect),
                            Color32::WHITE,
                        );
                    }
                }

                let btn = ui.add(ui::button::render_apply(settings, img.name().to_string()));
                if btn.clicked() {
                    let monitors = utils::get_monitors().unwrap_or_default();
                    ui::button::on_click(&img.path_str(), &monitors, backend);
                }
            },
        );
    }
}

impl eframe::App for Ashpaper {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.poll_thumbnails(ui.ctx());
        self.render_all_wallpapers(ui);
    }
}
