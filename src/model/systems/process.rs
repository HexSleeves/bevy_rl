use bevy::prelude::*;

use crate::model::components::{Action, Enemy, Player, Position, TurnActor, WaitingForInput};
use crate::model::resources::TurnQueue;

pub fn process_turns(
    mut commands: Commands,
    mut turn_queue: ResMut<TurnQueue>,
    awaiting_input: Query<Entity, With<WaitingForInput>>,
    mut turn_actors: Query<(Entity, &mut TurnActor, Option<&Player>, Option<&Enemy>)>,
) {
    // Don't process turns if player is awaiting input or system is paused
    if !awaiting_input.is_empty() || !turn_queue.processing {
        return;
    }

    // Find actor with earliest next turn
    let mut next_actor = None;
    let mut earliest_time = u64::MAX;

    for (entity, turn_actor, _, _) in turn_actors.iter() {
        if turn_actor.next_turn_time < earliest_time {
            earliest_time = turn_actor.next_turn_time;
            next_actor = Some(entity);
        }
    }

    // Process next actor's turn
    if let Some(entity) = next_actor {
        // Update global time
        turn_queue.current_time = earliest_time;

        // Check if player or enemy
        let (_, mut turn_actor, player, enemy) = turn_actors.get_mut(entity).unwrap();

        if player.is_some() {
            // Handle player turn - await input
            commands.entity(entity).insert(WaitingForInput);
        } else if enemy.is_some() {
            // Handle enemy turn - AI decides action
            // For example, move towards player
            // commands
            //     .entity(entity)
            //     .insert(Action::Move(IVec2::new(0, 1)));

            // Schedule next turn
            turn_actor.next_turn_time = turn_queue.current_time + turn_actor.speed as u64;
        } else {
            // Handle other turn types
            // For example, move towards player
            // Schedule next turn
        }
    }
}

pub fn execute_actions(
    mut commands: Commands,
    mut query: Query<(Entity, &Action)>,
    mut position_query: Query<&mut Position>,
    // Other queries for game state
) {
    for (entity, action) in &mut query {
        match action {
            Action::Move(direction) => {
                if let Ok(mut position) = position_query.get_mut(entity) {
                    // Update position if valid
                    *position = *position + *direction;
                }
            }
            Action::Wait => {
                // Do nothing
            }
            Action::Attack(_target) => {
                // Handle attack logic
            }
            _ => {
                // Handle other actions
            }
        }

        // Remove the action once processed
        commands.entity(entity).remove::<Action>();
    }
}
