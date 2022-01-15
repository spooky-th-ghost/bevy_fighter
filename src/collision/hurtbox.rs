pub use crate::prelude::*;

/// Player hurtboxes
pub struct Hurtbox {
  pub player_id: u8,
  ignored_properties: Vec<AttackProperty>,
}

impl Hurtbox {
  pub fn does_connect(&self, attack_property: &AttackProperty) -> bool {
    return !self.ignored_properties.contains(attack_property);
  }
}
