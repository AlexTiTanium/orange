use crate::Reducer;

pub struct Store<State, Action> {
  reducer: Reducer<State, Action>,
  state: State,
}

impl<State, Action> Store<State, Action> {
  pub fn new(reducer: Reducer<State, Action>, initial: State) -> Self {
    Self {
      reducer,
      state: initial,
    }
  }

  pub fn state(&self) -> &State {
    &self.state
  }

  pub fn dispatch(&mut self, action: Action) {
    self.dispatch_reducer(&action);
  }

  fn dispatch_reducer(&mut self, action: &Action) {
    self.state = (&self.reducer)(self.state(), action);
  }
}
