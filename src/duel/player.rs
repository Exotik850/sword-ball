use std::thread::spawn;

use avian2d::prelude::*;
use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use leafwing_input_manager::prelude::*;

use crate::{duel::weapon::spawn_weapon, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
        // .add_systems(Update, spawn_player)
        .add_systems(
            Update,
            (
                handle_gamepad_connection,
                handle_inputs,
                wrap_player_position,
            ),
        )
        .add_systems(
            OnEnter(Screen::Gameplay),
            (spawn_player, setup_gamepads).chain(),
        );
}

#[derive(Component)]
pub struct PlayerID(usize);

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component)]
#[relationship(relationship_target = AssignedPlayer)]
struct AssignedGamepad(Entity);

#[derive(Component)]
#[relationship_target(relationship = AssignedGamepad)]
struct AssignedPlayer(Entity);

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Dash,
    Gaurd,
}

fn default_input_map() -> InputMap<PlayerAction> {
    let mut input_map = InputMap::default();

    input_map.insert_dual_axis(PlayerAction::Move, GamepadStick::LEFT);
    input_map.insert(PlayerAction::Dash, GamepadButton::South);
    input_map.insert(PlayerAction::Gaurd, GamepadButton::RightTrigger);

    input_map
}

fn player(id: usize) -> impl Bundle {
    (PlayerID(id), default_input_map(), Speed(5000.))
}

pub(crate) fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player = commands
        .spawn((
            player(0),
            RigidBody::Dynamic,
            Collider::circle(15.),
            Transform::default(),
            Mesh2d(meshes.add(Mesh::from(Circle::new(15.)))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.7, 0.9)))),
        ))
        .id();
    // spawn_weapon(&mut commands, &mut meshes, &mut materials, player);
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

fn handle_inputs(mut query: Query<(&ActionState<PlayerAction>, Forces, &Speed), With<PlayerID>>) {
    for (action_state, mut forces, Speed(speed)) in query.iter_mut() {
        let movement = action_state.axis_pair(&PlayerAction::Move);
        forces.apply_force(movement * *speed);

        if action_state.just_pressed(&PlayerAction::Dash) {
            forces.apply_linear_impulse(movement * (*speed * 5.0));
            println!("Player is dashing!");
        }

        if action_state.just_pressed(&PlayerAction::Gaurd) {
            println!("Player is guarding!");
        }
    }
}

fn setup_gamepads(
    mut commands: Commands,
    mut gamepads: Query<Entity, With<Gamepad>>,
    mut without_gamepad: Query<(Entity, &mut InputMap<PlayerAction>), Without<AssignedGamepad>>,
) {
    // Assign gamepads to players without gamepads
    for (entity, mut input_map) in without_gamepad.iter_mut() {
        if let Some(gamepad_entity) = gamepads.iter_mut().next() {
            commands
                .entity(entity)
                .insert(AssignedGamepad(gamepad_entity));
            input_map.set_gamepad(gamepad_entity);
            println!(
                "Assigned gamepad {:?} to player entity {:?} on setup",
                gamepad_entity, entity
            );
        }
    }
}

fn handle_gamepad_connection(
    mut commands: Commands,
    mut ev: MessageReader<GamepadConnectionEvent>,
    mut q: Query<(
        Entity,
        &mut InputMap<PlayerAction>,
        Option<&AssignedGamepad>,
    )>,
) {
    use bevy::input::gamepad::GamepadConnection;
    for event in ev.read() {
        match &event.connection {
            GamepadConnection::Connected {
                name,
                vendor_id,
                product_id,
            } => {
                println!(
                    "Gamepad connected: {:?}, name: {}, vendor_id: {:?}, product_id: {:?}",
                    event.gamepad, name, vendor_id, product_id
                );
                let Some((entity, mut input_map, gamepad)) = q.iter_mut().next() else {
                    println!("No available player entities to assign the gamepad to.");
                    continue;
                };
                if gamepad.is_some() {
                    continue;
                }
                commands
                    .entity(entity)
                    .insert(AssignedGamepad(event.gamepad));
                input_map.set_gamepad(event.gamepad);
                println!(
                    "Assigned gamepad {:?} to player entity {:?}",
                    event.gamepad, entity
                );
            }
            GamepadConnection::Disconnected => {
                let Some((entity, mut input_map, _)) = q
                    .iter_mut()
                    .find(|(_, input_map, _)| input_map.gamepad() == Some(event.gamepad))
                else {
                    continue;
                };
                commands.entity(entity).remove::<AssignedGamepad>();
                input_map.clear_gamepad();
                println!("Removed gamepad assignment from player entity {:?}", entity);
            }
        }
    }
}
