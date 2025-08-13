use crate::{
    egui,
    panels::{WCPanel, side::SidePanelLocation},
    state::WCState,
};

pub struct TopPanel {
    ui_id: &'static str,
    title: &'static str,
}
impl Default for TopPanel {
    fn default() -> Self {
        Self {
            ui_id: "top_panel",
            title: "🎨 WallCord",
        }
    }
}

impl TopPanel {
    fn show(&self, ctx: &egui::Context, state: &mut WCState) {
        egui::TopBottomPanel::top(self.ui_id)
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    self.draw_center(ui);
                    self.draw_right(ui, state);
                });
            })
            .response
            .on_hover_cursor(egui::CursorIcon::Default);
    }

    fn draw_center(&self, ui: &mut egui::Ui) {
        ui.with_layout(
            egui::Layout::centered_and_justified(egui::Direction::BottomUp),
            |ui| {
                ui.heading(self.title);
            },
        );
    }

    fn draw_right(&self, ui: &mut egui::Ui, state: &mut WCState) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            self.show_menu(ui, state);

            let item = self.get_selected_image(state);
            let item_ref = item.as_ref().lock().unwrap();
            ui.monospace(format!(
                "{:>4}/{:<4}| {:>20}",
                self.get_selected_index(state) + 1,
                self.get_total_images(state),
                &item_ref.name[..item_ref.name.len().clamp(0, 20)]
            ));
        });
    }

    fn show_menu(&self, ui: &mut egui::Ui, state: &mut WCState) {
        let menu_config =
            egui::containers::menu::MenuConfig::default().close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);

        egui::containers::menu::MenuButton::new("Menu")
            .config(menu_config)
            .ui(ui, |ui| {
                ui.menu_button("Side panel", |ui| {
                    ui.checkbox(&mut state.menu.is_side_panel_pinning, "Pinning");
                    ui.checkbox(&mut state.menu.is_side_panel_disabled, "Disable");

                    ui.label("Location:");
                    ui.horizontal(|ui| {
                        let mut radios = [false; 4];
                        match self.menu(state).side_panel_location {
                            SidePanelLocation::Left => radios[0] = true,
                            SidePanelLocation::Right => radios[1] = true,
                        }
                        ui.radio(radios[0], "Left").clicked().then(|| {
                            self.menu(state).side_panel_location = SidePanelLocation::Left
                        });
                        ui.radio(radios[1], "Right").clicked().then(|| {
                            self.menu(state).side_panel_location = SidePanelLocation::Right
                        });
                    });
                });
            });
    }
}
impl WCPanel for TopPanel {
    fn update(&mut self, ctx: &egui::Context, state: &mut WCState) {
        self.show(ctx, state);
    }
}
