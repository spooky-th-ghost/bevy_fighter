#![allow(non_camel_case_types,clippy::needless_return, dead_code)]
#[macro_use]
pub extern crate lazy_static;

mod inputs;
mod movement;
mod constants;

pub mod prelude {
  // external crates
  pub use lazy_static;
  pub use regex::Regex;
  pub use bevy::{
    input::keyboard::KeyboardInput,
    core::FixedTimestep,
    prelude::*
  };
  pub use std::fmt::Write;
  // local mods
  pub use crate::inputs::*;
  pub use crate::movement::*;
  pub use crate::constants::*;
  }
