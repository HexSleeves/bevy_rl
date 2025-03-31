use bevy::prelude::*;

use crate::model::{
    components::{AwaitingInput, Description, PlayerTag, Position, Renderable, TerrainType, ViewShed},
    resources::{CurrentMap, FovMap, SpawnPoint},
    systems::{compute_fov, monsters_turn, process_turns, spawn_map, spawn_player},
};

use super::resources::TurnQueue;

pub fn is_player_turn(query: Query<&AwaitingInput>) -> bool {
    query.get_single().is_ok()
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    PlayerTurn,
    MonstersTurn,
    ComputeFov,
    ProcessTurns,
    ProcessActions,
}

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Description>();
        app.register_type::<PlayerTag>();
        app.register_type::<Position>();
        app.register_type::<Renderable>();
        app.register_type::<TerrainType>();
        app.register_type::<ViewShed>();
        app.register_type::<SpawnPoint>();

        app.init_state::<GameState>();

        app.init_resource::<TurnQueue>();
        app.init_resource::<CurrentMap>();
        app.init_resource::<FovMap>();
        app.init_resource::<SpawnPoint>();

        app.add_systems(Startup, (spawn_map, spawn_player, compute_fov, process_turns).chain());

        app.add_systems(Update, process_turns.run_if(in_state(GameState::ProcessTurns)));
        app.add_systems(Update, monsters_turn.run_if(in_state(GameState::MonstersTurn)));
        app.add_systems(OnExit(GameState::ProcessTurns), compute_fov);
    }
}
