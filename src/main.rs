#[allow(dead_code)]
mod resources;
use gl::GL;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use render;
use state::{create_store, Action};
use std::time::Instant;

fn main() {
    let mut store = create_store();

    store.dispatch(Action::WindowResize(600, 800));

    let event_loop = glutin::event_loop::EventLoop::new();
    let state = store.state();

    let wb = WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(LogicalSize::new(state.window.width, state.window.width));

    let windowed_context = ContextBuilder::new().build_windowed(wb, &event_loop).unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { windowed_context.make_current().unwrap() };

    // Load the OpenGL function pointers
    let gl = GL::Gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    // Init game render
    let target = render::create_target(&gl);

    // Store monotonic clock time since start
    let time = Instant::now();

    // Delta time
    let mut delta = Instant::now();

    // Game event loop
    event_loop.run(move |event, _, control_flow| {
        // Poll should work better for games
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    store.dispatch(Action::WindowResize(physical_size.width, physical_size.height));
                    gl_window.resize(physical_size)
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                //println!("[Game] Elapsed time ms: {:?}", time.elapsed().as_millis());
                //println!("[Game] Delta time ms: {:?}", Instant::now().duration_since(delta).as_millis());
                //gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                render::step(&gl, &target, time);
                gl_window.swap_buffers().unwrap();
            }
            Event::RedrawEventsCleared => {
                delta = Instant::now();
            }
            _ => (),
        }
    });
}
