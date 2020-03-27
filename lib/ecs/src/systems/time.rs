use crate::components::*;
use crate::*;
use std::f32;
use std::time::Instant;

#[system(UpdateTime)]
pub fn run(mut time: Unique<&mut Time>) {
  let now = Instant::now();
  let delta = now - time.last;

  let delta_s = delta.as_secs_f32();

  if delta_s > 0.0 {
    time.delta = delta_s;
  } else {
    time.delta = f32::MIN_POSITIVE;
  }

  time.last = Instant::now();
}

#[system(UpdateFPS)]
pub fn run(mut fps: Unique<&mut FPS>, time: Unique<&Time>) {
  fps.frames += 1.0;
  let time_from_start = time.running.elapsed().as_secs_f32();
  fps.fps = fps.frames / time_from_start;
  println!("FPS {:?}", fps.fps);
}
