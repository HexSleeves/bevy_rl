use bevy::prelude::*;

// Define components for the turn system
#[derive(Component, Debug)]
pub struct TurnActor {
    pub speed: u64,
    pub next_turn_time: u64,
}

#[derive(Component)]
pub struct WaitingForInput;
