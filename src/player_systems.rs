use crate::prelude::*;

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
      if let Some(ct) = buffer.command_type {
        match ct {
          CommandType::DASH => return (ActionState::AIR_DASHING, Some(MovementEventType::AIRDASH)),
          CommandType::BACK_DASH => return (ActionState::AIR_BACKDASHING, Some(MovementEventType::AIRBACKDASH)),
          _ => ()
        }               
      }
      match status.get_action_state() {
        ActionState::AIR_DASHING => return (ActionState::AIR_DASHING, None),
        ActionState::AIR_BACKDASHING => return (ActionState::AIR_BACKDASHING, None),
        _ =>  return (ActionState::AIRBORNE, None)
      }
     
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

pub fn player_airdash(body: &mut CharacterBody, status: &mut CharacterStatus, forward: bool) {
  if status.get_can_airdash() {
    if forward {
      body.airdash_time = body.max_airdash_time;
      status.airdashes_remaining = countdown(status.airdashes_remaining);
      status.busy = 10;
      status.airdash_lockout = 15;
    } else {
      body.air_backdash_time = body.max_air_backdash_time;
      status.airdashes_remaining = countdown(status.airdashes_remaining);
      status.busy = 5;
      status.airdash_lockout = 15;
    }
  }
}

pub fn update_player_status (
  player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterStatus)>,
  mut movement_writer: EventWriter<CharacterMovementEvent>
) {
  for (player_id, mut status) in query.iter_mut() {
    for buffer in player_data.buffers.iter() {
      if buffer.player_id == *player_id {
        let (new_state, movement_event_type) = calculate_action_state(*status, buffer);
        status.set_action_state(new_state);
        if let Some(move_type) = movement_event_type {
          movement_writer.send(CharacterMovementEvent::new(*player_id, move_type, buffer.current_motion));
        }
      }
    }
    status.tick();
  }
}

pub fn update_player_physics (
  player_data: ResMut<PlayerData>, 
  mut movement_events: EventReader<CharacterMovementEvent>,
  mut query: Query<(&PlayerId,&mut CharacterStatus, &mut CharacterBody)>
) {
  let events: Vec<&CharacterMovementEvent> = movement_events.iter().collect();
  for (player_id, mut status, mut body) in query.iter_mut() {
    if player_id == &PlayerId::P2 {
      println!("{:?}", status.get_action_state());
    }
    let facing_vector = player_data.get_facing_vector(player_id);
    if status.get_is_grounded() {body.set_facing_vector(facing_vector);}
    body.tick();
    if body.airdash_time == 0 {
      match status.get_action_state() {
        ActionState::AIR_DASHING | ActionState::AIR_BACKDASHING => status.set_action_state(ActionState::AIRBORNE),
        _ => (),
      }
    }

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
          MovementEventType::AIRDASH => {
            if status.get_can_airdash() {
              player_airdash(&mut body, &mut status, true);
            Vec2::X * body.air_dash_speed * body.facing_vector
            } else {
              body.velocity - (Vec2::Y * body.gravity)
            }
          },
          MovementEventType::AIRBACKDASH => {
            if status.get_can_airdash() {
              player_airdash(&mut body, &mut status, false);
              Vec2::X * body.air_back_dash_speed * -body.facing_vector
            } else {
              body.velocity - (Vec2::Y * body.gravity)
            }
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
        ActionState::AIR_DASHING => Vec2::X * body.air_dash_speed * body.facing_vector,
        ActionState::AIR_BACKDASHING => Vec2::X * body.air_back_dash_speed * -body.facing_vector,
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


pub fn update_debug_ui(
  mut q: QuerySet<(
    QueryState<(&mut Text, &PlayerId)>,
    QueryState<(&CharacterStatus, &CharacterBody, &PlayerId)>
  )>
) {
  let mut player_text: Vec<Vec<String>> = Vec::new();

  for (status, body, p_player_id) in q.q1().iter() {
    let mut my_strings: Vec<String> = Vec::new();
    my_strings.push(format!("Action State: {:?} \n", status.action_state));
    my_strings.push(format!("Busy: {:?} \n", status.busy));
    my_strings.push(format!("Is Grounded: {:?} \n", status.is_grounded));
    my_strings.push(format!("Airdashes: {:?} \n", status.airdashes_remaining));
    my_strings.push(format!("Airdash Lockout: {:?} \n", status.airdash_lockout));
    my_strings.push(format!("Velocity: {:?} \n", body.velocity));
    my_strings.push(format!("Airdash Time: {:?} \n", body.airdash_time));
    my_strings.push(format!("Air Backdash Time: {:?}\n", body.air_backdash_time));
    let strings_to_push = my_strings.clone();
    player_text.push(strings_to_push);
  }

  for (mut text, player_id) in q.q0().iter_mut() {
    let index = match player_id {
      PlayerId::P1 => 0,
      PlayerId::P2 => 1
    };
      text.sections[0].value = player_text[index][0].clone();
      text.sections[1].value = player_text[index][1].clone();
      text.sections[2].value = player_text[index][2].clone();
      text.sections[3].value = player_text[index][3].clone();
      text.sections[4].value = player_text[index][4].clone();
      text.sections[5].value = player_text[index][5].clone();
      text.sections[6].value = player_text[index][6].clone();
      text.sections[7].value = player_text[index][7].clone();
  }
}
