pub fn translate_cursor(cursor_icon: egui::CursorIcon) -> Option<window::CursorIcon> {
  match cursor_icon {
    egui::CursorIcon::None => None,

    egui::CursorIcon::Alias => Some(window::CursorIcon::Alias),
    egui::CursorIcon::AllScroll => Some(window::CursorIcon::AllScroll),
    egui::CursorIcon::Cell => Some(window::CursorIcon::Cell),
    egui::CursorIcon::ContextMenu => Some(window::CursorIcon::ContextMenu),
    egui::CursorIcon::Copy => Some(window::CursorIcon::Copy),
    egui::CursorIcon::Crosshair => Some(window::CursorIcon::Crosshair),
    egui::CursorIcon::Default => Some(window::CursorIcon::Default),
    egui::CursorIcon::Grab => Some(window::CursorIcon::Grab),
    egui::CursorIcon::Grabbing => Some(window::CursorIcon::Grabbing),
    egui::CursorIcon::Help => Some(window::CursorIcon::Help),
    egui::CursorIcon::Move => Some(window::CursorIcon::Move),
    egui::CursorIcon::NoDrop => Some(window::CursorIcon::NoDrop),
    egui::CursorIcon::NotAllowed => Some(window::CursorIcon::NotAllowed),
    egui::CursorIcon::PointingHand => Some(window::CursorIcon::PointingHand),
    egui::CursorIcon::Progress => Some(window::CursorIcon::Progress),

    egui::CursorIcon::ResizeHorizontal => Some(window::CursorIcon::ResizeHorizontal),
    egui::CursorIcon::ResizeNeSw => Some(window::CursorIcon::ResizeNeSw),
    egui::CursorIcon::ResizeNwSe => Some(window::CursorIcon::ResizeNwSe),
    egui::CursorIcon::ResizeVertical => Some(window::CursorIcon::ResizeVertical),

    egui::CursorIcon::ResizeEast => Some(window::CursorIcon::ResizeEast),
    egui::CursorIcon::ResizeSouthEast => Some(window::CursorIcon::ResizeSouthEast),
    egui::CursorIcon::ResizeSouth => Some(window::CursorIcon::ResizeSouth),
    egui::CursorIcon::ResizeSouthWest => Some(window::CursorIcon::ResizeSouthWest),
    egui::CursorIcon::ResizeWest => Some(window::CursorIcon::ResizeWest),
    egui::CursorIcon::ResizeNorthWest => Some(window::CursorIcon::ResizeNorthWest),
    egui::CursorIcon::ResizeNorth => Some(window::CursorIcon::ResizeNorth),
    egui::CursorIcon::ResizeNorthEast => Some(window::CursorIcon::ResizeNorthEast),
    egui::CursorIcon::ResizeColumn => Some(window::CursorIcon::ResizeColumn),
    egui::CursorIcon::ResizeRow => Some(window::CursorIcon::ResizeRow),

    egui::CursorIcon::Text => Some(window::CursorIcon::Text),
    egui::CursorIcon::VerticalText => Some(window::CursorIcon::VerticalText),
    egui::CursorIcon::Wait => Some(window::CursorIcon::Wait),
    egui::CursorIcon::ZoomIn => Some(window::CursorIcon::ZoomIn),
    egui::CursorIcon::ZoomOut => Some(window::CursorIcon::ZoomOut),
  }
}
