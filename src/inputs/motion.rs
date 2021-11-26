pub use crate::*;
pub use regex::Regex;
#[derive(Debug)]
pub struct MotionEvent{
  pub motion: u8,
  pub player_id: u8,
  pub special_motion_duration: u8,
  pub special_motion: Option<SpecialMotion>
}

/// Notation, Priority, and Regex for special motions
pub struct SpecialMotionData {
  notation: String,
  priority: u8,
  regular_expression: Regex,
  command: SpecialMotion
}

impl SpecialMotionData {
  pub fn new(notation: String, priority: u8, regular_expression: Regex, command: SpecialMotion) -> Self {
    SpecialMotionData { 
      notation, 
      priority, 
      regular_expression,
      command
    }
  }
}

#[derive(Debug, Clone)]
pub enum SpecialMotion {
    FIREBALL,
    R_FIREBALL,
    DP,
    R_DP,
    HALF_CIRCLE_BACK,
    HALF_CIRCLE_FORWARD,
    DASH,
    BACK_DASH,
    INVITE_HELL
}

use crate::prelude::*;

impl MotionEvent {
  fn new(motion: u8, player_id: u8) -> Self {
    MotionEvent {
      motion,
      player_id,
      special_motion_duration: 0,
      special_motion: None
    }
  }
}

#[derive(Debug)]
pub struct InputBuffer {
  pub motions: Vec<u8>,
  pub player_id: u8,
  pub special_priority: u8,
  pub special_motion_duration: u8,
  pub special_motion: Option<SpecialMotion>
}

impl InputBuffer {
  pub fn new(player_id: u8) -> Self {
    InputBuffer {
      motions: Vec::new(),
      player_id,
      special_priority: 0,
      special_motion_duration: 0,
      special_motion: None
    }
  }

  pub fn update(&mut self, motion_input_reader: &mut EventReader<MotionEvent>) {
    self.tick();
    for event in motion_input_reader.iter() {
      if event.player_id == self.player_id {
        self.motions.push(event.motion);
      };
    };
    let (motion_string, command_input) = self.extract_special_motions();

    let mut cm_input = String::new();
    if let Some(sp) = command_input {
      write!(cm_input,"{:?}", sp).unwrap();
    } else {
      write!(cm_input," ").unwrap();
    };

    //println!("{:?} : {:?}",motion_string,cm_input);
  }

  fn tick(&mut self) {
    if self.motions.len() > 20 {
      self.motions.remove(0);
    }

    if self.special_motion_duration > 0 {
      self.special_motion_duration -= 1;
    }

    if self.special_motion_duration == 0 {
      self.special_motion = None;
    }
  }

  fn motion_to_string(self: &Self) -> String {
    let mut motions_string = String::new();
    for motion in self.motions.iter() {
      write!(motions_string,"{:?}",motion).unwrap();
    }
    return motions_string;
  }

  fn extract_special_motions(self: &mut Self) -> (String,Option<SpecialMotion>) {
    let motion_string = self.motion_to_string();
    let mut priority: u8 = self.special_priority;
    let mut current_command: Option<SpecialMotion> = None;

    for special_motion in MOTIONS.iter() {
      if special_motion.regular_expression.is_match(&motion_string[..]) {
        if special_motion.priority > priority {
          priority = special_motion.priority;
          current_command = Some(special_motion.command.clone());
        }
      }
    }

    if let Some(c) = current_command {
      self.special_motion = Some(c.clone());
      self.special_motion_duration = 5;
    }

    return (motion_string, self.special_motion.clone());
  }

}

pub fn write_motion_inputs(
  keyboard_input: Res<Input<KeyCode>>, 
  mut motion_writer: EventWriter<MotionEvent>
) {
  if keyboard_input.pressed(KeyCode::A) {
    if keyboard_input.pressed(KeyCode::S) {
      motion_writer.send(MotionEvent::new(1,1));
      return
    }
    if keyboard_input.pressed(KeyCode::W) {
      motion_writer.send(MotionEvent::new(7,1));
      return
    }
      motion_writer.send(MotionEvent::new(4,1));
      return
  };
  if keyboard_input.pressed(KeyCode::D) {
    if keyboard_input.pressed(KeyCode::S) {
      motion_writer.send(MotionEvent::new(3,1));
      return
    }
    if keyboard_input.pressed(KeyCode::W) {
      motion_writer.send(MotionEvent::new(9,1));
      return
    }
    motion_writer.send(MotionEvent::new(6,1));
    return
  }
  if keyboard_input.pressed(KeyCode::W) {
    motion_writer.send(MotionEvent::new(8,1));
    return
  }

  if keyboard_input.pressed(KeyCode::S) {
    motion_writer.send(MotionEvent::new(2,1));
    return
  }
  motion_writer.send(MotionEvent::new(5,1));
  return
}

pub fn read_motion_inputs(
  mut motion_input_reader: EventReader<MotionEvent>, 
  mut query: Query<&mut InputBuffer>,
) {
  for mut buffer in query.iter_mut() {
    buffer.update(&mut motion_input_reader);
  };
}



