mod base;
mod state;
mod window;

use base::reducer::Reducer;

use state::State;
use window::Window;

pub type Store = base::store::Store<State, Action>;

pub fn create_store() -> Store {
    let window = Window::reducer;
    let reducer = combine_reducers!(State, &Action, window);
    Store::new(reducer, State::default())
}

pub enum Action {
    WindowResize(u32, u32),
}
