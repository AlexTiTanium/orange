use crate::api::WebGpuState;
use common::{stage, Builder, Plugin};

pub struct WebGpuPlugin;

///
/// Web gpu render API plugin
///
impl Plugin for WebGpuPlugin {
  ///
  /// Build phase
  ///
  fn build(&self, app: &mut Builder) {
    app
      .add_startup_system(&WebGpuState::init)
      .add_to_stage(stage::EVENT, &WebGpuState::on_resize_event);
  }
}
