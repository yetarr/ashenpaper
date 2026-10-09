use eframe::egui::{ColorImage, Context, Rect, TextureHandle, TextureOptions, pos2};
use image::RgbaImage;
use std::{
    io::{self, Read},
    path::Path,
    process::{Child, Command, Stdio},
    sync::mpsc::{Receiver, sync_channel},
    thread,
};

pub const PW: usize = 320;
pub const PH: usize = 180;

fn vf(w: u32, h: u32) -> String {
    format!("scale={w}:{h}:force_original_aspect_ratio=increase,crop={w}:{h}")
}

pub fn uv_cover(tile_aspect: f32) -> Rect {
    let src = PW as f32 / PH as f32;
    if tile_aspect > src {
        let m = (1.0 - src / tile_aspect) / 2.0;
        Rect::from_min_max(pos2(0.0, m), pos2(1.0, 1.0 - m))
    } else {
        let m = (1.0 - tile_aspect / src) / 2.0;
        Rect::from_min_max(pos2(m, 0.0), pos2(1.0 - m, 1.0))
    }
}

/// One-shot: first frame (1s in, falls back to 0 for short clips) as a ColorImage.
pub fn thumbnail(path: &Path, w: u32, h: u32) -> io::Result<RgbaImage> {
    for ss in ["1", "0"] {
        let out = Command::new("ffmpeg")
            .args(["-v", "error", "-ss", ss, "-i"])
            .arg(path)
            .args(["-frames:v", "1", "-vf", &vf(w, h), "-f", "rawvideo", "-pix_fmt", "rgba", "-"])
            .stderr(Stdio::null())
            .output()?;
        if let Some(img) = RgbaImage::from_raw(w, h, out.stdout) {
            return Ok(img);
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "ffmpeg produced no frame"))
}

/// Looping live preview. Dropping it kills ffmpeg.
pub struct VideoPreview {
    child: Child,
    rx: Receiver<ColorImage>,
    texture: Option<TextureHandle>,
}

impl VideoPreview {
    pub fn start(path: &Path, ctx: &Context) -> io::Result<Self> {
        let mut child = Command::new("ffmpeg")
            .args(["-v", "error", "-re", "-stream_loop", "-1", "-i"])
            .arg(path)
            .args([
                "-an",
                "-vf",
                &format!("fps=15,{}", vf(PW as u32, PH as u32)),
                "-f",
                "rawvideo",
                "-pix_fmt",
                "rgba",
                "-",
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let mut out = child.stdout.take().unwrap();
        let (tx, rx) = sync_channel(2);
        let ctx = ctx.clone();
        thread::spawn(move || {
            let mut buf = vec![0u8; PW * PH * 4];
            while out.read_exact(&mut buf).is_ok() {
                let img = ColorImage::from_rgba_unmultiplied([PW, PH], &buf);
                if tx.send(img).is_err() {
                    break;
                }
                ctx.request_repaint();
            }
        });

        Ok(Self {
            child,
            rx,
            texture: None,
        })
    }

    pub fn texture(&mut self, ctx: &Context) -> Option<&TextureHandle> {
        while let Ok(img) = self.rx.try_recv() {
            match &mut self.texture {
                Some(t) => t.set(img, TextureOptions::LINEAR),
                None => {
                    self.texture =
                        Some(ctx.load_texture("video_preview", img, TextureOptions::LINEAR))
                }
            }
        }
        self.texture.as_ref()
    }
}

impl Drop for VideoPreview {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
