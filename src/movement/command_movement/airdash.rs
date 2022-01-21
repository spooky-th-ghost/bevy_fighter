pub use crate::prelude::*;
pub struct BasicAirdash {
    speed: f32,
    busy: u8,
    motion_duration: u8
  }

  impl BasicAirdash {
    pub fn new(speed: f32, busy: u8, motion_duration: u8) -> Self {
      BasicAirdash {
        speed,
        busy,
        motion_duration
      }
    }
  }

  // impl Airdash for BasicAirdash {
  //   fn exec_forward(&self, facing_vector: f32) -> (InterpolatedForce, u8) {

  //   }

  //   fn exec_backwards(&self, facing_vector: f32) -> (InterpolatedForce, u8) {

  //   }
  // }

  
