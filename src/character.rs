#[cfg(feature = "debug")]
use bevy_inspector_egui::Inspectable;

use bevy::{
  diagnostic::{
    Diagnostics,
    FrameTimeDiagnosticsPlugin
  },
  prelude::*
};

use regex::Regex;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::{
  character_library::CharacterLibrary,
  attacks::Attack,
  collision::HitboxEvent,
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

/// Handles the current state of a character
#[cfg_attr(feature = "debug", derive(Inspectable))]
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
      Rising {busy} => {*busy = countdown(*busy)},
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
  pub fn update(&mut self, buffer: &mut FighterInputBuffer, movement: &mut CharacterMovement, position: Vec3) -> Option<AnimationTransition> {
    use CharacterState::*;
    self.tick();
    
    let new_state = match self {
      Idle | Walking | BackWalking | Crouching => self.from_neutral_states(buffer, movement),
      Dashing => self.from_dashing(buffer, movement),
      Jumpsquat { duration:_,velocity:_ } => self.from_jump_squat(movement),
      Rising { busy: _ } | Falling => self.from_neutral_airborne(buffer, movement, position),
      BackDashing { duration:_ } => self.from_backdashing(buffer, movement),
      Attacking {duration:_, attack:_, cancellable:_} => self.from_attacking(buffer, movement),
      AirDashing { busy:_,duration:_,velocity:_} | AirBackDashing { busy:_,duration:_,velocity:_} => self.from_air_dashing(buffer, movement),
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

  pub fn from_air_dashing(&self, buffer: &FighterInputBuffer, movement: &mut CharacterMovement) -> Self {
    use CharacterState::*;
    match self {
      AirDashing {busy:_ ,duration, velocity:_} => {
        if *duration == 0 {
          return self.from_neutral_airborne(buffer, movement, Vec3::ONE);
        }
        return self.clone();
      },
      AirBackDashing {busy:_,duration, velocity:_} => {
        if *duration == 0 {
          return self.from_neutral_airborne(buffer, movement, Vec3::ONE);
        }
        return self.clone();
      },
      _ => return self.clone(),
    }
  }

  /// Returns a new state based on input from the following states:
  ///  - Rising
  ///  - Falling
  ///  - Airdashing
  ///  - Airbackdashing
  pub fn from_neutral_airborne(&self, buffer: &FighterInputBuffer, movement: &mut CharacterMovement, position: Vec3) -> Self {
    use CharacterState::*;
    if position.y <= 0.0 {
      return Idle;
    }
    match self {
      Rising { busy } => {
        if *busy == 0 {
          return self.from_airborne_input(buffer, movement);
        } else {
          return self.clone();
        }
      },
      Falling | AirDashing {busy:_,duration:_,velocity:_} |  AirBackDashing {busy:_,duration:_,velocity:_} => {
        return self.from_airborne_input(buffer, movement);
      },
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
    use CharacterState::*;
    if let Some(attack) = movement.attack_to_execute(buffer, true) {
      return self.buffer_attack(attack);
    }

    if movement.can_airdash() {
      if let Some(ct) = buffer.command_type {
        match ct {
          CommandType::DASH => {
            movement.spend_airdash();
            return self.buffer_airdash(movement, true)
          },
          CommandType::BACK_DASH => {
            movement.spend_airdash();
            return self.buffer_airdash(movement, false)
          },
        _ => ()
        }               
      }
    }

    return match self {
      AirDashing {busy:_, duration:_,velocity:_} => {
        if self.is_finished_airdashing() {
          Falling
        } else {
          self.clone()
        }
      },
      AirBackDashing {busy:_, duration:_,velocity:_} => {
        if self.is_finished_airdashing() {
          Falling
        } else {
          self.clone()
        }
      },
      Rising {busy:_} => {
        if movement.is_falling() {
          Falling
        } else {
          self.clone()
        }
      }
      _ => self.clone()
    }
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
      (Rising {busy:_}, Falling) => Some(RiseToFall),
      (Falling, Idle) | (Rising {busy:_}, Idle) => Some(FallToIdle),
      (Crouching,Idle) => Some(CrouchToIdle),
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

  pub fn get_hitbox_events_this_frame(&self) -> Option<Vec<HitboxEvent>> {
    use CharacterState::*;
    if let Attacking{duration, attack, cancellable: _} = self.clone() {
      let mut events = Vec::new();
      for e in attack.hitbox_events.iter() {
        if (attack.busy as i8 - e.frame as i8) == duration as i8 {
          events.push(e.clone());
        }
      }
      if events.len() != 0 {
        return Some(events);
      } else {
        return None;
      }
    } else {
      return None;
    }
  }

  /// Returns whether or not the character can turn around, based on current state
  pub fn get_can_turn(&self) -> bool {
    use CharacterState::*;
    match self {
      Idle
      | Walking
      | BackWalking
      | Crouching
      | Rising {busy:_}
      | Falling => return true,
      _ => return false
    }
  }

  /// Returns whether or not the character is in the air, based on current state
  pub fn get_airborne(&self) -> bool {
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

  pub fn is_finished_airdashing(&self) -> bool {
    use CharacterState::*;
    match self {
      AirDashing {busy:_, duration,velocity:_} => return *duration == 0,
      AirBackDashing {busy:_, duration,velocity:_} => return *duration == 0,
      _ => return false,
    }
  }

  pub fn can_act_out_of_airdash(&self) -> bool {
    use CharacterState::*;
    match self {
      AirDashing {busy, duration:_,velocity:_} => return *busy == 0,
      AirBackDashing {busy, duration:_,velocity:_} => return *busy == 0,
      _ => return false,
    }
  }

  /// Called when the character lands, forcing them into a Idle state
  pub fn land(&mut self) {
    use CharacterState::*;
    *self = Idle;
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
  pub can_turn: bool,
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
      can_turn: true,
    }
  }
  pub fn determine_velocity(&mut self, state: &CharacterState) {
    use CharacterState::*;
    self.velocity = match state {
      Walking => Vec2::X * self.facing_vector * self.walk_speed,
      BackWalking => Vec2::X * -self.facing_vector * self.walk_speed,
      Rising {busy:_} | Falling | Juggle => self.velocity - (Vec2::Y * self.gravity),
      Dashing => Vec2::X * self.facing_vector * self.dash_speed,
      BackDashing {duration:_} => Vec2::ZERO,
      AirDashing {busy:_, duration:_, velocity} => *velocity,
      AirBackDashing {busy:_, duration:_, velocity} => *velocity,
      _ => self.velocity.custom_lerp(Vec2::ZERO, 0.5)
    }
  }

  pub fn get_target_velo(&mut self) -> Vec2 {
    if let Some(i_force) = self.interpolated_force.as_mut() {
      let i_force_velo = i_force.update();
      if i_force.is_finished() {self.interpolated_force = None;}
      return i_force_velo + self.velocity;
    } else {
      return self.velocity;
    }
  }

  pub fn is_falling(&self) -> bool {
    return self.velocity.y < 0.0;
  }

  pub fn attack_to_execute(&mut self,  buffer: &FighterInputBuffer, _airborne: bool) -> Option<Attack> {
    if buffer.current_press.any_pressed() {
      return self.find_attack(buffer.current_motion, buffer.current_press.to_string());
    } else {
      return None;
    }
  }

  pub fn can_airdash(&self) -> bool {
    return self.airdashes_remaining > 0;
  }

  pub fn spend_airdash(&mut self) {
    self.airdashes_remaining = countdown(self.airdashes_remaining);
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

  pub fn land(&mut self) {
    self.air_jumps_remaining = self.air_jumps;
    self.airdashes_remaining = self.airdashes;
  }

  pub fn set_interpolated_force(&mut self, i_force: InterpolatedForce) {
    self.interpolated_force = Some(i_force);
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
  P1,
  P2
}

impl Default for PlayerId {
  fn default() -> Self {
    PlayerId::P1
  }
}

#[derive(Serialize,Deserialize)]
pub struct CharacterMovementSerialized {
  pub jumpsquat: u8,
  pub air_jumps: u8,
  pub airdashes: u8,
  pub air_dash_speed: f32,
  pub air_back_dash_speed: f32,
  pub walk_speed: f32,
  pub back_walk_speed: f32,
  pub dash_speed: f32,
  pub gravity: f32,
  pub jump_height: f32,
  pub max_airdash_time: u8,
  pub max_air_backdash_time: u8,
  pub backdash: Backdash
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Clone, Debug, Default)]
pub struct BeatChain {
  pub all_attacks: Vec<String>,
  pub available_attacks: Vec<String>,
}

impl BeatChain {
  pub fn from_attack_names(attack_names: Vec<String>) -> Self {
    BeatChain {
      all_attacks: attack_names.clone(),
      available_attacks: attack_names.clone()
    }
  }

  pub fn reset(&mut self) {
    self.available_attacks = self.all_attacks.clone();
  }
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Backdash {
  Standard {busy: u8, speed: f32, motion_duration: u8},
  Teleport {busy: u8, distance: f32, motion_duration: u8},
  Leap {busy: u8, motion_duration: u8}
}

impl Default for Backdash {
  fn default() -> Self {
    Backdash::Standard{busy: 0, speed: 0.0, motion_duration: 0}
  }
}

impl Backdash {
  pub fn get_duration(&self) -> u8 {
    match self {
      Backdash::Standard {busy, speed: _, motion_duration: _} => return *busy,
      Backdash::Teleport {busy, distance: _, motion_duration: _} => return *busy,
      Backdash::Leap {busy, motion_duration: _} => return *busy
    }
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

/// Manage and update ChracterState for all characters based on input
pub fn manage_character_state(
  mut player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterState, &mut CharacterMovement)>,
  mut transition_writer: EventWriter<AnimationTransitionEvent>,
) {
  for (player_id, mut state, mut movement) in query.iter_mut() {
    let position = player_data.get_position(player_id);
    for buffer in player_data.buffers.iter_mut() {
      if buffer.player_id == *player_id {
        let transition = state.update(buffer,&mut movement, position);
        if let Some(t) = transition {
            if t == AnimationTransition::FallToIdle {
              movement.land();
            }
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
pub fn manage_character_velocity (
  mut query: Query<(&CharacterState, &mut CharacterMovement)>,
) {
  for(state, mut movement) in query.iter_mut() {
    movement.determine_velocity(state);
    movement.can_turn = state.get_can_turn();
  }
}

/// Apply player velocity
pub fn apply_character_velocity (
  mut player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterMovement, &mut Transform, &mut TextureAtlasSprite)>,
) {
  for(player_id, mut movement, mut transform, mut sprite) in query.iter_mut() {
    let tv = movement.get_target_velo();
    transform.translation += Vec3::new(tv.x, tv.y, 0.0);
    if transform.translation.y < 0.0 {
      transform.translation.y = 0.0;
    }

    player_data.set_position(player_id, transform.translation);
    let facing_vector = player_data.get_facing_vector(player_id);
    if movement.can_turn {
      sprite.flip_x = facing_vector < 0.0; 
      movement.facing_vector = facing_vector;
    }
  }
}

fn character_landing(state: &mut CharacterState, movement: &mut CharacterMovement) {
  state.land();
  movement.land();
} 

pub fn update_debug_ui(
  mut q: QuerySet<(
    QueryState<(&mut Text, &PlayerId)>,
    QueryState<&CharacterMovement>
  )>,
  diagnostics: Res<Diagnostics>,
  player_data: Res<PlayerData>
) {
  let distance = player_data.get_distance();
  let mut player_text: Vec<Vec<String>> = Vec::new();
  let mut fps = 0.0;
  if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
      if let Some(fps_avg) = fps_diagnostic.average() {
          fps = fps_avg;
      }
  }

  for movement in q.q1().iter() {
    let mut my_strings: Vec<String> = Vec::new();  
    my_strings.push(format!("Airdashes: {:?} \n", movement.airdashes_remaining));
    my_strings.push(format!("Velocity: {:?} \n", movement.velocity));
    my_strings.push(format!("Facing Vector: {:?} \n", movement.facing_vector));
    my_strings.push(format!("FPS: {:.1} \n", fps));
    my_strings.push(format!("Distance: {:?}", distance));
    let strings_to_push = my_strings.clone();
    player_text.push(strings_to_push);
  }

  for (mut text, player_id) in q.q0().iter_mut() {
    let index = match player_id {
      PlayerId::P1 => 0,
      PlayerId::P2 => 1
    };
      text.sections[0].value = player_text[index][0].clone();
      text.sections[1].value = player_text[index][1].clone();
      text.sections[2].value = player_text[index][2].clone();
      text.sections[3].value = player_text[index][3].clone();
      text.sections[4].value = player_text[index][4].clone();
  }
}

