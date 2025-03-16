use bevy::prelude::*;

use crate::model::{
    components::{Description, Player, Position, Renderable, TerrainType},
    resources::{Map, TurnQueue, TurnSystem},
    systems::{spawn_map, spawn_player},
};

use super::systems::{execute_actions, process_turns};

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Description>();
        app.register_type::<Player>();
        app.register_type::<Position>();
        app.register_type::<Renderable>();
        app.register_type::<TerrainType>();

        app.init_resource::<TurnSystem>();
        app.init_resource::<TurnQueue>();
        app.init_resource::<Map>();

        app.add_systems(Startup, (spawn_map, spawn_player).chain());
        app.add_systems(Update, (process_turns, execute_actions));
    }
}
