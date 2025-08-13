use std::sync::{Arc, Mutex};
pub mod image;
pub use image::WCImage;
pub mod status;
pub mod tasks;

pub type WCImages = Vec<Arc<Mutex<image::WCImage>>>;
