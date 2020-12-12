use game::EntityId;

#[derive(Clone)]
pub struct Sprite {
  pub texture: EntityId,
  pub x: f32,
  pub y: f32,
  pub source: String,
  pub width: u32,
  pub height: u32,
  pub rotated: bool,
  pub uv: [[f32; 2]; 4],
}
