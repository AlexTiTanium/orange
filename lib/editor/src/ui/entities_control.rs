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

            ui.drag_float(im_str!("X"), &mut transform.position.x).max(800.0).min(0.0).build();
            ui.drag_float(im_str!("Y"), &mut transform.position.y).max(860.0).min(0.0).build();

            let is_active = active.contains(id);
            let mut checked = is_active;

            ui.checkbox(im_str!("Active"), &mut checked);

            if !checked && is_active {
                active.delete(id);
            }

            if checked && !is_active {
                entities.add_component(&mut active, ActiveTag, id);
            }

            group.pop(&ui);
            ui.separator();
        });
}
