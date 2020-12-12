use crate::WindowContext;
use game::{NonSendSync, UniqueViewMut};

pub fn request_redraw(context: NonSendSync<UniqueViewMut<WindowContext>>) {
  context.window().request_redraw();
}
