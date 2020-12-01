use game::State;
use imgui::{Context, Ui};
use imgui_opengl_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::ffi::c_void;
use std::time::Instant;
use winit::event::Event;
use winit::window::Window;

mod ui;

pub struct Editor {
  pub context: Context,
  renderer: Renderer,
  platform: WinitPlatform,
  delta: Instant,
  state: UiState,
}

#[derive(Default)]
pub struct UiState {
  show_entities_control: bool,
  show_tiles_list: bool,
  show_camera_control: bool,
  show_window_info: bool,
}

impl Editor {
  pub fn new<F>(window: &Window, load: F) -> Self
  where
    F: FnMut(&'static str) -> *const c_void,
  {
    let mut context = Context::create();
    context.fonts().build_rgba32_texture();

    let renderer = Renderer::new(&mut context, load);

    let mut platform = WinitPlatform::init(&mut context); // step 1
    platform.attach_window(context.io_mut(), window, HiDpiMode::Default); // step 2

    let state = UiState::default();

    Self {
      context,
      renderer,
      platform,
      delta: Instant::now(),
      state,
    }
  }

  pub fn handle_event(&mut self, window: &Window, event: &Event<()>) {
    self.platform.handle_event(self.context.io_mut(), window, event);
  }

  pub fn update(&mut self) {
    self.delta = self.context.io_mut().update_delta_time(self.delta);
  }

  pub fn prepare_frame(&mut self, window: &Window) {
    self
      .platform
      .prepare_frame(self.context.io_mut(), window)
      .expect("Failed to prepare frame");
  }

  pub fn step(&mut self, state: &State, window: &Window) {
    let ui = self.context.frame();

    self.platform.prepare_render(&ui, &window);

    build_ui(&ui, &state, &mut self.state);

    self.renderer.render(ui);
  }
}

fn build_ui(ui: &Ui, state: &State, ui_state: &mut UiState) {
  ui::main_menu::build(ui, state, ui_state);

  if ui_state.show_entities_control {
    ui::entities_control::build(ui, state);
  }

  if ui_state.show_camera_control {
    ui::camera_control::build(ui, state);
  }

  if ui_state.show_window_info {
    ui::window_info::build(ui, state);
  }

  if ui_state.show_tiles_list {
    ui::tiles_list::build(ui, state);
  }
}
