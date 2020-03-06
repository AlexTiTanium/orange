use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

pub fn create_window() {
  let el = EventLoop::new();

  let wb = WindowBuilder::new()
    .with_title("Hello world!")
    .with_inner_size(LogicalSize::new(1024.0, 768.0));

  let _windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

  el.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::LoopDestroyed => return,
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Resized(_physical_size) => {
          // windowed_context.resize(physical_size)
        }
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        _ => (),
      },
      Event::RedrawRequested(_) => {
        // gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
        // windowed_context.swap_buffers().unwrap();
      }
      _ => (),
    }
  });
}
