#[derive(Default)]
pub struct Display {
  pub width: u32,
  pub height: u32,
}

impl Display {
  pub fn new(width: u32, height: u32) -> Self {
    Self { width, height }
  }

  pub fn update(&mut self, width: u32, height: u32) {
    self.width = width;
    self.height = height;
  }
}
