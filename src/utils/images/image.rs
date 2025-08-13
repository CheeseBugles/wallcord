use super::status::ImageStatus;
use eframe::egui::{ColorImage, TextureHandle};

#[derive(Default, PartialEq)]
pub struct WCImage {
    pub src: String,
    pub name: String,
    pub status: ImageStatus,
    pub raw_image: Option<ColorImage>,
    pub texture: Option<TextureHandle>,
}

impl WCImage {
    pub fn new(name: String, src: String) -> Self {
        Self {
            name,
            src,
            ..Default::default()
        }
    }
}
