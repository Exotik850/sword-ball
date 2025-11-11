use crate::screens::Screen;

use super::player::{PlayerID, Speed};
use avian2d::prelude::*;
use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .add_systems(Update, handle_inputs)
        .add_systems(OnEnter(Screen::Gameplay), setup_gamepads)
        .add_systems(
            Update,
            handle_gamepad_connection
                .run_if(in_state(Screen::Gameplay).and(on_message::<GamepadConnectionEvent>)),
        );
}

#[derive(Component)]
#[relationship(relationship_target = AssignedPlayer)]
struct AssignedGamepad(Entity);

#[derive(Component)]
#[relationship_target(relationship = AssignedGamepad)]
struct AssignedPlayer(Entity);

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    Dash,
    Gaurd,
}

pub fn default_input_map() -> InputMap<PlayerAction> {
    let mut input_map = InputMap::default();

    // Gamepad
    input_map.insert_dual_axis(PlayerAction::Move, GamepadStick::LEFT);
    input_map.insert(PlayerAction::Dash, GamepadButton::South);
    input_map.insert(PlayerAction::Gaurd, GamepadButton::RightTrigger);

    // Keyboard
    input_map.insert_dual_axis(PlayerAction::Move, VirtualDPad::wasd());
    input_map.insert(PlayerAction::Dash, KeyCode::ShiftLeft);
    input_map.insert(PlayerAction::Gaurd, KeyCode::Space);

    input_map
}

fn handle_inputs(mut query: Query<(&ActionState<PlayerAction>, Forces, &Speed), With<PlayerID>>) {
    for (action_state, mut forces, Speed(speed)) in query.iter_mut() {
        let movement = action_state.axis_pair(&PlayerAction::Move);
        let velocity = forces.linear_velocity();

        // if we are changing direction, apply an impulse to quickly change direction
        // if movement
        //     .normalize_or_zero()
        //     .dot(velocity.normalize_or_zero())
        //     < -0.7
        // {
        //     let impulse = -velocity.normalize_or_zero() * (*speed * 0.1);
        //     forces.apply_linear_impulse(impulse);
        // }
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
