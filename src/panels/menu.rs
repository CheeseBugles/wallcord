use super::side::SidePanelLocation;

#[derive(Debug)]
pub struct WCMenu {
    pub side_panel_location: SidePanelLocation,
    pub is_side_panel_disabled: bool,
    pub is_side_panel_pinning: bool,
}

impl Default for WCMenu {
    fn default() -> Self {
        Self {
            is_side_panel_pinning: true,
            is_side_panel_disabled: false,
            side_panel_location: Default::default(),
        }
    }
}
