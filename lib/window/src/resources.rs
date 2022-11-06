use crate::convertors::translate_cursor;
use crate::cursor::CursorIcon;
use winit::dpi::LogicalSize;
use winit::dpi::PhysicalSize;
use winit::window::Window;

#[derive(Default)]
pub struct WindowSize {
  pub logical: LogicalSize<f32>,
  pub physical: PhysicalSize<f32>,
  pub scale: f64,
}

pub struct WindowContext {
  pub(crate) window: Window,
}

///
/// Context wrapper for window module
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

  ///
  /// Return window ref
  ///
  pub fn window(&self) -> &Window {
    &self.window
  }

  ///
  /// Get physical size
  ///
  pub fn physical_size(&self) -> crate::dpi::PhysicalSize<u32> {
    let size = self.window.inner_size();

    crate::dpi::PhysicalSize {
      width: size.width,
      height: size.height,
    }
  }
}
