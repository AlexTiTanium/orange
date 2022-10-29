use super::common::InputPosition;

#[derive(Default, Debug)]
pub struct PointerCursor {
  /// All mouse event on current frame
  pub events: Vec<InputPosition>,

  /// Last known cursor position
  pub now: InputPosition,
}

///
/// Mouse cursor API
///
impl PointerCursor {
  ///
  /// Update mouse position
  ///
  pub fn update(&mut self, x: f64, y: f64) {
    self.events.push(InputPosition(x, y));
    self.now = InputPosition(x, y);
  }

  ///
  /// Clear all cursor event
  ///
  pub fn clear(&mut self) {
    self.events.clear();
  }
}
