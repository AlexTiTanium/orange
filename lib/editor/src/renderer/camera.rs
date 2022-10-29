use common::UniqueViewMut;
use math::{identity, inverse, ortho, translate, vec3, Mat4, Vec3};
use window::WindowSize;

pub struct EditorCamera {
  pub position: Vec3,
  pub projection: Mat4,
  pub view_projection: Mat4,
}

impl Default for EditorCamera {
  fn default() -> Self {
    Self {
      position: vec3(0.0, 0.0, 0.0),
      projection: ortho(0.0, 0.0, 0.0, 0.0, -1.0, 1.0),
      view_projection: identity(),
    }
  }
}

impl EditorCamera {
  pub fn resize(&mut self, width: f32, height: f32) {
    self.projection = ortho(0.0, width, height, 0.0, -1.0, 1.0);
    let translation = translate(&identity(), &self.position);
    self.view_projection = self.projection * inverse(&translation);
    println!("{:?}", self.view_projection)
  }

  pub fn init(mut camera: UniqueViewMut<EditorCamera>, window: UniqueViewMut<WindowSize>) {
    camera.resize(window.logical.width, window.logical.height);
  }

  // pub fn update(mut camera: UniqueViewMut<EditorCamera>, events: UniqueView<Events<WindowResizeEvent>>, window: UniqueViewMut<WindowSize>) {
  //   let mut reader = events.get_reader();

  //   for _ in reader.iter(&events) {
  //     camera.resize(window.logical.width, window.logical.height);
  //   }
  // }
}
