// Private
mod convertors;
mod cursor;
mod plugin;
mod resources;
mod runner;
mod systems;

// Public
pub mod events;
pub use cursor::CursorIcon;
pub use plugin::WindowPlugin;
pub use resources::WindowContext;
pub use resources::WindowSize;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WindowPhysicalSize<P>(P, P);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WindowPhysicalPosition<P>(P, P);
