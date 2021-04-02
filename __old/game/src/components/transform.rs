use crate::glm::*;

#[derive(Debug)]
pub struct Transform {
  pub position: Vec3,
}

impl Default for Transform {
  fn default() -> Self {
    Self {
      position: vec3(0.0, 0.0, 0.0),
    }
  }
}

impl Transform {
  pub fn new(x: f32, y: f32) -> Self {
    Self {
       position: vec3(x, y, 0.0),
    }
  }

  pub fn set(&mut self, x: f32, y: f32) {
    self.position.x = x;
    self.position.y = y;
  }
}