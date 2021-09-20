use std::mem;

use egui::{
  epaint::{Texture, Vertex},
  ClippedMesh, Rect, TextureId,
};

#[derive(Default)]
pub struct EditorBatchBuffer {
  pub texture: Texture,
  pub indexes: Vec<u32>,
  pub buffers: Vec<RenderBuffer>,
  pub cursor: usize,
}

pub struct RenderBuffer {
  pub vb: Vec<f32>,
  pub texture_id: TextureId,
  pub cursor: usize,
  pub clip: Rect,
}

impl RenderBuffer {
  pub fn new(texture_id: TextureId, clip: Rect) -> Self {
    let mut vb: Vec<f32> = Vec::new();
    vb.resize(EditorBatchBuffer::MAX_VERTEXES * EditorBatchBuffer::ATTRIBUTES_PER_VERTEX, 0.0);

    Self {
      vb,
      texture_id,
      cursor: Default::default(),
      clip,
    }
  }

  pub fn push(&mut self, vertex: &Vertex) {
    self.vb[self.cursor] = vertex.pos.x;
    self.vb[self.cursor + 1] = vertex.pos.y;

    self.vb[self.cursor + 2] = vertex.color.r() as f32;
    self.vb[self.cursor + 3] = vertex.color.g() as f32;
    self.vb[self.cursor + 4] = vertex.color.b() as f32;
    self.vb[self.cursor + 5] = vertex.color.a() as f32;

    self.vb[self.cursor + 6] = vertex.uv.x;
    self.vb[self.cursor + 7] = vertex.uv.y;

    self.cursor += 8;
  }

  pub fn clear(&mut self) {
    self.cursor = 0;

    for i in &mut self.vb {
      *i = 0.0
    }
  }
}

impl EditorBatchBuffer {
  const MAX_INDEXES: usize = 1000;
  const ATTRIBUTES_PER_VERTEX: usize = 8;
  const MAX_VERTEXES: usize = 1000;

  pub const MAX_INDEXES_SIZE: usize = mem::size_of::<u32>() * EditorBatchBuffer::MAX_INDEXES; // MAX_INDEXES
  pub const MAX_VBO_SIZE: usize = (mem::size_of::<f32>() * EditorBatchBuffer::ATTRIBUTES_PER_VERTEX) * EditorBatchBuffer::MAX_VERTEXES; // MAX_VERTEXES

  pub fn set_data(&mut self, meshes: &Vec<ClippedMesh>, texture: &Texture) {
    self.clear();

    self.texture = texture.clone();

    for ClippedMesh(rect, mesh) in meshes.iter() {
      self.indexes.extend_from_slice(&mesh.indices);

      // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.chunks
      for vertex in &mesh.vertices {
        self.push(vertex, mesh.texture_id, rect.clone());
      }

      // Start new buffer
    }
  }

  fn push(&mut self, vertex: &Vertex, texture_id: TextureId, clip: Rect) {
    if self.buffers.is_empty() {
      self.buffers.push(RenderBuffer::new(texture_id, clip));
    }

    let buffer = self.buffers.get_mut(self.cursor).unwrap();
    buffer.push(vertex);

    // if buffer.cursor >= Self::MAX_VERTEXES {
    //   self.buffers.push(RenderBuffer::new(texture_id, clip));
    //   self.cursor += 1;
    //   self.push(vertex, texture_id, clip);
    // } else {
    //   buffer.push(vertex);
    // }
  }

  fn clear(&mut self) {
    for buffer in self.buffers.iter_mut() {
      buffer.clear();
    }

    self.indexes.clear();
  }
}
