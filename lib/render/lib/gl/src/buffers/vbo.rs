use crate::ConstVoid;
use crate::Gl;
use crate::RenderID;
use crate::SizeIntPtr;
use crate::GL;
use std::mem;

pub struct VertexBuffer {
  id: RenderID,
  gl: Gl,
  pub size: usize, // Buffer size in bytes
}

impl VertexBuffer {
  pub fn new(gl: &Gl) -> Self {
    let mut id: RenderID = 0;
    let gl = gl.clone();

    unsafe {
      gl.GenBuffers(1, &mut id);
    }

    Self { gl, id, size: 0 }
  }

  pub fn set_data<T>(&self, data: &[T]) {
    unsafe {
      self.gl.BufferData(
        GL::ARRAY_BUFFER,
        mem::size_of_val(data) as SizeIntPtr,
        data.as_ptr() as ConstVoid,
        GL::STATIC_DRAW,
      );
    };
  }

  pub fn set_sub_data<T>(&self, data: &[T]) {
    unsafe {
      self
        .gl
        .BufferSubData(GL::ARRAY_BUFFER, 0, mem::size_of_val(data) as SizeIntPtr, data.as_ptr() as ConstVoid);
    };
  }

  pub fn set_size(&mut self, size: usize) {
    unsafe {
      self.gl.BufferData(GL::ARRAY_BUFFER, size as SizeIntPtr, 0 as ConstVoid, GL::DYNAMIC_DRAW);
    };

    self.size = size;
  }

  pub fn bind(&self) {
    unsafe {
      self.gl.BindBuffer(GL::ARRAY_BUFFER, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      self.gl.BindBuffer(GL::ARRAY_BUFFER, 0);
    }
  }
}

impl Drop for VertexBuffer {
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteBuffers(1, &self.id);
    }
  }
}
