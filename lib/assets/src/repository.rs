use png;
use stb_image;
use stb_image::image::LoadResult;
use std::collections::HashMap;
use std::fs::File;
use std::time::Instant;

pub struct Repository {
  map: HashMap<String, Vec<u8>>,
}

impl Repository {
  pub fn new() -> Repository {
    Repository { map: HashMap::new() }
  }

  pub fn get(&self, id: &str) -> &Vec<u8> {
    match self.map.get(id) {
      Some(data) => data,
      None => panic!("[Assets] Asset {:?} not found", id),
    }
  }

  pub fn load(&mut self, id: &str, path: &str) {
    let time = Instant::now();

    let decoder = png::Decoder::new(File::open(path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut img_data = vec![0; info.buffer_size()];
    reader.next_frame(&mut img_data).unwrap();
    self.map.insert(String::from(id), img_data);
    println!("Time png {:?} ", time.elapsed());
  }

  pub fn load2(&mut self, id: &str, path: &str) {
    let time = Instant::now();
    let img = stb_image::image::load(path);

    match img {
      LoadResult::Error(e) => {
        println!("Image loading: {:?} ", e);
      }
      LoadResult::ImageU8(im) => {
        self.map.insert(String::from(id), im.data);
      }
      LoadResult::ImageF32(im32) => {
        println!("f32: {:?} ", im32.width);
      }
    }
    println!("Time stb image {:?} ", time.elapsed());
  }
}
