use crate::prelude::*;

pub fn countdown(val: u8) -> u8 {
    if val > 0 {
        return val - 1;
    } else {
        return 0;
    }
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
pub enum FighterSystemLabels {
    InitializeCharacterData,
    Setup,
    InputWrite,
    InputRead,
    StatusUpdate,
    PhysicsUpdate,
    PhysicsExecute,
    AnimationUpdate,
    AnimationExecute
}

trait MotionGroups {
    fn y_negative(&self) -> bool;
    fn y_positive(&self) -> bool;
    fn x_positive(&self) -> bool;
    fn x_negative(&self) -> bool;
}

impl MotionGroups for u8 {
    fn x_positive(&self) -> bool {
        match self {
            6 | 3 | 9 => true,
            _ => false,
        }
    }

    fn y_positive(&self) -> bool {
        match self {
            7 | 8 | 9 => true,
            _ => false,
        }
    }

    fn x_negative(&self) -> bool {
        match self {
            4 | 1 | 7 => true,
            _ => false,
        }
    }

    fn y_negative(&self) -> bool {
        match self {
            1 | 2 | 3 => true,
            _ => false,
        }
    }
}
