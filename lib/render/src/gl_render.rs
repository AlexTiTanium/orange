use gl::Gl;
use gl::Renderer;
use gl::ShaderType;
use state::Store;

use std::{str, time::Instant};

pub static SHADER_BASIC_VERT: &'static str = include_str!("./shaders/gl/shader_basic_vert.glsl");
pub static SHADER_BASIC_FRAG: &'static str = include_str!("./shaders/gl/shader_basic_frag.glsl");

pub fn create_renderer(store: &Store, gl: &Gl) -> Renderer {
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

  //let res = store.assets.get("cat");

  renderer
    .add_vertices(&vertices)
    .add_layout::<f32>(2)
    .commit_layout()
    .add_shader(ShaderType::Vertex, SHADER_BASIC_VERT)
    .add_shader(ShaderType::Fragment, SHADER_BASIC_FRAG)
    .commit_shaders()
    .add_indexes(&indexes, 6);

  return renderer;
}

pub fn step(gl: &Gl, renderer: &mut Renderer, time: Instant) {
  let r = time.elapsed().as_secs_f32().sin() * 0.5 + 0.5;
  let g = time.elapsed().as_secs_f32().cos() * 0.5 + 0.5;

  renderer.bind();

  let color: [f32; 4] = [r, g, 0.5, 1.0];
  renderer.set_uniform_f4("u_Color", &[r, g, 0.5, 1.0]);

  renderer.draw();
}
