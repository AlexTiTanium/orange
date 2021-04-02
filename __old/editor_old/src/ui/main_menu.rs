use crate::editor::UiState;
use game::State;
use imgui::{im_str, MenuItem, Ui};

pub fn build(ui: &Ui, _state: &State, ui_state: &mut UiState) {
  if let Some(menu_bar) = ui.begin_main_menu_bar() {
    build_tools_menu(ui, ui_state);

    ui.same_line(ui.window_content_region_width() - 140.0);
    ui.text(format!("{:.3}ms / {:?} fps", ui.io().delta_time, ui.io().framerate.round()));

    menu_bar.end(ui);
  }
}

fn build_tools_menu(ui: &Ui, ui_state: &mut UiState) {
  // Tools
  if let Some(menu) = ui.begin_menu(im_str!("Tools"), true) {
    MenuItem::new(im_str!("Entities Control")).build_with_ref(ui, &mut ui_state.show_entities_control);
    MenuItem::new(im_str!("Tiles List")).build_with_ref(ui, &mut ui_state.show_tiles_list);
    MenuItem::new(im_str!("Camera")).build_with_ref(ui, &mut ui_state.show_camera_control);
    menu.end(ui);
  }

  // Info
  if let Some(menu) = ui.begin_menu(im_str!("Info"), true) {
    MenuItem::new(im_str!("Window")).build_with_ref(ui, &mut ui_state.show_window_info);
    menu.end(ui);
  }
}
