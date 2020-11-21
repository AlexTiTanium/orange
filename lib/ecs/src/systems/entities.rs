use crate::components::*;
use crate::EntitiesViewMut;
use crate::EntityId;
use crate::ViewMut;

pub fn create_game_object(
  mut entities: EntitiesViewMut,
  mut game_object: ViewMut<Object>,
  mut transform: ViewMut<Transform>,
  mut active: ViewMut<ActiveTag>,
) -> EntityId {
  entities.add_entity(
    (&mut game_object, &mut transform, &mut active),
    (Object::default(), Transform::default(), ActiveTag),
  )
}
