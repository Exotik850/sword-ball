//! Weapon logic and components
//!
//! Weapons are represented by an object attached to an entity that can swing around and hit enemies
//! Here are the different ways a weapon can be used:
//! - Melee: A weapon that can hit enemies in close range
//!     - Flail: Attached to a chain and can be swung around
//!     - Sword: A sharp blade that can slash enemies
//!     - Hammer: A heavy weapon that can crush enemies
//! - Ranged: A weapon that can shoot projectiles at enemies from a distance

use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Weapon;

fn weapon_joint(player: Entity, weapon: Entity) -> impl Bundle {
    DistanceJoint::new(player, weapon).with_limits(10., 50.)
}

fn weapon(commands: &mut Commands, player: Entity) -> Entity {
    let weapon = commands.spawn(Weapon).id();
    commands.spawn(weapon_joint(player, weapon));
    weapon
}

pub(crate) fn spawn_weapon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    player: Entity,
) {
    let weapon_entity = weapon(commands, player.entity());
    commands.entity(weapon_entity).insert((
        Mesh2d(meshes.add(Mesh::from(Circle::new(5.)))),
        MeshMaterial2d::from(materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)))),
        Collider::circle(5.),
        RigidBody::Dynamic,
        Mass(0.02),
        Name::new("Player Weapon"),
        Transform::from_xyz(50., 0., 0.),
    ));
}
