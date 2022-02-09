pub use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct CharacterSheetSerialized {
  pub animations: Vec<AnimationSerialized>,
  pub hitboxes: Vec<HitboxSerialized>,
  pub attacks: Vec<AttackSerialized>
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
  pub attacks: HashMap<String, Attack>,
}

impl CharacterLibrary {
  pub fn new() -> Self {
    let animations: HashMap<String, Animation> = HashMap::new();
    let hitboxes: HashMap<String, Hitbox> = HashMap::new();
    let attacks: HashMap<String, Attack> = HashMap::new();
    CharacterLibrary {
      animations,
      hitboxes,
      attacks
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
}

#[derive(Clone,Copy, PartialEq, Debug)]
pub struct Animation {
  pub first_frame: usize,
  pub length: usize,
  pub loopable: bool,
  pub final_frame: usize,
  pub hold: u8,
}

impl Animation {
  pub fn from_serialized(s: AnimationSerialized) -> Self {
     let final_frame: usize = s.first_frame + s.length - 1;
    
    Animation {
      first_frame: s.first_frame,
      length: s.length,
      loopable: s.loopable,
      final_frame,
      hold: s.hold
    }
  }
}
#[derive(Debug, PartialEq)]
pub enum AnimationState {
  LOOPING,
  SMEARING,
}

pub struct AnimationTransitionEvent {
  pub player_id: PlayerId,
  pub transition: AnimationTransition,
}

impl AnimationTransitionEvent {
  pub fn new(player_id: PlayerId, transition: AnimationTransition) -> Self {
    AnimationTransitionEvent {
      player_id,
      transition
    }
  }
}

#[derive(Clone, Copy)]
pub enum AnimationTransition {
  WalkToIdle,
  BackwalkToIdle,
  CrouchToIdle,
  DashToIdle,
  BackDashToIdle,
  RiseToFall,
  FallToIdle,
  AirdashToFall,
  AirbackdashToFall,
  ToCrouch,
  ToWalk,
  ToBackwalk,
  ToDash,
  ToBackdash,
  ToRise,
  ToAirdash,
  ToAirBackdash,
}

#[derive(Debug, Component)]
pub struct AnimationController {
  character_prefix: String,
  animation_state: AnimationState,
  pub core_animation: Animation,
  pub smear_animation: Option<Animation>,
  current_hold: u8,
  hold_counter: u8,
  current_index: usize,
  animations: HashMap<String, Animation>
}

impl AnimationController {
  pub fn new(character_prefix: &str, library: &CharacterLibrary) -> Self {
    let mut animations = HashMap::new();
    let my_regex = Regex::new(&format!("(^{}.+)", character_prefix)[..]).unwrap();
    for (anim_id, animation) in library.animations.iter() {
      if my_regex.is_match(anim_id) {
        animations.insert(anim_id.clone(), animation.clone());
      }
    }

    AnimationController {
      character_prefix: character_prefix.to_string(),
      animation_state: AnimationState::LOOPING,
      core_animation: library.get_animation(format!("{}_idle", character_prefix.to_string())).unwrap(),
      smear_animation: None,
      current_index: 0,
      current_hold: 2,
      hold_counter: 0,
      animations,
    }
  }

  pub fn get_next_frame(&mut self) -> usize {
    self.current_hold = match self.animation_state {
      AnimationState::SMEARING => {
        if let Some(sa) = self.smear_animation {
          sa.hold
        } else {
          2
        }
      },
      AnimationState::LOOPING => {
        self.core_animation.hold
      }
    };

    if self.hold_counter == self.current_hold {
      let mut new_index: usize = self.current_index + 1;
      match self.animation_state {
        AnimationState::LOOPING => {
          if new_index > self.core_animation.final_frame {
            new_index = self.core_animation.first_frame;
          }
        },
        AnimationState::SMEARING => {
          if let Some(smear) = self.smear_animation {
            if new_index > smear.final_frame {
              self.animation_state = AnimationState::LOOPING;
              self.smear_animation = None;
              new_index = self.core_animation.first_frame;
            }
          }
        }
      }
      self.current_index = new_index;
      self.hold_counter = 0;
    } else {
      self.hold_counter += 1; 
    }
    return self.current_index;
  }

  pub fn reset(&mut self) {
    self.hold_counter = 0;
    self.current_index = match self.animation_state {
      AnimationState::SMEARING => {
        if let Some(sa) = self.smear_animation {
          sa.first_frame
        } else {
          self.core_animation.first_frame
        }
      },
      AnimationState::LOOPING => self.core_animation.first_frame,
    }
  }

  pub fn get_animation(&self, anim_id: String) -> Option<Animation>{
    if let Some(animation) = self.animations.get(&anim_id) {
      return Some(animation.clone());
    } else {
      return None;
    }
  }

  pub fn transition(&mut self, transition: AnimationTransition) {
    match transition {
      AnimationTransition::ToDash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>dash", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_dash", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToBackdash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>backdash", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_backdash", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToRise => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_jumpsquat", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_rise", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToWalk => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>walk", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_walk", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToBackwalk => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>backwalk", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_backwalk", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToCrouch => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>crouch", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_crouch", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::WalkToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_walk<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::BackwalkToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_backwalk<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::DashToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_dash<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::BackDashToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_backdash<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToAirdash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_fall<>airdash", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_airdash", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToAirBackdash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_fall<>backairdash", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_backairdash", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::RiseToFall => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_rise<>fall", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_fall", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::FallToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_fall<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::AirdashToFall => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_airdash<>fall", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_fall", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::AirbackdashToFall => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_backairdash<>fall", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_fall", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::CrouchToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_crouch<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
    }
    self.reset();
  }
}

pub fn read_animation_transitions(
  mut query: Query<(&PlayerId, &mut AnimationController)>,
  mut transition_reader: EventReader<AnimationTransitionEvent>,
) {
  for event in transition_reader.iter() {
    for (player_id, mut controller) in query.iter_mut() {
      if event.player_id == *player_id {
        controller.transition(event.transition);
      }
    }
  }
}

pub fn animate_sprite_system(
    mut query: Query<(&mut TextureAtlasSprite, &mut AnimationController)>,
) {
  for (mut sprite, mut anim_controller) in query.iter_mut() {
    sprite.index = anim_controller.get_next_frame();
  }
}
