use crate::systems::{request_redraw, swap_buffers, update_window_size};
use crate::{resources::WindowSize, WindowResizeEvent};
use crate::{runner, systems::window_resize};
use common::{stage, Builder, Plugin};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
  fn build(&self, app: &mut Builder) {
    app
      .add_resource(WindowSize::default())
      .add_startup_system(&update_window_size)
      .add_to_stage(stage::PRE_UPDATE, &window_resize)
      .add_to_stage(stage::PRE_RENDER, &request_redraw)
      .add_to_stage(stage::POST_RENDER, &swap_buffers)
      .add_event::<WindowResizeEvent>()
      .set_runner(runner::window_runner);
  }
}
