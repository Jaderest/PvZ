use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ZombieSpawnEvent {
    pub y: u32,
}