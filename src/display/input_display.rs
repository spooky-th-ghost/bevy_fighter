use crate::prelude::*;

#[derive(Default, Component)]
pub struct InputRow {
  pub motion: u8,
  pub duration: u8,
  pub birth: f64,
}

pub struct InputDisplay;
