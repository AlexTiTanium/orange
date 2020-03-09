use crate::ConstVoid;
use crate::Gl;
use crate::RenderID;
use crate::SizeIntPtr;
use crate::GL;
use std::{mem, vec::Vec};

pub struct VertexBuffer {
  id: RenderID,
}

impl VertexBuffer {
  pub fn new(gl: &Gl, data: &Vec<f32>) -> Self {
    let mut id: RenderID = 0;

    unsafe {
      gl.GenBuffers(1, &mut id);
      gl.BindBuffer(GL::ARRAY_BUFFER, id);
      gl.BufferData(
        GL::ARRAY_BUFFER,
        mem::size_of_val(data) as SizeIntPtr,
        data.as_ptr() as ConstVoid,
        GL::STATIC_DRAW,
      );
    }

    Self { id }
  }

  pub fn bind(self: &Self, gl: &Gl) {
    unsafe {
      gl.BindBuffer(GL::ARRAY_BUFFER, self.id);
    }
  }

  pub fn unbind(self: &Self, gl: &Gl) {
    unsafe {
      gl.DeleteBuffers(1, &self.id);
    }
  }
}
