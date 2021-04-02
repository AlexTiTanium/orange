// Private
mod plugin;
mod resources;
mod runner;
mod systems;

use glutin::{dpi::PhysicalSize, ContextWrapper, PossiblyCurrent};
use winit::window::Window;

// Public
pub use plugin::WindowPlugin;

// Events
pub struct WindowResizeEvent(PhysicalSize<u32>);

// Types
pub type WindowContext = ContextWrapper<PossiblyCurrent, Window>;
