use bevy::prelude::*;

use crate::model::{
    components::{PlayerTag, Position, TerrainType, ViewShed},
    resources::{CurrentMap, FovMap},
};

/// System that computes FOV for all entities with a ViewShed component
pub fn compute_fov(
    map: Res<CurrentMap>,
    mut fov_map: ResMut<FovMap>,
    q_terrain: Query<&TerrainType>,
    query: Query<(&Position, &ViewShed), With<PlayerTag>>,
) {
    if let Ok((player_pos, view_shed)) = query.get_single() {
        log::info!("Computing FOV for player at {:?}", player_pos);
        fov_map.compute_fov(&q_terrain, &map, *player_pos, view_shed.radius);
    }
}
