use bevy_fighter::prelude::*;

#[test]
fn countdown_from_0() {
  assert_eq!(countdown(0), 0);
}

#[test]
fn countdown_from_10() {
  assert_eq!(countdown(10), 9);
}
