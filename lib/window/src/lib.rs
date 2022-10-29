// Private
pub mod events;
mod plugin;
mod resources;
mod runner;
mod systems;

use glutin::{ContextWrapper, PossiblyCurrent};
use winit::window::Window;

// Public
pub use plugin::WindowPlugin;
pub use resources::WindowSize;

// Types
pub type WindowContext = ContextWrapper<PossiblyCurrent, Window>;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WindowPhysicalSize<P>(P, P);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WindowPhysicalPosition<P>(P, P);
