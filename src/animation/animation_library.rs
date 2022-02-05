pub use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct SpriteSheetJson {
  pub animations: Vec<RawJsonAnimation>
}

#[derive(Deserialize, Serialize)]
pub struct RawJsonAnimation {
  pub name: String,
  pub first_frame: usize,
  pub length: usize,
  pub loopable: bool,
  pub hold: u8,
}

#[derive(Debug, Clone)]
pub struct AnimationLibrary {
  animations: HashMap<String, Animation>
}

impl AnimationLibrary {
  pub fn new() -> Self {
    let animations: HashMap<String, Animation> = HashMap::new();
    AnimationLibrary {
      animations
    }
  }

  pub fn load_character_sprite_data(&mut self, character_name: &str, raw_path: &str) {
  let path = Path::new(raw_path);
  if let Ok(raw_string) = read_to_string(path) {
    let raw_slice = &raw_string[..]; 
    let json_sheet: SpriteSheetJson = from_str(raw_slice).unwrap();

    let mut raw_anims: Vec<(String, Animation)> = Vec::new();
    for animation in json_sheet.animations {
      raw_anims.push(
        (
          format!("{}_{}",character_name,animation.name.clone()),
          Animation::new(animation.first_frame, animation.length, animation.loopable, animation.first_frame + animation.length - 1, animation.hold)
        )
      );
    }

    self.add(
      HashMap::from_iter::<HashMap<String, Animation>>(raw_anims.iter().cloned().collect())
    );
  }
}

  pub fn add(&mut self, animations: HashMap<String, Animation>) {
    self.animations.extend(animations);
  }

  pub fn get(&self, anim_id: String) -> Option<Animation>{
    if let Some(animation) = self.animations.get(&anim_id) {
      return Some(animation.clone());
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
  pub fn new(first_frame: usize, length: usize, loopable: bool, final_frame: usize, hold: u8) -> Self {
    Animation {
      first_frame,
      length,
      loopable,
      final_frame,
      hold
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
  pub transition: AnimationStateTransition,
}

impl AnimationTransitionEvent {
  pub fn new(player_id: PlayerId, transition: AnimationStateTransition) -> Self {
    AnimationTransitionEvent {
      player_id,
      transition
    }
  }
}

#[derive(Clone, Copy)]
pub enum AnimationStateTransition {
  IdleToDash,
  IdleToBackdash,
  IdleToRise,
  IdleToWalk,
  IdleToBackwalk,
  IdleToCrouching,
  WalkToRise,
  WalkToIdle,
  WalkToDash,
  BackwalkToRise,
  BackwalkToIdle,
  BackwalkToBackdash,
  CrouchToIdle,
  DashToRise,
  DashToIdle,
  BackDashToIdle,
  BackDashToBackwalk,
  RiseToAirdash,
  RiseToAirbackdash,
  RiseToFall,
  RiseToRise,
  FallToRise,
  FallToAirdash,
  FallToAirbackdash,
  FallToIdle,
  AirdashToFall,
  AirbackdashToFall,
  //Absolute Transitions
  ToCrouch,
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
  pub fn new(character_prefix: &str, library: &AnimationLibrary) -> Self {
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
      core_animation: library.get(format!("{}_idle", character_prefix.to_string())).unwrap(),
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

  pub fn transition(&mut self, transition: AnimationStateTransition) {
    match transition {
      AnimationStateTransition::IdleToDash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>dash", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_dash", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::IdleToBackdash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>backdash", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_backdash", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::IdleToRise 
      | AnimationStateTransition::WalkToRise 
      | AnimationStateTransition::BackwalkToRise 
      | AnimationStateTransition::DashToRise 
      | AnimationStateTransition::RiseToRise
      | AnimationStateTransition::FallToRise => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_jumpsquat", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_rise", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::IdleToWalk => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>walk", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_walk", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::IdleToBackwalk => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>backwalk", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_backwalk", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::IdleToCrouching => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>crouch", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_crouch", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::WalkToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_walk<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::WalkToDash => {
        self.animation_state = AnimationState::LOOPING;
        self.smear_animation = None;
        let core_animation = self.get_animation(format!("{}_dash", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::BackwalkToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_backwalk<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::BackwalkToBackdash => {
        self.animation_state = AnimationState::LOOPING;
        self.smear_animation = None;
        let core_animation = self.get_animation(format!("{}_backdash", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::DashToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_dash<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::BackDashToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_backdash<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::RiseToAirdash
      | AnimationStateTransition::FallToAirdash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_fall<>airdash", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_airdash", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::RiseToAirbackdash
      | AnimationStateTransition::FallToAirbackdash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_fall<>backairdash", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_backairdash", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::RiseToFall => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_rise<>fall", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_fall", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::FallToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_fall<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::AirdashToFall => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_airdash<>fall", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_fall", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::AirbackdashToFall => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_backairdash<>fall", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_fall", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::CrouchToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_crouch<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_idle", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::BackDashToBackwalk => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_backdash<>idle", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_backwalk", self.character_prefix));
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationStateTransition::ToCrouch => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation(format!("{}_idle<>crouch", self.character_prefix));
        let core_animation = self.get_animation(format!("{}_crouch", self.character_prefix));
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
    mut query: Query<(&PlayerId, &mut TextureAtlasSprite, &mut AnimationController)>,
) {
  for (player_id,mut sprite, mut anim_controller) in query.iter_mut() {
    sprite.index = anim_controller.get_next_frame();
    if *player_id == PlayerId::P2 {println!("{:?}", sprite.index);}
  }
}
