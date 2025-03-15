use bevy::prelude::*;

use crate::model::components::{Description, Position};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Description, Position)]
pub struct Player;
