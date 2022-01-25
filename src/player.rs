use crate::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum PlayerId {
  P1,
  P2
}

#[derive(Component, Clone, Copy, Debug)]
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

  pub fn land(&mut self) {
    self.is_grounded = true;
    self.air_jumps_remaining = self.air_jumps;
    self.action_state = ActionState::STANDING;
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

    self.spawn_bundle(SpriteBundle {
        sprite: Sprite{
        color,
        custom_size: Some(Vec2::new(30.0, 60.0)),
        ..Default::default()
      },
        transform,
        ..Default::default()
      })
      .insert(player_id)
      .insert(CharacterStatus::default())
      .insert( CharacterBody {
        facing_vector,
        ..Default::default()
      });
  }
}

pub fn calculate_action_state(status: CharacterStatus, buffer:&FighterInputBuffer) -> (ActionState, Option<MovementEventType>) {
  if !status.get_is_busy() {
    if status.get_is_grounded() {
      match buffer.current_motion {
        7 | 8 | 9 => {
          match status.get_action_state() {
            ActionState::DASHING =>  return (ActionState::JUMPSQUAT, Some(MovementEventType::DASHJUMP)),
            _ => return (ActionState::JUMPSQUAT, Some(MovementEventType::JUMP))
          }
        },
        _ => ()
      }
      match status.get_action_state() {
        ActionState::WALKING | ActionState::BACKWALKING | ActionState::CROUCHING | ActionState::STANDING | ActionState::BACKDASHING => {
          if let Some(ct) = buffer.command_type {
            match ct {
              CommandType::DASH => return (ActionState::DASHING, None),
              CommandType::BACK_DASH => return (ActionState::BACKDASHING,Some(MovementEventType::BACKDASH)),
              _ => ()
            }               
          } else {
            match buffer.current_motion {
              5 => return (ActionState::STANDING, None),
              6 => return (ActionState::WALKING, None),
              4 => return (ActionState::BACKWALKING, None),
              1 | 2 | 3 => return (ActionState::CROUCHING, None),
              7 | 8 | 9 => return (ActionState::JUMPSQUAT, Some(MovementEventType::JUMP)),
              _ => ()
            }
          }
        },
        ActionState::DASHING => {
          if let Some(ct) = buffer.command_type {
            match ct {
              CommandType::DASH => return (ActionState::DASHING, None),
              CommandType::BACK_DASH => return (ActionState::BACKDASHING,Some(MovementEventType::BACKDASH)),
              _ => ()
            }               
          } else {
            match buffer.current_motion {
              5 => return (ActionState::STANDING, None),
              6 => return (ActionState::DASHING, None),
              4 => return (ActionState::BACKWALKING, None),
              1 | 2 | 3 => return (ActionState::CROUCHING, None),
              7 | 8 | 9 => return (ActionState::JUMPSQUAT, Some(MovementEventType::JUMP)),
              _ => ()
            }
          }
        }
        _ => ()
      }
    } else {
      return (ActionState::AIRBORNE, None);
    }
  }
  return (status.get_action_state(), None);
}

pub fn buffer_player_jump(body: &mut CharacterBody, status: &mut CharacterStatus, motion: u8, superjump: bool, dashing: bool) {
  let forward_vector = if dashing {
    2.0
  } else {
    1.0
  };
  let x_velocity = match motion {
    7 => body.facing_vector * (-body.back_walk_speed*2.0),
    9 => body.facing_vector * (body.walk_speed * forward_vector),
    _ => 0.0
  };
  body.jumpdata = Some(JumpData::new(x_velocity, status.jumpsquat, superjump));
  status.set_busy(status.jumpsquat + 10);
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
  fn new(
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

#[derive(Debug, Clone, Copy)]
pub enum MovementEventType {
  JUMP,
  DASH,
  DASHJUMP,
  BACKDASH,
  AIRDASH,
  AIRBACKDASH,
}


pub fn update_player_status (
  player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterStatus)>,
  mut movement_writer: EventWriter<CharacterMovementEvent>
) {
  for (player_id, mut char_status) in query.iter_mut() {
    for buffer in player_data.buffers.iter() {
      if buffer.player_id == *player_id {
        let (new_state, movement_event_type) = calculate_action_state(*char_status, buffer);
        char_status.set_action_state(new_state);
        if let Some(move_type) = movement_event_type {
          movement_writer.send(CharacterMovementEvent::new(*player_id, move_type, buffer.current_motion));
        }
      }
    }
    char_status.tick();
  }
}

pub fn update_player_physics (
  player_data: ResMut<PlayerData>, 
  mut movement_events: EventReader<CharacterMovementEvent>,
  mut query: Query<(&PlayerId,&mut CharacterStatus, &mut CharacterBody)>
) {
  let events: Vec<&CharacterMovementEvent> = movement_events.iter().collect();
  for (player_id, mut status, mut body) in query.iter_mut() {
    let facing_vector = player_data.get_facing_vector(player_id);
    body.set_facing_vector(facing_vector);
    let mut movement_event_found = false;
    let mut new_velocity = Vec2::ZERO;

    for event in &events {
      if event.player_id == *player_id {
        movement_event_found = true;
        new_velocity = match event.event_type {
          MovementEventType::BACKDASH => {
            let (int_force, busy) = body.exec_backdash();
            body.set_i_force(int_force);
            status.set_busy(busy);
            Vec2::ZERO
          },
          MovementEventType::JUMP => {
            buffer_player_jump(&mut body, &mut status, event.motion, false, false);
            Vec2::ZERO
          },
          MovementEventType::DASHJUMP => {
            buffer_player_jump(&mut body, &mut status, event.motion, false, true);
            Vec2::ZERO
          },
          _ => Vec2::ZERO
        }
      }
    }
    if !movement_event_found{
      new_velocity = match status.get_action_state() {
        ActionState::WALKING => Vec2::new(body.walk_speed * body.facing_vector, 0.0),
        ActionState::BACKWALKING => Vec2::new(-body.back_walk_speed * body.facing_vector, 0.0),
        ActionState::DASHING => Vec2::new(body.dash_speed * body.facing_vector,0.0),
        ActionState::AIRBORNE => body.velocity - (Vec2::Y * body.gravity),
        _ =>  body.velocity.custom_lerp(Vec2::ZERO, 0.2),
      };
    }
    body.set_velocity(new_velocity);
    body.execute_jump(&mut status);
  }
}

pub fn execute_player_physics (
  mut player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterStatus, &mut CharacterBody, &mut Transform)>
) {
  for (player_id, mut status, mut body, mut transform) in query.iter_mut() {
    let tv = body.get_target_velo();
    transform.translation += Vec3::new(tv.x, tv.y, 0.0);
    if transform.translation.y < 0.0 {
      transform.translation.y = 0.0;
      status.land();
    }
    player_data.set_position(player_id, transform.translation);
  }
}
