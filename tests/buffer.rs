use bevy_fighter::{
    constants::MOTIONS
};


#[test]
fn dash_command_motion() {
    assert!(MOTIONS[0].check("5556665566666",0));
}

#[test]
fn backdash_dash_command_motion() {
    assert!(MOTIONS[1].check("55544455444",0));
}
#[test]
fn fireball_command_motion() {
    assert!(MOTIONS[2].check("55522222333366",1));
}

#[test]
fn rev_fireball_command_motion() {
    assert!(MOTIONS[3].check("5552222211144",1));
}

#[test]
fn dp_command_motion() {
    assert!(MOTIONS[4].check("555662223333",2));
}

#[test]
fn rev_dp_command_motion() {
    assert!(MOTIONS[5].check("554442221111",2));
}

#[test]
fn hcb_command_motion() {
    assert!(MOTIONS[6].check("5566322211144",3));
}

#[test]
fn hcf_command_motion() {
    assert!(MOTIONS[7].check("5544122233366",3));
}
