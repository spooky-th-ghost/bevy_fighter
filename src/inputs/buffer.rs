pub use crate::prelude::*;

#[derive(Debug)]
pub struct FighterInputBuffer {
  pub motions: Vec<u8>,
  pub pressed: Vec<Vec<FighterButtonType>>,
  pub just_pressed: Vec<Vec<FighterButtonType>>,
  pub player_id: u8,
  pub command_priority: u8,
  pub command_duration: u8,
  pub command_type: Option<CommandType>,
  pub current_motion: u8,
}

impl FighterInputBuffer {
  pub fn new(player_id: u8) -> Self {
    FighterInputBuffer {
      motions: Vec::new(),
      pressed: Vec::new(),
      just_pressed: Vec::new(),
      player_id,
      command_priority: 0,
      command_duration: 0,
      command_type: None,
      current_motion: 5
    }
  }

  pub fn update(&mut self, event: &FighterInputEvent) {
    self.tick();
    if event.player_id == self.player_id {
      self.motions.push(event.motion);
      self.pressed.push(event.pressed.clone());
      self.just_pressed.push(event.just_pressed.clone());
      self.current_motion = event.motion;
    };
    
    let (motion_string, command_input) = self.extract_special_motions();

    let mut cm_input = String::new();
    if let Some(sp) = command_input {
      write!(cm_input,"{:?}", sp).unwrap();
    } else {
      write!(cm_input," ").unwrap();
    };
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


    if self.command_duration > 0 {
      self.command_duration -= 1;
    }

    if self.command_duration == 0 {
      self.command_type = None;
    }
  }

  fn motion_to_string(&mut self) -> String {
    let mut motions_string = String::new();
    for motion in self.motions.iter() {
      write!(motions_string,"{:?}",motion).unwrap();
    }
    return motions_string;
  }
  
  fn extract_special_motions(&mut self) -> (String,Option<CommandType>) {
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

    return (motion_string, self.command_type.clone());
  }

}
