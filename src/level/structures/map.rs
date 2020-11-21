use super::decoders::{tileset_data_decoder,relative_file_name_decoder };
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
  Orthogonal,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Tileset {
  pub firstgid: u32,

  #[serde(deserialize_with = "relative_file_name_decoder")]
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

  // #[serde(rename = "layer")]
  // pub layers: Option<Vec<Layer>>,
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
  pub width: u32,
  pub height: u32,

  #[serde(rename = "tileset")]
  pub tilesets: Vec<Tileset>,

  pub group: Group
}
