use bevy::prelude::*;

use crate::model::{
    components::{AITag, PlayerTag, Position, Renderable, TerrainType, TurnActor, ViewShed},
    resources::{CurrentMap, SpawnPoint, TurnQueue},
    ModelConstants,
};

pub fn spawn_player(
    mut commands: Commands,
    mut current_map: ResMut<CurrentMap>,
    mut turn_system: ResMut<TurnQueue>,
    terrain_query: Query<&TerrainType>,
    spawn_point: Option<Res<SpawnPoint>>,
) {
    // Determine where to spawn the player
    let player_position = if let Some(spawn_point) = spawn_point {
        if let Some(pos) = spawn_point.player_spawn {
            pos
        } else {
            find_valid_position(&current_map, &terrain_query)
        }
    } else {
        find_valid_position(&current_map, &terrain_query)
    };

    // Spawn the player
    let player_id = commands
        .spawn((
            player_position,
            Renderable {
                glyph: '@',
                color: Color::srgb(1.0, 1.0, 0.0), // #ffff00
            },
            PlayerTag,
            TurnActor::new(100),
            ViewShed { radius: 8 },
        ))
        .id();

    // Spawn an enemy at a random location
    let actor_position = find_valid_position(&current_map, &terrain_query);
    let actor_id = commands
        .spawn((
            actor_position,
            Renderable {
                glyph: 'E',
                color: Color::srgb(1.0, 0.0, 0.0), // #ff0000
            },
            AITag,
            TurnActor::new(120),
        ))
        .id();

    // Set the player and actor on the map
    current_map.set_actor(player_position, Some(player_id));
    current_map.set_actor(actor_position, Some(actor_id));

    // Schedule the player and actor to take turns
    let current_time = turn_system.current_time();
    turn_system.schedule_turn(player_id, current_time);
    turn_system.schedule_turn(actor_id, current_time);
}

// Helper function to find a valid floor position
fn find_valid_position(current_map: &CurrentMap, terrain_query: &Query<&TerrainType>) -> Position {
    let mut rng = fastrand::Rng::new();
    let mut valid_positions = Vec::new();

    for y in 1..ModelConstants::MAP_HEIGHT - 1 {
        for x in 1..ModelConstants::MAP_WIDTH - 1 {
            if let Some(terrain_entity) = current_map.get_terrain(Position::new(x as i32, y as i32)) {
                if let Ok(terrain_type) = terrain_query.get(terrain_entity) {
                    if *terrain_type == TerrainType::Floor {
                        valid_positions.push(Position::new(x as i32, y as i32));
                    }
                }
            }
        }
    }

    if valid_positions.is_empty() {
        // If no valid positions found, return a default position
        Position::new(ModelConstants::MAP_WIDTH as i32 / 2, ModelConstants::MAP_HEIGHT as i32 / 2)
    } else {
        valid_positions[rng.usize(0..valid_positions.len())]
    }
}
