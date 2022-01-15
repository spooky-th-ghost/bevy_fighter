use bevy_fighter::prelude::*;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(MotionInputPlugin)
    .add_startup_system(setup)
    .add_system(update_player_states)
    .add_system(apply_player_velocity)
    .run();
}

fn setup(
    mut coms: Commands,
) {
    load_character_sprite_data("./src/test.json");
    coms.spawn_bundle(OrthographicCameraBundle::new_2d());
    coms.spawn_bundle(UiCameraBundle::default());

    coms
      .spawn_bundle(SpriteBundle {
        sprite: Sprite{
          color: Color::RED,
          custom_size: Some(Vec2::new(30.0, 60.0)),
          ..Default::default()
        },
        transform: Transform::default(),
        ..Default::default()
      })
      .insert(InputBuffer::new(1))
      .insert(PlayerId(1))
      .insert(PlayerMovement::new());
}
