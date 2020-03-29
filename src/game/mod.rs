use ecs::resources::*;
use ecs::*;

pub fn start(state: &State) {
  log::info!("Game start");

  let mut assets = state.world.borrow::<Unique<&mut Assets>>();

  assets.load_image("tree", "tree.png");
  assets.load_image("cat", "cat.png");
}
