use crate::prelude::*;

pub fn execute_player_physics (
  mut player_data: ResMut<PlayerData>, 
  mut query: Query<(&PlayerId, &mut CharacterMovement, &mut Transform, &mut TextureAtlasSprite)>,
  mut transition_writer: EventWriter<AnimationTransitionEvent>,
) {
  for (player_id, mut movement, mut transform, mut sprite) in query.iter_mut() {
    let tv = movement.get_target_velo();
    transform.translation += Vec3::new(tv.x, tv.y, 0.0);
    if transform.translation.y < 0.0 {
      transform.translation.y = 0.0;
      movement.land();
      transition_writer.send(
        AnimationTransitionEvent {
          player_id: *player_id,
          transition: AnimationTransition::FallToIdle
        }
      )
    }

    player_data.set_position(player_id, transform.translation);
    let facing_vector = player_data.get_facing_vector(player_id);
    if movement.can_turn() {
      sprite.flip_x = facing_vector < 0.0; 
      movement.set_facing_vector(facing_vector);
    }
  }
}


pub fn update_debug_ui(
  mut q: QuerySet<(
    QueryState<(&mut Text, &PlayerId)>,
    QueryState<&CharacterMovement>
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

  for movement in q.q1().iter() {
    let mut my_strings: Vec<String> = Vec::new();
    my_strings.push(format!("Action State: {:?} \n", movement.action_state));
    my_strings.push(format!("Busy: {:?} \n", movement.busy));
    my_strings.push(format!("Is Grounded: {:?} \n", movement.is_grounded));
    my_strings.push(format!("Airdashes: {:?} \n", movement.airdashes_remaining));
    my_strings.push(format!("Airdash Lockout: {:?} \n", movement.airdash_lockout));
    my_strings.push(format!("Velocity: {:?} \n", movement.velocity));
    my_strings.push(format!("Facing Vector: {:?} \n", movement.facing_vector));
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
  mut query: Query<(&PlayerId, &mut CharacterMovement)>,
  mut transition_writer: EventWriter<AnimationTransitionEvent>,
) {
  for (player_id, mut movement) in query.iter_mut() {
    movement.tick();
    for buffer in player_data.buffers.iter_mut() {
      if buffer.player_id == *player_id {
        movement.update_action_state(buffer);
        // Consume Movement Events
        if let Some(me) = movement.movement_event {
          match me.event_type {
            MovementEventType::BACKDASH => movement.execute_backdash(),
            _ => ()
          }
          movement.clear_movement_event();
        }

        movement.execute_airdash();
        let new_velocity = match movement.get_action_state() {
          ActionState::Walking => Vec2::new(movement.walk_speed * movement.facing_vector, 0.0),
          ActionState::BackWalking => Vec2::new(-movement.back_walk_speed * movement.facing_vector, 0.0),
          ActionState::Dashing => Vec2::new(movement.dash_speed * movement.facing_vector,0.0),
          ActionState::Airborne => movement.velocity - (Vec2::Y * movement.gravity),
          ActionState::AirDashing {duration: _, velocity} => velocity,
          ActionState::AirBackDashing {duration: _, velocity} => velocity,
          ActionState::Standing => Vec2::ZERO,
          _ => movement.velocity.custom_lerp(Vec2::ZERO, 0.5),
        };
        movement.set_velocity(new_velocity);
        movement.execute_jump();
      }
    }
    if movement.get_should_transition() {
      if let Some(transition) =  movement.calculate_transition() {
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
