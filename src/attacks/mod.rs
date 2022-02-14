#[path = "attack.components.rs"]
pub mod attack_components;

#[path = "attack.systems.rs"]
pub mod attack_systems;

#[doc(hidden)]
pub use attack_components::*;
#[doc(hidden)]
pub use attack_systems::*;

