use super::resources::Input;
use crate::systems::{clear_window_input, process_window_input};
use common::{stage, Builder, Plugin};

pub struct InputPlugin;

impl Plugin for InputPlugin {
  fn build(&self, app: &mut Builder) {
    app
      .add_resource(Input::default())
      .add_to_stage(stage::PRE_UPDATE, &process_window_input)
      .add_to_stage(stage::POST_UPDATE, &clear_window_input);
  }
}
