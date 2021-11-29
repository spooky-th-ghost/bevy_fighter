use std::default;

pub use crate::prelude::*;


/// How the player should move on the next frame
pub struct PhysicsState {
  pub velocity: Vec2,
  pub gravity: f32,
  pub collidable: bool,
  pub is_grounded: bool,
}

impl Default for PhysicsState {
  fn default() -> Self {
    {
      PhysicsState {
        velocity: Vec2::ZERO,
        gravity: 0.0,
        collidable: true,
        is_grounded: true,
      }
    }
  }
}

/// Primary way to handle if a player can perform an input
pub struct ActionState {
  pub busy_duration: u8,
  pub invuln: u8,
  pub armor_duration: u8,
  pub facing_right: bool,
  pub player_state_name: PlayerStateName,
  pub armor_type: Option<ArmorType>,
  pub cancellable_actions: Option<Vec<ActionType>>
}

impl Default for ActionState {
  fn default() -> Self {
      ActionState {
        busy_duration: 0,
        invuln: 0,
        armor_duration: 0,
        facing_right: true,
        player_state_name: PlayerStateName::default(),
        armor_type: None,
        cancellable_actions: None
      }
  }
}
pub enum PhysicsStateName {
  DASHING,
  ATTACKING,
  BLOCKING, 
}

pub enum PlayerStateName {
  DASHING,
  RUNNING,
  WALKING,
  ATTACKING,
  BLOCKING,
  JUMPING,
  JUGGLE,
  STANDING 
}

impl Default for PlayerStateName {
  fn default() -> Self {PlayerStateName::STANDING}
}


pub enum ArmorType {
  SUPER,
  HYPER,
}


pub enum ActionType {
  NORMAL,
  COMMAND_NORMAL,
  SPECIAL,
  SUPER,
  JUMP,
  DASH,
  AIRDASH,
  BACKDASH,
  AIR_BACKDASH
}


