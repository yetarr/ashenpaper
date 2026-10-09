use std::{
    collections::hash_map::DefaultHasher,
    fs,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::{
        mpsc::{channel, Receiver},
        Arc,
    },
    thread,
};

use eframe::egui::{ColorImage, Context};
use image::{imageops::FilterType, RgbaImage};

use crate::backend::{kind_of, WallpaperKind};
use crate::ui::video;

pub fn spawn_all(
    paths: Vec<PathBuf>,
    size: (u32, u32),
    ctx: Context,
) -> Receiver<(usize, ColorImage)> {
    let (tx, rx) = channel();
    let paths = Arc::new(paths);
    let n = thread::available_parallelism().map_or(2, |n| n.get()).min(4);

    for k in 0..n {
        let (tx, paths, ctx) = (tx.clone(), paths.clone(), ctx.clone());
        thread::spawn(move || {
            for i in (k..paths.len()).step_by(n) {
                if let Some(img) = load_or_make(&paths[i], size) {
                    if tx.send((i, img)).is_err() {
                        return; // app closed
                    }
                    ctx.request_repaint();
                }
            }
        });
    }
    rx
}

fn load_or_make(path: &Path, (w, h): (u32, u32)) -> Option<ColorImage> {
    let cache = cache_file(path, w, h);

    if let Some(c) = &cache {
        if let Ok(img) = image::open(c) {
            return Some(to_color(&img.to_rgba8()));
        }
    }

    let rgba = match kind_of(path) {
        WallpaperKind::Static => image::open(path)
            .ok()?
            .resize_to_fill(w, h, FilterType::Triangle)
            .to_rgba8(),
        WallpaperKind::Video => video::thumbnail(path, w, h).ok()?,
    };

    if let Some(c) = &cache {
        if let Some(dir) = c.parent() {
            let _ = fs::create_dir_all(dir);
        }
        let _ = rgba.save(c);
    }
    Some(to_color(&rgba))
}

fn to_color(img: &RgbaImage) -> ColorImage {
    ColorImage::from_rgba_unmultiplied(
        [img.width() as usize, img.height() as usize],
        img.as_raw(),
    )
}

fn cache_file(path: &Path, w: u32, h: u32) -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))?;
    let meta = fs::metadata(path).ok()?;
    let mut hs = DefaultHasher::new();
    (path, meta.modified().ok()?, meta.len(), w, h).hash(&mut hs);
    Some(base.join("ashenpaper/thumbs").join(format!("{:016x}.png", hs.finish())))
}