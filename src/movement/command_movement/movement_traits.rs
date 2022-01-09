pub use crate::prelude::*;

pub trait Dash {
    fn exec(&self, player_movement: PlayerMovement);
    fn sustainable(&self) -> bool;
}

pub trait Backdash: Sync + Send  {
    fn exec(&self, facing_vector: f32) -> (InterpolatedForce, u8);
}
