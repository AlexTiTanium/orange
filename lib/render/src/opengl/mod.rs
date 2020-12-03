mod quad_color;
use quad_color::QuadColorRender;

mod quad_atlas;
use quad_atlas::QuadAtlasRender;

use game::components::{Layer, Texture};
use game::{IntoIter};
use game::{NonSendSync, State, UniqueView, UniqueViewMut, View, IntoWithId};
use gl::{Gl, GL};
use std::ffi::c_void;

pub fn create<F>(state: &State, load: F)
where
  F: FnMut(&'static str) -> *const c_void,
{
  let gl = Gl::load_with(load);

  state.world.add_unique_non_send_sync(gl.clone());
  state.world.add_unique_non_send_sync(QuadColorRender::new(&gl));
  state.world.add_unique_non_send_sync(QuadAtlasRender::new(&gl));
}

pub fn load_textures(state: &State) {
  // When level is ready we load our textures to GPU once
  let mut quad_atlas_render = state.world.borrow::<NonSendSync<UniqueViewMut<QuadAtlasRender>>>();
  quad_atlas_render.load_textures(state);
}

pub fn step(state: &State) {
  let (mut color_render, mut texture_render, layers, textures, gl) = state.world.borrow::<(
    NonSendSync<UniqueViewMut<QuadColorRender>>,
    NonSendSync<UniqueViewMut<QuadAtlasRender>>,
    View<Layer>,
    View<Texture>,
    NonSendSync<UniqueView<Gl>>,
  )>();

  unsafe {
    gl.ClearColor(0.2, 0.2, 0.2, 1.0);
    gl.Clear(GL::COLOR_BUFFER_BIT);
  }

  for (layer_id, _) in (&layers).iter().with_id() {
    for (texture_id, _) in (&textures).iter().with_id() {
      texture_render.step(state, layer_id, texture_id);
    }

    //color_render.step(state, layer_id);
  }
}
