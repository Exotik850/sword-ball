pub(crate) mod player;
mod weapon;

use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((player::plugin, PhysicsPlugins::default()))
        .insert_resource(Gravity(Vec2::ZERO));
}
