use editor::Editor;
use flexi_logger::Logger;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use render;
use state::{create_store, Action};
use std::time::Instant;

fn main() {
    // Setup logger
    Logger::with_str("debug").start_with_specfile("./logspec.toml").unwrap();

    // Create storage
    let mut store = create_store();

    // Load assets
    store.assets.load("cat", "cat_big.png");
    store.assets.load("tree", "tree.png");

    // Set window size
    store.dispatch(Action::WindowResize(600, 800));

    // Start event loop
    let event_loop = glutin::event_loop::EventLoop::new();
    let state = store.state();

    // Create winit window
    let wb = WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(LogicalSize::new(state.window.width, state.window.width));

    // Create windowed context
    let windowed_context = ContextBuilder::new().with_vsync(true).build_windowed(wb, &event_loop).unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let window = unsafe { windowed_context.make_current().unwrap() };

    // Game render
    let mut render = render::create(&store, |symbol| window.get_proc_address(symbol));

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
                WindowEvent::Resized(physical_size) => {
                    store.dispatch(Action::WindowResize(physical_size.width, physical_size.height));
                    window.resize(physical_size)
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
                render.step(&store);
                editor.step(&store, &window.window());
                window.swap_buffers().unwrap();
            }
            Event::RedrawEventsCleared => {
                //delta = Instant::now();
            }
            _ => (),
        }
    });
}
