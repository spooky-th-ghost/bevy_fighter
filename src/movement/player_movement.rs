pub use crate::prelude::*;
#[derive(Component)]
pub struct PlayerMovement {
    pub player_id: u8,
    pub busy: u8,
    pub invuln: u8,
    pub armor: u8,
    pub facing_vector: f32,
    pub walk_speed: f32,
    pub back_walk_speed: f32,
    pub dash_speed: f32,
    pub velocity: Vec2,
    pub gravity: f32,
    pub jump_height: f32,
    pub jumpsquat: u8,
    pub is_grounded: bool,
    pub action_state: ActionState,
    pub int_force: Option<InterpolatedForce>,
    pub backdash: Box<dyn Backdash>,
    pub jumpdata: Option<JumpData>,
  }

  impl Default for PlayerMovement {
    fn default() -> Self {
      PlayerMovement{
        player_id: 0,
        busy: 0,
        invuln: 0,
        armor: 0,
        facing_vector: 1.0,
        walk_speed: 4.0,
        back_walk_speed: 2.5,
        dash_speed: 8.0,
        velocity: Vec2::ZERO,
        gravity: 1.0,
        jump_height: 15.0,
        jumpsquat: 3,
        is_grounded: true,
        action_state: ActionState::default(),
        int_force: None,
        backdash: Box::new(BasicBackdash::new(25.0,20,20)),
        jumpdata: None
      }
    }
  }
  
  impl PlayerMovement {
    // Setters
    pub fn set_action_state(&mut self, action_state: ActionState) {
      self.action_state = action_state
    }

    pub fn set_busy(&mut self, busy: u8) {
      self.busy = busy;
    }

    pub fn set_i_force(&mut self, i_force: InterpolatedForce) {
      self.int_force = Some(i_force);
    }

    pub fn set_facing_vector(&mut self, vector: f32) {
      self.facing_vector = vector;
    }

    // Getters
    pub fn get_busy(&self) -> bool {
      return self.busy != 0;
    }

    pub fn get_grounded(&self) -> bool {
      return self.is_grounded;
    }

    pub fn buffer_jump(&mut self, motion: u8, superjump: bool) {
      let x_velocity = match motion {
        7 => self.facing_vector * (-self.back_walk_speed*2.0),
        9 => self.facing_vector * self.walk_speed,
        _ => 0.0
      };
      self.jumpdata = Some(JumpData::new(x_velocity, self.jumpsquat, superjump));
      self.busy = self.jumpsquat + 10;
    }

    pub fn execute_jump(&mut self) {
      if let Some(jd) = self.jumpdata.as_mut() {
        if jd.squat > 0 {
          jd.tick();
        } else {
          self.velocity = Vec2::new(jd.x_velocity, self.jump_height);
          self.is_grounded = false;
          self.jumpdata = None;
          self.action_state = ActionState::AIRBORNE;
        }
      }
    }

    // Logic
    pub fn action_state_maintenence(&mut self) {
      self.busy = countdown(self.busy);
      self.invuln = countdown( self.invuln);
      self.armor = countdown(self.armor);
      self.execute_jump();
    }

    pub fn execute_backdash(&mut self) {
      let (bd_i_force, bd_busy) = self.backdash.exec(self.facing_vector);
      self.set_i_force(bd_i_force);
      self.set_busy(bd_busy);
    }

    pub fn target_velo(&mut self) -> Vec2 {
      if let Some(i_force) = self.int_force.as_mut() {
        let i_force_velo = i_force.update();
        if i_force.is_finished() {self.int_force = None;}
        return i_force_velo;
      } else {
        return self.velocity;
      }
    }



    pub fn action_state(&self) -> ActionState {
      return self.action_state;
    }

    pub fn manage_player_action_state(&mut self, buffer: &FighterInputBuffer) {
      if !self.get_busy() {
        let mut  new_state = ActionState::default();
        if self.is_grounded {
          match self.action_state {
            ActionState::WALKING | ActionState::BACKWALKING | ActionState::CROUCHING | ActionState::STANDING => {
              match buffer.current_motion {
                5 => new_state = ActionState::STANDING,
                6 => new_state = ActionState::WALKING,
                4 => new_state = ActionState::BACKWALKING,
                1 | 2 | 3 => new_state = ActionState::CROUCHING,
                7 | 8 | 9 => new_state = {
                  self.buffer_jump(buffer.current_motion,false);
                  ActionState::JUMPSQUAT
                },
                _ => ()
              }
              if let Some(ct) = buffer.command_type {
                match ct {
                  CommandType::DASH => {
                    new_state = ActionState::DASHING;
                  },
                  CommandType::BACK_DASH => {
                    new_state = ActionState::BACKDASHING;
                    self.execute_backdash()
                  }
                  _ => ()
                }               
              }
            },
            ActionState::DASHING => {
              match buffer.current_motion {
                5 => new_state = ActionState::STANDING,
                6 => new_state = ActionState::DASHING,
                4 => new_state = ActionState::BACKWALKING,
                1 | 2 | 3 => new_state = ActionState::CROUCHING,
                7 | 8 | 9 => new_state = {
                  self.buffer_jump(buffer.current_motion,false);
                  ActionState::JUMPSQUAT
                },
                _ => ()
              }
            }
            _ => ()
          }
        } else {
          new_state = ActionState::AIRBORNE;
        }
        self.action_state = new_state;
      }
      self.update_velocity_from_state()
    }

    pub fn update_velocity_from_state (&mut self) {
      let mut new_velocity = Vec2::ZERO;

      match self.action_state {
        ActionState::WALKING => new_velocity = Vec2::new(self.walk_speed * self.facing_vector, 0.0),
        ActionState::BACKWALKING => new_velocity = Vec2::new(-self.back_walk_speed * self.facing_vector, 0.0),
        ActionState::DASHING => new_velocity = Vec2::new(self.dash_speed * self.facing_vector,0.0),
        ActionState::AIRBORNE => new_velocity = self.velocity - (Vec2::Y * self.gravity),
        _ => new_velocity = self.velocity.custom_lerp(Vec2::ZERO, 0.2),
      }
      self.velocity = new_velocity;
    }
  }

  pub fn update_player_states (player_data: ResMut<PlayerData>, mut query: Query<&mut PlayerMovement>) {
    for mut player_movement in query.iter_mut() {
      for buffer in player_data.buffers.iter() {
        if buffer.player_id == player_movement.player_id {
          let facing_vector = player_data.get_facing_vector(&player_movement.player_id);
          player_movement.set_facing_vector(facing_vector);
          player_movement.manage_player_action_state(buffer);
        }
      }
      player_movement.action_state_maintenence();
    }
  }

  pub fn apply_player_velocity(mut player_data: ResMut<PlayerData>,mut query: Query<(&mut Transform, &mut PlayerMovement)>) {
    for (mut transform, mut movement) in query.iter_mut() {
      let tv = movement.target_velo();
      transform.translation += Vec3::new(tv.x, tv.y, 0.0);
      if transform.translation.y < 0.0 {
        transform.translation.y = 0.0;
        movement.is_grounded = true;
      }
      player_data.set_position(&movement.player_id, transform.translation);
    }
  }

#[derive(Clone, Copy)]
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
      squat
    }
  }

  pub fn tick(&mut self) {
    self.squat = countdown(self.squat);
  }
}
  
