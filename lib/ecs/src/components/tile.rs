use shipyard::EntityId;
#[derive(Default)]
pub struct Tile {
  pub id: u32,
}

pub struct TileRef(pub EntityId);