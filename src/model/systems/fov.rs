use bevy::prelude::*;

use crate::model::{
    components::{AwaitingInput, PlayerTag, Position, TerrainType, ViewShed},
    resources::{CurrentMap, FovMap},
};

/// System that computes FOV for all entities with a ViewShed component
pub fn compute_fov(
    map: Res<CurrentMap>,
    mut fov_map: ResMut<FovMap>,
    q_terrain: Query<&TerrainType>,
    query: Query<(&Position, &ViewShed), With<PlayerTag>>,
    awaiting_input: Query<&AwaitingInput>,
) {
    if !awaiting_input.is_empty() {
        return;
    }

    if let Ok((player_pos, view_shed)) = query.get_single() {
        fov_map.compute_fov(&q_terrain, &map, *player_pos, view_shed.radius);
    }
}
