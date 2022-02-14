use bevy::prelude::*;
use regex::Regex;
use std::collections::HashMap;

use crate::{
  character_library::CharacterLibrary,
  attacks::Attack,
  inputs::{
    FighterInputBuffer,
    PlayerData,
    CommandType
  },
  utils::countdown,
  physics::{
    InterpolatedForce,
    CustomLerp
  },
  animation::{
    AnimationTransitionEvent,
    AnimationTransition, AnimationController
  }
};

use super::{
  Backdash,
  PlayerId,
  BeatChain
};

/// Handles the current state of a character
#[derive(Debug, Clone, Component)]
pub enum CharacterState {
  Idle,
  Walking,
  BackWalking,
  Attacking {
    ///The number of frames until the action completes naturally
    duration: u8,
    ///The current attack being executed 
    attack: Attack,
    ///Can the current attack me cancelled prematurely 
    cancellable: bool
  },
  AttackingAirborne {
    ///The number of frames until the action completes naturally
    duration: u8, 
    ///The current attack being executed 
    attack: Attack,
    ///Can the current attack me cancelled prematurely 
    cancellable: bool
  },
  Crouching,
  Jumpsquat {
    ///The number of frames until the action completes naturally
    duration: u8,
    ///The Velocity of the buffered jump 
    velocity: Vec2 
  },
  AirJumpsquat {
    ///The number of frames until the action completes naturally
    duration: u8,
    ///The velocity of the buffered jump  
    velocity: Vec2 
  },
  Rising {
    ///The number of frames until the player can act out of the state
    busy: u8
  },
  Falling,
  Juggle,
  Dashing,
  BackDashing {
    ///The number of frames until the action completes naturally
    duration: u8
  },
  AirDashing {
    ///The number of frames until the player can act out of the state
    busy: u8,
    ///The number of frames until the action completes naturally
    duration: u8,
    ///The velocity of the air dash 
    velocity: Vec2
  },
  AirBackDashing {
    ///The number of frames until the player can act out of the state
    busy: u8,
    ///The number of frames until the action completes naturally 
    duration: u8,
    ///The velocity of the air dash
    velocity: Vec2
  } 
}

impl PartialEq for CharacterState {
  fn eq(&self, other: &Self) -> bool {
    std::mem::discriminant(self) == std::mem::discriminant(other)
  }
}

impl Default for CharacterState {
  fn default() -> Self {
    CharacterState::Idle
  }
}

impl CharacterState {
  fn tick(&mut self) {
    use CharacterState::*;
    match self {
      Attacking {duration, attack: _, cancellable:_} => { *duration = countdown(*duration);},
      Jumpsquat {duration, velocity:_ } => { *duration = countdown(*duration);},
      AirJumpsquat {duration, velocity: _ } => { *duration = countdown(*duration);},
      BackDashing {duration} => { *duration = countdown(*duration);},
      AirDashing {busy,duration, velocity:_} => {
        *busy = countdown(*busy); 
        *duration = countdown(*duration);
      },
      AirBackDashing {busy, duration, velocity:_} => {
        *busy = countdown(*busy); 
         *duration = countdown(*duration);
        },
      _ => () 
    }
  } 
  /// updates a character state, advancing it's timers and changing it based on input and character stats
  pub fn update(&mut self, buffer: &mut FighterInputBuffer, stats: &mut CharacterStats) -> Option<AnimationTransition> {
    use CharacterState::*;
    self.tick();
    
    let new_state = match self {
      Idle | Walking | BackWalking | Crouching => self.from_neutral_states(buffer, stats),
      Dashing => self.from_dashing(buffer, stats),
      Jumpsquat { duration:_,velocity:_ } => self.from_jump_squat(stats),
      Rising { busy: _ } | Falling => self.from_neutral_airborne(buffer, stats),
      BackDashing { duration:_ } => self.from_backdashing(buffer, stats),
      Attacking {duration:_, attack:_, cancellable:_} => self.from_attacking(buffer, stats),
      _ => self.clone()
    };
    let transition = if self.clone() != new_state {
      self.calculate_transition(&new_state)
    } else {
      None
    };
    *self = new_state;
    return transition;
  }

