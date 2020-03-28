use crate::components::*;
use crate::glm::*;
use crate::systems::*;
use crate::*;
use winit::event::WindowEvent;

#[derive(Default)]
pub struct State {
  pub world: World,
}

impl State {
  pub fn new() -> Self {
    let world = World::new();

    Self { world }
  }

  pub fn create_entities(&self) {
    let (mut entities, mut positions) = self.world.borrow::<(EntitiesMut, &mut Position)>();
    entities.add_entity(&mut positions, Position::default());
  }

  pub fn create_resources(&self) {
    self.world.add_unique(Display::new(100, 100));
    self.world.add_unique(Input::default());
    self.world.add_unique(Time::default());
    self.world.add_unique(FPS::default());
  }

  pub fn update_display(&self, width: u32, height: u32) {
    let mut display = self.world.borrow::<Unique<&mut Display>>();
    display.update(width, height);
  }

  pub fn handle_window_events(&self, event: &WindowEvent) {
    handle_keyboard_input(&self.world, event);
  }

  pub fn update_input(&self) {
    self.world.run_system::<MoveOnInput>();
  }

  pub fn update_time(&self) {
    self.world.run_system::<UpdateTime>();
  }

  pub fn update_fps(&self) {
    self.world.run_system::<UpdateFPS>();
  }
}
