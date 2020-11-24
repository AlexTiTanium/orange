use ecs::components::*;
use ecs::resources::Camera;
use ecs::{EntityId, IntoIter, Shiperator, State, UniqueView, View};
use gl::{Gl, Renderer, ShaderType};
use std::collections::HashMap;
use std::{cmp, mem, str};

pub static SHADER_COLOR_VERT: &str = include_str!("./shader_color_vert.glsl");
pub static SHADER_COLOR_FRAG: &str = include_str!("./shader_color_frag.glsl");

pub struct QuadColorRender {
  renderer: Renderer,
  buffer: Vec<f32>,
  map: HashMap<EntityId, usize>,
  max_quads: usize,
}

impl QuadColorRender {
  pub fn new(gl: &Gl) -> Self {
    let max_quads: usize = 4;

    let renderer = Self::create_renderer(&gl, max_quads);
    let buffer = Vec::with_capacity(50);
    let map = HashMap::with_capacity(50);

    Self {
      map,
      buffer,
      renderer,
      max_quads,
    }
  }

  fn create_renderer(gl: &Gl, max_quads: usize) -> Renderer {
    let mut renderer = Renderer::new(gl);

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

  pub fn update(&mut self, state: &State) {
    let (transform, active, images, tile) = state.world.borrow::<(View<Transform>, View<ActiveTag>, View<Image>, View<TileRef>)>();

    let mut width: f32 = 300.0;
    let mut height: f32 = 300.0;
    let shift = 20;

    for i in &mut self.buffer {
      *i = 0.0
    }

    for (index, (id, (trans, tile, _))) in (&transform, &tile, &active).iter().with_id().enumerate() {
      self.map.insert(id, index);

      if index * shift >= self.buffer.len() {
        self.buffer.resize(shift * (index + 1), 0.0);
      }

      if images.contains(tile.0) {
        width = images[tile.0].width;
        height = images[tile.0].height;
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

  pub fn step(&mut self, state: &State) {
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
}
