use crate::events::{WindowInnerEvent, WindowInputEvent};
use crate::runner;
use common::{Builder, Plugin};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
  fn build(&self, app: &mut Builder) {
    app
      .add_event::<WindowInputEvent>()
      .add_event::<WindowInnerEvent>()
      .set_runner(runner::window_runner);
  }
}
