use crate::ConstVoid;
use crate::Gl;
use crate::RenderID;
use crate::SizeIntPtr;
use crate::GL;
use std::mem;

pub struct VertexBuffer {
  id: RenderID,
  gl: Gl,
}

impl VertexBuffer {
  pub fn new(gl: &Gl) -> Self {
    let mut id: RenderID = 0;
    let gl = gl.clone();

    unsafe {
      gl.GenBuffers(1, &mut id);
    }

    Self { gl, id }
  }

  pub fn set_data(self: &Self, data: &[f32]) {
    self.bind();

    unsafe {
      self.gl.BufferData(
        GL::ARRAY_BUFFER,
        mem::size_of_val(data) as SizeIntPtr,
        data.as_ptr() as ConstVoid,
        GL::STATIC_DRAW,
      );
    };
  }

  pub fn bind(self: &Self) {
    unsafe {
      self.gl.BindBuffer(GL::ARRAY_BUFFER, self.id);
    }
  }

  pub fn unbind(self: &Self) {
    unsafe {
      self.gl.BindBuffer(GL::ARRAY_BUFFER, 0);
    }
  }
}
