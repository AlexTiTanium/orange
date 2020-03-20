use crate::Gl;
use crate::IndexBuffer;
use crate::Layout;
use crate::Program;
use crate::ShaderType;
use crate::Texture;
use crate::TextureSlot;
use crate::VertexArray;
use crate::VertexBuffer;
use crate::GL;

pub struct Renderer {
  gl: Gl,
  vbo: VertexBuffer,
  ibo: IndexBuffer,
  vao: VertexArray,
  layout: Layout,
  program: Program,
  texture: Texture,
}

impl Renderer {
  pub fn new(gl: &Gl) -> Self {
    let gl = gl.clone();

    Self {
      vbo: VertexBuffer::new(&gl),
      ibo: IndexBuffer::new(&gl),
      vao: VertexArray::new(&gl),
      layout: Layout::new(),
      program: Program::new(&gl),
      texture: Texture::new(&gl),
      gl,
    }
  }

  pub fn add_vertices<T>(&mut self, vertices: &[T]) -> &mut Self {
    self.vbo.bind();
    self.vbo.set_data::<T>(&vertices);
    self.vbo.unbind();
    self
  }

  pub fn add_layout<T>(&mut self, count: usize) -> &mut Self {
    self.layout.push::<T>(count);
    self
  }

  pub fn commit_layout(&mut self) -> &mut Self {
    self.vao.bind();
    self.vbo.bind();
    self.vao.add_buffer(&self.layout);
    self.vbo.unbind();
    self.vao.unbind();
    self
  }

  pub fn add_shader(&mut self, shader_type: ShaderType, source: &str) -> &mut Self {
    self.program.add_shaders(shader_type, source);
    self
  }

  pub fn commit_shaders(&mut self) -> &mut Self {
    self.program.link();
    self
  }

  pub fn add_indexes<T>(&mut self, indexes: &[T], count: i32) -> &mut Self {
    self.ibo.bind();
    self.ibo.set_data::<T>(&indexes, count);
    self.ibo.unbind();
    self
  }

  pub fn add_texture(&mut self, slot: TextureSlot, width: usize, height: usize, data: &[u8]) -> &mut Self {
    self.select_texture_slot(slot);
    self.texture.set_param();
    self.texture.generate_mipmap();
    self.texture.set_data(width as i32, height as i32, data);
    self.texture.unbind();
    // TODO: Should I free image buffer?
    // TODO: Crate consume_texture method variant ?
    self
  }

  pub fn debug(&mut self) -> &mut Self {
    unsafe {
      self.gl.PolygonMode(GL::FRONT_AND_BACK, GL::LINE);
    }
    self
  }

  pub fn set_uniform_i1(&mut self, name: &str, data: i32) {
    self.program.uniform1i(&name, data);
  }

  pub fn set_uniform_f4(&mut self, name: &str, data: &[f32; 4]) {
    self.program.uniform4f(&name, &data);
  }

  pub fn bind(&self) {
    self.vao.bind();
    self.ibo.bind();
    self.program.bind();
  }

  pub fn select_texture_slot(&self, slot: TextureSlot) {
    self.texture.bind(slot);
  }

  pub fn draw(&self) {
    self.ibo.draw();
  }
}
