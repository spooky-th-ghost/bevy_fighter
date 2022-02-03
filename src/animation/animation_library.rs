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
          Animation::new(animation.first_frame, animation.length, animation.loopable)
        )
      );
    }

    self.add(
      HashMap::from_iter::<HashMap<String, Animation>>(raw_anims.iter().cloned().collect())
    );
    println!("{:?}", self);
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
}

impl Animation {
  pub fn new(first_frame: usize, length: usize, loopable: bool) -> Self {
    Animation {
      first_frame,
      length,
      loopable
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

pub struct AnimationState {
  core: Animation,
  enter: Option<Animation>,
  exit: Option<Animation>,
}
