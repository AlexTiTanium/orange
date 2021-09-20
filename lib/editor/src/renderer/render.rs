use common::{NonSendSync, UniqueView, UniqueViewMut};
use egui::ClippedMesh;
use opengl::{render, Gl, Renderer, ShaderType, GL};
use std::{mem, str};

use crate::resources::EditorBatchBuffer;

use super::camera::EditorCamera;

pub static SHADER_COLOR_VERT: &str = include_str!("./shader_color_vert.glsl");
pub static SHADER_COLOR_FRAG: &str = include_str!("./shader_color_frag.glsl");

pub struct EditorRenderer {
  renderer: Renderer,
}

impl EditorRenderer {
  pub fn new(gl: &Gl) -> Self {
    Self { renderer: Self::create(&gl) }
  }

  fn create(gl: &Gl) -> Renderer {
    let mut renderer = Renderer::new(gl);

    let texture = renderer.create_texture();
    texture.bind();
    texture.set_param(GL::TEXTURE_2D, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE);
    texture.set_param(GL::TEXTURE_2D, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE);
    texture.set_param(GL::TEXTURE_2D, GL::TEXTURE_MIN_FILTER, GL::LINEAR);
    texture.set_param(GL::TEXTURE_2D, GL::TEXTURE_MAG_FILTER, GL::LINEAR);

    renderer
      .set_indexes_buffer_size(EditorBatchBuffer::MAX_INDEXES_SIZE)
      .set_vertices_buffer_size(EditorBatchBuffer::MAX_VBO_SIZE)
      .add_layout::<f32>(2) // Loc = 0 position
      .add_layout::<f32>(4) // Loc = 1 color
      .add_layout::<f32>(2) // Loc = 2 uv
      .build_layout()
      .add_shader(ShaderType::Vertex, SHADER_COLOR_VERT)
      .add_shader(ShaderType::Fragment, SHADER_COLOR_FRAG)
      .build_shader()
      .bind_texture_slot(0, texture)
      .create_uniform("u_Texture")
      .create_mvp();

    renderer
  }

  pub fn set_buffer(&mut self, buffer: &EditorBatchBuffer) {
    self.renderer.set_indexes(&buffer.indexes);
    //self.renderer.set_vertices(&buffer.buffers);
  }

  //pub fn update(mut editor: NonSendSync<UniqueViewMut<EditorRenderer>>, camera: UniqueViewMut<EditorCamera>, ctx: UniqueView<egui::CtxRef>) {
  // let (_, shapes) = ctx.end_frame();
  // let clipped_meshes = ctx.tessellate(shapes); // create triangles to paint
  // editor.renderer.set_view_projection(&camera.view_projection);

  // for i in &mut editor.buffer {
  //   *i = 0.0
  // }

  // for ClippedMesh(_, mesh) in clipped_meshes.iter() {
  //   //println!("______ {:?}", mesh.vertices.len());
  //   //panic!("oops");
  //   let mut index = 0;

  //   //mesh.texture_id

  //   editor.buffer.resize(mesh.vertices.len() * 8, 0.0);

  //   for vertex in &mesh.vertices {
  //     editor.buffer[index] = vertex.pos.x;
  //     editor.buffer[index + 1] = vertex.pos.y;

  //     // TODO: Use u8
  //     editor.buffer[index + 2] = vertex.color.r() as f32;
  //     editor.buffer[index + 3] = vertex.color.g() as f32;
  //     editor.buffer[index + 4] = vertex.color.b() as f32;
  //     editor.buffer[index + 5] = vertex.color.a() as f32;

  //     editor.buffer[index + 6] = vertex.uv.x;
  //     editor.buffer[index + 7] = vertex.uv.y;

  //     index += 8;
  //   }

  //   //println!("{:?}", mesh);
  // }

  // let egui_texture = ctx.texture();
  // editor
  //   .renderer
  //   .set_texture_data_for_slot(0, egui_texture.width, egui_texture.height, &egui_texture.pixels);
  //}

  pub fn draw(mut editor: NonSendSync<UniqueViewMut<EditorRenderer>>, buffer: UniqueViewMut<EditorBatchBuffer>) {
    editor.set_buffer(&buffer);

    // buffer.set_data(meshes, texture);

    //editor.renderer.bind();
    //editor.renderer.activate_texture_slot(0);
    //editor.renderer.debug();
    //let data = editor.buffer.clone();
    //editor.renderer.set_data(&data);
    // editor.renderer.draw();
  }
}
