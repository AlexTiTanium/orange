use crate::{
  api::cursor::CursorIcon,
  events::{PointerButton, WindowInnerEvent, WindowInputEvent},
};
use winit::event::{ElementState, MouseButton, WindowEvent};

///
/// Convert winit input events to module WindowInputEvent
///
pub fn translate_input(event: &WindowEvent) -> Option<WindowInputEvent> {
  let result = match event {
    // Cursor move event
    WindowEvent::CursorMoved { position, .. } => WindowInputEvent::PointerMoved(position.x, position.y),

    // Mouse button pressed
    WindowEvent::MouseInput { state, button, .. } => WindowInputEvent::PointerButton {
      button: match button {
        MouseButton::Left => PointerButton::Primary,
        MouseButton::Right => PointerButton::Secondary,
        MouseButton::Middle => PointerButton::Secondary,
        _ => PointerButton::Extra1,
      },
      pressed: match state {
        ElementState::Pressed => true,
        ElementState::Released => false,
      },
    },
    // Ignore anything else
    _ => WindowInputEvent::None,
  };

  if result == WindowInputEvent::None {
    return None;
  } else {
    return Some(result);
  }
}

///
/// Convert winit window events to module WindowEvent
///
pub fn translate_window_events(event: &WindowEvent) -> Option<WindowInnerEvent> {
  let result = match event {
    // Window resize event
    WindowEvent::Resized(size) => WindowInnerEvent::Resized(size.width, size.height),

    // Window scale factor event
    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
      WindowInnerEvent::ScaleFactorChange(new_inner_size.width, new_inner_size.height)
    }

    // Ignore anything else
    _ => WindowInnerEvent::None,
  };

  if result == WindowInnerEvent::None {
    return None;
  } else {
    return Some(result);
  }
}

///
/// Convert cursor icons to winit cursor icons
///
pub fn translate_cursor(cursor_icon: CursorIcon) -> Option<winit::window::CursorIcon> {
  match cursor_icon {
    CursorIcon::None => None,

    CursorIcon::Alias => Some(winit::window::CursorIcon::Alias),
    CursorIcon::AllScroll => Some(winit::window::CursorIcon::AllScroll),
    CursorIcon::Cell => Some(winit::window::CursorIcon::Cell),
    CursorIcon::ContextMenu => Some(winit::window::CursorIcon::ContextMenu),
    CursorIcon::Copy => Some(winit::window::CursorIcon::Copy),
    CursorIcon::Crosshair => Some(winit::window::CursorIcon::Crosshair),
    CursorIcon::Default => Some(winit::window::CursorIcon::Default),
    CursorIcon::Grab => Some(winit::window::CursorIcon::Grab),
    CursorIcon::Grabbing => Some(winit::window::CursorIcon::Grabbing),
    CursorIcon::Help => Some(winit::window::CursorIcon::Help),
    CursorIcon::Move => Some(winit::window::CursorIcon::Move),
    CursorIcon::NoDrop => Some(winit::window::CursorIcon::NoDrop),
    CursorIcon::NotAllowed => Some(winit::window::CursorIcon::NotAllowed),
    CursorIcon::PointingHand => Some(winit::window::CursorIcon::Hand),
    CursorIcon::Progress => Some(winit::window::CursorIcon::Progress),

    CursorIcon::ResizeHorizontal => Some(winit::window::CursorIcon::EwResize),
    CursorIcon::ResizeNeSw => Some(winit::window::CursorIcon::NeswResize),
    CursorIcon::ResizeNwSe => Some(winit::window::CursorIcon::NwseResize),
    CursorIcon::ResizeVertical => Some(winit::window::CursorIcon::NsResize),

    CursorIcon::ResizeEast => Some(winit::window::CursorIcon::EResize),
    CursorIcon::ResizeSouthEast => Some(winit::window::CursorIcon::SeResize),
    CursorIcon::ResizeSouth => Some(winit::window::CursorIcon::SResize),
    CursorIcon::ResizeSouthWest => Some(winit::window::CursorIcon::SwResize),
    CursorIcon::ResizeWest => Some(winit::window::CursorIcon::WResize),
    CursorIcon::ResizeNorthWest => Some(winit::window::CursorIcon::NwResize),
    CursorIcon::ResizeNorth => Some(winit::window::CursorIcon::NResize),
    CursorIcon::ResizeNorthEast => Some(winit::window::CursorIcon::NeResize),
    CursorIcon::ResizeColumn => Some(winit::window::CursorIcon::ColResize),
    CursorIcon::ResizeRow => Some(winit::window::CursorIcon::RowResize),

    CursorIcon::Text => Some(winit::window::CursorIcon::Text),
    CursorIcon::VerticalText => Some(winit::window::CursorIcon::VerticalText),
    CursorIcon::Wait => Some(winit::window::CursorIcon::Wait),
    CursorIcon::ZoomIn => Some(winit::window::CursorIcon::ZoomIn),
    CursorIcon::ZoomOut => Some(winit::window::CursorIcon::ZoomOut),
  }
}
