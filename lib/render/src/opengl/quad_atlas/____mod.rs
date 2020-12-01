use game::components::*;
use game::resources::{Assets, Camera};
use game::{IntoIter, Shiperator, State, UniqueView, View};
use gl::Gl;
use gl::Renderer;
use gl::ShaderType;
use gl::GLT;
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

    let (transform, active, textures, assets, camera) =
      state
        .world
        .borrow::<(View<Transform>, View<ActiveTag>, View<Texture>, UniqueView<Assets>, UniqueView<Camera>)>();

    self.renderer.set_view_projection(&camera.view_projection);

    for (id, (trans, _)) in (&transform, &active).iter().with_id() {
      let has_texture = textures.contains(id);
      let slot_id = if has_texture { assets.images[&textures[id].id] } else { 0 };

      self.renderer.translate(&trans.position);
      self.renderer.bind();

      if has_texture {
        self.renderer.bind_texture(slot_id);
      }

      self.renderer.draw();

      if has_texture {
        self.renderer.unbind_texture(slot_id);
      }
    }
  }
}

fn create_renderer(state: &State, gl: &Gl) -> Renderer {
  let assets = state.world.borrow::<UniqueView<Assets>>();

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

  renderer.create_mvp();

  renderer
}
