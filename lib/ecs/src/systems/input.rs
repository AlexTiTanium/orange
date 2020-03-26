use crate::components::{Input, Position, Time};
use crate::*;

#[system(MoveOnInput)]
pub fn run(mut pos: &mut Position, input: Unique<&Input>, time: Unique<&Time>) {
  if input.action {
    println!("_________{:?}", time.delta.as_millis());
  }
}
