use bevy::prelude::*;

use crate::model::components::GridPosition;

#[derive(Event, Debug)]
pub struct SpawnPlantEvent {
    pub grid_position: GridPosition,
}

#[derive(Event, Debug)]
pub struct DespawnPlantEvent {
    pub grid_position: GridPosition,
}