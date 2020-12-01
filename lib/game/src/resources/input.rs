use crate::UniqueViewMut;
use crate::World;
use winit::event::*;

#[derive(Debug, Default)]
pub struct Input {
  pub action: bool,
  pub up: bool,
  pub down: bool,
  pub left: bool,
  pub right: bool,
}

pub fn handle_keyboard_input(world: &World, event: &WindowEvent) {
  if let WindowEvent::KeyboardInput {
    input: KeyboardInput {
      virtual_keycode: Some(key),
      state,
      ..
    },
    ..
  } = event
  {
    keyboard_input(world, *key, *state)
  }
}

fn keyboard_input(world: &World, key: VirtualKeyCode, state: ElementState) {
  #[allow(clippy::single_match)]
  match key {
    VirtualKeyCode::Space => action_input(world, state),
    VirtualKeyCode::Up => action_up(world, state),
    _ => (),
  }
}

fn action_input(world: &World, state: ElementState) {
  println!("Action with pressed status: {:?}", state);
  let mut input = world.borrow::<UniqueViewMut<Input>>();

  match state {
    ElementState::Pressed => input.action = true,
    ElementState::Released => input.action = false,
  }
}

fn action_up(world: &World, state: ElementState) {
  println!("Action UP with pressed status: {:?}", state);
  let mut input = world.borrow::<UniqueViewMut<Input>>();

  match state {
    ElementState::Pressed => input.up = true,
    ElementState::Released => input.up = false,
  }
}
