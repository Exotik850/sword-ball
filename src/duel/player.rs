use std::thread::spawn;

use avian2d::prelude::*;
use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::{
    duel::{control::default_input_map, dynamic_obj, weapon::spawn_weapon},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_player);
}

#[derive(Component)]
pub struct PlayerID(usize);

#[derive(Component)]
pub struct Speed(pub f32);

pub(crate) fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player = commands
        .spawn((
            PlayerID(0),
            Speed(75000.),
            MaxLinearSpeed(1000.),
            Transform::default(),
            Mesh2d(meshes.add(Mesh::from(Circle::new(15.)))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.7, 0.9)))),
            default_input_map(),
            dynamic_obj(15.),
        ))
        .id();
    spawn_weapon(&mut commands, &mut meshes, &mut materials, player);
}

fn wrap_player_position(mut query: Query<&mut Transform, With<PlayerID>>, windows: Query<&Window>) {
    let window = windows.single().unwrap();
    let size = window.size();
    for mut transform in query.iter_mut() {
        if transform.translation.x > size.x / 2.0 {
            transform.translation.x = -size.x / 2.0;
        } else if transform.translation.x < -size.x / 2.0 {
            transform.translation.x = size.x / 2.0;
        }

        if transform.translation.y > size.y / 2.0 {
            transform.translation.y = -size.y / 2.0;
        } else if transform.translation.y < -size.y / 2.0 {
            transform.translation.y = size.y / 2.0;
        }
    }
}
