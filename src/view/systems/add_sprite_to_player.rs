use bevy::prelude::*;

use crate::{
    model::components::PlayerTag,
    view::{components::TileSprite, resources::TileMap},
};

/// System that adds sprite components to the player entity
pub fn add_sprite_to_player(
    mut commands: Commands,
    tilemap: Option<Res<TileMap>>,
    q_player: Option<Single<Entity, (With<PlayerTag>, Without<Sprite>)>>,
) {
    // If the tilemap resource isn't available yet, we can't add sprites
    let Some(tilemap) = tilemap else {
        return;
    };

    if let Some(player) = q_player {
        let entity = player.into_inner();

        // Get player sprite coordinates from the tilemap
        let player_coords = tilemap.get_player_sprite_coords();
        let index = tilemap.coords_to_index(player_coords);

        // Create a TileSprite component
        let tile_sprite = TileSprite::new(player_coords, tilemap.tile_size);

        // Generate the sprite using our helper method
        let sprite = tilemap.generate_sprite_for_terrain(index);

        // Add the sprite components to the entity
        commands.entity(entity).insert((sprite, tile_sprite));
    }
}
