use gl::Gl;
use gl::Renderer;
use gl::ShaderType;
use gl::TextureSlot;
use state::Store;

use std::{str, time::Instant};

pub static SHADER_BASIC_VERT: &str = include_str!("./shaders/gl/shader_basic_vert.glsl");
pub static SHADER_BASIC_FRAG: &str = include_str!("./shaders/gl/shader_basic_frag.glsl");

pub fn create_renderer(store: &Store, gl: &Gl) -> Renderer {
  #[rustfmt::skip]
  let vertices: [f32; 2 * 4 + 4 * 3 + 4 * 2] = [
  // position loc=0    | color loc=1  | texture loc=2 |
   -0.5, -0.5, /* 0 */  1.0, 0.0, 0.0,  0.0, 0.0,  // bottom left
    0.5, -0.5, /* 1 */  0.0, 1.0, 0.0,  1.0, 0.0,  // bottom right
    0.5,  0.5, /* 2 */  0.0, 0.0, 1.0,  1.0, 1.0,  // top right
   -0.5,  0.5, /* 3 */  1.0, 1.0, 0.0,  0.0, 1.0,  // top left
  ];

  let indexes: [u16; 2 * 3] = [
    0, 1, 2, // First triangle
    0, 2, 3, // Second triangle
  ];

  let mut renderer = Renderer::new(&gl);

  let cat = store.assets.get("cat");
  let _tree = store.assets.get("tree");

  renderer
    .add_vertices(&vertices)
    .add_layout::<f32>(2) // Loc = 0
    .add_layout::<f32>(3) // Loc = 1
    .add_layout::<f32>(2) // Loc = 2
    .commit_layout()
    .add_shader(ShaderType::Vertex, SHADER_BASIC_VERT)
    .add_shader(ShaderType::Fragment, SHADER_BASIC_FRAG)
    .commit_shaders()
    .add_texture(TextureSlot::DEFAULT, cat.width, cat.height, &cat.data)
    .add_indexes(&indexes, 6);

  renderer.create_mvp();
  renderer.create_uniform("u_Texture");
  renderer.create_uniform("u_Color");

  renderer
}

pub fn step(_gl: &Gl, renderer: &mut Renderer, time: Instant) {
  let r = time.elapsed().as_secs_f32().sin() * 0.5 + 0.5;
  let g = time.elapsed().as_secs_f32().cos() * 0.5 + 0.5;

  renderer.select_texture_slot(TextureSlot::DEFAULT);
  renderer.select_texture_slot(TextureSlot::ONE);

  renderer.bind();

  renderer.set_uniform_i1("u_Texture", 0);
  renderer.set_uniform_f4("u_Color", &[r, g, 0.5, 1.0]);

  renderer.draw();
}
