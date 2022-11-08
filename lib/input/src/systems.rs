use crate::api::common::{InputEvent, InputPosition, PointerButton};
use crate::resources::Input;
use common::{events::Events, UniqueView, UniqueViewMut};
use window::events::WindowInputEvent;
use window::WindowContext;

// Cursor moved events
type WindowInputEventView<'a> = UniqueView<'a, Events<WindowInputEvent>>;

//
// Process window input
//
pub fn process_window_input(
  mut input: UniqueViewMut<Input>,
  window_input: WindowInputEventView,
  window: UniqueView<WindowContext>,
) {
  let events = window_input.get_reader().iter(&window_input);
  let scale_factor = window.scale_factor();

  for event in events {
    match event {
      WindowInputEvent::Copy => todo!(),
      WindowInputEvent::Cut => todo!(),
      WindowInputEvent::Paste(_) => todo!(),
      WindowInputEvent::Text(_) => todo!(),
      WindowInputEvent::PointerMoved(x, y) => {
        input.push_event(InputEvent::PointerMoved(InputPosition(
          *x / scale_factor,
          *y / scale_factor,
        )));
      }
      WindowInputEvent::PointerButton { button, pressed } => {
        let pos = input.pointer.now.clone();

        input.push_event(InputEvent::PointerButton {
          pos,
          button: match button {
            window::events::PointerButton::Primary => PointerButton::Primary,
            window::events::PointerButton::Secondary => PointerButton::Secondary,
            window::events::PointerButton::Middle => PointerButton::Middle,
            window::events::PointerButton::Extra1 => PointerButton::Extra1,
            window::events::PointerButton::Extra2 => PointerButton::Extra2,
          },
          pressed: *pressed,
        })
      }
      WindowInputEvent::PointerGone => todo!(),
      WindowInputEvent::Scroll(_) => todo!(),
      WindowInputEvent::Zoom(_) => todo!(),
      WindowInputEvent::None => todo!(),
    }
  }
}

//
// Clear input
//
pub fn clear_window_input(mut input: UniqueViewMut<Input>) {
  input.clear();
}
