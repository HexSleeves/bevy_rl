use bevy::prelude::*;
use rand::Rng;

use crate::model::{
    components::{Enemy, Player, Position, Renderable, TerrainType, TurnActor},
    resources::Map,
    utils::spawn_ascii_entity,
};

pub fn spawn_player(mut commands: Commands, map: Res<Map>, asset_server: Res<AssetServer>) {
    // Find a valid floor tile for the player
    let mut valid_positions = Vec::new();
    for (pos, tile_type) in &map.tiles {
        if *tile_type == TerrainType::Floor {
            valid_positions.push(*pos);
        }
    }

    // Choose a random position
    let mut rng = rand::rng();
    let player_pos = valid_positions[rng.random_range(0..valid_positions.len())];
    let (x, y) = player_pos;

    let player_id = spawn_ascii_entity(
        &mut commands,
        &asset_server,
        Position::new(x, y),
        Renderable {
            glyph: '@',
            color: Color::srgb(1.0, 1.0, 0.0), // Yellow
        },
        1.0,
    );

    commands.entity(player_id).insert((
        Player,
        TurnActor {
            speed: 100,
            next_turn_time: 0, // Player goes first
        },
    ));

    // Spawn an enemy
    let enemy_id = spawn_ascii_entity(
        &mut commands,
        &asset_server,
        Position::new(x + 1, y + 1),
        Renderable {
            glyph: 'E',
            color: Color::srgb(1.0, 0.0, 0.0), // Red
        },
        1.0,
    );

    commands.entity(enemy_id).insert((
        Enemy,
        TurnActor {
            speed: 120, // Enemy is slower
            next_turn_time: 0,
        },
    ));
}
