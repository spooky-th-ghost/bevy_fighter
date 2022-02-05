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
  pub movement_event: Option<MovementEvent>,
  pub busy: u8,
  pub jumpsquat: u8,
  pub is_grounded: bool,
  pub air_jumps: u8,
  pub air_jumps_remaining: u8,
  pub airdashes: u8,
  pub airdashes_remaining: u8,
  pub airdash_lockout: u8,
}

impl CharacterStatus {
  // Setters

  /// Set a players ActionState
  pub fn set_action_state(&mut self, action_state: ActionState) {
    self.action_state = action_state
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
      | ActionState::CROUCHING
      | ActionState::DASHING
      | ActionState::JUMPSQUAT => return true,
      _ => return false,
    }
  }

  pub fn calculate_transition(&self) -> Option<AnimationStateTransition> {

    match self.action_state {
      ActionState::JUMPSQUAT => return Some(AnimationStateTransition::ToRise),
      ActionState::WALKING => return Some(AnimationStateTransition::ToWalk),
      ActionState::BACKWALKING => return Some(AnimationStateTransition::ToBackwalk),
      ActionState::CROUCHING => return Some(AnimationStateTransition::ToCrouch),
      ActionState::DASHING => return Some(AnimationStateTransition::ToDash),
      ActionState::BACKDASHING => return Some(AnimationStateTransition::ToBackdash),
      ActionState::AIR_DASHING => return Some(AnimationStateTransition::ToAirdash),
      ActionState::AIR_BACKDASHING => return Some(AnimationStateTransition::ToAirBackdash),
      ActionState::AIRBORNE => {
        match self.previous_action_state { 
          ActionState::AIR_DASHING => return Some(AnimationStateTransition::AirdashToFall),
          ActionState::AIR_BACKDASHING => return Some(AnimationStateTransition::AirbackdashToFall),
          _ => return None,
        }
      }, // need to do the Rise_Fall_Split
      ActionState::STANDING => {
        match self.previous_action_state {
          ActionState::DASHING => return Some(AnimationStateTransition::DashToIdle),
          ActionState::BACKDASHING =>  return Some(AnimationStateTransition::BackDashToIdle),
          ActionState::WALKING => return Some(AnimationStateTransition::WalkToIdle),
          ActionState::BACKWALKING => return Some(AnimationStateTransition::BackwalkToIdle),
          ActionState::CROUCHING => return Some(AnimationStateTransition::CrouchToIdle),
          ActionState::AIRBORNE => return Some(AnimationStateTransition::FallToIdle),
          _ => return None
        }
      },
      _ => return None
    }
    // match self.previous_action_state {
    //   ActionState::DASHING => {
    //     match self.action_state {
    //       ActionState::JUMPSQUAT => return Some(AnimationStateTransition::ToRise),
    //       ActionState::STANDING =>  return Some(AnimationStateTransition::DashToIdle),
    //       ActionState::CROUCHING => return Some(AnimationStateTransition::ToCrouch),
    //       _ => return None
    //     }
    //   },
    //   ActionState::WALKING => {
    //     match self.action_state {
    //       ActionState::JUMPSQUAT => return Some(AnimationStateTransition::ToRise),
    //       ActionState::STANDING =>  return Some(AnimationStateTransition::WalkToIdle),
    //       ActionState::DASHING => return Some(AnimationStateTransition::WalkToDash),
    //       ActionState::CROUCHING => return Some(AnimationStateTransition::ToCrouch),
    //       _ => return None
    //     }
    //   },
    //   ActionState::BACKWALKING => {
    //     match self.action_state {
    //       ActionState::JUMPSQUAT => return Some(AnimationStateTransition::ToRise),
    //       ActionState::STANDING =>  return Some(AnimationStateTransition::BackwalkToIdle),
    //       ActionState::DASHING => return Some(AnimationStateTransition::BackwalkToBackdash),
    //       ActionState::CROUCHING => return Some(AnimationStateTransition::ToCrouch),
    //       _ => return None
    //     }
    //   },
    //   ActionState::CROUCHING => {
    //     match self.action_state {
    //       ActionState::STANDING =>  return Some(AnimationStateTransition::CrouchToIdle),
    //       ActionState::JUMPSQUAT => return Some(AnimationStateTransition::ToRise),
    //       ActionState::WALKING => return Some(AnimationStateTransition::ToWalk),
    //       ActionState::BACKWALKING => return Some(AnimationStateTransition::ToBackwalk),
    //       _ => return None
    //     }
    //   },
    //   ActionState::AIRBORNE => {
    //     match self.action_state { 
    //       ActionState::AIR_DASHING => return Some(AnimationStateTransition::FallToAirdash),
    //       ActionState::AIR_BACKDASHING => return Some(AnimationStateTransition::FallToAirbackdash),
    //       _ => return None,
    //     }
    //   }, // need to do the Rise_Fall_Split
    //   ActionState::STANDING => {
    //     match self.action_state {
    //       ActionState::DASHING => return Some(AnimationStateTransition::IdleToDash),
    //       ActionState::BACKDASHING =>  return Some(AnimationStateTransition::IdleToBackdash),
    //       ActionState::WALKING => return Some(AnimationStateTransition::IdleToWalk),
    //       ActionState::BACKWALKING => return Some(AnimationStateTransition::IdleToBackwalk),
    //       ActionState::CROUCHING => return Some(AnimationStateTransition::IdleToCrouching),
    //       ActionState::JUMPSQUAT => return Some(AnimationStateTransition::ToRise),
    //       _ => return None
    //     }
    //   },
    //   ActionState::BACKDASHING => {
    //     match self.action_state {
    //       ActionState::STANDING => return Some(AnimationStateTransition::BackDashToIdle),
    //       ActionState::BACKWALKING => return Some(AnimationStateTransition::BackDashToBackwalk),
    //       ActionState::CROUCHING => return Some(AnimationStateTransition::ToCrouch),
    //       _ => return None
    //     }
    //   },
    //   ActionState::AIR_DASHING => {
    //     match self.action_state {
    //       ActionState::AIRBORNE => return Some(AnimationStateTransition::AirdashToFall),
    //       _ => return None
    //     }
    //   },
    //   ActionState::AIR_BACKDASHING => {
    //     match self.action_state {
    //       ActionState::AIRBORNE => return Some(AnimationStateTransition::AirbackdashToFall),
    //       _ => return None
    //     }
    //   },
    //   _ => return None
    // }
  }

  // Logic

  /// Reduce component timers by 1 frame
  pub fn tick(&mut self) {
    self.busy = countdown(self.busy);
    self.airdash_lockout = countdown(self.airdash_lockout);
    self.previous_action_state = self.action_state;
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
            self.action_state = ActionState::JUMPSQUAT; 
            self.movement_event = Some(MovementEvent::new(MovementEventType::JUMP, buffer.current_motion));
          },
          9 => {
            if self.action_state == ActionState::DASHING {
              self.action_state = ActionState::JUMPSQUAT; 
              self.movement_event = Some(MovementEvent::new(MovementEventType::DASHJUMP, buffer.current_motion));
            } else {
              self.action_state = ActionState::JUMPSQUAT; 
              self.movement_event = Some(MovementEvent::new(MovementEventType::JUMP, buffer.current_motion));
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
              self.action_state = ActionState::AIR_DASHING;
              self.movement_event = Some(MovementEvent::new(MovementEventType::AIRDASH,buffer.current_motion));
              buffer.consume_motion();
            },
            CommandType::BACK_DASH =>  {
              self.action_state = ActionState::AIR_BACKDASHING;
              self.movement_event = Some(MovementEvent::new(MovementEventType::AIRBACKDASH,buffer.current_motion));
              buffer.consume_motion();
            },
            _ => ()
          }
        }
      }
    }
  }
}



