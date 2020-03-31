use log::*;
use stb_image;
use stb_image::image::LoadResult;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub struct Texture {
  pub data: Vec<u8>,
  pub width: usize,
  pub height: usize,
}

#[derive(Default)]
pub struct Assets {
  // Slot to texture map
  pub images: HashMap<String, u32>,
  // Textures
  pub textures: HashMap<u32, Texture>,
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
      path: resources,
    }
  }

  pub fn get_texture(&self, slot: u32) -> &Texture {
    match self.textures.get(&slot) {
      Some(texture) => texture,
      None => {
        error!("Texture {:?} not found", slot);
        panic!("[Assets] Texture {:?} not found", slot);
      }
    }
  }

  pub fn load_texture(&mut self, slot: u32, resource: &str) {
    let time = Instant::now();

    let mut path = PathBuf::from(&self.path);
    path.push(resource);

    // unsafe {
    //   stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
    // }

    info!("Texture {:?} loading from: {:?}", slot, path);

    let img = stb_image::image::load(&path);

    match img {
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
    }
    info!("Time stb image {:?} ", time.elapsed());
  }
}
