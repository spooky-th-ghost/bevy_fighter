use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Attack {
  pub name: String,
  pub hitbox_events: Vec<HitboxEvent>,
  pub busy: u8,
}
impl Attack {
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
#[derive(Debug, Clone, Copy)]
pub struct HitboxEvent {
  pub hitbox: Hitbox,
  pub position: Vec2,
  pub size: Vec2,
  pub frame: u8,
}



#[derive(Deserialize, Serialize)]
pub struct Vec2Serialzed {
  pub x: f32,
  pub y: f32,
}

impl Vec2Serialzed {
  pub fn to_vec2(&self) -> Vec2 {
    Vec2::new(self.x,self.y)
  }
}

impl HitboxEvent {
  pub fn from_serialized(s: HitboxEventSerialized, library: &CharacterLibrary, character_name: &str) -> Self {
    let hitbox = library.get_hitbox(format!("{}_{}",character_name,s.hitbox.clone())).unwrap();
    HitboxEvent {
      hitbox,
      position: s.position.to_vec2(),
      size: s.size.to_vec2(),
      frame: s.frame
    }
  }
}

pub struct HurtboxEvent {
  hurtbox_type: HurtboxType,
  /// Is the hurtbox collidable
  active: bool,
  /// Where should the hurtbox be placed
  position: Vec2,
  /// What size should the hurtbox change to
  size: Vec2,
}

#[derive(Component,Debug, Clone, Copy)]
pub struct Hitbox {
  /// Attack level, effects hit/block stun
  attack_level: u8,
  /// Base damage of the hitbox
  damage: u8,
  /// Proration when this hitbox connects first in a combo
  proration: f32,
  /// Force to be applied when a player is hit by this
  force: Vec2,
  /// Has the hitbox connected
  hit_state: HitState,
  /// Can the hitbox be blocked in the air
  air_blockable: bool,
  /// The block property of the hitbox
  property: HitboxProperty,
  /// How many frames will this hitbox stay out
  duration: u8,
  /// Does this hitbox cause damage when blocked
  chip: bool,
  /// Is the hitbox currently active
  active: bool,
  /// Is the hitbox attached to the player that generated it
  projectile: bool,
}

impl Hitbox {
  pub fn from_serialized(s: HitboxSerialized) -> Self {
    Hitbox {
      attack_level: s.attack_level,
      damage: s.damage,
      proration: s.proration,
      force: s.force.to_vec2(),
      air_blockable: s.air_blockable,
      property: s.property,
      duration: s.duration,
      chip: s.chip,
      projectile: s.projectile,
      hit_state: HitState::None,
      active: false
    }
  }

  pub fn tick(&mut self) {
    self.duration = countdown(self.duration);
  }

  pub fn is_finished(&self) -> bool {
    return self.duration == 0;
  }
}

#[derive(Debug, Clone, Copy)]
pub enum HitState {
  None,
  Hit,
  Blocked
}

#[derive(Debug, Clone, Copy)]
pub enum HurtboxType {
  Upper,
  Lower,
  Limb
}

/// How the attack must be blocked
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum HitboxProperty {
  Mid,
  Low,
  High,
}


#[derive(Deserialize, Serialize)]
pub struct AttackSerialized {
  pub name: String,
  pub hitbox_events: Vec<HitboxEventSerialized>,
  pub busy: u8,
}

#[derive(Deserialize, Serialize)]
pub struct HitboxEventSerialized {
  pub hitbox: String,
  pub position: Vec2Serialzed,
  pub size: Vec2Serialzed,
  pub frame: u8,
}

#[derive(Deserialize, Serialize)]
pub struct HitboxSerialized {
  pub name: String,
  pub attack_level: u8,
  pub damage: u8,
  pub proration: f32,
  pub force: Vec2Serialzed,
  pub air_blockable: bool,
  pub property: HitboxProperty,
  pub duration: u8,
  pub chip: bool,
  pub projectile: bool,
}


pub trait SpawnHitbox {
  fn spawn_hitbox(&mut self, player_id: &PlayerId, hitbox_event: &HitboxEvent, parent_transform: &Transform, facing_vector: f32);
}

impl SpawnHitbox for Commands<'_, '_>{
  fn spawn_hitbox(&mut self, player_id: &PlayerId, hitbox_event: &HitboxEvent, parent_transform: &Transform, facing_vector: f32 ) {
    let offset = Vec3::new(hitbox_event.position.x * facing_vector, hitbox_event.position.y, 1.0);
    let parent_translation = parent_transform.translation;
    let transform = Transform::from_translation(parent_translation + offset);

    self.spawn_bundle( SpriteBundle {
      sprite: Sprite {
        color: Color::rgb(0.25, 0.25, 0.75),
        custom_size: Some(hitbox_event.size),
        ..Default::default()
      },
      transform,
      ..Default::default()
      }
    )
    .insert(player_id.clone())
    .insert(hitbox_event.hitbox);
  }
}
