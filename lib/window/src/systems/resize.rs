use game::{NonSendSync, UniqueViewMut};
use logger::log;
use winit::dpi::PhysicalSize;

use crate::{resources::WindowSize, WindowContext};

pub fn window_resize(size: (u32, u32), context: NonSendSync<UniqueViewMut<WindowContext>>) {
  let (width, height) = size;
  log::info!("Window resize, new size: {:?}x{:?}", width, height);
  context.resize(PhysicalSize::<u32> { width, height });
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
