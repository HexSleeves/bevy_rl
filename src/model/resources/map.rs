use bevy::{prelude::*, utils::HashMap};
use brtk::prelude::*;

use crate::model::{
    components::{Description, Position, TerrainType},
    ModelConstants,
};

#[derive(Reflect, Clone)]
pub struct Map {
    pub size: (usize, usize),

    pub terrain: Grid<Entity>,
    pub actors: HashMap<Position, Entity>,
}

impl FromWorld for Map {
    fn from_world(world: &mut World) -> Self {
        let size = (ModelConstants::MAP_WIDTH, ModelConstants::MAP_HEIGHT);

        Self {
            size,
            actors: HashMap::new(),
            terrain: Grid::new_fn(size, |_index, (x, y)| {
                let (tile_type, tile_description) = if x == 0 || y == 0 || x == size.0 - 1 || y == size.1 - 1
                {
                    (TerrainType::Wall, Description::new("Wall"))
                } else {
                    (TerrainType::Floor, Description::new("Floor"))
                };
                world.spawn((tile_type, tile_description, Position::new(x as i32, y as i32))).id()
            }),
        }
    }
}

impl Map {
    pub fn new(commands: &mut Commands, size: (usize, usize)) -> Self {
        let terrain = Grid::new_fn(size, |_index, (x, y)| {
            let (tile_type, tile_description) = if x == 0 || y == 0 || x == size.0 - 1 || y == size.1 - 1 {
                (TerrainType::Wall, Description::new("Wall"))
            } else {
                (TerrainType::Floor, Description::new("Floor"))
            };
            commands.spawn((tile_type, tile_description, Position::new(x as i32, y as i32))).id()
        });

        Self { size, terrain, actors: HashMap::new() }
    }

    pub fn pos_to_idx(&self, position: Position) -> usize {
        self.terrain.position_to_index_unchecked(position.into())
    }

    pub fn idx_to_pos(&self, idx: usize) -> Option<(i32, i32)> {
        self.terrain.index_to_position(idx)
    }

    pub fn get_terrain(&self, position: Position) -> Option<Entity> {
        self.terrain.get(position.into()).copied()
    }

    pub fn get_actor(&self, position: Position) -> Option<Entity> {
        self.actors.get(&position).copied()
    }

    pub fn set_actor(&mut self, position: Position, actor: Option<Entity>) {
        if let Some(actor) = actor {
            self.actors.insert(position, actor);
        } else {
            self.actors.remove(&position);
        }
    }

    // Helper method to check if a position is in bounds
    pub fn in_bounds(&self, position: Position) -> bool {
        let (x, y) = position.into();
        x >= 0 && y >= 0 && x < self.size.0 as i32 && y < self.size.1 as i32
    }
}
