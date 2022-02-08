pub use crate::prelude::*;

#[derive(Clone, Copy, Debug)]
  pub struct JumpData {
  pub superjump: bool,
  pub x_velocity: f32,
  pub squat: u8,
}

impl JumpData {
  pub fn new(x_velocity: f32, squat: u8, superjump: bool) -> Self {
    JumpData {
      superjump,
      x_velocity,
      squat,
    }
  }

  pub fn tick(&mut self) {
    self.squat = countdown(self.squat);
  }
}


  
