use bevy::prelude::*;

use super::systems::update_ascii_display;
use crate::{AppSet, RunningState};

pub struct ViewPlugin;
impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_ascii_display
                .run_if(not(in_state(RunningState::Paused)))
                .in_set(AppSet::Render),
        );
    }
}
