mod resources;
mod systems;

use game::{system, State, WorkloadBuilder};
use resources::FPS;
use systems::update_fps;

/// Module initialization
pub fn init(state: &State) {
  state.add_resource(FPS::default());
}
