use crate::{resources::WindowSize, WindowContext, WindowResizeEvent};
use common::{events::Events, log, NonSendSync, UniqueView, UniqueViewMut};

pub fn window_resize(
  context: NonSendSync<UniqueViewMut<WindowContext>>,
  events: UniqueView<Events<WindowResizeEvent>>,
  window_size: UniqueViewMut<WindowSize>,
) {
  let mut reader = events.get_reader();
  let mut changed_window_size = false;

  for event in reader.iter(&events) {
    let WindowResizeEvent(size) = event;
    log::info!("Window resize, new size: {:?}", size);
    context.resize(size.clone());
    changed_window_size = true;
  }

  if changed_window_size {
    update_window_size(context, window_size);
  }
}

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

pub fn swap_buffers(context: NonSendSync<UniqueViewMut<WindowContext>>) {
  context.swap_buffers().unwrap();
}

pub fn request_redraw(context: NonSendSync<UniqueViewMut<WindowContext>>) {
  context.window().request_redraw();
}
