pub use crate::prelude::*;
pub struct BasicBackdash {
    speed: f32,
    busy: u8,
    motion_duration: u8
  }
  
  impl BasicBackdash{
    pub fn new(speed: f32, busy: u8, motion_duration: u8) -> Self {
      BasicBackdash {
        speed,
        busy,
        motion_duration
      }
    }
  }
  
  impl Backdash for BasicBackdash {
    fn exec(&self, facing_vector: f32) -> (InterpolatedForce, u8) {
      return (
        InterpolatedForce::new(
          Vec2::new(-self.speed * facing_vector, 0.0),
          Vec2::new(-2.0 * facing_vector, 0.0),
          self.motion_duration
        ),
        self.busy
      );
    }
  }
