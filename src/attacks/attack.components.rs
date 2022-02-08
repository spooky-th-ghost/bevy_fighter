use crate::prelude::*;

pub struct Attack {
  name: String,
  hitbox_events: Vec<HitboxEvent>,
  hurtbox_events: Vec<HurtboxEvent>,
  startup: u8,
  recovery: u8,
}



pub struct HitboxEvent {
  pub hitbox: Hitbox,
  pub position: Vec2,
  pub size: Vec2,
  pub frame: u8,
}

pub struct HurtboxEvent {
  hurtbox_type: HurtboxType,
  active: bool,
  position: Vec2,
  size: Vec2,
}

pub enum HurtboxType {
  UPPER,
  LOWER
}
struct n_HitboxData {
  pub air_blockable: bool,
  pub property: AttackProperty,
  pub attack_level: u8,
  pub damage: u8,
  pub p1_proration: f32,
  pub p2_proration: f32,
  pub force: Vec2,
}

struct n_HitboxCollision {
  pub duration: u8,
  pub active: bool,
}
