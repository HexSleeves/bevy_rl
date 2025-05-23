use bevy::{ecs::system::SystemState, prelude::*};

use crate::{
    impl_debug_with_field, impl_game_action,
    model::{
        components::{Position, TerrainType},
        resources::CurrentMap,
        types::{GameAction, GameError},
    },
};

#[derive(Default)]
pub struct WalkBuilder {
    entity: Option<Entity>,
    direction: Option<IVec2>,
}

impl WalkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_entity(mut self, entity: Entity) -> Self {
        self.entity = Some(entity);
        self
    }

    pub fn with_direction(mut self, direction: IVec2) -> Self {
        self.direction = Some(direction);
        self
    }
}

pub struct Walk {
    entity: Entity,
    direction: IVec2,
}

impl GameAction for Walk {
    fn entity(&self) -> Option<Entity> {
        Some(self.entity)
    }

    fn perform(&self, world: &mut World) -> Result<u64, GameError> {
        let mut state: SystemState<(ResMut<CurrentMap>, Query<&mut Position>, Query<&TerrainType>)> =
            SystemState::new(world);

        // Get references to the data
        let (current_map, mut q_position, q_terrain_type) = state.get_mut(world);

        // Get the entity's current position
        if let Ok(mut current_pos) = q_position.get_mut(self.entity) {
            let new_pos = *current_pos + self.direction;

            let Some(terrain_entity) = current_map.get_terrain(new_pos) else {
                log::error!("Failed to get terrain for entity: {}", self.entity);
                return Err(GameError::InvalidPosition);
            };

            let Ok(terrain_type) = q_terrain_type.get(terrain_entity) else {
                log::error!("Failed to get terrain type for entity: {}", self.entity);
                return Err(GameError::MissingComponent);
            };

            match terrain_type {
                TerrainType::Wall => {
                    log::error!("Wall in the way");
                    return Err(GameError::TerrainBlocked);
                }
                _ => {
                    *current_pos = new_pos;
                }
            }
        } else {
            return Err(GameError::EntityNotFound);
        }

        // Return the system state to update the world
        state.apply(world);

        Ok(1000)
    }
}

impl_debug_with_field!(Walk, direction);
impl_game_action!(Walk, WalkBuilder, entity, direction);
