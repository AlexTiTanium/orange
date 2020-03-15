use crate::Gl;
use crate::Layout;
use crate::RenderID;
use crate::VertexBuffer;
use crate::GLT;
use std::ffi::c_void;

pub struct VertexArray {
  id: RenderID,
  gl: Gl,
}

impl VertexArray {
  pub fn new(gl: &Gl) -> Self {
    let mut id: RenderID = 0;
    let gl = gl.clone();

    unsafe {
      gl.GenVertexArrays(1, &mut id);
    }

    Self { id, gl }
  }

  pub fn add_buffer(&self, buffer: &VertexBuffer, layout: &Layout) {
    self.bind();
    buffer.bind();

    unsafe {
      self.gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    }

    let mut offset: usize = 0;
    for (index, element) in layout.elements.iter().enumerate() {
      println!("--------------");
      println!("index : {:?}", index as u32);
      println!("count : {:?}", element.count as i32);
      println!("stride : {:?}", layout.stride as i32);
      println!("offset : {:?}", offset as *const GLT::GLvoid);
      println!("offset : {:?}", offset);
      println!("--------------");
      unsafe {
        self.gl.VertexAttribPointer(
          index as u32,                 // index of the generic vertex attribute ("layout (location = 0)")
          element.count as i32,         // the number of components per generic vertex attribute
          element.element_type,         // data type
          element.normalized,           // normalized (int-to-float conversion)
          layout.stride as i32,         // stride (byte offset between consecutive attributes)
          offset as *const GLT::GLvoid, // offset of the first component
        );
      }
      offset += element.count * element.size;
    }
  }

  pub fn bind(&self) {
    unsafe {
      self.gl.BindVertexArray(self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      self.gl.BindVertexArray(0);
    }
  }
}

impl Drop for VertexArray {
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteVertexArrays(1, &mut self.id);
    }
  }
}
