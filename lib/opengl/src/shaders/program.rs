use super::utils;
use super::ProgramError;
use super::ShaderType;
use crate::glm::Mat4;
use crate::Gl;
use crate::RenderID;
use crate::ShaderID;
use crate::GLT;
use std::collections::HashMap;
use std::ffi::CString;

type Location = GLT::GLint;

pub struct Program {
  id: RenderID,
  gl: Gl,
  shaders: Vec<ShaderID>,
  locations: HashMap<CString, Location>,
}

impl Program {
  pub fn new(gl: &Gl) -> Self {
    let gl = gl.clone();
    let id = unsafe { gl.CreateProgram() };

    Self {
      id,
      gl,
      shaders: vec![],
      locations: HashMap::new(),
    }
  }

  fn delete_shaders(&mut self) {
    for &shader in &self.shaders {
      unsafe {
        self.gl.DeleteProgram(shader);
      }
    }

    self.shaders.clear();
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

    self.delete_shaders();
  }

  pub fn bind(&self) {
    unsafe {
      self.gl.UseProgram(self.id);
    }
  }

  pub fn create_uniform_location(&mut self, name: &str) -> Result<Location, ProgramError> {
    self.bind();

    let uniform_name = CString::new(name).unwrap();

    let new_location = unsafe { self.gl.GetUniformLocation(self.id, uniform_name.as_ptr()) };

    if new_location == -1 {
      return Err(ProgramError::NoLocation(uniform_name));
    }

    self.locations.insert(uniform_name, new_location);
    Ok(new_location)
  }

  pub fn get_uniform_location(&self, name: &str) -> Result<Location, ProgramError> {
    let uniform_name = CString::new(name).unwrap();

    let found_location = self.locations.get(&uniform_name);

    match found_location {
      Some(&location) => Ok(location),
      None => Err(ProgramError::CreateLocation(uniform_name)),
    }
  }

  pub fn uniform_mat4(&self, name: &str, data: &Mat4) {
    let location = self.get_uniform_location(name).unwrap();
    unsafe {
      self.gl.UniformMatrix4fv(location, 1, 0, data.as_ptr());
    }
  }

  pub fn uniform1i(&self, name: &str, data: i32) {
    let location = self.get_uniform_location(name).unwrap();
    unsafe {
      self.gl.Uniform1i(location, data);
    }
  }

  pub fn uniform4f(&self, name: &str, data: &[f32; 4]) {
    let location = self.get_uniform_location(name).unwrap();
    unsafe {
      self.gl.Uniform4f(location, data[0], data[1], data[2], data[3]);
    }
  }
}

impl Drop for Program {
  fn drop(&mut self) {
    self.delete_shaders();
    unsafe {
      self.gl.DeleteProgram(self.id);
    }
  }
}
