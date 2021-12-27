use bevy_fighter::prelude::*;

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(MotionInputPlugin)
    .add_startup_system(setup.system())
    .add_system(update_player_states.system())
    .add_system(apply_player_velocity.system())
    .run();
}

fn setup(
    mut coms: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    coms.spawn_bundle(OrthographicCameraBundle::new_2d());
    coms.spawn_bundle(UiCameraBundle::default());

    coms
      .spawn_bundle(SpriteBundle {
        sprite: Sprite::new(Vec2::new(30.0, 60.0)),
        material: materials.add(Color::RED.into()),
        transform: Transform::default(),
        ..Default::default()
      })
      .insert(InputBuffer::new(1))
      .insert(PlayerId(1))
      .insert(PlayerMovement::new());
}
