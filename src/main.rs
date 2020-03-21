use flexi_logger::Logger;
use gl::Gl;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use imgui::{im_str, Condition, Context, Io, Ui, Window};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
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

    let mut imgui = Context::create();
    imgui.fonts().build_rgba32_texture();

    // Create windowed context
    let windowed_context = ContextBuilder::new().with_vsync(true).build_windowed(wb, &event_loop).unwrap();

    // It is essential to make the context current before calling `gl::load_with`.
    let window = unsafe { windowed_context.make_current().unwrap() };

    let ui_renderer = imgui_opengl_renderer::Renderer::new(&mut imgui, |symbol| window.get_proc_address(symbol));

    // Create render context
    let context = Gl::load_with(|symbol| window.get_proc_address(symbol));

    // Init game render
    let mut renderer = render::create_renderer(&store, &context);

    // Store monotonic clock time since start
    let time = Instant::now();

    // Delta time
    let mut delta = Instant::now();

    let mut platform = WinitPlatform::init(&mut imgui); // step 1
    platform.attach_window(imgui.io_mut(), &window.window(), HiDpiMode::Default); // step 2

    // Game event loop
    event_loop.run(move |event, _, control_flow| {
        // Poll should work better for games
        *control_flow = ControlFlow::Poll;

        platform.handle_event(imgui.io_mut(), &window.window(), &event);

        match event {
            Event::NewEvents(_) => {
                delta = imgui.io_mut().update_delta_time(delta);
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
                platform.prepare_frame(imgui.io_mut(), &window.window()).expect("Failed to prepare frame");
                window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                renderer.clear();

                render::step(&context, &mut renderer, time);

                let ui = imgui.frame();
                platform.prepare_render(&ui, &window.window());

                draw_ui(&ui, time);

                ui_renderer.render(ui);
                window.swap_buffers().unwrap();
            }
            Event::RedrawEventsCleared => {
                //delta = Instant::now();
            }
            _ => (),
        }
    });
}

fn draw_ui(ui: &Ui, time: Instant) {
    Window::new(im_str!("Hello world"))
        .size([300.0, 100.0], Condition::FirstUseEver)
        .build(&ui, || {
            ui.text(im_str!("Hello world!{:?}", time.elapsed().as_millis()));
            ui.text(im_str!("こんにちは世界！"));
            ui.text(im_str!("This...is...imgui-rs!  {:?}", ui.io().delta_time));
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!("Mouse Position: ({:.1},{:.1})", mouse_pos[0], mouse_pos[1]));
        });
}
