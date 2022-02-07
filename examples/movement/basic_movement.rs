use bevy_fighter::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(FighterPlugin)
    // .add_plugin(LogDiagnosticsPlugin::default())
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(setup)
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

// fn add_hitbox(
//   mut commands: Commands,
//   box_colors: Res<CollisionBoxColors>, 
//   keyboard_input: Res<Input<KeyCode>>,
//   button_input: Res<Input<GamepadButton>>,
//   player_data: Res<PlayerData>,
//   query: Query<(&PlayerMovement, Entity)>,
// ) -> () {
//   for (player_movement, entity) in query.iter() {
//     for mapper in player_data.local_devices.iter() {
//       if mapper.player_id == player_movement.player_id {
//         let InputActionsPressed {a, ..} = mapper.get_just_pressed_buttons(&keyboard_input, &button_input);

//         if a {
//           spawn_hitbox(
//             &mut commands,
//             box_colors.hitbox_color,
//             entity,
//             player_movement.player_id,
//             Vec2::new(40.0,20.0),
//             Vec2::new(15.0*player_movement.facing_vector, 25.0),
//             Hitbox::new(
//               player_movement.player_id,
//               false,
//               HitboxData::jab(5) ,
//               AttackProperty::MID,
//               50
//             )
//           );
//         }
//       }
//     }
//   }
// }

fn setup(
    mut coms: Commands,
    asset_server: Res<AssetServer>,
    mut animation_library: ResMut<AnimationLibrary>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>> 
) {
    let roa_texture_handle = asset_server.load("sprites/roa.png");
    let roa_texture_atlas = TextureAtlas::from_grid(roa_texture_handle, Vec2::new(256.0,256.0), 12, 16);
    let roa_texture_atlas_handle = texture_atlases.add(roa_texture_atlas);

    let aoko_texture_handle = asset_server.load("sprites/aoko.png");
    let aoko_texture_atlas = TextureAtlas::from_grid(aoko_texture_handle, Vec2::new(256.0,256.0), 12, 28);
    let aoko_texture_atlas_handle = texture_atlases.add(aoko_texture_atlas);

    animation_library.load_character_sprite_data("roa","./src/roa.json");
    animation_library.load_character_sprite_data("aoko","./src/aoko.json");
    coms.spawn_bundle(OrthographicCameraBundle::new_2d());
    coms.spawn_bundle(UiCameraBundle::default());
    coms.spawn_player(PlayerId::P1, "roa", &animation_library, roa_texture_atlas_handle.clone());
    coms.spawn_player(PlayerId::P2,"aoko", &animation_library, aoko_texture_atlas_handle.clone());
    coms.spawn_debug_ui(PlayerId::P1, &asset_server.load("fonts/Roboto-Black.ttf"));
    coms.spawn_debug_ui(PlayerId::P2, &asset_server.load("fonts/Roboto-Black.ttf"));

    // let hurtbox = coms
    //   .spawn_bundle(SpriteBundle {
    //     sprite: Sprite{
    //       color: box_colors.hurtbox_color,
    //       custom_size: Some(Vec2::new(35.0, 35.0)),
    //       ..Default::default()
    //     },
    //     transform: Transform::from_xyz(0.0, -12.5, 1.0),
    //     ..Default::default()
    //   })
    //   .insert(Hurtbox {
    //     player_id: 1,
    //     ..Default::default()
    //   })
    //   .id();
    
    // coms.entity(player).push_children(&[hurtbox]);
}
