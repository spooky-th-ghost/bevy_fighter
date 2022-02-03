pub use crate::prelude::*;

#[derive(Deserialize, Serialize)]
pub struct SpriteSheetJson {
  pub animations: Vec<RawAnimation>
}

#[derive(Deserialize, Serialize)]
pub struct RawAnimation {
  pub name: String,
  pub first_frame: usize,
  pub length: usize,
  pub loopable: bool
}

#[derive(Debug)]
pub struct AnimationLibrary {
  animations: HashMap<String, AnimationCell>
}


impl AnimationLibrary {
  pub fn new() -> Self {
    let animations: HashMap<String, AnimationCell> = HashMap::new();
    AnimationLibrary {
      animations
    }
  }

  pub fn load_character_sprite_data(&mut self, character_name: &str, raw_path: &str) {
  let path = Path::new(raw_path);
  if let Ok(raw_string) = read_to_string(path) {
    let raw_slice = &raw_string[..]; 
    let json_sheet: SpriteSheetJson = from_str(raw_slice).unwrap();

    let mut raw_anims: Vec<(String, AnimationCell)> = Vec::new();
    for animation in json_sheet.animations {
      raw_anims.push(
        (
          format!("{}_{}",character_name,animation.name.clone()),
          AnimationCell::new(animation.first_frame, animation.length, animation.loopable, animation.first_frame + animation.length - 1)
        )
      );
    }

    self.add(
      HashMap::from_iter::<HashMap<String, AnimationCell>>(raw_anims.iter().cloned().collect())
    );
    println!("{:?}", self);
  }
}

  pub fn add(&mut self, animations: HashMap<String, AnimationCell>) {
    self.animations.extend(animations);
  }

  pub fn get(&self, anim_id: String) -> Option<AnimationCell>{
    if let Some(animation) = self.animations.get(&anim_id) {
      return Some(animation.clone());
    } else {
      return None;
    }
  }
}

#[derive(Clone,Copy, PartialEq, Debug)]
pub struct AnimationCell {
  pub first_frame: usize,
  pub length: usize,
  pub loopable: bool,
  pub final_frame: usize,
}

impl AnimationCell {
  pub fn new(first_frame: usize, length: usize, loopable: bool, final_frame: usize) -> Self {
    AnimationCell {
      first_frame,
      length,
      loopable,
      final_frame,
    }
  }
  pub fn transform_index(&self, relative_index: usize) -> (usize,usize) {
    let new_true_index: usize;
    let mut new_relative_index: usize = relative_index + 1;
    if new_relative_index >= self.length {
      new_true_index = self.first_frame;
      new_relative_index = 0;
    } else {
      new_true_index = self.first_frame + new_relative_index;
    }
    return (new_relative_index, new_true_index);
  }
}

pub struct AnimationSheet {
  character_name: String,
  idle: Animation,
  walk: Animation,
  crouch: Animation,
  dash: Animation,
  backdash: Animation,
  jumpsquat: Animation,
  rising: Animation,
  falling: Animation,
  airdash: Animation,
  backairdash: Animation,
  landing: Animation,
}

#[derive(Debug)]
pub struct Animation {
  core: AnimationCell,
  enter: Option<AnimationCell>,
  exit: Option<AnimationCell>,
  loopable: bool,
  current_index: usize,
  current_cell: AnimationCell,
  animation_state: AnimationState,
}

impl Animation {
  pub fn new(core: AnimationCell, enter: Option<AnimationCell>, exit: Option<AnimationCell>, loopable: bool) -> Self {
    let (current_index, current_cell, animation_state) = if let Some(enter_cell) = enter {
      (enter_cell.first_frame, enter_cell, AnimationState::ENTER)
    } else {
      (core.first_frame, core, AnimationState::CORE)
    };
    Animation {
      core,
      enter,
      exit,
      loopable,
      current_index,
      current_cell,
      animation_state
    }
  }

  pub fn reset(&mut self) {
     let (current_index, current_cell, animation_state) = if let Some(enter_cell) = self.enter {
      (enter_cell.first_frame, enter_cell, AnimationState::ENTER)
    } else {
      (self.core.first_frame, self.core, AnimationState::CORE)
    };

    self.current_index = current_index;
    self.current_cell = current_cell;
    self.animation_state = animation_state;
  }

  pub fn get_next_frame(&mut self) {
    let mut new_index = self.current_index + 1;

    if new_index > self.current_cell.final_frame  {
      if self.current_cell.loopable {
        new_index = self.current_cell.first_frame;
      } else {
        match self.animation_state {
          AnimationState::ENTER => {
            self.animation_state = AnimationState::CORE;
            self.current_cell = self.core;
            new_index = self.current_cell.first_frame;
          },
          _ => ()
        }
      }
    }
    self.current_index = new_index;
  }
}

#[derive(Debug, PartialEq)]
pub enum AnimState {
  Run,
  Idle
}

#[derive(Debug, PartialEq)]
pub enum AnimationState {
  ENTER,
  CORE,
  EXIT,
}

#[derive(Debug)]
pub struct AnimationController {
  character_prefix: String,
  direction: u8,
  anim_state: AnimState,
  pub current_animation: Option<AnimationCell>,
  pub prev_animation: Option<Animation>,
  current_index: usize,
  relative_index: usize
}

impl AnimationController {
  pub fn new(character_prefix: String) -> Self {
    AnimationController {
      character_prefix,
      anim_state: AnimState::Idle,
      direction: 1,
      current_animation: None,
      prev_animation: None,
      current_index: 0,
      relative_index: 0
    }
  }

  pub fn get_next_frame(&mut self) -> usize {
    if let Some(animation) = self.current_animation {
      let (new_relative_index, true_index) = animation.transform_index(self.relative_index);
      self.relative_index = new_relative_index;
      return true_index;
    } else {
      self.relative_index = 0;
      return 0;
    }
  }

  pub fn set_direction(&mut self, library: &AnimationLibrary, new_direction: u8) {
    if self.direction != new_direction {
      self.direction = new_direction;
      let state_str = match self.anim_state {
        AnimState::Idle => "idle",
        AnimState::Run => "run"
      };
      self.current_animation = library.get(format!("{0}_{1}_{2}", self.character_prefix,state_str, self.direction));
    }
  }

  pub fn set_anim_state(&mut self, library: &AnimationLibrary, new_state: AnimState) {
    if self.anim_state != new_state {
      self.anim_state = new_state;
      let state_str = match self.anim_state {
        AnimState::Idle => "idle",
        AnimState::Run => "run"
      };
      self.current_animation = library.get(format!("{0}_{1}_{2}", self.character_prefix,state_str, self.direction));
    }
  }

  pub fn get_initial_animation(&mut self, library: &AnimationLibrary) {
      let state_str = match self.anim_state {
        AnimState::Idle => "idle",
        AnimState::Run => "run"
      };
      let anim_id = format!("{0}_{1}_{2}", self.character_prefix,state_str, self.direction);
      println!("{}",anim_id);
      self.current_animation = library.get(anim_id);
    }
  
}
