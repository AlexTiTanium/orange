use ecs::components::*;
use ecs::resources::Camera;
use ecs::{EntityId, IntoIter, Shiperator, State, UniqueView, View};
use gl::{Gl, Renderer, ShaderType, GL, GLT};
use std::collections::HashMap;
use std::{cmp, mem, str};

// pub static SHADER_BASIC_VERT: &str = include_str!("../shaders/gl/shader_basic_vert.glsl");
// pub static SHADER_BASIC_FRAG: &str = include_str!("../shaders/gl/shader_basic_frag.glsl");

pub static SHADER_COLOR_VERT: &str = include_str!("../shaders/gl/shader_color_vert.glsl");
pub static SHADER_COLOR_FRAG: &str = include_str!("../shaders/gl/shader_color_frag.glsl");

pub struct OpenGL {
  gl: Gl,
  renderer: Renderer,
  buffer: Vec<f32>,
  map: HashMap<EntityId, usize>,
  max_quads: usize,
}

impl OpenGL {
  pub fn new<F>(_: &State, load: F) -> Self
  where
    F: FnMut(&'static str) -> *const GLT::GLvoid,
  {
    let gl = Gl::load_with(load);

    let max_quads: usize = 4;

    let renderer = create_quads_renderer(max_quads, &gl);
    let buffer = Vec::with_capacity(50);
    let map = HashMap::with_capacity(50);

    Self { map, buffer, renderer, max_quads, gl: gl.clone() }
  }

  pub fn fill_buffer(&mut self, state: &State) {
    let (transform, active) = state.world.borrow::<(View<Transform>, View<ActiveTag>)>();

    let width = 300.0;
    let height = 300.0;
    let shift = 20;

    for i in &mut self.buffer {
      *i = 0.0
    }

    for (index, (id, (trans, _))) in (&transform, &active).iter().with_id().enumerate() {
      self.map.insert(id, index);

      if index * shift >= self.buffer.len() {
        self.buffer.resize(shift * (index + 1), 0.0);
      }

      // top left
      // Position
      self.buffer[index * shift] = trans.position.x;
      self.buffer[index * shift + 1] = trans.position.y;
      // Color
      self.buffer[index * shift + 2] = 1.0;
      self.buffer[index * shift + 3] = 0.0;
      self.buffer[index * shift + 4] = 0.0;

      // bottom left
      // Position
      self.buffer[index * shift + 5] = trans.position.x;
      self.buffer[index * shift + 6] = trans.position.y + height;
      // Color
      self.buffer[index * shift + 7] = 1.0;
      self.buffer[index * shift + 8] = 0.0;
      self.buffer[index * shift + 9] = 0.0;

      // top right
      // Position
      self.buffer[index * shift + 10] = trans.position.x + width;
      self.buffer[index * shift + 11] = trans.position.y;
      // Color
      self.buffer[index * shift + 12] = 1.0;
      self.buffer[index * shift + 13] = 0.0;
      self.buffer[index * shift + 14] = 0.0;

      // bottom right
      // Position
      self.buffer[index * shift + 15] = trans.position.x + width;
      self.buffer[index * shift + 16] = trans.position.y + height;
      // Color
      self.buffer[index * shift + 17] = 1.0;
      self.buffer[index * shift + 18] = 0.0;
      self.buffer[index * shift + 19] = 0.0;
    }
  }

  pub fn update(&mut self, state: &State) {
    self.fill_buffer(&state);
  }

  pub fn step(&mut self, state: &State) {
    self.clear();

    let camera = state.world.borrow::<UniqueView<Camera>>();

    self.renderer.set_view_projection(&camera.view_projection);
    self.renderer.bind();

    let draw_calls: f32 = self.buffer.len() as f32 / (self.max_quads * 20) as f32;
    for n in 0..draw_calls.ceil() as u32 {
      let start = n as usize * (self.max_quads * 20) as usize;
      let end = cmp::min(start + self.max_quads * 20, self.buffer.len());

      self.renderer.set_data(&self.buffer[start..end]);
      self.renderer.draw();
    }
  }

  pub fn clear(&self) {
    unsafe {
      self.gl.ClearColor(0.2, 0.2, 0.2, 1.0);
      self.gl.Clear(GL::COLOR_BUFFER_BIT);
    }
  }
}

fn create_quads_renderer(max_quads: usize, gl: &Gl) -> Renderer {
  let mut renderer = Renderer::new(&gl);

  let buffer_size: usize = (20 * mem::size_of::<f32>()) * max_quads;
  let mut indexes: Vec<u32> = Vec::with_capacity(6 * max_quads);

  // Clockwise
  for n in 0..max_quads {
    // first triangle
    indexes.push((1 + n * 4) as u32);
    indexes.push((0 + n * 4) as u32);
    indexes.push((2 + n * 4) as u32);
    // second triangle
    indexes.push((1 + n * 4) as u32);
    indexes.push((2 + n * 4) as u32);
    indexes.push((3 + n * 4) as u32);
  }

  renderer
    .set_vertices_buffer_size(buffer_size)
    .add_layout::<f32>(2) // Loc = 0 position
    .add_layout::<f32>(3) // Loc = 1 color
    .build_layout()
    .add_shader(ShaderType::Vertex, SHADER_COLOR_VERT)
    .add_shader(ShaderType::Fragment, SHADER_COLOR_FRAG)
    .build_shader()
    .add_indexes(&indexes, indexes.len())
    .create_mvp();

  renderer
}
