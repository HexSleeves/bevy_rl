use bevy::prelude::*;

use crate::{
    model::{
        components::{Position, Renderable},
        ModelConstants,
    },
    view::ViewConstants,
};

pub fn position_to_transform(
    mut q_objects: Query<(&Position, &Renderable, &mut Text2d, &mut TextColor, &mut Transform)>,
) {
    for (position, renderable, mut text, mut text_color, mut transform) in &mut q_objects {
        // Update the text with the current glyph
        text.0 = renderable.glyph.to_string();
        text_color.0 = renderable.color;

        // Update position
        transform.translation.x = position.x() as f32 * ViewConstants::TILE_SIZE
            - (ModelConstants::MAP_WIDTH as f32 * ViewConstants::TILE_SIZE / 2.0)
            + (ViewConstants::TILE_SIZE / 2.0);
        transform.translation.y = position.y() as f32 * ViewConstants::TILE_SIZE
            - (ModelConstants::MAP_HEIGHT as f32 * ViewConstants::TILE_SIZE / 2.0)
            + (ViewConstants::TILE_SIZE / 2.0);
    }
}
