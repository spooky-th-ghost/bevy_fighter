pub use crate::prelude::*;

#[derive(Debug)]
pub struct FighterInputEvent{
  pub player_id: u8,
  /// Direction of the input, expressed in numpad notation
  pub motion: u8,
  /// Any command motion found in the input event
  pub special_motion: Option<CommandType>,
  /// Duration of the current command motion
  pub special_motion_duration: u8,
  /// Buttons Pressed
  pub pressed: Vec<FighterButtonType>,
  /// Buttons Just Pressed
  pub just_pressed: Vec<FighterButtonType>
}

impl FighterInputEvent{
  fn new(
    motion: u8, 
    player_id: u8, 
    pressed: Vec<FighterButtonType>, 
    just_pressed: Vec<FighterButtonType>
  ) -> Self {
    FighterInputEvent {
      motion,
      player_id,
      special_motion_duration: 0,
      special_motion: None,
      pressed,
      just_pressed,
    }
  }
}
#[derive(Debug,Clone, Copy)]
pub enum FighterButtonType {
  A,
  B,
  C,
  D,
  E,
  F,
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
      ..} = mapper.get_pressed_buttons(&keyboard_input, &button_input);

    let InputActionsPressed {
      a:j_a, 
      b:j_b, 
      c:j_c, 
      d:j_d,
      e:j_e,
      f:j_f,
      ..} = mapper.get_just_pressed_buttons(&keyboard_input, &button_input);


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

    let mut pressed = Vec::new();
    let mut just_pressed = Vec::new();

    if a {pressed.push(FighterButtonType::A)}
    if b {pressed.push(FighterButtonType::B)}
    if c {pressed.push(FighterButtonType::C)}
    if d {pressed.push(FighterButtonType::D)}
    if e {pressed.push(FighterButtonType::E)}
    if f {pressed.push(FighterButtonType::F)}

    if j_a {just_pressed.push(FighterButtonType::A)}
    if j_b {just_pressed.push(FighterButtonType::B)}
    if j_c {just_pressed.push(FighterButtonType::C)}
    if j_d {just_pressed.push(FighterButtonType::D)}
    if j_e {just_pressed.push(FighterButtonType::E)}
    if j_f {just_pressed.push(FighterButtonType::F)}

    input_writer.send(
      FighterInputEvent::new(
        motion,
        *player_id,
        pressed,
        just_pressed
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

pub struct FighterInputPlugin;

impl Plugin for FighterInputPlugin {
  fn build(&self, app: &mut App) {
    app
    .add_event::<FighterInputEvent>()
    .insert_resource(PlayerData::default())
    .add_system_set(
      SystemSet::new()
        .with_run_criteria(FixedTimestep::step(0.01667))
        .with_system(write_fighter_inputs.label(FighterSystemLabels::InputWrite))
        .with_system(read_fighter_inputs.after(FighterSystemLabels::InputWrite))
    );
  }
}



