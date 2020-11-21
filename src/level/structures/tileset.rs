use super::decoders::{relative_image_name_decoder };
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Tile {
  pub id: u32,
  pub image: Image,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Image {
  #[serde(deserialize_with = "relative_image_name_decoder")]
  pub source: String,
  pub width: u32,
  pub height: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Tileset {
  pub name: String,

  #[serde(rename = "tile")]
  pub tiles: Vec<Tile>,
}