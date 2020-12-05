mod resources;
mod systems;

use game::{system, State, WorkloadBuilder};
use resources::FPS;
use systems::update_fps;

/// Module initialization
pub fn init(state: &State) {
  state.add_resource(FPS::default());
  //state.update.borrow_mut().with_system(system!(update_fps));

  let mut workload = WorkloadBuilder::default();

  workload.with_system(system!(update_fps));

  state.on_update(workload);
}
