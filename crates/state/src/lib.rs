use redux::Store as ReduxStore;
pub type Store = ReduxStore<Game, Action>;

mod game;
pub use game::Game;

pub fn create_store() -> Store {
    return Store::new(Game::reducer, Game::default());
}

pub enum Action {
    Increment,
    Decrement,
}
