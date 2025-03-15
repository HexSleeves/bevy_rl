use bevy::prelude::*;
use rand::Rng;

use crate::model::{
    components::{Position, Renderable, TerrainType},
    model_constants::ModelConstants,
    resources::Map,
    utils::spawn_ascii_entity,
};

pub fn spawn_map(mut commands: Commands, mut map: ResMut<Map>, asset_server: Res<AssetServer>) {
    let mut rng = rand::rng();

    // Initialize all tiles as walls
    for x in 0..ModelConstants::MAP_WIDTH as i32 {
        for y in 0..ModelConstants::MAP_HEIGHT as i32 {
            map.tiles.insert((x, y), TerrainType::Wall);
        }
    }

    // Create a simple room in the center
    let room_width = 20;
    let room_height = 15;
    let start_x = (ModelConstants::MAP_WIDTH as i32 - room_width) / 2;
    let start_y = (ModelConstants::MAP_HEIGHT as i32 - room_height) / 2;

    for x in start_x..(start_x + room_width) {
        for y in start_y..(start_y + room_height) {
            map.tiles.insert((x, y), TerrainType::Floor);
        }
    }

    // Add some random walls inside the room
    for _ in 0..10 {
        let x = rng.random_range(start_x + 1..(start_x + room_width - 1));
        let y = rng.random_range(start_y + 1..(start_y + room_height - 1));
        map.tiles.insert((x, y), TerrainType::Wall);
    }

    // Render the map
    for (pos, tile_type) in &map.tiles {
        let (x, y) = *pos;

        match tile_type {
            TerrainType::Wall => {
                spawn_ascii_entity(
                    &mut commands,
                    &asset_server,
                    Position::new(x, y),
                    Renderable {
                        glyph: '#',
                        color: Color::srgb(0.5, 0.5, 0.5), // Gray
                    },
                    0.0,
                );
            }
            TerrainType::Floor => {
                spawn_ascii_entity(
                    &mut commands,
                    &asset_server,
                    Position::new(x, y),
                    Renderable {
                        glyph: '.',
                        color: Color::srgb(0.0, 0.5, 0.0), // Dark green
                    },
                    0.0,
                );
            }
        }
    }
}
