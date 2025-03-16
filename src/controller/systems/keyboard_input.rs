use bevy::{prelude::*, utils::HashMap};
use once_cell::sync::Lazy;

use crate::model::{
    components::{Action, MoveDirection, Player, TurnActor, WaitingForInput},
    resources::TurnSystem,
};

/// Static mapping of input actions to their corresponding keyboard keys
static ACTION_KEYS: Lazy<HashMap<Action, Vec<KeyCode>>> = Lazy::new(|| {
    HashMap::from([
        (
            Action::Move(MoveDirection::North),
            vec![KeyCode::KeyW, KeyCode::ArrowUp],
        ),
        (
            Action::Move(MoveDirection::South),
            vec![KeyCode::KeyS, KeyCode::ArrowDown],
        ),
        (
            Action::Move(MoveDirection::West),
            vec![KeyCode::KeyA, KeyCode::ArrowLeft],
        ),
        (
            Action::Move(MoveDirection::East),
            vec![KeyCode::KeyD, KeyCode::ArrowRight],
        ),
        (Action::Wait, vec![KeyCode::Space, KeyCode::Period]),
        (Action::PickupItem, vec![KeyCode::KeyG, KeyCode::Comma]),
    ])
});

/// System that handles player input and converts it into game actions
pub fn player_input_system(
    mut commands: Commands,
    mut turn_system: ResMut<TurnSystem>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &TurnActor), (With<WaitingForInput>, With<Player>)>,
) {
    if let Ok((entity, turn_actor)) = query.get_single() {
        let mut player_action: Option<Action> = None;

        // Check predefined action keys
        for (action, keys) in ACTION_KEYS.iter() {
            if keys.iter().any(|key| keyboard_input.just_pressed(*key)) {
                player_action = Some(*action);
                break;
            }
        }

        if let Some(input_action) = player_action {
            log::info!("Player action: {:?}", input_action);

            // Remove awaiting input
            commands.entity(entity).remove::<WaitingForInput>();

            // Schedule next turn based on action type
            let time_cost = match input_action {
                Action::Move(_) => turn_actor.speed,
                Action::Wait => turn_actor.speed / 2,
                _ => turn_actor.speed,
            };

            // Calculate next turn time
            let next_turn_time = turn_actor.next_turn_time + time_cost;
            turn_system.schedule_turn(entity, next_turn_time);

            // Add action as a component
            commands.entity(entity).insert(input_action);
        }
    }
}
