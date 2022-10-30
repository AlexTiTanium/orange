use crate::{events::WindowInputEvent, resources::WindowSize, WindowContext};
use common::{events::Events, log, NonSendSync, UniqueView, UniqueViewMut};

///
/// Update window context size
///
pub fn on_window_resize(
  context: NonSendSync<UniqueViewMut<WindowContext>>,
  events: UniqueView<Events<WindowInputEvent>>,
  window_size: UniqueViewMut<WindowSize>,
) {
  let reader = events.get_reader().iter(&events);
  let mut changed_window_size = false;

  for event in reader {
    if let WindowInputEvent::Resized(width, height) = event {
      context.resize(width, height);
      changed_window_size = true;
    }
  }

  if changed_window_size {
    update_window_size(context, window_size);
  }
}

///
/// Update WindowSize resource on size change
///
pub fn update_window_size(context: NonSendSync<UniqueViewMut<WindowContext>>, mut window_size: UniqueViewMut<WindowSize>) {
  let window = context.window();

  let scale_factor = window.scale_factor(); // TODO: round?
  let logical_size = window.inner_size().to_logical::<f32>(scale_factor);
  let physical_size = logical_size.to_physical::<f32>(scale_factor);

  log::info!(
    "Window set size new logical: {:?};  physical: {:?}; scale: {:?}",
    logical_size,
    physical_size,
    scale_factor
  );

  window_size.logical = logical_size;
  window_size.scale = scale_factor;
  window_size.physical = physical_size;
}

///
/// Swap buffers
///
pub fn swap_buffers(context: NonSendSync<UniqueViewMut<WindowContext>>) {
  context.swap_buffers();
}
