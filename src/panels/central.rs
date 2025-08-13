use crate::{
    egui,
    panels::WCPanel,
    state::WCState,
};

#[derive(Default)]
pub struct CentralPanel;
impl CentralPanel {
    fn event_handler(&self, ui: &mut egui::Ui, state: &mut WCState) {
        ui.allocate_rect(ui.max_rect(), egui::Sense::HOVER)
            .hovered()
            .then(|| {
                ui.input(|i| {
                    let current_index = self.get_selected_index(state);
                    let total_items = self.get_total_images(state);
                    let scroll_delta = i.raw_scroll_delta.y;

                    if scroll_delta > 0.0 {
                        if current_index < total_items - 1 {
                            self.set_selected_item(state, current_index + 1);
                        } else {
                            self.set_selected_item(state, 0);
                        }
                    } else if scroll_delta < 0.0 {
                        if current_index > 0 {
                            self.set_selected_item(state, current_index - 1);
                        } else {
                            self.set_selected_item(state, total_items - 1);
                        }
                    }
                });
            });
    }
}
impl WCPanel for CentralPanel {
    fn update(&mut self, ctx: &egui::Context, state: &mut WCState) {
        {
            egui::CentralPanel::default()
                .frame(egui::Frame::new().fill(ctx.style().visuals.window_fill()))
                .show(ctx, |ui| {
                    let img = self.get_selected_image(state);
                    let mut img_ref = img.as_ref().lock().unwrap();
                    if img_ref.status.is_done() {
                        if let Some(raw_image) = img_ref.raw_image.take() {
                            img_ref.texture = Some(ctx.load_texture(
                                &img_ref.name,
                                raw_image,
                                egui::TextureOptions::default(),
                            ));
                        }
                        if let Some(ref texture) = img_ref.texture {
                            ui.image(egui::load::SizedTexture::new(
                                texture.id(),
                                ui.available_size(),
                            ));
                        }
                    } else {
                        self.spinner_with_label(ui, ui.available_size(), &img_ref.name);
                    }
                    self.event_handler(ui, state);
                });
        }
    }
}
