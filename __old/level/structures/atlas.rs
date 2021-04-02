use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Sprite {
  #[serde(rename = "n")]
  pub source: String,

  pub x: f32,
  pub y: f32,

  #[serde(rename = "w")]
  pub width: u32,

  #[serde(rename = "h")]
  pub height: u32,

  #[serde(default)]
  #[serde(rename = "r")]
  pub rotated: bool
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TextureAtlas {
  #[serde(rename = "imagePath")]
  pub source: String,
  pub width: u32,
  pub height: u32,

  #[serde(rename = "sprite")]
  pub sprites: Vec<Sprite>
}