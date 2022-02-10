use crate::prelude::*;
pub struct FighterPlugin;

impl Plugin for FighterPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<FighterInputEvent>()
      .add_event::<CharacterMovementEvent>()
      .add_event::<AnimationTransitionEvent>()
      .add_plugin(FrameTimeDiagnosticsPlugin)
      .insert_resource(CharacterLibrary::new())
      .insert_resource(PlayerData::default())
      .add_startup_system(initialize_character_library.label(FighterSystemLabels::InitializeCharacterData))
      //.insert_resource(CollisionBoxColors::new(0.4))
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
          determine_player_velocity_and_state
          .label(FighterSystemLabels::PhysicsUpdate)
          .after(FighterSystemLabels::InputRead)
        )
        .with_system(
          execute_player_physics
            .label(FighterSystemLabels::PhysicsExecute)
            .after(FighterSystemLabels::PhysicsUpdate)
        )
        .with_system(
          read_animation_transitions
            .label(FighterSystemLabels::AnimationUpdate)
            .after(FighterSystemLabels::PhysicsExecute)
        )
        .with_system(
          animate_sprite_system
            .label(FighterSystemLabels::AnimationExecute)
            .after(FighterSystemLabels::AnimationUpdate)
        )
        .with_system(
          update_debug_ui
            .after(FighterSystemLabels::PhysicsExecute)
        )
      );
  }
}
