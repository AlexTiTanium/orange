use crate::ConstVoid;
use crate::RenderID;
use crate::{Gl, GL, GLT};

pub struct Texture {
  id: RenderID,
  gl: Gl,
  width: usize,
  height: usize,
}

impl Texture {
  pub fn new(gl: &Gl) -> Self {
    let mut id: RenderID = 0;
    let gl = gl.clone();

    unsafe {
      gl.GenTextures(1, &mut id);
    }

    Self { id, gl, width: 0, height: 0 }
  }

  pub fn set_data(&mut self, width: usize, height: usize, data: &[u8]) {
    unsafe {
      self.gl.TexImage2D(
        GL::TEXTURE_2D,
        0,
        GL::RGBA as i32,
        width as i32,
        height as i32,
        0,
        GL::RGBA,
        GL::UNSIGNED_BYTE,
        data.as_ptr() as ConstVoid,
      );
    }

    self.width = width;
    self.height = height;
  }

  pub fn generate_mipmap(&self) {
    unsafe {
      self.gl.GenerateMipmap(GL::TEXTURE_2D);
    }
  }

  pub fn set_param(&self, param_one: GLT::GLenum, param_two: GLT::GLenum, param_three: GLT::GLenum) {
    unsafe {
      #[rustfmt::skip]
      self.gl.TexParameteri(param_one, param_two, param_three as i32);
    }
  }

  pub fn set_default_params(&self) {
    unsafe {
      self.gl.Enable(GL::BLEND);
      self.gl.BlendFunc(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
    }

    self.set_param(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT);
    self.set_param(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT);
    self.set_param(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST);
    self.set_param(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST);
  }

  pub fn bind(&self) {
    unsafe {
      self.gl.BindTexture(GL::TEXTURE_2D, self.id);
    }
  }

  pub fn activate(&self, slot: i32) {
    unsafe {
      self.gl.ActiveTexture(GL::TEXTURE0 + slot as u32);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      self.gl.BindTexture(GL::TEXTURE_2D, 0);
    }
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    unsafe {
      self.gl.DeleteTextures(1, &self.id);
    }
  }
}
