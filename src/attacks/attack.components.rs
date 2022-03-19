use serde::{Deserialize, Serialize};
use crate::{
  character_library::CharacterLibrary,
  collision::{
    HitboxEvent,
    HitboxEventSerialized
  }
};


/// Attacks that can be done by a player
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Clone, Default)]
pub struct Attack {
  /// The motion required to perform the attack
  pub name: String,
  /// moments throughout the attack when hitboxes need to be generated
  pub hitbox_events: Vec<HitboxEvent>,
  /// how long the attack will take to complete
  pub busy: u8,
}
impl Attack {
  /// Create an attack from it's serialized counterpart
  pub fn from_serialized(s: AttackSerialized, library: &CharacterLibrary, character_name: &str) -> Self {
    let mut hitbox_events: Vec<HitboxEvent> = Vec::new();

    for s_he in s.hitbox_events {
      hitbox_events.push(HitboxEvent::from_serialized(s_he, library, character_name));
    }
    Attack {
      name: s.name,
      hitbox_events,
      busy: s.busy,
    }
  }
}

/// Serialized version of an attack
#[derive(Deserialize, Serialize)]
pub struct AttackSerialized {
  pub name: String,
  pub hitbox_events: Vec<HitboxEventSerialized>,
  pub busy: u8,
}
