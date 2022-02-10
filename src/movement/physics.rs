use lerp::Lerp;
pub use crate::prelude::*;

pub trait CustomLerp {
  fn custom_lerp(&self, target: Self, t: f32) -> Self;
}


impl CustomLerp for Vec2 {
  fn custom_lerp(&self, target: Vec2, t: f32) -> Vec2 {
    if &self.distance(target) > &0.02 {
      let _x = self.x.lerp(target.x,t);
      let _y = self.y.lerp(target.y,t);
      return Vec2::new(_x,_y);
    } else {
      return target;
    }
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
#[derive(Clone, Debug)]
pub enum ActionState {
  Dashing,
  Walking,
  BackWalking,
  Attacking {duration: u8, attack: Attack},
  AttackingAirborne,
  Blocking,
  CrouchBlocking,
  Crouching,
  Jumpsquat {squat: u8, velocity: Vec2 },
  AirJumpsquat {squat: u8, velocity: Vec2 },
  Airborne,
  Juggle,
  Standing,
  BackDashing,
  AirDashing {duration: u8, velocity: Vec2},
  AirBackDashing {duration: u8, velocity: Vec2} 
}

impl PartialEq for ActionState {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (ActionState::Dashing, ActionState::Dashing) => true,
      (ActionState::Walking, ActionState::Walking) => true,
      (ActionState::BackWalking, ActionState::BackWalking) => true,
      (ActionState::Attacking {duration: _, attack: _}, ActionState::Attacking {duration: _, attack: _}) => true,
      (ActionState::Blocking, ActionState::Blocking) => true,
      (ActionState::CrouchBlocking, ActionState::CrouchBlocking) => true,
      (ActionState::Crouching, ActionState::Crouching) => true,
      (ActionState::Airborne, ActionState::Airborne) => true,
      (ActionState::Juggle, ActionState::Juggle) => true,
      (ActionState::Standing, ActionState::Standing) => true,
      (ActionState::BackDashing, ActionState::BackDashing) => true,
      (ActionState::Jumpsquat {squat:_, velocity:_},ActionState::Jumpsquat {squat: _, velocity:_ }) => true,
      (ActionState::AirJumpsquat {squat:_, velocity:_}, ActionState::AirJumpsquat {squat:_, velocity:_}) => true,
      (ActionState::AirDashing {duration:_, velocity:_},ActionState::AirDashing {duration:_, velocity:_},) => true,
      (ActionState::AirBackDashing {duration:_, velocity:_}, ActionState::AirBackDashing {duration:_, velocity:_}) => true,
      _ => false,
    }
  }
}

impl ActionState {
  pub fn tick(&mut self) {
    match self {
      ActionState::Jumpsquat { squat, velocity: _} => {
        *squat = countdown(*squat);
      },
      ActionState::AirJumpsquat { squat, velocity: _} => {
        *squat = countdown(*squat);
      },
      ActionState::AirDashing {duration, velocity: _} => {
        *duration = countdown(*duration);
      },
      ActionState::AirBackDashing {duration, velocity: _} => {
        *duration = countdown(*duration);
      },
      _ => ()
    }
  }

  pub fn is_finished_jumping(&self) -> bool {
    match self {
      ActionState::Jumpsquat{squat, velocity: _} => {
        if *squat == 0 {
          return true;
        } else {
          return false;
        }
      },
      ActionState::AirJumpsquat{squat, velocity: _} => {
        if *squat == 0 {
          return true;
        } else {
          return false;
        }
      }
      _ => return false,
    }
  }

  pub fn is_finished_airdashing(&self) -> bool {
    match self {
      ActionState::AirDashing {duration, velocity: _} => {
        if *duration == 0 {
          return true;
        } else {
          return false;
        }
      },
      ActionState::AirBackDashing {duration, velocity: _} => {
        if *duration == 0 {
          return true;
        } else {
          return false;
        }
      },
      _ => return false,
    }
  }

  pub fn get_jump_velocity(&self) -> Vec2{
    match self {
      ActionState::AirJumpsquat {squat: _, velocity} => *velocity,
      ActionState::Jumpsquat {squat: _, velocity} => *velocity,
      _ => Vec2::ZERO,
    }
  }
}

impl Default for ActionState {
  fn default() -> Self {Self::Standing}
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


