#[derive(Default, Clone)]
pub struct Sprite {
  pub slot: u32,
  pub x: f32,
  pub y: f32,
  pub source: String,
  pub width: u32,
  pub height: u32,
  pub rotated: bool
}