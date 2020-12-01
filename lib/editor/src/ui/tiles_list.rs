use game::components::*;
use game::State;
use game::*;
use imgui::{im_str, Condition, Ui, Window};

pub fn build(ui: &Ui, state: &State) {
  Window::new(im_str!("Tiles"))
    .size([300.0, 100.0], Condition::FirstUseEver)
    .build(&ui, || {
      build_ui(&ui, state);
    });
}

fn build_ui(ui: &Ui, state: &State) {
  ui.same_line(ui.window_content_region_width() / 2.0 - 170.0 / 2.0);

  let tiles = state.world.borrow::<View<Tile>>();
  let images = state.world.borrow::<View<Image>>();
  let sprites = state.world.borrow::<View<Sprite>>();
  let textures = state.world.borrow::<View<Texture>>();
  let no_sprite = state.world.borrow::<View<NoSpriteTag>>();

  tiles.iter().enumerate().with_id().for_each(|(id, (index, tiles))| {
    let group = ui.push_id(index as i32);
    ui.separator();

    ui.text(format!("Entity ID {:?}", id));
    ui.text(format!("Tile ID: {:?}", tiles.id));

    if images.contains(id) {
      let image = &images[id];
      ui.text(format!("Image source {:?}", image.source));
      ui.text(format!("Image size: {:?}x{:?}", image.width, image.height));
    }

    if sprites.contains(id) {
      let sprite = &sprites[id];
      ui.text(format!("Sprite texture slot {:?}", textures[sprite.texture].slot));
      ui.text(format!("Sprite source {:?}", sprite.source));
      ui.text(format!("Sprite size: {:?}x{:?}", sprite.width, sprite.height));
      ui.text(format!("Sprite x, y: {:?}, {:?}", sprite.x, sprite.y));
      ui.text(format!("Sprite rotated: {:?}", sprite.rotated));
    }

    if no_sprite.contains(id) {
      ui.text(format!("Tile doesn't have texture"));
    }

    group.pop(&ui);
    ui.separator();
  });
}
