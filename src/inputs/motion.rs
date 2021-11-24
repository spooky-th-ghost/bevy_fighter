pub use crate::*;

#[derive(Debug)]
pub struct MotionEvent{
  pub motion: u8,
  pub player_id: u8,
  pub special_motion: Option<SpecialMotion>
}

#[derive(Debug)]
pub enum SpecialMotion {
    FIREBALL,
    R_FIREBALL,
    DP,
    R_DP,
    HALF_CIRCLE_BACK,
    HALF_CIRCLE_FORWARD,
    DASH,
    BACK_DASH,
    INVITE_HELL
}

