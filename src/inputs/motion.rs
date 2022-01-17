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
  player_inputs: Res<PlayerInputs>,
  keyboard_input: Res<Input<KeyCode>>, 
  button_input: Res<Input<GamepadButton>>,
  mut motion_writer: EventWriter<MotionEvent>
) {
  for mapper in player_inputs.local_devices.iter() {
    let mut h_axis: f32 = 0.0;
    let mut v_axis: f32 = 0.0;
    let InputMapper { player_id, ..} = mapper;

    // let h_axis_positive_pressed = match mapper.x_positive {
    //   RawButton::K(keycode) => keyboard_input.pressed(keycode),
    //   RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
    // };

    // let h_axis_negative_pressed = match mapper.x_negative {
    //   RawButton::K(keycode) => keyboard_input.pressed(keycode),
    //   RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
    // };

    // let v_axis_positive_pressed = match mapper.y_positive {
    //   RawButton::K(keycode) => keyboard_input.pressed(keycode),
    //   RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
    // };

    // let v_axis_negative_pressed = match mapper.y_negative {
    //   RawButton::K(keycode) => keyboard_input.pressed(keycode),
    //   RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
    // };
    
    let InputActionsPressed {right, left, up, down, ..} = mapper.get_pressed_buttons(&keyboard_input, &button_input);


    if left {
      h_axis -= 1.0 *  mapper.get_facing_vector();
    }

    if right {
      h_axis += 1.0 * mapper.get_facing_vector();
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
    motion_writer.send(MotionEvent::new(motion,*player_id));
  }
}

pub fn read_motion_inputs(
  mut motion_input_reader: EventReader<MotionEvent>, 
  mut query: Query<(&mut InputBuffer, &PlayerId)>,
) {
  for (mut buffer, pid) in query.iter_mut() {
    buffer.update(&mut motion_input_reader, pid.0);
  };
}

pub struct MotionInputPlugin;

impl Plugin for MotionInputPlugin {
  fn build(&self, app: &mut App) {
    app
    .add_event::<MotionEvent>()
    .insert_resource(PlayerInputs::default())
    .add_system_set(
      SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.01667))
        .with_system(write_motion_inputs.label(FighterSystemLabels::InputWrite))
        .with_system(read_motion_inputs.after(FighterSystemLabels::InputWrite))
    );
  }
}



