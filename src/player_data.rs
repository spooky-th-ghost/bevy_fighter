pub use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
  P1,
  P2
}

#[derive(Component, Clone, Copy, Debug)]
pub struct CharacterStatus {
  pub action_state: ActionState,
  pub previous_action_state: ActionState,
  pub facing_vector: f32,
  pub movement_event: Option<MovementEvent>,
  pub busy: u8,
  pub jumpsquat: u8,
  pub is_grounded: bool,
  pub air_jumps: u8,
  pub air_jumps_remaining: u8,
  pub airdashes: u8,
  pub airdashes_remaining: u8,
  pub airdash_lockout: u8,
  pub air_dash_speed: f32,
  pub air_back_dash_speed: f32,
  pub velocity: Vec2,
  pub walk_speed: f32,
  pub back_walk_speed: f32,
  pub dash_speed: f32,
  pub gravity: f32,
  pub jump_height: f32,
  pub max_airdash_time: u8,
  pub max_air_backdash_time: u8,
  pub int_force: Option<InterpolatedForce>,
  pub backdash: Backdash,
}

impl CharacterStatus {
  // Setters

  /// Set a players ActionState
  pub fn set_action_state(&mut self, action_state: ActionState) {
    self.action_state = action_state
  }

  /// Set the players facing direction
  pub fn set_facing_vector(&mut self, facing_vector: f32) {
    self.facing_vector = facing_vector;
  }

  /// Set a players busy value, which translates to how many frames it will be until the players inputs will be read again
  pub fn set_busy(&mut self, busy: u8) {
    self.busy = busy;
  }

  pub fn clear_movement_event(&mut self) {
    self.movement_event = None;
  }

  pub fn land(&mut self) {
    self.is_grounded = true;
    self.air_jumps_remaining = self.air_jumps;
    self.action_state = ActionState::STANDING;
    self.airdashes_remaining = self.airdashes;
    self.airdash_lockout = 0;
  }

  // Getters

  /// Get a players ActionState
  pub fn get_action_state(&self) -> ActionState {
    return self.action_state;
  }

  /// Get if a player is grounded
  pub fn get_is_grounded(&self) -> bool {
    return self.is_grounded;
  }

  pub fn get_is_busy(&self) -> bool {
    return self.busy != 0;
  }

  pub fn get_can_airdash(&self) -> bool {
    return self.airdashes_remaining > 0 && self.airdash_lockout == 0;
  }

  pub fn get_should_transition(&self) -> bool {
    return self.action_state != self.previous_action_state;
  }

  pub fn can_turn(&self) -> bool {
    match self.action_state {
      ActionState::STANDING
      | ActionState::AIRBORNE
      | ActionState::WALKING
      | ActionState::BACKWALKING
      | ActionState::CROUCHING => return true,
      _ => return false,
    }
  }

  pub fn calculate_transition(&self) -> Option<AnimationTransition> {

    match self.action_state {
      ActionState::JUMPSQUAT {squat: _, velocity: _} => return Some(AnimationTransition::ToRise),
      ActionState::WALKING => return Some(AnimationTransition::ToWalk),
      ActionState::BACKWALKING => return Some(AnimationTransition::ToBackwalk),
      ActionState::CROUCHING => return Some(AnimationTransition::ToCrouch),
      ActionState::DASHING => return Some(AnimationTransition::ToDash),
      ActionState::BACKDASHING => return Some(AnimationTransition::ToBackdash),
      ActionState::AIR_DASHING {duration: _, velocity: _} => return Some(AnimationTransition::ToAirdash),
      ActionState::AIR_BACKDASHING {duration: _, velocity: _} => return Some(AnimationTransition::ToAirBackdash),
      ActionState::AIRBORNE => {
        match self.previous_action_state { 
          ActionState::AIR_DASHING {duration:_, velocity: _} => return Some(AnimationTransition::AirdashToFall),
          ActionState::AIR_BACKDASHING {duration:_, velocity: _} => return Some(AnimationTransition::AirbackdashToFall),
          _ => return None,
        }
      }, // need to do the Rise_Fall_Split
      ActionState::STANDING => {
        match self.previous_action_state {
          ActionState::DASHING => return Some(AnimationTransition::DashToIdle),
          ActionState::BACKDASHING =>  return Some(AnimationTransition::BackDashToIdle),
          ActionState::WALKING => return Some(AnimationTransition::WalkToIdle),
          ActionState::BACKWALKING => return Some(AnimationTransition::BackwalkToIdle),
          ActionState::CROUCHING => return Some(AnimationTransition::CrouchToIdle),
          ActionState::AIRBORNE => return Some(AnimationTransition::FallToIdle),
          _ => return None
        }
      },
      _ => return None
    }
  }

