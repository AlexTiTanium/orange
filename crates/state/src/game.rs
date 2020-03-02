use crate::Action;

#[derive(Clone)]
pub struct Game {
  pub count: i32,
}

impl Game {
  pub fn default() -> Self {
    Self { count: 0 }
  }

  pub fn reducer(state: &Self, action: &Action) -> Self {
    match action {
      Action::Increment => Self::increment(state),
      Action::Decrement => Self::decrement(state),
      _ => state.clone(),
    }
  }

  fn increment(state: &Self) -> Self {
    Self {
      count: state.count + 1,
    }
  }

  fn decrement(state: &Self) -> Self {
    Self {
      count: state.count - 1,
    }
  }
}
