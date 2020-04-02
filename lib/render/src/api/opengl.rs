use ecs::components::*;
use ecs::resources::Assets;
use ecs::resources::Camera;
use ecs::resources::Window;
use ecs::*;
use gl::Gl;
use gl::Renderer;
use gl::GLT;

use gl::ShaderType;

use std::str;

pub static SHADER_BASIC_VERT: &str = include_str!("../shaders/gl/shader_basic_vert.glsl");
pub static SHADER_BASIC_FRAG: &str = include_str!("../shaders/gl/shader_basic_frag.glsl");

pub struct OpenGL {
  renderer: Renderer,
}

impl OpenGL {
  pub fn new<F>(state: &State, load: F) -> Self
  where
    F: FnMut(&'static str) -> *const GLT::GLvoid,
  {
    let gl = Gl::load_with(load);
    let renderer = create_renderer(state, &gl);

    Self { renderer }
  }

  pub fn update_time() {
    println!("Not implemented");
  }

  pub fn step(&mut self, state: &State) {
    self.renderer.clear();

    let (transform, _, active) = state.world.borrow::<(&Transform, &GameObject, &ActiveTag)>();
    let textures = state.world.borrow::<&Texture>();
    let assets = state.world.borrow::<Unique<&Assets>>();
    let camera = state.world.borrow::<Unique<&Camera>>();

    self.renderer.set_uniform_mat4("u_View", &camera.view);

    (&transform, &active).iter().with_id().for_each(|(id, (trans, _))| {
      self.renderer.translate(&trans.position);
      self.renderer.bind();

      if textures.contains(id) {
        self.renderer.bind_texture(assets.images[&textures[id].id]);
      }

      self.renderer.draw();

      if textures.contains(id) {
        self.renderer.unbind_texture(assets.images[&textures[id].id]);
      }
    });
  }
}

fn create_renderer(state: &State, gl: &Gl) -> Renderer {
  let display = state.world.borrow::<Unique<&Window>>();
  let assets = state.world.borrow::<Unique<&Assets>>();

  #[rustfmt::skip]
  let vertices: [f32; 2 * 4 + 4 * 3 + 4 * 2] = [
  // position loc=0          | color loc=1  | texture loc=2 |
    0.0,    0.0,    /* 0 */  1.0, 1.0, 0.0,  0.0, 1.0,  // top left     -, +
    0.0,    300.0,  /* 1 */  1.0, 0.0, 0.0,  0.0, 0.0,  // bottom left  -, -
    300.0,  0.0,    /* 2 */  0.0, 0.0, 1.0,  1.0, 1.0,  // top right    +, +
    300.0,  300.0,  /* 3 */  0.0, 1.0, 0.0,  1.0, 0.0,  // bottom right +, -
  ];

  // Clockwise
  let indexes: [u16; 2 * 3] = [
    1, 0, 2, // first triangle
    1, 2, 3, // second triangle
  ];

  let mut renderer = Renderer::new(&gl);

  renderer
    .add_vertices(&vertices)
    .add_layout::<f32>(2) // Loc = 0 position
    .add_layout::<f32>(3) // Loc = 1 color
    .add_layout::<f32>(2) // Loc = 2 uv
    .commit_layout()
    .add_shader(ShaderType::Vertex, SHADER_BASIC_VERT)
    .add_shader(ShaderType::Fragment, SHADER_BASIC_FRAG)
    .commit_shaders()
    .add_indexes(&indexes, 6);

  // Add textures to slots
  renderer.create_uniform("u_Texture");
  for (&slot, texture) in assets.textures.iter() {
    renderer.add_texture(slot, texture.width, texture.height, &texture.data);
    // TODO: Unload texture from assets
  }

  renderer.create_mvp(display.width, display.height);

  renderer
}
