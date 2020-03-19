use crate::ConstVoid;
use crate::Gl;
use crate::RenderID;
use crate::GL;

pub struct Texture {
  id: RenderID,
  gl: Gl,
}

impl Texture {
  pub fn new(gl: &Gl) -> Self {
    let mut id: RenderID = 0;
    let gl = gl.clone();

    unsafe {
      gl.GenTextures(1, &mut id);
    }

    Self { id, gl }
  }

  pub fn set_data(&mut self, width: i32, height: i32, data: &[u8]) {
    unsafe {
      self.gl.TexImage2D(
        GL::TEXTURE_2D,
        0,
        GL::RGBA as i32,
        width,
        height,
        0,
        GL::RGBA,
        GL::UNSIGNED_BYTE,
        data.as_ptr() as ConstVoid,
      );
    }
  }

  pub fn generate_mipmap(&self) {
    unsafe {
      self.gl.GenerateMipmap(GL::TEXTURE_2D);
    }
  }

  pub fn set_param(&self) {
    unsafe {
      self.bind();
      self.gl.Enable(GL::BLEND);
      self.gl.BlendFunc(GL::SRC_ALPHA, GL::ONE_MINUS_SRC_ALPHA);
      #[rustfmt::skip]
      self.gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::REPEAT as i32);
      #[rustfmt::skip]
      self.gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::REPEAT as i32);
      #[rustfmt::skip]
      self.gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
      #[rustfmt::skip]
      self.gl.TexParameteri(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
    }
  }

  pub fn bind(&self) {
    unsafe {
      // self.gl.ActiveTexture(GL::TEXTURE0);
      self.gl.BindTexture(GL::TEXTURE_2D, self.id);
    }
  }

  pub fn unbind(&self) {
    unsafe {
      self.gl.BindBuffer(GL::TEXTURE_2D, 0);
    }
  }
}

// impl Drop for Texture {
//   fn drop(&mut self) {
//     //unsafe {
//     // self.gl.DeleteBuffers(1, &mut self.id);
//     //}
//   }
// }
