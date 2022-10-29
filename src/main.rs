use common::Application;
use editor::EditorPlugin;
use input::InputPlugin;
use opengl::RenderPlugin;
use window::WindowPlugin;

fn main() {
  Application::build()
    .add_plugin(WindowPlugin)
    .add_plugin(InputPlugin)
    .add_plugin(RenderPlugin)
    .add_plugin(EditorPlugin)
    .run();
}
