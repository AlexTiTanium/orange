mod structures;
use std::collections::HashMap;

use structures::atlas::TextureAtlas;
use structures::map::Chunk;
use structures::map::Map;
use structures::tileset::Tileset;

use ecs::components::{Group, Image, Layer, LayerRef, Map as MapComp, NoSpriteTag, Sprite, Tile, TileRef, Transform};
use ecs::resources::Assets;
use ecs::{EntitiesViewMut, EntityId, State, UniqueViewMut, ViewMut};

use quick_xml::de::from_str;

struct TileCoordinates {
  pub id: u32,
  pub x: f32,
  pub y: f32,
}

pub fn load(state: &State, level: &str, atlases: Vec<&str>) {
  let mut assets = state.world.borrow::<UniqueViewMut<Assets>>();

  let map: Map = from_str(assets.load_text(level)).unwrap();
  let mut sprites: Vec<Sprite> = Vec::new();
  let mut tiles: HashMap<u32, EntityId> = HashMap::new();

  // Load textures to esc and memory
  for (slot, path) in atlases.iter().enumerate() {
    let atlas: TextureAtlas = from_str(assets.load_text(&path)).unwrap();

    assets.load_texture(slot as u32, &atlas.source);

    for sprite in atlas.sprites {
      sprites.push(Sprite {
        slot: slot as u32,
        source: sprite.source,
        x: sprite.x,
        y: sprite.y,
        width: sprite.width,
        height: sprite.height,
        rotated: sprite.rotated,
      });
    }
  }

  // Load tiles to esc
  for map_tileset in &map.tilesets {
    let tileset: Tileset = from_str(assets.load_text(&map_tileset.source)).unwrap();
    for tile in tileset.tiles {
      let tile_sprite = sprites.iter().find(|&item| item.source == tile.image.source);
      let tile_id = map_tileset.first_id + tile.id;

      state.world.run(
        |mut entities: EntitiesViewMut,
         mut tile_comp: ViewMut<Tile>,
         mut image: ViewMut<Image>,
         mut sprite: ViewMut<Sprite>,
         mut no_sprite: ViewMut<NoSpriteTag>| {
          let entity_id = entities.add_entity(
            (&mut tile_comp, &mut image),
            (
              Tile { id: tile_id },
              Image {
                source: tile.image.source,
                width: tile.image.width,
                height: tile.image.height,
              },
            ),
          );

          match tile_sprite {
            Some(s) => entities.add_component(&mut sprite, s.clone(), entity_id),
            None => entities.add_component(&mut no_sprite, NoSpriteTag, entity_id),
          }

          tiles.insert(tile_id, entity_id);
        },
      ); // RUN ---------------------------
    }
  }

  // group layers and chunk processing
  for (group_index, group) in map.groups.iter().enumerate() {
    let group_component = Group {
      id: group.id,
      name: group.name.clone(),
      render_index: group_index as u32,
    };

    for (layer_index, layer) in group.layers.iter().enumerate() {
      let layer_component = Layer {
        id: layer.id,
        name: layer.name.clone(),
        width: layer.width,
        height: layer.height,
        render_index: layer_index as u32,
      };

      let positions = get_tiles_positions(&map, &layer.data.chunk, &layer.data.chunk.value).unwrap();

      // Create layer entity iw will store information about layers
      let layer_id = state.world.run(
        |mut entities: EntitiesViewMut, mut groups: ViewMut<Group>, mut layers: ViewMut<Layer>, mut maps: ViewMut<MapComp>| {
          entities.add_entity(
            (&mut maps, &mut groups, &mut layers),
            (
              MapComp {
                tile_width: map.tile_width,
                tile_height: map.tile_width,
              },
              group_component.clone(),
              layer_component.clone(),
            ),
          )
        },
      );

      state.world.run(
        |mut entities: EntitiesViewMut, mut transforms: ViewMut<Transform>, mut layers: ViewMut<LayerRef>, mut tile_store: ViewMut<TileRef>| {
          for position in positions {
            let tile_entity_id = tiles[&position.id];
            entities.add_entity(
              (&mut transforms, &mut layers, &mut tile_store),
              (Transform::new(position.x, position.y), LayerRef(layer_id), TileRef(tile_entity_id)),
            );
          }
        },
      );
    }
  }

  //println!("{:#?}", map);
  //panic!("close");
}

fn get_tiles_positions(map: &Map, chunks: &Chunk, data: &Vec<u32>) -> Result<Vec<TileCoordinates>, ()> {
  let tile_width: u32 = map.tile_width;
  let tile_height: u32 = map.tile_height;

  let mut result: Vec<TileCoordinates> = Vec::new();

  for column in 0..chunks.height {
    let y = (column * tile_height) as f32;

    for row in 0..chunks.width {
      let x = (row * tile_width) as f32;
      let tile_id_index = column * chunks.width + row;
      let tile_id = data.get(tile_id_index as usize);

      match tile_id {
        Some(id) => {
          if *id > 0 {
            result.push(TileCoordinates { id: *id, x, y });
          }
        }
        None => panic!("Chunk data Index not found"),
      };
    }
  }

  Ok(result)
}
