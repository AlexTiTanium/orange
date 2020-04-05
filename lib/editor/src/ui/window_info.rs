use ecs::resources::Window as WindowResource;
use ecs::State;
use ecs::*;
use imgui::{im_str, Condition, Ui, Window};

pub fn build(ui: &Ui, state: &State) {
  Window::new(im_str!("Window"))
    .size([230.0, 105.0], Condition::FirstUseEver)
    .build(&ui, || {
      build_ui(&ui, state);
    });
}

fn build_ui(ui: &Ui, state: &State) {
  let window = state.world.borrow::<Unique<&WindowResource>>();
  ui.text(format!("Logical size: {:?}x{:?}", window.logical.width, window.logical.height));
  ui.text(format!("Physical size {:?}x{:?}", window.physical.width, window.physical.height));
  ui.text(format!("Scale factor(HDPI) {:?}", window.scale));
}
