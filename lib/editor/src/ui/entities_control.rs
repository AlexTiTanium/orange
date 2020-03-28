use crate::glm::*;
use ecs::components::Position;
use ecs::components::FPS;
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
    let (mut entities, mut positions, fps) = state.world.borrow::<(EntitiesMut, &mut Position, Unique<&FPS>)>();

    (&mut positions).iter().enumerate().with_id().for_each(|(id, (index, Position(pos)))| {
        let group = ui.push_id(index as i32);
        ui.text(format!("EntityId: {:?}", &id));

        ui.drag_float(im_str!("X"), &mut pos.x).max(800.0).min(0.0).build();
        ui.drag_float(im_str!("Y"), &mut pos.y).max(860.0).min(0.0).build();

        group.pop(&ui);
        ui.separator();
    });

    if ui.button(im_str!("Add"), [110.0, 25.0]) {
        entities.add_entity(&mut positions, Position(vec3(0.0, 0.0, 0.0)));
    }
}
