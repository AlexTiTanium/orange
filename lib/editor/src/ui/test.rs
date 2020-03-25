use ecs::component::Position;
use ecs::State;
use ecs::*;
use imgui::{im_str, Condition, Ui, Window};

pub fn build(ui: &Ui, state: &State) {
  Window::new(im_str!("Entities"))
    .size([300.0, 100.0], Condition::FirstUseEver)
    .build(&ui, || {
      build_ui(&ui, state);
    });
}

fn build_ui(ui: &Ui, state: &State) {
  let (mut entities, mut positions) = state.world.borrow::<(EntitiesMut, &mut Position)>();

  let mut index = 0;
  (&mut positions).iter().with_id().for_each(|(id, pos)| {
    let group = ui.push_id(index);
    ui.text(format!("EntityId: {:?}", &id));
    ui.drag_float(im_str!("X"), &mut pos.x).max(800.0).min(0.0).build();
    ui.drag_float(im_str!("Y"), &mut pos.y).max(860.0).min(0.0).build();
    group.pop(&ui);
    index += 1;
    ui.separator();
  });

  if ui.button(im_str!("Add"), [110.0, 25.0]) {
    entities.add_entity(&mut positions, Position { x: 0.0, y: 0.0 });
  }
}
