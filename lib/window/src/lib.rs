pub mod resources;
mod systems;
use game::{system, State, WorkloadBuilder};
use resources::Window;

/// Module initialization
pub fn init(state: &State) {
  state.add_resource(Window::default());

  let mut workload = WorkloadBuilder::default();

  workload.with_system(system!(systems::create_window));

  state.on_start(workload);
}
