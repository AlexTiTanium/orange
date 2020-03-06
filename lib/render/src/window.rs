use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

use gl;
use gl::gl::types::*;

use std::ffi::CStr;
use std::ffi::CString;

pub fn create_window() {
  let event_loop = glutin::event_loop::EventLoop::new();

  gl::debug();

  let wb = WindowBuilder::new()
    .with_title("Hello world!")
    .with_inner_size(LogicalSize::new(1024.0, 768.0));

  let gl_window = ContextBuilder::new()
    .build_windowed(wb, &event_loop)
    .unwrap();

  // It is essential to make the context current before calling `gl::load_with`.
  let gl_window = unsafe { gl_window.make_current().unwrap() };

  // Load the OpenGL function pointers
  gl::gl::load_with(|symbol| gl_window.get_proc_address(symbol));

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::LoopDestroyed => return,
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Resized(_physical_size) => {
          // gl_window.resize(physical_size)
        }
        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
        _ => (),
      },
      Event::RedrawRequested(_) => {
        unsafe {
          gl::gl::ClearColor(1.0, 0.0, 0.0, 0.0);
          gl::gl::Clear(gl::gl::COLOR_BUFFER_BIT);
          let x = gl::gl::GetString(gl::gl::VENDOR);
          let s = CStr::from_ptr(x as *mut _);
          println!("{:?}", s);

          let x = gl::gl::GetString(gl::gl::RENDERER);
          let s = CStr::from_ptr(x as *mut _);
          println!("{:?}", s);

          let x = gl::gl::GetString(gl::gl::VERSION);
          let s = CStr::from_ptr(x as *mut _);
          println!("{:?}", s);
        }
        gl_window.swap_buffers().unwrap();
      }
      _ => (),
    }
  });
}
