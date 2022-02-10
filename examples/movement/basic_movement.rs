use bevy_fighter::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(FighterPlugin)
    .add_startup_system(setup.after(FighterSystemLabels::InitializeCharacterData))
    .add_system(set_camera_scale)
    .run();
}



fn setup(
    mut coms: Commands,
    asset_server: Res<AssetServer>,
    character_library: Res<CharacterLibrary>,
) {
    coms
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraController {
            ..Default::default()
        });
    coms.spawn_bundle(UiCameraBundle::default());
    coms.spawn_player(PlayerId::P1, "roa", &character_library);
    coms.spawn_player(PlayerId::P2,"aoko", &character_library);
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
