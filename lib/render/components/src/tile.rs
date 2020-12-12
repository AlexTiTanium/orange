use game::EntityId;
#[derive(Default)]
pub struct Tile {
  pub id: u32,
}

pub struct TileRef(pub EntityId);

impl TileRef {
  pub fn entity_id(&self) -> EntityId {
    self.0
  }
}
