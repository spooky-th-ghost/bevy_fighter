pub use crate::prelude::*;

pub trait Dash {
  fn exec(&self, player_state: ActionState, physics_state: PhysicsState) -> (ActionState, PhysicsState);
}

pub struct StepDash {
  pub speed: f32,
  pub recovery_frames: u8,
}

impl StepDash {
  pub fn new(speed: f32, recovery_frames: u8) -> Self {
    StepDash {speed,recovery_frames}
  }
}

impl Dash for StepDash {
  fn exec(&self, player_state: ActionState, physics_state: PhysicsState) -> (ActionState, PhysicsState) {
    let facing_multiplier;
    if player_state.facing_right {
      facing_multiplier = 1.0;
    } else {
      facing_multiplier = -1.0;
    }
    (
      ActionState {
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

pub struct Backdash {
  pub speed: f32,
  pub recovery_frames: u8,
  pub invunl: u8
}


pub struct PlayerMovement {
  grounded_dash: Option<Box<dyn Dash>>,
}

impl PlayerMovement {
  pub fn new(grounded_dash: impl Dash + 'static) -> Self {
    PlayerMovement{
      grounded_dash: Some(Box::new(grounded_dash))
    }
  }
}

// fn do_stuff() {
//   let a = PlayerMovement::new(
//     StepDash::new(5.0, 16)
//   );

//   if let Some(dash) = a.grounded_dash {
//     dash.exec()
//   }
// }

