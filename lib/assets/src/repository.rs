use log::*;
use stb_image;
use stb_image::image::LoadResult;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::time::Instant;

#[derive(Default)]
pub struct Repository {
  map: HashMap<String, Vec<u8>>,
  path: PathBuf,
}

impl Repository {
  pub fn new() -> Repository {
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

    Repository {
      map: HashMap::new(),
      path: resources,
    }
  }

  pub fn get(&self, id: &str) -> &Vec<u8> {
    match self.map.get(id) {
      Some(data) => data,
      None => {
        error!("Asset {:?} not found", id);
        panic!("[Assets] Asset {:?} not found", id);
      }
    }
  }

  pub fn load(&mut self, id: &str, resource: &str) {
    let time = Instant::now();

    let mut path = PathBuf::from(&self.path);
    path.push(resource);

    unsafe {
      stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
    }

    let img = stb_image::image::load(&path);

    match img {
      LoadResult::Error(e) => {
        error!("Image loading: {:?} {:?} ", e, &path);
        panic!("Image loading: {:?} {:?} ", e, &path);
      }
      LoadResult::ImageU8(im) => {
        self.map.insert(String::from(id), im.data);
      }
      LoadResult::ImageF32(im32) => {
        info!("f32: {:?} ", im32.width);
      }
    }
    info!("Time stb image {:?} ", time.elapsed());
  }
}
