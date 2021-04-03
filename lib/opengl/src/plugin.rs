use crate::{Gl, GL};
use common::{stage, AllStoragesViewMut, Builder, NonSendSync, Plugin, UniqueView, UniqueViewMut};
use window::WindowContext;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
  fn build(&self, app: &mut Builder) {
    app
      .add_to_stage(stage::PRE_RENDER, &clear)
      .add_to_stage(stage::POST_STARTUP, &init_gl_context);
  }
}

pub fn clear(gl: NonSendSync<UniqueView<Gl>>) {
  unsafe {
    gl.ClearColor(0.2, 0.2, 0.2, 1.0);
    gl.Clear(GL::COLOR_BUFFER_BIT);
  }
}

pub fn init_gl_context(all_storages: AllStoragesViewMut) {
  let context = all_storages.borrow::<NonSendSync<UniqueViewMut<WindowContext>>>().unwrap();
  let gl = Gl::load_with(|symbol| context.get_proc_address(symbol));
  all_storages.add_unique_non_send_sync(gl);
}
