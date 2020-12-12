use std::rc::Rc;

use game::{events::GameEvent, State};
use logger::log;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run_game_loop(state: &Rc<State>, event_loop: EventLoop<()>) {
  log::info!("Start game loop...");

  let state = Rc::clone(state);

  // Game event loop
  event_loop.run(move |event, _, control_flow| {
    // Listen user input for editor UI
    //editor.handle_event(&context.window(), &event);

    match event {
      Event::LoopDestroyed => {}
      Event::WindowEvent { event, .. } => {
        match event {
          WindowEvent::Resized(new_size) => {
            state.send(GameEvent::WindowResize(new_size.width, new_size.height));
          }
          WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
          _ => (),
        }

        //state.handle_window_events(&event);
      }
      Event::NewEvents(_) => {
        //state.update_time();
        //editor.update();
      }
      Event::MainEventsCleared => {
        state.run_workload(game::stage::PRE_RENDER);
      }
      Event::RedrawRequested(_) => {
        state.run_workload(game::stage::RENDER);
        state.run_workload(game::stage::POST_RENDER);
        //render::step(&state);
        //editor.step(&state, &context.window());
        //context.swap_buffers().unwrap();
      }
      Event::RedrawEventsCleared => {
        //state.update_fps();
      }
      _ => (),
    }
  });
}
