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