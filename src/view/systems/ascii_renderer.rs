use bevy::prelude::*;

use crate::model::{
    components::{Position, Renderable},
    ModelConstants,
};
use crate::view::ViewConstants;

// This system updates the text display for entities with both Position and Renderable components
pub fn update_ascii_display(
    mut query: Query<(
        &Position,
        &Renderable,
        &mut Text2d,
        &mut TextColor,
        &mut Transform,
    )>,
) {
    for (position, renderable, mut text, mut text_color, mut transform) in query.iter_mut() {
        // Update the text with the current glyph
        text.0 = renderable.glyph.to_string();
        text_color.0 = renderable.color;

        // Update position
        transform.translation.x = position.x as f32 * ViewConstants::TILE_SIZE
            - (ModelConstants::MAP_WIDTH as f32 * ViewConstants::TILE_SIZE / 2.0)
            + (ViewConstants::TILE_SIZE / 2.0);
        transform.translation.y = position.y as f32 * ViewConstants::TILE_SIZE
            - (ModelConstants::MAP_HEIGHT as f32 * ViewConstants::TILE_SIZE / 2.0)
            + (ViewConstants::TILE_SIZE / 2.0);
    }
}
