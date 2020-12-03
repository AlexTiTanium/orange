use game::resources::Time;
use game::{UniqueView, UniqueViewMut};

use crate::resources::FPS;

pub fn update_fps(mut fps: UniqueViewMut<FPS>, time: UniqueView<Time>) {
  fps.frames += 1.0;
  let time_from_start = time.running.elapsed().as_secs_f32();
  fps.fps = fps.frames / time_from_start;
  println!("dsdsjdksjdskdj")
}
