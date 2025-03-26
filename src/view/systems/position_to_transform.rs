use bevy::prelude::*;

use crate::{
    model::{components::Position, ModelConstants},
    view::ViewConstants,
};

pub fn position_to_transform(mut q_objects: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut q_objects {
        // Update position
        transform.translation.x = position.x() as f32 * ViewConstants::TILE_SIZE
            - (ModelConstants::MAP_WIDTH as f32 * ViewConstants::TILE_SIZE / 2.0)
            + (ViewConstants::TILE_SIZE / 2.0);
        transform.translation.y = position.y() as f32 * ViewConstants::TILE_SIZE
            - (ModelConstants::MAP_HEIGHT as f32 * ViewConstants::TILE_SIZE / 2.0)
            + (ViewConstants::TILE_SIZE / 2.0);
    }
}
