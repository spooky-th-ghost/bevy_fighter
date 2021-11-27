pub use crate::prelude::*;

pub trait Dash {
    fn exec(&self, player_state: ActionState, physics_state: PhysicsState) -> (ActionState, PhysicsState);
    fn sustainable(&self) -> bool;
}

pub trait BackDash {
    fn exec(&self, player_state: ActionState, physics_state: PhysicsState) -> (ActionState, PhysicsState);
}