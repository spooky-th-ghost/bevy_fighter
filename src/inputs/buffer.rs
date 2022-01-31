pub use crate::prelude::*;

#[derive(Debug)]
pub struct FighterInputBuffer {
  pub motions: Vec<u8>,
  pub pressed: Vec<Vec<FighterButtonType>>,
  pub just_pressed: Vec<Vec<FighterButtonType>>,
  pub player_id: PlayerId,
  pub command_priority: u8,
  pub command_duration: u8,
  pub command_type: Option<CommandType>,
  pub current_motion: u8,
  pub previous_motion: u8,
  pub command_lockout: u8,
}

impl FighterInputBuffer {
  pub fn new(player_id: PlayerId) -> Self {
    FighterInputBuffer {
      motions: Vec::new(),
      pressed: Vec::new(),
      just_pressed: Vec::new(),
      player_id,
      command_priority: 0,
      command_duration: 0,
      command_type: None,
      current_motion: 5,
      previous_motion: 5,
      command_lockout: 0,
    }
  }

  pub fn update(&mut self, event: &FighterInputEvent) {
    self.tick();
    if event.player_id == self.player_id {
      self.motions.push(event.motion);
      self.pressed.push(event.pressed.clone());
      self.just_pressed.push(event.just_pressed.clone());
      self.previous_motion = self.current_motion;
      self.current_motion = event.motion;
    };
    self.extract_special_motions();
  }

  fn tick(&mut self) {
    if self.motions.len() > 20 {
      self.motions.remove(0);
    }

    if self.pressed.len() > 20 {
      self.pressed.remove(0);
    }

    if self.just_pressed.len() > 20 {
      self.just_pressed.remove(0);
    }

    if self.command_duration == 0 {
      self.command_type = None;
    }

    self.command_duration = countdown(self.command_duration);
    self.command_lockout = countdown(self.command_lockout);
  }

  fn motion_to_string(&mut self) -> String {
    let mut motions_string = String::new();
    for motion in self.motions.iter() {
      write!(motions_string,"{:?}",motion).unwrap();
    }
    return motions_string;
  }

  pub fn consume_motion(&mut self) {
    self.command_type = None;
    self.command_lockout = 3;
    self.command_duration = 0;
  }
  
  fn extract_special_motions(&mut self) {
    if self.command_lockout == 0 {
      let motion_string = self.motion_to_string();
      let mut priority: u8 = self.command_priority;
      let mut current_command: Option<CommandType> = None;

      for command_motion in MOTIONS.iter() {
        if  command_motion.check(&motion_string[..], priority) {
          priority = command_motion.priority;
          current_command = Some(command_motion.command.clone());
        }
      }

      if let Some(c) = current_command {
        self.command_type = Some(c);
        self.command_duration = 5;
      }
    }
  }

}
