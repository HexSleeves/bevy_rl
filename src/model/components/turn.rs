use bevy::prelude::*;

#[derive(Component)]
pub struct WaitingForInput;

#[derive(Component)]
pub struct MovementSpeed(pub u32); // Time units needed for a move
