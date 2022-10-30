use crate::resources::EditorBatchBuffer;
use crate::{convertors::translate_cursor, renderer::EditorRenderPlugin};
use common::{stage, Builder, NonSendSync, Plugin, UniqueView, UniqueViewMut};
use egui::{self, Modifiers, Pos2, Rect, Vec2};
use input::{Input, InputEvent, InputPosition, PointerButton};
use window::{WindowContext, WindowSize};

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

///
/// Init egui
///
fn init_egui(mut input: UniqueViewMut<egui::RawInput>, window: UniqueViewMut<WindowSize>) {
  input.screen_rect = Some(Rect::from_min_size(
    Default::default(),
    Vec2::new(window.logical.width, window.logical.height),
  ));

  input.pixels_per_point = Some(window.scale as f32);
}

///
/// On each frame
///
fn begin_frame(ctx: UniqueView<egui::Context>, mut egui_input: UniqueViewMut<egui::RawInput>, input: UniqueView<Input>) {
  egui_input.take();

  for event in &input.events {
    match event {
      InputEvent::PointerMoved(InputPosition(x, y)) => {
        egui_input.events.push(egui::Event::PointerMoved(Pos2 { x: *x as f32, y: *y as f32 }));
      }
      InputEvent::PointerButton {
        pos: InputPosition(x, y),
        button,
        pressed,
      } => {
        egui_input.events.push(egui::Event::PointerButton {
          pos: Pos2 { x: *x as f32, y: *y as f32 },
          button: match button {
            PointerButton::Primary => egui::PointerButton::Primary,
            PointerButton::Secondary => egui::PointerButton::Secondary,
            PointerButton::Middle => egui::PointerButton::Middle,
            PointerButton::Extra1 => egui::PointerButton::Extra1,
            PointerButton::Extra2 => egui::PointerButton::Extra2,
          },
          pressed: *pressed,
          modifiers: Modifiers {
            alt: false,
            ctrl: false,
            shift: false,
            mac_cmd: false,
            command: false,
          },
        });
      }
    }
  }

  ctx.begin_frame(egui_input.clone());
}

///
/// Prepare mesh data for render
///
fn end_frame(ctx: UniqueViewMut<egui::Context>, mut batch: UniqueViewMut<EditorBatchBuffer>, window: NonSendSync<UniqueViewMut<WindowContext>>) {
  let output = ctx.end_frame();
  let clipped_meshes = ctx.tessellate(output.shapes);
  let egui_icon = output.platform_output.cursor_icon;
  let window_icon = translate_cursor(egui_icon);

  match window_icon {
    Some(icon) => window.set_cursor(icon),
    None => window.set_cursor(window::CursorIcon::Default),
  }

  batch.set_data(&clipped_meshes, &output.textures_delta);
}

///
/// Finish render frame, clear resorcess
///
fn finish_render(mut batch: UniqueViewMut<EditorBatchBuffer>) {
  batch.free_texures_delta();
}

///
/// Just experemental UI example to proof render and input works
///
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
}
