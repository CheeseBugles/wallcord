use crate::panels::{CentralPanel, SidePanel, TopPanel, WCPanelKind};
use crate::state::WCState;
use crate::utils::images::WCImages;
pub use crate::utils::images::image::WCImage;

pub struct WallCord {
    pub state: WCState,
    pub panels: [WCPanelKind; 3],
}

impl WallCord {
    pub fn new(images: WCImages) -> Self {
        let mut wc = Self {
            state: WCState::default(),
            panels: [
                WCPanelKind::TopPanel(TopPanel::default()),
                WCPanelKind::SidePanel(SidePanel::default()),
                WCPanelKind::CentralPanel(CentralPanel::default()),
            ],
        };
        wc.state.tasks.load_images(images);
        wc
    }
}

impl eframe::App for WallCord {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        for panel in &mut self.panels {
            panel.update(ctx, &mut self.state);
        }
        ctx.request_repaint();
    }
}
