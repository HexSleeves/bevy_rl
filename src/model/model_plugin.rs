use bevy::{prelude::*, utils::HashMap};

use crate::model::{
    components::{Description, Player, Position, Renderable, TerrainType, TurnQueue},
    model_constants::ModelConstants,
    resources::Map,
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

        app.init_resource::<TurnQueue>();

        app.insert_resource(Map {
            size: (ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT),
            tiles: HashMap::new(),
        });

        app.add_systems(Startup, (spawn_map, spawn_player).chain());
        app.add_systems(Update, (process_turns, execute_actions));
    }
}
