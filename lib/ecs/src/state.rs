use crate::entities::Display;
use crate::*;

#[derive(Default)]
pub struct State {
  pub world: World,
}

impl State {
  pub fn new() -> Self {
    let world = World::new();

    Self { world }
  }

  pub fn create_entities(&self) {}

  pub fn create_resources(&self) {
    self.world.add_unique(Display::new(100, 100));
  }

  pub fn update_display(&self, width: u32, height: u32) {
    let mut display = self.world.borrow::<Unique<&mut Display>>();
    display.update(width, height);
  }
}