  /// Returns whether or not the character is in the air
  pub fn is_airborne(&self) -> bool {
    use CharacterState::*;
    match self {
      AirJumpsquat {duration:_, velocity:_}
      | Rising {busy:_}
      | Falling
      | AirDashing {busy:_, duration:_, velocity:_}
      | AirBackDashing {busy:_, duration:_, velocity:_} => return true,
      _ => return false
    }
  }

  /// Returns a new state based on input from the following states:
  ///  - Idle
  ///  - Walking
  ///  - Backwalking
  ///  - Crouching
  pub fn from_neutral_states(&self, buffer: &FighterInputBuffer, stats: &mut CharacterStats) -> Self {
    use CharacterState::*;
    if let Some(attack) = stats.attack_to_execute(buffer, false) {
      return self.buffer_attack(attack);
    }

    if let Some(ct) = buffer.command_type {
      match ct {
        CommandType::DASH => return Dashing,
        CommandType::BACK_DASH => return self.buffer_backdash(stats),
      _ => ()
      }               
    }

    match buffer.current_motion {
      4 => return BackWalking,
      6 => return Walking,
      1 | 2 | 3 => return Crouching,
      7 | 8 | 9 => return Self::buffer_jump(buffer.current_motion, &stats.clone(), false),
      _ => return Idle
    }
  }

  /// Returns a new state based on the current state when in jump squat
  pub fn from_jump_squat(&self, stats: &mut CharacterStats) -> Self{
    use CharacterState::*;
    match self {
      Jumpsquat { duration, velocity } => {
        if *duration == 0 {
          stats.velocity = *velocity;
          return Rising {busy: stats.jump_lockout};
        } else {
          return self.clone();
        }
      },
      _ => return self.clone(),
    };
  }

  /// Returns a new state based on input from dashing
  pub fn from_dashing(&self, buffer: &FighterInputBuffer, stats: &CharacterStats) -> Self {
    use CharacterState::*;
    match buffer.current_motion {
      4 => return BackWalking,
      6 => return Dashing,
      1 | 2 | 3 => return Crouching,
      7 | 8 | 9 => return Self::buffer_dash_jump(buffer.current_motion, stats, false),
      _ => return Idle
    }
  }

  /// Returns a new state based on input from the following states:
  ///  - Rising
  ///  - Falling
  ///  - Airdashing
  ///  - Airbackdashing
  pub fn from_neutral_airborne(&self, buffer: &FighterInputBuffer, stats: &mut CharacterStats) -> Self {
    use CharacterState::*;
    match self {
      Rising { busy } => {
        if *busy == 0 {
          return self.from_airborne_input(buffer, stats);
        } else {
          return self.clone();
        }
      },
      Falling => {
        return self.from_airborne_input(buffer, stats);
      }
      _ => return self.clone(),
    };
  }

  /// Returns a new state based on input and the backdash timer from backdash
  pub fn from_backdashing(&self, buffer: &FighterInputBuffer, stats: &mut CharacterStats) -> Self {
    use CharacterState::*;
    match self {
      BackDashing {duration} => {
        if *duration == 0 {
          return self.from_neutral_states(buffer, stats);
        }
        return self.clone();
      },
      _ => return self.clone(),
    }
  }

  /// Returns a new state based on input and the attack timer from attack
  pub fn from_attacking(&self, buffer: &FighterInputBuffer, stats: &mut CharacterStats) -> Self {
    use CharacterState::*;
    match self {
      Attacking {duration, attack:_, cancellable} => {
        if *duration == 0 || *cancellable {
          return self.from_neutral_states(buffer, stats);
        }
        return self.clone();
      },
      _ => return self.clone(),
    }
  }

  // Returns a new state from input while aireborne
  pub fn from_airborne_input (&self, buffer: &FighterInputBuffer, stats: &mut CharacterStats) -> Self {
    if let Some(attack) = stats.attack_to_execute(buffer, true) {
      return self.buffer_attack(attack);
    }

    if let Some(ct) = buffer.command_type {
      match ct {
        CommandType::DASH => return self.buffer_airdash(stats, true),
        CommandType::BACK_DASH => return self.buffer_airdash(stats, false),
      _ => ()
      }               
    }

    return self.clone();
  }

  /// Returns an attacking state, with the passed attack
  fn buffer_attack(&self, attack: Attack) -> Self {
    return CharacterState::Attacking {duration: attack.busy, attack: attack.clone(), cancellable: false}
  }

