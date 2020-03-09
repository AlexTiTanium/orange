use crate::IndexBuffer;
use crate::Program;
use crate::VertexArray;
use crate::VertexBuffer;

pub struct RenderTarget {
  pub vbo: VertexBuffer,
  pub ibo: IndexBuffer,
  pub vao: VertexArray,
  pub program: Program,
  pub color_location: i32,
}
