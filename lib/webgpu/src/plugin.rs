use crate::api::WebGpuState;
use common::{Builder, Plugin};

pub struct WebGpuPlugin;

///
/// Web gpu render API plugin
///
impl Plugin for WebGpuPlugin {
  ///
  /// Build phase
  ///
  fn build(&self, app: &mut Builder) {
    app.add_startup_system(&WebGpuState::init);
  }
}
