use bevy::prelude::*;
use bevy_fighter::prelude::*;


fn main() {
    let mut app = App::new();
  app
    .add_plugins(DefaultPlugins)
    .add_plugin(FighterPlugin);
  app
    .add_startup_system(setup.after(FighterSystemLabels::InitializeCharacterData))
    .add_system(set_camera_scale)
    .run();
}



fn setup(
    mut coms: Commands,
    character_library: Res<CharacterLibrary>,
) {
    coms
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraController {
            ..Default::default()
        });
    coms.spawn_bundle(UiCameraBundle::default());
    coms.spawn_bundle(
        FighterCharacterBundle::new(PlayerId::P1, "roa", &character_library)
    );
    coms.spawn_bundle(
        FighterCharacterBundle::new(PlayerId::P2,"aoko", &character_library)
    );
}
