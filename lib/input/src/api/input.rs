use super::{
  common::{InputEvent, InputPosition},
  pointer::PointerCursor,
};

#[derive(Default, Debug)]
pub struct Input {
  pub events: Vec<InputEvent>,
  pub pointer: PointerCursor,
}

impl Input {
  ///
  /// Clear all temrary input events
  ///
  pub fn clear(&mut self) {
    self.events.clear();
    self.pointer.clear();
  }

  ///
  /// Process input event
  ///
  pub fn push_event(&mut self, event: InputEvent) {
    match event {
      // InputEvent::Copy => todo!(),
      // InputEvent::Cut => todo!(),
      // InputEvent::Paste(_) => todo!(),
      // InputEvent::Text(_) => todo!(),
      // InputEvent::Key { key, pressed, modifiers } => todo!(),
      InputEvent::PointerMoved(InputPosition(x, y)) => self.pointer.update(x, y),
      // InputEvent::PointerButton {
      //   pos,
      //   button,
      //   pressed,
      //   modifiers,
      // } => todo!(),
      // InputEvent::PointerGone => todo!(),
      // InputEvent::Scroll(_) => todo!(),
      // InputEvent::Zoom(_) => todo!(),
      // InputEvent::CompositionStart => todo!(),
      // InputEvent::CompositionUpdate(_) => todo!(),
      // InputEvent::CompositionEnd(_) => todo!(),
      _ => {}
    }

    self.events.push(event);
  }
}
