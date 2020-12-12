#[derive(Default)]
pub struct Texture {
  pub slot: i32,
  pub id: String,
  pub width: u32,
  pub height: u32,
}

impl Texture {
  pub fn new(slot: i32, id: &str, width: u32, height: u32) -> Self {
    Self {
      id: String::from(id),
      slot,
      width,
      height,
    }
  }
}
