mod structures;
use std::collections::HashMap;

use structures::atlas::TextureAtlas;
use structures::map;
use structures::map::Map;
use structures::tileset::Tileset;

use game::components::{ActiveTag, Group, Image, Layer, LayerRef, Map as MapComp, NoSpriteTag, Sprite, Texture, Tile, TileRef, Transform};
use game::resources::Assets;
use game::{EntitiesViewMut, EntityId, State, UniqueViewMut, ViewMut};

use quick_xml::de::from_str;

struct TileCoordinates {
  pub id: u32,
  pub x: f32,
  pub y: f32,
}

/// Constants for tile flipping
/// http://doc.mapeditor.org/reference/tmx-map-format/#tile-flipping
static FLIPPED_HORIZONTALLY_FLAG: u32 = 0x80000000;
static FLIPPED_VERTICALLY_FLAG: u32 = 0x40000000;
static FLIPPED_DIAGONALLY_FLAG: u32 = 0x20000000;

pub fn load(state: &State, level: &str, textures: Vec<&str>) {
  let mut assets = state.world.borrow::<UniqueViewMut<Assets>>();

  let map: Map = from_str(assets.load_text(level)).unwrap();
  let sprites: Vec<Sprite> = get_sprites(state, &mut assets, &textures);
  let mut tiles: HashMap<u32, EntityId> = HashMap::new();

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

      let mut positions: Vec<TileCoordinates> = Vec::new();

      for chunk in &layer.data.chunks {
        let mut result = get_tiles_positions(&map, &layer, &chunk);
        positions.append(&mut result);
      }

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
        |mut entities: EntitiesViewMut,
         mut transforms: ViewMut<Transform>,
         mut layers: ViewMut<LayerRef>,
         mut tile_store: ViewMut<TileRef>,
         mut active: ViewMut<ActiveTag>| {
          for position in positions {
            let tile_entity_id = tiles[&position.id];
            entities.add_entity(
              (&mut transforms, &mut layers, &mut tile_store, &mut active),
              (
                Transform::new(position.x, position.y),
                LayerRef(layer_id),
                TileRef(tile_entity_id),
                ActiveTag,
              ),
            );
          }
        },
      );
    }
  }
}

/// Get tiles position and flip params
fn get_tiles_positions(map: &Map, layer: &map::Layer, chunk: &map::Chunk) -> Vec<TileCoordinates> {
  let tile_width: f32 = map.tile_width as f32;
  let tile_height: f32 = map.tile_height as f32;

  let mut result: Vec<TileCoordinates> = Vec::new();

  let mut count = 0;

  for id in &chunk.value {
    let int_id = id & 0xFFFFFFFF;

    if int_id == 0 {
      count += 1;
      continue;
    }

    let flipped_h: bool = int_id & FLIPPED_HORIZONTALLY_FLAG == 1;
    let flipped_v: bool = int_id & FLIPPED_VERTICALLY_FLAG == 1;
    let flipped_d: bool = int_id & FLIPPED_DIAGONALLY_FLAG == 1;

    let tile_id = int_id & !(FLIPPED_HORIZONTALLY_FLAG | FLIPPED_VERTICALLY_FLAG | FLIPPED_DIAGONALLY_FLAG);

    let ceil_x = chunk.x + (count % chunk.width) as f32;
    let ceil_y = chunk.y + (count / chunk.height) as f32;

    let x = ceil_x * tile_width + layer.offset_x;
    let y = ceil_y * tile_height + layer.offset_y;

    result.push(TileCoordinates { id: tile_id, x, y: -y });
    count += 1;
  }

  return result;
}

/// Import sprites, sprite contains information about texture on atlas, size position UV coordinates
fn get_sprites(state: &State, assets: &mut Assets, textures: &Vec<&str>) -> Vec<Sprite> {
  let mut sprites: Vec<Sprite> = Vec::new();

  // Load textures to esc and memory
  for (slot, path) in textures.iter().enumerate() {
    let atlas: TextureAtlas = from_str(assets.load_text(&path)).unwrap();

    // Load texture to memory
    let texture = assets.load_texture(slot as i32, &atlas.source).unwrap();

    // Create entity for shaders
    let texture_id = state.world.run(|mut entities: EntitiesViewMut, mut textures: ViewMut<Texture>| {
      entities.add_entity(
        &mut textures,
        Texture::new(slot as i32, &atlas.source, texture.width as u32, texture.height as u32),
      )
    });

    for sprite in atlas.sprites {
      let w = sprite.width as f32 / texture.width as f32;
      let h = sprite.height as f32 / texture.height as f32;
      let x = sprite.x / texture.width as f32;
      let y = sprite.y / texture.height as f32;

      sprites.push(Sprite {
        texture: texture_id,
        source: sprite.source,
        x: sprite.x,
        y: sprite.y,
        width: sprite.width,
        height: sprite.height,
        rotated: sprite.rotated,
        uv: [
          [x, y + h],     // top right
          [x, y],         // top left
          [x + w, y + h], // bottom right
          [x + w, y],     // bottom left
        ],
      });
    }
  }

  return sprites;
}
