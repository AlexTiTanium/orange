use log::*;
use stb_image;
use stb_image::image::LoadResult;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Instant;
use std::{env, fs};

pub struct Texture {
  pub data: Vec<u8>,
  pub width: usize,
  pub height: usize,
}

#[derive(Default)]
pub struct Assets {
  // Slot to texture map
  pub images: HashMap<String, i32>,
  // Textures
  pub textures: HashMap<i32, Texture>,
  // Xml resources
  pub strings: HashMap<String, String>,
  //Path to resources
  path: PathBuf,
}

impl Assets {
  pub fn new() -> Self {
    let mut path_to_exec = env::current_exe().unwrap();
    path_to_exec.pop();
    path_to_exec.push("resources");

    let mut path_to_exec_osx = env::current_exe().unwrap();
    path_to_exec_osx.pop();
    path_to_exec_osx.pop();
    path_to_exec_osx.push("Resources/resources/");

    let resources_path = vec![
      Path::new("resources/"),
      Path::new(path_to_exec_osx.as_path()),
      Path::new(path_to_exec.as_path()),
    ];

    let mut resources: PathBuf = PathBuf::new();

    for path in resources_path {
      if path.exists() {
        info!("Resources found: {:?}", path);
        resources = PathBuf::from(path);
        break;
      }
      warn!("Resources path not found: {:?}", path);
    }

    if resources.as_os_str().is_empty() {
      error!("Resource file no found");
      panic!("Resources folder not found");
    }

    Self {
      images: HashMap::new(),
      textures: HashMap::new(),
      strings: HashMap::new(),
      path: resources,
    }
  }

  pub fn get_texture(&self, slot: i32) -> &Texture {
    match self.textures.get(&slot) {
      Some(texture) => texture,
      None => {
        error!("Texture {:?} not found", slot);
        panic!("[Assets] Texture {:?} not found", slot);
      }
    }
  }

  pub fn load_texture(&mut self, slot: i32, resource: &str) -> Result<&Texture, ()> {
    let time = Instant::now();

    let path = self.get_path(resource);

    // unsafe {
    //   stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
    // }

    info!("Texture {:?} loading from: {:?}", slot, path);

    let img = stb_image::image::load(&path);

    let result = match img {
      LoadResult::Error(e) => {
        error!("Texture loading: {:?} {:?} ", e, &path);
        panic!("Texture loading: {:?} {:?} ", e, &path);
      }
      LoadResult::ImageU8(im) => {
        let texture = Texture {
          width: im.width,
          height: im.height,
          data: im.data,
        };
        self.textures.insert(slot, texture);
        self.images.insert(String::from(resource), slot);
      }
      LoadResult::ImageF32(_im32) => {
        warn!("Got unsupported f32 texture");
      }
    };

    info!("Time stb image {:?} ", time.elapsed());

    Ok(self.get_texture(slot))
  }

  pub fn get_path(&mut self, resource: &str) -> PathBuf {
    let mut path = PathBuf::from(&self.path);
    path.push(resource);
    path
  }

  pub fn load_text(&mut self, resource: &str) -> &String {
    let path = self.get_path(resource);
    info!("Text file loading from: {:?}", &path);
    let string = fs::read_to_string(path).unwrap();
    self.strings.insert(String::from(resource), string);
    return self.get_text(resource);
  }

  pub fn get_text(&mut self, resource: &str) -> &String {
    self.strings.get(resource).unwrap()
  }
}
