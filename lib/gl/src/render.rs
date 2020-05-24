use crate::glm;
use crate::glm::{Mat4, Vec3};
use crate::{Gl, GL};
use crate::{IndexBuffer, Layout, Program, ShaderType, Texture, VertexArray, VertexBuffer};
use std::collections::HashMap;

pub struct Renderer {
  gl: Gl,
  vbo: VertexBuffer,
  ibo: IndexBuffer,
  vao: VertexArray,
  layout: Layout,
  program: Program,
  textures: HashMap<u32, Texture>,
  view_projection: Mat4,
  model: Mat4,
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
      view_projection: glm::identity(),
      model: glm::identity(),
    }
  }

  pub fn set_data(&mut self, data: &[f32]) {
    self.vbo.bind();
    self.vbo.set_sub_data::<f32>(&data);
    self.vbo.unbind();
  }

  pub fn add_vertices<T>(&mut self, vertices: &[T]) -> &mut Self {
    self.vbo.bind();
    self.vbo.set_data::<T>(&vertices);
    self.vbo.unbind();
    self
  }

  pub fn set_vertices_buffer_size(&mut self, size: usize) -> &mut Self {
    self.vbo.bind();
    self.vbo.set_size(size);
    self.vbo.unbind();
    self
  }

  pub fn add_layout<T>(&mut self, count: usize) -> &mut Self {
    self.layout.push::<T>(count);
    self
  }

  pub fn build_layout(&mut self) -> &mut Self {
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

  pub fn build_shader(&mut self) -> &mut Self {
    self.program.link();
    self
  }

  pub fn add_indexes<T>(&mut self, indexes: &[T], count: usize) -> &mut Self {
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

  pub fn set_uniform_mat4(&self, name: &str, data: &Mat4) {
    self.program.uniform_mat4(&name, data);
  }

  pub fn set_uniform_mat4_rm(&self, name: &str, data: &Mat4) {
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

    self.set_uniform_mat4("u_ViewProjection", &self.view_projection);
  }

  pub fn translate(&mut self, vec3: &Vec3) {
    self.model = glm::translate(&glm::identity(), vec3);
  }

  pub fn create_mvp(&mut self) -> &mut Self {
    self.create_uniform("u_ViewProjection");
    self
  }

  pub fn set_view_projection(&mut self, vp: &Mat4) {
    self.view_projection = vp.clone();
  }

  pub fn draw(&self) {
    self.ibo.draw();
  }
}
