use crate::{
    egui,
    panels::{CentralPanel, SidePanel, TopPanel, WCPanel},
    state::WCState,
};

pub enum WCPanelKind {
    SidePanel(SidePanel),
    TopPanel(TopPanel),
    CentralPanel(CentralPanel),
}

impl WCPanelKind {
    pub fn update(&mut self, ctx: &egui::Context, state: &mut WCState) {
        use WCPanelKind::*;
        match self {
            CentralPanel(p) => p.update(ctx, state),
            SidePanel(p) => p.update(ctx, state),
            TopPanel(p) => p.update(ctx, state),
        }
    }
}
