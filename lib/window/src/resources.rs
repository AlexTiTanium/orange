use crate::api::translate_cursor;
use crate::api::CursorIcon;
use winit::window::Window;

pub struct WindowContext {
  pub(crate) window: Window,
}

///
/// Context wrapper for window module
///
impl WindowContext {
  ///
  /// Return window ref
  ///
  pub fn window(&self) -> &Window {
    &self.window
  }

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
  /// Get physical size
  ///
  pub fn physical_size(&self) -> crate::api::PhysicalSize<u32> {
    let size = self.window.inner_size();

    crate::api::PhysicalSize {
      width: size.width,
      height: size.height,
    }
  }

  ///
  /// Get logical size
  ///
  pub fn logical_size(&self) -> crate::api::LogicalSize<u32> {
    let scale_factor = self.window.scale_factor();
    let size = self.window.inner_size().to_logical(scale_factor);

    crate::api::LogicalSize {
      width: size.width,
      height: size.height,
    }
  }

  ///
  /// Get scale factor
  ///
  pub fn scale_factor(&self) -> f64 {
    return self.window.scale_factor();
  }
}
