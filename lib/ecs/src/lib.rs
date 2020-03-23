pub mod entities;
mod state;

use shipyard::prelude::*;
pub use state::State;

pub fn create_state() -> State {
    let state = State::new();

    state.create_entities();
    state.create_resources();

    state
}
