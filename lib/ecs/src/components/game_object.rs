use crate::glm::*;
use std::string::String;

#[derive(Debug)]
pub struct GameObject {
  pub name: String,
}

impl Default for GameObject {
  fn default() -> Self {
    Self {
      name: String::from("Game object"),
    }
  }
}

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
