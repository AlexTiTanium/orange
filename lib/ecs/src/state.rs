use crate::components::*;
use crate::resources::Window;
use crate::resources::*;
use crate::systems::*;
use crate::*;
use winit::event::WindowEvent;

pub fn create_state() -> State {
  let state = State::new();
  state.create_resources();
  state
}

#[derive(Default)]
pub struct State {
  pub world: World,
}

impl State {
  pub fn new() -> Self {
    let world = World::new();

    Self { world }
  }

  pub fn create_resources(&self) {
    self.world.add_unique(Window::default());
    self.world.add_unique(Input::default());
    self.world.add_unique(Time::default());
    self.world.add_unique(FPS::default());
  }

  pub fn create_game_object(&self) -> EntityId {
    let (mut entities, mut game_object, mut transform, mut active) =
      self.world.borrow::<(EntitiesMut, &mut GameObject, &mut Transform, &mut ActiveTag)>();

    entities.add_entity(
      (&mut game_object, &mut transform, &mut active),
      (GameObject::default(), Transform::default(), ActiveTag),
    )
  }

  pub fn handle_window_events(&self, event: &WindowEvent) {
    handle_keyboard_input(&self.world, event);
    handle_window_resize(&self.world, event);
  }

  pub fn update_time(&self) {
    self.world.run_system::<UpdateTime>();
  }

  pub fn update_fps(&self) {
    self.world.run_system::<UpdateFPS>();
  }
}
