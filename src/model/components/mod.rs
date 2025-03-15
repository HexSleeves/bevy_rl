use bevy::prelude::*;

mod description;
pub use self::description::*;

mod turn;
pub use self::turn::*;

mod player;
pub use self::player::*;

mod position;
pub use self::position::*;

mod renderable;
pub use self::renderable::*;

mod terrain_type;
pub use self::terrain_type::*;

// Define components for the turn system
#[derive(Component, Debug)]
pub struct TurnActor {
    pub speed: u32,
    pub next_turn_time: u64,
}

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Resource)]
pub struct TurnQueue {
    pub current_time: u64,
    pub processing: bool,
}

// For players awaiting input
#[derive(Component)]
pub struct AwaitingInput;

// For actions being performed
#[derive(Component, Clone, Debug)]
pub enum Action {
    Move(IVec2),
    Attack(Entity),
    Wait,
    // Other actions...
}

impl Default for TurnQueue {
    fn default() -> Self {
        Self {
            current_time: 0,
            processing: true,
        }
    }
}
