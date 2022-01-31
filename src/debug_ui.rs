use crate::prelude::*;

pub trait SpawnDebugUi {
  fn spawn_debug_ui(&mut self, player_id: PlayerId,  font: &Handle<Font>);
}

impl SpawnDebugUi for Commands<'_,'_> {
  fn spawn_debug_ui(&mut self, player_id: PlayerId, font: &Handle<Font>) {
    let flex = match player_id {
      PlayerId::P1 => AlignSelf::FlexStart,
      PlayerId::P2 => AlignSelf::FlexEnd
    };
      let font_size = 20.0;
      self
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: flex,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Action State: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: "Busy: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: "Is Grounded: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: "Air Dashes: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: "Airdash Lockout: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: "Velocity: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size,
                            color: Color::BLACK,
                        },
                    },
                    TextSection {
                        value: "Airdash Time: ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size,
                            color: Color::BLACK,
                        },
                    },
                ],
                alignment: TextAlignment {
                  vertical: VerticalAlign::Top,
                  horizontal: HorizontalAlign::Left,
                }
            },
            ..Default::default()
        })
        .insert(player_id);
  }
}
