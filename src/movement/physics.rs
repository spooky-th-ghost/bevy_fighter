use lerp::Lerp;
pub use crate::prelude::*;

pub trait CustomLerp {
  fn custom_lerp(&self, target: Self, t: f32) -> Self;
}


impl CustomLerp for Vec2 {
  fn custom_lerp(&self, target: Vec2, t: f32) -> Vec2 {
    let _x = self.x.lerp(target.x,t);
    let _y = self.y.lerp(target.y,t);
    return Vec2::new(_x,_y);
  }
}


/// Used to add fake physics forces, instead of constant changes to velocity, 
/// interpolated forces are set to ease to 0 accross their duration, the proper velocity from
/// the interpolated force can be accessed by calling their `update()` method, which will update 
/// the easing value for the IntForce as well
#[derive(Clone, Copy, Debug)]
pub struct InterpolatedForce {
  current_velocity: Vec2,
  starting_velocity: Vec2,
  ending_velocity: Vec2,
  duration: u8,
  frames_elapsed: u8
}

impl InterpolatedForce {

  pub fn new(starting_velocity: Vec2, ending_velocity: Vec2, duration: u8) -> Self {
    return InterpolatedForce {
      current_velocity: starting_velocity,
      starting_velocity,
      ending_velocity,
      duration,
      frames_elapsed: 0
    }
  }

  pub fn update(&mut self) -> Vec2 {
    self.tick();
    let t = self.frames_elapsed as f32 / self.duration as f32;
    self.current_velocity = self.current_velocity.custom_lerp(self.ending_velocity,t);
    return self.current_velocity;
  }

  pub fn tick(&mut self) {
    self.frames_elapsed += 1;
  }

  pub fn is_finished(&self) -> bool {
    return self.duration == self.frames_elapsed;
  }
}

/// States representing all possible player actions
#[derive(Clone,Copy, Debug, PartialEq)]
pub enum ActionState {
  DASHING,
  WALKING,
  BACKWALKING,
  ATTACKING,
  BLOCKING,
  CROUCHBLOCKING,
  CROUCHING,
  JUMPSQUAT,
  AIRBORNE,
  JUGGLE,
  STANDING,
  BACKDASHING,
  AIR_DASHING,
  AIR_BACKDASHING 
}

impl Default for ActionState {
  fn default() -> Self {Self::STANDING}
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


