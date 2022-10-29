use std::collections::HashMap;

use egui::{
  epaint::{Primitive, Vertex},
  ClippedPrimitive, Rect, TextureId, TexturesDelta,
};

#[derive(Default)]
pub struct SrgTexture {
  pub pixels: Vec<u8>,
  pub width: usize,
  pub height: usize,
}

#[derive(Default)]
pub struct EditorBatchBuffer {
  pub textures_delta: TexturesDelta,
  pub textures: HashMap<egui::TextureId, SrgTexture>,
  pub buffers: Vec<RenderBuffer>,
  pub cursor: usize,
}

#[derive(Debug)]
pub struct RenderBuffer {
  pub vb: Vec<f32>,
  pub indexes: Vec<u32>,
  pub texture_id: TextureId,
  pub cursor: usize,
  pub clip: Rect,
}

impl RenderBuffer {
  pub fn new(texture_id: TextureId, clip: Rect) -> Self {
    Self {
      vb: Vec::new(),
      indexes: Vec::new(),
      texture_id,
      cursor: Default::default(),
      clip,
    }
  }

  pub fn add_indexes(&mut self, indexes: Vec<u32>) {
    self.indexes = indexes;
  }

  pub fn add_vertex(&mut self, vertex: &Vertex) {
    // Position
    self.vb.push(vertex.pos.x);
    self.vb.push(vertex.pos.y);

    // RGBA
    self.vb.push(vertex.color.r() as f32);
    self.vb.push(vertex.color.g() as f32);
    self.vb.push(vertex.color.b() as f32);
    self.vb.push(vertex.color.a() as f32);

    // UV
    self.vb.push(vertex.uv.x);
    self.vb.push(vertex.uv.y);
  }
}

impl EditorBatchBuffer {
  pub fn set_data(&mut self, primitives: &Vec<ClippedPrimitive>, texture: &TexturesDelta) {
    self.buffers.clear();

    for (id, delta) in &texture.set {
      let pixels: Vec<u8> = match &delta.image {
        egui::ImageData::Color(image) => {
          assert_eq!(
            image.width() * image.height(),
            image.pixels.len(),
            "Mismatch between texture size and texel count"
          );
          image.pixels.iter().flat_map(|color| color.to_srgba_unmultiplied()).collect()
        }
        egui::ImageData::Font(image) => image.srgba_pixels(1.0).flat_map(|color| color.to_array()).collect(),
      };

      let width = delta.image.width();
      let height = delta.image.height();

      self.textures.insert(*id, SrgTexture { pixels, width, height });
    }

    self.textures_delta = texture.clone();

    for ClippedPrimitive { clip_rect, primitive } in primitives.iter() {
      let mesh = match primitive {
        Primitive::Mesh(mesh) => mesh,
        Primitive::Callback(_) => {
          unimplemented!("Paint callbacks aren't supported")
        }
      };

      let mut buffer = RenderBuffer::new(mesh.texture_id, clip_rect.clone());

      buffer.add_indexes(mesh.indices.to_vec());

      for vertex in &mesh.vertices {
        buffer.add_vertex(vertex);
      }

      self.buffers.push(buffer);
    }
  }

  pub fn free_texures_delta(&mut self) {
    for &id in &self.textures_delta.free {
      self.textures.remove(&id);
    }
  }
}
