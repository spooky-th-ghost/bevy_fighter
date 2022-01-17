use bevy_fighter::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FighterInputPlugin)
        .add_startup_system(setup)
        .run();
}


fn setup(
    mut commands: Commands,
  ) {
  
  commands
    .spawn_bundle(UiCameraBundle::default());
}

