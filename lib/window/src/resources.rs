use crate::convertors::translate_cursor;
use crate::cursor::CursorIcon;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;
use winit::window::Window;

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
  pub(crate) window: Window,
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
      Some(icon) => self.window.set_cursor_icon(icon),
      None => self.window.set_cursor_icon(winit::window::CursorIcon::Default),
    }
  }
}
