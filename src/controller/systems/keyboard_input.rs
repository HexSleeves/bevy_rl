use bevy::{platform::collections::HashMap, prelude::*};
use once_cell::sync::Lazy;

use crate::model::{
    actions::Walk,
    components::{AwaitingInput, TurnActor},
    types::{ActionType, BuildableGameAction, GameActionBuilder, MoveDirection},
    GameState,
};
#[macro_export]
macro_rules! action_keys {
    ($(($action:expr, [$($key:expr),*])),*) => {
        Lazy::new(|| {
            HashMap::from([
                $(($action, vec![$($key),*])),*
            ])
        })
    }
}

static ACTION_KEYS: Lazy<HashMap<ActionType, Vec<KeyCode>>> = action_keys![
    (ActionType::Move(MoveDirection::North), [KeyCode::KeyW, KeyCode::ArrowUp]),
    (ActionType::Move(MoveDirection::South), [KeyCode::KeyS, KeyCode::ArrowDown]),
    (ActionType::Move(MoveDirection::West), [KeyCode::KeyA, KeyCode::ArrowLeft]),
    (ActionType::Move(MoveDirection::East), [KeyCode::KeyD, KeyCode::ArrowRight]),
    (ActionType::Wait, [KeyCode::Space, KeyCode::Period])
];

/// System that handles player input and converts it into game actions
pub fn player_input_system(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    q_awaiting_input: Option<Single<(Entity, &mut TurnActor), With<AwaitingInput>>>,
) {
    if let Some(a) = q_awaiting_input {
        let (entity, mut p_actor) = a.into_inner();
        let mut action: Option<ActionType> = None;

        for (act, keys) in ACTION_KEYS.iter() {
            if keys.iter().any(|key| input.just_pressed(*key)) {
                log::info!("Player input: {:?}", act);
                action = Some(*act);
                break;
            }
        }

        if let Some(act) = &action {
            if let ActionType::Move(dir) = act {
                p_actor.add_action(Walk::builder().with_entity(entity).with_direction((*dir).into()).build());
            }

            commands.entity(entity).remove::<AwaitingInput>();
            next_state.set(GameState::MonstersTurn);
        }
    }
}
