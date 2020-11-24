mod opengl;

use ecs::State;
use std::ffi::c_void;

pub fn create<F>(state: &State, load: F)
where
  F: FnMut(&'static str) -> *const c_void,
{
  opengl::create(state, load);
}

pub fn update(state: &State) {
  opengl::update(state);
}

pub fn step(state: &State) {
  opengl::step(state);
}
