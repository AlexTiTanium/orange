use std::num::NonZeroU64;

use super::Vertex;
use common::{log, AllStoragesViewMut, UniqueView, UniqueViewMut};
use webgpu::api::{self as wgpu, BufferDescriptor};
use webgpu::{Pipeline, Pipelines, WebGpuState};
use wgpu::{BufferUsages, PipelineLayoutDescriptor};

pub static SHADER: &str = include_str!("./shader_editor.wgsl");

pub struct EditorRenderer;

///
/// Editor renderer pipeline
///
impl EditorRenderer {
  ///
  /// Create a new editor renderer pipeline
  ///
  pub fn init(all_storages: AllStoragesViewMut) {
    log::info!("Editor renderer init starts");

    let (webgpu, mut pipelines) = all_storages
      .borrow::<(UniqueViewMut<WebGpuState>, UniqueViewMut<Pipelines>)>()
      .unwrap();

    const VERTEX_BUFFER_START_CAPACITY: wgpu::BufferAddress = (std::mem::size_of::<Vertex>() * 1024) as _;
    const INDEX_BUFFER_START_CAPACITY: wgpu::BufferAddress = (std::mem::size_of::<u32>() * 1024) as _;

    let device = &webgpu.device;
    let config = &webgpu.config;

    let vertex_buffer = device.create_buffer(&BufferDescriptor {
      label: Some("Editor Vertex Buffer"),
      size: VERTEX_BUFFER_START_CAPACITY,
      usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    // Create index buffer
    let index_buffer = device.create_buffer(&BufferDescriptor {
      label: Some("Editor Index Buffer"),
      size: INDEX_BUFFER_START_CAPACITY,
      usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
      label: Some("Editor Render Pipeline Layout"),
      bind_group_layouts: &[],
      push_constant_ranges: &[],
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Editor Shader"),
      source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });

    // Create a render pipeline with the given descriptor
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      // Set a human-readable label for easy debugging purposes
      label: Some("Editor Render Pipeline"),
      // Use the provided layout for this pipeline
      layout: Some(&pipeline_layout),
      // Describe how vertex data will be handled by the pipeline
      vertex: wgpu::VertexState {
        // Provide the shader module containing the vertex shader
        module: &shader,
        // Specify the entry point function in the shader code
        entry_point: "vs_main",
        // Declare no buffer bindings for this pipeline pass
        buffers: &[Vertex::desc()],
      },
      // Describe how fragment data (color) will be handled by the pipeline
      fragment: Some(wgpu::FragmentState {
        // Provide the shader module containing the fragment shader
        module: &shader,
        // Specify the entry point function in the shader code
        entry_point: "fs_main",
        // Define an array of color target states, one per render target
        targets: &[Some(wgpu::ColorTargetState {
          // Specify the format of the color output
          format: config.format,
          // Enable replace blending operation between source and destination values
          blend: Some(wgpu::BlendState::REPLACE),
          // Enable writing to all rgba color channels
          write_mask: wgpu::ColorWrites::ALL,
        })],
      }),
      // Determine how tessellation or geometry shader stages should behave
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList, // 1.
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw, // 2.
        cull_mode: Some(wgpu::Face::Back),
        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
        polygon_mode: wgpu::PolygonMode::Fill,
        // Requires Features::DEPTH_CLIP_CONTROL
        unclipped_depth: false,
        // Requires Features::CONSERVATIVE_RASTERIZATION
        conservative: false,
      },
      // Configure depth/stencil testing when enabled
      depth_stencil: None,
      // Configure multisampling settings for sub-pixel color averaging
      multisample: wgpu::MultisampleState {
        // Set the number of samples per pixel
        count: 1,
        // Enable all sample masks by default
        mask: !0,
        // Disable alpha-to-coverage mapping
        alpha_to_coverage_enabled: false,
      },
      // Plan for producing multiple related views in advanced scenarios
      multiview: None,
    });

    // Create editor render
    let renderer = Pipeline {
      num_indices: 0,
      vertex_buffer,
      index_buffer,
      pipeline_layout,
      render_pipeline,
    };

    // Add editor render to pipeline resource
    pipelines.add("editor", renderer);

    log::info!("Editor renderer created");
  }

  ///
  /// Draw
  ///
  pub fn draw(mut pipelines: UniqueViewMut<Pipelines>, webgpu: UniqueView<WebGpuState>) {
    let pipeline = pipelines.get_mut("editor");

    log::info!("Editor renderer draw starts");

    # [rustfmt::skip]
    const VERTICES: &[Vertex] = &[
      Vertex { position: [-0.0868241, 0.49240386],    color: [0.5, 0.0, 0.5] }, // A
      Vertex { position: [-0.49513406, 0.06958647],   color: [0.5, 0.0, 0.5] }, // B
      Vertex { position: [-0.21918549, -0.44939706],  color: [0.5, 0.0, 0.5] }, // C
      Vertex { position: [0.35966998, -0.3473291],    color: [0.5, 0.0, 0.5] }, // D
      Vertex { position: [0.44147372, 0.2347359],     color: [0.5, 0.0, 0.5] }, // E
    ];

    # [rustfmt::skip]
    const INDICES: &[u32] = &[
      0, 1, 4,
      1, 2, 4,
      2, 3, 4,
    ];

    pipeline.num_indices = INDICES.len() as u32;

    let indexes_size: u64 = (std::mem::size_of::<u32>() * INDICES.len()) as u64;
    let vertex_size: u64 = (std::mem::size_of::<Vertex>() * VERTICES.len()) as u64;

    let mut vertexes = webgpu
      .queue
      .write_buffer_with(&pipeline.vertex_buffer, 0, NonZeroU64::new(vertex_size).unwrap())
      .expect("Failed to create staging buffer for vertex data");

    vertexes.copy_from_slice(bytemuck::cast_slice(VERTICES));

    let mut indexes = webgpu
      .queue
      .write_buffer_with(&pipeline.index_buffer, 0, NonZeroU64::new(indexes_size).unwrap())
      .expect("Failed to create staging buffer for index data");

    indexes.copy_from_slice(bytemuck::cast_slice(INDICES));
  }
}
