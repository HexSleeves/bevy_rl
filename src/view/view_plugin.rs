use bevy::prelude::*;

use super::systems::{add_sprite_to_player, add_sprite_to_tile, init_tilemap, position_to_transform};
pub struct ViewPlugin;
impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        // Register the TileSprite component for reflection
        app.register_type::<crate::view::components::TileSprite>();

        // Initialize the tilemap during startup
        app.add_systems(Startup, init_tilemap);
        app.add_systems(PostUpdate, ((add_sprite_to_player, add_sprite_to_tile), position_to_transform));
    }
}
