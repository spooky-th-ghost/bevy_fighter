use crate::prelude::*;

pub fn execute_player_physics (
  mut player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterStatus, &mut Transform, &mut TextureAtlasSprite)>,
  mut transition_writer: EventWriter<AnimationTransitionEvent>,
) {
  for (player_id, mut status, mut transform, mut sprite) in query.iter_mut() {
    let tv = status.get_target_velo();
    transform.translation += Vec3::new(tv.x, tv.y, 0.0);
    if transform.translation.y < 0.0 {
      transform.translation.y = 0.0;
      status.land();
      transition_writer.send(
        AnimationTransitionEvent {
          player_id: *player_id,
          transition: AnimationTransition::FallToIdle
        }
      )
    }

    player_data.set_position(player_id, transform.translation);
    let facing_vector = player_data.get_facing_vector(player_id);
    if status.can_turn() {
      sprite.flip_x = facing_vector < 0.0; 
      status.set_facing_vector(facing_vector);
    }
  }
}


pub fn update_debug_ui(
  mut q: QuerySet<(
    QueryState<(&mut Text, &PlayerId)>,
    QueryState<&CharacterStatus>
  )>,
  diagnostics: Res<Diagnostics>,
  player_data: Res<PlayerData>
) {
  let distance = player_data.get_distance();
  let mut player_text: Vec<Vec<String>> = Vec::new();
  let mut fps = 0.0;
  if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
      if let Some(fps_avg) = fps_diagnostic.average() {
          fps = fps_avg;
      }
  }

  for status in q.q1().iter() {
    let mut my_strings: Vec<String> = Vec::new();
    my_strings.push(format!("Action State: {:?} \n", status.action_state));
    my_strings.push(format!("Busy: {:?} \n", status.busy));
    my_strings.push(format!("Is Grounded: {:?} \n", status.is_grounded));
    my_strings.push(format!("Airdashes: {:?} \n", status.airdashes_remaining));
    my_strings.push(format!("Airdash Lockout: {:?} \n", status.airdash_lockout));
    my_strings.push(format!("Velocity: {:?} \n", status.velocity));
    my_strings.push(format!("Facing Vector: {:?} \n", status.facing_vector));
    my_strings.push(format!("FPS: {:.1} \n", fps));
    my_strings.push(format!("Distance: {:?}", distance));
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
  mut query: Query<(&PlayerId, &mut CharacterStatus)>,
  mut transition_writer: EventWriter<AnimationTransitionEvent>,
) {
  for (player_id, mut status) in query.iter_mut() {
    status.tick();
    for buffer in player_data.buffers.iter_mut() {
      if buffer.player_id == *player_id {
        status.update_action_state(buffer);
        // Consume Movement Events
        if let Some(me) = status.movement_event {
          match me.event_type {
            MovementEventType::BACKDASH => status.execute_backdash(),
            _ => ()
          }
          status.clear_movement_event();
        }

        status.execute_airdash();
        let new_velocity = match status.get_action_state() {
          ActionState::Walking => Vec2::new(status.walk_speed * status.facing_vector, 0.0),
          ActionState::BackWalking => Vec2::new(-status.back_walk_speed * status.facing_vector, 0.0),
          ActionState::Dashing => Vec2::new(status.dash_speed * status.facing_vector,0.0),
          ActionState::Airborne => status.velocity - (Vec2::Y * status.gravity),
          ActionState::AirDashing {duration: _, velocity} => velocity,
          ActionState::AirBackDashing {duration: _, velocity} => velocity,
          ActionState::Standing => Vec2::ZERO,
          _ => status.velocity.custom_lerp(Vec2::ZERO, 0.5),
        };
        status.set_velocity(new_velocity);
        status.execute_jump();
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
