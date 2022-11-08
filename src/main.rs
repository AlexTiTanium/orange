use common::Application;
use input::InputPlugin;
use webgpu::WebGpuPlugin;
use window::WindowPlugin;

fn main() {
  // Build and start app instance
  Application::build()
    .add_plugin(WindowPlugin)
    .add_plugin(InputPlugin)
    .add_plugin(WebGpuPlugin)
    .run();
}
