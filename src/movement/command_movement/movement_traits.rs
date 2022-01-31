pub use crate::prelude::*;

pub trait Backdash: Sync + Send  {
    fn exec(&self, facing_vector: f32) -> (InterpolatedForce, u8);
}

pub trait Airdash: Sync + Send {
    fn exec_forward(&self, facing_vector: f32) -> (InterpolatedForce, u8);
    fn exec_backwards(&self, facing_vector: f32) -> (InterpolatedForce, u8);
}
