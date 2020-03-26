mod state;
pub use state::State;

pub(crate) use nalgebra_glm as glm;

pub mod components;
pub mod systems;

pub use shipyard::prelude::*;

pub fn create_state() -> State {
    let state = State::new();

    state.create_entities();
    state.create_resources();

    state
}
