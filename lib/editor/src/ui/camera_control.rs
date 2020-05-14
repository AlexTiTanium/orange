use ecs::resources::Camera;
use ecs::State;
use ecs::*;
use imgui::{im_str, Condition, Ui, Window};

pub fn build(ui: &Ui, state: &State) {
  Window::new(im_str!("Camera"))
    .size([300.0, 100.0], Condition::FirstUseEver)
    .build(&ui, || {
      build_ui(&ui, state);
    });
}

fn build_ui(ui: &Ui, state: &State) {
  let mut camera = state.world.borrow::<UniqueViewMut<Camera>>();
  translate_control(ui, &mut camera);
}

fn translate_control(ui: &Ui, camera: &mut Camera) {
  let mut position: [f32; 2] = [camera.position.x, camera.position.y];
  ui.drag_float2(im_str!("Position"), &mut position).max(800.0).min(-800.0).build();
  camera.update_position(position[0], position[1]);
}
