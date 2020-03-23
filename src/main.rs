use editor::Editor;
use flexi_logger::Logger;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use render;
use std::time::Instant;

fn main() {
    //Default window size
    let window_size = (800, 600);

    // Setup logger
    Logger::with_str("debug").start_with_specfile("./logspec.toml").unwrap();

    // Create entities component system
    let state = ecs::create_state();

    // Set default resolution
    state.update_display(window_size.0, window_size.1);

    // Load assets
    // store.assets.load("cat", "cat_big.png");
    // store.assets.load("tree", "tree.png");

    // Start event loop
    let event_loop = glutin::event_loop::EventLoop::new();

    // Create winit window
    let wb = WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(LogicalSize::new(window_size.0, window_size.1));

    // Create windowed context
    let windowed_context = ContextBuilder::new().with_vsync(true).build_windowed(wb, &event_loop).unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let window = unsafe { windowed_context.make_current().unwrap() };

    // Game render
    let mut render = render::create(&state, |symbol| window.get_proc_address(symbol));

    // Create editor UI render
    let mut editor = Editor::new(&window.window(), |symbol| window.get_proc_address(symbol));

    // Store monotonic clock time since start
    let time = Instant::now();

    // Delta time
    let mut delta = Instant::now();

    // Game event loop
    event_loop.run(move |event, _, control_flow| {
        // Poll should work better for games
        *control_flow = ControlFlow::Poll;

        // Listen user input for editor UI
        editor.handle_event(&window.window(), &event);

        match event {
            Event::NewEvents(_) => {
                editor.update();
            }
            Event::LoopDestroyed => {}
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(new_size) => {
                    state.update_display(new_size.width, new_size.height);
                    window.resize(new_size)
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                //println!("[Game] Elapsed time ms: {:?}", time.elapsed().as_millis());
                //println!("[Game] Delta time ms: {:?}", Instant::now().duration_since(delta).as_millis());
                editor.prepare_frame(&window.window());
                window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                render.step();
                editor.step(&state, &window.window());
                window.swap_buffers().unwrap();
            }
            Event::RedrawEventsCleared => {
                //delta = Instant::now();
            }
            _ => (),
        }
    });
}
