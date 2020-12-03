use game::components::*;
use game::State;
use game::*;
use game::{IntoWithId, View};
use imgui::{im_str, Condition, Ui, Window};

pub fn build(ui: &Ui, state: &State) {
  Window::new(im_str!("Entities"))
    .size([300.0, 100.0], Condition::FirstUseEver)
    .build(&ui, || {
      build_ui(&ui, state);
    });
}

fn build_ui(ui: &Ui, state: &State) {
  ui.same_line(ui.window_content_region_width() / 2.0 - 170.0 / 2.0);
  if ui.button(im_str!("Create Game Object"), [170.0, 25.0]) {
    state.create_game_object();
  }

  ui.separator();

  let (mut transforms, tile, tiles, layer, layers, sprites, groups, images, objects, textures) = state.world.borrow::<(
    ViewMut<Transform>,
    View<TileRef>,
    View<Tile>,
    View<LayerRef>,
    View<Layer>,
    View<Sprite>,
    View<Group>,
    View<Image>,
    View<Object>,
    View<Texture>,
  )>();
  let (entities, mut active) = state.world.borrow::<(EntitiesView, ViewMut<ActiveTag>)>();

  (&mut transforms, &tile, &layer)
    .iter()
    .with_id()
    .enumerate()
    .for_each(|(index, (id, (mut transform, tile, layer)))| {
      let group = ui.push_id(index as i32);
      let TileRef(tile_entity_id) = tile;
      let LayerRef(layer_entity_id) = layer;

      ui.text(format!("Entity id: {:?}", id));

      if objects.contains(id) {
        ui.text(format!("Name: {:?}", objects[id].name));
      }

      if tiles.contains(*tile_entity_id) {
        ui.text(format!("Tile id: {:?}", tiles[*tile_entity_id].id));
      }

      if images.contains(*tile_entity_id) {
        let image = &images[*tile_entity_id];
        ui.text(format!("Image size: {:?}x{:?}", image.width, image.height));
      }

      if sprites.contains(*tile_entity_id) {
        let sprite = &sprites[*tile_entity_id];
        ui.text(format!("Sprite texture slot: {:?}", textures[sprite.texture].slot));
        ui.text(format!("Sprite source: {:?}", sprite.source));
        ui.text(format!("Sprite size: {:?}x{:?}", sprite.width, sprite.height));
        ui.text(format!("Sprite x, y: {:?}, {:?}", sprite.x, sprite.y));
        ui.text(format!("Sprite rotated: {:?}", sprite.rotated));
      }

      if groups.contains(*layer_entity_id) {
        let group = &groups[*layer_entity_id];
        ui.text(format!("Group name: {:?}", group.name));
      }

      if layers.contains(*layer_entity_id) {
        let layer = &layers[*layer_entity_id];
        ui.text(format!("Layer name: {:?}", layer.name));
      }

      transform_control(&ui, &mut transform);
      active_control(&ui, &mut active, id, &entities);

      group.pop(&ui);
      ui.separator();
    });
}

fn transform_control(ui: &Ui, transform: &mut Transform) {
  let mut position: [f32; 3] = [transform.position.x, transform.position.y, transform.position.z];
  ui.drag_float3(im_str!("Position"), &mut position).max(800.0).min(0.0).build();
  transform.position.x = position[0];
  transform.position.y = position[1];
  transform.position.z = position[2];
}

fn active_control(ui: &Ui, active: &mut ViewMut<'_, ActiveTag>, id: EntityId, entities: &Entities) {
  let is_active = active.contains(id);
  let mut checked = is_active;

  ui.checkbox(im_str!("Active"), &mut checked);

  if !checked && is_active {
    active.delete(id);
  }

  if checked && !is_active {
    entities.add_component(id, active, ActiveTag);
  }
}
