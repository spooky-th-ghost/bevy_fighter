#![allow(non_camel_case_types,clippy::needless_return, dead_code)]
#[macro_use]
pub extern crate lazy_static;

mod inputs;
mod attacks;
mod movement;
mod constants;
mod utils;
mod animation;
mod collision;
mod fighter_plugin;
mod character;
mod debug_ui;
mod camera;
mod character_library;

pub mod prelude {
  pub use lazy_static;
  pub use regex::Regex;
  pub use serde::{Deserialize, Serialize};
  pub use serde_json::{
    from_str,
    Result,
    Value
  };
  pub use bevy::{
    input::keyboard::KeyboardInput,
    core::FixedTimestep,
    ui::Val::*,
    ecs::{
      schedule::SystemLabel,
      system::EntityCommands
    },
    diagnostic::{
      Diagnostics,
      FrameTimeDiagnosticsPlugin, 
      LogDiagnosticsPlugin
    },
    prelude::*
  };
  pub use std::{
    f32::*,
    fmt::Write,
    path::Path,
    fs::read_to_string,
    io::BufReader,
    collections::{
      HashMap,
      hash_map::Iter
    }
  };
  
  // local mods
  pub use crate::inputs::*;
  pub use crate::movement::*;
  pub use crate::constants::*;
  pub use crate::utils::*;
  pub use crate::animation::*;
  pub use crate::collision::*;
  pub use crate::fighter_plugin::*;
  pub use crate::character::*;
  pub use crate::debug_ui::*;
  pub use crate::attacks::*;
  pub use crate::camera::*;
  pub use crate::character_library::*;
  }
