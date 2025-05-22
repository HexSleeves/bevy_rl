use bevy::{ecs::system::SystemState, prelude::*};

use crate::model::{
    components::{Position, TerrainType},
    resources::CurrentMap,
    types::MoveDirection,
};

pub struct TryMove(pub MoveDirection);

impl TryMove {
    pub fn new(dir: MoveDirection) -> Self { Self(dir) }
}

impl EntityCommand for TryMove {
    // fn apply(self, entity: Entity, world: &mut World) {
    fn apply(self, mut entity_world: EntityWorldMut) {
        let entity = entity_world.id();
        log::info!("Executing TryMove for entity: {}", entity);

        entity_world.world_scope(move |world: &mut World| {
            let mut state: SystemState<(
                ResMut<CurrentMap>,
                // ResMut<GameLog>,
                Query<&mut Position>,
                Query<&TerrainType>,
            )> = SystemState::new(world);

            let (current_map, mut q_position, q_terrain_type) = state.get_mut(world);
            let Ok(mut position) = q_position.get_mut(entity) else {
                log::error!("Failed to get mut position for entity: {}", entity);
                return;
            };

            let new_position = *position + self.0;
            let Some(terrain_entity) = current_map.get_terrain(new_position) else {
                log::error!("Failed to get terrain for entity: {}", entity);
                return;
            };

            let Ok(terrain_type) = q_terrain_type.get(terrain_entity) else {
                log::error!("Failed to get terrain type for entity: {}", entity);
                return;
            };

            if *terrain_type == TerrainType::Floor {
                *position = new_position;
            } else {
                log::info!("Cannot move to {:?}", new_position);
            }
        });
    }
}
