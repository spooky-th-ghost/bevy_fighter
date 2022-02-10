pub use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct CharacterSheetSerialized {
  pub animations: Vec<AnimationSerialized>,
  pub hitboxes: Vec<HitboxSerialized>,
  pub attacks: Vec<AttackSerialized>,
  pub movement: CharacterMovementSerialized
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
  movements: HashMap<String, CharacterMovement>
}

impl CharacterLibrary {
  pub fn new() -> Self {
    let animations: HashMap<String, Animation> = HashMap::new();
    let hitboxes: HashMap<String, Hitbox> = HashMap::new();
    let attacks: HashMap<String, Attack> = HashMap::new();
    let movements: HashMap<String, CharacterMovement> = HashMap::new();
    CharacterLibrary {
      animations,
      hitboxes,
      attacks,
      movements
    }
  }

  pub fn load_character_data(&mut self, character_name: &str) {
  let raw_path = format!("./assets/character_data/{}.json", character_name);
  let path = Path::new(&raw_path[..]);
  if let Ok(raw_string) = read_to_string(path) {
    let raw_slice = &raw_string[..]; 
    let character_sheet: CharacterSheetSerialized = from_str(raw_slice).unwrap();

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
