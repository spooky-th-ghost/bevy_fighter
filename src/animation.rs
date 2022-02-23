use bevy::prelude::*;
use regex::Regex;
use crate::{
  character_library::{
    AnimationSerialized,
    CharacterLibrary
  },
  character::PlayerId
};

use std::collections::HashMap;

/// Information on which cells of a sprite sheet should be displayed for the given action
#[derive(Clone,Copy, PartialEq, Debug, Default)]
pub struct Animation {
  /// What frame in the spritesheet does the animation start
  pub first_frame: usize,
  /// How many frames does the animation take up
  pub length: usize,
  /// Should the animation loop
  pub loopable: bool,
  /// Where on the sprite sheet is the final frame located
  pub final_frame: usize,
  /// How long should each frame of the animation be displayed
  pub hold: u8,
}

impl Animation {
  /// Generate an animation from its serialized counterpart
  pub fn from_serialized(s: AnimationSerialized) -> Self {
     let final_frame: usize = s.first_frame + s.length - 1;
    
    Animation {
      first_frame: s.first_frame,
      length: s.length,
      loopable: s.loopable,
      final_frame,
      hold: s.hold
    }
  }
}

/// Indicates if the [AnimationController] should loop it's current [Animation]
#[derive(Debug, PartialEq)]
pub enum AnimationState {
  LOOPING,
  SMEARING,
}

impl Default for AnimationState {
  fn default() -> Self {
    AnimationState::LOOPING
  }
}

/// Used to indicate what, if any, transition [Animation] should be played by the [AnimationController]
pub struct AnimationTransitionEvent {
  pub player_id: PlayerId,
  pub transition: AnimationTransition,
}

impl AnimationTransitionEvent {
  pub fn new(player_id: PlayerId, transition: AnimationTransition) -> Self {
    AnimationTransitionEvent {
      player_id,
      transition
    }
  }
}

/// Transition variants used by [AnimationTransitionEvent]
#[derive(Clone, PartialEq)]
pub enum AnimationTransition {
  WalkToIdle,
  BackwalkToIdle,
  CrouchToIdle,
  DashToIdle,
  BackDashToIdle,
  RiseToFall,
  FallToIdle,
  AirdashToFall,
  AirbackdashToFall,
  ToCrouch,
  ToWalk,
  ToBackwalk,
  ToDash,
  ToBackdash,
  ToRise,
  ToIdle,
  ToAirdash,
  ToAirBackdash,
  ToAttack {name: String},
}

/// Handles the sprite animation for a character
#[derive(Debug, Component, Default)]
pub struct AnimationController {
  character_prefix: String,
  animation_state: AnimationState,
  pub core_animation: Animation,
  pub smear_animation: Option<Animation>,
  current_hold: u8,
  hold_counter: u8,
  current_index: usize,
  animations: HashMap<String, Animation>
}

impl AnimationController {
  pub fn new(character_prefix: &str, library: &CharacterLibrary) -> Self {
    let mut animations = HashMap::new();
    let my_regex = Regex::new(&format!("(^{}.+)", character_prefix)[..]).unwrap();
    for (anim_id, animation) in library.read_animations() {
      if my_regex.is_match(anim_id) {
        let trimmed_animation_name = anim_id.replace(character_prefix, "").replace("_","");
        animations.insert(trimmed_animation_name.clone(), animation.clone());
      }
    }

    AnimationController {
      character_prefix: character_prefix.to_string(),
      animation_state: AnimationState::LOOPING,
      core_animation: library.get_animation(format!("{}_idle", character_prefix.to_string())).unwrap(),
      smear_animation: None,
      current_index: 0,
      current_hold: 2,
      hold_counter: 0,
      animations,
    }
  }

