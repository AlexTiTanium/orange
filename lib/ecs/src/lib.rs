mod state;
pub use state::State;

pub mod components;
pub mod entities;

pub use components as component;
pub use entities as entity;

pub use shipyard::prelude::*;

pub fn create_state() -> State {
    let state = State::new();

    state.create_entities();
    state.create_resources();

    state
}
