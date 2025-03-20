use bevy::{ecs::system::SystemState, prelude::*};

use crate::model::{
    actions::WalkBuilder,
    components::{AwaitingInput, PlayerTag, TurnActor},
    resources::TurnQueue,
    types::{GameActionBuilder, MoveDirection},
};

pub fn process_turns(world: &mut World) {
    // Break if we're waiting for player input
    if world.query_filtered::<(), With<AwaitingInput>>().iter(world).count() > 0 {
        return;
    }

    // Quick check if any actor has actions
    // {
    //     let mut query = world.query::<&TurnActor>();
    //     let has_actions = query.iter(world).any(|actor| !actor.actions.is_empty());
    //     if !has_actions {
    //         println!("No actions to process. Skipping...");
    //         return; // Skip the expensive processing
    //     }
    // }

    let mut state: SystemState<(Query<(Entity, &mut TurnActor, Option<&PlayerTag>)>,)> =
        SystemState::new(world);

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
            let (mut q_actor,) = state.get_mut(world);

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

            if is_player && !has_action {
                world.entity_mut(entity).insert(AwaitingInput);
                turn_queue.schedule_turn(entity, time);
                log::info!("Player is awaiting input: {:?}", entity);
                return;
            }

            let Some(action) = actor.next_action() else {
                // log::info!("No action for entity: {:?}. Rescheduling turn.", entity);
                // turn_queue.schedule_turn(entity, time);
                // return;

                // Generate a random walk direction
                let direction = MoveDirection::random_direction();
                actor.add_action(
                    WalkBuilder::new().with_entity(entity).with_direction(direction.into()).build(),
                );

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
