use crate::components::{Input, Position, Time};
use crate::*;

#[system(MoveOnInput)]
pub fn run(mut positions: &mut Position, input: Unique<&Input>, time: Unique<&Time>) {
  let speed = 180.0;

  if input.action {
    (&mut positions).iter().for_each(|pos| {
      pos.add_y(speed * time.delta);
    });
  } else {
    (&mut positions).iter().for_each(|pos| {
      if pos.y() > 0.0 {
        pos.minus_y(speed * 3.0 * time.delta);
      }
    });
  }
}
