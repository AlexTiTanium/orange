use game::State;
use glutin::ContextBuilder;
use logger::log;

use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::WindowContext;

pub fn build_window(state: &State) -> EventLoop<()> {
  log::info!("Building window...");

  // Create event loop
  let event_loop = winit::event_loop::EventLoop::new();

  // Create winit window
  let wb = WindowBuilder::new()
    .with_title("Hello world!")
    .with_inner_size(LogicalSize::new(1024, 768));

  // Create windowed context
  let windowed_context = ContextBuilder::new()
    .with_srgb(true)
    .with_vsync(true)
    .build_windowed(wb, &event_loop)
    .unwrap();

  // It is essential to make the context current before calling `gl::load_with`.
  let context: WindowContext = unsafe { windowed_context.make_current().unwrap() };

  // Move context and event loop to window resource
  state.add_none_sync_resource(context);

  // return event loop
  return event_loop;
}
