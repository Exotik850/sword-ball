mod control;
mod player;
mod weapon;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, control::plugin, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec2::NEG_Y * 80.));
}

pub fn dynamic_obj(radius: f32) -> impl Bundle {
    (
        RigidBody::Dynamic,
        Collider::circle(radius),
        Friction::new(0.7),
        Restitution::new(0.2),
        TransformExtrapolation,
        TransformHermiteEasing,
        DespawnOnExit(Screen::Gameplay),
    )
}
