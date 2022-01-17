use crate::prelude::*;

pub struct PlayerInputs {
  pub local_devices: Vec<InputMapper>
}

impl Default for PlayerInputs {
  fn default() -> Self {
    PlayerInputs {
      local_devices: vec![
        InputMapper {
            player_id: 1,
            a: KeyCode::U,
            b: KeyCode::I,
            c: KeyCode::O,
            d: KeyCode::H,
            x_positive: KeyCode::E,
            x_negative: KeyCode::Q,
            y_positive: KeyCode::Space,
            y_negative: KeyCode::W,
            facing_right: false,
            device_type: InputDeviceType::Keyboard
        },
          InputMapper {
            player_id: 2,
            a: KeyCode::J,
            b: KeyCode::K,
            c: KeyCode::L,
            d: KeyCode::N,
            x_positive: KeyCode::D,
            x_negative: KeyCode::A,
            y_positive: KeyCode::V,
            y_negative: KeyCode::S,
            facing_right: false,
            device_type: InputDeviceType::Keyboard
        },
      ]
    }
  }
}

/// Implementing from world can be used later to read a config file and map saved inputs as the resource is instantiated
// impl FromWorld for InputDevices {
//     fn from_world(_world: &mut World) -> Self {
//       InputDevices {
//         devices: vec! [

//         ]
//       }
//     }
// }

pub struct InputMapper {
  pub player_id: u8,
  pub a: KeyCode,
  pub b: KeyCode,
  pub c: KeyCode,
  pub d: KeyCode,
  pub x_positive: KeyCode,
  pub x_negative: KeyCode,
  pub y_positive: KeyCode,
  pub y_negative: KeyCode,
  pub facing_right: bool,
  pub device_type: InputDeviceType
}

impl InputMapper {
    pub fn get_facing_vector(&self) -> f32 {
       if self.facing_right {
        return 1.0;
      } else {
        return -1.0;
      }
    }
}

pub enum InputDeviceType {
  Keyboard,
  Gamepad
}