  /// Returns a backdashing state, based on stats
  fn buffer_backdash(&self, stats: &mut CharacterStats) -> Self {
    use Backdash::*;
    match stats.backdash {
      Standard {speed, busy, motion_duration} => {
        let i_force = InterpolatedForce::new(
          Vec2::new(-speed * stats.facing_vector, 0.0),
          Vec2::new(-2.0 * stats.facing_vector, 0.0),
          motion_duration
        );
        stats.set_interpolated_force(i_force);
        return CharacterState::BackDashing {duration: busy}
      },
      _ => return Self::Idle
    }
  }

  fn buffer_airdash(&self, stats: &mut CharacterStats, forward: bool) -> Self {
    use CharacterState::*;
    if forward {
        return AirDashing {busy: 10, duration: stats.max_airdash_time, velocity: Vec2::X * stats.air_dash_speed * stats.facing_vector};
      } else {
        return AirBackDashing {busy: 10, duration: stats.max_air_backdash_time, velocity: Vec2::X * stats.air_dash_speed * -stats.facing_vector };
      }
  }

  /// Returns a Jumpsquat state from a Dash state, with a buffered jump based on character stats and input buffer
  fn buffer_dash_jump(motion: u8, stats: &CharacterStats, superjump: bool) -> Self {
    let x_velocity = match motion {
      7 => stats.facing_vector * (-stats.back_walk_speed),
      9 => stats.facing_vector * (stats.walk_speed * 2.0),
      _ => stats.facing_vector * (stats.walk_speed * 0.5)
    };

    let y_velocity = if superjump {
      stats.jump_height * 1.25
    } else {
      stats.jump_height
    };
    
    let velocity = Vec2::new(x_velocity, y_velocity);
    return Self::Jumpsquat {duration: 3, velocity}
  }

  /// Returns a Jumpsquat state from a neutral state, with a buffered jump based on character stats and input buffer
  fn buffer_jump(motion:u8, stats: &CharacterStats, superjump: bool) -> Self {
    let x_velocity = match motion {
      7 => stats.facing_vector * (-stats.back_walk_speed*1.75),
      9 => stats.facing_vector * (stats.walk_speed),
      _ => 0.0
    };

    let y_velocity = if superjump {
      stats.jump_height * 1.25
    } else {
      stats.jump_height
    };
    
    let velocity = Vec2::new(x_velocity, y_velocity);
    return Self::Jumpsquat {duration: 3, velocity}
  }

  /// If the new state does not match the old state, generate an animation transition
  fn calculate_transition(&self, other: &Self) -> Option<AnimationTransition> {
    use CharacterState::*;
    use AnimationTransition::*;
    match (self, other) {
      (Crouching,Idle) => Some(CrouchToIdle),
      (Falling,Idle) => Some(FallToIdle),
      (Walking,Idle) => Some(WalkToIdle),
      (BackWalking,Idle) => Some(BackwalkToIdle),
      (Dashing,Idle) => Some(DashToIdle),
      (BackDashing { duration:_},Idle) => Some(BackDashToIdle),
      (AirDashing {busy:_, duration:_, velocity:_}, Falling) => Some(AirdashToFall),
      (AirBackDashing {busy:_, duration:_, velocity:_}, Falling) => Some(AirbackdashToFall),
      (_, Idle) => Some(ToIdle),
      (_, Jumpsquat {duration:_, velocity:_}) => Some(ToRise),
      (_, Walking) => Some(ToWalk),
      (_, BackWalking) => Some(ToBackwalk),
      (_, Dashing) => Some(ToDash),
      (_, BackDashing {duration:_}) => Some(ToBackdash),
      (_, AirDashing {busy:_, duration:_, velocity:_}) => Some(ToAirdash),
      (_, AirBackDashing {busy:_, duration:_, velocity:_}) => Some(ToAirBackdash),
      (_, Crouching) => Some(ToCrouch),
      (_, Attacking {duration:_, attack, cancellable:_}) => Some(ToAttack {name: attack.name.clone()}),
      (_,_) => None
    }
  }
}

