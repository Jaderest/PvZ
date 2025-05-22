use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ZombieSpawnEvent {
    pub x: f32,
    pub y: u32,
}