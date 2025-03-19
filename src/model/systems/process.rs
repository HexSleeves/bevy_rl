use bevy::{ecs::system::SystemState, prelude::*};

use crate::model::{
    components::{AwaitingInput, PlayerTag, TurnActor},
    resources::TurnQueue,
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

    // Limit how many turns we process per frame
    // let start_time = std::time::Instant::now();
    // let max_processing_time = std::time::Duration::from_millis(5);

    // Process turns until budget is exhausted
    // while start_time.elapsed() < max_processing_time {
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

            if player.is_some() && actor.peak_next_action().is_none() {
                world.entity_mut(entity).insert(AwaitingInput);
                turn_queue.schedule_turn(entity, time);
                log::info!("Player is awaiting input: {:?}", entity);
                return;
            }

            let Some(action) = actor.next_action() else {
                log::info!("No action for entity: {:?}. Rescheduling turn.", entity);
                turn_queue.schedule_turn(entity, time);
                return;
            };

            match action.perform(world) {
                Ok(d_time) => turn_queue.schedule_turn(entity, time + d_time),
                Err(e) => {
                    log::error!("Failed to perform action: {:?}", e);
                }
            }
        }
    });
    // }
}
