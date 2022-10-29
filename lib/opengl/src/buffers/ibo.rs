use crate::ConstVoid;
use crate::Gl;
use crate::RenderID;
use crate::SizeIntPtr;
use crate::GL;
use crate::GLT;
use std::{any, mem, ptr};

pub struct IndexBuffer {
  id: RenderID,
  gl: Gl,
  indexes_type: GLT::GLenum,
  pub size: usize, // Buffer size in bytes
  pub count: usize,
}

impl IndexBuffer {
  pub fn new(gl: &Gl) -> Self {
    let mut id: RenderID = 0;
    let gl = gl.clone();

    unsafe {
      gl.GenBuffers(1, &mut id);
    }

    Self {
      id,
      count: 0,
      size: 0,
      gl,
      indexes_type: GL::UNSIGNED_SHORT,
    }
  }

  pub fn set_data<T>(&mut self, data: &[T], count: usize) {
    self.count = count;

    self.indexes_type = match any::type_name::<T>() {
      "u32" => GL::UNSIGNED_INT,
      "u16" => GL::UNSIGNED_SHORT,
      _ => panic!("Unsupported ibo type"),
    };

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

  pub fn set_sub_data<T>(&mut self, data: &[T]) {
    self.count = data.len();

    self.indexes_type = match any::type_name::<T>() {
      "u32" => GL::UNSIGNED_INT,
      "u16" => GL::UNSIGNED_SHORT,
      _ => panic!("Unsupported ibo type"),
    };

    unsafe {
      self.gl.BufferSubData(
        GL::ELEMENT_ARRAY_BUFFER,
        0,
        mem::size_of_val(data) as SizeIntPtr,
        data.as_ptr() as ConstVoid,
      );
    };
  }

  pub fn set_size(&mut self, size: usize) {
    unsafe {
      self
        .gl
        .BufferData(GL::ELEMENT_ARRAY_BUFFER, size as SizeIntPtr, 0 as ConstVoid, GL::DYNAMIC_DRAW);
    };

    self.size = size;
  }

  pub fn draw(&self) {
    unsafe {
      self.gl.DrawElements(GL::TRIANGLES, self.count as i32, self.indexes_type, ptr::null());
    }
  }

  pub fn bind(&self) {
    unsafe {
      self.gl.BindBuffer(GL::ELEMENT_ARRAY_BUFFER, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      self.gl.BindBuffer(GL::ELEMENT_ARRAY_BUFFER, 0);
    }
  }
}

impl Drop for IndexBuffer {
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteBuffers(1, &self.id);
    }
  }
}
