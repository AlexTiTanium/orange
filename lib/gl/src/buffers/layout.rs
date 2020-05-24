use crate::GL;
use crate::GLT;
use std::any;
use std::mem::size_of;

pub struct VertexBufferElement {
  pub element_type: u32,
  pub count: usize,
  pub size: usize,
  pub normalized: u8,
}

#[derive(Default)]
pub struct Layout {
  pub elements: Vec<VertexBufferElement>,
  pub stride: usize,
}

impl Layout {
  pub fn new() -> Self {
    Self { elements: vec![], stride: 0 }
  }

  pub fn push<T>(&mut self, count: usize) {
    let (element_type, size, normalized) = match any::type_name::<T>() {
      "f32" => (GL::FLOAT, size_of::<GLT::GLfloat>(), GL::FALSE),
      "i32" => (GL::INT, size_of::<GLT::GLuint>(), GL::FALSE),
      "u32" => (GL::UNSIGNED_INT, size_of::<GLT::GLuint>(), GL::FALSE),
      "u8" => (GL::UNSIGNED_BYTE, size_of::<GLT::GLbyte>(), GL::TRUE),
      _ => panic!("Unsupported layout type"),
    };

    let el = VertexBufferElement {
      element_type,
      count,
      size,
      normalized,
    };

    self.elements.push(el);
    self.stride += count * size;
  }
}

impl Drop for Layout {
  fn drop(&mut self) {
    self.elements.clear();
  }
}
