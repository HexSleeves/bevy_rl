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
