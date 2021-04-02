use common::{events::Events, log::info, stage, Application, UniqueViewMut};
use glutin::{
  event::{Event, WindowEvent},
  event_loop::ControlFlow,
  ContextBuilder,
};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::WindowContext;
use crate::WindowResizeEvent;

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
      Event::WindowEvent { event, .. } => {
        match event {
          WindowEvent::Resized(new_size) => {
            app.send(WindowResizeEvent(new_size));
          }
          WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
            app.exit();
          }
          _ => (),
        }

        //state.handle_window_events(&event);
      }
      Event::LoopDestroyed => {}
      Event::NewEvents(_) => {
        //state.update_time();
        //editor.update();
        app.run_stage(stage::PRE_UPDATE)
      }
      Event::MainEventsCleared => app.run_stage(stage::PRE_RENDER),
      Event::RedrawEventsCleared => {
        //state.update_fps();
        app.update();
      }
      Event::RedrawRequested(_) => {
        //state.run_workload(game::stage::RENDER);
        //state.run_workload(game::stage::POST_RENDER);
        //render::step(&state);
        //editor.step(&state, &context.window());
        app.run_stage(stage::POST_RENDER);
      }
      _ => (),
    }
  });
}
