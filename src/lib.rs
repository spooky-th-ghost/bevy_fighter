#![allow(non_camel_case_types,clippy::needless_return, dead_code)]
#[macro_use]
extern crate lazy_static;

/// Parsing inputs, mapping input devices
pub mod inputs;
pub mod attacks;
pub mod movement;
pub mod constants;
pub mod utils;
pub mod animation;
pub mod collision;
pub mod character;
pub mod debug_ui;
pub mod camera;
pub mod character_library;

use bevy::{
  prelude::*,
   diagnostic::{
      FrameTimeDiagnosticsPlugin, 
    },
  core::FixedTimestep,
};

use crate::inputs::*;
use crate::movement::*;

use crate::utils::*;
use crate::animation::*;

use crate::character::*;

use crate::attacks::*;

use crate::character_library::*;

/// Core plugin, handles deserializing data, collision, animation, and physics
pub struct FighterPlugin;

impl Plugin for FighterPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<FighterInputEvent>()
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
          manage_character_state
          .label(FighterSystemLabels::StatusUpdate)
          .after(FighterSystemLabels::InputRead)
        )
        .with_system(
          manage_character_velocity
          .label(FighterSystemLabels::PhysicsUpdate)
          .after(FighterSystemLabels::StatusUpdate)
        )
        .with_system(
          apply_character_velocity
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
      FighterCharacterBundle
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