  pub fn get_next_frame(&mut self) -> usize {
    use AnimationState::*;
    self.current_hold = match self.animation_state {
      SMEARING => {
        if let Some(sa) = self.smear_animation {
          sa.hold
        } else {
          2
        }
      },
      LOOPING => {
        self.core_animation.hold
      }
    };

    if self.hold_counter == self.current_hold {
      let mut new_index: usize = self.current_index + 1;
      match self.animation_state {
        LOOPING => {
          if new_index > self.core_animation.final_frame {
            new_index = self.core_animation.first_frame;
          }
        },
        SMEARING => {
          if let Some(smear) = self.smear_animation {
            if new_index > smear.final_frame {
              self.animation_state = LOOPING;
              self.smear_animation = None;
              new_index = self.core_animation.first_frame;
            }
          }
        }
      }
      self.current_index = new_index;
      self.hold_counter = 0;
    } else {
      self.hold_counter += 1; 
    }
    return self.current_index;
  }

  pub fn reset(&mut self) {
    self.hold_counter = 0;
    self.current_index = match self.animation_state {
      AnimationState::SMEARING => {
        if let Some(sa) = self.smear_animation {
          sa.first_frame
        } else {
          self.core_animation.first_frame
        }
      },
      AnimationState::LOOPING => self.core_animation.first_frame,
    }
  }

  pub fn get_animation(&self, anim_id: String) -> Option<Animation>{
    if let Some(animation) = self.animations.get(&anim_id) {
      return Some(animation.clone());
    } else {
      return None;
    }
  }

  pub fn transition(&mut self, transition: AnimationTransition) {
    use AnimationTransition::*;

    match transition {
      ToIdle => self.loop_animation("idle".into()),

      ToAttack {name} => self.loop_animation(name),

      ToDash => self.smear_animation("idle<>dash".into(), "dash".into()),

      ToBackdash => self.smear_animation("idle<>backdash".into(), "backdash".into()),

      ToRise => self.smear_animation("jumpsquat".into(), "rise".into()),

      ToWalk => self.smear_animation("idle<>walk".into(), "walk".into()),

      ToBackwalk => self.smear_animation("idle<>backwalk".into(), "backwalk".into()),

      ToCrouch => self.smear_animation("idle<>crouch".into(), "crouch".into()),

      WalkToIdle => self.smear_animation("walk<>idle".into(), "idle".into()),

      BackwalkToIdle => self.smear_animation("backwalk<>idle".into(), "idle".into()),

      DashToIdle => self.smear_animation("dash<>idle".into(), "idle".into()),

      BackDashToIdle => self.smear_animation("backdash<>idle".into(), "idle".into()),

      ToAirdash => self.smear_animation("fall<>airdash".into(), "airdash".into()),

      ToAirBackdash => self.smear_animation("fall<>backairdash".into(), "backairdash".into()),

      RiseToFall => self.smear_animation("rise<>fall".into(), "fall".into()),

      FallToIdle => self.smear_animation("fall<>idle".into(), "idle".into()),

      AirdashToFall => self.smear_animation("airdash<>fall".into(), "fall".into()),

      AirbackdashToFall => self.smear_animation("backairdash<>fall".into(), "fall".into()),

      CrouchToIdle => self.smear_animation("crouch<>idle".into(), "idle".into()),
    }
    self.reset();
  }

  fn update_animation(&mut self, state: AnimationState, smear: Option<String>, core: String) {
    self.animation_state = state;
    self.smear_animation = smear.map(|s| self.get_animation(s).unwrap());
    let core_animation = self.get_animation(core);

    if let Some(ca) = core_animation {
      self.core_animation = ca;
    }
  }
  
  fn loop_animation(&mut self, animation: String) {
    self.update_animation(AnimationState::LOOPING, None, animation);
  }

  fn smear_animation(&mut self, smear_animation: String, core_animation: String) {
    self.update_animation(AnimationState::SMEARING, Some(smear_animation), core_animation);
  }
}

#[doc(hidden)]
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

#[doc(hidden)]
pub fn animate_sprite_system(
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationController)>,
) {
  for (mut sprite, mut anim_controller) in query.iter_mut() {
    sprite.index = anim_controller.get_next_frame();
  }
}
