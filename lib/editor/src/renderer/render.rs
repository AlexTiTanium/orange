use common::{NonSendSync, UniqueViewMut};
use opengl::{Gl, Renderer, ShaderType, GL};
use std::str;

use crate::resources::EditorBatchBuffer;

use super::camera::EditorCamera;

pub static SHADER_COLOR_VERT: &str = include_str!("./shader_editor_vert.glsl");
pub static SHADER_COLOR_FRAG: &str = include_str!("./shader_editor_frag.glsl");

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
      .set_indexes_buffer_size(9000000)
      .set_vertices_buffer_size(900000)
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

  pub fn draw(mut editor: NonSendSync<UniqueViewMut<EditorRenderer>>, buffer: UniqueViewMut<EditorBatchBuffer>, camera: UniqueViewMut<EditorCamera>) {
    let textures = &buffer.textures;

    unsafe {
      editor.renderer.gl.Enable(GL::SCISSOR_TEST);
      editor.renderer.gl.Disable(GL::CULL_FACE);
      editor.renderer.gl.Disable(GL::DEPTH_TEST);
      editor.renderer.gl.Enable(GL::BLEND);

      editor.renderer.gl.BlendEquationSeparate(GL::FUNC_ADD, GL::FUNC_ADD);
      editor.renderer.gl.BlendFuncSeparate(
        // egui outputs colors with premultiplied alpha:
        GL::ONE,
        GL::ONE_MINUS_SRC_ALPHA,
        // Less important, but this is technically the correct alpha blend function
        // when you want to make use of the framebuffer alpha (for screenshots, compositing, etc).
        GL::ONE_MINUS_DST_ALPHA,
        GL::ONE,
      );
    }

    for buff in &buffer.buffers {
      let texture = textures.get(&buff.texture_id).unwrap();

      editor
        .renderer
        .set_texture_data_for_slot_srgb(0, texture.width, texture.height, &texture.pixels);

      editor.renderer.set_view_projection(&camera.view_projection);
      editor.renderer.set_indexes(&buff.indexes);
      editor.renderer.set_vertices(&buff.vb);
      editor.renderer.activate_texture_slot(0);

      editor.renderer.bind();
      //editor.renderer.debug();
      editor.renderer.draw();
    }

    // exit(0);
  }
}
