use bevy::prelude::*;

use crate::model::{
    components::{Description, Player, Position, Renderable, TerrainType},
    resources::{ActionQueue, CurrentMap, TurnSystem},
    systems::{process_action_queue, process_turns, spawn_map, spawn_player},
};

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Description>();
        app.register_type::<Player>();
        app.register_type::<Position>();
        app.register_type::<Renderable>();
        app.register_type::<TerrainType>();

        app.init_resource::<TurnSystem>();
        app.init_resource::<ActionQueue>();

        app.init_resource::<CurrentMap>();

        app.add_systems(Startup, (spawn_map, spawn_player).chain());
        app.add_systems(
            Update,
            (
                process_turns,
                // handle_player_input,
                process_action_queue,
            )
                .chain(),
        );
    }
}
