use std::f32;
use std::time::Instant;

pub struct Time {
  pub delta: f32,
  pub last: Instant,
  pub running: Instant,
}

impl Default for Time {
  fn default() -> Self {
    Self {
      delta: f32::MIN_POSITIVE,
      last: Instant::now(),
      running: Instant::now(),
    }
  }
}
