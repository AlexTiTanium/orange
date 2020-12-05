use crate::resources::Window as WindowResource;
use game::UniqueViewMut;
use logger::log;

pub fn create_window(mut window: UniqueViewMut<WindowResource>) {
  log::info!("Creating window started");
  log::error!("Error example");
  log::debug!("Debug example");
  log::warn!("Warn example");
}
