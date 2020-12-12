pub mod stage;
mod state;

/// Game state
pub use state::State;

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub(crate) use nalgebra_glm as glm;
pub use shipyard::*;

// For init
use resources::Assets;
use resources::Camera;

pub fn init(state: &State) {
  //state.world.add_unique(Window::default());
  state.world.add_unique(Camera::default());
  // state.world.add_unique(Input::default());
  // state.world.add_unique(Time::default());
  // state.world.add_unique(FPS::default());
  state.world.add_unique(Assets::new());
}
