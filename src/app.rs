use eframe::epaint::ColorImage;
use egui::{CentralPanel, TextureFilter};
use image::{load_from_memory, DynamicImage, ImageError};

use crate::get_last_capture;

/// shhhhh
pub struct UI {}

impl Default for UI {
    fn default() -> Self {
        Self {}
    }
}

impl UI {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for UI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |image_area| match get_next_frame() {
            Some(img) => {
                let tex = image_area
                    .ctx()
                    .load_texture("frame", img, TextureFilter::Linear);
                image_area.image(&tex, image_area.available_size());
            }
            None => {}
        });
        ctx.request_repaint();
    }
}

pub fn get_next_frame() -> Option<ColorImage> {
    match get_last_capture() {
        None => None,
        Some(jpg_img) => {
            let buf: Result<DynamicImage, ImageError> = load_from_memory(&jpg_img);
            if buf.is_err() {
                println!("failed to load img from memory!");
                return None;
            }
            let buf: DynamicImage = buf.unwrap();
            let img: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = buf.to_rgba8();
            let size: [usize; 2] = [img.width() as _, img.height() as _];
            return Some(ColorImage::from_rgba_unmultiplied(
                size,
                img.as_flat_samples().as_slice(),
            ));
        }
    }
}
