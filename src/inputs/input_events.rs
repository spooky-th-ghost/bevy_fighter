use regex::Regex;
use bevy::prelude::*;
use crate::{
  inputs::{
    PlayerData,
    FighterInputMapper,
    InputActionsPressed
  },
  character::{
    PlayerId
  }
};

#[derive(Debug)]
pub struct FighterInputEvent{
  pub player_id: PlayerId,
  /// Direction of the input, expressed in numpad notation
  pub motion: u8,
  /// Reperesents what buttons are pressed as a single byte
  pub button_press: ButtonPress,
  /// Any command motion found in the input event
  pub special_motion: Option<CommandType>,
  /// Duration of the current command motion
  pub special_motion_duration: u8,
}

impl FighterInputEvent{
  fn new(
    motion: u8, 
    player_id: PlayerId, 
    button_press: ButtonPress
  ) -> Self {
    FighterInputEvent {
      motion,
      player_id,
      button_press,
      special_motion_duration: 0,
      special_motion: None,
    }
  }
}

#[derive(Debug,Clone, Copy)]
pub struct ButtonPress {
  pub value: u8,
}

impl ButtonPress {
  pub fn new(value: u8) -> Self {
    ButtonPress {
      value
    }
  }

  pub fn any_pressed(&self) -> bool {
    return self.value != 0;
  }

  pub fn to_string(&self) -> String {
    let mut button_string = String::new();
    if self.is_bit_set(0) {button_string.push('A')}
    if self.is_bit_set(1) {button_string.push('B')}
    if self.is_bit_set(2) {button_string.push('C')}
    if self.is_bit_set(3) {button_string.push('D')}
    if self.is_bit_set(4) {button_string.push('E')}
    if self.is_bit_set(5) {button_string.push('F')}
    if self.is_bit_set(6) {button_string.push('G')}
    if self.is_bit_set(7) {button_string.push('H')}
    return button_string;
  }

  pub fn is_button_pressed(&self, button: char) -> bool {
    let shift: u8 = match button {
      'A' => 0,
      'B' => 1,
      'C' => 2,
      'D' => 3,
      'E' => 4,
      'F' => 5,
      'G' => 6,
      'H' => 7,
        _ => return false
    };

    return self.is_bit_set(shift);
  }


  pub fn is_bit_set(&self, position: u8) -> bool {
    return (self.value & (1 << position)) != 0;
  }
}

/// Notation, Priority, and Regex for special motions
#[derive(Debug)]
pub struct CommandMotion {
  pub priority: u8,
  regular_expression: Regex,
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

  pub fn check(&self, buffer_string: &str, buffer_priority: u8) -> bool {
    return self.regular_expression.is_match(buffer_string) && self.priority > buffer_priority;
  }


}

#[derive(Debug, Clone, Copy)]
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

pub fn write_fighter_inputs(
  player_data: Res<PlayerData>,
  keyboard_input: Res<Input<KeyCode>>, 
  button_input: Res<Input<GamepadButton>>,
  mut input_writer: EventWriter<FighterInputEvent>
) {
  for mapper in player_data.local_devices.iter() {
    let mut h_axis: f32 = 0.0;
    let mut v_axis: f32 = 0.0;
    let FighterInputMapper { player_id, ..} = mapper;
    
    let InputActionsPressed {
      right, 
      left, 
      up, 
      down,
      a, 
      b, 
      c, 
      d,
      e,
      f,
      macro_1,
      macro_2} = mapper.get_pressed_buttons(&keyboard_input, &button_input);


    if left {
      h_axis -= 1.0 * player_data.get_facing_vector(player_id);
    }

    if right {
      h_axis += 1.0 * player_data.get_facing_vector(player_id);
    }

    if up {
      v_axis = 1.0;
    }

    if down {
      if v_axis == 0.0 {
        v_axis = -1.0;
      }
    }

    let mut motion: u8 = 5;

    if h_axis == 0.0 {
      if v_axis == 1.0 {
        motion = 8;
      }

      if v_axis == -1.0 {
        motion = 2;
      }
    }

    if h_axis == -1.0 {
      if v_axis == 1.0 {
        motion = 7;
      }

      if v_axis == 0.0 {
        motion = 4;
      }

      if v_axis == -1.0 {
        motion = 1;
      }
    }

    if h_axis == 1.0 {
      if v_axis == 1.0 {
        motion = 9;
      }

      if v_axis == 0.0 {
        motion = 6;
      }

      if v_axis == -1.0 {
        motion = 3;
      }
    }


    let mut pressed_byte: u8 = 0b0000_0000;
    if a {pressed_byte |= 0b0000_0001}
    if b {pressed_byte |= 0b0000_0010}
    if c {pressed_byte |= 0b0000_0100}
    if d {pressed_byte |= 0b0000_1000}
    if e {pressed_byte |= 0b0001_0000}
    if f {pressed_byte |= 0b0010_0000}
    if macro_1 {pressed_byte |= 0b0100_0000}
    if macro_2 {pressed_byte |= 0b1000_0000}
    let button_press = ButtonPress::new(pressed_byte);
    input_writer.send(
      FighterInputEvent::new(
        motion,
        *player_id,
        button_press
      )
    );

  }
}

pub fn read_fighter_inputs(
  mut input_reader: EventReader<FighterInputEvent>, 
  mut player_data: ResMut<PlayerData>,
) {
  for event in input_reader.iter() {
    for buffer in player_data.buffers.iter_mut() {
      if event.player_id == buffer.player_id {
        buffer.update(event);
      }
    };
  };
}



