#![allow(non_camel_case_types,clippy::needless_return, dead_code)]
#[macro_use]
pub extern crate lazy_static;

mod inputs;
mod movement;
mod constants;
mod display;
mod utils;
mod animation;
mod collision;
mod fighter_plugin;
mod player_systems;
mod player_data;
mod debug_ui;

pub mod prelude {
  // external crates
  pub use lazy_static;
  pub use regex::Regex;
  pub use serde_json::{
    from_str,
    Result,
    Value
  };
  pub use bevy::{
    input::keyboard::KeyboardInput,
    core::FixedTimestep,
    ui::Val::*,
    utils::HashMap,
    ecs::schedule::SystemLabel,
    diagnostic::{
      FrameTimeDiagnosticsPlugin, 
      LogDiagnosticsPlugin
    },
    prelude::*
  };
  pub use std::{
    fmt::Write,
    path::Path,
    fs::read_to_string,
    io::BufReader,
  };
  
  // local mods
  pub use crate::inputs::*;
  pub use crate::movement::*;
  pub use crate::constants::*;
  pub use crate::display::*;
  pub use crate::utils::*;
  pub use crate::animation::*;
  pub use crate::collision::*;
  pub use crate::fighter_plugin::*;
  pub use crate::player_systems::*;
  pub use crate::player_data::*;
  pub use crate::debug_ui::*;
  }
