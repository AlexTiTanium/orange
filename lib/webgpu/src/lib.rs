// Private
mod pipeline;
mod plugin;
mod state;

// Public
pub use pipeline::{Pipeline, Pipelines};
pub use plugin::WebGpuPlugin;
pub use state::WebGpuState;
pub use wgpu as api;
