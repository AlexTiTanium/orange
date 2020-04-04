use ecs::components::*;
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
  ui.same_line(ui.window_content_region_width() / 2.0 - 170.0 / 2.0);
  if ui.button(im_str!("Create Game Object"), [170.0, 25.0]) {
    state.create_game_object();
  }

  ui.separator();

  let (game_object, mut transform) = state.world.borrow::<(&GameObject, &mut Transform)>();
  let (entities, mut active) = state.world.borrow::<(Entities, &mut ActiveTag)>();

  (&mut transform, &game_object)
    .iter()
    .enumerate()
    .with_id()
    .for_each(|(id, (index, (transform, game_object)))| {
      let group = ui.push_id(index as i32);

      ui.text(format!("{:?}", id));
      ui.text(format!("Name: {:?}", game_object.name));

      transform_control(&ui, transform);
      active_control(&ui, &mut active, id, &entities);

      group.pop(&ui);
      ui.separator();
    });
}

fn transform_control(ui: &Ui, transform: &mut Transform) {
  let mut position: [f32; 3] = [transform.position.x, transform.position.y, transform.position.z];
  ui.drag_float3(im_str!("Position"), &mut position).max(800.0).min(0.0).build();
  transform.position.x = position[0];
  transform.position.y = position[1];
  transform.position.z = position[2];
}

fn active_control(ui: &Ui, active: &mut ViewMut<'_, ActiveTag>, id: EntityId, entities: &Entities) {
  let is_active = active.contains(id);
  let mut checked = is_active;

  ui.checkbox(im_str!("Active"), &mut checked);

  if !checked && is_active {
    active.delete(id);
  }

  if checked && !is_active {
    entities.add_component(active, ActiveTag, id);
  }
}
