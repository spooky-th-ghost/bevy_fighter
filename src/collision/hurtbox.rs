// TODO: Re-design once attacks are working

// pub use crate::prelude::*;


// /// Player hurtboxes
// #[derive(Component)]
// pub struct Hurtbox {
//   pub player_id: u8,
//   pub is_grounded: bool,
//   pub block_type: Option<BlockType>,
//   pub block_modifier: Option<BlockModifier>,
//   pub ignored_properties: Vec<AttackProperty>,
// }

// impl Default for Hurtbox {
//   fn default() -> Self {
//       Hurtbox {
//         player_id: 0,
//         is_grounded: true,
//         block_type: None,
//         block_modifier: None,
//         ignored_properties: Vec::new()
//       }
//   }
// }

// impl Hurtbox {
//   pub fn ignores(&self, attack_property: &AttackProperty) -> bool {
//      return !self.ignored_properties.contains(attack_property);
//   }
// }

// #[derive(PartialEq, Clone, Copy)]
// pub enum BlockType {
//   HIGH,
//   LOW,
// }

// #[derive(PartialEq, Clone, Copy)]
// pub enum BlockModifier {
//   BARRIER,
//   INSTANT,
// }



