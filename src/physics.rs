use lerp::Lerp;
use bevy::prelude::*;

pub trait CustomLerp {
  fn custom_lerp(&self, target: Self, t: f32) -> Self;
}

#[allow(unstable_name_collisions)]
impl CustomLerp for Vec2 {
  fn custom_lerp(&self, target: Vec2, t: f32) -> Vec2 {
    if &self.distance(target) > &0.02 {
      let _x = self.x.lerp(target.x,t);
      let _y = self.y.lerp(target.y,t);
      return Vec2::new(_x,_y);
    } else {
      return target;
    }
  }
}


/// Used to add fake physics forces, instead of constant changes to velocity, 
/// interpolated forces are set to ease to 0 accross their duration, the proper velocity from
/// the interpolated force can be accessed by calling their `update()` method, which will update 
/// the easing value for the IntForce as well
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Clone, Copy, Debug)]
pub struct InterpolatedForce {
  current_velocity: Vec2,
  starting_velocity: Vec2,
  ending_velocity: Vec2,
  duration: u8,
  frames_elapsed: u8
}

impl InterpolatedForce {

  pub fn new(starting_velocity: Vec2, ending_velocity: Vec2, duration: u8) -> Self {
    return InterpolatedForce {
      current_velocity: starting_velocity,
      starting_velocity,
      ending_velocity,
      duration,
      frames_elapsed: 0
    }
  }

  pub fn update(&mut self) -> Vec2 {
    self.tick();
    let t = self.frames_elapsed as f32 / self.duration as f32;
    self.current_velocity = self.current_velocity.custom_lerp(self.ending_velocity,t);
    return self.current_velocity;
  }

  pub fn tick(&mut self) {
    self.frames_elapsed += 1;
  }

  pub fn is_finished(&self) -> bool {
    return self.duration == self.frames_elapsed;
  }
}
