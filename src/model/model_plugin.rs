use bevy::prelude::*;

use crate::model::{
    components::{Description, PlayerTag, Position, Renderable, TerrainType},
    resources::CurrentMap,
    systems::{process_turns, spawn_map, spawn_player},
};

use super::resources::TurnQueue;

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Description>();
        app.register_type::<PlayerTag>();
        app.register_type::<Position>();
        app.register_type::<Renderable>();
        app.register_type::<TerrainType>();

        app.init_resource::<TurnQueue>();
        // app.init_resource::<ActionQueueV2>();

        app.init_resource::<CurrentMap>();

        app.add_systems(Startup, (spawn_map, spawn_player).chain());
        app.add_systems(
            Update,
            (
                process_turns,
                // handle_player_input,
                // process_action_queue,
            )
                .chain(),
        );
    }
}
