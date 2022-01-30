use crate::prelude::*;
pub struct FighterPlugin;

// impl Plugin for FighterPlugin {
//   fn build(&self, app: &mut App) {
//     app
//       .add_event::<FighterInputEvent>()
//       .insert_resource(PlayerData::default())
//       .insert_resource(CollisionBoxColors::new(0.4))
//       .add_stage("main",SystemStage::single_threaded()
//         .with_run_criteria(FixedTimestep::step(0.01667))
//         .with_system(write_fighter_inputs)
//         .with_system(read_fighter_inputs)
//         .with_system(update_player_states)
//         .with_system(apply_player_velocity)
//       );
//   }
// }


impl Plugin for FighterPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<FighterInputEvent>()
      .add_event::<CharacterMovementEvent>()
      .insert_resource(PlayerData::default())
      .insert_resource(CollisionBoxColors::new(0.4))
      .add_stage("main",SystemStage::single_threaded()
        .with_run_criteria(FixedTimestep::step(0.01667))
        .with_system(
          write_fighter_inputs
            .label(FighterSystemLabels::InputWrite)
        )
        .with_system(
          read_fighter_inputs
            .label(FighterSystemLabels::InputRead)
            .after(FighterSystemLabels::InputWrite)
        )
        .with_system(
          update_player_status
          .label(FighterSystemLabels::StatusUpdate)
          .after(FighterSystemLabels::InputRead)
        )
        .with_system(
          update_player_physics
          .label(FighterSystemLabels::PhysicsUpdate)
          .after(FighterSystemLabels::StatusUpdate)
        )
        .with_system(
          execute_player_physics
            .label(FighterSystemLabels::PhysicsExecute)
            .after(FighterSystemLabels::PhysicsUpdate)
        )
        .with_system(
          update_debug_ui
            .after(FighterSystemLabels::PhysicsExecute)
        )
      );
  }
}
