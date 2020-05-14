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
    self.world.add_unique(Camera::default());
    self.world.add_unique(Input::default());
    self.world.add_unique(Time::default());
    self.world.add_unique(FPS::default());
    self.world.add_unique(Assets::new());
  }

  pub fn create_game_object(&self) -> EntityId {
    self.world.run(entities::create_game_object)
  }

  pub fn handle_window_events(&self, event: &WindowEvent) {
    handle_keyboard_input(&self.world, event);
    handle_window_resize(&self.world, event);
    handle_camera_resize(&self.world, event);
  }

  pub fn attach_window(&self, window: &winit::window::Window) {
    let scale_factor = window.scale_factor(); // TODO: round?
    let logical_size = window.inner_size().to_logical::<f32>(scale_factor);
    let physical_size = logical_size.to_physical::<f32>(scale_factor);

    update_window_sizes(&self.world, logical_size, physical_size, scale_factor);
    handle_camera_update(&self.world);
  }

  pub fn update_time(&self) {
    self.world.run(time::update);
  }

  pub fn update_fps(&self) {
    self.world.run(time::update_fps);
  }
}
