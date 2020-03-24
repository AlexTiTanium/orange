mod api;
use ecs::State;
use std::ffi::c_void;

pub fn create<F>(state: &State, load: F) -> api::OpenGL
where
  F: FnMut(&'static str) -> *const c_void,
{
  api::OpenGL::new(state, load)
}
