use bevy::prelude::*;
use crate::{
  animation::{
    AnimationController,
    AnimationTransitionEvent
  },
  character::PlayerId
};

pub fn read_animation_transitions(
  mut query: Query<(&PlayerId, &mut AnimationController)>,
  mut transition_reader: EventReader<AnimationTransitionEvent>,
) {
  for event in transition_reader.iter() {
    for (player_id, mut controller) in query.iter_mut() {
      if event.player_id == *player_id {
        controller.transition(event.transition.clone());
      }
    }
  }
}

pub fn animate_sprite_system(
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationController)>,
) {
  for (mut sprite, mut anim_controller) in query.iter_mut() {
    sprite.index = anim_controller.get_next_frame();
  }
}
