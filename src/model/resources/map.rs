use bevy::{prelude::*, utils::HashMap};

use crate::model::components::TerrainType;

#[derive(Reflect, Resource)]
pub struct Map {
    pub size: (u32, u32),
    pub tiles: HashMap<(i32, i32), TerrainType>,
}

impl Map {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            size,
            tiles: HashMap::new(),
        }
    }
}
