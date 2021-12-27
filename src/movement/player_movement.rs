pub use crate::prelude::*;
pub struct PlayerMovement {
    physics_state: PhysicsState,
    action_state: ActionState,
    walk_speed: f32,
    back_walk_speed: f32,
    dash_speed: f32,
    backdash_speed: f32,
  }
  
  impl PlayerMovement {
    pub fn new() -> Self {
      PlayerMovement{
        walk_speed: 4.0,
        back_walk_speed: 2.5,
        dash_speed: 8.0,
        backdash_speed: 15.0,
        physics_state: PhysicsState::default(),
        action_state: ActionState::default(),
      }
    }

    pub fn action_state_maintenence(&mut self) {
      let new_busy = countdown(self.action_state.busy_duration);
      let new_invuln = countdown(self.action_state.invuln);
      let new_armor = countdown(self.action_state.armor_duration);

      self.action_state = ActionState {
        busy_duration: new_busy,
        invuln: new_invuln,
        armor_duration: new_armor,
        ..self.action_state
      }
    }

    pub fn is_busy(&self) -> bool {
      return self.action_state.busy_duration != 0;
    }

    pub fn is_grounded(&self) -> bool {
      return self.physics_state.is_grounded;
    }

    pub fn target_velo(&self) -> Vec2 {
      if let Some(mut i_force) = self.physics_state.int_force {
        return i_force.update();
      } else {
        return self.physics_state.velocity;
      }
    }

    pub fn set_action_state(&mut self, state_enum: PlayerStateName) {
      self.action_state = ActionState {
        player_state_name: state_enum,
        ..Default::default()
      }
    }

    pub fn set_action_state_busy(&mut self, state_enum: PlayerStateName, busy: u8) {
      self.action_state = ActionState {
        player_state_name: state_enum,
        busy_duration: busy,
        ..Default::default()
      }
    }

    pub fn state_enum(&self) -> PlayerStateName {
      return self.action_state.player_state_name;
    }

    pub fn update_states_from_buffer(&mut self, buffer: &InputBuffer) {
      self.action_state_maintenence();
      let mut new_velocity = Vec2::ZERO;
      if self.is_grounded() {
        match buffer.current_motion {
          6 => new_velocity = Vec2::new(self.walk_speed, 0.0),
          4 => new_velocity = Vec2::new(-self.back_walk_speed, 0.0),
          _ => ()
        }

        match self.state_enum() {
          PlayerStateName::DASHING => {
            match buffer.current_motion {
              6 | 3 => new_velocity = Vec2::new(self.dash_speed, 0.0),
              4 => {
                new_velocity = Vec2::new(-self.back_walk_speed, 0.0);
                self.set_action_state(PlayerStateName::WALKING);
              }
              _ => self.set_action_state(PlayerStateName::STANDING)
            }
          }
          _ => ()
        }

        if let Some(ct) = buffer.command_type {
          match ct {
            CommandType::DASH => {
              new_velocity = Vec2::new(self.dash_speed, 0.0);
              self.set_action_state(PlayerStateName::DASHING);
            },
            CommandType::BACK_DASH => {
              new_velocity = Vec2::new(-self.backdash_speed, 0.0);
              self.set_action_state_busy(PlayerStateName::BACKDASHING,30);
            }
            _ => ()
          }
        }
      } else {

      }
      self.physics_state = PhysicsState {
        velocity: new_velocity,
        ..Default::default()
      }
    }
  }

  pub fn update_player_states (mut query: Query<(&InputBuffer, &mut PlayerMovement)>) {
    for (buffer, mut player_movement) in query.iter_mut() {
      if !player_movement.is_busy() {
        player_movement.update_states_from_buffer(buffer);
      }
    }
  }

  pub fn apply_player_velocity(mut query: Query<(&mut Transform, &PlayerMovement)>) {
    for (mut transform, movement) in query.iter_mut() {
      match movement.state_enum() {
        PlayerStateName::BACKDASHING => {

        }
        _ => {
          let tv = movement.target_velo();
          transform.translation += Vec3::new(tv.x, tv.y, 0.0)
        }
      }
    }
  }
