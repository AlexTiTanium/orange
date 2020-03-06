mod store;
use store::Store as ReduxStore;
mod game;
pub use game::Game;
mod reducer;
use reducer::Reducer;

pub type Store = ReduxStore<Game, Action>;

pub fn create_store() -> Store {
    return Store::new(Game::reducer, Game::default());
}

pub enum Action {
    Increment,
    Decrement,
}
