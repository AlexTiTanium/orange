use crate::*;
use winit::event::*;

pub fn handle_window_resize(world: &World, event: &WindowEvent) {
  if let WindowEvent::Resized(new_size) = event {
    let mut display = world.borrow::<Unique<&mut Window>>();
    display.resize(new_size.width, new_size.height);
  }
}

pub struct Window {
  pub target_width: u32,
  pub target_height: u32,
  pub width: u32,
  pub height: u32,
}

impl Default for Window {
  fn default() -> Self {
    let target_width = 1024;
    let target_height = 768;

    Self {
      target_width,
      target_height,
      width: target_width,
      height: target_height,
    }
  }
}

impl Window {
  pub fn resize(&mut self, width: u32, height: u32) {
    println!("resize");
    self.width = width;
    self.height = height;
  }
}
