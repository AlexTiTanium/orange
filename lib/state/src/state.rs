use crate::Window;

#[derive(Clone, Debug)]
pub struct State {
  pub window: Window,
}

impl State {
  pub fn default() -> Self {
    Self {
      window: Window::default(),
    }
  }
}
