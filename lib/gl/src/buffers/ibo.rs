use crate::ConstVoid;
use crate::Gl;
use crate::RenderID;
use crate::SizeIntPtr;
use crate::GL;
use std::{mem, vec::Vec};

pub struct IndexBuffer {
  id: RenderID,
  count: u32,
}

impl IndexBuffer {
  pub fn new(gl: &Gl, data: &Vec<usize>, count: u32) -> Self {
    let mut id: RenderID = 0;

    unsafe {
      gl.GenBuffers(1, &mut id);
      gl.BindBuffer(GL::ELEMENT_ARRAY_BUFFER, id);
      gl.BufferData(
        GL::ELEMENT_ARRAY_BUFFER,
        mem::size_of_val(data) as SizeIntPtr,
        data.as_ptr() as ConstVoid,
        GL::STATIC_DRAW,
      );
    }

    Self { id, count }
  }

  pub fn bind(self: &Self, gl: &Gl) {
    unsafe {
      gl.BindBuffer(GL::ELEMENT_ARRAY_BUFFER, self.id);
    }
  }

  pub fn unbind(self: &Self, gl: &Gl) {
    unsafe {
      gl.BindBuffer(GL::ELEMENT_ARRAY_BUFFER, 0);
    }
  }
}
