use bevy::prelude::*;

use crate::model::commands::TryMove;
use crate::model::components::{
    Action, Actor, MoveDirection, Player, Position, TurnActor, WaitingForInput,
};
use crate::model::resources::TurnSystem;

pub fn process_turns(
    mut commands: Commands,
    mut turn_system: ResMut<TurnSystem>,
    awaiting_input: Query<Entity, With<WaitingForInput>>,
    actor_query: Query<(Entity, &TurnActor, Option<&Player>, Option<&Actor>)>,
) {
    // Don't process turns if waiting for input
    if !awaiting_input.is_empty() {
        return;
    }

    // Process next turn if available
    if let Some((entity, _)) = turn_system.get_next_actor() {
        // Find the actor's data
        if let Ok((entity, turn_actor, player, enemy)) = actor_query.get(entity) {
            if player.is_some() {
                println!();
                log::info!("Processing turn for player entity: {}", entity);

                // Player's turn - wait for input
                commands.entity(entity).insert(WaitingForInput);
            } else if enemy.is_some() {
                log::info!("Processing turn for enemy entity: {}", entity);

                // Enemy turn - perform AI actions
                // Calculate move or attack...

                // Example: move in random direction
                // Randomly select one of the MoveDirection values
                let directions = MoveDirection::ALL_DIRECTIONS;
                let random_direction = directions[fastrand::usize(..directions.len())];

                commands
                    .entity(entity)
                    .insert(Action::Move(random_direction));

                // Schedule next turn using TurnActor's speed
                turn_system.schedule_turn(entity, turn_actor.speed);
            }
        }
    }
}

pub fn execute_actions(
    mut commands: Commands,
    actions: Query<(Entity, &Action)>,
    // mut position_query: Query<&mut Position>,
    // Other queries for game state
) {
    for (entity, action) in &actions {
        log::info!("Executing action: {:?} for entity: {}", action, entity);

        match action {
            Action::Move(direction) => {
                commands.entity(entity).queue(TryMove::new(*direction));

                // Update position if valid
                // if let Ok(mut position) = position_query.get_mut(entity) {
                //     // Update actual position (data model)
                //     *position = *position + *direction;
                // }
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
