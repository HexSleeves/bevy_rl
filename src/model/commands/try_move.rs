// use bevy::{ecs::system::SystemState, prelude::*};

// use crate::model::components::{Position, TerrainType};

// pub struct TryMove(pub Direction);

// impl TryMove {
//     pub fn new(dir: Direction) -> Self {
//         Self(dir)
//     }
// }

// impl EntityCommand for TryMove {
//     fn apply(self, entity: Entity, world: &mut World) {
//         let mut state: SystemState<(
//             // ResMut<CurrentMap>,
//             // ResMut<GameLog>,
//             Query<&mut Position>,
//             Query<&TerrainType>,
//         )> = SystemState::new(world);

//         let (mut current_map, mut game_log, mut q_position, q_terrain_type) = state.get_mut(world);
//         let Ok(mut position) = q_position.get_mut(entity) else {
//             return;
//         };

//         let new_position = *position + self.0.coord();
//         let Some(terrain_entity) = current_map.get_terrain(new_position) else {
//             return;
//         };

//         let Ok(terrain_type) = q_terrain_type.get(terrain_entity) else {
//             return;
//         };

//         match terrain_type {
//             TerrainType::Floor => {
//                 game_log.add_message(format!("You move {}", self.0));
//                 current_map.set_actor(*position, None);
//                 *position = new_position;
//                 current_map.set_actor(new_position, Some(entity));
//             }
//             TerrainType::Wall => {
//                 game_log.add_message("You bump into a wall.");
//             }
//         }
//     }
// }
