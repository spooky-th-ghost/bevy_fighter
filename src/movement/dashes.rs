pub use crate::prelude::*;

pub trait Dash {
  fn exec(&self, player_state: PlayerState, physics_state: PhysicsState) -> (PlayerState, PhysicsState);
}

pub struct StepDash {
  speed: f32,
  recovery_frames: u8,
}

impl Dash for StepDash {
  fn exec(&self, player_state: PlayerState, physics_state: PhysicsState) -> (PlayerState, PhysicsState) {
    let facing_multiplier;
    if player_state.facing_right {
      facing_multiplier = 1.0;
    } else {
      facing_multiplier = -1.0;
    }
    (
      PlayerState {
        busy_duration: self.recovery_frames,
        invuln: 0,
        armor_duration: 0,
        facing_right: player_state.facing_right,
        player_state_name: PlayerStateName::DASHING,
        armor_type: None,
        cancellable_actions: None,
      },
      PhysicsState {
        velocity: Vec2::new(self.speed * facing_multiplier, physics_state.velocity.y),
        gravity: 0.0,
        collidable: true,
      }
    )
  }
}
