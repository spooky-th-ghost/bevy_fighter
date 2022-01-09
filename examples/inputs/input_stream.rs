use bevy_fighter::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<MotionEvent>()
        .add_startup_system(setup)
        .add_system_set(
          SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.01667))
            .with_system(write_motion_inputs.label("WRITE"))
            .with_system(read_motion_inputs.after("WRITE"))
        )
        .run();
}


fn setup(
    mut commands: Commands,
  ) {
  
  commands
    .spawn()
    .insert(InputBuffer::new(1));
  commands
    .spawn_bundle(UiCameraBundle::default());
}

