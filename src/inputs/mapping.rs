use crate::prelude::*;

pub struct FighterCharacterPosition {
  pub player_id: PlayerId,
  position: Vec3
}

impl FighterCharacterPosition {
  pub fn new(player_id: PlayerId, position: Vec3) -> Self {
    FighterCharacterPosition {
      player_id,
      position,
    }
  }

  pub fn set_position(&mut self, position: Vec3) {
    self.position = position;
  }

  pub fn get_position(&self) -> Vec3 {
    return self.position;
  }
}

pub struct FighterFacingVector {
  player_id: PlayerId,
  vector: f32,
}

impl FighterFacingVector {
  pub fn new(player_id: PlayerId, vector: f32) -> Self {
    FighterFacingVector {
      player_id,
      vector
    }
  }
}

pub struct PlayerData {
  pub local_devices: Vec<FighterInputMapper>,
  pub buffers: Vec<FighterInputBuffer>,
  pub positions: Vec<FighterCharacterPosition>
}

impl Default for PlayerData {
  fn default() -> Self {
    PlayerData {
      local_devices: vec![
        FighterInputMapper {
            player_id: PlayerId::P1,
            a: RawButton::G(Gamepad(0),GamepadButtonType::West),
            b: RawButton::G(Gamepad(0),GamepadButtonType::North),
            c: RawButton::G(Gamepad(0),GamepadButtonType::RightTrigger),
            d: RawButton::G(Gamepad(0),GamepadButtonType::South),
            e: RawButton::G(Gamepad(0),GamepadButtonType::East),
            f: RawButton::G(Gamepad(0),GamepadButtonType::RightTrigger2),
            macro_1: RawButton::G(Gamepad(0),GamepadButtonType::LeftTrigger),
            macro_2: RawButton::G(Gamepad(0),GamepadButtonType::LeftTrigger2),
            x_positive: RawButton::G(Gamepad(0),GamepadButtonType::DPadRight),
            x_negative: RawButton::G(Gamepad(0),GamepadButtonType::DPadLeft),
            y_positive: RawButton::G(Gamepad(0),GamepadButtonType::DPadUp),
            y_negative: RawButton::G(Gamepad(0),GamepadButtonType::DPadDown),
        },
          FighterInputMapper {
            player_id: PlayerId::P2,
            a: RawButton::K(KeyCode::Y),
            b: RawButton::K(KeyCode::U),
            c: RawButton::K(KeyCode::I),
            d: RawButton::K(KeyCode::G),
            e: RawButton::K(KeyCode::H),
            f: RawButton::K(KeyCode::J),
            macro_1: RawButton::K(KeyCode::O),
            macro_2: RawButton::K(KeyCode::K),
            x_positive: RawButton::K(KeyCode::E),
            x_negative: RawButton::K(KeyCode::Q),
            y_positive: RawButton::K(KeyCode::Space),
            y_negative: RawButton::K(KeyCode::W),
        },
      ],
      buffers: vec![
        FighterInputBuffer::new(PlayerId::P1),
        FighterInputBuffer::new(PlayerId::P2),
      ],
      positions: vec![
        FighterCharacterPosition::new(PlayerId::P1,Vec3::new(-50.0, 0.0, 0.0)),
        FighterCharacterPosition::new(PlayerId::P2,Vec3::new(50.0, 0.0, 0.0)),
      ]
    }
  }
}

impl PlayerData {
  pub fn get_facing_vector(&self, player_id: &PlayerId) -> f32 {
    let p1_x_pos = self.positions[0].get_position().x;
    let p2_x_pos = self.positions[1].get_position().x;

    if p1_x_pos > p2_x_pos {
      if *player_id == PlayerId::P1 {
        return -1.0;
      } else {
        return 1.0;
      }
    } else {
      if *player_id == PlayerId::P1 {
        return 1.0;
      } else {
        return -1.0;
      }
    }
  }

  pub fn set_position(&mut self, player_id: &PlayerId, position: Vec3) {
    let i: usize = match player_id {
      PlayerId::P1 => 0,
      PlayerId::P2 => 1,
    };
    self.positions[i].set_position(position);
  }
}
pub struct FighterInputMapper {
  pub player_id: PlayerId,
  pub a: RawButton,
  pub b: RawButton,
  pub c: RawButton,
  pub d: RawButton,
  pub e: RawButton,
  pub f: RawButton,
  pub macro_1: RawButton,
  pub macro_2: RawButton,
  pub x_positive: RawButton,
  pub x_negative: RawButton,
  pub y_positive: RawButton,
  pub y_negative: RawButton,
}

impl FighterInputMapper {
    pub fn get_pressed_buttons(&self, keyboard_input: &Res<Input<KeyCode>>, button_input: &Res<Input<GamepadButton>>) -> InputActionsPressed {
      let right_pressed = match self.x_positive {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let left_pressed = match self.x_negative {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let down_pressed = match self.y_negative {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let up_pressed = match self.y_positive {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let a_pressed = match self.a {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let b_pressed = match self.b {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let c_pressed = match self.c {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let d_pressed = match self.d {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let e_pressed = match self.e {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let f_pressed = match self.f {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let macro_1_pressed = match self.macro_1 {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let macro_2_pressed = match self.macro_2 {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      return InputActionsPressed {
        a: a_pressed,
        b: b_pressed,
        c: c_pressed,
        d: d_pressed,
        e: e_pressed,
        f: f_pressed,
        macro_1: macro_1_pressed,
        macro_2: macro_2_pressed,
        right: right_pressed,
        left: left_pressed,
        up: up_pressed,
        down: down_pressed,
      }
    }

    pub fn get_just_pressed_buttons(&self, keyboard_input: &Res<Input<KeyCode>>, button_input: &Res<Input<GamepadButton>>) -> InputActionsPressed {
      let right_pressed = match self.x_positive {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let left_pressed = match self.x_negative {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let down_pressed = match self.y_positive {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let up_pressed = match self.y_negative {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let a_pressed = match self.a {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let b_pressed = match self.b {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let c_pressed = match self.c {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let d_pressed = match self.d {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let e_pressed = match self.e {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let f_pressed = match self.f {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let macro_1_pressed = match self.macro_1 {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      let macro_2_pressed = match self.macro_2 {
        RawButton::K(keycode) => keyboard_input.just_pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.just_pressed(GamepadButton(device_id, button_type)),
      };

      return InputActionsPressed {
        a: a_pressed,
        b: b_pressed,
        c: c_pressed,
        d: d_pressed,
        e: e_pressed,
        f: f_pressed,
        macro_1: macro_1_pressed,
        macro_2: macro_2_pressed,
        right: right_pressed,
        left: left_pressed,
        up: up_pressed,
        down: down_pressed,
      }
    }
}

pub struct InputActionsPressed {
  pub a: bool,
  pub b: bool,
  pub c: bool,
  pub d: bool,
  pub e: bool,
  pub f: bool,
  pub macro_1: bool,
  pub macro_2: bool,
  pub right: bool,
  pub left: bool,
  pub up: bool,
  pub down: bool,
}
pub enum RawButton {
  K(KeyCode),
  G(Gamepad,GamepadButtonType)
}
