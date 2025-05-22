use bevy::prelude::*;

use crate::model::components::GridPosition;

#[derive(Event, Debug)]
pub struct SpawnPlantEvent {
    pub grid_position: GridPosition,
}

#[derive(Event, Debug)]
pub struct SuccessSpawnPlantEvent {
    pub sun_cost: u32,
}

#[derive(Event, Debug)]
pub struct FailedSpawnPlantEvent;

#[derive(Event, Debug)]
pub struct DespawnPlantEvent {
    pub grid_position: GridPosition,
}