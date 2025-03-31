use bevy::{ecs::system::SystemState, prelude::*};

use crate::model::{
    actions::WalkBuilder,
    components::{AITag, AwaitingInput, DeadTag, PlayerTag, Position, TerrainType, TurnActor},
    resources::{CurrentMap, TurnQueue},
    types::{GameActionBuilder, MoveDirection},
    GameState,
};

pub fn process_turns(world: &mut World) {
    let mut state: SystemState<(
        ResMut<NextState<GameState>>,
        Query<(Entity, &mut TurnActor, Option<&PlayerTag>)>,
    )> = SystemState::new(world);

    world.resource_scope(|world, mut turn_queue: Mut<TurnQueue>| {
        // Periodically clean up the queue
        let metrics = turn_queue.cleanup_dead_entities(world);

        // Log significant cleanups
        if metrics.entities_removed > 10 {
            log::info!(
                "Turn queue cleanup: removed {} entities in {:?}",
                metrics.entities_removed,
                metrics.processing_time
            );
        }
        turn_queue.print_queue();

        while let Some((entity, time)) = turn_queue.get_next_actor() {
            let (mut next_state, mut q_actor) = state.get_mut(world);

            let Ok((entity, mut actor, player)) = q_actor.get_mut(entity) else {
                log::error!("Actor not found: {:?}", entity);
                continue;
            };

            if !actor.is_alive() {
                log::info!("Actor is dead. Why is it still in the queue?");
                continue;
            }

            let is_player = player.is_some();
            let has_action = actor.peak_next_action().is_some();

            // Player is waiting for input
            if is_player && !has_action {
                log::info!("Player is awaiting input: {:?}", entity);
                next_state.set(GameState::PlayerTurn);
                world.entity_mut(entity).insert(AwaitingInput);
                turn_queue.schedule_turn(entity, time);
                return;
            }

            let Some(action) = actor.next_action() else {
                log::info!("No action for entity: {:?}. Rescheduling turn.", entity);
                turn_queue.schedule_turn(entity, time);
                return;
            };

            // Get next action and drop turn_queue borrow temporarily
            match action.perform(world) {
                Ok(d_time) => turn_queue.schedule_turn(entity, time + d_time),
                Err(e) => {
                    log::error!("Failed to perform action: {:?}", e);

                    if is_player {
                        turn_queue.schedule_turn(entity, time);
                    } else {
                        turn_queue.schedule_turn(entity, time + 100);
                    }
                }
            }
        }
    });
}

pub fn monsters_turn(world: &mut World) {
    log::info!("Monsters turn");

    let mut state: SystemState<(
        ResMut<NextState<GameState>>,
        Query<(Entity, &mut TurnActor), (With<AITag>, Without<PlayerTag>, Without<DeadTag>)>,
        Query<&Position>,
        Res<CurrentMap>,
        Query<&TerrainType>,
    )> = SystemState::new(world);

    let (mut next_state, mut ai_query, position_query, current_map, terrain_query) = state.get_mut(world);

    for (entity, mut turn_actor) in &mut ai_query {
        // Skip entities that already have actions queued
        if turn_actor.peak_next_action().is_some() {
            continue;
        }

        // Get the entity's current position
        if let Ok(position) = position_query.get(entity) {
            // Try different directions in a random order
            let directions = MoveDirection::all_directions();
            let mut valid_direction = None;

            // Find a valid direction to move (one that leads to a walkable tile)
            for _ in 0..directions.len() {
                let direction = MoveDirection::random_direction();
                let new_position = *position + direction;

                // Check if the new position is valid
                if let Some(terrain_entity) = current_map.get_terrain(new_position) {
                    if let Ok(terrain_type) = terrain_query.get(terrain_entity) {
                        // Check if we can walk there
                        if *terrain_type == TerrainType::Floor {
                            valid_direction = Some(direction);
                            break;
                        }
                    }
                }
            }

            // If we found a valid direction, queue the walk action
            if let Some(direction) = valid_direction {
                log::debug!("AI entity {:?} moving in direction {:?}", entity, direction);
                turn_actor.add_action(
                    WalkBuilder::new().with_entity(entity).with_direction(direction.into()).build(),
                );
            } else {
                // If no valid direction was found, just wait
                log::debug!("AI entity {:?} has no valid move, waiting", entity);
                // Here we could add a wait action if we had one
            }
        }
    }

    next_state.set(GameState::ProcessTurns);
}
