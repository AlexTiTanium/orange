use imgui::{im_str, Condition, Ui, Window};
use state::*;

pub fn build(ui: &Ui, store: &Store) {
  Window::new(im_str!("Hello world"))
    .size([300.0, 100.0], Condition::FirstUseEver)
    .build(&ui, || {
      ui.text(im_str!("Hello world!"));
      ui.text(im_str!("こんにちは世界！"));
      ui.text(im_str!("This...is...imgui-rs!  {:?}", ui.io().delta_time));
      ui.separator();
      let mouse_pos = ui.io().mouse_pos;
      ui.text(format!("Mouse Position: ({:.1},{:.1})", mouse_pos[0], mouse_pos[1]));
    });
}
