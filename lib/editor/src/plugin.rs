use crate::renderer::EditorRenderPlugin;
use crate::resources::EditorBatchBuffer;
use common::{stage, Builder, Plugin, UniqueView, UniqueViewMut};
use egui::{self, Pos2, Rect, Vec2};
use input::{Input, InputEvent, InputPosition};
use window::WindowSize;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
  fn build(&self, app: &mut Builder) {
    app
      .add_resource(egui::Context::default())
      .add_resource(egui::RawInput::default())
      .add_resource(EditorBatchBuffer::default())
      .add_startup_system(&init_egui)
      .add_to_stage(stage::PRE_UPDATE, &begin_frame)
      .add_to_stage(stage::POST_UPDATE, &end_frame)
      .add_to_stage(stage::POST_RENDER, &finish_render)
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

fn begin_frame(mut ctx: UniqueViewMut<egui::Context>, mut egui_input: UniqueViewMut<egui::RawInput>, input: UniqueView<Input>) {
  egui_input.take();

  for event in &input.events {
    match event {
      InputEvent::PointerMoved(InputPosition(x, y)) => {
        egui_input.events.push(egui::Event::PointerMoved(Pos2 { x: *x as f32, y: *y as f32 }));
      }
    }
  }

  ctx.begin_frame(egui_input.clone());

  // println!("Input {:?}", input.events);
}

fn end_frame(ctx: UniqueViewMut<egui::Context>, mut batch: UniqueViewMut<EditorBatchBuffer>) {
  let output = ctx.end_frame();
  let clipped_meshes = ctx.tessellate(output.shapes);

  batch.set_data(&clipped_meshes, &output.textures_delta);
}

fn finish_render(mut batch: UniqueViewMut<EditorBatchBuffer>) {
  batch.free_texures_delta();
}

fn build_ui(ctx: UniqueViewMut<egui::Context>) {
  egui::Window::new("My window").show(&ctx, |ui| {
    let mut name = "alex";
    let mut age = 10;

    ui.heading("My egui Application");
    ui.horizontal(|ui| {
      ui.label("Your name: ");
      ui.text_edit_singleline(&mut name);
    });

    ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
    if ui.button("Click each year").clicked() {
      age += 1;
    }

    ui.label(format!("Hello '{}', age {}", name, age));
  });

  // egui::CentralPanel::default().show(&ctx, |ui| {
  //   ui.heading("My egui Application");
  //   ui.horizontal(|ui| {
  //     ui.label("Your name: ");
  //     ui.text_edit_singleline(&mut name);
  //   });
  //   ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
  //   if ui.button("Click each year").clicked() {
  //     age += 1;
  //   }
  //   ui.label(format!("Hello '{}', age {}", name, age));
  // });

  // egui::CentralPanel::default().show(&ctx, |ui| {
  //   ui.heading("My egui Application");
  //   ui.horizontal(|ui| {
  //     ui.label("Your name: ");
  //   });
  // });
}
