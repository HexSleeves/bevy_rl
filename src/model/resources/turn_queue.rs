use std::{cmp::Reverse, collections::BinaryHeap};

use bevy::prelude::*;

#[derive(Resource)]
pub struct TurnQueue {
    pub current_time: u64,
    pub processing: bool,
}

impl Default for TurnQueue {
    fn default() -> Self {
        Self {
            current_time: 0,
            processing: true,
        }
    }
}

#[derive(Resource, Default)]
pub struct TurnSystem {
    current_time: u64,
    turn_queue: BinaryHeap<Reverse<(u64, Entity)>>,
}

impl TurnSystem {
    // Add actor to the queue with wrapping time calculation
    pub fn schedule_turn(&mut self, entity: Entity, delay: u64) {
        let next_time = self.current_time.wrapping_add(delay);
        self.turn_queue.push(Reverse((next_time, entity)));
        log::info!(
            "Current time: {}, Scheduled turn for entity {} at time {}.",
            self.current_time,
            entity,
            next_time
        );
    }

    // Get the next actor in the queue
    pub fn get_next_actor(&mut self) -> Option<(Entity, u64)> {
        if let Some(Reverse((time, entity))) = self.turn_queue.pop() {
            // Update current time with wrapping protection
            self.current_time = time;
            Some((entity, time))
        } else {
            None
        }
    }

    // Get current time
    pub fn current_time(&self) -> u64 {
        self.current_time
    }

    // Peek at next actor without removing
    pub fn peek_next(&self) -> Option<(Entity, u64)> {
        self.turn_queue
            .peek()
            .map(|Reverse((time, entity))| (*entity, *time))
    }

    // Check if an entity's turn is scheduled
    pub fn is_scheduled(&self, entity: Entity) -> bool {
        self.turn_queue.iter().any(|Reverse((_, e))| *e == entity)
    }

    // Properly handle time comparison with wrapping
    pub fn time_until(&self, time: u64) -> u64 {
        time.wrapping_sub(self.current_time)
    }

    // Compare two times accounting for wrapping
    pub fn is_before(&self, time_a: u64, time_b: u64) -> bool {
        // This handles wrapping correctly
        self.time_until(time_a) < self.time_until(time_b)
    }
}
