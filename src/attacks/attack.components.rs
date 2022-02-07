use crate::prelude::*;

pub struct Attack {
  animation_id: String,
  hitbox_events: Vec<HitboxEvent>,
  startup: u8,
  recovery: u8,
}

pub struct HitboxEvent {
  pub hitbox: Hitbox,
  pub position: Vec2,
  pub size: Vec2,
  pub frame: u8,
}
