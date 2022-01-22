use crate::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
  P1,
  P2
}

#[derive(Component, Clone, Copy)]
pub struct CharacterStatus {
  pub action_state: ActionState,
  pub busy: u8,
  pub jumpsquat: u8,
  pub is_grounded: bool,
  pub air_jumps: u8,
  pub air_jumps_remaining: u8
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
    return self.busy == 0;
  }

  // Logic

  /// Reduce component timers by 1 frame
  pub fn tick(&mut self) {
    self.busy = countdown(self.busy);
  }
}

impl Default for CharacterStatus {
  fn default() -> Self {
    CharacterStatus {
      action_state: ActionState::default(),
      busy: 0,
      jumpsquat: 3,
      is_grounded: true,
      air_jumps: 1,
      air_jumps_remaining: 1
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
  pub gravity: f32,
  pub jump_height: f32,
  pub int_force: Option<InterpolatedForce>,
  pub backdash: Box<dyn Backdash>,
  pub jumpdata: Option<JumpData>,
}

impl CharacterBody {
  /// Set the players facing direction
  pub fn set_facing_vector(&mut self, facing_vector: f32) {
    self.facing_vector = facing_vector;
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
      gravity: 1.0,
      jump_height: 15.0,
      int_force: None,
      backdash: Box::new(BasicBackdash::new(25.0,20,20)),
      jumpdata: None,
    }
  }
}

#[derive(Bundle)]
pub struct CharacterBundle {
  pub player_id: PlayerId,
  pub sprite: Sprite,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
  pub visibility: Visibility,
  pub status: CharacterStatus,
  pub body: CharacterBody,
}

impl Default for CharacterBundle {
  fn default() -> Self {
    CharacterBundle {
      player_id: PlayerId::P1,
      sprite: Sprite {
        ..Default::default()
      },
      transform: Transform::default(),
      global_transform: GlobalTransform::default(),
      visibility: Visibility::default(),
      status: CharacterStatus::default(),
      body: CharacterBody::default(),
    }
  }
}

pub trait SpawnPlayer {
  fn spawn_player(&mut self, player_id: PlayerId);
}

impl SpawnPlayer for Commands<'_, '_> {
  fn spawn_player(&mut self, player_id: PlayerId) {
    let (transform, color, facing_vector) = match player_id {
      PlayerId::P1 => (
        Transform::from_xyz(-40.0,0.0,0.0),
        Color::TEAL,
        1.0
      ),
      PlayerId::P2 => (
        Transform::from_xyz(40.0,0.0,0.0),
        Color::INDIGO,
        -1.0
      )
    };

    self.spawn_bundle(
      CharacterBundle {
        player_id,
        sprite: Sprite {
          color,
          custom_size: Some(Vec2::new(30.0, 60.0)),
          ..Default::default()
        },
        transform,
        global_transform: GlobalTransform::default(),
        visibility: Visibility::default(),
        status: CharacterStatus::default(),
        body: CharacterBody{
          facing_vector,
            ..Default::default()
        },
      }
    );
  }
}

pub fn calculate_action_state(status: CharacterStatus, buffer:&FighterInputBuffer) -> ActionState {
  if !status.get_is_busy() {
    if status.get_is_grounded() {
      match status.get_action_state() {
        ActionState::WALKING | ActionState::BACKWALKING | ActionState::CROUCHING | ActionState::STANDING => {
          match buffer.current_motion {
            5 => return ActionState::STANDING,
            6 => return ActionState::WALKING,
            4 => return ActionState::BACKWALKING,
            1 | 2 | 3 => return ActionState::CROUCHING,
            7 | 8 | 9 => return ActionState::JUMPSQUAT,
            _ => ()
          }
          if let Some(ct) = buffer.command_type {
            match ct {
              CommandType::DASH => return ActionState::DASHING,
              CommandType::BACK_DASH => return ActionState::BACKDASHING,
              _ => ()
            }               
          }
        },
        ActionState::DASHING => {
          match buffer.current_motion {
            5 => return ActionState::STANDING,
            6 => return ActionState::DASHING,
            4 => return ActionState::BACKWALKING,
            1 | 2 | 3 => return ActionState::CROUCHING,
            7 | 8 | 9 => return ActionState::JUMPSQUAT,
            _ => ()
          }
        }
        _ => ()
      }
    } else {
      return ActionState::AIRBORNE;
    }
  }
  return status.get_action_state();
}

#[derive(Debug)]
pub struct CharacterMovementEvent{
  /// PlayerId for the movement event
  pub player_id: PlayerId,
  /// Type of movement event
  pub event_type: MovementEventType,
}

impl CharacterMovementEvent{
  fn new(
    player_id: PlayerId, 
    event_type: MovementEventType
  ) -> Self {
    CharacterMovementEvent {
      player_id,
      event_type
    }
  }
}

#[derive(Debug)]
pub enum MovementEventType {
  JUMP,
  DASH,
  BACKDASH,
  AIRDASH,
  AIRBACKDASH,
}


pub fn update_player_status (player_data: ResMut<PlayerData>, mut query: Query<(&PlayerId, &mut CharacterStatus)>) {
  for (player_id, mut char_status) in query.iter_mut() {
    for buffer in player_data.buffers.iter() {
      if buffer.player_id == *player_id {
        let new_state = calculate_action_state(*char_status, buffer);
        char_status.set_action_state(new_state);
        // Need something here to write MovementEvents
      }
    }
    char_status.tick();
  }
}

pub fn update_player_physics (player_data: ResMut<PlayerData>, mut query: Query<(&PlayerId, &mut CharacterBody)>) {
  for (player_id, mut char_body) in query.iter_mut() {
    for buffer in player_data.buffers.iter() {
      if buffer.player_id == *player_id {

        let facing_vector = player_data.get_facing_vector(player_id);
        char_body.set_facing_vector(facing_vector);
        // Need to update velocity and run physics calculation here   
      }
    }
  }
}

