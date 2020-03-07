use gl as GL;
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use std::ffi::CStr;

pub fn create_window() {
  let event_loop = glutin::event_loop::EventLoop::new();

  let wb = WindowBuilder::new()
    .with_title("Hello world!")
    .with_inner_size(LogicalSize::new(1024.0, 768.0));

  let gl_window = ContextBuilder::new()
    .build_windowed(wb, &event_loop)
    .unwrap();

  // It is essential to make the context current before calling `gl::load_with`.
  let gl_window = unsafe { gl_window.make_current().unwrap() };

  // Load the OpenGL function pointers
  let gl = GL::Gl::load_with(|symbol| gl_window.get_proc_address(symbol));

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::LoopDestroyed => return,
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Resized(physical_size) => gl_window.resize(physical_size),
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        _ => (),
      },
      Event::RedrawRequested(_) => {
        unsafe {
          gl.ClearColor(1.0, 0.0, 0.0, 0.0);
          gl.Clear(GL::COLOR_BUFFER_BIT);
          let x = gl.GetString(GL::VENDOR);
          let s = CStr::from_ptr(x as *mut _);
          println!("{:?}", s);

          let x = gl.GetString(GL::RENDERER);
          let s = CStr::from_ptr(x as *mut _);
          println!("{:?}", s);

          let x = gl.GetString(GL::VERSION);
          let s = CStr::from_ptr(x as *mut _);
          println!("{:?}", s);
        }
        gl_window.swap_buffers().unwrap();
      }
      _ => (),
    }
  });
}
