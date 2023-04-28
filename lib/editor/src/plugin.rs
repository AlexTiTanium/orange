use crate::renderer::EditorRenderPlugin;
use common::{Builder, Plugin};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
  fn build(&self, app: &mut Builder) {
    app.add_plugin(EditorRenderPlugin);
  }
}
