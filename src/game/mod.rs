use ecs::components::*;
use ecs::resources::*;
use ecs::*;

pub fn start(state: &State) {
  log::info!("Game start");

  let mut assets = state.world.borrow::<Unique<&mut Assets>>();

  assets.load_image("tree", "tree.png");
  assets.load_image("cat", "cat.png");

  let entity_id = state.create_game_object();
  let (entities, mut textures) = state.world.borrow::<(Entities, &mut Texture)>();
  entities.add_component(&mut textures, Texture::new("tree", 32, 32), entity_id);
}
