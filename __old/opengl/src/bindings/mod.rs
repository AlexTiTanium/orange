use std::ops::Deref;
use std::rc::Rc;

pub mod open_gl {
  //include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
  #![allow(clippy::all)]
  // include!("./gl-41-debug.rs");
  include!("./gl-41-release.rs");
}

pub use open_gl as GL;
pub use open_gl::types as GLT;

#[derive(Clone)]
pub struct Gl {
  inner: Rc<open_gl::Gl>,
}

impl Gl {
  pub fn load_with<F>(loadfn: F) -> Gl
  where
    F: FnMut(&'static str) -> *const GLT::GLvoid,
  {
    Gl {
      inner: Rc::new(open_gl::Gl::load_with(loadfn)),
    }
  }
}

impl Deref for Gl {
  type Target = open_gl::Gl;

  fn deref(&self) -> &open_gl::Gl {
    &self.inner
  }
}
