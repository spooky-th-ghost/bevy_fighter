pub use crate::prelude::*;

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

#[derive(Clone)]
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
  ToIdle,
  ToAirdash,
  ToAirBackdash,
  Attack {name: String}
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
    for (anim_id, animation) in library.read_animations() {
      if my_regex.is_match(anim_id) {
        let trimmed_animation_name = anim_id.replace(character_prefix, "").replace("_","");
        animations.insert(trimmed_animation_name.clone(), animation.clone());
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
      AnimationTransition::ToIdle => {
        self.animation_state = AnimationState::LOOPING;
        self.smear_animation = None;
        let core_animation = self.get_animation("idle".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::Attack {name} => {
        self.animation_state = AnimationState::LOOPING;
        self.smear_animation = None;
        let core_animation = self.get_animation(name);
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToDash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("idle<>dash".to_string());
        let core_animation = self.get_animation("dash".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToBackdash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("idle<>backdash".to_string());
        let core_animation = self.get_animation("backdash".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToRise => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("jumpsquat".to_string());
        let core_animation = self.get_animation("rise".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToWalk => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("idle<>walk".to_string());
        let core_animation = self.get_animation("walk".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToBackwalk => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("idle<>backwalk".to_string());
        let core_animation = self.get_animation("backwalk".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToCrouch => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("idle<>crouch".to_string());
        let core_animation = self.get_animation("crouch".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::WalkToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("walk<>idle".to_string());
        let core_animation = self.get_animation("idle".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::BackwalkToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("backwalk<>idle".to_string());
        let core_animation = self.get_animation("idle".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::DashToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("dash<>idle".to_string());
        let core_animation = self.get_animation("idle".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::BackDashToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("backdash<>idle".to_string());
        let core_animation = self.get_animation("idle".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToAirdash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("fall<>airdash".to_string());
        let core_animation = self.get_animation("airdash".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::ToAirBackdash => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("fall<>backairdash".to_string());
        let core_animation = self.get_animation("backairdash".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::RiseToFall => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("rise<>fall".to_string());
        let core_animation = self.get_animation("fall".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::FallToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("fall<>idle".to_string());
        let core_animation = self.get_animation("idle".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::AirdashToFall => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("airdash<>fall".to_string());
        let core_animation = self.get_animation("fall".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::AirbackdashToFall => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("backairdash<>fall".to_string());
        let core_animation = self.get_animation("fall".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
      AnimationTransition::CrouchToIdle => {
        self.animation_state = AnimationState::SMEARING;
        self.smear_animation = self.get_animation("crouch<>idle".to_string());
        let core_animation = self.get_animation("idle".to_string());
        if let Some(ca) = core_animation {
          self.core_animation = ca;
        }
      },
    }
    self.reset();
  }
}
