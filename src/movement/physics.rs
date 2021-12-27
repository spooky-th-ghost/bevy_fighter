use std::default;
use lerp::Lerp;
pub use crate::prelude::*;

trait CustomLerp {
  fn custom_lerp(&self, target: Self, t: f32) -> Self;
}

impl CustomLerp for Vec2 {
  fn custom_lerp(&self, target: Vec2, t: f32) -> Vec2 {
    let _x = self.x.lerp(target.x,t);
    let _y = self.y.lerp(target.y,t);
    return Vec2::new(_x,_y);
  }
}

#[derive(Clone, Copy)]
pub struct InterpolatedForce {
  current_velocity: Vec2,
  starting_velocity: Vec2,
  duration: u8,
  frames_elapsed: u8
}

impl InterpolatedForce {

  pub fn new(starting_velocity: Vec2, duration: u8) -> Self {
    return InterpolatedForce {
      current_velocity: starting_velocity,
      starting_velocity,
      duration,
      frames_elapsed: 0
    }
  }

  pub fn update(&mut self) -> Vec2 {
    self.frames_elapsed += 1;
    let t = (self.frames_elapsed / self.duration) as f32;
    self.current_velocity = self.current_velocity.custom_lerp(Vec2::ZERO,t);
    return self.current_velocity;
  }
}

/// How the player should move on the next frame
pub struct PhysicsState {
  pub velocity: Vec2,
  pub gravity: f32,
  pub collidable: bool,
  pub is_grounded: bool,
  pub int_force: Option<InterpolatedForce>
}

impl Default for PhysicsState {
  fn default() -> Self {
    {
      PhysicsState {
        velocity: Vec2::ZERO,
        gravity: 0.0,
        collidable: true,
        is_grounded: true,
        int_force: None
      }
    }
  }
}

/// Primary way to handle if a player can perform an input
#[derive(Clone, Copy)]
pub struct ActionState {
  pub busy_duration: u8,
  pub invuln: u8,
  pub armor_duration: u8,
  pub facing_right: bool,
  pub player_state_name: PlayerStateName,
  pub armor_type: Option<ArmorType>,
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
      }
  }
}
pub enum PhysicsStateName {
  DASHING,
  ATTACKING,
  BLOCKING, 
}

#[derive(Clone,Copy)]
pub enum PlayerStateName {
  DASHING,
  RUNNING,
  WALKING,
  ATTACKING,
  BLOCKING,
  JUMPING,
  JUGGLE,
  STANDING,
  BACKDASHING 
}

impl Default for PlayerStateName {
  fn default() -> Self {PlayerStateName::STANDING}
}


#[derive(Clone,Copy)]
pub enum ArmorType {
  SUPER,
  HYPER,
}

#[derive(Clone, Copy)]
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


