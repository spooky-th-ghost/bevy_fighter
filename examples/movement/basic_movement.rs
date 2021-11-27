use bevy_fighter::prelude::*;

fn main() {
  App::build()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup.system())
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
        sprite: Sprite::new(Vec2::new(600.0, 20.0)),
        material: materials.add(Color::BLACK.into()),
        transform: Transform::from_xyz(-150.0, -100.0, 0.0),
        ..Default::default()
      });
}
