mod api;
use state::*;
use std::ffi::c_void;

pub fn create<F>(store: &Store, load: F) -> api::OpenGL
where
  F: FnMut(&'static str) -> *const c_void,
{
  api::OpenGL::new(store, load)
}
