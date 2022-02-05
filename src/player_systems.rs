use crate::prelude::*;

pub fn player_airdash(body: &mut CharacterBody, status: &mut CharacterStatus, forward: bool) {
  if status.get_can_airdash() {
    status.airdashes_remaining = countdown(status.airdashes_remaining);
    status.busy = 5;
    status.airdash_lockout = 15;

    if forward {
      body.airdash_time = body.max_airdash_time;
    } else {
      body.airdash_time = body.max_air_backdash_time;
    }
  }
}

pub fn execute_player_physics (
  mut player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterStatus, &mut CharacterBody, &mut Transform, &mut TextureAtlasSprite)>,
  mut transition_writer: EventWriter<AnimationTransitionEvent>,
) {
  for (player_id, mut status, mut body, mut transform, mut sprite) in query.iter_mut() {
    let tv = body.get_target_velo();
    transform.translation += Vec3::new(tv.x, tv.y, 0.0);
    if transform.translation.y < 0.0 {
      transform.translation.y = 0.0;
      status.land();
      transition_writer.send(
        AnimationTransitionEvent {
          player_id: *player_id,
          transition: AnimationStateTransition::FallToIdle
        }
      )
    }

    player_data.set_position(player_id, transform.translation);
    let facing_vector = player_data.get_facing_vector(player_id);
    if status.can_turn() {
      sprite.flip_x = facing_vector < 0.0; 
      body.set_facing_vector(facing_vector);
    }
  }
}


pub fn update_debug_ui(
  mut q: QuerySet<(
    QueryState<(&mut Text, &PlayerId)>,
    QueryState<(&CharacterStatus, &CharacterBody)>
  )>,
  diagnostics: Res<Diagnostics>,
) {
  let mut player_text: Vec<Vec<String>> = Vec::new();
  let mut fps = 0.0;
  if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
      if let Some(fps_avg) = fps_diagnostic.average() {
          fps = fps_avg;
      }
  }

  for (status, body) in q.q1().iter() {
    let mut my_strings: Vec<String> = Vec::new();
    my_strings.push(format!("Action State: {:?} \n", status.action_state));
    my_strings.push(format!("Busy: {:?} \n", status.busy));
    my_strings.push(format!("Is Grounded: {:?} \n", status.is_grounded));
    my_strings.push(format!("Airdashes: {:?} \n", status.airdashes_remaining));
    my_strings.push(format!("Airdash Lockout: {:?} \n", status.airdash_lockout));
    my_strings.push(format!("Velocity: {:?} \n", body.velocity));
    my_strings.push(format!("Airdash Time: {:?} \n", body.airdash_time));
    my_strings.push(format!("Facing Vector: {:?} \n", body.facing_vector));
    my_strings.push(format!("FPS: {:.1}", fps));
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
      text.sections[8].value = player_text[index][8].clone();
  }
}

pub fn determine_player_velocity_and_state (
  mut player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterStatus, &mut CharacterBody)>,
  mut transition_writer: EventWriter<AnimationTransitionEvent>,
) {
  for (player_id, mut status, mut body) in query.iter_mut() {
    status.tick();
    body.tick();
    for buffer in player_data.buffers.iter_mut() {
      if buffer.player_id == *player_id {
        status.update_action_state(buffer);
        // Consume Movement Events
        if let Some(me) = status.movement_event {
          match me.event_type {
            MovementEventType::BACKDASH => {
              let (int_force, busy) = body.exec_backdash();
              body.set_i_force(int_force);
              status.set_busy(busy);
            },
            MovementEventType::JUMP => {
              body.buffer_jump(me.motion, false, false, false);
              status.set_busy(12);
            },
            MovementEventType::DASHJUMP => {
              body.buffer_jump(me.motion, false, true, false);
              status.set_busy(12);
            },
            MovementEventType::SUPERJUMP => {
              body.buffer_jump(me.motion, true, false, false);
              status.set_busy(12);
            },
            MovementEventType::AIRDASH => {
              if status.get_can_airdash() {
                player_airdash(&mut body, &mut status, true);
              }
            },
            MovementEventType::AIRBACKDASH => {
              if status.get_can_airdash() {
                player_airdash(&mut body, &mut status, false);
              }
            },
            _ => ()
          }
          status.clear_movement_event();
        }

        if body.airdash_time == 0 {
          match status.get_action_state() {
            ActionState::AIR_DASHING | ActionState::AIR_BACKDASHING => status.set_action_state(ActionState::AIRBORNE),
            _ => (),
          }
        }

        let new_velocity = match status.get_action_state() {
          ActionState::WALKING => Vec2::new(body.walk_speed * body.facing_vector, 0.0),
          ActionState::BACKWALKING => Vec2::new(-body.back_walk_speed * body.facing_vector, 0.0),
          ActionState::DASHING => Vec2::new(body.dash_speed * body.facing_vector,0.0),
          ActionState::AIRBORNE => body.velocity - (Vec2::Y * body.gravity),
          ActionState::AIR_DASHING => Vec2::X * body.air_dash_speed * body.facing_vector,
          ActionState::AIR_BACKDASHING => Vec2::X * body.air_back_dash_speed * -body.facing_vector,
          ActionState::STANDING => Vec2::ZERO,
          _ => body.velocity.custom_lerp(Vec2::ZERO, 0.5),
        };
        body.set_velocity(new_velocity);
        body.execute_jump(&mut status);
      }
    }
    if status.get_should_transition() {
      if let Some(transition) =  status.calculate_transition() {
        transition_writer.send(
    AnimationTransitionEvent {
            player_id: *player_id,
            transition,
          }
        )
      }
    }
  }
}
