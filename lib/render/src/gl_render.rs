use gl::Gl;
use gl::Renderer;
use gl::ShaderType;

use std::ffi::CString;
use std::{str, time::Instant};

pub static SHADER_BASIC_VERT: &'static str = include_str!("../../../resources/shader_basic_vert.glsl");
pub static SHADER_BASIC_FRAG: &'static str = include_str!("../../../resources/shader_basic_frag.glsl");

pub fn create_renderer(gl: &Gl) -> Renderer {
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

  let mut renderer = Renderer::new(&gl);

  renderer
    .add_vertices(&vertices)
    .add_layout::<f32>(2)
    .commit_layout()
    .add_shader(ShaderType::Vertex, SHADER_BASIC_VERT)
    .add_shader(ShaderType::Fragment, SHADER_BASIC_FRAG)
    .commit_shaders()
    .add_indexes(&indexes, 6);

  // // Setup color uniform
  // let color_uniform_name = CString::new("u_Color").unwrap();
  // let color_location = unsafe { gl.GetUniformLocation(program, color_uniform_name.as_ptr()) };
  // // TODO: Add warning for location == -1

  return renderer;
}

pub fn step(gl: &Gl, renderer: &Renderer, time: Instant) {
  let r = time.elapsed().as_secs_f32().sin() * 0.5 + 0.5;
  let g = time.elapsed().as_secs_f32().cos() * 0.5 + 0.5;

  // unsafe {
  //   gl.UseProgram(target.program);
  //   gl.Uniform4f(target.color_location, r, g, 0.5, 1.0);

  //   target.vao.bind(&gl);
  //   target.ibo.bind(&gl);

  //   target.ibo.draw(&gl);
  // }

  renderer.draw();
}
