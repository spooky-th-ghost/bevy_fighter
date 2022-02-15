use bevy::prelude::*;
use crate::{
  character::{
    CharacterMovement,
    CharacterState,
    PlayerId
  },
  attacks::{
    Hitbox,
    SpawnHitbox
  }
};


/// Find which hitboxes should be created each frame and create them
pub fn spawn_hitboxes(
  mut coms: Commands,
  query: Query<(&PlayerId, &CharacterState, &CharacterMovement, &Transform)>,
) {
  for (player_id, state, movement, transform) in query.iter() {
    if let Some(hitbox_events) = state.get_hitbox_events_this_frame() {
      for he in hitbox_events.iter() {
        coms.spawn_hitbox(
          player_id,
          he,
          transform,
          movement.facing_vector
        )
      }
    }
  }
}

/// update/remove hitboxes based on how long they've been on screen
pub fn manage_hitboxes(
  mut coms: Commands,
  mut query: Query<(&mut Hitbox, Entity)>,
) {
  for (mut hitbox, entity) in query.iter_mut() {
    if hitbox.is_finished() {
      coms.entity(entity).despawn();
    } else {
      hitbox.tick();
    }
  }
} 