  // Logic

  /// Reduce component timers by 1 frame
  pub fn tick(&mut self) {
    self.busy = countdown(self.busy);
    self.airdash_lockout = countdown(self.airdash_lockout);
    self.previous_action_state = self.action_state;
    self.action_state.tick();
  }

  // Run each frame to determine the players action state and if any movement events should be executed
  pub fn update_action_state(&mut self, buffer: &mut FighterInputBuffer) {
    if !self.get_is_busy() {
      if self.get_is_grounded() {
        match buffer.current_motion {
          5 => self.action_state = ActionState::STANDING,
          6 => {
            if self.action_state != ActionState::DASHING {
              self.action_state = ActionState::WALKING;
            }
          },
          4 => self.action_state = ActionState::BACKWALKING,
          1 | 2 | 3 => self.action_state = ActionState::CROUCHING,
          7 | 8 => {
            self.buffer_jump(buffer.current_motion, false,false, false);
          },
          9 => {
            if self.action_state == ActionState::DASHING {
              self.buffer_jump(buffer.current_motion, false,true, false);
            } else {
              self.buffer_jump(buffer.current_motion, false,false, false);
            }
          }
          _ => ()
        }
        if let Some(ct) = buffer.command_type {
          match ct {
            CommandType::DASH => {
              self.action_state = ActionState::DASHING;
              buffer.consume_motion();
            },
            CommandType::BACK_DASH => {
              self.action_state = ActionState::BACKDASHING;
              self.movement_event = Some(MovementEvent::new(MovementEventType::BACKDASH, buffer.current_motion));
              buffer.consume_motion();
            },
            _ => ()
          }               
        }

      } else {
        if let Some(ct) = buffer.command_type {
          match ct {
            CommandType::DASH => {
              self.buffer_airdash(true);
              buffer.consume_motion();
            },
            CommandType::BACK_DASH =>  {
              self.buffer_airdash(false);
              buffer.consume_motion();
            },
            _ => ()
          }
        }
      }
    }
  }

  /// Set the players velocity
  pub fn set_velocity(&mut self, velocity: Vec2) {
    self.velocity = velocity;
  }

  /// Set the players current interpolated force
  pub fn set_i_force(&mut self, int_force: InterpolatedForce) {
    self.int_force = Some(int_force);
  }

  pub fn get_target_velo(&mut self) -> Vec2 {
    if let Some(i_force) = self.int_force.as_mut() {
      let i_force_velo = i_force.update();
      if i_force.is_finished() {self.int_force = None;}
      return i_force_velo;
    } else {
      return self.velocity;
    }
  }

  pub fn buffer_jump(&mut self, motion: u8, superjump: bool, dashing: bool, airborne: bool) {
    let forward_vector = if dashing {
      2.0
    } else {
      1.0
    };

    let x_velocity = match motion {
      7 => self.facing_vector * (-self.back_walk_speed*2.0),
      9 => self.facing_vector * (self.walk_speed * forward_vector),
      _ => 0.0
    };

    let squat = if airborne {
      1
    } else {
      3
    };

    let y_velocity = if airborne {
      if superjump {
        self.jump_height
      } else {
        self.jump_height * 0.75 
      }
    } else {
      if superjump {
        self.jump_height * 1.25
      } else {
        self.jump_height
      }
    };

    let velocity = Vec2::new(x_velocity, y_velocity);
    self.busy = {
      if airborne {
        3
      } else {
        12
      }
    };
    self.action_state = if airborne {
      ActionState::AIR_JUMPSQUAT { squat, velocity}
    } else {
      ActionState::JUMPSQUAT {squat, velocity}
    };
  }

  pub fn buffer_airdash(&mut self, forward: bool) {
    if self.get_can_airdash() {
      self.airdashes_remaining = countdown(self.air_jumps_remaining);
      self.busy = 5;
      self.airdash_lockout = 15;

      if forward {
        self.action_state = ActionState::AIR_DASHING {duration: self.max_airdash_time, velocity: Vec2::X * self.air_dash_speed * self.facing_vector};
      } else {
        self.action_state = ActionState::AIR_BACKDASHING {duration: self.max_air_backdash_time, velocity: Vec2::X * self.air_dash_speed * -self.facing_vector };
      }
    }
  }

