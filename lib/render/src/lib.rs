mod api;
use gl::GLT;
use state::*;

pub fn create<F>(store: &Store, load: F) -> api::OpenGL
where
  F: FnMut(&'static str) -> *const GLT::GLvoid,
{
  api::OpenGL::new(store, load)
}
