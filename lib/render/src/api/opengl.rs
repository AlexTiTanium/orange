use ecs::components::*;
use ecs::resources::Window;
use ecs::*;
use gl::Gl;
use gl::Renderer;
use gl::GLT;

use gl::ShaderType;
use gl::TextureSlot;

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
    // let r = time.elapsed().as_secs_f32().sin() * 0.5 + 0.5;
    // let g = time.elapsed().as_secs_f32().cos() * 0.5 + 0.5;

    let (transform, _, active) = state.world.borrow::<(&Transform, &GameObject, &ActiveTag)>();

    (&transform, &active).iter().for_each(|(trans, _)| {
      self.renderer.translate(&trans.position);
      self.renderer.bind();
      self.renderer.draw();
    });

    //renderer.select_texture_slot(TextureSlot::DEFAULT);
    //renderer.select_texture_slot(TextureSlot::ONE);

    //self.renderer.bind();

    //renderer.set_uniform_i1("u_Texture", 0);
    //self.renderer.set_uniform_f4("u_Color", &[r, g, 0.5, 1.0]);

    //self.renderer.draw();
  }
}

fn create_renderer(state: &State, gl: &Gl) -> Renderer {
  let display = state.world.borrow::<Unique<&Window>>();

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
    //.add_texture(TextureSlot::DEFAULT, cat.width, cat.height, &cat.data)
    //.add_texture(TextureSlot::DEFAULT, _tree.width, _tree.height, &_tree.data)
    .add_indexes(&indexes, 6);

  renderer.create_mvp(display.width, display.height);
  //renderer.create_uniform("u_Texture");
  renderer.create_uniform("u_Color");

  renderer
}
