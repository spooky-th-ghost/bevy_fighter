#![allow(non_camel_case_types,clippy::needless_return, dead_code)]
#[macro_use]
pub extern crate lazy_static;

mod inputs;
mod attacks;
mod movement;
pub mod constants;
pub mod utils;
mod animation;
mod collision;
mod character;
mod debug_ui;
mod camera;
mod character_library;

pub use bevy::{
  prelude::*,
   diagnostic::{
      Diagnostics,
      FrameTimeDiagnosticsPlugin, 
      LogDiagnosticsPlugin
    },
  core::FixedTimestep,
};

pub use crate::inputs::*;
pub use crate::movement::*;
pub use crate::constants::*;
pub use crate::utils::*;
pub use crate::animation::*;
pub use crate::collision::*;
pub use crate::character::*;
pub use crate::debug_ui::*;
pub use crate::attacks::*;
pub use crate::camera::*;
pub use crate::character_library::*;

/// Core plugin, handles deserializing data, collision, animation, and physics
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
      .add_stage("main",SystemStage::single_threaded()
        .with_run_criteria(FixedTimestep::steps_per_second(60.0))
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
          manage_hitboxes
            .label(FighterSystemLabels::HitboxUpdate)
            .after(FighterSystemLabels::AnimationUpdate)
        )
        .with_system(
          spawn_hitboxes
            .label(FighterSystemLabels::HitboxCreation)
            .after(FighterSystemLabels::HitboxUpdate)
        )
        .with_system(
          update_debug_ui
            .after(FighterSystemLabels::PhysicsExecute)
        )
      );
  }
}

pub mod prelude {
  pub use crate::{
    FighterPlugin,
    character::{
      PlayerId,
      SpawnPlayer
    },
    utils::FighterSystemLabels,
    camera::{
      CameraController,
      set_camera_scale
    },
    debug_ui::SpawnDebugUi,
    character_library::CharacterLibrary,
  };
}
