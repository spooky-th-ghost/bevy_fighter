use crate::prelude::*;

pub struct PlayerInputs {
  pub local_devices: Vec<InputMapper>,
  pub buffers: Vec<FighterInputBuffer>
}

impl Default for PlayerInputs {
  fn default() -> Self {
    PlayerInputs {
      local_devices: vec![
        InputMapper {
            player_id: 1,
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
            facing_right: false,
        },
          InputMapper {
            player_id: 2,
            a: RawButton::K(KeyCode::J),
            b: RawButton::K(KeyCode::K),
            c: RawButton::K(KeyCode::L),
            d: RawButton::K(KeyCode::N),
            e: RawButton::K(KeyCode::J),
            f: RawButton::K(KeyCode::K),
            macro_1: RawButton::K(KeyCode::L),
            macro_2: RawButton::K(KeyCode::N),
            x_positive: RawButton::K(KeyCode::D),
            x_negative: RawButton::K(KeyCode::A),
            y_positive: RawButton::K(KeyCode::V),
            y_negative: RawButton::K(KeyCode::S),
            facing_right: false,
        },
      ],
      buffers: vec![
        FighterInputBuffer::new(1),
        FighterInputBuffer::new(2),
      ]
    }
  }
}

pub struct InputMapper {
  pub player_id: u8,
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
  pub facing_right: bool
}

impl InputMapper {
    pub fn get_facing_vector(&self) -> f32 {
       if self.facing_right {
        return 1.0;
      } else {
        return -1.0;
      }
    }

    pub fn get_pressed_buttons(&self, keyboard_input: &Res<Input<KeyCode>>, button_input: &Res<Input<GamepadButton>>) -> InputActionsPressed {
      let right_pressed = match self.x_positive {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let left_pressed = match self.x_negative {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let down_pressed = match self.y_positive {
        RawButton::K(keycode) => keyboard_input.pressed(keycode),
        RawButton::G(device_id,button_type) => button_input.pressed(GamepadButton(device_id, button_type)),
      };

      let up_pressed = match self.y_negative {
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

