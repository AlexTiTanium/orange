use crate::level;
use ecs::State;

pub fn start(state: &State) {
  log::info!("Game start");

  level::load(state, "maps/level_3.tmx", vec!["textures/winter.xml"]);

  // Load textures on GPU
  render::load_textures(state);
}
