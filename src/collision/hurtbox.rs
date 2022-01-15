pub use crate::prelude::*;


/// Player hurtboxes
pub struct Hurtbox {
  pub player_id: u8,
  pub is_grounded: bool,
  pub block_type: Option<BlockType>,
  pub block_modifier: Option<BlockModifier>,
  ignored_properties: Vec<AttackProperty>,
}

impl Hurtbox {
  pub fn does_connect(&self, attack_property: &AttackProperty) -> bool {
    return !self.ignored_properties.contains(attack_property);
  }
}

#[derive(PartialEq, Clone, Copy)]
pub enum BlockType {
  HIGH,
  LOW,
}

#[derive(PartialEq, Clone, Copy)]
pub enum BlockModifier {
  BARRIER,
  INSTANT,
}