// Manage and update ChracterState for all characters based on input
pub fn manage_player_state(
  mut player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterState, &mut CharacterStats)>,
  mut transition_writer: EventWriter<AnimationTransitionEvent>,
) {
  for (player_id, mut state, mut stats) in query.iter_mut() {
    for buffer in player_data.buffers.iter_mut() {
      if buffer.player_id == *player_id {
        let transition = state.update(buffer,&mut stats);
        if let Some(t) = transition {
          transition_writer.send(
      AnimationTransitionEvent {
              player_id: *player_id,
              transition: t,
            }
          );
        }
      }
    }
  }
}

/// Manage and update velocity based on player state
pub fn manage_player_velocity (
  mut player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterState, &mut CharacterStats)>,
) {

}

#[derive(Component, Clone, Debug, Default)]
pub struct CharacterStats {
  pub jumpsquat: u8,
  pub air_jumps: u8,
  pub airdashes: u8,
  pub air_dash_speed: f32,
  pub air_back_dash_speed: f32,
  pub jump_lockout: u8,
  pub walk_speed: f32,
  pub back_walk_speed: f32,
  pub dash_speed: f32,
  pub gravity: f32,
  pub jump_height: f32,
  pub max_airdash_time: u8,
  pub max_air_backdash_time: u8,
  pub backdash: Backdash,
  pub attacks: HashMap<String, Attack>,
  pub beat_chain: BeatChain,
  pub facing_vector: f32,
  pub velocity: Vec2,
  pub interpolated_force: Option<InterpolatedForce>,
}

impl CharacterStats {
  pub fn determine_velocity(&mut self, state: &CharacterState) {
    use CharacterState::*;
    self.velocity = match state {
      Walking => Vec2::X * self.facing_vector * self.walk_speed,
      BackWalking => Vec2::X * self.facing_vector * self.walk_speed,
      Rising {busy:_} | Falling | Juggle => self.velocity - (Vec2::Y * self.gravity),
      Dashing => Vec2::X * self.facing_vector * self.dash_speed,
      BackDashing {duration:_} => Vec2::ZERO,
      AirDashing {busy:_, duration:_, velocity} => *velocity,
      AirBackDashing {busy:_, duration:_, velocity} => *velocity,
      _ => self.velocity.custom_lerp(Vec2::ZERO, 0.5)
    }
  }
  pub fn attack_to_execute(&mut self,  buffer: &FighterInputBuffer, airborne: bool) -> Option<Attack> {
    if buffer.current_press.any_pressed() {
      return self.find_attack(buffer.current_motion, buffer.current_press.to_string());
    } else {
      return None;
    }
  }

  pub fn find_attack(&mut self, motion: u8, buttons: String) -> Option<Attack> {
    let mut current_regex: Regex;
    for button in buttons.chars().rev() {
      current_regex = Regex::new(&format!("({}).*({})", motion, button)[..]).unwrap();
      for attack_name in self.beat_chain.available_attacks.iter() {
        if current_regex.is_match(attack_name) {
          return self.attacks.get(attack_name).cloned();
        }
      }
    }
    return None;
  }

  pub fn set_interpolated_force(&mut self, i_force: InterpolatedForce) {
    self.interpolated_force = Some(i_force);
  }
}

#[derive(Component, Clone, Debug)]
pub struct CharacterPhysics {
  velocity: Vec2,
  interpolated_force: InterpolatedForce,
}

#[derive(Bundle, Default)]
pub struct FighterCharacterBundle {
  pub sprite: TextureAtlasSprite,
  pub texture_atlas: Handle<TextureAtlas>,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
  pub visibility: Visibility,
  pub player_id: PlayerId,
  pub state: CharacterState,
  pub movement: CharacterStats,
  pub animation_controller: AnimationController
}

// impl FighterCharacterBundle {
//   pub fn new(player_id: PlayerId, character_prefix: &str, library: &CharacterLibrary) -> Self {
//     let transform = match player_id {
//       PlayerId::P1 => Transform::from_xyz(-40.0,0.0,0.0),
//       PlayerId::P2 => Transform::from_xyz(40.0,0.0,0.0),
//     };

//     let movement = library.get_movement(character_prefix).unwrap();
//     let texture_atlas = library.get_atlas(character_prefix).unwrap();
    
//     FighterCharacterBundle {
//       movement,
//       texture_atlas,
//       transform,
//       player_id,
//       animation_controller: AnimationController::new(character_prefix, library),
//       ..Default::default()
//     }
//   }
// }
