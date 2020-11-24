mod quad_color;
use quad_color::QuadColorRender;

use ecs::{NonSendSync, State, UniqueView, UniqueViewMut};
use gl::{Gl, GL};
use std::ffi::c_void;

pub fn create<F>(state: &State, load: F)
where
  F: FnMut(&'static str) -> *const c_void,
{
  let gl = Gl::load_with(load);

  state.world.add_unique_non_send_sync(gl.clone());
  state.world.add_unique_non_send_sync(QuadColorRender::new(&gl));

  // let mut render = api::OpenGL::new(state, load);
  // render.fill_buffer(&state);

  // render
}

pub fn update(state: &State) {
  let mut quad_color_render = state.world.borrow::<NonSendSync<UniqueViewMut<QuadColorRender>>>();

  quad_color_render.update(state);
}

pub fn step(state: &State) {
  let mut quad_color_render = state.world.borrow::<NonSendSync<UniqueViewMut<QuadColorRender>>>();

  clear(state);

  quad_color_render.step(state);
}

fn clear(state: &State) {
  let gl = state.world.borrow::<NonSendSync<UniqueView<Gl>>>();

  unsafe {
    gl.ClearColor(0.2, 0.2, 0.2, 1.0);
    gl.Clear(GL::COLOR_BUFFER_BIT);
  }
}
