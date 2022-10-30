use crate::convertors::translate_cursor;
use crate::cursor::CursorIcon;
use glutin::window::Window;
use glutin::ContextWrapper;
use glutin::PossiblyCurrent;
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

pub struct WindowContext {
  pub(crate) wraper: ContextWrapper<PossiblyCurrent, Window>,
}

///
/// Context wraper for window module
///
impl WindowContext {
  ///
  /// Change cursor
  ///
  pub fn set_cursor(&self, cursor: CursorIcon) {
    let icon = translate_cursor(cursor);

    match icon {
      Some(icon) => self.window().set_cursor_icon(icon),
      None => self.window().set_cursor_icon(glutin::window::CursorIcon::Default),
    }
  }

  ///
  /// Get window
  ///
  pub fn window(&self) -> &Window {
    self.wraper.window()
  }

  ///
  /// Resize context
  ///
  pub fn resize(&self, width: &u32, height: &u32) {
    self.wraper.resize(PhysicalSize {
      width: *width,
      height: *height,
    })
  }

  ///
  /// Switch frame buffer
  ///
  pub fn swap_buffers(&self) {
    self.wraper.swap_buffers().unwrap();
  }

  ///
  /// Returns the address of an OpenGL function.
  ///
  pub fn get_proc_address(&self, addr: &str) -> *const core::ffi::c_void {
    self.wraper.get_proc_address(addr)
  }
}
