pub use crate::prelude::*;

lazy_static! {
  pub static ref MOTIONS: [CommandMotion; 8] = [
    CommandMotion::new(
      1,
      Regex::new("([^6]+[69]{1,5}[^6]{0,9}5[^5]{0,4}6)").unwrap(),
      CommandType::DASH
    ),
    CommandMotion::new(
      1,
      Regex::new("([^4]+[47]{1,5}[^4]{0,9}5[^5]{0,4}4)").unwrap(),
      CommandType::BACK_DASH
    ),
    CommandMotion::new(
      2,
      Regex::new("(2[^2]{0,4}3[^3]{0,4}6)").unwrap(),
      CommandType::FIREBALL
    ),
    CommandMotion::new(
      2,
      Regex::new("(2[^2]{0,4}1[^1]{0,4}4)").unwrap(),
      CommandType::R_FIREBALL
    ),
    CommandMotion::new(
      3,
      Regex::new("(6[^6]{0,4}2[^2]{0,4}3)").unwrap(),
      CommandType::DP
    ),
    CommandMotion::new(
      3,
      Regex::new("(4[^4]{0,4}2[^2]{0,4}1)").unwrap(),
      CommandType::R_DP
    ),
    CommandMotion::new(
      4,
      Regex::new("(6[^6]{0,6}2[^2]{0,6}4)").unwrap(),
      CommandType::HALF_CIRCLE_BACK
    ),
    CommandMotion::new(
      4,
      Regex::new("(4[^4]{0,6}2[^2]{0,6}6)").unwrap(),
      CommandType::HALF_CIRCLE_FORWARD
    ),
  ];
}
