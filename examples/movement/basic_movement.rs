use bevy_fighter::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(FighterPlugin)
    .add_startup_system(setup)
    .add_system(set_camera_scale)
    .run();
}

fn setup(
    mut coms: Commands,
    asset_server: Res<AssetServer>,
    mut character_library: ResMut<CharacterLibrary>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>> 
) {
    let roa_texture_handle = asset_server.load("sprites/roa.png");
    let roa_texture_atlas = TextureAtlas::from_grid(roa_texture_handle, Vec2::new(256.0,256.0), 12, 16);
    let roa_texture_atlas_handle = texture_atlases.add(roa_texture_atlas);

    let aoko_texture_handle = asset_server.load("sprites/aoko.png");
    let aoko_texture_atlas = TextureAtlas::from_grid(aoko_texture_handle, Vec2::new(256.0,256.0), 12, 28);
    let aoko_texture_atlas_handle = texture_atlases.add(aoko_texture_atlas);

    character_library.load_character_data("roa");
    character_library.load_character_data("aoko");
    println!("{:?}", character_library.attacks);
    coms
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraController {
            ..Default::default()
        });
    coms.spawn_bundle(UiCameraBundle::default());
    coms.spawn_player(PlayerId::P1, "roa", &character_library, roa_texture_atlas_handle.clone());
    coms.spawn_player(PlayerId::P2,"aoko", &character_library, aoko_texture_atlas_handle.clone());
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
