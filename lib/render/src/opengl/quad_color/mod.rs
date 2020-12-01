use game::components::{ActiveTag, Image, LayerRef, TileRef, Transform};
use game::resources::Camera;
use game::{EntityId, IntoIter, Shiperator, State, UniqueView, View};
use gl::{Gl, Renderer, ShaderType};
use std::{cmp, mem, str};

pub static SHADER_COLOR_VERT: &str = include_str!("./shader_color_vert.glsl");
pub static SHADER_COLOR_FRAG: &str = include_str!("./shader_color_frag.glsl");

pub struct QuadColorRender {
  renderer: Renderer,
  buffer: Vec<f32>,
  max_quads: usize,
  element_buffer_size: usize,
}

impl QuadColorRender {
  pub fn new(gl: &Gl) -> Self {
    let max_quads: usize = 400;
    let element_buffer_size = 20;

    let renderer = Self::create_renderer(&gl, max_quads, element_buffer_size);
    let buffer = Vec::with_capacity(50);

    Self {
      buffer,
      renderer,
      max_quads,
      element_buffer_size,
    }
  }

  fn create_renderer(gl: &Gl, max_quads: usize, element_buffer_size: usize) -> Renderer {
    let mut renderer = Renderer::new(gl);

    let buffer_size: usize = (element_buffer_size * mem::size_of::<f32>()) * max_quads;
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

  #[inline]
  fn update(&mut self, state: &State, layer_entity_id: EntityId) {
    let (transform, active, images, tile, layers) = state
      .world
      .borrow::<(View<Transform>, View<ActiveTag>, View<Image>, View<TileRef>, View<LayerRef>)>();

    let mut width: f32;
    let mut height: f32;
    let shift = self.element_buffer_size;

    for i in &mut self.buffer {
      *i = 0.0
    }

    for (index, (trans, tile, _, _)) in (&transform, &tile, &layers, &active)
      .iter()
      .enumerate()
      .filter(|(_, (_, _, layer, _))| layer.0 == layer_entity_id)
    {
      if index * shift >= self.buffer.len() {
        self.buffer.resize(shift * (index + 1), 0.0);
      }

      if images.contains(tile.0) {
        width = images[tile.0].width;
        height = images[tile.0].height;
      } else {
        width = 0.0;
        height = 0.0;
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

  pub fn step(&mut self, state: &State, layer_entity_id: EntityId) {
    // Prepare buffer for rendering
    self.update(state, layer_entity_id);

    let camera = state.world.borrow::<UniqueView<Camera>>();
    let shift = self.element_buffer_size;

    self.renderer.set_view_projection(&camera.view_projection);
    self.renderer.bind();

    let draw_calls: f32 = self.buffer.len() as f32 / (self.max_quads * shift) as f32;
    for n in 0..draw_calls.ceil() as u32 {
      let start = n as usize * (self.max_quads * shift) as usize;
      let end = cmp::min(start + self.max_quads * shift, self.buffer.len());

      self.renderer.set_data(&self.buffer[start..end]);
      self.renderer.draw();
    }
  }
}
