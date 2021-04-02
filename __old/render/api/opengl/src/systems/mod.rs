use crate::components::{Image, LayerRef, Sprite, Texture, TileRef};
use game::components::{ActiveTag, Transform};
use game::resources::Camera;
use game::{IntoIter, IntoWithId, NonSendSync, UniqueView, UniqueViewMut, View};
use gl::{Gl, GL};

use crate::quad_atlas::QuadAtlasRender;

pub fn clear(gl: NonSendSync<UniqueView<Gl>>) {
  unsafe {
    gl.ClearColor(0.2, 0.2, 0.2, 1.0);
    gl.Clear(GL::COLOR_BUFFER_BIT);
  }
}

pub fn step(
  mut texture_render: NonSendSync<UniqueViewMut<QuadAtlasRender>>,
  ref textures: View<Texture>,
  ref camera: UniqueView<Camera>,
  ref transforms: View<Transform>,
  ref active: View<ActiveTag>,
  ref images: View<Image>,
  ref sprites: View<Sprite>,
  ref tile: View<TileRef>,
  ref layer: View<LayerRef>,
) {
  texture_render.set_camera(camera);

  for (texture_id, _) in (&textures).iter().with_id() {
    texture_render.update(texture_id, transforms, active, images, sprites, tile, layer, textures);
    texture_render.step();
  }
}
