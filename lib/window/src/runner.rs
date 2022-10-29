use crate::events::WindowInputEvent;
use crate::WindowContext;
use common::{log::info, stage, Application};
use glutin::{
  event::{Event, WindowEvent},
  event_loop::ControlFlow,
  ContextBuilder,
};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

///
/// Window event loop
///
pub fn window_runner(mut app: Application) {
  info!("Window runner start");

  // Create event loop
  let event_loop = EventLoop::new();

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
  app.world.add_unique_non_send_sync(context).unwrap();

  // Prepare app for start
  app.initialize();

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Poll;

    match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          *control_flow = ControlFlow::Exit;
          app.exit();
        }
        WindowEvent::Resized(size) => {
          app.send(WindowInputEvent::Resized(size.width, size.height));
        }

        WindowEvent::CursorMoved { position, .. } => {
          app.send(WindowInputEvent::PointerMoved(position.x, position.y));
        }
        _ => (),
      },
      Event::LoopDestroyed => {}
      Event::NewEvents(_) => app.run_stage(stage::PRE_UPDATE),
      Event::MainEventsCleared => app.run_stage(stage::PRE_RENDER),
      Event::RedrawEventsCleared => {
        app.update();
      }
      Event::RedrawRequested(_) => {
        app.run_stage(stage::RENDER);
        app.run_stage(stage::POST_RENDER);
      }
      _ => (),
    }
  });
}
