use bevy::prelude::*;
use std::collections::VecDeque;

use crate::model::components::MoveDirection;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum ActionType {
    Move(MoveDirection),
    Attack(Entity),
    Wait,
    // Other actions
}

#[derive(Clone, Debug)]
pub struct Action {
    pub entity: Entity,
    pub action_type: ActionType,
}

// Queue for processing actions in order
#[derive(Resource, Default)]
pub struct ActionQueue {
    actions: VecDeque<Action>,
    waiting_for_animation: bool,
}

impl ActionQueue {
    pub fn len(&self) -> usize {
        self.actions.len()
    }

    pub fn print_queue(&self) {
        log::info!("Action queue: {:?}", self.actions);
    }

    pub fn is_waiting_for_animation(&self) -> bool {
        self.waiting_for_animation
    }

    pub fn set_waiting_for_animation(&mut self, waiting: bool) {
        self.waiting_for_animation = waiting;
    }

    pub fn push(&mut self, entity: Entity, action_type: ActionType) {
        self.actions.push_back(Action {
            entity,
            action_type,
        });
    }

    pub fn next_action(&mut self) -> Option<Action> {
        self.actions.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}
