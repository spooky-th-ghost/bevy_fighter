pub use crate::prelude::*;

lazy_static! {
  static ref BLAH: u8 = 4;
  pub static ref MOTIONS: [SpecialMotionData; 1] = [
    SpecialMotionData::new(
      String::from("236"),
      1,
      Regex::new("(2[^2]{0,4}3[^3]{0,4}6)").unwrap(),
      SpecialMotion::FIREBALL
    )
  ];
}
