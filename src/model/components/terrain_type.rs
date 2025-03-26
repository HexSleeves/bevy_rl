use bevy::prelude::*;

use crate::model::components::Description;

#[derive(Component, Reflect, Default, PartialEq, Eq, Clone)]
#[reflect(Component)]
#[require(Description)]
pub enum TerrainType {
    #[default]
    Floor,
    Wall,
    Door,
    UpStairs,
    DownStairs,
}

impl TerrainType {
    /// Returns true if this terrain type blocks vision (walls, etc.)
    pub fn blocks_vision(&self) -> bool {
        match self {
            TerrainType::Wall => true,
            TerrainType::Door => true, // Closed doors block vision
            _ => false,
        }
    }
}
