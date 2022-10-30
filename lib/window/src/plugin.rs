use crate::events::WindowInputEvent;
use crate::resources::WindowSize;
use crate::systems::{swap_buffers, update_window_size};
use crate::{runner, systems::on_window_resize};
use common::{stage, Builder, Plugin};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
  fn build(&self, app: &mut Builder) {
    app
      .add_resource(WindowSize::default())
      .add_startup_system(&update_window_size)
      .add_to_stage(stage::PRE_UPDATE, &on_window_resize)
      .add_to_stage(stage::POST_RENDER, &swap_buffers)
      .add_event::<WindowInputEvent>()
      .set_runner(runner::window_runner);
  }
}
