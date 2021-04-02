// Private
mod plugin;
mod resources;
mod runner;
mod systems;

use glutin::{dpi::PhysicalSize, ContextWrapper, PossiblyCurrent};
use winit::window::Window;
type WindowContext = ContextWrapper<PossiblyCurrent, Window>;

// Public
pub use plugin::WindowPlugin;

pub struct WindowResizeEvent(PhysicalSize<u32>);

// /// Module initialization
// fn init(state: &Rc<State>) {
//   // Register resources
//   state.add_resource(WindowSize::default());

//   state.pre_render.borrow_mut().with_system(system!(systems::request_redraw));
//   state.post_render.borrow_mut().with_system(system!(systems::swap));
// }

// // Create system window
// pub fn create(state: &Rc<State>) -> EventLoop<()> {
//   // Init resources
//   init(state);

//   // Build window and get event loop
//   let event_loop = systems::build_window(state);

//   // Update window size
//   state.world.run(systems::update_window_size);

//   // Return event loop for main
//   return event_loop;
// }
