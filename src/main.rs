#[allow(dead_code)]
mod resources;
use gl as GL;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use state::{create_store, Action};

fn main() {
    let mut store = create_store();

    store.dispatch(Action::WindowResize(600, 800));

    let event_loop = glutin::event_loop::EventLoop::new();
    let state = store.state();

    let wb = WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(LogicalSize::new(state.window.width, state.window.width));

    let gl_window = ContextBuilder::new()
        .build_windowed(wb, &event_loop)
        .unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let gl_window = unsafe { gl_window.make_current().unwrap() };

    // Load the OpenGL function pointers
    let gl = GL::Gl::load_with(|symbol| gl_window.get_proc_address(symbol));

    // Init game render
    render::init(&store, &gl);

    // Game event loop
    event_loop.run(move |event, _, control_flow| {
        // Poll should work better for games
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    store.dispatch(Action::WindowResize(
                        physical_size.width,
                        physical_size.height,
                    ));
                    gl_window.resize(physical_size)
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                render::step(&store, &gl);
                gl_window.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
