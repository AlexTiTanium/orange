use crate::State;

pub trait Module {
  fn init(&self, state: &State);
}
