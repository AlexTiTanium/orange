use crate::Gl;
use crate::IndexBuffer;
use crate::Program;
use crate::VertexArray;
use crate::VertexBuffer;

pub struct RenderTarget {
  gl: Gl,
  vbo: VertexBuffer,
  ibo: IndexBuffer,
  vao: VertexArray,
  program: Program,
  pub color_location: i32,
}

impl RenderTarget {
  pub fn new(gl: &Gl) -> Self {
    let gl = gl.clone();

    Self {
      gl,
      vbo: VertexBuffer::new(&gl),
      ibo: IndexBuffer::new(&gl),
      vao: VertexArray::new(&gl),
      program: Program::new(&gl),
      color_location: 0,
    }
  }

  pub fn add_vertex_buffer() {}
}
