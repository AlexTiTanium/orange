use game::State;

pub mod editor;
mod ui;

pub use editor::Editor;

/// Module initialization
pub fn init(state: &State) {
  // let mut workload = WorkloadBuilder::default();
  // workload.with_system(system!(update_fps));
  // state.on_update(workload);
}
