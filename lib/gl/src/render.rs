use crate::glm;
use crate::glm::*;
use crate::Gl;
use crate::IndexBuffer;
use crate::Layout;
use crate::Program;
use crate::ShaderType;
use crate::Texture;
use crate::VertexArray;
use crate::VertexBuffer;
use crate::GL;
use std::collections::HashMap;

pub struct Renderer {
  gl: Gl,
  vbo: VertexBuffer,
  ibo: IndexBuffer,
  vao: VertexArray,
  layout: Layout,
  program: Program,
  textures: HashMap<u32, Texture>,
  projection: TMat4<f32>,
  pub model: TMat4<f32>,
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
      textures: HashMap::new(),
      gl,
      projection: glm::identity(),
      model: glm::identity(),
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

  pub fn add_texture(&mut self, slot: u32, width: usize, height: usize, data: &[u8]) -> &mut Self {
    let mut texture = Texture::new(&self.gl);

    texture.bind(slot);
    texture.set_param();
    texture.generate_mipmap();
    texture.set_data(width, height, data);
    texture.unbind();

    self.textures.insert(slot, texture);

    self
  }

  pub fn debug(&mut self) -> &mut Self {
    unsafe {
      self.gl.PolygonMode(GL::FRONT_AND_BACK, GL::LINE);
    }
    self
  }

  pub fn create_uniform(&mut self, name: &str) {
    self.program.create_uniform_location(name).unwrap();
  }

  pub fn set_uniform_mat4(&self, name: &str, data: &TMat4<f32>) {
    self.program.uniform_mat4(&name, data);
  }

  pub fn set_uniform_mat4_rm(&self, name: &str, data: &TMat4<f32>) {
    self.program.uniform_mat4(&name, data);
  }

  pub fn set_uniform_i1(&self, name: &str, data: i32) {
    self.program.uniform1i(&name, data);
  }

  pub fn set_uniform_f4(&self, name: &str, data: &[f32; 4]) {
    self.program.uniform4f(&name, &data);
  }

  pub fn bind_texture(&self, slot: u32) {
    self.textures[&slot].bind(slot);
    self.set_uniform_i1("u_Texture", slot as i32);
  }

  pub fn unbind_texture(&self, slot: u32) {
    self.textures[&slot].unbind();
  }

  pub fn bind(&mut self) {
    self.vao.bind();
    self.ibo.bind();
    self.program.bind();

    self.set_uniform_mat4("u_Projection", &self.projection);
    self.set_uniform_mat4("u_Model", &self.model);
  }

  pub fn translate(&mut self, vec3: &Vec3) {
    self.model = glm::translate(&glm::identity(), vec3);
  }

  pub fn create_mvp(&mut self, width: u32, height: u32) {
    self.projection = glm::ortho(0.0, width as f32, 0.0, height as f32, -1.0, 1.0);

    self.create_uniform("u_Projection");
    self.create_uniform("u_View");
    self.create_uniform("u_Model");

    self.set_uniform_mat4("u_Projection", &self.projection);
    self.set_uniform_mat4("u_Model", &self.model);
  }

  pub fn draw(&self) {
    self.ibo.draw();
  }

  pub fn clear(&self) {
    unsafe {
      self.gl.ClearColor(0.2, 0.2, 0.2, 1.0);
      self.gl.Clear(GL::COLOR_BUFFER_BIT);
    }
  }
}
