pub mod display;
pub mod input;
pub mod time;

pub use display::Display;
pub use input::handle_keyboard_input;
pub use input::Input;
pub use time::Time;

use crate::glm::*;

#[derive(Debug)]
pub struct Size {
  pub width: u32,
  pub height: u32,
}

#[derive(Debug)]
pub struct Position(pub Vec3);
