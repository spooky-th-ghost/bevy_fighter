pub use crate::prelude::*;

pub struct AnimationLibrary {
  animations: HashMap<String, Animation>
}

impl AnimationLibrary {
  pub fn new(animations: HashMap<String, Animation>) -> Self {
    AnimationLibrary {
      animations
    }
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
