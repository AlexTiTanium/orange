use common::Application;
use window::WindowPlugin;

fn main() {
  Application::build().add_plugin(WindowPlugin).run();
}
