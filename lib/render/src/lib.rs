use gl as GL;
use gl::types as GLT;
use state::Store;
use std::mem;

pub fn init(_store: &Store, gl: &GL::Gl) {
  let vertices: [f32; 6] = [-0.5, -0.5, 0.5, -0.5, 0.0, 0.5];

  let mut vbo: GLT::GLuint = 0;

  unsafe {
    gl.GenBuffers(0, &mut vbo);
    gl.BindBuffer(GL::ARRAY_BUFFER, vbo);

    gl.BufferData(
      GL::ARRAY_BUFFER,
      mem::size_of_val(&vbo) as GLT::GLsizeiptr,
      vertices.as_ptr() as *const GLT::GLvoid,
      GL::STATIC_DRAW,
    );
  }
}

pub fn step(_store: &Store, gl: &GL::Gl) {
  unsafe {
    gl.ClearColor(1.0, 0.0, 0.0, 0.0);
    gl.Clear(GL::COLOR_BUFFER_BIT);
  }
}
