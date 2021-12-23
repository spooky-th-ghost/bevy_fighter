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
            .with_system(add_child_input.system().label("DISPLAY").after("WRITE"))
        )
        .run();
}

fn add_child_input(
  mut commands: Commands,
  arrow: Res<ArrowImage>,
  time: Res<Time>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut query: Query<Entity, With<InputDisplay>>,
) {
  for entity in query.iter_mut() {
      let child = commands.spawn_bundle(ImageBundle {
        style: Style {
          size: Size::new(Val::Px(50.0), Val::Auto),
          position_type: PositionType::Relative,
          flex_shrink: 0.0,
          ..Default::default()
        },
        material: materials
          .add(arrow.handle.clone().into()),
          ..Default::default()
      }
    )
    .insert(InputRow {
      motion: 5,
      duration: 0,
      birth: time.seconds_since_startup()
    }
    ).id();
  commands.entity(entity).push_children(&[child]);
  }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
  ) {
    let arrow_handle: Handle<Texture> = asset_server.load("arrow.png");
    let arrow_image = ArrowImage{handle: arrow_handle};
  
  commands
    .insert_resource(arrow_image);
  commands
    .spawn()
    .insert(InputBuffer::new(1));
  commands
    .spawn_bundle(UiCameraBundle::default());


    commands
      .spawn_bundle(NodeBundle {
          style: Style {
              padding: Rect{
                left: Px(30.0),
                right: Px(30.0),
                top: Px(30.0),
                bottom: Px(30.0)
              },
              position_type: PositionType::Absolute,
              flex_direction: FlexDirection::ColumnReverse,
              size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
              justify_content: JustifyContent::FlexStart,
              align_items: AlignItems::FlexStart,
              flex_shrink: 0.0,
              ..Default::default()
          },
          material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
          ..Default::default()
      })
      .insert(InputDisplay);
}

pub fn cleanup_input(
  mut commands: Commands,
  mut query: Query<&Children, With<InputDisplay>>,
) {
  for children in query.iter_mut() {
    if children.len() > 10 {
      if let Some(c) = children.first() {
        commands.entity(c).despawn();
      }
    }
  }
}

