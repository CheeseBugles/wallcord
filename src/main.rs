use wallcord::{NativeOptions, WallCord, egui::ViewportBuilder, get_image_tasks};

fn main() {
    let images = get_image_tasks();
    let app = WallCord::new(images);

    let native_options = NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1500., 900.]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "WallCord",
        native_options,
        Box::new(|_cc| Ok(Box::new(app))),
    )
    .unwrap();
}
