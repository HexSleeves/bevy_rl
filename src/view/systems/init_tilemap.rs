use bevy::prelude::*;

use crate::view::{resources::TileMap, ViewConstants};

const TILEMAP_PATH: &str = "tilemap.png";
const TILEMAP_COLUMNS: usize = 16;
const TILEMAP_ROWS: usize = 16;

/// Initialize the tilemap resource
pub fn init_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a new tilemap
    let tilemap = TileMap::new(
        &asset_server,
        TILEMAP_PATH,
        Vec2::splat(ViewConstants::TILE_SIZE),
        TILEMAP_COLUMNS,
        TILEMAP_ROWS,
    );

    // Insert the tilemap as a resource
    commands.insert_resource(tilemap);
}
