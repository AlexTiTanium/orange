use common::{Pod, Zeroable};
use webgpu::api as wgpu;

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
pub struct Vertex {
  pub position: [f32; 2],
  pub color: [f32; 3],
}

impl Vertex {
  # [rustfmt::skip]
  const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
    0 => Float32x2,
    1 => Float32x3
  ];

  pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    use std::mem;

    wgpu::VertexBufferLayout {
      array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIBUTES,
    }
  }
}