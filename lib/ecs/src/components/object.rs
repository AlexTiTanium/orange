use std::string::String;

#[derive(Debug)]
pub struct Object {
  pub name: String,
}

impl Default for Object {
  fn default() -> Self {
    Self {
      name: String::from("Game object"),
    }
  }
}

