use crate::Gl;
use crate::Layout;
use crate::RenderID;
use crate::VertexBuffer;
use std::ffi::c_void;

pub struct VertexArray {
  id: RenderID,
}

impl VertexArray {
  pub fn new(gl: &Gl) -> Self {
    let mut id: RenderID = 0;

    unsafe {
      gl.GenVertexArrays(1, &mut id);
    }

    Self { id }
  }

  pub fn add_buffer(self: &Self, gl: &Gl, buffer: &VertexBuffer, layout: &Layout) {
    self.bind(&gl);
    buffer.bind(&gl);

    unsafe {
      gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    }

    let mut offset: usize = 0;
    for (index, element) in layout.elements.iter().enumerate() {
      unsafe {
        gl.VertexAttribPointer(
          index as u32,            // index of the generic vertex attribute ("layout (location = 0)")
          element.count as i32,    // the number of components per generic vertex attribute
          element.element_type,    // data type
          element.normalized,      // normalized (int-to-float conversion)
          layout.stride as i32,    // stride (byte offset between consecutive attributes)
          offset as *const c_void, // offset of the first component
        );
      }
      offset += element.count * element.size;
    }
  }

  pub fn bind(self: &Self, gl: &Gl) {
    unsafe {
      gl.BindVertexArray(self.id);
    }
  }
}
