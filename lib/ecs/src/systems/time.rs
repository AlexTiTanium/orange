use crate::components::*;
use crate::*;
use std::time::Instant;

#[system(UpdateTime)]
pub fn run(mut time: Unique<&mut Time>) {
  time.delta = time.last.elapsed();
  time.last = Instant::now();
}
