pub use crate::prelude::*;

pub trait Dash {
    fn exec(&self, player_movement: PlayerMovement);
    fn sustainable(&self) -> bool;
}

pub trait BackDash {
    fn exec(&self, player_movement: PlayerMovement);
}
