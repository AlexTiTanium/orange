use common::Application;
use window::WindowPlugin;

// mod components;
// mod level;

// use std::rc::Rc;

// use diagnostic;

// use editor::Editor;
// use flexi_logger::Logger;
// use game::{self, State};
// use glutin::ContextBuilder;
// use render;
// use winit::dpi::LogicalSize;
// use winit::event::{Event, WindowEvent};
// use winit::event_loop::ControlFlow;
// use winit::window::WindowBuilder;

fn main() {
  Application::build().add_plugin(WindowPlugin).run();

  // // Register logger
  // logger::init();

  // // Create game state and init component system
  // let state = Rc::new(State::new());

  // Init modules
  // let event_loop = window::create(&state);

  // //diagnostic::init(&state);
  // game::init(&state);
  // render::init(&state);

  //editor::init(&state);

  // Game event loop
  // event_loop.run(move |event, _, control_flow| {
  //   match event {
  //     Event::LoopDestroyed => {}
  //     Event::WindowEvent { event, .. } => {

  //     }
  //     Event::NewEvents(_) => {
  //       //state.update_time();
  //       //editor.update();
  //     }
  //     Event::MainEventsCleared => {
  //       //state.run_workload(game::stage::PRE_RENDER);
  //     }
  //     Event::RedrawRequested(_) => {
  //       //state.run_workload(game::stage::RENDER);
  //       //state.run_workload(game::stage::POST_RENDER);
  //       //render::step(&state);
  //       //editor.step(&state, &context.window());
  //       //context.swap_buffers().unwrap();
  //     }
  //     Event::RedrawEventsCleared => {
  //       //state.update_fps();
  //     }
  //     _ => (),
  //   }
  // });

  // // Start game
  // //log::info!("Game start");
  // level::load(&state, "maps/level_3.tmx", vec!["textures/winter.xml"]);
  // // Load textures on GPU
  // render::load_textures(&state);

  // // Game event loop
  // event_loop.run(move |event, _, control_flow| {
  //   // Listen user input for editor UI
  //   editor.handle_event(&context.window(), &event);

  //   match event {
  //     Event::LoopDestroyed => {}
  //     Event::WindowEvent { event, .. } => {
  //       match event {
  //         WindowEvent::Resized(new_size) => context.resize(new_size),
  //         WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
  //         _ => (),
  //       }

  //       //state.handle_window_events(&event);
  //     }
  //     Event::NewEvents(_) => {
  //       state.update_time();
  //       editor.update();
  //     }
  //     Event::MainEventsCleared => {
  //       context.window().request_redraw();
  //     }
  //     Event::RedrawRequested(_) => {
  //       render::step(&state);
  //       editor.step(&state, &context.window());
  //       context.swap_buffers().unwrap();
  //     }
  //     Event::RedrawEventsCleared => {
  //       //state.update_fps();
  //     }
  //     _ => (),
  //   }
  // });
}