#[derive(Component)]
pub struct CharacterBody {
	pub velocity: Vec2,
  pub facing_vector: f32,
  pub walk_speed: f32,
  pub back_walk_speed: f32,
  pub dash_speed: f32,
  pub air_dash_speed: f32,
  pub air_back_dash_speed: f32,
  pub gravity: f32,
  pub jump_height: f32,
  pub max_airdash_time: u8,
  pub max_air_backdash_time: u8,
  pub airdash_time: u8,
  pub int_force: Option<InterpolatedForce>,
  pub backdash: Box<dyn Backdash>,
  pub jumpdata: Option<JumpData>,
}

impl CharacterBody {
  /// Set the players facing direction
  pub fn set_facing_vector(&mut self, facing_vector: f32) {
    self.facing_vector = facing_vector;
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
    self.jumpdata = Some(JumpData::new(x_velocity, squat, superjump));
  }

  pub fn execute_jump(&mut self, status: &mut CharacterStatus) {
    if let Some(jd) = self.jumpdata.as_mut() {
      if jd.squat > 0 {
        jd.tick();
      } else {
        let jumpheight = if status.get_is_grounded() {
          self.jump_height
        } else {
          self.jump_height * 0.75
        };
        self.velocity = Vec2::new(jd.x_velocity, jumpheight);
        status.is_grounded = false;
        self.jumpdata = None;
        status.set_action_state(ActionState::AIRBORNE);
      }
    }
  }

  pub fn exec_backdash(&self) -> (InterpolatedForce, u8) {
    return self.backdash.exec(self.facing_vector);
  }

  pub fn tick(&mut self) {
    self.airdash_time = countdown(self.airdash_time);
  }
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
      .insert(CharacterStatus::default())
      .insert( CharacterBody {
        facing_vector,
        ..Default::default()
      })
      .insert(AnimationController::new(character_prefix, library));
  }
}

impl Default for CharacterStatus {
  fn default() -> Self {
    CharacterStatus {
      action_state: ActionState::default(),
      previous_action_state: ActionState::default(),
      movement_event: None,
      busy: 0,
      jumpsquat: 3,
      is_grounded: true,
      air_jumps: 1,
      air_jumps_remaining: 1,
      airdashes: 1,
      airdashes_remaining: 1,
      airdash_lockout: 0
    }
  }
}

impl Default for CharacterBody {
  fn default() -> Self {
    CharacterBody {
      velocity: Vec2::ZERO,
      facing_vector: 1.0,
      walk_speed: 4.0,
      back_walk_speed: 2.5,
      dash_speed: 8.0,
      air_dash_speed: 8.0,
      air_back_dash_speed: 6.0,
      gravity: 1.0,
      jump_height: 20.0,
      max_airdash_time: 25,
      max_air_backdash_time: 15,
      airdash_time: 0,
      int_force: None,
      backdash: Box::new(BasicBackdash::new(25.0,20,20)),
      jumpdata: None,
    }
  }
}
