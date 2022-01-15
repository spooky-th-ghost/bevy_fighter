pub use crate::prelude::*;

/// Attack hitboxes
pub struct Hitbox {
  pub player_id: u8,
  attack_data: AttackData,
  property: AttackProperty,
  duration: u8,
  active: bool
}

impl Hitbox {
  pub fn new(player_id: u8, attack_data: AttackData, property: AttackProperty, duration: u8) -> Self {
    Hitbox {
      player_id,
      attack_data,
      property,
      duration,
      active: true,
    }
  }

  pub fn update(&mut self) {
    self.duration -= 1;
  }

  ///
  pub fn collide_with_hurtbox(&self,hurtbox: Hurtbox, mut hit_writer: EventWriter<HitEvent>) {
    if hurtbox.does_connect(&self.property) {
      hit_writer.send(HitEvent::new(self.attack_data, self.player_id, hurtbox.player_id));
    }
  }
}

/// All pertinent data for attacks
#[derive(Clone, Copy)]
pub struct AttackData {
  attack_level: u8,
  damage: u8,
  bonus_hitstun: u8,
  starting_proration: f32,
  mid_proration: f32,
  i_force: InterpolatedForce,
  hit_state: HitState,
  block_state: BlockState
}

/// Which part of the body does this attack hit?
#[derive(PartialEq)]
pub enum AttackProperty {
  HEAD,
  BODY,
  FOOT
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
  pub attack_data: AttackData,
  pub player_id: u8,
  pub recieving_player_id: u8,
}

impl HitEvent {
  pub fn new(attack_data: AttackData, player_id: u8, recieving_player_id: u8) -> Self {
    HitEvent {
      attack_data,
      player_id,
      recieving_player_id
    }
  }
}
