use avian2d::prelude::*;
use bevy::{input::gamepad::GamepadConnectionEvent, prelude::*};
use leafwing_input_manager::prelude::*;

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
        );
}

#[derive(Component)]
pub struct PlayerID(usize);

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component)]
struct AssignedGamepad(Option<Entity>);

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
    (
        PlayerID(id),
        AssignedGamepad(None),
        default_input_map(),
        Speed(5000.),
    )
}

pub(crate) fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        player(0),
        RigidBody::Dynamic,
        Collider::circle(15.),
        Transform::default(),
        Mesh2d(meshes.add(Mesh::from(Circle::new(15.)))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.3, 0.7, 0.9)))),
    ));
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

fn handle_gamepad_connection(
    mut ev: MessageReader<GamepadConnectionEvent>,
    mut q: Query<(&mut AssignedGamepad, &mut InputMap<PlayerAction>), With<PlayerID>>,
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
                for (mut assigned_gamepad, mut input_map) in q.iter_mut() {
                    if assigned_gamepad.0.is_none() {
                        assigned_gamepad.0 = Some(event.gamepad);
                        *input_map = default_input_map();
                        println!("Assigned gamepad {:?} to player", event.gamepad);
                        break;
                    }
                }
            }
            GamepadConnection::Disconnected => {
                println!("Gamepad disconnected: {:?}", event.gamepad);
                for (mut assigned_gamepad, _) in q.iter_mut() {
                    if assigned_gamepad.0 == Some(event.gamepad) {
                        assigned_gamepad.0 = None;
                        println!("Unassigned gamepad {:?} from player", event.gamepad);
                        break;
                    }
                }
            }
        }
    }
}
