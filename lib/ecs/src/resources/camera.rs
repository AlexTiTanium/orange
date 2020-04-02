use crate::glm::*;

pub struct Camera {
  pub view: TMat4<f32>,
}

impl Default for Camera {
  fn default() -> Self {
    Self { view: identity() }
  }
}
