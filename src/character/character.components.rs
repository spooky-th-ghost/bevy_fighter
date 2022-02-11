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
pub struct BeatChain {
  pub all_attacks: Vec<String>,
  pub available_attacks: Vec<String>,
}

impl BeatChain {
  pub fn from_attack_names(attack_names: Vec<String>) -> Self {
    BeatChain {
      all_attacks: attack_names.clone(),
      available_attacks: attack_names.clone()
    }
  }

  pub fn reset(&mut self) {
    self.available_attacks = self.all_attacks.clone();
  }
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
  pub attacks: HashMap<String, Attack>,
  pub beat_chain: BeatChain
}

impl CharacterMovement {
  pub fn from_serialized(s: CharacterMovementSerialized, library: &CharacterLibrary, character_name: &str) -> Self {
    let mut attacks = HashMap::new();
    let mut attack_names = Vec::new();
    let my_regex = Regex::new(&format!("(^{}.+)", character_name)[..]).unwrap();
    for (attack_id, attack) in library.read_attacks() {
      if my_regex.is_match(attack_id) {
        let trimmed_attack_name = attack_id.replace(character_name, "").replace("_","");
        attack_names.push(trimmed_attack_name.clone());
        attacks.insert(trimmed_attack_name.clone(), attack.clone());
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
      beat_chain: BeatChain::from_attack_names(attack_names),
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
  pub fn find_attack(&mut self, motion: u8, buttons: String) -> Option<Attack> {
    let mut current_regex: Regex;
    for button in buttons.chars().rev() {
      current_regex = Regex::new(&format!("({}).*({})", motion, button)[..]).unwrap();
      for attack_name in self.beat_chain.available_attacks.iter() {
        if current_regex.is_match(attack_name) {
          return self.attacks.get(attack_name).cloned();
        }
      }
    }
    return None;
  }

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

    match &self.action_state {
      ActionState::Attacking {duration: _, attack} => return Some(AnimationTransition::Attack{name: attack.name.clone()}),
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
          ActionState::Attacking {duration: _, attack: _} => return Some(AnimationTransition::ToIdle),
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
    self.action_state.tick();
    self.previous_action_state = self.action_state.clone();
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

  pub fn attack_to_execute(&mut self,  buffer: &mut FighterInputBuffer) -> Option<Attack> {
    if !self.get_is_busy() {
      if buffer.current_press.any_pressed() {
        return self.find_attack(buffer.current_motion, buffer.current_press.to_string());
      } else {
        return None;
      }
    } else {
      return None;
    }
  }

  pub fn buffer_attack(&mut self, attack: Attack) {
    self.action_state = ActionState::Attacking {duration: attack.busy, attack: attack.clone()};
    self.busy = attack.busy + 1;
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

  pub fn execute_attack(&mut self) {
    if self.action_state.is_finished_attacking() {
      self.action_state = ActionState::Standing;
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
    std::mem::discriminant(self) == std::mem::discriminant(other)
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
      ActionState::Attacking {duration, attack: _} => {
        *duration = countdown(*duration);
      }
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

  pub fn is_finished_attacking(&self) -> bool {
    match self {
      ActionState::Attacking {duration, attack: _} => {
        if *duration == 0 {
          return true;
        } else {
          return false;
        }
      },
      _ => return false,
    }
  }

  pub fn perform_hitbox_events(&self) {
     match self {
      ActionState::Attacking {duration, attack} => {
        // Need to spawn the hitboxes in hitbox events here
      },
      _ => ()
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
  );
}

impl SpawnPlayer for Commands<'_, '_> {
  fn spawn_player(
    &mut self, 
    player_id: PlayerId, 
    character_prefix: &str, 
    library: &CharacterLibrary,
  ) {
    let transform = match player_id {
      PlayerId::P1 => Transform::from_xyz(-40.0,0.0,0.0),
      PlayerId::P2 => Transform::from_xyz(40.0,0.0,0.0),
    };

    let movement = library.get_movement(character_prefix).unwrap();
    let texture_atlas = library.get_atlas(character_prefix).unwrap();

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
