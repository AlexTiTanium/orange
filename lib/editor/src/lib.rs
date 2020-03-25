use ecs::State;
use glutin::event::Event;
use glutin::window::Window;
use imgui::{Context, Ui};
use imgui_opengl_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::ffi::c_void;
use std::time::Instant;
mod ui;
pub(crate) use nalgebra_glm as glm;

pub struct Editor {
    context: Context,
    renderer: Renderer,
    platform: WinitPlatform,
    delta: Instant,
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

        Self {
            context,
            renderer,
            platform,
            delta: Instant::now(),
        }
    }

    pub fn handle_event(&mut self, window: &Window, event: &Event<()>) {
        self.platform.handle_event(self.context.io_mut(), &window, &event);
    }

    pub fn update(&mut self) {
        self.delta = self.context.io_mut().update_delta_time(self.delta);
    }

    pub fn prepare_frame(&mut self, window: &Window) {
        self.platform
            .prepare_frame(self.context.io_mut(), window)
            .expect("Failed to prepare frame");
    }

    pub fn step(&mut self, state: &State, window: &Window) {
        let ui = self.context.frame();

        self.platform.prepare_render(&ui, &window);

        build_ui(&ui, &state);

        self.renderer.render(ui);
    }
}

fn build_ui(ui: &Ui, state: &State) {
    ui::test::build(&ui, &state);
}
