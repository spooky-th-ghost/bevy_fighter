use bevy_fighter::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_event::<MotionEvent>()
        .add_startup_system(setup.system())
        .add_system_set(
          SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.01667))
            .with_system(write_motion_inputs.system().label("WRITE"))
            .with_system(read_motion_inputs.system().after("WRITE"))
        )
        .run();
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
  ) {
  commands
    .spawn()
    .insert(InputBuffer::new(1));
  commands
    .spawn_bundle(UiCameraBundle::default());

    commands
      .spawn_bundle(NodeBundle {
          style: Style {
              position_type: PositionType::Absolute,
              size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::FlexEnd,
              ..Default::default()
          },
          material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
          ..Default::default()
      })
      .with_children(|parent| {
        parent
          .spawn_bundle(ImageBundle {
            style: Style {
              size: Size::new(Val::Px(50.0), Val::Auto),
              position_type: PositionType::Relative,
              position: Rect {
                left: Val::Px(80.0),
                bottom: Val::Px(80.0),
                ..Default::default()
              },
              ..Default::default()
            },
            material: materials
              .add(asset_server.load("arrow.png").into()),
              ..Default::default()
          });
      });
}

