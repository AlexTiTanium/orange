use ecs::components::{ActiveTag, Image, LayerRef, Sprite, Texture, TileRef, Transform};
use ecs::resources::{Assets, Camera};
use ecs::{EntityId, IntoIter, Shiperator, State, UniqueView, View};
use gl::{Gl, Renderer, ShaderType};
use std::{cmp, mem, str};

pub static SHADER_COLOR_VERT: &str = include_str!("./shader_atlas_vert.glsl");
pub static SHADER_COLOR_FRAG: &str = include_str!("./shader_atlas_frag.glsl");

pub struct QuadAtlasRender {
  renderer: Renderer,
  buffer: Vec<f32>,
  max_quads: usize,
  element_buffer_size: usize,
  texture_slot: i32,
}

impl QuadAtlasRender {
  pub fn new(gl: &Gl) -> Self {
    let max_quads: usize = 400;
    let element_buffer_size = 16;
    let texture_slot = 0;

    let renderer = Self::create_renderer(&gl, max_quads, element_buffer_size);
    let buffer = Vec::with_capacity(50);

    Self {
      buffer,
      renderer,
      max_quads,
      element_buffer_size,
      texture_slot,
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
      .add_layout::<f32>(2) // Loc = 1 uv
      .build_layout()
      .add_shader(ShaderType::Vertex, SHADER_COLOR_VERT)
      .add_shader(ShaderType::Fragment, SHADER_COLOR_FRAG)
      .build_shader()
      .add_indexes(&indexes, indexes.len())
      .create_uniform("u_Texture")
      .create_mvp();

    renderer
  }

  #[inline]
  fn update(&mut self, state: &State, layer_id: EntityId, texture_id: EntityId) {
    let (transform, active, images, sprites, tile, layers, textures) = state.world.borrow::<(
      View<Transform>,
      View<ActiveTag>,
      View<Image>,
      View<Sprite>,
      View<TileRef>,
      View<LayerRef>,
      View<Texture>,
    )>();

    let mut width: f32;
    let mut height: f32;
    let shift = self.element_buffer_size;

    // what texture slot we going to bind
    self.texture_slot = textures[texture_id].slot;

    // Clear buffer
    for i in &mut self.buffer {
      *i = 0.0
    }

    for (index, (trans, tile, _, _)) in (&transform, &tile, &layers, &active)
      .iter()
      .enumerate()
      .filter(|(_, (_, tile, layer, _))| layer.0 == layer_id && sprites[tile.0].texture == texture_id)
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

      // 0 -- 1
      // |    |   1 -> 0 -> 2 first
      // 3 -- 2   1 -> 2 -> 3 second

      // top right - 1
      // Position
      self.buffer[index * shift] = trans.position.x;
      self.buffer[index * shift + 1] = trans.position.y;
      // UV
      self.buffer[index * shift + 2] = sprites[tile.0].uv[0][0];
      self.buffer[index * shift + 3] = sprites[tile.0].uv[0][1];

      // top left - 0
      // Position
      self.buffer[index * shift + 4] = trans.position.x;
      self.buffer[index * shift + 5] = trans.position.y + height;
      // UV
      self.buffer[index * shift + 6] = sprites[tile.0].uv[1][0];
      self.buffer[index * shift + 7] = sprites[tile.0].uv[1][1];

      // bottom right - 2
      // Position
      self.buffer[index * shift + 8] = trans.position.x + width;
      self.buffer[index * shift + 9] = trans.position.y;
      // UV
      self.buffer[index * shift + 10] = sprites[tile.0].uv[2][0];
      self.buffer[index * shift + 11] = sprites[tile.0].uv[2][1];

      // bottom left - 3
      // Position
      self.buffer[index * shift + 12] = trans.position.x + width;
      self.buffer[index * shift + 13] = trans.position.y + height;
      // UV
      self.buffer[index * shift + 14] = sprites[tile.0].uv[3][0];
      self.buffer[index * shift + 15] = sprites[tile.0].uv[3][1];
    }
  }

  pub fn step(&mut self, state: &State, layer_id: EntityId, texture_id: EntityId) {
    // Prepare render step
    self.update(state, layer_id, texture_id);

    let camera = state.world.borrow::<UniqueView<Camera>>();
    let shift = self.element_buffer_size;

    self.renderer.set_view_projection(&camera.view_projection);
    self.renderer.bind();

    self.renderer.bind_texture(self.texture_slot);

    let draw_calls: f32 = self.buffer.len() as f32 / (self.max_quads * shift) as f32;
    for n in 0..draw_calls.ceil() as u32 {
      let start = n as usize * (self.max_quads * shift) as usize;
      let end = cmp::min(start + self.max_quads * shift, self.buffer.len());

      self.renderer.set_data(&self.buffer[start..end]);

      self.renderer.draw();
    }

    self.renderer.unbind_texture(self.texture_slot);
  }

  pub fn load_textures(&mut self, state: &State) {
    let assets = state.world.borrow::<UniqueView<Assets>>();

    for (&slot, texture) in assets.textures.iter() {
      self.renderer.add_texture(slot, texture.width, texture.height, &texture.data);
      // TODO: Unload texture from assets?
    }
  }
}
