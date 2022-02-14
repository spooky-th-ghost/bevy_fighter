use crate::prelude::*;

#[derive(Debug, Clone, Component)]
pub enum CharacterState {
  Idle,
  Walking,
  BackWalking,
  Attacking {duration: u8, attack: Attack},
  Crouching,
  Jumpsquat {duration: u8, velocity: Vec2 },
  AirJumpsquat {duration: u8, velocity: Vec2 },
  Rising,
  Falling,
  Juggle,
  Standing,
  BackDashing {duration: u8},
  AirDashing {duration: u8, velocity: Vec2},
  AirBackDashing {duration: u8, velocity: Vec2} 
}

impl CharacterState {
  pub fn determine_state(&mut self, buffer: &mut FighterInputBuffer) {

  }

  pub fn is_airborne(&self) -> bool {
    use CharacterState::*;
    match self {
      AirJumpsquat {duration:_, velocity:_}
      | Rising
      | Falling
      | AirDashing {duration:_, velocity:_}
      | AirBackDashing {duration:_, velocity:_} => return true,
      _ => return false
    }
  }
}
