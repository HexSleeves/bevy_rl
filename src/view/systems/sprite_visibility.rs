use bevy::{prelude::*, text::TextColor};

use crate::model::{components::Position, resources::FovMap};

pub fn update_sprite_visibility(fov_map: Res<FovMap>, mut query: Query<(&Position, &mut TextColor)>) {
    for (pos, mut color) in &mut query {
        if fov_map.is_visible(*pos) {
            color.0.set_alpha(1.0);
        } else if fov_map.is_revealed(*pos) {
            // Visible but not visible
            color.0.set_alpha(0.1);
        } else {
            // Not visible or revealed
            color.0.set_alpha(0.0);
        }
    }
}
