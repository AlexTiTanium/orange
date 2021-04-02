use std::f32;
use std::time::Instant;

pub struct FPS {
  pub frames: f32,
  pub fps: f32,
  time: Instant,
}

impl Default for FPS {
  fn default() -> Self {
    Self {
      frames: 0.0,
      fps: 0.0,
      time: Instant::now(),
    }
  }
}

impl FPS {
  pub fn get_average_fps(&self) -> f32 {
    self.fps.round()
  }

  pub fn get_fps(&self) -> f32 {
    let time_from_start = self.time.elapsed().as_secs_f32();
    let fps = self.frames / time_from_start;
    fps.round()
  }
}
