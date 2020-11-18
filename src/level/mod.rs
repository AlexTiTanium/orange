mod map;
mod tileset;
pub mod decoders;

use std::path::Path;

use ecs::resources::Assets;
use ecs::{State, UniqueViewMut};

use map::Map;
use tileset::Tileset;

use quick_xml::de::from_str;

pub fn load(state: &State, level: &str) {
  let mut assets = state.world.borrow::<UniqueViewMut<Assets>>();

  let level_data = assets.load_text(level);
  let map: Map = from_str(level_data).unwrap();

  for tileset in &map.tilesets {
    let tileset_data = assets.load_text(get_tileset_path(&tileset.source));
    let tileset: Tileset = from_str(tileset_data).unwrap();

    //println!("{}", tileset_path.display());

    //assets.load_text(tileset.source);
    eprintln!("tileset = {:#?}", tileset);
  }

  //print!("{:?} ", map);
  panic!("close");
}

fn get_tileset_path(source: &String) -> &str {
  return Path::new(source).strip_prefix("../").unwrap().to_str().unwrap();
}
