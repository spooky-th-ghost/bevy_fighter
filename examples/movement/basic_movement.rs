use bevy_fighter::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(FighterInputPlugin)
    .insert_resource(CollisionBoxColors::new(0.4))
    .add_startup_system(setup)
    //.add_system(read_gamepad_inputs)
    .add_system_set(
      SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.01667))
        .with_system(update_player_states)
        .with_system(apply_player_velocity)
        .with_system(add_hitbox)
        .with_system(manage_hitboxes)
    )
    .run();
}

fn manage_hitboxes(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Hitbox)>
) {
  for (e,mut hb) in query.iter_mut() {
    hb.update();
    if !hb.active {
      commands.entity(e).despawn();
    }
  }
}

fn add_hitbox(
  mut commands: Commands,
  box_colors: Res<CollisionBoxColors>, 
  keyboard_input: Res<Input<KeyCode>>,
  button_input: Res<Input<GamepadButton>>,
  player_inputs: Res<PlayerInputs>,
  query: Query<(&PlayerId, &PlayerMovement, Entity)>,
) -> () {
  for (player_id, player_movement, entity) in query.iter() {
    for mapper in player_inputs.local_devices.iter() {
      if mapper.player_id == player_id.0 {
        let InputActionsPressed {a, ..} = mapper.get_just_pressed_buttons(&keyboard_input, &button_input);

        if a {
          spawn_hitbox(
            &mut commands,
            box_colors.hitbox_color,
            entity,
            player_id.0,
            Vec2::new(40.0,20.0),
            Vec2::new(15.0*player_movement.get_facing_vector(), 25.0),
            Hitbox::new(
              player_id.0,
              false,
              HitboxData::jab(5) ,
              AttackProperty::MID,
              50
            )
          );
        }
      }
    }
  }
}


fn read_gamepad_inputs(mut gamepad_event: EventReader<GamepadEvent>) {
    for event in gamepad_event.iter() {
        match &event {
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                info!("{:?} Connected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                info!("{:?} Disconnected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::ButtonChanged(button_type, value)) => {
                info!("Button {:?} of {:?} is changed to {}", button_type, gamepad, value);
            }
            GamepadEvent(gamepad, GamepadEventType::AxisChanged(axis_type, value)) => {
                info!("Axis {:?} of {:?} is changed to {}", axis_type, gamepad, value);
            }
        }
    }
}
fn setup(
    mut coms: Commands,
    box_colors: Res<CollisionBoxColors>,
) {
    load_character_sprite_data("./src/test.json");
    coms.spawn_bundle(OrthographicCameraBundle::new_2d());
    coms.spawn_bundle(UiCameraBundle::default());

    let player = coms
      .spawn_bundle(SpriteBundle {
        sprite: Sprite{
          color: Color::RED,
          custom_size: Some(Vec2::new(30.0, 60.0)),
          ..Default::default()
        },
        transform: Transform::default(),
        ..Default::default()
      })
      .insert(PlayerMovement::new())
      .id();

    let hurtbox = coms
      .spawn_bundle(SpriteBundle {
        sprite: Sprite{
          color: box_colors.hurtbox_color,
          custom_size: Some(Vec2::new(35.0, 35.0)),
          ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -12.5, 1.0),
        ..Default::default()
      })
      .insert(Hurtbox {
        player_id: 1,
        ..Default::default()
      })
      .id();
    
    coms.entity(player).push_children(&[hurtbox]);
}
