use bevy::prelude::*;

use crate::model::{
    components::{Position, Renderable},
    model_constants::ModelConstants,
};
use crate::view::ViewConstants;

// Helper function to spawn an ASCII entity
pub fn spawn_ascii_entity(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Position,
    renderable: Renderable,
    z_index: f32,
) -> Entity {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let text_style = TextFont {
        font: font.clone(),
        font_size: 25.0,
        ..default()
    };

    commands
        .spawn((
            Text2d::new(renderable.glyph.to_string()),
            text_style,
            TextColor(renderable.color),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(
                position.x as f32 * ViewConstants::TILE_SIZE
                    - (ModelConstants::MAP_WIDTH as f32 * ViewConstants::TILE_SIZE / 2.0)
                    + (ViewConstants::TILE_SIZE / 2.0),
                position.y as f32 * ViewConstants::TILE_SIZE
                    - (ModelConstants::MAP_HEIGHT as f32 * ViewConstants::TILE_SIZE / 2.0)
                    + (ViewConstants::TILE_SIZE / 2.0),
                z_index,
            ),
            position,
            renderable,
        ))
        .id()
}
