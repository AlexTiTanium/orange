#[allow(dead_code)]
use gl::Gl;
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

    //store.assets.load("cat", "./resources/cat_big.png");
    store.assets.load("cat", "./resources/cat_big.png");

    store.dispatch(Action::WindowResize(600, 800));

    let event_loop = glutin::event_loop::EventLoop::new();
    let state = store.state();

    let wb = WindowBuilder::new()
        .with_title("Hello world!")
        .with_inner_size(LogicalSize::new(state.window.width, state.window.width));

    let windowed_context = ContextBuilder::new().build_windowed(wb, &event_loop).unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let window = unsafe { windowed_context.make_current().unwrap() };

    // Create render context
    let context = Gl::load_with(|symbol| window.get_proc_address(symbol));

    // Init game render
    let mut renderer = render::create_renderer(&store, &context);

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
                    window.resize(physical_size)
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::MainEventsCleared => {
                //println!("[Game] Elapsed time ms: {:?}", time.elapsed().as_millis());
                //println!("[Game] Delta time ms: {:?}", Instant::now().duration_since(delta).as_millis());
                window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                render::step(&context, &mut renderer, time);
                window.swap_buffers().unwrap();
            }
            Event::RedrawEventsCleared => {
                delta = Instant::now();
            }
            _ => (),
        }
    });
}
