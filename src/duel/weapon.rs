//! Weapon logic and components
//!
//! Weapons are represented by an object attached to an entity that can swing around and hit enemies
//! Here are the different ways a weapon can be used:
//! - Melee: A weapon that can hit enemies in close range
//!     - Flail: Attached to a chain and can be swung around
//!     - Sword: A sharp blade that can slash enemies
//!     - Hammer: A heavy weapon that can crush enemies
//! - Ranged: A weapon that can shoot projectiles at enemies from a distance

use bevy::prelude::*;

#[derive(Component)]
pub struct Weapon;

// pub spawn_weapon
