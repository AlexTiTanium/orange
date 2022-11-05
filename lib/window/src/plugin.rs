use crate::events::{WindowInnerEvent, WindowInputEvent};
use crate::resources::WindowSize;
use crate::runner;
use common::{Builder, Plugin};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
  fn build(&self, app: &mut Builder) {
    app
      .add_resource(WindowSize::default())
      //.add_startup_system(&update_window_size)
      .add_event::<WindowInputEvent>()
      .add_event::<WindowInnerEvent>()
      .set_runner(runner::window_runner);
  }
}
