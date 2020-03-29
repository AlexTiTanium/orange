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

#[derive(Default)]
pub struct Texture {
  pub id: String,
  pub width: u32,
  pub height: u32,
}

impl Texture {
  pub fn new(id: &str, width: u32, height: u32) -> Self {
    Self {
      id: String::from(id),
      width,
      height,
    }
  }
}
