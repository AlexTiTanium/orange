use crate::components::Position;
use crate::entities::Input;
use crate::*;

#[system(MoveOnInput)]
pub fn run(mut pos: &mut Position, input: Unique<&Input>) {
  if input.action {
    println!("_________",);
  }
}
