mod structures;
use structures::map::Map;
use structures::tileset::Tileset;
use structures::atlas::TextureAtlas;

use ecs::components::{ Image, Tile, Sprite, NoSpriteTag };
use ecs::resources::Assets;
use ecs::{State, UniqueViewMut, ViewMut, EntitiesViewMut};

use quick_xml::de::from_str;

pub fn load(state: &State, level: &str, atlases: Vec<&str>) {
  let mut assets = state.world.borrow::<UniqueViewMut<Assets>>();

  let map: Map = from_str(assets.load_text(level)).unwrap();
  let mut sprites: Vec<Sprite> = Vec::new();

  // Load textures to esc and memory
  for (slot, path) in atlases.iter().enumerate() {
    let atlas: TextureAtlas = from_str(assets.load_text(&path)).unwrap();

    assets.load_texture(slot as u32, &atlas.source);

    for sprite in atlas.sprites {
      sprites.push(
        Sprite {
          slot: slot as u32,
          source: sprite.source,
          x: sprite.x,
          y: sprite.y,
          width: sprite.width,
          height: sprite.height,
          rotated: sprite.rotated
        }
      );
    }
  }

  // Load tiles to esc
  for tileset in &map.tilesets {
    let tileset: Tileset = from_str(assets.load_text(&tileset.source)).unwrap();
    for tile in tileset.tiles {
      let tile_sprite = sprites.iter().find(|&item| item.source == tile.image.source);

      state.world.run(|mut entities: EntitiesViewMut, mut tile_comp: ViewMut<Tile>, mut image: ViewMut<Image>, mut sprite: ViewMut<Sprite>, mut no_sprite: ViewMut<NoSpriteTag>| {

        let entity_id = entities.add_entity((&mut tile_comp, &mut image), (
          Tile { id: tile.id },
          Image { source: tile.image.source, width: tile.image.width, height: tile.image.height }
        ));

        match tile_sprite {
          Some(s) => entities.add_component(&mut sprite, s.clone(), entity_id),
          None => entities.add_component(&mut no_sprite, NoSpriteTag, entity_id)
        }

      }); // RUN ---------------------------
    }
  }

  //panic!("close");
}