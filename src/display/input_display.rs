use crate::prelude::*;

#[derive(Default, Component)]
pub struct InputRow {
  pub motion: u8,
  pub duration: u8,
  pub birth: f64,
}

pub struct InputDisplay;


// pub fn add_input_row(parent_entity: Entity, arrow_texture: Handle<Texture>, mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>,) {

//   let child = commands.spawn_bundle(InputRowBundle {
//       motion: 5,
//       duration: 0,
//       image: ImageBundle {
//         style: Style {
//           size: Size::new(Val::Px(50.0), Val::Auto),
//           position_type: PositionType::Relative,
//           ..Default::default()
//         },
//         material: materials
//           .add(arrow_texture.into()),
//           ..Default::default()
//       }
//     }).id();
//   commands.entity(parent_entity).push_children(&[child]);
// }

