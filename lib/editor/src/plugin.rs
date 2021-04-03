use common::{Builder, Plugin, UniqueViewMut};
use egui;
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
  fn build(&self, app: &mut Builder) {
    app.add_resource(egui::CtxRef::default()).add_system(&build_ui);
  }
}

fn build_ui(mut ctx: UniqueViewMut<egui::CtxRef>) {
  let input = egui::RawInput::default();
  ctx.begin_frame(input);

  egui::CentralPanel::default().show(&ctx, |ui| {
    ui.label("Hello world!");
    if ui.button("Click me").clicked() { /* take some action here */ }
  });

  //let (_, shapes) = ctx.end_frame();
  //let clipped_meshes = ctx.tessellate(shapes); // create triangles to paint

  //println!("{:?}", clipped_meshes);
}
