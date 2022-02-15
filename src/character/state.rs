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
  BeatChain,
  CharacterMovementSerialized
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
  /// updates a character state, advancing it's timers and changing it based on input and character movement
  pub fn update(&mut self, buffer: &mut FighterInputBuffer, movement: &mut CharacterMovement) -> Option<AnimationTransition> {
    use CharacterState::*;
    self.tick();
    
    let new_state = match self {
      Idle | Walking | BackWalking | Crouching => self.from_neutral_states(buffer, movement),
      Dashing => self.from_dashing(buffer, movement),
      Jumpsquat { duration:_,velocity:_ } => self.from_jump_squat(movement),
      Rising { busy: _ } | Falling => self.from_neutral_airborne(buffer, movement),
      BackDashing { duration:_ } => self.from_backdashing(buffer, movement),
      Attacking {duration:_, attack:_, cancellable:_} => self.from_attacking(buffer, movement),
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
  pub fn from_neutral_states(&self, buffer: &FighterInputBuffer, movement: &mut CharacterMovement) -> Self {
    use CharacterState::*;
    if let Some(attack) = movement.attack_to_execute(buffer, false) {
      return self.buffer_attack(attack);
    }

    if let Some(ct) = buffer.command_type {
      match ct {
        CommandType::DASH => return Dashing,
        CommandType::BACK_DASH => return self.buffer_backdash(movement),
      _ => ()
      }               
    }

    match buffer.current_motion {
      4 => return BackWalking,
      6 => return Walking,
      1 | 2 | 3 => return Crouching,
      7 | 8 | 9 => return Self::buffer_jump(buffer.current_motion, &movement.clone(), false),
      _ => return Idle
    }
  }

  /// Returns a new state based on the current state when in jump squat
  pub fn from_jump_squat(&self, movement: &mut CharacterMovement) -> Self{
    use CharacterState::*;
    match self {
      Jumpsquat { duration, velocity } => {
        if *duration == 0 {
          movement.velocity = *velocity;
          return Rising {busy: movement.jump_lockout};
        } else {
          return self.clone();
        }
      },
      _ => return self.clone(),
    };
  }

  /// Returns a new state based on input from dashing
  pub fn from_dashing(&self, buffer: &FighterInputBuffer, movement: &CharacterMovement) -> Self {
    use CharacterState::*;
    match buffer.current_motion {
      4 => return BackWalking,
      6 => return Dashing,
      1 | 2 | 3 => return Crouching,
      7 | 8 | 9 => return Self::buffer_dash_jump(buffer.current_motion, movement, false),
      _ => return Idle
    }
  }

  /// Returns a new state based on input from the following states:
  ///  - Rising
  ///  - Falling
  ///  - Airdashing
  ///  - Airbackdashing
  pub fn from_neutral_airborne(&self, buffer: &FighterInputBuffer, movement: &mut CharacterMovement) -> Self {
    use CharacterState::*;
    match self {
      Rising { busy } => {
        if *busy == 0 {
          return self.from_airborne_input(buffer, movement);
        } else {
          return self.clone();
        }
      },
      Falling => {
        return self.from_airborne_input(buffer, movement);
      }
      _ => return self.clone(),
    };
  }

  /// Returns a new state based on input and the backdash timer from backdash
  pub fn from_backdashing(&self, buffer: &FighterInputBuffer, movement: &mut CharacterMovement) -> Self {
    use CharacterState::*;
    match self {
      BackDashing {duration} => {
        if *duration == 0 {
          return self.from_neutral_states(buffer, movement);
        }
        return self.clone();
      },
      _ => return self.clone(),
    }
  }

  /// Returns a new state based on input and the attack timer from attack
  pub fn from_attacking(&self, buffer: &FighterInputBuffer, movement: &mut CharacterMovement) -> Self {
    use CharacterState::*;
    match self {
      Attacking {duration, attack:_, cancellable} => {
        if *duration == 0 || *cancellable {
          return self.from_neutral_states(buffer, movement);
        }
        return self.clone();
      },
      _ => return self.clone(),
    }
  }

  // Returns a new state from input while aireborne
  pub fn from_airborne_input (&self, buffer: &FighterInputBuffer, movement: &mut CharacterMovement) -> Self {
    if let Some(attack) = movement.attack_to_execute(buffer, true) {
      return self.buffer_attack(attack);
    }

    if let Some(ct) = buffer.command_type {
      match ct {
        CommandType::DASH => return self.buffer_airdash(movement, true),
        CommandType::BACK_DASH => return self.buffer_airdash(movement, false),
      _ => ()
      }               
    }

    return self.clone();
  }

  /// Returns an attacking state, with the passed attack
  fn buffer_attack(&self, attack: Attack) -> Self {
    return CharacterState::Attacking {duration: attack.busy, attack: attack.clone(), cancellable: false}
  }

  /// Returns a backdashing state, based on movement
  fn buffer_backdash(&self, movement: &mut CharacterMovement) -> Self {
    use Backdash::*;
    match movement.backdash {
      Standard {speed, busy, motion_duration} => {
        let i_force = InterpolatedForce::new(
          Vec2::new(-speed * movement.facing_vector, 0.0),
          Vec2::new(-2.0 * movement.facing_vector, 0.0),
          motion_duration
        );
        movement.set_interpolated_force(i_force);
        return CharacterState::BackDashing {duration: busy}
      },
      _ => return Self::Idle
    }
  }

  fn buffer_airdash(&self, movement: &mut CharacterMovement, forward: bool) -> Self {
    use CharacterState::*;
    if forward {
        return AirDashing {busy: 10, duration: movement.max_airdash_time, velocity: Vec2::X * movement.air_dash_speed * movement.facing_vector};
      } else {
        return AirBackDashing {busy: 10, duration: movement.max_air_backdash_time, velocity: Vec2::X * movement.air_dash_speed * -movement.facing_vector };
      }
  }

  /// Returns a Jumpsquat state from a Dash state, with a buffered jump based on character movement and input buffer
  fn buffer_dash_jump(motion: u8, movement: &CharacterMovement, superjump: bool) -> Self {
    let x_velocity = match motion {
      7 => movement.facing_vector * (-movement.back_walk_speed),
      9 => movement.facing_vector * (movement.walk_speed * 2.0),
      _ => movement.facing_vector * (movement.walk_speed * 0.5)
    };

    let y_velocity = if superjump {
      movement.jump_height * 1.25
    } else {
      movement.jump_height
    };
    
    let velocity = Vec2::new(x_velocity, y_velocity);
    return Self::Jumpsquat {duration: 3, velocity}
  }

  /// Returns a Jumpsquat state from a neutral state, with a buffered jump based on character movement and input buffer
  fn buffer_jump(motion:u8, movement: &CharacterMovement, superjump: bool) -> Self {
    let x_velocity = match motion {
      7 => movement.facing_vector * (-movement.back_walk_speed*1.75),
      9 => movement.facing_vector * (movement.walk_speed),
      _ => 0.0
    };

    let y_velocity = if superjump {
      movement.jump_height * 1.25
    } else {
      movement.jump_height
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

#[derive(Component, Clone, Debug, Default)]
pub struct CharacterMovement {
  pub jumpsquat: u8,
  pub air_jumps: u8,
  pub air_jumps_remaining: u8,
  pub airdashes: u8,
  pub airdashes_remaining: u8,
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

impl CharacterMovement {
    pub fn from_serialized(s: CharacterMovementSerialized, library: &CharacterLibrary, character_name: &str) -> Self {
    let mut attacks = HashMap::new();
    let mut attack_names = Vec::new();
    let my_regex = Regex::new(&format!("(^{}.+)", character_name)[..]).unwrap();
    for (attack_id, attack) in library.read_attacks() {
      if my_regex.is_match(attack_id) {
        let trimmed_attack_name = attack_id.replace(character_name, "").replace("_","");
        attack_names.push(trimmed_attack_name.clone());
        attacks.insert(trimmed_attack_name.clone(), attack.clone());
      }
    }

    CharacterMovement {
      jumpsquat: s.jumpsquat,
      jump_lockout: 10,
      air_jumps: s.air_jumps,
      airdashes: s.airdashes,
      air_dash_speed: s.air_dash_speed,
      air_back_dash_speed: s.air_back_dash_speed,
      walk_speed: s.walk_speed,
      back_walk_speed: s.back_walk_speed,
      dash_speed: s.dash_speed,
      gravity: s.gravity,
      jump_height: s.jump_height,
      max_airdash_time: s.max_airdash_time,
      max_air_backdash_time: s.max_air_backdash_time,
      backdash: s.backdash,
      facing_vector: 1.0,
      air_jumps_remaining: s.air_jumps,
      airdashes_remaining: s.airdashes,
      velocity: Vec2::ZERO,
      interpolated_force: None,
      attacks,
      beat_chain: BeatChain::from_attack_names(attack_names),
    }
  }
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

#[derive(Bundle, Default)]
pub struct FighterCharacterBundle {
  pub sprite: TextureAtlasSprite,
  pub texture_atlas: Handle<TextureAtlas>,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
  pub visibility: Visibility,
  pub player_id: PlayerId,
  pub state: CharacterState,
  pub movement: CharacterMovement,
  pub animation_controller: AnimationController
}

impl FighterCharacterBundle {
  pub fn new(player_id: PlayerId, character_prefix: &str, library: &CharacterLibrary) -> Self {
    let transform = match player_id {
      PlayerId::P1 => Transform::from_xyz(-40.0,0.0,0.0),
      PlayerId::P2 => Transform::from_xyz(40.0,0.0,0.0),
    };

    let movement = library.get_movement(character_prefix).unwrap();
    let texture_atlas = library.get_atlas(character_prefix).unwrap();
    
    FighterCharacterBundle {
      movement,
      texture_atlas,
      transform,
      player_id,
      animation_controller: AnimationController::new(character_prefix, library),
      ..Default::default()
    }
  }
}

// Manage and update ChracterState for all characters based on input
pub fn manage_player_state(
  mut player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterState, &mut CharacterMovement)>,
  mut transition_writer: EventWriter<AnimationTransitionEvent>,
) {
  for (player_id, mut state, mut movement) in query.iter_mut() {
    for buffer in player_data.buffers.iter_mut() {
      if buffer.player_id == *player_id {
        let transition = state.update(buffer,&mut movement);
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
  mut query: Query<(&PlayerId, &mut CharacterState, &mut CharacterMovement)>,
) {
  for (player_id, state, movement) in query.iter_mut() {
    movement.determine_velocity(&state);
  }
}
