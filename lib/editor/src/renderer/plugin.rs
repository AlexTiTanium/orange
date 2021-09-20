use super::{camera::EditorCamera, render::EditorRenderer};
use common::{stage, AllStoragesViewMut, Builder, NonSendSync, Plugin, UniqueViewMut};
use opengl::Gl;

pub struct EditorRenderPlugin;

impl Plugin for EditorRenderPlugin {
  fn build(&self, app: &mut Builder) {
    app
      .add_startup_system(&EditorCamera::init)
      .add_resource(EditorCamera::default())
      .add_to_stage(stage::POST_STARTUP, &init_renderer)
      .add_to_stage(stage::RENDER, &EditorRenderer::draw)
      .add_system(&EditorCamera::update);
  }
}

fn init_renderer(all_storages: AllStoragesViewMut) {
  let gl = all_storages.borrow::<NonSendSync<UniqueViewMut<Gl>>>().unwrap();
  all_storages.add_unique_non_send_sync(EditorRenderer::new(&gl));
}
