use game::module::Module;
use game::State;

use crate::resources::FPS;

#[derive(Default)]
pub struct DiagnosticModule;

impl Module for DiagnosticModule {
  fn init(&self, state: &State) {
    state.add_resource(FPS::default());
  }
}
