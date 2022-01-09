use crate::prelude::*;

// struct PlayerData {
//   health: u8,
//   position: Vec2,
//   action_state: ActionState,
//   physics_state: PhysicsState,
//   round_count: u8,
//   meter: u8,
//   dash: impl Dash,
//   backsash: impl BackDash,
// }

pub struct Player;
#[derive(Component)]
pub struct PlayerId(pub u8);
