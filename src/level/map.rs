use super::decoders::tileset_data_decoder;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
  Orthogonal,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Tileset {
  pub firstgid: u32,
  pub source: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Layer {
  id: u32,
  name: String,
  width: u32,
  height: u32,

  #[serde(deserialize_with = "tileset_data_decoder")]
  data: Vec<u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Group {
  pub id: u32,
  pub name: String,
  pub locked: Option<u32>,

  #[serde(rename = "objectgroups")]
  pub object_groups: Option<Vec<ObjectGroup>>,

  #[serde(rename = "layer")]
  pub layers: Option<Vec<Layer>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ObjectGroup {
  id: u32,
  name: String,

  #[serde(rename = "object")]
  objects: Vec<Object>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Object {
  id: u32,
  gid: u32,
  x: u32,
  y: u32,
  width: u32,
  height: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Map {
  pub version: String,
  tiledversion: String,
  orientation: Orientation,
  renderorder: String,
  pub width: u32,
  pub height: u32,

  #[serde(rename = "tilewidth")]
  pub tile_width: u32,
  #[serde(rename = "tileheight")]
  pub tile_height: u32,

  infinite: u32,
  nextlayerid: u32,
  nextobjectid: u32,

  #[serde(rename = "tileset")]
  pub tilesets: Vec<Tileset>,

  #[serde(rename = "group")]
  pub groups: Vec<Group>,
}
