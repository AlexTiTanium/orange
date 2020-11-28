mod opengl;

use ecs::State;
use std::ffi::c_void;

pub fn create<F>(state: &State, load: F)
where
  F: FnMut(&'static str) -> *const c_void,
{
  opengl::create(state, load);
}

pub fn step(state: &State) {
  opengl::step(state);
}

pub fn load_textures(state: &State) {
  opengl::load_textures(state);
}
