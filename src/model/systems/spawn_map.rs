use bevy::prelude::*;

use crate::model::{generation::DungeonGenerator, resources::CurrentMap, ModelConstants};

pub fn spawn_map(mut commands: Commands, mut current_map: ResMut<CurrentMap>) {
    let mut rng = fastrand::Rng::new();
    let mut generator = DungeonGenerator::new(ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT);

    // Generate terrain types
    let terrain_grid = generator.generate(&mut rng);

    // Generate entities and update the map
    let terrain_entities = generator.generate_entities(&mut commands, &terrain_grid);

    // Update the current map
    current_map.terrain = terrain_entities;
}
