pub use crate::prelude::*;
pub struct PlayerMovement {
    grounded_dash: Option<Box<dyn Dash>>,
    grounded_backdash: Option<Box<dyn BackDash>>,
  }
  
  impl PlayerMovement {
    pub fn new(grounded_dash: impl Dash + 'static, grounded_backdash: impl BackDash + 'static) -> Self {
      PlayerMovement{
        grounded_dash: Some(Box::new(grounded_dash)),
        grounded_backdash: Some(Box::new(grounded_backdash)),
      }
    }
  }