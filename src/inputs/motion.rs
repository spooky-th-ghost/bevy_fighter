pub use crate::*;
pub use regex::Regex;
#[derive(Debug)]
pub struct MotionEvent{
  pub motion: u8,
  pub player_id: u8,
  pub special_motion_duration: u8,
  pub special_motion: Option<CommandType>
}

/// Notation, Priority, and Regex for special motions
#[derive(Debug)]
pub struct CommandMotion {
  pub priority: u8,
  pub regular_expression: Regex,
  pub command: CommandType
}

impl CommandMotion {
  pub fn new(priority: u8, regular_expression: Regex, command: CommandType) -> Self {
    CommandMotion { 
      priority, 
      regular_expression,
      command
    }
  }
}

#[derive(Debug, Clone)]
pub enum CommandType {
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



