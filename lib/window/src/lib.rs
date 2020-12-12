pub mod resources;
mod systems;

use game::{events::GameEvent, system, State};
use glutin::{ContextWrapper, PossiblyCurrent};
use resources::WindowSize;
use winit::event_loop::EventLoop;
use winit::window::Window;

use std::rc::Rc;

pub type WindowContext = ContextWrapper<PossiblyCurrent, Window>;

/// Module initialization
fn init(state: &Rc<State>) {
  // Register resources
  state.add_resource(WindowSize::default());

  // Listen resize event
  let ref_state = Rc::clone(&state);
  state.listen(Box::new(move |event| match event {
    GameEvent::WindowResize(width, height) => {
      ref_state.world.run_with_data(systems::window_resize, (*width, *height));
      ref_state.world.run(systems::update_window_size);
    }
  }));

  state.pre_render.borrow_mut().with_system(system!(systems::request_redraw));
  state.post_render.borrow_mut().with_system(system!(systems::swap));
}

// Create system window
pub fn build(state: &Rc<State>) -> EventLoop<()> {
  // Init resources
  init(state);

  // Build window and get event loop
  let event_loop = systems::build_window(state);

  // Update window size
  state.world.run(systems::update_window_size);

  // Return event loop for main
  return event_loop;
}

/// Run game loop
pub fn run(state: &Rc<State>, event_loop: EventLoop<()>) {
  systems::run_game_loop(state, event_loop);
}