  pub fn execute_jump(&mut self) {
    if self.action_state.is_finished_jumping() {
      self.is_grounded = false;
      self.velocity = self.action_state.get_jump_velocity();
      self.action_state = ActionState::AIRBORNE;
    }
  }

  pub fn execute_airdash(&mut self) {
    if self.action_state.is_finished_airdashing() {
      self.action_state = ActionState::AIRBORNE;
    }
  }

  pub fn execute_backdash(&mut self) {
    match self.backdash {
      Backdash::STANDARD {speed, busy, motion_duration} => {
        let int_force = InterpolatedForce::new(
          Vec2::new(-speed * self.facing_vector, 0.0),
          Vec2::new(-2.0 * self.facing_vector, 0.0),
          motion_duration
        );
        self.set_i_force(int_force);
        self.set_busy(busy);
      },
      _ => ()
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Backdash {
  STANDARD {speed: f32, busy: u8, motion_duration: u8},
  TELEPORT { distance: f32, busy: u8, motion_duration: u8},
  LEAP {busy: u8, motion_duration: u8}
}

#[derive(Debug, Clone, Copy)]
pub struct CharacterMovementEvent{
  /// PlayerId for the movement event
  pub player_id: PlayerId,
  /// Type of movement event
  pub event_type: MovementEventType,
  /// Motion for the movement event
  pub motion: u8,
}

impl CharacterMovementEvent{
  pub fn new(
    player_id: PlayerId, 
    event_type: MovementEventType,
    motion: u8
  ) -> Self {
    CharacterMovementEvent {
      player_id,
      event_type,
      motion
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub struct MovementEvent {
  /// Type of movement event
  pub event_type: MovementEventType,
  /// Motion for the movement event
  pub motion: u8,
}

impl MovementEvent {
  pub fn new( 
    event_type: MovementEventType,
    motion: u8
  ) -> Self {
    MovementEvent {
      event_type,
      motion
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum MovementEventType {
  JUMP,
  SUPERJUMP,
  DASHJUMP,
  DASH,
  BACKDASH,
  AIRDASH,
  AIRBACKDASH,
}



impl Default for CharacterStatus {
  fn default() -> Self {
    CharacterStatus {
      action_state: ActionState::default(),
      previous_action_state: ActionState::default(),
      facing_vector: 1.0,
      movement_event: None,
      busy: 0,
      jumpsquat: 3,
      is_grounded: true,
      air_jumps: 1,
      air_jumps_remaining: 1,
      airdashes: 1,
      airdashes_remaining: 1,
      airdash_lockout: 0,
      air_dash_speed: 8.0,
      air_back_dash_speed: 6.0,
      velocity: Vec2::ZERO,
      walk_speed: 4.0,
      back_walk_speed: 2.5,
      dash_speed: 8.0,
      gravity: 1.0,
      jump_height: 20.0,
      max_airdash_time: 25,
      max_air_backdash_time: 15,
      int_force: None,
      backdash: Backdash::STANDARD{
        speed: 25.0,
        busy: 20,
        motion_duration: 20
      }
    }
  }
}
pub trait SpawnPlayer {
  fn spawn_player(
    &mut self, 
    player_id: PlayerId, 
    character_prefix: &str, 
    library: &AnimationLibrary,
    texture_atlas: Handle<TextureAtlas>
  );
}

impl SpawnPlayer for Commands<'_, '_> {
  fn spawn_player(
    &mut self, 
    player_id: PlayerId, 
    character_prefix: &str, 
    library: &AnimationLibrary,
    texture_atlas: Handle<TextureAtlas>) {
    let (transform, facing_vector, flip_x) = match player_id {
      PlayerId::P1 => (
        Transform::from_xyz(-40.0,0.0,0.0),
        1.0,
        false
      ),
      PlayerId::P2 => (
        Transform::from_xyz(40.0,0.0,0.0),
        -1.0,
        true
      )
    };

    self.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
          flip_x,
          ..Default::default()
        },
        texture_atlas,
        transform,
        ..Default::default()
      })
      .insert(player_id)
      .insert(CharacterStatus {
        facing_vector,
        ..Default::default()
      })
      .insert(AnimationController::new(character_prefix, library));
  }
}
