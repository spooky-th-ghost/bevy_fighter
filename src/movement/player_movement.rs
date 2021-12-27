pub use crate::prelude::*;
pub struct PlayerMovement {
    busy: u8,
    invuln: u8,
    armor: u8,
    facing_right: bool,
    walk_speed: f32,
    back_walk_speed: f32,
    dash_speed: f32,
    backdash_speed: f32,
    velocity: Vec2,
    gravity: f32,
    is_grounded: bool,
    action_state: ActionState,
    int_force: Option<InterpolatedForce>
  }
  
  impl PlayerMovement {
    pub fn new() -> Self {
      PlayerMovement{
        busy: 0,
        invuln: 0,
        armor: 0,
        facing_right: true,
        walk_speed: 4.0,
        back_walk_speed: 2.5,
        dash_speed: 8.0,
        backdash_speed: 15.0,
        velocity: Vec2::ZERO,
        gravity: 10.0,
        is_grounded: true,
        action_state: ActionState::default(),
        int_force: None
      }
    }

    pub fn action_state_maintenence(&mut self) {
      self.busy = countdown(self.busy);
      self.invuln = countdown( self.invuln);
      self.armor = countdown(self.armor);
    }

    pub fn is_busy(&self) -> bool {
      return self.busy != 0;
    }

    pub fn is_grounded(&self) -> bool {
      return self.is_grounded;
    }

    pub fn target_velo(&self) -> Vec2 {
      if let Some(mut i_force) = self.int_force {
        return i_force.update();
      } else {
        return self.velocity;
      }
    }

    pub fn set_action_state(&mut self, action_state: ActionState) {
      self.action_state = action_state
    }

    pub fn set_busy(&mut self, busy: u8) {
      self.busy = busy;
    }

    pub fn action_state(&self) -> ActionState {
      return self.action_state;
    }

    pub fn update_states_from_buffer(&mut self, buffer: &InputBuffer) {
      self.action_state_maintenence();
      let mut new_velocity = Vec2::ZERO;
      if self.is_grounded {
        match buffer.current_motion {
          6 => new_velocity = Vec2::new(self.walk_speed, 0.0),
          4 => new_velocity = Vec2::new(-self.back_walk_speed, 0.0),
          _ => ()
        }

        match self.action_state {
          ActionState::DASHING => {
            match buffer.current_motion {
              6 | 3 => new_velocity = Vec2::new(self.dash_speed, 0.0),
              4 => {
                new_velocity = Vec2::new(-self.back_walk_speed, 0.0);
                self.action_state = ActionState::WALKING;
              }
              _ => self.action_state = ActionState::STANDING,
            }
          }
          _ => ()
        }

        if let Some(ct) = buffer.command_type {
          match ct {
            CommandType::DASH => {
              new_velocity = Vec2::new(self.dash_speed, 0.0);
              self.action_state = ActionState::DASHING;
            },
            CommandType::BACK_DASH => {
              new_velocity = Vec2::new(-self.backdash_speed, 0.0);
              self.action_state = ActionState::BACKDASHING;
              self.busy = 30;
            }
            _ => ()
          }
        }
      } else {

      }
      self.velocity = new_velocity;
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
      match movement.action_state {
        ActionState::BACKDASHING => {

        }
        _ => {
          let tv = movement.target_velo();
          transform.translation += Vec3::new(tv.x, tv.y, 0.0)
        }
      }
    }
  }
