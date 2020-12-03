mod state;

/// Game state
pub use state::{Stage, State};

// Game public API
pub use state::create_state;

pub mod components;
pub mod resources;
pub mod systems;

pub(crate) use nalgebra_glm as glm;
pub use shipyard::*;
