use bevy::prelude::*;

use super::components::GridPosition;

#[derive(Event, Debug)]
pub struct PeaSpawnEvent {
    pub start_grid: GridPosition,
    pub start: Vec3,
    pub damage: f32,
}