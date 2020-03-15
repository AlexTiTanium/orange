use crate::ConstVoid;
use crate::Gl;
use crate::Layout;
use crate::RenderID;
use crate::VertexBuffer;

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

    let mut offset: usize = 0;
    for (index, element) in layout.elements.iter().enumerate() {
      unsafe {
        self.gl.EnableVertexAttribArray(index as u32); // this is "layout (location = 0)" in vertex shader
        self.gl.VertexAttribPointer(
          index as u32,         // index of the generic vertex attribute ("layout (location = 0)")
          element.count as i32, // the number of components per generic vertex attribute
          element.element_type, // data type
          element.normalized,   // normalized (int-to-float conversion)
          layout.stride as i32, // stride (byte offset between consecutive attributes)
          offset as ConstVoid,  // offset of the first component
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
