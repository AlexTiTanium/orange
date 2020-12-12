use crate::WindowContext;
use game::{NonSendSync, UniqueViewMut};

pub fn swap(context: NonSendSync<UniqueViewMut<WindowContext>>) {
  context.swap_buffers().unwrap();
}
