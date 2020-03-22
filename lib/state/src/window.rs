use crate::Action;
use crate::State;

#[derive(Clone, Debug)]
pub struct Window {
  pub width: u32,
  pub height: u32,
}

impl Window {
  pub fn default() -> Self {
    Self { width: 1024, height: 768 }
  }

  pub fn reducer(state: &State, action: &Action) -> State {
    match action {
      Action::WindowResize(width, height) => Self::resize(state, *width, *height),
      _ => state.clone(),
    }
  }

  fn resize(state: &State, width: u32, height: u32) -> State {
    State {
      window: Window { width, height },
      ..state.clone()
    }
  }
}
