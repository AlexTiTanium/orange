use crate::glm::*;
use crate::resources::Window;
use crate::Unique;
use crate::World;
use winit::event::*;

pub fn handle_camera_update(world: &World) {
  let window = world.borrow::<Unique<&Window>>();
  let mut camera = world.borrow::<Unique<&mut Camera>>();
  camera.projection = ortho(0.0, window.logical.width, 0.0, window.logical.height, -1.0, 1.0);
  camera.update();
}

pub fn handle_camera_resize(world: &World, event: &WindowEvent) {
  if let WindowEvent::Resized(_) = event {
    handle_camera_update(world);
  }

  if let WindowEvent::ScaleFactorChanged {
    scale_factor: _,
    new_inner_size: _,
  } = event
  {
    handle_camera_update(world);
  }
}

pub struct Camera {
  pub position: Vec3,
  pub projection: Mat4,
  view: Mat4,
  pub view_projection: Mat4,
}

impl Default for Camera {
  fn default() -> Self {
    Self {
      position: vec3(0.0, 0.0, 0.0),
      projection: ortho(0.0, 0.0, 0.0, 0.0, -1.0, 1.0),
      view: identity(),
      view_projection: identity(),
    }
  }
}

impl Camera {
  pub fn update(&mut self) {
    let translation = translate(&identity(), &self.position); // TODO: rotation
    self.view = inverse(&translation);
    self.view_projection = self.projection * self.view;
  }

  pub fn update_position(&mut self, x: f32, y: f32) {
    self.position.x = x;
    self.position.y = y;
    self.update();
  }
}
