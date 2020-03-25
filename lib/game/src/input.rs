use glutin::event::*;

pub fn handle_keyboard_input(event: &WindowEvent) {
  if let WindowEvent::KeyboardInput {
    input: KeyboardInput {
      virtual_keycode: Some(key),
      state,
      ..
    },
    ..
  } = event
  {
    keyboard_input(*key, *state)
  }
}

fn keyboard_input(key: VirtualKeyCode, state: ElementState) {
  #[allow(clippy::single_match)]
  match key {
    VirtualKeyCode::Space => action_input(state),
    VirtualKeyCode::Up => action_up(state),
    _ => (),
  }
}

fn action_input(state: ElementState) {
  println!("Action with pressed status: {:?}", state);
}

fn action_up(state: ElementState) {
  println!("Action UP with pressed status: {:?}", state);
}
