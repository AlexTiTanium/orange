use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;

pub struct WindowSize {
  pub logical: LogicalSize<f32>,
  pub physical: PhysicalSize<f32>,
  pub scale: f64,
}

impl Default for WindowSize {
  fn default() -> Self {
    Self {
      logical: LogicalSize { width: 0.0, height: 0.0 },
      physical: PhysicalSize { width: 0.0, height: 0.0 },
      scale: 0.0,
    }
  }
}
