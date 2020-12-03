mod resources;
mod systems;

use game::{system, Stage, State};
use resources::FPS;
use systems::update_fps;

/// Module initialization
pub fn init(state: &State) {
  state.add_resource(FPS::default());
  //state.workload(Stage::Update).with_system(system!(update_fps));
}
