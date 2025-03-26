use bevy::prelude::*;

use super::systems::{add_sprite_to_tile, position_to_transform, update_sprite_visibility};
use crate::{AppSet, RunningState};

pub struct ViewPlugin;
impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Update,
        //     position_to_transform
        //         .run_if(not(in_state(RunningState::Paused)))
        //         .in_set(AppSet::Render),
        // );

        app.add_systems(
            PostUpdate,
            (
                (add_sprite_to_tile, update_sprite_visibility),
                position_to_transform.in_set(AppSet::Render).run_if(not(in_state(RunningState::Paused))),
            ),
        );
    }
}
