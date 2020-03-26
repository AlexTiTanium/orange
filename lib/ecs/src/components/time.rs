use std::time::Duration;
use std::time::Instant;

pub struct Time {
  pub delta: Duration,
  pub last: Instant,
}

impl Default for Time {
  fn default() -> Self {
    Self {
      delta: Duration::from_secs(0),
      last: Instant::now(),
    }
  }
}
