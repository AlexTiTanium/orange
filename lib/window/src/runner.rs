use crate::convertors::{translate_input, translate_window_events};
use crate::WindowContext;
use common::{log::info, stage, Application};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::{
  event::{Event, WindowEvent},
  event_loop::ControlFlow,
};

///
/// Window event loop
///
pub fn window_runner(mut app: Application) {
  info!("Window runner start");

  // Create event loop
  let event_loop = EventLoop::new();

  // Create winit window
  let window = WindowBuilder::new()
    .with_title("Hello world!")
    .with_inner_size(LogicalSize::new(1024, 768))
    .build(&event_loop)
    .unwrap();

  // It is essential to make the context current before calling `gl::load_with`.
  // let wraper = unsafe { windowed_context.make_current().unwrap() };
  let context = WindowContext { window };

  // Move context and event loop to window resource
  app.world.add_unique(context).unwrap();

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
        _ => {
          // Window events like resize and change scale factor
          match translate_window_events(&event) {
            Some(window_event) => app.send(window_event),
            None => (),
          };

          // Window input
          match translate_input(&event) {
            Some(window_input) => app.send(window_input),
            None => (),
          };
        }
      },
      Event::LoopDestroyed => {}
      Event::NewEvents(_) => app.run_stage(stage::PRE_UPDATE),
      Event::MainEventsCleared => {
        app.update();
        app.run_stage(stage::PRE_RENDER);
        app.run_stage(stage::RENDER);
        app.run_stage(stage::POST_RENDER);
      }
      _ => (),
    }
  });
}
