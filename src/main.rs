use common::Application;
// use editor::EditorPlugin;
use input::InputPlugin;
use webgpu::WebGpuPlugin;
use window::WindowPlugin;

fn main() {
  // Build and start app
  Application::build()
    .add_plugin(WindowPlugin)
    .add_plugin(InputPlugin)
    .add_plugin(WebGpuPlugin)
    //.add_plugin(EditorPlugin)
    .run();
}
