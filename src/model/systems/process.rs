use bevy::prelude::*;

use crate::model::components::*;

pub fn process_turns(
    mut commands: Commands,
    mut turn_queue: ResMut<TurnQueue>,
    awaiting_input: Query<Entity, With<AwaitingInput>>,
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
        log::info!("processing turn_actor: {:?}", turn_actor);
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

        log::info!("next_actor: {:?} => {:?}", entity, turn_actor);

        if player.is_some() {
            log::info!("player turn");
            // Handle player turn - await input
            commands.entity(entity).insert(AwaitingInput);
        } else if enemy.is_some() {
            log::info!("enemy turn");
            // Handle enemy turn - AI decides action
            // For example, move towards player
            commands
                .entity(entity)
                .insert(Action::Move(IVec2::new(0, 1)));

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
        log::info!("Executing action: {:?}", action);

        match action {
            Action::Move(dir) => {
                if let Ok(mut position) = position_query.get_mut(entity) {
                    // Check for collision (you'd expand this)
                    let new_pos = position.0 + *dir;

                    // Update position if valid
                    *position = Position(new_pos);
                }
            }
            Action::Wait => {
                // Do nothing
            }
            Action::Attack(target) => {
                // Handle attack logic
            }
        }

        // Remove the action once processed
        commands.entity(entity).remove::<Action>();
        log::info!("removed action: {:?} from entity: {:?}", action, entity);
        log::info!("");
    }
}
