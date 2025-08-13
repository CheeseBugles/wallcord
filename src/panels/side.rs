use std::sync::Mutex;

use eframe::egui;

use crate::{panels::WCPanel, state::WCState, wallcord::WCImage};

#[derive(Debug, Default)]
pub enum SidePanelLocation {
    Left,
    #[default]
    Right,
    // Up,
    // Down,
}
#[derive(Default)]
struct Width {
    current: f32,
    old: f32,
}

pub struct SidePanel {
    hight: f32,
    width: Width,
    // scroll_offset: f32,
    padding: i8,
}

impl SidePanel {
    fn calculate_motion(&mut self, ctx: &egui::Context, state: &mut WCState) {
        let outer_width = ctx.input(|i| i.viewport().outer_rect.unwrap().size().x);
        let target_width = outer_width * 0.2;

        let hovering = match self.menu(state).side_panel_location {
            SidePanelLocation::Left => ctx
                .pointer_latest_pos()
                .map_or(false, |pos| pos.x <= target_width),
            SidePanelLocation::Right => ctx
                .pointer_latest_pos()
                .map_or(false, |pos| pos.x >= outer_width - target_width),
        };

        let speed_open = 10.0;
        let speed_close = 2.0;
        let dt = ctx.input(|i| i.stable_dt).min(0.033);
        let goal = if hovering { target_width } else { 0.0 };

        let speed = if goal > self.width.current {
            self.width.old = target_width;
            speed_open
        } else {
            speed_close
        };

        let step = (speed * dt as f32).clamp(0.0, 0.5);
        self.width.current = self.width.current + (goal - self.width.current) * step;
        if self.menu(state).is_side_panel_pinning {
            self.width.current = target_width;
            self.padding = 0;
        }
    }

    fn show(&self, ctx: &egui::Context, state: &mut WCState) {
        let frame = egui::Frame::new()
            .fill(ctx.style().visuals.window_fill())
            .inner_margin(self.padding);

        let side_panel = match self.menu(state).side_panel_location {
            SidePanelLocation::Left => egui::SidePanel::left("left_side"),
            SidePanelLocation::Right => egui::SidePanel::right("right_side"),
        };

        side_panel
            .frame(frame)
            .resizable(false)
            .exact_width(self.width.current)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    // .vertical_scroll_offset(self.scroll_offset)
                    .show(ui, |ui| {
                        for (index, image) in state.tasks.images.clone().iter().enumerate() {
                            self.show_items(ui, state, image, index);
                        }
                    });
            });
    }

    fn show_items(
        &self,
        ui: &mut egui::Ui,
        state: &mut WCState,
        image: &Mutex<WCImage>,
        index: usize,
    ) {
        let mut image = image.lock().unwrap();

        if let Some(raw_image) = image.raw_image.take() {
            image.texture = Some(ui.ctx().load_texture(
                &image.name,
                raw_image,
                egui::TextureOptions::default(),
            ));
        }

        if let Some(texture) = &image.texture {
            let image_size = egui::Vec2::new(ui.available_width(), self.hight);
            // Reserve the space and make it clickable
            let (rect, response) = ui.allocate_exact_size(image_size, egui::Sense::click());

            // Draw the image
            ui.painter().image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(egui::Pos2::new(0.0, 0.0), egui::Pos2::new(1.0, 1.0)), // full texture UV
                egui::Color32::WHITE,
            );

            // Draw blue border if selected
            if self.get_selected_index(state) == index {
                ui.painter().rect_stroke(
                    rect,
                    0.0,
                    egui::Stroke::new(2.5, egui::Color32::DARK_BLUE),
                    egui::StrokeKind::Inside,
                );
            }

            // Set cursor icon on hover
            if response.hovered() {
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
            }

            response.clicked().then(|| {
                self.set_selected_item(state, index);
            });
        } else {
            image
                .status
                .is_faild()
                .then(|| state.tasks.images.remove(index));
            image.status.is_loading().then(|| {
                self.spinner_with_label(
                    ui,
                    egui::Vec2::new(ui.available_width(), self.hight),
                    &image.name,
                )
                .clicked()
                .then(|| {
                    self.set_selected_item(state, index);
                })
            });
        }
    }
}
impl WCPanel for SidePanel {
    fn update(&mut self, ctx: &egui::Context, state: &mut WCState) {
        if self.menu(state).is_side_panel_disabled {
            return;
        }
        self.calculate_motion(ctx, state);
        self.show(ctx, state)
    }
}

impl Default for SidePanel {
    fn default() -> Self {
        Self {
            hight: 250.0,
            padding: 20,
            width: Default::default(),
            // scroll_offset: Default::default(),
        }
    }
}
