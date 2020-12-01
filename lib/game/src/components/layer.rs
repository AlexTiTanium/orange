use shipyard::EntityId;

#[derive(Default, Clone)]
pub struct Layer {
  pub id: u32,
  pub name: String,
  pub width: u32,
  pub height: u32,
  pub render_index: u32
}

pub struct LayerRef(pub EntityId);

