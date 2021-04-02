use super::decoders::{relative_file_name_decoder, tileset_data_decoder};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
  Orthogonal,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Tileset {
  #[serde(rename = "firstgid")]
  pub first_id: u32,

  #[serde(deserialize_with = "relative_file_name_decoder")]
  pub source: String,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Chunk {
  pub x: f32,
  pub y: f32,
  pub width: u32,
  pub height: u32,
  #[serde(rename = "$value", deserialize_with = "tileset_data_decoder")]
  pub value: Vec<u32>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Data {
  #[serde(rename = "chunk", default)]
  pub chunks: Vec<Chunk>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Layer {
  pub id: u32,
  pub name: String,
  pub width: u32,
  pub height: u32,

  #[serde(rename = "offsetx", default)]
  pub offset_x: f32,

  #[serde(rename = "offsety", default)]
  pub offset_y: f32,

  pub data: Data,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Group {
  pub id: u32,
  pub name: String,

  #[serde(rename = "layer")]
  pub layers: Vec<Layer>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Object {
  pub id: u32,
  pub gid: u32,
  pub x: u32,
  pub y: u32,
  pub width: u32,
  pub height: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Map {
  pub width: u32,
  pub height: u32,

  #[serde(rename = "tilewidth")]
  pub tile_width: u32,

  #[serde(rename = "tileheight")]
  pub tile_height: u32,

  #[serde(rename = "tileset")]
  pub tilesets: Vec<Tileset>,

  #[serde(rename = "group")]
  pub groups: Vec<Group>,
}
