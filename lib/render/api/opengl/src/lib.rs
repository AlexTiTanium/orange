// General render components
use render_components as components;

// Opengl render systems
mod systems;

// Color quads render
mod quad_color;
use quad_color::QuadColorRender;

// Texture tiles render
mod quad_atlas;
use quad_atlas::QuadAtlasRender;

// Dependencies for render start
use game::{system, NonSendSync, State, UniqueViewMut};
use gl::Gl;
use window::WindowContext;

// Initialize open gl render
pub fn init(state: &State) {
  let context = state.world.borrow::<NonSendSync<UniqueViewMut<WindowContext>>>();

  let gl = Gl::load_with(|symbol| context.get_proc_address(symbol));

  state.world.add_unique_non_send_sync(gl.clone());
  state.world.add_unique_non_send_sync(QuadColorRender::new(&gl));
  state.world.add_unique_non_send_sync(QuadAtlasRender::new(&gl));

  state
    .render
    .borrow_mut()
    .with_system(system!(systems::clear))
    .with_system(system!(systems::step));
}

// pub fn load_textures(state: &State) {
//   // When level is ready we load our textures to GPU once
//   let mut quad_atlas_render = state.world.borrow::<NonSendSync<UniqueViewMut<QuadAtlasRender>>>();
//   quad_atlas_render.load_textures(state);
// }
