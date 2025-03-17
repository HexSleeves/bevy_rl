use bevy::prelude::*;

use crate::model::{
    commands::TryMove,
    components::{AIControlled, AwaitingInput, MoveDirection, Player, TurnActor},
    resources::{ActionQueue, ActionType, TurnSystem},
};

pub fn process_turns(
    mut commands: Commands,
    mut turn_system: ResMut<TurnSystem>,
    mut action_queue: ResMut<ActionQueue>,
    awaiting_input: Query<Entity, With<AwaitingInput>>,
    actor_query: Query<(Entity, Option<&Player>, Option<&AIControlled>), With<TurnActor>>,
) {
    // Don't process new turns if:
    // 1. Actions are still being processed
    // 2. Waiting for player input
    // 3. Waiting for animations
    if !action_queue.is_empty()
        || !awaiting_input.is_empty()
        || action_queue.is_waiting_for_animation()
    {
        return;
    }

    log::info!("Processing turns");
    turn_system.print_queue();

    // Process next turn
    if let Some((entity, _time)) = turn_system.get_next_actor() {
        if let Ok((entity, player, ai)) = actor_query.get(entity) {
            if player.is_some() {
                // Player's turn - wait for input
                commands.entity(entity).insert(AwaitingInput);
                log::info!("Player's turn");
            } else if ai.is_some() {
                // AI's turn - decide action immediately
                let action = decide_ai_action(entity, &actor_query);
                action_queue.push(entity, action);
                log::info!("AI's turn");
            }
        }
    }
}

// Helper function for AI decision making
fn decide_ai_action(
    _entity: Entity,
    _query: &Query<(Entity, Option<&Player>, Option<&AIControlled>), With<TurnActor>>,
) -> ActionType {
    // Simple AI logic - for illustration
    let random_dir = match fastrand::i32(0..4) {
        0 => MoveDirection::North,
        1 => MoveDirection::East,
        2 => MoveDirection::South,
        _ => MoveDirection::West,
    };

    ActionType::Move(random_dir)
}

pub fn process_action_queue(
    mut commands: Commands,
    mut action_queue: ResMut<ActionQueue>,
    mut turn_system: ResMut<TurnSystem>,
    actor_query: Query<(Entity, &TurnActor)>,
    // collision_query: Query<(Entity, &Position)>,
) {
    // Don't process if waiting for animations
    if action_queue.is_waiting_for_animation() {
        return;
    }

    // Make sure all actors have actions queued before processing
    let actor_count = actor_query.iter().count();
    if action_queue.is_empty() {
        // if action_queue.is_empty() || action_queue.len() < actor_count {
        return;
    }

    log::info!("Processing action queue");
    action_queue.print_queue();

    // Process next action
    while let Some(action) = action_queue.next_action() {
        log::info!("Processing action: {:?}", action);
        if let Ok((entity, turn_actor)) = actor_query.get(action.entity) {
            match action.action_type {
                ActionType::Move(dir) => {
                    commands.entity(entity).queue(TryMove::new(dir));

                    // Schedule entity's next turn (only after completing action)
                    turn_system.schedule_turn(entity, turn_actor.speed);
                }
                ActionType::Attack(_target) => {
                    // Handle attack logic...

                    // Schedule next turn
                    turn_system.schedule_turn(entity, turn_actor.speed * 2);
                }
                // Other action types...
                _ => {
                    // Default fallback
                    turn_system.schedule_turn(entity, turn_actor.speed);
                }
            }
        }
    }
}
