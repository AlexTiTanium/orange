use crate::renderer::EditorRenderPlugin;
use crate::resources::EditorBatchBuffer;
use common::{stage, Builder, Plugin, UniqueViewMut};
use egui::{self, Rect, Vec2};
use window::WindowSize;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
  fn build(&self, app: &mut Builder) {
    app
      .add_resource(egui::CtxRef::default())
      .add_resource(egui::RawInput::default())
      .add_resource(EditorBatchBuffer::default())
      .add_startup_system(&init_egui)
      .add_to_stage(stage::PRE_UPDATE, &begin_frame)
      .add_to_stage(stage::POST_UPDATE, &end_frame)
      .add_system(&build_ui)
      .add_plugin(EditorRenderPlugin);
  }
}

fn init_egui(mut input: UniqueViewMut<egui::RawInput>, window: UniqueViewMut<WindowSize>) {
  input.screen_rect = Some(Rect::from_min_size(
    Default::default(),
    Vec2::new(window.logical.width, window.logical.height),
  ));

  input.pixels_per_point = Some(window.scale as f32);

  println!("Startup input {:?}", input.clone());
}

fn begin_frame(mut ctx: UniqueViewMut<egui::CtxRef>, input: UniqueViewMut<egui::RawInput>) {
  ctx.begin_frame(input.clone());
}

fn end_frame(ctx: UniqueViewMut<egui::CtxRef>, mut batch: UniqueViewMut<EditorBatchBuffer>) {
  let (_, shapes) = ctx.end_frame();
  let clipped_meshes = ctx.tessellate(shapes);

  batch.set_data(&clipped_meshes, &ctx.texture());
}

fn build_ui(ctx: UniqueViewMut<egui::CtxRef>) {
  egui::CentralPanel::default().show(&ctx, |ui| {
    ui.label("Hello world!");
    if ui.button("Click me").clicked() { /* take some action here */ }
  });
}
