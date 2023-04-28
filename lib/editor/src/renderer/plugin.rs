use super::render::EditorRenderer;
use common::{stage, Builder, Plugin};

pub struct EditorRenderPlugin;

impl Plugin for EditorRenderPlugin {
  fn build(&self, app: &mut Builder) {
    app
      .add_to_stage(stage::POST_STARTUP, &EditorRenderer::init)
      .add_to_stage(stage::RENDER, &EditorRenderer::draw);
  }
}
