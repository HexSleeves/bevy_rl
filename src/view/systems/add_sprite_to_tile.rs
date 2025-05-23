use bevy::prelude::*;

use crate::{
    model::components::TerrainType,
    view::{components::TileSprite, resources::TileMap},
};

/// System that adds sprite components to tiles based on their terrain type
pub fn add_sprite_to_tile(
    mut commands: Commands,
    q_tiles: Query<(Entity, &TerrainType), (Added<TerrainType>, Without<Sprite>)>,
    tilemap: Option<Res<TileMap>>,
) {
    // If the tilemap resource isn't available yet, we can't add sprites
    let Some(tilemap) = tilemap else {
        return;
    };

    for (entity, tile_type) in q_tiles.iter() {
        // Get the sprite coordinates for this terrain type
        let tile_coords = tilemap.get_sprite_index_for_terrain(tile_type);
        let index = tilemap.coords_to_index(tile_coords);

        // Create a TileSprite component
        let tile_sprite = TileSprite::new(tile_coords, tilemap.tile_size);

        // Generate the sprite using our helper method
        let sprite = tilemap.generate_sprite_for_terrain(index);

        // Add the sprite components to the entity
        commands.entity(entity).insert((sprite, tile_sprite));
    }
}
