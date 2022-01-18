#![allow(non_camel_case_types,clippy::needless_return, dead_code)]
#[macro_use]
pub extern crate lazy_static;

mod inputs;
mod movement;
mod constants;
mod player;
mod display;
mod utils;
mod animation;
mod collision;

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
  pub use crate::player::*;
  pub use crate::display::*;
  pub use crate::utils::*;
  pub use crate::animation::*;
  pub use crate::collision::*;
  }
