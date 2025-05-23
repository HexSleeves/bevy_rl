use bevy::{prelude::*, sprite::Anchor};

use crate::view::ViewConstants;

/// Resource that manages a tilemap for rendering sprites
#[derive(Resource, Clone)]
pub struct TileMap {
    /// The texture atlas handle for the tilemap
    pub texture_atlas: Handle<TextureAtlasLayout>,
    /// The image handle for the tilemap
    pub texture: Handle<Image>,
    /// The size of each tile in the tilemap
    pub tile_size: Vec2,
    /// The number of columns in the tilemap
    pub columns: usize,
    /// The number of rows in the tilemap
    pub rows: usize,
}

impl TileMap {
    /// Create a new tilemap from an image path
    pub fn new(
        asset_server: &AssetServer,
        image_path: &str,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
    ) -> Self {
        let texture = asset_server.load(image_path);

        // Create a texture atlas layout
        let layout = TextureAtlasLayout::from_grid(
            UVec2::new(tile_size.x as u32, tile_size.y as u32),
            columns as u32,
            rows as u32,
            None,
            None,
        );

        let texture_atlas = asset_server.add(layout);

        Self { texture_atlas, texture, tile_size, columns, rows }
    }

    /// Convert tile coordinates to a sprite index
    pub fn coords_to_index(&self, coords: (u32, u32)) -> usize {
        let (x, y) = coords;
        (y as usize * self.columns + x as usize) % (self.columns * self.rows)
    }

    /// Get the sprite index for a given terrain type
    pub fn get_sprite_index_for_terrain(
        &self,
        terrain_type: &crate::model::components::TerrainType,
    ) -> (u32, u32) {
        match terrain_type {
            crate::model::components::TerrainType::Floor => (14, 2),
            crate::model::components::TerrainType::Wall => (3, 2),
            crate::model::components::TerrainType::Door => (2, 0),
            crate::model::components::TerrainType::UpStairs => (3, 0),
            crate::model::components::TerrainType::DownStairs => (4, 0),
        }
    }

    /// Get the sprite coordinates for the player
    pub fn get_player_sprite_coords(&self) -> (u32, u32) {
        // This should be the coordinates of a player sprite in your tilemap
        // Adjust these values to match your specific tilemap
        (0, 4)
    }

    /// Generate a sprite for a terrain type using its index in the texture atlas
    ///
    /// Note: Handle<T> in Bevy is a lightweight reference (essentially an ID),
    /// so we can clone it without significant overhead.
    pub fn generate_sprite_for_terrain(&self, index: usize) -> Sprite {
        Sprite {
            image: self.texture.clone(),
            texture_atlas: Some(TextureAtlas { layout: self.texture_atlas.clone(), index }),
            custom_size: Some(Vec2::splat(ViewConstants::TILE_SIZE)),
            anchor: Anchor::BottomLeft,
            ..Default::default()
        }
    }
}
