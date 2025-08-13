use crate::{egui, panels::menu::WCMenu, state::WCState, wallcord::WCImage};

pub trait WCPanel {
    fn update(&mut self, ctx: &egui::Context, state: &mut WCState);

    fn get_selected_image(&self, state: &mut WCState) -> std::sync::Arc<std::sync::Mutex<WCImage>> {
        let current_index = self.get_selected_index(state);
        state.tasks.images[current_index].clone()
    }

    fn menu<'a>(&self, state: &'a mut WCState) -> &'a mut WCMenu {
        &mut state.menu
    }

    fn get_total_images(&self, state: &mut WCState) -> usize {
        state.tasks.images.len()
    }

    fn get_selected_index(&self, state: &mut WCState) -> usize {
        state.selected_image_index
    }
    fn set_selected_item(&self, state: &mut WCState, index: usize) {
        state.selected_image_index = index;
    }

    // UI
    fn spinner_with_label(
        &self,
        ui: &mut egui::Ui,
        size: egui::Vec2,
        label: &str,
    ) -> egui::Response {

        let size = size;
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
        let center = rect.center();

        
        let time = ui.input(|i| i.time);
        let progress = (time * 2.0) % 1.0;

        
        let radius = 24.0;
        let stroke = egui::epaint::PathStroke::new(3.0, egui::Color32::from_gray(150));
        let start_angle = (progress as f32) * 2.0 * std::f32::consts::TAU;
        let sweep_angle = std::f32::consts::FRAC_PI_2;
        let num_points = 20;

        let points: Vec<egui::Pos2> = (0..=num_points)
            .map(|i| {
                let t = start_angle + sweep_angle * (i as f32 / num_points as f32);
                egui::Pos2::new(center.x + radius * t.cos(), center.y + radius * t.sin())
            })
            .collect();

        let path = egui::epaint::PathShape {
            points,
            closed: false,
            stroke,
            fill: egui::Color32::TRANSPARENT,
        };

        ui.painter().add(egui::Shape::Path(path));

        // Draw label under the spinner
        ui.painter().text(
            egui::Pos2::new(center.x, rect.bottom() - 20.0),
            egui::Align2::CENTER_CENTER,
            format!("Loading: {label}"),
            egui::FontId::proportional(14.0),
            egui::Color32::GRAY,
        );

        // Hover cursor
        response.on_hover_cursor(egui::CursorIcon::Progress)
    }
}
