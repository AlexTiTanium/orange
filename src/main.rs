use common::Application;
use editor::EditorPlugin;
use opengl::RenderPlugin;
use window::WindowPlugin;

fn main() {
  Application::build()
    .add_plugin(WindowPlugin)
    .add_plugin(EditorPlugin)
    .add_plugin(RenderPlugin)
    .run();
}
