use crate::glm::*;

#[derive(Debug)]
pub struct Position(pub Vec3);

impl Default for Position {
  fn default() -> Self {
    Self(vec3(0.0, 0.0, 0.0))
  }
}

impl Position {
  pub fn set_x(&mut self, x: f32) {
    self.0.x = x;
  }
  pub fn set_y(&mut self, y: f32) {
    self.0.y = y;
  }
  pub fn set_xy(&mut self, x: f32, y: f32) {
    self.0.x = x;
    self.0.y = y;
  }
  pub fn add_x(&mut self, x: f32) {
    self.0.x += x;
  }
  pub fn add_y(&mut self, y: f32) {
    self.0.y += y;
  }
  pub fn minus_x(&mut self, x: f32) {
    self.0.x -= x;
  }
  pub fn minus_y(&mut self, y: f32) {
    self.0.y -= y;
  }
  pub fn x(&self) -> f32 {
    self.0.x
  }
  pub fn y(&self) -> f32 {
    self.0.y
  }
  pub fn vec(&self) -> Vec3 {
    self.0
  }
  pub fn mut_vec(&mut self) -> Vec3 {
    self.0
  }
}
