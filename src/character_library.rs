use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use crate::animation::{
  Animation
};
use crate::attacks::{Hitbox, Attack, HitboxSerialized, AttackSerialized};
use crate::character::{
  CharacterMovement,
  CharacterMovementSerialized
};
use std::{
  path::Path,
  collections::{
    HashMap,
    hash_map::Iter
  },
  fs::read_to_string,
};
use serde_json::from_str;

#[derive(Deserialize, Serialize)]
pub struct CharacterSheetSerialized {
  pub animations: Vec<AnimationSerialized>,
  pub hitboxes: Vec<HitboxSerialized>,
  pub attacks: Vec<AttackSerialized>,
  pub movement: CharacterMovementSerialized,
  pub sprite_info: SpriteInfo
}

#[derive(Deserialize, Serialize)]
pub struct SpriteInfo {
  pub sprite_x: f32,
  pub sprite_y: f32,
  pub columns: usize,
  pub rows: usize,
}

#[derive(Deserialize, Serialize)]
pub struct AnimationSerialized {
  pub name: String,
  pub first_frame: usize,
  pub length: usize,
  pub loopable: bool,
  pub hold: u8,
}

#[derive(Debug, Clone)]
pub struct CharacterLibrary {
  animations: HashMap<String, Animation>,
  hitboxes: HashMap<String, Hitbox>,
  attacks: HashMap<String, Attack>,
  movements: HashMap<String, CharacterMovement>,
  atlases: HashMap<String, Handle<TextureAtlas>>,

}

impl CharacterLibrary {
  pub fn new() -> Self {
    let animations: HashMap<String, Animation> = HashMap::new();
    let hitboxes: HashMap<String, Hitbox> = HashMap::new();
    let attacks: HashMap<String, Attack> = HashMap::new();
    let movements: HashMap<String, CharacterMovement> = HashMap::new();
    let atlases: HashMap<String, Handle<TextureAtlas>> = HashMap::new();
    CharacterLibrary {
      animations,
      hitboxes,
      attacks,
      movements,
      atlases,
    }
  }

pub fn load_character_data(&mut self, character_name: &str, asset_server: &Res<AssetServer>, texture_atlases: &mut ResMut<Assets<TextureAtlas>>) {
  let raw_path = format!("./assets/character_data/{}.json", character_name);
  let path = Path::new(&raw_path[..]);
  if let Ok(raw_string) = read_to_string(path) {
    let raw_slice = &raw_string[..]; 
    let character_sheet: CharacterSheetSerialized = from_str(raw_slice).unwrap();
    let SpriteInfo {sprite_x, sprite_y, columns, rows} = character_sheet.sprite_info;

    let raw_texture_path = format!("sprites/{}.png", character_name);
    let texture_path = Path::new(&raw_texture_path[..]);
    let texture_handle = asset_server.load(texture_path);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(sprite_x,sprite_y), columns, rows);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    self.atlases.insert(
      character_name.to_string(),
      texture_atlas_handle
    );

    let mut raw_hitboxes: Vec<(String, Hitbox)> = Vec::new();
    let mut raw_anims: Vec<(String, Animation)> = Vec::new();
    for animation in character_sheet.animations {
      raw_anims.push(
        (
          format!("{}_{}",character_name,animation.name.clone()),
          Animation::from_serialized(animation),
        )
      );
    }

    for hitbox in character_sheet.hitboxes {
      raw_hitboxes.push(
        (
          format!("{}_{}",character_name,hitbox.name.clone()),
          Hitbox::from_serialized(hitbox),
        )
      );
    }

    self.add_animations(
      HashMap::from_iter::<HashMap<String, Animation>>(raw_anims.iter().cloned().collect())
    );

    self.add_hitboxes(
       HashMap::from_iter::<HashMap<String, Hitbox>>(raw_hitboxes.iter().cloned().collect())
    );

    let mut raw_attacks: Vec<(String, Attack)> = Vec::new();

    for attack in character_sheet.attacks {
      raw_attacks.push(
        (
          format!("{}_{}",character_name,attack.name.clone()),
          Attack::from_serialized(attack,&self,character_name)
        )
      )
    }

    self.add_attacks(
      HashMap::from_iter::<HashMap<String, Attack>>(raw_attacks.iter().cloned().collect())
    );

    let movement = CharacterMovement::from_serialized(
      character_sheet.movement,
      &self,
      character_name
    );

    self.movements.insert(
      character_name.to_string(),
      movement
    );
  }
}

  pub fn add_animations(&mut self, animations: HashMap<String, Animation>) {
    self.animations.extend(animations);
  }

  pub fn add_hitboxes(&mut self, hitboxes: HashMap<String, Hitbox>) {
    self.hitboxes.extend(hitboxes);
  }

  pub fn add_attacks(&mut self, attacks: HashMap<String, Attack>) {
    self.attacks.extend(attacks)
  }

  pub fn get_animation(&self, anim_id: String) -> Option<Animation> {
    if let Some(animation) = self.animations.get(&anim_id) {
      return Some(animation.clone());
    } else {
      return None;
    }
  }

  pub fn get_hitbox(&self, hitbox_id: String) -> Option<Hitbox> {
    if let Some(hitbox) = self.hitboxes.get(&hitbox_id) {
      return Some(hitbox.clone());
    } else {
      return None;
    }
  }

  pub fn get_movement(&self, movement_id: &str) -> Option<CharacterMovement> {
    if let Some(movement) = self.movements.get(movement_id) {
      return Some(movement.clone());
    } else {
      return None;
    }
  }

  pub fn get_atlas(&self, atlas_id: &str) -> Option<Handle<TextureAtlas>> {
    if let Some(atlas) = self.atlases.get(atlas_id) {
      return Some(atlas.clone());
    } else {
      return None;
    }
  }

  pub fn read_animations(&self) -> Iter<String, Animation> {
    self.animations.iter()
  }

  pub fn read_hitboxes(&self) -> Iter<String, Hitbox> {
    self.hitboxes.iter()
  }

  pub fn read_attacks(&self) -> Iter<String, Attack> {
    self.attacks.iter()
  }
}


pub fn initialize_character_library(
    asset_server: Res<AssetServer>,
    mut character_library: ResMut<CharacterLibrary>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>> 
) {
    character_library.load_character_data("roa", &asset_server, &mut texture_atlases);
    character_library.load_character_data("aoko", &asset_server, &mut texture_atlases);
}
