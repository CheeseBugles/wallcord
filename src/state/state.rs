use crate::panels::menu::WCMenu;
use crate::utils::images::tasks::WCImageTasks;

#[derive(Default)]
pub struct WCState {
    pub tasks: WCImageTasks,
    pub menu: WCMenu,
    pub selected_image_index: usize,
}
