use super::decoders::tileset_file_name_decoder;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Grid {
  orientation: String,
  width: u32,
  height: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Tile {
  pub id: String,
  image: Option<Image>,
  terrain: Option<String>
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Image {
  #[serde(deserialize_with = "tileset_file_name_decoder")]
  source: String,
  width: u32,
  height: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TerrainType {
  terrain: Terrain,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Terrain {
  name: String,
  tile: u32
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Tileset {
  pub version: String,
  tiledversion: String,
  pub name: String,

  #[serde(rename = "tilewidth")]
  pub tile_width: u32,

  #[serde(rename = "tileheight")]
  pub tile_height: u32,

  #[serde(rename = "tilecount")]
  pub tile_count: u32,

  pub columns: u32,

  pub grid: Option<Grid>,

  #[serde(rename = "tile")]
  pub tiles: Vec<Tile>,

  pub image: Option<Image>,

  #[serde(rename = "terraintypes")]
  pub terrain_types: Option<TerrainType>

}