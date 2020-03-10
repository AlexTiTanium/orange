use gl::shader;
use gl::Gl;
use gl::IndexBuffer;
use gl::Layout;
pub use gl::RenderTarget;
use gl::VertexArray;
use gl::VertexBuffer;
use gl::GL;

use std::ffi::CString;
use std::{str, time::Instant};

pub static SHADER_BASIC_VERT: &'static str = include_str!("../../../resources/shader_basic_vert.glsl");
pub static SHADER_BASIC_FRAG: &'static str = include_str!("../../../resources/shader_basic_frag.glsl");

pub fn create_target(gl: &Gl) -> RenderTarget {
  let vertices: [f32; 2 * 4] = [
    -0.5, -0.5, // 0
    0.5, -0.5, //  1
    0.5, 0.5, //   2
    -0.5, 0.5, //  3
  ];

  let indexes: [u16; 2 * 3] = [
    0, 1, 2, // First triangle
    0, 2, 3, // Second triangle
  ];

  let vao = VertexArray::new(&gl);
  let vbo = VertexBuffer::new(&gl, &vertices);
  let mut layout = Layout::new();
  let ibo = IndexBuffer::new(gl, &indexes, 6);

  layout.push::<f32>(2);
  vao.add_buffer(&gl, &vbo, &layout);

  // Debug draw
  // unsafe {
  //   gl.PolygonMode(GL::FRONT_AND_BACK, GL::LINE);
  // }

  let vert_shader = shader::compile(&gl, shader::Type::Vertex, &SHADER_BASIC_VERT).unwrap();
  let frag_shader = shader::compile(&gl, shader::Type::Fragment, &SHADER_BASIC_FRAG).unwrap();
  let program = shader::create_program(&gl, vert_shader, frag_shader).unwrap();

  // Setup color uniform
  let color_uniform_name = CString::new("u_Color").unwrap();
  let color_location = unsafe { gl.GetUniformLocation(program, color_uniform_name.as_ptr()) };
  // TODO: Add warning for location == -1

  RenderTarget {
    program,
    vao,
    vbo,
    ibo,
    color_location,
  }
}

pub fn step(gl: &GL::Gl, target: &RenderTarget, time: Instant) {
  let r = time.elapsed().as_secs_f32().sin() * 0.5 + 0.5;
  let g = time.elapsed().as_secs_f32().cos() * 0.5 + 0.5;

  unsafe {
    gl.UseProgram(target.program);
    gl.Uniform4f(target.color_location, r, g, 0.5, 1.0);

    target.vao.bind(&gl);
    target.ibo.bind(&gl);

    target.ibo.draw(&gl);
  }
}
