use bevy::prelude::*;

use super::MoveDirection;

// Define components for the turn system
#[derive(Component, Debug)]
pub struct TurnActor {
    pub speed: u64,
    pub next_turn_time: u64,
}

#[derive(Component)]
pub struct WaitingForInput;

/// Represents an action that can be taken by an entity
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    /// Move in a specific direction
    Move(MoveDirection),
    /// Attack a target entity
    Attack(Entity),
    /// Pick up an item at the current position
    PickupItem,
    /// Wait and skip the turn
    Wait,
}

/// Represents an action that can be mapped to input keys
/// Similar to Action but without Entity references for static storage
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputAction {
    /// Move in a specific direction
    Move(MoveDirection),
    /// Pick up an item at the current position
    PickupItem,
    /// Wait and skip the turn
    Wait,
}

impl From<InputAction> for Action {
    fn from(input: InputAction) -> Self {
        match input {
            InputAction::Move(dir) => Action::Move(dir),
            InputAction::PickupItem => Action::PickupItem,
            InputAction::Wait => Action::Wait,
        }
    }
}
