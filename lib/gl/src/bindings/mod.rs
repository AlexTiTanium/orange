use std::ops::Deref;
use std::rc::Rc;

pub mod OpenGl {
  //include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
  include!("./gl-41-debug.rs");
  //include!("./gl-41-release.rs");
}

pub use OpenGl as GL;
pub use OpenGl::types as GLT;

#[derive(Clone)]
pub struct Gl {
  inner: Rc<OpenGl::Gl>,
}

impl Gl {
  pub fn load_with<F>(loadfn: F) -> Gl
  where
    F: FnMut(&'static str) -> *const GLT::GLvoid,
  {
    Gl {
      inner: Rc::new(OpenGl::Gl::load_with(loadfn)),
    }
  }
}

impl Deref for Gl {
  type Target = OpenGl::Gl;

  fn deref(&self) -> &OpenGl::Gl {
    &self.inner
  }
}
