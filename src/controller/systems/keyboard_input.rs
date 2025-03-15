use bevy::prelude::*;
// use once_cell::sync::Lazy;

use crate::model::components::*;

// static ACTION_KEYS: Lazy<HashMap<PlayerAction, Vec<KeyCode>>> = Lazy::new(|| {
//     HashMap::from([
//         (
//             PlayerAction::Move(MoveDirection::North),
//             vec![KeyCode::KeyW, KeyCode::ArrowUp],
//         ),
//         (
//             PlayerAction::Move(MoveDirection::South),
//             vec![KeyCode::KeyS, KeyCode::ArrowDown],
//         ),
//         (
//             PlayerAction::Move(MoveDirection::West),
//             vec![KeyCode::KeyA, KeyCode::ArrowLeft],
//         ),
//         (
//             PlayerAction::Move(MoveDirection::East),
//             vec![KeyCode::KeyD, KeyCode::ArrowRight],
//         ),
//         (PlayerAction::Wait, vec![KeyCode::Space, KeyCode::Period]),
//         (
//             PlayerAction::PickupItem,
//             vec![KeyCode::KeyG, KeyCode::Comma],
//         ),
//     ])
// });

// pub fn keyboard_input(
//     mut commands: Commands,
//     button_input: Res<ButtonInput<KeyCode>>,
//     player: Single<(Entity, &Position), With<Player>>,
// ) {
//     commands.entity(player_entity).queue(TryMove::new(dir));
// }

// System to detect when it's the player's turn and await input
pub fn player_input_system(
    mut commands: Commands,
    turn_queue: Res<TurnQueue>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut TurnActor), (With<AwaitingInput>, With<Player>)>,
) {
    if let Ok((entity, mut turn_actor)) = query.get_single_mut() {
        let mut action = None;

        // Handle movement input
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            action = Some(Action::Move(IVec2::new(0, 1)));
        } else if keyboard_input.just_pressed(KeyCode::KeyS) {
            action = Some(Action::Move(IVec2::new(0, -1)));
        } else if keyboard_input.just_pressed(KeyCode::KeyA) {
            action = Some(Action::Move(IVec2::new(-1, 0)));
        } else if keyboard_input.just_pressed(KeyCode::KeyD) {
            action = Some(Action::Move(IVec2::new(1, 0)));
        }

        if let Some(act) = action {
            log::info!("Player action: {:?}", act);

            // Remove awaiting input
            commands.entity(entity).remove::<AwaitingInput>();

            // Schedule next turn based on action type (simplified)
            let time_cost = match act {
                Action::Move(_) => turn_actor.speed,
                Action::Wait => turn_actor.speed / 2,
                _ => turn_actor.speed,
            };

            // Add action
            commands.entity(entity).insert(act);
            turn_actor.next_turn_time = turn_queue.current_time + time_cost as u64;
        }
    }
}
