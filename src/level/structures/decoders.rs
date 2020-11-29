use base64;
use serde::de::Deserialize;
use serde::Deserializer;
use std::{convert::TryInto, path::Path};

pub fn tileset_data_decoder<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
  D: Deserializer<'de>,
{
  let base64: String = Deserialize::deserialize(deserializer)?;
  let data: Vec<u8> = base64::decode(base64).unwrap();
  let mut result: Vec<u32> = Vec::new();

  for i in (0..data.len()).step_by(4) {
    let bytes: [u8; 4] = data[i..i + 4].try_into().expect("slice with incorrect length");
    let id = u32::from_le_bytes(bytes);
    result.push(id);
  }

  Ok(result)
}

pub fn tileset_file_name_decoder<'de, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  let path: String = Deserialize::deserialize(deserializer)?;
  let file = Path::new(&path).file_name().unwrap().to_os_string().into_string().unwrap();

  Ok(file)
}

pub fn relative_file_name_decoder<'de, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  let path: String = Deserialize::deserialize(deserializer)?;
  let file = Path::new(&path).strip_prefix("../").unwrap();

  Ok(file.to_str().unwrap().into())
}

pub fn relative_image_name_decoder<'de, D>(deserializer: D) -> Result<String, D::Error>
where
  D: Deserializer<'de>,
{
  let path: String = Deserialize::deserialize(deserializer)?;
  let file = Path::new(&path).strip_prefix("../../assets/").unwrap();

  Ok(file.to_str().unwrap().into())
}
