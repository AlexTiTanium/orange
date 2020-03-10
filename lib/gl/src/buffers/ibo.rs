use crate::ConstVoid;
use crate::Gl;
use crate::RenderID;
use crate::SizeIntPtr;
use crate::GL;
use std::{mem, ptr};

pub struct IndexBuffer {
  id: RenderID,
  gl: Gl,
  pub count: i32,
}

impl IndexBuffer {
  pub fn new(gl: &Gl) -> Self {
    let mut id: RenderID = 0;
    let gl = gl.clone();

    unsafe {
      gl.GenBuffers(1, &mut id);
    }

    Self { id, count, gl }
  }

  pub fn set_data(self: &Self, data: &[u16], count: i32) {
    unsafe {
      self.gl.BindBuffer(GL::ELEMENT_ARRAY_BUFFER, self.id);
      self.gl.BufferData(
        GL::ELEMENT_ARRAY_BUFFER,
        mem::size_of_val(data) as SizeIntPtr,
        data.as_ptr() as ConstVoid,
        GL::STATIC_DRAW,
      );
    }
  }

  pub fn draw(self: &Self) {
    unsafe {
      self.gl.DrawElements(GL::TRIANGLES, self.count, GL::UNSIGNED_SHORT, ptr::null());
    }
  }

  pub fn bind(self: &Self) {
    unsafe {
      self.gl.BindBuffer(GL::ELEMENT_ARRAY_BUFFER, self.id);
    }
  }

  pub fn unbind(self: &Self) {
    unsafe {
      self.gl.BindBuffer(GL::ELEMENT_ARRAY_BUFFER, 0);
    }
  }
}
