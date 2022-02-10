pub use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
  P1,
  P2
}

#[derive(Serialize,Deserialize)]
pub struct CharacterMovementSerialized {
  pub jumpsquat: u8,
  pub air_jumps: u8,
  pub airdashes: u8,
  pub air_dash_speed: f32,
  pub air_back_dash_speed: f32,
  pub walk_speed: f32,
  pub back_walk_speed: f32,
  pub dash_speed: f32,
  pub gravity: f32,
  pub jump_height: f32,
  pub max_airdash_time: u8,
  pub max_air_backdash_time: u8,
  pub backdash: Backdash
}

#[derive(Component, Clone, Debug)]
pub struct CharacterMovement {
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
  pub attacks: HashMap<String, Attack>
}

impl CharacterMovement {
  pub fn from_serialized(s: CharacterMovementSerialized, library: &CharacterLibrary, character_name: &str) -> Self {
    let mut attacks = HashMap::new();
    let my_regex = Regex::new(&format!("(^{}.+)", character_name)[..]).unwrap();
    for (attack_id, attack) in library.read_attacks() {
      if my_regex.is_match(attack_id) {
        attacks.insert(attack_id.clone(), attack.clone());
      }
    }

    CharacterMovement {
      jumpsquat: s.jumpsquat,
      air_jumps: s.air_jumps,
      airdashes: s.airdashes,
      air_dash_speed: s.air_dash_speed,
      air_back_dash_speed: s.air_back_dash_speed,
      walk_speed: s.walk_speed,
      back_walk_speed: s.back_walk_speed,
      dash_speed: s.dash_speed,
      gravity: s.gravity,
      jump_height: s.jump_height,
      max_airdash_time: s.max_airdash_time,
      max_air_backdash_time: s.max_air_backdash_time,
      backdash: s.backdash,
      facing_vector: 1.0,
      movement_event: None,
      busy: 0,
      is_grounded: true,
      air_jumps_remaining: s.air_jumps,
      airdashes_remaining: s.airdashes,
      airdash_lockout: 0,
      velocity: Vec2::ZERO,
      int_force: None,
      attacks,
      action_state: ActionState::Standing,
      previous_action_state: ActionState::Standing
    }
  }
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
    self.action_state = ActionState::Standing;
    self.airdashes_remaining = self.airdashes;
    self.airdash_lockout = 0;
  }

  // Getters

  /// Get a players ActionState
  pub fn get_action_state(&self) -> ActionState {
    return self.action_state.clone();
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
      ActionState::Standing
      | ActionState::Airborne
      | ActionState::Walking
      | ActionState::BackWalking
      | ActionState::Crouching => return true,
      _ => return false,
    }
  }

  pub fn calculate_transition(&self) -> Option<AnimationTransition> {

    match self.action_state {
      ActionState::Jumpsquat {squat: _, velocity: _} => return Some(AnimationTransition::ToRise),
      ActionState::Walking => return Some(AnimationTransition::ToWalk),
      ActionState::BackWalking => return Some(AnimationTransition::ToBackwalk),
      ActionState::Crouching => return Some(AnimationTransition::ToCrouch),
      ActionState::Dashing => return Some(AnimationTransition::ToDash),
      ActionState::BackDashing => return Some(AnimationTransition::ToBackdash),
      ActionState::AirDashing {duration: _, velocity: _} => return Some(AnimationTransition::ToAirdash),
      ActionState::AirBackDashing {duration: _, velocity: _} => return Some(AnimationTransition::ToAirBackdash),
      ActionState::Airborne => {
        match self.previous_action_state { 
          ActionState::AirDashing {duration:_, velocity: _} => return Some(AnimationTransition::AirdashToFall),
          ActionState::AirBackDashing {duration:_, velocity: _} => return Some(AnimationTransition::AirbackdashToFall),
          _ => return None,
        }
      }, // need to do the Rise_Fall_Split
      ActionState::Standing => {
        match self.previous_action_state {
          ActionState::Dashing => return Some(AnimationTransition::DashToIdle),
          ActionState::BackDashing =>  return Some(AnimationTransition::BackDashToIdle),
          ActionState::Walking => return Some(AnimationTransition::WalkToIdle),
          ActionState::BackWalking => return Some(AnimationTransition::BackwalkToIdle),
          ActionState::Crouching => return Some(AnimationTransition::CrouchToIdle),
          ActionState::Airborne => return Some(AnimationTransition::FallToIdle),
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
    self.previous_action_state = self.action_state.clone();
    self.action_state.tick();
  }

  // Run each frame to determine the players action state and if any movement events should be executed
  pub fn update_action_state(&mut self, buffer: &mut FighterInputBuffer) {
    if !self.get_is_busy() {
      if self.get_is_grounded() {
        match buffer.current_motion {
          5 => self.action_state = ActionState::Standing,
          6 => {
            if self.action_state != ActionState::Dashing {
              self.action_state = ActionState::Walking;
            }
          },
          4 => self.action_state = ActionState::BackWalking,
          1 | 2 | 3 => self.action_state = ActionState::Crouching,
          7 | 8 => {
            self.buffer_jump(buffer.current_motion, false,false, false);
          },
          9 => {
            if self.action_state == ActionState::Dashing {
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
              self.action_state = ActionState::Dashing;
              buffer.consume_motion();
            },
            CommandType::BACK_DASH => {
              self.action_state = ActionState::BackDashing;
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
      ActionState::AirJumpsquat { squat, velocity}
    } else {
      ActionState::Jumpsquat {squat, velocity}
    };
  }

  pub fn buffer_airdash(&mut self, forward: bool) {
    if self.get_can_airdash() {
      self.airdashes_remaining = countdown(self.air_jumps_remaining);
      self.busy = 5;
      self.airdash_lockout = 15;

      if forward {
        self.action_state = ActionState::AirDashing {duration: self.max_airdash_time, velocity: Vec2::X * self.air_dash_speed * self.facing_vector};
      } else {
        self.action_state = ActionState::AirBackDashing {duration: self.max_air_backdash_time, velocity: Vec2::X * self.air_dash_speed * -self.facing_vector };
      }
    }
  }

  pub fn execute_jump(&mut self) {
    if self.action_state.is_finished_jumping() {
      self.is_grounded = false;
      self.velocity = self.action_state.get_jump_velocity();
      self.action_state = ActionState::Airborne;
    }
  }

  pub fn execute_airdash(&mut self) {
    if self.action_state.is_finished_airdashing() {
      self.action_state = ActionState::Airborne;
    }
  }

  pub fn execute_backdash(&mut self) {
    match self.backdash {
      Backdash::Standard {speed, busy, motion_duration} => {
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

/// States representing all possible player actions
#[derive(Clone, Debug)]
pub enum ActionState {
  Dashing,
  Walking,
  BackWalking,
  Attacking {duration: u8, attack: Attack},
  AttackingAirborne,
  Blocking,
  CrouchBlocking,
  Crouching,
  Jumpsquat {squat: u8, velocity: Vec2 },
  AirJumpsquat {squat: u8, velocity: Vec2 },
  Airborne,
  Juggle,
  Standing,
  BackDashing,
  AirDashing {duration: u8, velocity: Vec2},
  AirBackDashing {duration: u8, velocity: Vec2} 
}

impl PartialEq for ActionState {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (ActionState::Dashing, ActionState::Dashing) => true,
      (ActionState::Walking, ActionState::Walking) => true,
      (ActionState::BackWalking, ActionState::BackWalking) => true,
      (ActionState::Attacking {duration: _, attack: _}, ActionState::Attacking {duration: _, attack: _}) => true,
      (ActionState::Blocking, ActionState::Blocking) => true,
      (ActionState::CrouchBlocking, ActionState::CrouchBlocking) => true,
      (ActionState::Crouching, ActionState::Crouching) => true,
      (ActionState::Airborne, ActionState::Airborne) => true,
      (ActionState::Juggle, ActionState::Juggle) => true,
      (ActionState::Standing, ActionState::Standing) => true,
      (ActionState::BackDashing, ActionState::BackDashing) => true,
      (ActionState::Jumpsquat {squat:_, velocity:_},ActionState::Jumpsquat {squat: _, velocity:_ }) => true,
      (ActionState::AirJumpsquat {squat:_, velocity:_}, ActionState::AirJumpsquat {squat:_, velocity:_}) => true,
      (ActionState::AirDashing {duration:_, velocity:_},ActionState::AirDashing {duration:_, velocity:_},) => true,
      (ActionState::AirBackDashing {duration:_, velocity:_}, ActionState::AirBackDashing {duration:_, velocity:_}) => true,
      _ => false,
    }
  }
}

impl ActionState {
  pub fn tick(&mut self) {
    match self {
      ActionState::Jumpsquat { squat, velocity: _} => {
        *squat = countdown(*squat);
      },
      ActionState::AirJumpsquat { squat, velocity: _} => {
        *squat = countdown(*squat);
      },
      ActionState::AirDashing {duration, velocity: _} => {
        *duration = countdown(*duration);
      },
      ActionState::AirBackDashing {duration, velocity: _} => {
        *duration = countdown(*duration);
      },
      _ => ()
    }
  }

  pub fn is_finished_jumping(&self) -> bool {
    match self {
      ActionState::Jumpsquat{squat, velocity: _} => {
        if *squat == 0 {
          return true;
        } else {
          return false;
        }
      },
      ActionState::AirJumpsquat{squat, velocity: _} => {
        if *squat == 0 {
          return true;
        } else {
          return false;
        }
      }
      _ => return false,
    }
  }

  pub fn is_finished_airdashing(&self) -> bool {
    match self {
      ActionState::AirDashing {duration, velocity: _} => {
        if *duration == 0 {
          return true;
        } else {
          return false;
        }
      },
      ActionState::AirBackDashing {duration, velocity: _} => {
        if *duration == 0 {
          return true;
        } else {
          return false;
        }
      },
      _ => return false,
    }
  }

  pub fn get_jump_velocity(&self) -> Vec2{
    match self {
      ActionState::AirJumpsquat {squat: _, velocity} => *velocity,
      ActionState::Jumpsquat {squat: _, velocity} => *velocity,
      _ => Vec2::ZERO,
    }
  }
}

impl Default for ActionState {
  fn default() -> Self {Self::Standing}
}


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Backdash {
  Standard {speed: f32, busy: u8, motion_duration: u8},
  Teleport { distance: f32, busy: u8, motion_duration: u8},
  Leap {busy: u8, motion_duration: u8}
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
    library: &CharacterLibrary,
    texture_atlas: Handle<TextureAtlas>
  );
}

impl SpawnPlayer for Commands<'_, '_> {
  fn spawn_player(
    &mut self, 
    player_id: PlayerId, 
    character_prefix: &str, 
    library: &CharacterLibrary,
    texture_atlas: Handle<TextureAtlas>) {
    let transform = match player_id {
      PlayerId::P1 => Transform::from_xyz(-40.0,0.0,0.0),
      PlayerId::P2 => Transform::from_xyz(40.0,0.0,0.0),
    };

    let movement = library.get_movement(character_prefix).unwrap();

    self.spawn_bundle(SpriteSheetBundle {
        texture_atlas,
        transform,
        ..Default::default()
      })
      .insert(player_id)
      .insert(movement)
      .insert(AnimationController::new(character_prefix, library));
  }
}
