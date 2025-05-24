use bevy::prelude::*;

use crate::model::projectile::*;

use super::components::GridPosition;

#[derive(Event, Debug)]
pub struct SpawnPeaEvent {
    pub start_grid: GridPosition,
    pub start: Vec3,
}