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
  
  // impl Dash for StepDash {
  //   fn exec(&self, player_movement: PlayerMovement) -> {
  //     let facing_multiplier;
  //     if player_state.facing_right {
  //       facing_multiplier = 1.0;
  //     } else {
  //       facing_multiplier = -1.0;
  //     }
  //     (
  //       ActionState {
  //         busy_duration: self.recovery_frames,
  //         action_state: ActionState::DASHING,
  //         ..Default::default()
  //       },
  //       PhysicsState {
  //         velocity: Vec2::new(self.speed * facing_multiplier, physics_state.velocity.y),
  //         ..Default::default()
  //       }
  //     )
  //   }

  //   fn sustainable(&self) -> bool {
  //       return false;
  //   }
  // }
