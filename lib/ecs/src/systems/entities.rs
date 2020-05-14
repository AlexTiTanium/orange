use crate::components::*;
use crate::EntitiesViewMut;
use crate::EntityId;
use crate::ViewMut;

pub fn create_game_object(
  mut entities: EntitiesViewMut,
  mut game_object: ViewMut<GameObject>,
  mut transform: ViewMut<Transform>,
  mut active: ViewMut<ActiveTag>,
) -> EntityId {
  entities.add_entity(
    (&mut game_object, &mut transform, &mut active),
    (GameObject::default(), Transform::default(), ActiveTag),
  )
}
