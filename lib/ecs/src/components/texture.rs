
#[derive(Default)]
pub struct Texture {
  pub id: String,
  pub width: u32,
  pub height: u32,
}

impl Texture {
  pub fn new(id: &str, width: u32, height: u32) -> Self {
    Self {
      id: String::from(id),
      width,
      height,
    }
  }
}