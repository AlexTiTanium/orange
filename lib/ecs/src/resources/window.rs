use crate::*;
use log::*;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;
use winit::event::*;

pub fn handle_window_resize(world: &World, event: &WindowEvent) {
  if let WindowEvent::Resized(new_size) = event {
    let mut window = world.borrow::<Unique<&mut Window>>();
    window.physical.width = new_size.width as f32;
    window.physical.height = new_size.height as f32;
  }

  if let WindowEvent::ScaleFactorChanged {
    scale_factor,
    new_inner_size,
  } = event
  {
    let mut window = world.borrow::<Unique<&mut Window>>();
    window.physical.width = new_inner_size.width as f32;
    window.physical.height = new_inner_size.height as f32;
    window.scale = scale_factor.clone();
  }
}

pub fn update_window_sizes(world: &World, logical: LogicalSize<f32>, physical: PhysicalSize<f32>, scale: f64) {
  let mut window = world.borrow::<Unique<&mut Window>>();
  window.logical = logical;
  window.physical = physical;
  window.scale = scale;
}

pub struct Window {
  pub logical: LogicalSize<f32>,
  pub physical: PhysicalSize<f32>,
  pub scale: f64,
}

impl Default for Window {
  fn default() -> Self {
    Self {
      logical: LogicalSize { width: 0.0, height: 0.0 },
      physical: PhysicalSize { width: 0.0, height: 0.0 },
      scale: 0.0,
    }
  }
}
