use opengl_api;

pub use render_components::*;

use game::State;
use std::ffi::c_void;

/// Module initialization
pub fn init(state: &State) {
  opengl_api::init(state);
}

// pub fn create<F>(state: &State, load: F)
// where
//   F: FnMut(&'static str) -> *const c_void,
// {
//   opengl::create(state, load);
// }

// pub fn step(state: &State) {
//   opengl::step(state);
// }

// pub fn load_textures(state: &State) {
//   opengl::load_textures(state);
// }
