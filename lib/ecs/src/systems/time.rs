use crate::resources::*;
use crate::UniqueView;
use crate::UniqueViewMut;
use std::f32;
use std::time::Instant;

pub fn update(mut time: UniqueViewMut<Time>) {
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

pub fn update_fps(mut fps: UniqueViewMut<FPS>, time: UniqueView<Time>) {
  fps.frames += 1.0;
  let time_from_start = time.running.elapsed().as_secs_f32();
  fps.fps = fps.frames / time_from_start;
}
