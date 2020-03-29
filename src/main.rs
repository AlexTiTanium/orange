use editor::Editor;
use flexi_logger::Logger;
use glutin::ContextBuilder;
use render;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::WindowBuilder;

fn main() {
    // Setup logger
    Logger::with_str("debug").start_with_specfile("./logspec.toml").unwrap();

    // Create entities component system
    let state = ecs::create_state();

    // Start event loop
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
    let context = unsafe { windowed_context.make_current().unwrap() };

    // Game render
    let mut render = render::create(&state, |symbol| context.get_proc_address(symbol));

    // Create editor UI render
    let mut editor = Editor::new(&context.window(), |symbol| context.get_proc_address(symbol));

    // Game event loop
    event_loop.run(move |event, _, control_flow| {
        // Listen user input for editor UI
        editor.handle_event(&context.window(), &event);

        match event {
            Event::LoopDestroyed => {}
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::Resized(new_size) => context.resize(new_size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                }

                state.handle_window_events(&event);
            }
            Event::NewEvents(_) => {
                state.update_time();
                editor.update();
            }
            Event::MainEventsCleared => {
                context.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                render.step(&state);
                editor.step(&state, &context.window());
                context.swap_buffers().unwrap();
            }
            Event::RedrawEventsCleared => {
                state.update_fps();
            }
            _ => (),
        }
    });
}
