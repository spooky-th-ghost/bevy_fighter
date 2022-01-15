pub use crate::prelude::*;

/// Attack hitboxes
pub struct Hitbox {
  pub player_id: u8,
  pub air_blockable: bool,
  pub property: AttackProperty,
  hitbox_data: HitboxData,
  duration: u8,
  active: bool
}

impl Hitbox {
  pub fn new(player_id: u8, air_blockable: bool, hitbox_data: HitboxData, property: AttackProperty, duration: u8) -> Self {
    Hitbox {
      player_id,
      air_blockable,
      hitbox_data,
      property,
      duration,
      active: true,
    }
  }

  pub fn update(&mut self) {
    self.duration -= 1;
  }

  ///
  pub fn collide_with_hurtbox(&self,hurtbox: &Hurtbox, mut hit_writer: EventWriter<HitEvent>, mut block_writer: EventWriter<BlockEvent>) {
    if self.player_id != hurtbox.player_id {
      if self.does_connect(hurtbox) {
        if self.is_blocked(hurtbox) {
          block_writer.send(BlockEvent::new(self.hitbox_data, self.player_id, hurtbox.player_id))
        } else {
          hit_writer.send(HitEvent::new(self.hitbox_data, self.player_id, hurtbox.player_id));
        }
      }
    }
  }

  pub fn does_connect(&self, hurtbox: &Hurtbox) -> bool {
    return !hurtbox.ignores(&self.property);
  }

  pub fn is_blocked(&self, hurtbox: &Hurtbox) -> bool {
    if let Some(block_type) = hurtbox.block_type {
      if hurtbox.is_grounded {
        match self.property {
          AttackProperty::MID => {return true},
          AttackProperty::HIGH => {
            return block_type == BlockType::HIGH;
          },
          AttackProperty::LOW => {
            return block_type == BlockType::LOW;
          }
        }
      } else {
        if self.air_blockable {
          return true;
        } else {
          if let Some(modifier) = hurtbox.block_modifier {
            match modifier {
              _ => {return true}
            }
          } else {
            return false;
          }
        }
      }
    } else {
      return false;
    }
  }
}

/// All pertinent data for attacks
#[derive(Clone, Copy)]
pub struct HitboxData {
  attack_level: u8,
  damage: u8,
  bonus_hitstun: u8,
  starting_proration: f32,
  mid_proration: f32,
  i_force: InterpolatedForce,
  hit_state: HitState,
  block_state: BlockState,
}

/// Which part of the body does this attack hit?
#[derive(PartialEq, Clone, Copy)]
pub enum AttackProperty {
  HIGH,
  MID,
  LOW
}

#[derive(PartialEq, Clone, Copy)]
pub enum HitState {
  STANDING,
  FLOATING,
  TUMBLING,

}

#[derive(PartialEq, Clone, Copy)]
pub enum BlockState {
  BLOCKING,
  BUCKLED,
  BROKEN,
}

pub struct HitEvent{
  pub hitbox_data: HitboxData,
  pub player_id: u8,
  pub recieving_player_id: u8,
}

impl HitEvent {
  pub fn new(hitbox_data: HitboxData, player_id: u8, recieving_player_id: u8) -> Self {
    HitEvent {
      hitbox_data,
      player_id,
      recieving_player_id
    }
  }
}

pub struct BlockEvent{
  pub hitbox_data: HitboxData,
  pub player_id: u8,
  pub recieving_player_id: u8,
}

impl BlockEvent {
  pub fn new(hitbox_data: HitboxData, player_id: u8, recieving_player_id: u8) -> Self {
    BlockEvent {
      hitbox_data,
      player_id,
      recieving_player_id
    }
  }
}


