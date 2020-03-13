use super::utils;
use super::ShaderType;
use crate::Gl;
use crate::RenderID;
use crate::ShaderID;

pub struct Program {
  id: RenderID,
  gl: Gl,
  shaders: Vec<ShaderID>,
}

impl Program {
  pub fn new(gl: &Gl) -> Self {
    let gl = gl.clone();
    let id = unsafe { gl.CreateProgram() };
    Self { id, gl, shaders: vec![] }
  }

  pub fn add_shaders(&mut self, shader_type: ShaderType, source: &str) {
    let shader = utils::compile(&self.gl, shader_type, source).unwrap();

    unsafe {
      self.gl.AttachShader(self.id, shader);
    }

    self.shaders.push(shader);
  }

  pub fn link(&mut self) {
    unsafe {
      self.gl.LinkProgram(self.id);
    }

    utils::check_program_errors(&self.gl, self.id).unwrap();

    for &shader in &self.shaders {
      unsafe {
        self.gl.DeleteProgram(shader);
      }
    }

    self.shaders.clear();
  }

  pub fn bind(&self) {
    unsafe {
      self.gl.UseProgram(self.id);
    }
  }
}
