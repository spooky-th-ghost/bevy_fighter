pub use crate::prelude::*;
pub use bevy::prelude::*;

/// How the player should move on the next frame
pub struct PhysicsState {
  pub velocity: Vec2,
  pub gravity: f32,
  pub collidable: bool,
}

/// Primary way to handle if a player can perform an input
pub struct PlayerState {
  pub busy_duration: u8,
  pub invuln: u8,
  pub armor_duration: u8,
  pub facing_right: bool,
  pub player_state_name: PlayerStateName,
  pub armor_type: Option<ArmorType>,
  pub cancellable_actions: Option<Vec<ActionType>>
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
