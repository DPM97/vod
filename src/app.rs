use std::{fs, io::Error, path::PathBuf};

use chrono::Utc;
use eframe::epaint::ColorImage;
use egui::{CentralPanel, Color32, FontId, RichText, SidePanel, TextureFilter};
use image::{load_from_memory, DynamicImage, ImageError};

use crate::get_last_capture;

#[derive(Debug)]
enum StreamState {
    GOOD,
    BAD,
}

pub struct UI {
    stream_state: StreamState,
    capture_status_message: Result<String, String>,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            stream_state: StreamState::BAD,
            capture_status_message: Ok(String::default()),
        }
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
        SidePanel::left("Options").show(ctx, |sidebar| {
            sidebar.spacing_mut().item_spacing.y = 10.0;
            sidebar.separator();

            // don't allow to take photos if stream is in a bad state
            if sidebar.button("Take Photo").clicked()
                && matches!(self.stream_state, StreamState::GOOD)
            {
                self.capture_status_message = match capture() {
                    Ok(path) => Ok(String::from(format!("Saved capture: {}", path))),
                    Err(e) => Err(String::from(format!("Error: Failed to take photo: {}", e))),
                };
            }

            match &self.capture_status_message {
                Ok(m) => {
                    sidebar.label(m);
                }
                Err(e) => {
                    sidebar.label(RichText::new(e).color(Color32::RED));
                }
            }
        });

        CentralPanel::default().show(ctx, |image_area| match get_next_frame() {
            Some(img) => {
                let tex = image_area
                    .ctx()
                    .load_texture("frame", img, TextureFilter::Linear);
                image_area.image(&tex, image_area.available_size());
                self.stream_state = StreamState::GOOD;
            }
            None => {
                image_area.centered_and_justified(|image_area| {
                    image_area.label(
                        RichText::new("Error: Failed to parse frame data.")
                            .font(FontId::proportional(40.0))
                            .color(Color32::RED),
                    );
                });
                self.stream_state = StreamState::BAD;
            }
        });

        ctx.request_repaint();
    }
}

fn get_next_frame() -> Option<ColorImage> {
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

fn capture() -> Result<String, String> {
    let buf: Option<Vec<u8>> = get_last_capture();
    if buf.is_none() {
        return Err(String::from(
            "failed to capture (failed to get last capture)!",
        ));
    }
    let buf: Vec<u8> = buf.unwrap();

    let dls: Option<PathBuf> = dirs::download_dir();
    if dls.is_none() {
        return Err(String::from(
            "failed to capture (failed to get downloads directory)!",
        ));
    }
    let mut dls: PathBuf = dls.unwrap();
    dls.push(&format!("vod-capture-{}.jpeg", Utc::now().to_rfc3339()));

    let write: Result<(), Error> = fs::write(dls.as_path(), buf);
    if write.is_err() {
        return Err(write.unwrap_err().to_string());
    }

    return Ok(dls.display().to_string());
}
