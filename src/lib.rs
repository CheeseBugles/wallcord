mod panels;
mod state;
mod utils;
pub use eframe::{NativeOptions, egui};

pub mod wallcord;
pub use utils::image_provider::get_image_tasks;
pub use wallcord::WallCord;
