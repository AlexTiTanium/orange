use ecs::components::*;
use ecs::resources::*;
use ecs::{EntitiesView, State, UniqueViewMut, ViewMut};

pub fn start(state: &State) {
  log::info!("Game start");

  let mut assets = state.world.borrow::<UniqueViewMut<Assets>>();

  assets.load_texture(0, "tree.png");
  assets.load_texture(1, "cat.png");

  let entity_id = state.create_game_object();
  let entity_id2 = state.create_game_object();

  let (entities, mut textures) = state.world.borrow::<(EntitiesView, ViewMut<Texture>)>();

  entities.add_component(&mut textures, Texture::new("tree.png", 32, 32), entity_id);
  entities.add_component(&mut textures, Texture::new("cat.png", 32, 32), entity_id2);
}
