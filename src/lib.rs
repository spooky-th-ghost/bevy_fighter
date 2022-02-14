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
mod state;

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
pub use crate::state::*;

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

// pub mod prelude {
//   pub use lazy_static;
//   pub use regex::Regex;
//   pub use serde::{Deserialize, Serialize};
//   pub use serde_json::{
//     from_str,
//     Result,
//     Value
//   };
//   pub use bevy::{
//     input::keyboard::KeyboardInput,
//     core::FixedTimestep,
//     ui::Val::*,
//     ecs::{
//       schedule::SystemLabel,
//       system::EntityCommands
//     },
//     diagnostic::{
//       Diagnostics,
//       FrameTimeDiagnosticsPlugin, 
//       LogDiagnosticsPlugin
//     },
//     prelude::*
//   };
//   pub use std::{
//     f32::*,
//     fmt::Write,
//     path::Path,
//     fs::read_to_string,
//     io::BufReader,
//     collections::{
//       HashMap,
//       hash_map::Iter
//     }
//   };
  
  // // local mods
  // pub use crate::inputs::*;
  // pub use crate::movement::*;
  // pub use crate::constants::*;
  // pub use crate::utils::*;
  // pub use crate::animation::*;
  // pub use crate::collision::*;
  // pub use crate::fighter_plugin::*;
  // pub use crate::character::*;
  // pub use crate::debug_ui::*;
  // pub use crate::attacks::*;
  // pub use crate::camera::*;
  // pub use crate::character_library::*;
  // pub use crate::state::*;
  // }
