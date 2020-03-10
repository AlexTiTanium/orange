use std::ops::Deref;
use std::rc::Rc;

pub mod bindings {
  //include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
  include!("./bindings/gl-41-debug.rs");
  //include!("./bindings/gl-41-release.rs");
}

pub use bindings as GL;
pub use bindings::types as GLT;

#[derive(Clone)]
pub struct Gl {
  inner: Rc<bindings::Gl>,
}

impl Gl {
  pub fn load_with<F>(loadfn: F) -> Gl
  where
    F: FnMut(&'static str) -> *const GLT::GLvoid,
  {
    Gl {
      inner: Rc::new(bindings::Gl::load_with(loadfn)),
    }
  }
}

impl Deref for Gl {
  type Target = bindings::Gl;

  fn deref(&self) -> &bindings::Gl {
    &self.inner
  }
}
