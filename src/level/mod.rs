mod map;

use ecs::resources::Assets;
use ecs::{State, UniqueViewMut};

use map::Map;
use quick_xml::de::from_str;

pub fn load(state: &State, level: &str) {
  let mut assets = state.world.borrow::<UniqueViewMut<Assets>>();

  assets.load_string(level);

  let level_data = assets.get_string(level);

  let map: Map = from_str(level_data).unwrap();

  for tileset in &map.tilesets {
    eprintln!("tileset = {:#?}", tileset);
  }

  //print!("{:?} ", map);
  //panic!("close");
}
