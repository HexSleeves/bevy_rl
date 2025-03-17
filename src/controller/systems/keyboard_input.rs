use bevy::{prelude::*, utils::HashMap};
use once_cell::sync::Lazy;

use crate::model::{
    components::{AwaitingInput, MoveDirection},
    resources::{ActionQueue, ActionType},
};

/// Static mapping of input actions to their corresponding keyboard keys
static ACTION_KEYS: Lazy<HashMap<ActionType, Vec<KeyCode>>> = Lazy::new(|| {
    HashMap::from([
        (
            ActionType::Move(MoveDirection::North),
            vec![KeyCode::KeyW, KeyCode::ArrowUp],
        ),
        (
            ActionType::Move(MoveDirection::South),
            vec![KeyCode::KeyS, KeyCode::ArrowDown],
        ),
        (
            ActionType::Move(MoveDirection::West),
            vec![KeyCode::KeyA, KeyCode::ArrowLeft],
        ),
        (
            ActionType::Move(MoveDirection::East),
            vec![KeyCode::KeyD, KeyCode::ArrowRight],
        ),
        (ActionType::Wait, vec![KeyCode::Space, KeyCode::Period]),
    ])
});

/// System that handles player input and converts it into game actions
pub fn player_input_system(
    mut commands: Commands,
    mut action_queue: ResMut<ActionQueue>,
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<AwaitingInput>>,
) {
    if let Ok(entity) = query.get_single() {
        let mut action: Option<ActionType> = None;

        for (act, keys) in ACTION_KEYS.iter() {
            if keys.iter().any(|key| input.just_pressed(*key)) {
                action = Some(*act);
                break;
            }
        }

        if let Some(act) = &action {
            // Remove awaiting input
            commands.entity(entity).remove::<AwaitingInput>();

            // Add action to queue
            action_queue.push(entity, *act);

            log::info!("Player input: {:?}", act);
        }
    }
}
