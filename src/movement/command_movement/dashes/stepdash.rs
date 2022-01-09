pub use crate::prelude::*;

pub struct StepDash {
    pub speed: f32,
    pub recovery_frames: u8,
  }
  
  impl StepDash {
    pub fn new(speed: f32, recovery_frames: u8) -> Self {
      StepDash {speed,recovery_frames}
    }
  }
