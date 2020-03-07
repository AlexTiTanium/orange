mod reducer;
mod state;
mod store;
mod window;

use reducer::Reducer;
use state::State;
use store::Store as ReduxStore;
use window::Window;

pub type Store = ReduxStore<State, Action>;

pub fn create_store() -> Store {
    let window = Window::reducer;

    let reducer = combine_reducers!(State, &Action, window);
    return Store::new(reducer, State::default());
}

pub enum Action {
    WindowResize(u32, u32),
}
